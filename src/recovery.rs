//! Preserve forensic bytes and recover a format-2 canonical store to a
//! verified new root (`T01-04`).
//!
//! **Scope boundary.** This module recovers `crate::canonical::CanonicalStore`
//! (format-2, `canonical.sqlite`) only. Format-1's own pre-existing
//! torn-tail repair (`crate::events::EventLog::quarantine_and_repair_torn_tail`,
//! run inside `crate::vault::Vault::open_write`) is untouched — `T01-01`'s
//! evidence already records that plan §14's shared/exclusive "access lock"
//! model is described "in terms of the future SQLite canonical store," not
//! the file-based format-1 store, and this module follows that same
//! reading.
//!
//! **What "recovery" means here.** SQLite already performs its own
//! automatic crash recovery (hot-journal rollback) the moment a database
//! with a leftover `-journal` file is opened — this module does not
//! reimplement that. What it adds is the *safety envelope* plan §14/T01-04
//! require around it: never touch the live original while inspecting it,
//! preserve its exact bytes first, verify a disposable working copy far
//! more thoroughly than an ordinary `open` does (recomputing the entire
//! command chain from scratch rather than trusting stored values), and
//! publish a new, independently verified root only if that verification
//! passes — leaving the original completely untouched either way.
//!
//! **Not implemented here** (recorded, not silently dropped): an owner-facing
//! "partial salvage" mode that admits a truncated-but-partially-valid
//! history (this module's `verify` is all-or-nothing: any chain break,
//! dangling reference, or head mismatch refuses complete publication,
//! consistent with "irrecoverable fixtures refuse complete publication" —
//! but a *labeled partial* recovery is a richer, separate UX-facing
//! capability this task's own "minimal" pattern does not build yet); a CLI
//! `recover`/`verify` command (this store has no CLI wiring at all yet, per
//! `T01-03`'s own recorded scope boundary); and restoring from a *separate*
//! backup artifact (`T01-05`'s objective, not this one's — this module only
//! ever reads from the live root's own current bytes).

use crate::canonical;
use crate::vault::{WriteLock, ACCESS_LOCK_FILE, CONTROL_DIR, VAULT_META_FILE};
use crate::{Error, Result};
use serde::Serialize;
use std::fs;
use std::fs::TryLockError;
use std::path::{Path, PathBuf};

fn journal_file_name() -> String {
    format!("{}-journal", canonical::CANONICAL_DB_FILE)
}

const INCIDENT_MANIFEST_FILE: &str = "incident-manifest.json";

/// Durable record of one recovery attempt, written into the *preservation*
/// directory — which exists regardless of whether recovery ultimately
/// succeeded — so "what happened to this incident" survives independently
/// of whatever this process returns to its caller (§28: no bare `PASS`/
/// `SAFE`/`REFUSED` claim without exact supporting detail alongside it).
#[derive(Debug, Serialize)]
struct IncidentManifest<'a> {
    schema: &'static str,
    original_root: String,
    attempted_at: String,
    outcome: &'static str,
    recovered_root: Option<String>,
    vault_id: Option<&'a str>,
    verified_transaction_head_seq: Option<i64>,
    verified_object_count: Option<usize>,
    refusal_reason: Option<String>,
}

fn write_incident_manifest(preserved: &Path, manifest: &IncidentManifest) {
    // Best-effort: a failure to write the manifest itself must never mask
    // or replace the real recovery outcome already decided by the caller.
    if let Ok(json) = serde_json::to_string_pretty(manifest) {
        let _ = fs::write(preserved.join(INCIDENT_MANIFEST_FILE), json);
    }
}

/// Same documented limitation as `canonical::now_iso8601_placeholder` and
/// `vault::chrono_like_now_iso8601`: no `chrono`/`time` dependency is
/// admitted for this task, so this is a distinct, sortable, RFC3339-shaped
/// placeholder rather than a calendar-correct timestamp.
fn now_iso8601_placeholder() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("2026-09-15T{:05}Z", secs % 86400)
}

/// What a successful recovery actually did and verified — never a bare
/// "PASS" (§28: no `PASS`/`SAFE` claim without exact supporting detail).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RecoveryReport {
    pub original_root: PathBuf,
    pub recovered_root: PathBuf,
    pub vault_id: String,
    /// Where the exact original guard/database/journal bytes were copied
    /// *before* any verification or working-copy manipulation. Never
    /// removed by this module, on success or failure.
    pub preserved_at: PathBuf,
    pub verified_transaction_head_seq: i64,
    pub verified_object_count: usize,
}

/// Exclusive recovery ownership: writer ownership first, then exclusive
/// access — the fixed order §14 mandates ("Acquire writer ownership before
/// requesting exclusive recovery access; all processes use this order"),
/// matching the same order `Vault`/`CanonicalStore` use for their own
/// shared-access acquisition, so every code path in the crate acquires
/// these two locks in one consistent global order.
struct RecoveryGuard {
    _writer_lock: WriteLock,
    _access: fs::File,
}

impl RecoveryGuard {
    /// Non-blocking on the access lock: if any normal connection currently
    /// holds shared access, this returns `Busy`-shaped
    /// `Error::Recovery` immediately rather than waiting indefinitely (§14:
    /// "Recovery waits for normal connections to close or returns Busy" —
    /// this module chooses the immediate-Busy half of that, leaving a
    /// bounded-wait variant to a future task if needed).
    fn acquire(root: &Path) -> Result<Self> {
        let writer_lock = WriteLock::acquire(root)?;
        let control = root.join(CONTROL_DIR);
        let access_path = control.join(ACCESS_LOCK_FILE);
        let file = fs::OpenOptions::new()
            .write(true)
            .open(&access_path)
            .map_err(|e| {
                Error::Recovery(format!(
                    "cannot open access lock for recovery (missing coordination metadata at {}): {e}",
                    access_path.display()
                ))
            })?;
        match file.try_lock() {
            Ok(()) => {}
            Err(TryLockError::WouldBlock) => {
                return Err(Error::Recovery(
                    "recovery busy: a normal connection currently holds shared access".into(),
                ))
            }
            Err(TryLockError::Error(e)) => {
                return Err(Error::Recovery(format!(
                    "cannot acquire exclusive access for recovery: {e}"
                )))
            }
        }
        Ok(RecoveryGuard {
            _writer_lock: writer_lock,
            _access: file,
        })
    }
}

/// Recover the format-2 store at `original_root` to a fresh, independently
/// verified root at `new_root`. See module docs for exactly what this does
/// and does not do.
///
/// Ordering, matching plan §14/T01-04 exactly:
/// 1. Refuse immediately if `new_root` already has a published store
///    (no-clobber, checked again right before publication).
/// 2. Acquire recovery ownership (writer lease, then exclusive access) —
///    excludes every new normal connection and any concurrent recovery.
/// 3. Preserve the exact guard/database/journal bytes to a forensic
///    location *before* anything else touches them.
/// 4. Build a disposable working copy from the *preserved* bytes (never
///    the live original) and verify it far more thoroughly than an
///    ordinary `open` — [`canonical::verify_recovery_candidate`]
///    recomputes the whole command chain rather than trusting it.
/// 5. On verification failure: remove only the disposable working copy;
///    the original is untouched and the preserved bytes remain for
///    inspection.
/// 6. On success: publish the verified working copy to `new_root` (staged,
///    no-clobber, matching `T01-02`'s own creation discipline) and return a
///    complete report.
pub fn recover_to_new_root(
    original_root: impl AsRef<Path>,
    new_root: impl AsRef<Path>,
) -> Result<RecoveryReport> {
    recover_to_new_root_with_fault(original_root, new_root, None)
}

/// `T01-07` (D1/D4, "second crash injected during recovery itself"):
/// deterministic injection points inside `recover_to_new_root`'s own
/// sequence, distinct from corrupting the *source* before recovery starts.
/// Every variant must leave the preserved bytes and the live original both
/// completely intact — a second interruption during recovery must never
/// compound into losing the one thing recovery exists to protect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RecoveryFaultPoint {
    /// Immediately after forensic preservation completes, before the
    /// disposable working copy is built from it.
    AfterPreservation,
    /// After the working copy is built and independently verified, before
    /// the access lock is created and publication begins.
    AfterVerification,
    /// After the working copy's access lock is created, immediately before
    /// the final publishing rename.
    BeforePublish,
}

pub(crate) fn recover_to_new_root_with_fault(
    original_root: impl AsRef<Path>,
    new_root: impl AsRef<Path>,
    fault: Option<RecoveryFaultPoint>,
) -> Result<RecoveryReport> {
    let original_root = original_root.as_ref().to_path_buf();
    let new_root = new_root.as_ref().to_path_buf();

    let new_control = new_root.join(CONTROL_DIR);
    if new_control.exists() {
        return Err(Error::Recovery(format!(
            "recovery destination already published: {}",
            new_control.display()
        )));
    }

    let _guard = RecoveryGuard::acquire(&original_root)?;

    let control = original_root.join(CONTROL_DIR);
    let guard_path = control.join(VAULT_META_FILE);
    let db_path = control.join(canonical::CANONICAL_DB_FILE);
    let journal_path = control.join(journal_file_name());

    if !guard_path.exists() {
        return Err(Error::Recovery(format!(
            "no guard at {}: nothing to recover",
            guard_path.display()
        )));
    }
    if !db_path.exists() {
        return Err(Error::Recovery(format!(
            "guard present but database missing at {}: nothing to recover",
            db_path.display()
        )));
    }

    // Preserve forensic bytes BEFORE any verification or working-copy
    // manipulation touches anything derived from them. Never removed by a
    // later failure in this function.
    let preserved = control.join(format!("recovery-preserved-{}", uuid::Uuid::now_v7()));
    fs::create_dir_all(&preserved)
        .map_err(|e| Error::Recovery(format!("cannot create preservation dir: {e}")))?;
    fs::copy(&guard_path, preserved.join(VAULT_META_FILE))
        .map_err(|e| Error::Recovery(format!("cannot preserve guard: {e}")))?;
    fs::copy(&db_path, preserved.join(canonical::CANONICAL_DB_FILE))
        .map_err(|e| Error::Recovery(format!("cannot preserve database: {e}")))?;
    if journal_path.exists() {
        fs::copy(&journal_path, preserved.join(journal_file_name()))
            .map_err(|e| Error::Recovery(format!("cannot preserve journal: {e}")))?;
    }

    if fault == Some(RecoveryFaultPoint::AfterPreservation) {
        return Err(Error::Recovery("injected fault: AfterPreservation".into()));
    }

    // Build a disposable working copy from the preserved bytes (never the
    // live original) and verify it.
    let working = control.join(format!("recovery-working-{}", uuid::Uuid::now_v7()));
    let verify_outcome = (|| -> Result<canonical::RecoveryVerification> {
        fs::create_dir_all(&working)
            .map_err(|e| Error::Recovery(format!("cannot create working dir: {e}")))?;
        fs::copy(
            preserved.join(VAULT_META_FILE),
            working.join(VAULT_META_FILE),
        )
        .map_err(|e| Error::Recovery(format!("cannot stage working guard: {e}")))?;
        fs::copy(
            preserved.join(canonical::CANONICAL_DB_FILE),
            working.join(canonical::CANONICAL_DB_FILE),
        )
        .map_err(|e| Error::Recovery(format!("cannot stage working database: {e}")))?;
        let preserved_journal = preserved.join(journal_file_name());
        if preserved_journal.exists() {
            fs::copy(&preserved_journal, working.join(journal_file_name()))
                .map_err(|e| Error::Recovery(format!("cannot stage working journal: {e}")))?;
        }
        canonical::verify_recovery_candidate(&working)
    })();

    let verified = match verify_outcome {
        Ok(v) => v,
        Err(e) => {
            let _ = fs::remove_dir_all(&working);
            write_incident_manifest(
                &preserved,
                &IncidentManifest {
                    schema: "flake-recovery-incident-v1",
                    original_root: original_root.display().to_string(),
                    attempted_at: now_iso8601_placeholder(),
                    outcome: "refused",
                    recovered_root: None,
                    vault_id: None,
                    verified_transaction_head_seq: None,
                    verified_object_count: None,
                    refusal_reason: Some(format!("{e}")),
                },
            );
            // Original untouched; `preserved` remains on disk for
            // inspection — this is the "offer backup restore/partial
            // salvage labeling" clause's evidence trail, even though this
            // module does not itself build a labeled-partial-salvage UX.
            return Err(e);
        }
    };

    if fault == Some(RecoveryFaultPoint::AfterVerification) {
        let _ = fs::remove_dir_all(&working);
        return Err(Error::Recovery("injected fault: AfterVerification".into()));
    }

    // Publish: a fresh, empty access.lock (this is itself "explicit vault
    // initialization" of the recovered root, per §14 — never copied from
    // the original, which may have been mid-use when preserved), then
    // stage-verify-publish exactly like `CanonicalStore::create`.
    if let Err(e) = fs::File::create(working.join(ACCESS_LOCK_FILE)) {
        let _ = fs::remove_dir_all(&working);
        return Err(Error::Recovery(format!(
            "cannot create access lock for recovered root: {e}"
        )));
    }

    if fault == Some(RecoveryFaultPoint::BeforePublish) {
        let _ = fs::remove_dir_all(&working);
        return Err(Error::Recovery("injected fault: BeforePublish".into()));
    }

    if let Err(e) = fs::create_dir_all(&new_root) {
        let _ = fs::remove_dir_all(&working);
        return Err(Error::Recovery(format!("cannot create new root: {e}")));
    }
    if new_control.exists() {
        let _ = fs::remove_dir_all(&working);
        return Err(Error::Recovery(format!(
            "recovery destination published concurrently: {}",
            new_control.display()
        )));
    }
    if let Err(e) = fs::rename(&working, &new_control) {
        let _ = fs::remove_dir_all(&working);
        return Err(Error::Recovery(format!(
            "cannot publish recovered root to {}: {e}",
            new_control.display()
        )));
    }

    write_incident_manifest(
        &preserved,
        &IncidentManifest {
            schema: "flake-recovery-incident-v1",
            original_root: original_root.display().to_string(),
            attempted_at: now_iso8601_placeholder(),
            outcome: "verified_and_published",
            recovered_root: Some(new_root.display().to_string()),
            vault_id: Some(&verified.vault_id),
            verified_transaction_head_seq: Some(verified.transaction_head_seq),
            verified_object_count: Some(verified.object_count),
            refusal_reason: None,
        },
    );

    Ok(RecoveryReport {
        original_root,
        recovered_root: new_root,
        vault_id: verified.vault_id,
        preserved_at: preserved,
        verified_transaction_head_seq: verified.transaction_head_seq,
        verified_object_count: verified.object_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::{CanonicalStore, CommandInput, CommandTarget, RecordOrigin};
    use rusqlite::Connection;

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-recovery-{}", uuid::Uuid::now_v7()))
    }

    fn cleanup(p: &Path) {
        let _ = fs::remove_dir_all(p);
    }

    /// Build a real store with a couple of committed revisions — closer to
    /// a real recovery candidate than an empty one.
    fn seeded_store(root: &Path) -> (String, String) {
        let mut store = CanonicalStore::create(root).unwrap();
        let created = {
            let mut writer = store.writer().unwrap();
            writer
                .commit(CommandInput {
                    command_id: uuid::Uuid::now_v7().to_string(),
                    actor: "owner".into(),
                    origin: RecordOrigin::User,
                    target: CommandTarget::CreateObject {
                        payload: "seed".into(),
                    },
                })
                .unwrap()
        };
        let updated = {
            let mut writer = store.writer().unwrap();
            writer
                .commit(CommandInput {
                    command_id: uuid::Uuid::now_v7().to_string(),
                    actor: "owner".into(),
                    origin: RecordOrigin::User,
                    target: CommandTarget::UpdateObject {
                        object_id: created.object_id.clone(),
                        expected_revision_id: created.revision_id.clone(),
                        payload: "seed-v2".into(),
                    },
                })
                .unwrap()
        };
        drop(store);
        (created.object_id, updated.revision_id)
    }

    #[test]
    fn clean_store_recovers_to_new_root_with_matching_state() {
        let root = tmp();
        let (object_id, revision_id) = seeded_store(&root);

        let new_root = tmp();
        let report = recover_to_new_root(&root, &new_root).unwrap();
        assert_eq!(report.verified_transaction_head_seq, 2);
        assert_eq!(report.verified_object_count, 1);
        assert!(report.preserved_at.exists());

        let recovered = CanonicalStore::open(&new_root).unwrap();
        let (rev, payload) = recovered.read_current(&object_id).unwrap().unwrap();
        assert_eq!(rev, revision_id);
        assert_eq!(payload, "seed-v2");

        // The original is completely untouched: still opens normally, with
        // the same content, at the same location.
        let original_still_open = CanonicalStore::open(&root).unwrap();
        assert_eq!(
            original_still_open
                .read_current(&object_id)
                .unwrap()
                .unwrap()
                .1,
            "seed-v2"
        );
        cleanup(&root);
        cleanup(&new_root);
        cleanup(&report.preserved_at);
    }

    #[test]
    fn tampered_resulting_head_hash_is_irrecoverable_and_original_untouched() {
        let root = tmp();
        let (_object_id, _revision_id) = seeded_store(&root);

        {
            let raw = Connection::open(root.join(CONTROL_DIR).join(canonical::CANONICAL_DB_FILE))
                .unwrap();
            raw.execute(
                "UPDATE command SET resulting_head_hash = 'tampered' WHERE recorded_seq = 1",
                [],
            )
            .unwrap();
        }
        // Snapshot the (already-tampered) original *after* the test's own
        // tamper edit: what this test proves is that `recover_to_new_root`
        // itself never further mutates the live original — not that the
        // original is pristine (it deliberately is not).
        let original_db_bytes_after_tamper =
            fs::read(root.join(CONTROL_DIR).join(canonical::CANONICAL_DB_FILE)).unwrap();

        let new_root = tmp();
        let err = recover_to_new_root(&root, &new_root).unwrap_err();
        let msg = format!("{err}");
        assert!(
            msg.contains("recomputed value") || msg.contains("chain broken"),
            "got {msg}"
        );
        // Nothing published at the destination.
        assert!(!new_root.join(CONTROL_DIR).exists());
        // Original bytes on disk are byte-for-byte unchanged — recovery
        // read a copy, never the live file.
        let after = fs::read(root.join(CONTROL_DIR).join(canonical::CANONICAL_DB_FILE)).unwrap();
        assert_eq!(
            original_db_bytes_after_tamper, after,
            "recovery itself must never further mutate the live original"
        );
        cleanup(&root);
    }

    #[test]
    fn missing_database_with_guard_present_is_refused_before_any_preservation() {
        let root = tmp();
        fs::create_dir_all(root.join(CONTROL_DIR)).unwrap();
        fs::write(
            root.join(CONTROL_DIR).join(VAULT_META_FILE),
            r#"{"vault_id":"018f0000-0000-7000-8000-000000000000","format_version":2,"created_by_version":"x","created_at":"t"}"#,
        )
        .unwrap();
        fs::File::create(root.join(CONTROL_DIR).join(ACCESS_LOCK_FILE)).unwrap();

        let new_root = tmp();
        let err = recover_to_new_root(&root, &new_root).unwrap_err();
        assert!(format!("{err}").contains("database missing"));
        assert!(!new_root.join(CONTROL_DIR).exists());
        cleanup(&root);
    }

    #[test]
    fn destination_already_published_is_refused_without_touching_the_original() {
        let root = tmp();
        seeded_store(&root);
        let new_root = tmp();
        CanonicalStore::create(&new_root).unwrap();

        let err = recover_to_new_root(&root, &new_root).unwrap_err();
        assert!(format!("{err}").contains("already published"));
        // Original still opens fine.
        assert!(CanonicalStore::open(&root).is_ok());
        cleanup(&root);
        cleanup(&new_root);
    }

    #[test]
    fn recovery_is_refused_while_a_normal_connection_holds_shared_access() {
        let root = tmp();
        seeded_store(&root);
        let _reader = CanonicalStore::open(&root).unwrap(); // holds shared access

        let new_root = tmp();
        let err = recover_to_new_root(&root, &new_root).unwrap_err();
        assert!(format!("{err}").contains("busy"), "got {err}");
        assert!(!new_root.join(CONTROL_DIR).exists());
        cleanup(&root);
    }

    #[test]
    fn a_new_normal_open_is_blocked_while_recovery_holds_exclusive_access() {
        // Ordering proof, without ever calling the blocking `acquire_shared`
        // API while this test itself holds the exclusive lock (that would
        // deadlock the test thread against itself): take the same exclusive
        // lock `RecoveryGuard` would, then show a *non-blocking* shared
        // attempt from a separate handle is refused while it is held, and
        // succeeds immediately once released — the same contention
        // `CanonicalStore::open`'s blocking `lock_shared()` would wait on.
        let root = tmp();
        seeded_store(&root);
        let access_path = root.join(CONTROL_DIR).join(ACCESS_LOCK_FILE);

        let exclusive = fs::OpenOptions::new()
            .write(true)
            .open(&access_path)
            .unwrap();
        exclusive.try_lock().unwrap();

        let contended = fs::OpenOptions::new()
            .write(true)
            .open(&access_path)
            .unwrap();
        assert!(
            matches!(contended.try_lock_shared(), Err(TryLockError::WouldBlock)),
            "a new shared access attempt must be blocked while exclusive is held"
        );

        drop(exclusive);
        assert!(
            contended.try_lock_shared().is_ok(),
            "shared access must succeed immediately once the exclusive holder releases"
        );
        cleanup(&root);
    }

    #[test]
    fn successful_recovery_writes_a_verified_incident_manifest() {
        let root = tmp();
        seeded_store(&root);
        let new_root = tmp();

        let report = recover_to_new_root(&root, &new_root).unwrap();
        let manifest_path = report.preserved_at.join(INCIDENT_MANIFEST_FILE);
        assert!(manifest_path.exists());
        let manifest: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&manifest_path).unwrap()).unwrap();
        assert_eq!(manifest["schema"], "flake-recovery-incident-v1");
        assert_eq!(manifest["outcome"], "verified_and_published");
        assert_eq!(manifest["verified_transaction_head_seq"], 2);
        assert_eq!(manifest["verified_object_count"], 1);
        assert_eq!(manifest["vault_id"], report.vault_id);
        assert!(manifest["refusal_reason"].is_null());

        cleanup(&root);
        cleanup(&new_root);
        cleanup(&report.preserved_at);
    }

    #[test]
    fn refused_recovery_writes_a_refusal_incident_manifest_and_preserves_evidence() {
        let root = tmp();
        seeded_store(&root);
        {
            let raw = Connection::open(root.join(CONTROL_DIR).join(canonical::CANONICAL_DB_FILE))
                .unwrap();
            raw.execute(
                "UPDATE command SET resulting_head_hash = 'tampered' WHERE recorded_seq = 1",
                [],
            )
            .unwrap();
        }
        let new_root = tmp();

        let err = recover_to_new_root(&root, &new_root).unwrap_err();
        // The preservation directory is named uniquely per attempt; find it
        // to check its manifest without recovery.rs exposing it on `Err`.
        let control = root.join(CONTROL_DIR);
        let preserved_dir = fs::read_dir(&control)
            .unwrap()
            .filter_map(|e| e.ok())
            .find(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with("recovery-preserved-")
            })
            .expect("a preservation directory must exist even on refusal")
            .path();
        let manifest_path = preserved_dir.join(INCIDENT_MANIFEST_FILE);
        assert!(manifest_path.exists());
        let manifest: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&manifest_path).unwrap()).unwrap();
        assert_eq!(manifest["outcome"], "refused");
        assert!(manifest["recovered_root"].is_null());
        let reason = manifest["refusal_reason"].as_str().unwrap();
        assert!(format!("{err}").contains(reason) || reason.contains("recomputed value"));

        cleanup(&root);
    }

    // -----------------------------------------------------------------
    // T01-07 — D4 durability class: deterministic fault-schedule matrix
    // -----------------------------------------------------------------

    /// D4 ("bit corruption/missing journal/inconsistent head with safe
    /// refusal and backup recovery"): 100 genuinely distinct single-byte
    /// corruption schedules, each flipping a byte **inside a specific,
    /// located, currently-live revision payload** — not an arbitrary file
    /// offset.
    ///
    /// An earlier version of this matrix corrupted 100 offsets spread
    /// evenly across the raw file length and expected the overwhelming
    /// majority to be refused. That run found only 25–59/100 refused and
    /// was investigated rather than weakened: diagnostic output showed
    /// most "passing" offsets were either all-zero bytes or leftover text
    /// from a page SQLite's own rollback-journal-mode B-tree had already
    /// stopped referencing (dead freelist/reused-page content from the
    /// `current_object` row's 30 in-place updates during fixture setup) —
    /// genuinely inert corruption of bytes no live query path ever reads,
    /// not a missed detection. Confounding "did we fail to verify a real
    /// value" with "did the flip land on a byte SQLite itself no longer
    /// considers part of any row" made the original matrix's floor
    /// unfalsifiable in either direction. This version removes that
    /// confound entirely: each schedule locates the *exact* live byte
    /// range of one revision's stored payload (via `PRAGMA` -independent
    /// byte-string search, since SQLite stores short TEXT values as their
    /// literal UTF-8 bytes inline) and flips a byte strictly inside it,
    /// so every schedule provably targets a byte a live query actually
    /// returns — for which safe refusal is a reasonable, falsifiable
    /// expectation, closed by this task's `payload_sha256` corrective fix
    /// (see `docs/evidence/flake-v1/T01-07/REPORT.md`).
    #[test]
    fn d4_bit_corruption_fault_schedule_matrix() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let mut prev = {
            let mut writer = store.writer().unwrap();
            writer
                .commit(CommandInput {
                    command_id: uuid::Uuid::now_v7().to_string(),
                    actor: "owner".into(),
                    origin: RecordOrigin::User,
                    target: CommandTarget::CreateObject {
                        payload: "MARKER-SCHEDULE-0000-PAYLOAD".into(),
                    },
                })
                .unwrap()
        };
        // 99 further revisions, each with a unique, locatable marker —
        // 100 live payloads total (the create above is schedule 0).
        for i in 1..100 {
            let mut writer = store.writer().unwrap();
            prev = writer
                .commit(CommandInput {
                    command_id: uuid::Uuid::now_v7().to_string(),
                    actor: "owner".into(),
                    origin: RecordOrigin::User,
                    target: CommandTarget::UpdateObject {
                        object_id: prev.object_id.clone(),
                        expected_revision_id: prev.revision_id.clone(),
                        payload: format!("MARKER-SCHEDULE-{i:04}-PAYLOAD"),
                    },
                })
                .unwrap();
        }
        drop(store);

        let db_path = root.join(CONTROL_DIR).join(canonical::CANONICAL_DB_FILE);
        let original_bytes = fs::read(&db_path).unwrap();

        let candidate = tmp();
        fs::create_dir_all(&candidate).unwrap();
        fs::copy(
            root.join(CONTROL_DIR).join(VAULT_META_FILE),
            candidate.join(VAULT_META_FILE),
        )
        .unwrap();

        let mut refused = 0usize;
        for schedule_id in 0..100usize {
            let marker = format!("MARKER-SCHEDULE-{schedule_id:04}-PAYLOAD");
            let needle = marker.as_bytes();
            // A B-tree page split during the 99 sequential inserts above
            // can leave a stale, no-longer-referenced copy of an earlier
            // page's bytes behind (SQLite does not zero freed space), so
            // the marker text can legitimately appear more than once in
            // the raw file — at most one occurrence is the live one a
            // query actually returns. Flip a byte inside *every*
            // occurrence found, so the live copy is corrupted regardless
            // of how many stale ghosts of it also exist on disk.
            let positions: Vec<usize> = original_bytes
                .windows(needle.len())
                .enumerate()
                .filter(|(_, w)| *w == needle)
                .map(|(i, _)| i)
                .collect();
            assert!(
                !positions.is_empty(),
                "schedule {schedule_id}: marker {marker:?} not found anywhere in the database file"
            );

            let mut corrupted = original_bytes.clone();
            for pos in &positions {
                corrupted[pos + needle.len() / 2] ^= 0xFF;
            }
            fs::write(candidate.join(canonical::CANONICAL_DB_FILE), &corrupted).unwrap();

            match canonical::verify_recovery_candidate(&candidate) {
                Err(_) => refused += 1,
                Ok(_) => panic!(
                    "schedule {schedule_id}: corrupting a byte inside a located, live revision \
                     payload must be refused, not silently admitted"
                ),
            }
        }

        assert_eq!(
            refused, 100,
            "D4 schedule matrix: every one of 100 live-payload-byte corruptions must be refused"
        );

        cleanup(&root);
        cleanup(&candidate);
    }

    // -----------------------------------------------------------------
    // T01-07 — D3 durability class: deterministic fault-schedule matrix
    // -----------------------------------------------------------------

    /// D3 ("competing opens, relocation and lock release after crash"): 50
    /// revision depths x 2 real, structurally distinct contention types =
    /// 100 genuinely distinct deterministic schedules:
    ///
    /// - writer-vs-writer: a second independently-opened `CanonicalStore`
    ///   handle's `writer()` call while the first writer is held.
    /// - normal-open-vs-recovery: a held normal (shared-access) handle
    ///   blocking `recover_to_new_root`'s exclusive-access attempt.
    ///
    /// Sweeping revision depth is not padding: both contention checks are
    /// OS-lock-level and depth-independent in principle, but exercising
    /// them against a genuinely growing, real store — not a fixed toy
    /// fixture — is what proves the invariant holds at every size actually
    /// reached during the test, not only at one arbitrarily chosen size.
    /// Every schedule additionally proves the *released* side: the losing
    /// contender succeeds immediately once the holder is dropped.
    #[test]
    fn d3_competing_opens_fault_schedule_matrix() {
        let mut schedule_id = 0usize;
        for depth in 0..50usize {
            // --- Type A: writer-vs-writer ---
            schedule_id += 1;
            {
                let root = tmp();
                let mut store = CanonicalStore::create(&root).unwrap();
                let mut prev = {
                    let mut writer = store.writer().unwrap();
                    writer
                        .commit(CommandInput {
                            command_id: uuid::Uuid::now_v7().to_string(),
                            actor: "owner".into(),
                            origin: RecordOrigin::User,
                            target: CommandTarget::CreateObject {
                                payload: "seed".into(),
                            },
                        })
                        .unwrap()
                };
                for i in 0..depth {
                    let mut writer = store.writer().unwrap();
                    prev = writer
                        .commit(CommandInput {
                            command_id: uuid::Uuid::now_v7().to_string(),
                            actor: "owner".into(),
                            origin: RecordOrigin::User,
                            target: CommandTarget::UpdateObject {
                                object_id: prev.object_id.clone(),
                                expected_revision_id: prev.revision_id.clone(),
                                payload: format!("depth-{i}"),
                            },
                        })
                        .unwrap();
                }

                let held_writer = store.writer().unwrap();
                let mut second_handle = CanonicalStore::open(&root).unwrap();
                let err = second_handle.writer().unwrap_err();
                assert!(
                    matches!(err, Error::WriterLocked { .. }),
                    "schedule {schedule_id} (depth {depth}, writer-vs-writer): {err}"
                );
                drop(held_writer);
                // Released: the loser succeeds immediately once the holder drops.
                assert!(
                    second_handle.writer().is_ok(),
                    "schedule {schedule_id} (depth {depth}): writer must succeed once released"
                );
                cleanup(&root);
            }

            // --- Type B: normal-open-vs-recovery-exclusive-access ---
            schedule_id += 1;
            {
                let root = tmp();
                let mut store = CanonicalStore::create(&root).unwrap();
                let mut prev = {
                    let mut writer = store.writer().unwrap();
                    writer
                        .commit(CommandInput {
                            command_id: uuid::Uuid::now_v7().to_string(),
                            actor: "owner".into(),
                            origin: RecordOrigin::User,
                            target: CommandTarget::CreateObject {
                                payload: "seed".into(),
                            },
                        })
                        .unwrap()
                };
                for i in 0..depth {
                    let mut writer = store.writer().unwrap();
                    prev = writer
                        .commit(CommandInput {
                            command_id: uuid::Uuid::now_v7().to_string(),
                            actor: "owner".into(),
                            origin: RecordOrigin::User,
                            target: CommandTarget::UpdateObject {
                                object_id: prev.object_id.clone(),
                                expected_revision_id: prev.revision_id.clone(),
                                payload: format!("depth-{i}"),
                            },
                        })
                        .unwrap();
                }
                drop(store);

                let held_reader = CanonicalStore::open(&root).unwrap();
                let new_root = tmp();
                let err = recover_to_new_root(&root, &new_root).unwrap_err();
                assert!(
                    format!("{err}").contains("busy"),
                    "schedule {schedule_id} (depth {depth}, open-vs-recovery): {err}"
                );
                assert!(!new_root.join(CONTROL_DIR).exists());
                drop(held_reader);
                // Released: recovery succeeds immediately once the reader drops.
                let report = recover_to_new_root(&root, &new_root).unwrap();
                assert_eq!(report.verified_transaction_head_seq as usize, depth + 1);
                cleanup(&root);
                cleanup(&new_root);
                cleanup(&report.preserved_at);
            }
        }
        assert_eq!(
            schedule_id, 100,
            "D3 schedule matrix must run exactly 100 schedules"
        );
    }

    /// D1/D4 checklist item — "second crash injected during recovery
    /// itself does not corrupt or lose the preserved original": a fault
    /// at each of `recover_to_new_root`'s own internal stages (distinct
    /// from corrupting the source *before* recovery starts, which
    /// `d4_bit_corruption_fault_schedule_matrix` already covers) must
    /// leave both the live original and the already-preserved forensic
    /// copy completely intact.
    #[test]
    fn second_crash_during_recovery_itself_never_loses_the_preserved_original() {
        for fault in [
            RecoveryFaultPoint::AfterPreservation,
            RecoveryFaultPoint::AfterVerification,
            RecoveryFaultPoint::BeforePublish,
        ] {
            let root = tmp();
            seeded_store(&root);
            let original_bytes_before =
                fs::read(root.join(CONTROL_DIR).join(canonical::CANONICAL_DB_FILE)).unwrap();

            let new_root = tmp();
            let err = recover_to_new_root_with_fault(&root, &new_root, Some(fault)).unwrap_err();
            assert!(
                format!("{err}").contains("injected fault"),
                "{fault:?}: {err}"
            );
            assert!(
                !new_root.join(CONTROL_DIR).exists(),
                "{fault:?}: nothing must be published"
            );

            // The live original is byte-for-byte unchanged.
            let original_bytes_after =
                fs::read(root.join(CONTROL_DIR).join(canonical::CANONICAL_DB_FILE)).unwrap();
            assert_eq!(
                original_bytes_before, original_bytes_after,
                "{fault:?}: live original must be untouched by a crash during recovery"
            );

            // The forensic preservation directory (created before any of
            // these fault points can fire) survived and still verifies.
            let preserved_dir = fs::read_dir(root.join(CONTROL_DIR))
                .unwrap()
                .filter_map(|e| e.ok())
                .find(|e| {
                    e.file_name()
                        .to_string_lossy()
                        .starts_with("recovery-preserved-")
                })
                .unwrap_or_else(|| panic!("{fault:?}: preserved directory must survive"))
                .path();
            assert!(canonical::verify_recovery_candidate(&preserved_dir).is_ok());

            cleanup(&root);
            cleanup(&new_root);
        }
    }
}
