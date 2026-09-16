//! Create and restore consistent verified backups of the format-2 canonical
//! store, using SQLite's own Online Backup API (`T01-05`).
//!
//! **Why not a raw file copy?** "Copying a live database file is not a
//! consistent backup" (this task's own rationale): if another connection
//! commits mid-copy, a plain `fs::copy` can capture a torn mix of old and
//! new pages. SQLite's Online Backup API (`rusqlite::backup`, the `backup`
//! Cargo feature — see "Dependency note" below) is specifically designed to
//! produce a page-consistent snapshot of a live database even while other
//! connections keep reading and writing it.
//!
//! **How this differs from `crate::recovery`.** Recovery (`T01-04`) exists
//! to rescue a possibly-*damaged* live root by copying its raw bytes and
//! verifying a working copy — it deliberately never assumes the source
//! database is even openable. Backup (this module) exists to make an
//! owner-controlled, independently verified copy of a *healthy* vault for
//! safekeeping, using the engine's own consistency guarantee instead of a
//! raw byte copy. Both ultimately call the same
//! [`crate::canonical::verify_recovery_candidate`] before trusting anything
//! they produced — a backup is not "verified" on the strength of the
//! Online Backup API alone.
//!
//! **Dependency note (admission).** `rusqlite`'s `backup` Cargo feature is
//! newly enabled in `Cargo.toml` (previously only `bundled` was). This is
//! not a new dependency: same crate, same version (`0.37.0`), same MIT
//! license already admitted at `T00-02`/`T01-02`. The feature only compiles
//! in a thin, safe Rust wrapper (`rusqlite::backup::Backup`) over
//! `sqlite3_backup_init`/`_step`/`_finish` — core SQLite C API functions
//! already present in the vendored amalgamation regardless of this Cargo
//! feature, so enabling it adds no new C code, no new transitive
//! dependency, and no license change.
//!
//! **Not implemented here** (recorded, not silently dropped): a CLI
//! `backup`/`restore` command (this store has no CLI wiring at all yet,
//! consistent with `T01-03`/`T01-04`'s own recorded scope boundary); a
//! *selected-project* backup (plan §16's "shareable project package" —
//! this task only builds `kind: "full-backup"`, matching the task's own
//! objective "complete saved work"); and literal disk-full/OS-resource
//! exhaustion testing (this module's fault-injection test uses the same
//! deterministic in-process approach every prior `flake-v1` task has used
//! instead of a real resource-exhaustion harness).

use crate::canonical::{self, CanonicalStore};
use crate::vault::{ACCESS_LOCK_FILE, CONTROL_DIR, VAULT_META_FILE};
use crate::{Error, Result};
use rusqlite::backup::{Backup, StepResult};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const BACKUP_MANIFEST_FILE: &str = "backup-manifest.json";

/// One published member's exact recorded length/digest — checked again,
/// independently, before every restore (never trust a backup blindly
/// either, even one this module itself produced).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManifestMember {
    pub path: String,
    pub length: u64,
    pub sha256: String,
}

/// §15 "Export/backup manifest": version, kind, vault snapshot head, member
/// paths/lengths/digests. `snapshot_head_seq`/`snapshot_head_hash` are
/// captured from the source *before* the backup step loop begins — "record
/// snapshot head and later backup result separately" — so the manifest
/// states what was *intended* to be captured, independent of `verified`,
/// which records whether the produced bytes actually passed independent
/// verification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BackupManifest {
    pub schema: String,
    pub kind: String,
    pub vault_id: String,
    pub snapshot_head_seq: i64,
    pub snapshot_head_hash: Option<String>,
    pub created_at: String,
    pub members: Vec<ManifestMember>,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BackupReport {
    pub source_root: PathBuf,
    pub backup_root: PathBuf,
    pub manifest: BackupManifest,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RestoreReport {
    pub backup_root: PathBuf,
    pub restored_root: PathBuf,
    pub vault_id: String,
    pub restored_transaction_head_seq: i64,
    pub restored_object_count: usize,
}

fn now_iso8601_placeholder() -> String {
    // Same documented limitation as canonical.rs/vault.rs/recovery.rs: no
    // chrono/time dependency admitted; distinct, sortable, RFC3339-shaped,
    // not calendar-correct.
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("2026-09-15T{:05}Z", secs % 86400)
}

fn manifest_member(dir: &Path, name: &str) -> Result<ManifestMember> {
    let bytes = fs::read(dir.join(name))
        .map_err(|e| Error::Backup(format!("cannot read {name} for manifest: {e}")))?;
    Ok(ManifestMember {
        path: name.to_string(),
        length: bytes.len() as u64,
        sha256: crate::events::hash_bytes(&bytes),
    })
}

/// Create a consistent, independently verified full backup of the format-2
/// store at `source_root`, published at `backup_root`.
///
/// `should_cancel` is checked between every backup step; returning `true`
/// cancels before any publication — the staging directory is removed and
/// `Err` is returned, leaving `backup_root` with no `.fehrest` at all
/// ("cancellation before publication leaves explicit incomplete output").
/// Pass `|| false` for no cancellation support.
pub fn backup_to_new_root(
    source_root: impl AsRef<Path>,
    backup_root: impl AsRef<Path>,
    mut should_cancel: impl FnMut() -> bool,
) -> Result<BackupReport> {
    let source_root = source_root.as_ref().to_path_buf();
    let backup_root = backup_root.as_ref().to_path_buf();

    let backup_control = backup_root.join(CONTROL_DIR);
    if backup_control.exists() {
        return Err(Error::Backup(format!(
            "backup destination already published: {}",
            backup_control.display()
        )));
    }

    // A normal, shared-access open: backup is a read-like operation, not
    // recovery. It coexists with other readers and is itself excluded while
    // `crate::recovery` holds exclusive access, exactly like any other
    // normal connection.
    let source = CanonicalStore::open(&source_root)?;
    let (snapshot_head_seq, snapshot_head_hash) = source.transaction_head()?;
    let vault_id = source.vault_id().to_string();

    fs::create_dir_all(&backup_root)
        .map_err(|e| Error::Backup(format!("cannot create backup root: {e}")))?;
    let staging = backup_root.join(format!(".fehrest.staging-{}", uuid::Uuid::now_v7()));
    fs::create_dir_all(&staging)
        .map_err(|e| Error::Backup(format!("cannot create staging dir: {e}")))?;

    if let Err(e) = fs::copy(
        source.control_dir().join(VAULT_META_FILE),
        staging.join(VAULT_META_FILE),
    ) {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup(format!("cannot copy guard: {e}")));
    }

    // The consistent snapshot itself.
    let db_dest_path = staging.join(canonical::CANONICAL_DB_FILE);
    let cancelled = {
        let mut dest_conn = match Connection::open(&db_dest_path) {
            Ok(c) => c,
            Err(e) => {
                let _ = fs::remove_dir_all(&staging);
                return Err(Error::Backup(format!("cannot create backup database: {e}")));
            }
        };
        if let Err(e) = dest_conn.busy_timeout(std::time::Duration::from_secs(2)) {
            let _ = fs::remove_dir_all(&staging);
            return Err(Error::Backup(format!("cannot set busy_timeout: {e}")));
        }
        let step_result = (|| -> Result<bool> {
            let bk = Backup::new(source.connection(), &mut dest_conn)
                .map_err(|e| Error::Backup(format!("cannot start backup: {e}")))?;
            loop {
                if should_cancel() {
                    return Ok(true);
                }
                match bk.step(100) {
                    Ok(StepResult::Done) => return Ok(false),
                    Ok(StepResult::More) | Ok(StepResult::Busy) | Ok(StepResult::Locked) => {
                        continue
                    }
                    Ok(other) => {
                        return Err(Error::Backup(format!(
                            "unexpected backup step result: {other:?}"
                        )))
                    }
                    Err(e) => return Err(Error::Backup(format!("backup step failed: {e}"))),
                }
            }
        })();
        // `dest_conn` (and the `Backup` borrowing it) both drop here,
        // before this database is opened again independently below.
        match step_result {
            Ok(c) => c,
            Err(e) => {
                let _ = fs::remove_dir_all(&staging);
                return Err(e);
            }
        }
    };

    if cancelled {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup("backup cancelled before publication".into()));
    }

    // Independent verification — never trust the connection that wrote the
    // snapshot to also certify it, same discipline as `CanonicalStore::create`
    // and `crate::recovery`.
    if let Err(e) = canonical::verify_recovery_candidate(&staging) {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup(format!("backup verification failed: {e}")));
    }

    let members = vec![
        match manifest_member(&staging, VAULT_META_FILE) {
            Ok(m) => m,
            Err(e) => {
                let _ = fs::remove_dir_all(&staging);
                return Err(e);
            }
        },
        match manifest_member(&staging, canonical::CANONICAL_DB_FILE) {
            Ok(m) => m,
            Err(e) => {
                let _ = fs::remove_dir_all(&staging);
                return Err(e);
            }
        },
    ];

    let manifest = BackupManifest {
        schema: "flake-backup-manifest-v1".to_string(),
        kind: "full-backup".to_string(),
        vault_id,
        snapshot_head_seq,
        snapshot_head_hash,
        created_at: now_iso8601_placeholder(),
        members,
        verified: true,
    };
    let manifest_json = match serde_json::to_string_pretty(&manifest) {
        Ok(j) => j,
        Err(e) => {
            let _ = fs::remove_dir_all(&staging);
            return Err(Error::Backup(format!("cannot serialize manifest: {e}")));
        }
    };
    if let Err(e) = fs::write(staging.join(BACKUP_MANIFEST_FILE), manifest_json) {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup(format!("cannot write manifest: {e}")));
    }

    // A fresh, empty access.lock — explicit initialization of the new
    // backup root, never copied from the source (mirrors T01-02/T01-04).
    if let Err(e) = fs::File::create(staging.join(ACCESS_LOCK_FILE)) {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup(format!("cannot create access lock: {e}")));
    }

    if backup_control.exists() {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup(format!(
            "backup destination published concurrently: {}",
            backup_control.display()
        )));
    }
    if let Err(e) = fs::rename(&staging, &backup_control) {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup(format!(
            "cannot publish backup to {}: {e}",
            backup_control.display()
        )));
    }

    Ok(BackupReport {
        source_root,
        backup_root,
        manifest,
    })
}

/// Restore a backup produced by [`backup_to_new_root`] to a fresh
/// `new_root`. Never restores in place: `new_root` must not already have a
/// published store.
///
/// Verifies, in order: every manifest member's exact recorded length/digest
/// against the backup's actual bytes; the backup database's own internal
/// consistency ([`canonical::verify_recovery_candidate`], the same
/// from-scratch chain recomputation `crate::recovery` uses); and that the
/// manifest's claimed snapshot identity/head matches what the database
/// itself actually contains. Only then is the restored root published.
///
/// No derived index is ever restored: format-2 has no derived-state store
/// yet (I06/§16 — derived state is disposable and never treated as
/// canonical), so there is nothing to invalidate beyond simply never
/// copying one into existence here.
pub fn restore_from_backup(
    backup_root: impl AsRef<Path>,
    new_root: impl AsRef<Path>,
) -> Result<RestoreReport> {
    let backup_root = backup_root.as_ref().to_path_buf();
    let new_root = new_root.as_ref().to_path_buf();
    let backup_control = backup_root.join(CONTROL_DIR);
    let new_control = new_root.join(CONTROL_DIR);

    if new_control.exists() {
        return Err(Error::Backup(format!(
            "restore destination already published: {}",
            new_control.display()
        )));
    }

    let manifest_path = backup_control.join(BACKUP_MANIFEST_FILE);
    let manifest_text = fs::read_to_string(&manifest_path).map_err(|e| {
        Error::Backup(format!(
            "cannot read backup manifest at {}: {e}",
            manifest_path.display()
        ))
    })?;
    let manifest: BackupManifest = serde_json::from_str(&manifest_text)
        .map_err(|e| Error::Backup(format!("backup manifest corrupt: {e}")))?;

    for member in &manifest.members {
        let path = backup_control.join(&member.path);
        let bytes = fs::read(&path).map_err(|e| {
            Error::Backup(format!("cannot read backup member {}: {e}", member.path))
        })?;
        if bytes.len() as u64 != member.length {
            return Err(Error::Backup(format!(
                "backup member {} length mismatch: manifest says {}, found {}",
                member.path,
                member.length,
                bytes.len()
            )));
        }
        let digest = crate::events::hash_bytes(&bytes);
        if digest != member.sha256 {
            return Err(Error::Backup(format!(
                "backup member {} digest mismatch: manifest says {}, found {digest}",
                member.path, member.sha256
            )));
        }
    }

    let verified = canonical::verify_recovery_candidate(&backup_control)
        .map_err(|e| Error::Backup(format!("backup verification failed: {e}")))?;
    if verified.vault_id != manifest.vault_id {
        return Err(Error::Backup(format!(
            "backup manifest vault_id ({}) does not match database vault_id ({})",
            manifest.vault_id, verified.vault_id
        )));
    }
    if verified.transaction_head_seq != manifest.snapshot_head_seq
        || verified.transaction_head_hash != manifest.snapshot_head_hash
    {
        return Err(Error::Backup(format!(
            "backup manifest head ({}, {:?}) does not match the database's actual head ({}, {:?})",
            manifest.snapshot_head_seq,
            manifest.snapshot_head_hash,
            verified.transaction_head_seq,
            verified.transaction_head_hash
        )));
    }

    fs::create_dir_all(&new_root)
        .map_err(|e| Error::Backup(format!("cannot create new root: {e}")))?;
    let staging = new_root.join(format!(".fehrest.staging-{}", uuid::Uuid::now_v7()));
    fs::create_dir_all(&staging)
        .map_err(|e| Error::Backup(format!("cannot create staging dir: {e}")))?;
    if let Err(e) = fs::copy(
        backup_control.join(VAULT_META_FILE),
        staging.join(VAULT_META_FILE),
    ) {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup(format!("cannot stage restored guard: {e}")));
    }
    if let Err(e) = fs::copy(
        backup_control.join(canonical::CANONICAL_DB_FILE),
        staging.join(canonical::CANONICAL_DB_FILE),
    ) {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup(format!(
            "cannot stage restored database: {e}"
        )));
    }
    if let Err(e) = fs::File::create(staging.join(ACCESS_LOCK_FILE)) {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup(format!("cannot create access lock: {e}")));
    }

    if new_control.exists() {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup(format!(
            "restore destination published concurrently: {}",
            new_control.display()
        )));
    }
    if let Err(e) = fs::rename(&staging, &new_control) {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Backup(format!(
            "cannot publish restored root to {}: {e}",
            new_control.display()
        )));
    }

    Ok(RestoreReport {
        backup_root,
        restored_root: new_root,
        vault_id: verified.vault_id,
        restored_transaction_head_seq: verified.transaction_head_seq,
        restored_object_count: verified.object_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::{CommandInput, CommandTarget, RecordOrigin};

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-backup-{}", uuid::Uuid::now_v7()))
    }

    fn cleanup(p: &Path) {
        let _ = fs::remove_dir_all(p);
    }

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
    fn backup_then_restore_round_trips_exact_state() {
        let root = tmp();
        let (object_id, revision_id) = seeded_store(&root);

        let backup_root = tmp();
        let report = backup_to_new_root(&root, &backup_root, || false).unwrap();
        assert!(report.manifest.verified);
        assert_eq!(report.manifest.snapshot_head_seq, 2);
        assert_eq!(report.manifest.members.len(), 2);

        let restored_root = tmp();
        let restore = restore_from_backup(&backup_root, &restored_root).unwrap();
        assert_eq!(restore.restored_transaction_head_seq, 2);
        assert_eq!(restore.restored_object_count, 1);

        let restored = CanonicalStore::open(&restored_root).unwrap();
        let (rev, payload) = restored.read_current(&object_id).unwrap().unwrap();
        assert_eq!(rev, revision_id);
        assert_eq!(payload, "seed-v2");

        // Source untouched.
        let original = CanonicalStore::open(&root).unwrap();
        assert_eq!(original.transaction_head().unwrap().0, 2);

        cleanup(&root);
        cleanup(&backup_root);
        cleanup(&restored_root);
    }

    #[test]
    fn backup_destination_no_clobber() {
        let root = tmp();
        seeded_store(&root);
        let backup_root = tmp();
        backup_to_new_root(&root, &backup_root, || false).unwrap();

        let err = backup_to_new_root(&root, &backup_root, || false).unwrap_err();
        assert!(format!("{err}").contains("already published"));
        cleanup(&root);
        cleanup(&backup_root);
    }

    #[test]
    fn cancellation_before_publication_leaves_no_published_backup() {
        let root = tmp();
        seeded_store(&root);
        let backup_root = tmp();

        let mut calls = 0;
        let err = backup_to_new_root(&root, &backup_root, || {
            calls += 1;
            true // cancel on the very first check
        })
        .unwrap_err();
        assert!(format!("{err}").contains("cancelled"));
        assert!(
            !backup_root.join(CONTROL_DIR).exists(),
            "a cancelled backup must publish nothing"
        );
        assert!(calls >= 1);
        cleanup(&root);
        cleanup(&backup_root);
    }

    #[test]
    fn restore_refuses_when_a_member_digest_does_not_match_manifest() {
        let root = tmp();
        seeded_store(&root);
        let backup_root = tmp();
        backup_to_new_root(&root, &backup_root, || false).unwrap();

        // Corrupt the published backup database after the fact, preserving
        // its exact length so this test isolates the digest check from the
        // (separately real, fail-fast-first) length check.
        let db_path = backup_root
            .join(CONTROL_DIR)
            .join(canonical::CANONICAL_DB_FILE);
        let mut bytes = fs::read(&db_path).unwrap();
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF;
        fs::write(&db_path, &bytes).unwrap();

        let restored_root = tmp();
        let err = restore_from_backup(&backup_root, &restored_root).unwrap_err();
        assert!(format!("{err}").contains("digest mismatch"), "got {err}");
        assert!(!restored_root.join(CONTROL_DIR).exists());
        cleanup(&root);
        cleanup(&backup_root);
    }

    #[test]
    fn restore_refuses_when_manifest_head_disagrees_with_database() {
        let root = tmp();
        seeded_store(&root);
        let backup_root = tmp();
        backup_to_new_root(&root, &backup_root, || false).unwrap();

        // Hand-edit the published manifest's claimed head, leaving the
        // (still internally self-consistent) database bytes untouched.
        let manifest_path = backup_root.join(CONTROL_DIR).join(BACKUP_MANIFEST_FILE);
        let mut manifest: BackupManifest =
            serde_json::from_str(&fs::read_to_string(&manifest_path).unwrap()).unwrap();
        manifest.snapshot_head_seq = 999;
        fs::write(
            &manifest_path,
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        // The manifest members list is unchanged, so the digest check
        // above still passes; this test targets the head cross-check.
        // Recompute the manifest's own member digests so this test
        // isolates the head check rather than tripping the digest check.
        let db_path = backup_root
            .join(CONTROL_DIR)
            .join(canonical::CANONICAL_DB_FILE);
        let db_bytes = fs::read(&db_path).unwrap();
        for m in &mut manifest.members {
            if m.path == canonical::CANONICAL_DB_FILE {
                m.sha256 = crate::events::hash_bytes(&db_bytes);
                m.length = db_bytes.len() as u64;
            }
        }
        fs::write(
            &manifest_path,
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();

        let restored_root = tmp();
        let err = restore_from_backup(&backup_root, &restored_root).unwrap_err();
        assert!(format!("{err}").contains("does not match the database's actual head"));
        assert!(!restored_root.join(CONTROL_DIR).exists());
        cleanup(&root);
        cleanup(&backup_root);
    }

    #[test]
    fn concurrent_legitimate_commits_during_backup_do_not_produce_mixed_state() {
        // Deterministic interleaving proof of SQLite's own backup-page
        // consistency guarantee: step the backup one page at a time, with
        // exactly one real, independent write committed on a *separate*
        // connection partway through. The published backup must still be
        // fully self-consistent (integrity_check, full chain
        // recomputation) — never a torn mix of old and new pages.
        //
        // Only one interleaved write, and both connections carry a
        // `busy_timeout`: SQLite's rollback-journal locking means a writer's
        // commit and an in-progress backup step can briefly conflict
        // (`SQLITE_BUSY`); without a bounded retry on both sides, repeated
        // interleaving can livelock two connections that each immediately
        // retry against the other. A hard iteration cap below turns any
        // remaining risk into a fast, explicit test failure instead of a
        // hang.
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let created = {
            let mut writer = store.writer().unwrap();
            writer
                .commit(CommandInput {
                    command_id: uuid::Uuid::now_v7().to_string(),
                    actor: "owner".into(),
                    origin: RecordOrigin::User,
                    target: CommandTarget::CreateObject {
                        payload: "before-backup".into(),
                    },
                })
                .unwrap()
        };

        let source = CanonicalStore::open(&root).unwrap();
        let backup_root = tmp();
        fs::create_dir_all(&backup_root).unwrap();
        let staging = backup_root.join(".fehrest.staging-test");
        fs::create_dir_all(&staging).unwrap();
        fs::copy(
            root.join(CONTROL_DIR).join(VAULT_META_FILE),
            staging.join(VAULT_META_FILE),
        )
        .unwrap();
        let mut dest = Connection::open(staging.join(canonical::CANONICAL_DB_FILE)).unwrap();
        dest.busy_timeout(std::time::Duration::from_secs(2))
            .unwrap();
        {
            let bk = Backup::new(source.connection(), &mut dest).unwrap();
            let mut steps: u32 = 0;
            let mut interleaved = false;
            loop {
                steps += 1;
                assert!(
                    steps < 100_000,
                    "backup step loop exceeded a sane iteration bound; treat as a failure, not a hang"
                );
                match bk.step(1).unwrap() {
                    StepResult::Done => break,
                    StepResult::More | StepResult::Busy | StepResult::Locked => {
                        if !interleaved && steps == 2 {
                            interleaved = true;
                            let mut writer = store.writer().unwrap();
                            writer
                                .commit(CommandInput {
                                    command_id: uuid::Uuid::now_v7().to_string(),
                                    actor: "owner".into(),
                                    origin: RecordOrigin::User,
                                    target: CommandTarget::UpdateObject {
                                        object_id: created.object_id.clone(),
                                        expected_revision_id: created.revision_id.clone(),
                                        payload: "during-backup".into(),
                                    },
                                })
                                .unwrap();
                        }
                        continue;
                    }
                    other => panic!("unexpected: {other:?}"),
                }
            }
        }
        drop(dest);

        let verified = canonical::verify_recovery_candidate(&staging)
            .expect("a backup taken during a concurrent commit must still verify cleanly");
        // The backup captured *some* consistent point in time — not
        // necessarily the very latest commit, since SQLite's backup API is
        // allowed to restart/re-copy pages that changed mid-backup. What
        // matters is that it is a real, internally consistent state that
        // actually existed, not a mix that never existed.
        assert!(verified.transaction_head_seq >= 1);

        cleanup(&root);
        cleanup(&backup_root);
    }

    // -----------------------------------------------------------------
    // T01-07 — D5 durability class: deterministic fault-schedule matrix
    // -----------------------------------------------------------------

    /// D5 ("interrupted backup/import/migration/export publication"): 100
    /// genuinely distinct cancellation schedules, each stopping
    /// `backup_to_new_root` at a different real backup-step count (1
    /// through 100) against a source large enough to have well over 100
    /// actual SQLite pages to copy. Every schedule is a full, independent
    /// invocation — not a single run inspected 100 ways — and every one
    /// must publish nothing at all.
    #[test]
    fn d5_backup_cancellation_fault_schedule_matrix() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        // The production backup step loop copies up to 100 pages
        // (`Backup::step(100)`) per `should_cancel` check, so reaching 100
        // *genuinely distinct* cancellation points requires the database
        // to need more than 100 such calls to finish — over ~9,900 pages
        // (~40.5 MiB at this store's fixed 4096-byte page size). 42
        // separate ~1 MiB payloads (just under the per-object limit)
        // reach that comfortably without needing 100+ commit() calls.
        let big_payload = |tag: &str| {
            let filler = "x".repeat(1_000_000 - tag.len());
            format!("{tag}{filler}") // exactly ~1,000,000 bytes, under the 1 MiB limit
        };
        for i in 0..48 {
            let mut writer = store.writer().unwrap();
            writer
                .commit(CommandInput {
                    command_id: uuid::Uuid::now_v7().to_string(),
                    actor: "owner".into(),
                    origin: RecordOrigin::User,
                    target: CommandTarget::CreateObject {
                        payload: big_payload(&format!("obj{i}-")),
                    },
                })
                .unwrap();
        }
        drop(store);

        let db_size = fs::metadata(root.join(CONTROL_DIR).join(canonical::CANONICAL_DB_FILE))
            .unwrap()
            .len();
        assert!(
            db_size > 9_901 * 4096,
            "fixture must exceed 9,901 real SQLite pages so 100 cancel-at-step values are all \
             genuinely distinct checks, got {db_size} bytes ({} pages)",
            db_size / 4096
        );

        for cancel_at_step in 1..=100u32 {
            let backup_root = tmp();
            let mut steps_taken = 0u32;
            let err = backup_to_new_root(&root, &backup_root, || {
                steps_taken += 1;
                steps_taken >= cancel_at_step
            })
            .unwrap_err();
            assert!(
                format!("{err}").contains("cancelled"),
                "schedule cancel_at_step={cancel_at_step}: got {err}"
            );
            assert!(
                !backup_root.join(CONTROL_DIR).exists(),
                "schedule cancel_at_step={cancel_at_step}: a cancelled backup must publish nothing"
            );
            cleanup(&backup_root);
        }

        // The uninterrupted operation still works after 100 interrupted
        // attempts against the same, untouched source.
        let backup_root = tmp();
        let report = backup_to_new_root(&root, &backup_root, || false).unwrap();
        assert!(report.manifest.verified);

        cleanup(&root);
        cleanup(&backup_root);
    }
}
