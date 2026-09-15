//! `SourceCheck`: append-only observation of whether a previously admitted
//! `Source`'s selected local file still matches, has changed, disappeared
//! or became unreadable (`T03-01`).
//!
//! **Architecture.** Exactly like `Relation` (`T02-03`), a `SourceCheck` is
//! a seventh [`crate::project::RecordPayload`] kind — an ordinary
//! opaque-payload canonical object committed through the existing
//! `CommandTarget::CreateObject`. No new `CommandTarget` variant, no schema
//! change. Per §15's own "Source check" row: "Append-only observation, not
//! a rewritten revision. Check result says what was observed, not that the
//! source remains unchanged forever" — `check_source` below therefore never
//! mutates the `Source` object it inspects; it only ever creates a brand
//! new `SourceCheck` object, one per invocation, permanently.
//!
//! **The locator.** Until this task, `capture::import_file` always left
//! `Source::claimed_path` at `None` for a `SourceKind::File` source — there
//! was nothing yet that needed to reopen the original path. This task
//! populates it (see `capture.rs`'s `import_file`) with "the owner-selected
//! locator hint" §15 already names as part of `Source`'s own row. It
//! remains exactly as non-authoritative as `claimed_repository`/
//! `claimed_commit` already were: a hint of where to look on *this*
//! machine, never filesystem/read authority on its own, and never
//! re-followed automatically — every function in this module that opens a
//! path does so only in direct, synchronous response to one explicit
//! caller invocation (`source-check`/`source-reselect`/
//! `source-admit-change`), never a background watcher (a "watcher" is
//! explicitly forbidden by this task's own contract).
//!
//! **Why `observe` does not reuse `capture::capture_file` (deliberate
//! non-reuse, not an oversight).** `capture_file` already exists,
//! already-audited, and already asserted against by `T02-02`'s own merged
//! tests for its *exact* error message text ("missing or unreachable",
//! "symlink not followed (S03)", "not a regular file", "exceeds limit").
//! This module's own classification need is materially different: a
//! recheck must distinguish **Missing** from **Denied**, which
//! `capture_file` was never designed to report (it only ever needs one
//! outcome, "refuse the whole import"). Reusing it here would mean either
//! changing its return type (risking every one of those already-audited
//! call sites and their pinned error text) or string-matching its error
//! messages after the fact (fragile, and exactly the kind of coupling this
//! crate's own `AGENTS.md` §6 discourages). `relation.rs` already
//! documents this identical tradeoff for `temporal::validate_supersession`
//! ("reuse the *algorithm shape*, not the code, when the surrounding
//! contract differs") — `observe` below is that same choice applied here:
//! the same symlink-then-regular-file-then-bounded-read shape
//! `capture_file` uses, reimplemented standalone with its own outcome
//! taxonomy. `capture::capture_file` itself *is* reused, unmodified, by
//! `admit_changed_source` below, where the contract (build a brand new,
//! fully-defended `Capture`) is identical to import's own.
//!
//! **Security boundary (S03/S07).** No path other than the source's own
//! already-recorded locator (`check_source`) or one explicit
//! caller-supplied path (`reselect_source`/`admit_changed_source`'s
//! `path_override`) is ever opened — no directory walk, no glob, no `..`
//! traversal resolution beyond what the OS's own `symlink_metadata`/`open`
//! already perform on the exact string given. A symlink at the checked
//! path — including one that silently replaced a previously-regular file
//! after admission — is refused exactly like at import time, never
//! followed "just to compare a digest": one selected path never expands
//! into read authority over whatever it might now point at (S03 "one
//! selected source path does not authorize its repository, parent
//! directory or credentials"). No Git subprocess, remote fetch or watcher
//! exists anywhere in this module: `claimed_commit`/`claimed_repository`
//! remain untouched, purely descriptive strings, and are never used to
//! verify anything (F14 "unavailable network/remote commit ... no fetch
//! attempted; reference cannot be locally verified").
//!
//! **Owner admission, never automatic.** `check_source` only ever *records
//! an observation* — it cannot change what a `Source`'s current capture
//! is, even when it observes `Changed`. Only two separate, explicitly
//! caller-invoked functions can ever replace a `Source`'s captured bytes
//! after the fact: `admit_changed_source` (changed content, optionally also
//! a new path) and `reselect_source` (a pure relocation, refused unless the
//! new location's bytes are digest-identical to what was already saved).
//! Both require the caller's own `expected_revision_id`, so a stale
//! decision cannot silently clobber a newer observation (F21/I05, the
//! same optimistic-concurrency discipline every other lifecycle-significant
//! transition in this crate already uses for `Action` transitions and
//! `Decision` accept/withdraw/supersede, applied here for the same reason).

use crate::canonical::{CanonicalStore, CommandOutcome, RecordOrigin};
use crate::capture::{self, Source, SourceKind};
use crate::project::{self, RecordPayload};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Map as JsonMap;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatus {
    /// The observed bytes hash to exactly the currently pinned capture.
    Match,
    /// The path opened and read successfully, but hashes to something
    /// else — F09 "stale/revision mismatch". Never auto-replaces the
    /// cited evidence; owner admission is a separate, explicit action.
    Changed,
    /// The path could not be found at all (F08 "missing source").
    Missing,
    /// The path exists but was refused: permission denied, a symlink now
    /// sits where a regular file used to (S03), it is no longer a regular
    /// file, or it now exceeds the bounded-read ceiling and cannot be
    /// safely hashed in full. Deliberately distinct from `Missing`: the
    /// path is reachable, but this module refuses to trust or read it.
    Denied,
}

/// One immutable, append-only observation of a `Source`'s selected local
/// file. §15 "Source check" row: "observed digest or Missing/Denied/
/// Unchecked, time, checking actor and exact algorithm/version." (This
/// format has no persisted `Unchecked` variant — see "Unchecked is
/// derived, not stored" below.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceCheck {
    pub payload_schema_version: u32,
    pub project_id: String,
    pub source_id: String,
    /// The `Source`'s own current `revision_id` at the moment this check
    /// ran — "Source/revision" per §15's own row wording, so a later
    /// reader can tell exactly which capture this observation was made
    /// against, even if the `Source` has since been superseded.
    pub checked_revision_id: String,
    pub status: CheckStatus,
    /// `Some` only for `Match`/`Changed` (something was actually read and
    /// hashed); `None` for `Missing`/`Denied` — never a fabricated digest
    /// for bytes this module never read.
    pub observed_sha256: Option<String>,
    pub observed_byte_length: Option<u64>,
    pub observed_at: String,
    pub checking_actor: String,
    /// "Exact algorithm/version" per §15 — always `"sha256"` today; a
    /// named, versioned field rather than an implicit assumption, so a
    /// future algorithm change is detectable rather than silently
    /// reinterpreted.
    pub algorithm: String,
    #[serde(flatten)]
    pub unknown: JsonMap<String, serde_json::Value>,
}

enum RawObservation {
    Ok { sha256: String, byte_length: u64 },
    Missing,
    Denied,
}

/// See module docs, "Why `observe` does not reuse `capture::capture_file`".
fn observe(path: &Path) -> RawObservation {
    let meta = match std::fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return RawObservation::Denied;
        }
        Err(_) => return RawObservation::Missing,
    };
    if meta.file_type().is_symlink() {
        // S03: never follow a symlink at recheck time either. A source
        // whose exact selected path was silently replaced by a symlink
        // after admission is exactly the relocation/escape attempt this
        // gate exists to catch — reported as Denied, not a bare Missing.
        return RawObservation::Denied;
    }
    if !meta.file_type().is_file() {
        return RawObservation::Denied;
    }
    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return RawObservation::Denied;
        }
        Err(_) => return RawObservation::Missing,
    };
    let mut buf = Vec::new();
    if file
        .by_ref()
        .take(capture::MAX_ARTIFACT_BYTES as u64 + 1)
        .read_to_end(&mut buf)
        .is_err()
    {
        return RawObservation::Denied;
    }
    if buf.len() > capture::MAX_ARTIFACT_BYTES {
        // Cannot safely bound-read it to prove a digest either way; refuse
        // rather than assert Match/Changed from a partial read (I07/F16:
        // "no automatic locator remap or currentness assertion").
        return RawObservation::Denied;
    }
    let sha256 = crate::events::hash_bytes(&buf);
    RawObservation::Ok {
        sha256,
        byte_length: buf.len() as u64,
    }
}

fn read_file_source(store: &CanonicalStore, source_id: &str) -> Result<(String, Source)> {
    let (revision_id, payload) = store
        .read_current(source_id)?
        .ok_or_else(|| Error::Capture(format!("no source exists with id {source_id}")))?;
    let source = RecordPayload::from_json(&payload)?.as_source()?.clone();
    if source.source_kind != SourceKind::File {
        return Err(Error::Capture(format!(
            "source {source_id} is a manual reference; there is no local file to check (I07: \
             a claimed repository/commit/path on a manual reference is a declared label, never \
             verified content)"
        )));
    }
    Ok((revision_id, source))
}

fn pinned_sha256(source: &Source, source_id: &str) -> Result<String> {
    source
        .capture
        .as_ref()
        .map(|c| c.sha256.clone())
        .ok_or_else(|| {
            Error::Capture(format!(
                "source {source_id} has no captured bytes to compare against"
            ))
        })
}

fn claimed_locator<'a>(source: &'a Source, source_id: &str) -> Result<&'a str> {
    source.claimed_path.as_deref().ok_or_else(|| {
        Error::Capture(format!(
            "source {source_id} has no recorded local locator to check (imported before \
             T03-01, or already known to have no selected path)"
        ))
    })
}

/// Explicitly recheck a previously admitted, file-backed `Source` against
/// its currently pinned capture digest. Never mutates the `Source` — see
/// module docs. Creates exactly one new `SourceCheck` object per call
/// ("I06: derivation" does not apply here — this *is* canonical state, an
/// intentional user-triggered observation, not a disposable index).
pub fn check_source(
    store: &mut CanonicalStore,
    actor: &str,
    source_id: &str,
) -> Result<(CommandOutcome, SourceCheck)> {
    let (revision_id, source) = read_file_source(store, source_id)?;
    let path = claimed_locator(&source, source_id)?.to_string();
    let pinned = pinned_sha256(&source, source_id)?;

    let (status, observed_sha256, observed_byte_length) = match observe(Path::new(&path)) {
        RawObservation::Missing => (CheckStatus::Missing, None, None),
        RawObservation::Denied => (CheckStatus::Denied, None, None),
        RawObservation::Ok {
            sha256,
            byte_length,
        } => {
            if sha256 == pinned {
                (CheckStatus::Match, Some(sha256), Some(byte_length))
            } else {
                (CheckStatus::Changed, Some(sha256), Some(byte_length))
            }
        }
    };

    let check = SourceCheck {
        payload_schema_version: project::RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: source.project_id.clone(),
        source_id: source_id.to_string(),
        checked_revision_id: revision_id,
        status,
        observed_sha256,
        observed_byte_length,
        observed_at: capture::now_rfc3339_utc(),
        checking_actor: actor.to_string(),
        algorithm: "sha256".to_string(),
        unknown: JsonMap::new(),
    };
    let mut writer = store.writer()?;
    let (outcome, record) = project::commit_create(
        &mut writer,
        actor,
        RecordOrigin::User,
        RecordPayload::SourceCheck(check),
    )?;
    Ok((outcome, record.as_source_check()?.clone()))
}

/// Explicitly admit the selected file's *current* bytes as a new `Source`
/// revision — the only path that ever replaces a `Source`'s captured
/// bytes after the fact, and only on an explicit, separately-invoked
/// caller action, never automatically from `check_source`. Re-opens and
/// re-hashes the file itself through `capture::capture_file` (fully
/// re-validated: symlink/regular-file/size checks all reapplied), rather
/// than trusting a possibly-stale prior `SourceCheck` digest — no gap
/// between "what was checked" and "what was admitted". `path_override`
/// lets one call cover "content changed" and "also moved" together;
/// omitting it re-admits from the source's existing recorded locator.
pub fn admit_changed_source(
    store: &mut CanonicalStore,
    actor: &str,
    source_id: &str,
    expected_revision_id: &str,
    path_override: Option<&Path>,
) -> Result<(CommandOutcome, Source)> {
    let (_, mut source) = read_file_source(store, source_id)?;
    let path_string = match path_override {
        Some(p) => p.to_string_lossy().into_owned(),
        None => claimed_locator(&source, source_id)?.to_string(),
    };
    let new_capture = capture::capture_file(Path::new(&path_string))?;
    source.capture = Some(new_capture);
    source.claimed_path = Some(path_string);
    let mut writer = store.writer()?;
    let (outcome, record) = project::commit_update(
        &mut writer,
        actor,
        RecordOrigin::User,
        source_id,
        expected_revision_id,
        RecordPayload::Source(source),
    )?;
    Ok((outcome, record.as_source()?.clone()))
}

/// Explicit relocation: point an already-admitted `Source` at a new local
/// path, verifying the new location's bytes are digest-identical to the
/// currently pinned capture before recording the move ("Reselection after
/// move verifies digest and preserves source identity/history" — §
/// T03-01's own implementation requirement). Refuses, rather than silently
/// admitting changed content, when the digest differs — callers must use
/// `admit_changed_source` for that, which records the move and the changed
/// bytes together as one explicit owner-admitted event.
pub fn reselect_source(
    store: &mut CanonicalStore,
    actor: &str,
    source_id: &str,
    expected_revision_id: &str,
    new_path: &Path,
) -> Result<(CommandOutcome, Source)> {
    let (_, mut source) = read_file_source(store, source_id)?;
    let pinned = pinned_sha256(&source, source_id)?;
    match observe(new_path) {
        RawObservation::Ok { sha256, .. } if sha256 == pinned => {}
        RawObservation::Ok { .. } => {
            return Err(Error::Capture(
                "new location's content differs from the last saved capture; use \
                 source-admit-change to record both the move and the changed bytes together"
                    .into(),
            ));
        }
        RawObservation::Missing => {
            return Err(Error::Capture(format!(
                "new location is missing or unreachable: {}",
                new_path.display()
            )));
        }
        RawObservation::Denied => {
            return Err(Error::Capture(format!(
                "new location cannot be safely read (denied, symlinked, not a regular file, or \
                 too large to bound-read): {}",
                new_path.display()
            )));
        }
    }
    source.claimed_path = Some(new_path.to_string_lossy().into_owned());
    let mut writer = store.writer()?;
    let (outcome, record) = project::commit_update(
        &mut writer,
        actor,
        RecordOrigin::User,
        source_id,
        expected_revision_id,
        RecordPayload::Source(source),
    )?;
    Ok((outcome, record.as_source()?.clone()))
}

/// Every `SourceCheck` ever recorded for `source_id`, oldest first — the
/// full observation history §15 promises is never a rewritten revision.
/// Full scan, same documented limitation every other `list_*` function in
/// this crate already carries.
pub fn list_checks_for_source(
    store: &CanonicalStore,
    source_id: &str,
) -> Result<Vec<(String, SourceCheck)>> {
    let mut out = Vec::new();
    for (object_id, _, payload) in store.list_current_objects()? {
        if let Ok(RecordPayload::SourceCheck(c)) = RecordPayload::from_json(&payload) {
            if c.source_id == source_id {
                out.push((object_id, c));
            }
        }
    }
    out.sort_by(|a, b| a.1.observed_at.cmp(&b.1.observed_at));
    Ok(out)
}

/// Every `SourceCheck` in `project_id` — the `SourceCheck` counterpart of
/// `relation::list_project_relations`, reused by `export::
/// project_scope_object_ids` so a project-scoped export never silently
/// drops this canonical kind (I06/I10).
pub fn list_project_source_checks(
    store: &CanonicalStore,
    project_id: &str,
) -> Result<Vec<(String, SourceCheck)>> {
    let mut out = Vec::new();
    for (object_id, _, payload) in store.list_current_objects()? {
        if let Ok(RecordPayload::SourceCheck(c)) = RecordPayload::from_json(&payload) {
            if c.project_id == project_id {
                out.push((object_id, c));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::CanonicalStore;
    use std::path::PathBuf;

    fn tmp() -> PathBuf {
        let d = std::env::temp_dir().join(format!("fehrest-source-check-{}", uuid::Uuid::now_v7()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }
    fn cleanup(p: &Path) {
        let _ = std::fs::remove_dir_all(p);
    }

    fn new_project(store: &mut CanonicalStore) -> String {
        let mut writer = store.writer().unwrap();
        project::create_project(&mut writer, "owner", "P", None)
            .unwrap()
            .0
            .object_id
    }

    fn import(store: &mut CanonicalStore, project_id: &str, path: &Path) -> (String, String) {
        let mut writer = store.writer().unwrap();
        let (outcome, _) =
            capture::import_file(&mut writer, "owner", project_id, "l", path).unwrap();
        (outcome.object_id, outcome.revision_id)
    }

    #[test]
    fn recheck_reports_match_when_bytes_are_unchanged() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let (source_id, _) = import(&mut store, &project_id, &file_path);

        let (_, check) = check_source(&mut store, "owner", &source_id).unwrap();
        assert_eq!(check.status, CheckStatus::Match);
        assert_eq!(
            check.observed_sha256.as_deref(),
            Some(crate::events::hash_bytes(b"hello")).as_deref()
        );

        // Purely an observation: the Source itself is untouched.
        let (_, payload) = store.read_current(&source_id).unwrap().unwrap();
        let source = RecordPayload::from_json(&payload)
            .unwrap()
            .as_source()
            .unwrap()
            .clone();
        assert_eq!(
            source.capture.unwrap().sha256,
            check.observed_sha256.unwrap()
        );

        cleanup(&root);
    }

    #[test]
    fn recheck_reports_changed_without_auto_replacing_the_capture() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let (source_id, original_revision) = import(&mut store, &project_id, &file_path);

        std::fs::write(&file_path, b"goodbye, this is different content").unwrap();
        let (_, check) = check_source(&mut store, "owner", &source_id).unwrap();
        assert_eq!(check.status, CheckStatus::Changed);
        assert_eq!(
            check.observed_sha256.as_deref(),
            Some(crate::events::hash_bytes(
                b"goodbye, this is different content"
            ))
            .as_deref()
        );

        // F09: no auto-replacement of cited evidence.
        let (revision_id, payload) = store.read_current(&source_id).unwrap().unwrap();
        assert_eq!(revision_id, original_revision);
        let source = RecordPayload::from_json(&payload)
            .unwrap()
            .as_source()
            .unwrap()
            .clone();
        assert_eq!(
            source.capture.unwrap().sha256,
            crate::events::hash_bytes(b"hello")
        );

        cleanup(&root);
    }

    #[test]
    fn recheck_reports_missing_when_the_file_is_deleted() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let (source_id, _) = import(&mut store, &project_id, &file_path);

        std::fs::remove_file(&file_path).unwrap();
        let (_, check) = check_source(&mut store, "owner", &source_id).unwrap();
        assert_eq!(check.status, CheckStatus::Missing);
        assert!(check.observed_sha256.is_none());

        // F08: the saved source revision (and its bytes) is retained.
        assert_eq!(
            capture::extract_bytes(&store, &source_id).unwrap(),
            b"hello"
        );

        cleanup(&root);
    }

    #[test]
    fn recheck_reports_denied_when_the_path_becomes_a_directory() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let (source_id, _) = import(&mut store, &project_id, &file_path);

        std::fs::remove_file(&file_path).unwrap();
        std::fs::create_dir_all(&file_path).unwrap();
        let (_, check) = check_source(&mut store, "owner", &source_id).unwrap();
        assert_eq!(check.status, CheckStatus::Denied);
        assert!(check.observed_sha256.is_none());

        cleanup(&root);
    }

    /// S03: a symlink silently placed at the exact previously-authorized
    /// path is refused, never followed to "helpfully" compare a digest —
    /// the adversarial case this task's own security clause names.
    #[test]
    fn recheck_refuses_a_symlink_swapped_in_after_admission() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let (source_id, _) = import(&mut store, &project_id, &file_path);

        let elsewhere = root.join("elsewhere-secret.txt");
        std::fs::write(&elsewhere, b"attacker content").unwrap();
        std::fs::remove_file(&file_path).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(&elsewhere, &file_path).unwrap();
        #[cfg(windows)]
        let symlink_created = std::os::windows::fs::symlink_file(&elsewhere, &file_path).is_ok();
        #[cfg(not(windows))]
        let symlink_created = true;
        #[cfg(windows)]
        if !symlink_created {
            // Same documented environment limitation `capture.rs`'s own
            // symlink test already carries: creating a file symlink on
            // Windows can require dev-mode/elevation the CI runner lacks.
            cleanup(&root);
            return;
        }

        let (_, check) = check_source(&mut store, "owner", &source_id).unwrap();
        assert_eq!(check.status, CheckStatus::Denied);
        assert!(check.observed_sha256.is_none());

        cleanup(&root);
    }

    #[test]
    fn admit_changed_source_replaces_the_capture_only_on_explicit_call() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let (source_id, original_revision) = import(&mut store, &project_id, &file_path);

        std::fs::write(&file_path, b"new content entirely").unwrap();
        let (outcome, updated) =
            admit_changed_source(&mut store, "owner", &source_id, &original_revision, None)
                .unwrap();
        assert_ne!(outcome.revision_id, original_revision);
        assert_eq!(
            updated.capture.unwrap().sha256,
            crate::events::hash_bytes(b"new content entirely")
        );
        // History preserved: two revisions now exist for the same object.
        assert_eq!(store.history(&source_id).unwrap().len(), 2);

        cleanup(&root);
    }

    #[test]
    fn admit_changed_source_requires_the_correct_expected_revision() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let (source_id, _) = import(&mut store, &project_id, &file_path);
        std::fs::write(&file_path, b"changed").unwrap();

        let stale = uuid::Uuid::now_v7().to_string();
        let err = admit_changed_source(&mut store, "owner", &source_id, &stale, None).unwrap_err();
        assert!(format!("{err}").to_lowercase().contains("revision"));

        cleanup(&root);
    }

    #[test]
    fn reselect_source_accepts_a_move_with_identical_bytes() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let (source_id, original_revision) = import(&mut store, &project_id, &file_path);

        let new_path = root.join("moved").join("f.txt");
        std::fs::create_dir_all(new_path.parent().unwrap()).unwrap();
        std::fs::rename(&file_path, &new_path).unwrap();

        let (outcome, updated) = reselect_source(
            &mut store,
            "owner",
            &source_id,
            &original_revision,
            &new_path,
        )
        .unwrap();
        assert_ne!(outcome.revision_id, original_revision);
        assert_eq!(
            updated.claimed_path.as_deref(),
            Some(new_path.to_string_lossy()).as_deref()
        );
        assert_eq!(
            updated.capture.as_ref().unwrap().sha256,
            crate::events::hash_bytes(b"hello")
        );

        // The relocation itself is recheckable at the new path afterwards.
        let (_, check) = check_source(&mut store, "owner", &source_id).unwrap();
        assert_eq!(check.status, CheckStatus::Match);

        cleanup(&root);
    }

    #[test]
    fn reselect_source_refuses_a_move_with_different_bytes() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let (source_id, original_revision) = import(&mut store, &project_id, &file_path);

        let new_path = root.join("different-content.txt");
        std::fs::write(&new_path, b"not the same bytes at all").unwrap();

        let err = reselect_source(
            &mut store,
            "owner",
            &source_id,
            &original_revision,
            &new_path,
        )
        .unwrap_err();
        assert!(format!("{err}").contains("source-admit-change"));

        // Refused: the Source is completely untouched.
        let (revision_id, _) = store.read_current(&source_id).unwrap().unwrap();
        assert_eq!(revision_id, original_revision);

        cleanup(&root);
    }

    #[test]
    fn check_refuses_a_manual_reference_with_no_local_file() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (outcome, _) = {
            let mut writer = store.writer().unwrap();
            capture::create_manual_reference(
                &mut writer,
                "owner",
                &project_id,
                "l",
                None,
                None,
                None,
            )
            .unwrap()
        };
        let err = check_source(&mut store, "owner", &outcome.object_id).unwrap_err();
        assert!(format!("{err}").contains("manual reference"));
        cleanup(&root);
    }

    #[test]
    fn check_history_is_append_only_and_chronological() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let (source_id, _) = import(&mut store, &project_id, &file_path);

        check_source(&mut store, "owner", &source_id).unwrap();
        std::fs::write(&file_path, b"changed now").unwrap();
        check_source(&mut store, "owner", &source_id).unwrap();

        let history = list_checks_for_source(&store, &source_id).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].1.status, CheckStatus::Match);
        assert_eq!(history[1].1.status, CheckStatus::Changed);

        let project_checks = list_project_source_checks(&store, &project_id).unwrap();
        assert_eq!(project_checks.len(), 2);

        cleanup(&root);
    }

    #[test]
    fn a_fault_during_check_commit_leaves_no_partial_admission() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let (source_id, revision_id) = import(&mut store, &project_id, &file_path);
        let source = {
            let (_, payload) = store.read_current(&source_id).unwrap().unwrap();
            RecordPayload::from_json(&payload)
                .unwrap()
                .as_source()
                .unwrap()
                .clone()
        };

        let check = SourceCheck {
            payload_schema_version: project::RECORD_PAYLOAD_SCHEMA_VERSION,
            project_id: source.project_id.clone(),
            source_id: source_id.clone(),
            checked_revision_id: revision_id,
            status: CheckStatus::Match,
            observed_sha256: source.capture.map(|c| c.sha256),
            observed_byte_length: Some(5),
            observed_at: capture::now_rfc3339_utc(),
            checking_actor: "owner".into(),
            algorithm: "sha256".into(),
            unknown: JsonMap::new(),
        };
        let payload = RecordPayload::SourceCheck(check).to_json().unwrap();
        let head_before = store.transaction_head().unwrap();
        {
            let mut writer = store.writer().unwrap();
            let result = writer.commit_with_fault(
                crate::canonical::CommandInput {
                    command_id: uuid::Uuid::now_v7().to_string(),
                    actor: "owner".into(),
                    origin: RecordOrigin::User,
                    target: crate::canonical::CommandTarget::CreateObject { payload },
                },
                Some(crate::canonical::CommitFaultPoint::BeforeSqlCommit),
            );
            assert!(result.is_err());
        }
        assert_eq!(store.transaction_head().unwrap(), head_before);
        // Only the project and the source remain; no orphaned check object.
        assert_eq!(store.list_current_objects().unwrap().len(), 2);

        cleanup(&root);
    }
}
