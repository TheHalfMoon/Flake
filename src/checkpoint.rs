//! `ReviewCheckpoint`: an explicit, owner-driven "reviewed through here"
//! marker for one project (`T03-03`).
//!
//! §15's own "Review checkpoint" row: "Project, owner, reviewed-through
//! transaction sequence. Explicit user transition, monotonically advances
//! unless user explicitly resets with reason; not an automatic session
//! record." §12 reinforces this: "A checkpoint is an explicit 'Mark
//! reviewed through here' sequence marker; opening a page never implies
//! the user read it." This module enforces both halves of that sentence
//! structurally:
//!
//! - **Not automatic.** No code path anywhere in this crate creates or
//!   advances a `ReviewCheckpoint` except [`mark_reviewed_through`] and
//!   [`reset_checkpoint`], both of which require an explicit caller
//!   invocation (a CLI command in practice) naming the project and actor.
//!   [`crate::resume::resume`] (`T03-03`'s other half) only ever *reads*
//!   the current checkpoint; it never writes one, so opening/computing a
//!   resume view can never itself advance the marker.
//! - **Monotonic unless explicitly reset.** [`mark_reviewed_through`]
//!   refuses a `through_seq` lower than the checkpoint's current value;
//!   [`reset_checkpoint`] is the only function that may lower it, and it
//!   requires a mandatory, non-empty `reason` (persisted on the resulting
//!   revision) — mirroring `withdraw_decision`'s identical "a transition
//!   away from the normal path requires a stated reason" rule.
//!
//! **Architecture: an eighth `RecordPayload` kind, `Relation`'s exact
//! shape.** One canonical object per project (found by a full scan
//! filtering on `project_id`, the same pattern `source_check::
//! list_project_source_checks` already establishes), committed through the
//! existing `CommandTarget::CreateObject`/`UpdateObject` — no new
//! `CommandTarget`, no schema change. Every prior mutating transition is
//! its own new, permanent revision; "visible history" is
//! [`checkpoint_history`] reading `CanonicalStore::history`, not a parallel
//! log.

use crate::canonical::{CanonicalStore, CanonicalWriter, CommandOutcome, RecordOrigin};
use crate::project::{self, RecordPayload};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Map as JsonMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReviewCheckpoint {
    pub payload_schema_version: u32,
    pub project_id: String,
    pub owner: String,
    pub reviewed_through_seq: i64,
    /// Set only on the revision an explicit [`reset_checkpoint`] call
    /// produced; `None` on every ordinary [`mark_reviewed_through`]
    /// revision, including the first.
    #[serde(default)]
    pub reset_reason: Option<String>,
    #[serde(flatten)]
    pub unknown: JsonMap<String, serde_json::Value>,
}

/// The project's current checkpoint object, if one has ever been marked —
/// `(object_id, revision_id, checkpoint)`.
pub fn current_checkpoint(
    store: &CanonicalStore,
    project_id: &str,
) -> Result<Option<(String, String, ReviewCheckpoint)>> {
    for (object_id, revision_id, payload) in store.list_current_objects()? {
        if let Ok(RecordPayload::ReviewCheckpoint(c)) = RecordPayload::from_json(&payload) {
            if c.project_id == project_id {
                return Ok(Some((object_id, revision_id, c)));
            }
        }
    }
    Ok(None)
}

/// Every checkpoint revision ever recorded for this project, oldest first
/// — empty if the project has never been marked reviewed.
pub fn checkpoint_history(
    store: &CanonicalStore,
    project_id: &str,
) -> Result<Vec<ReviewCheckpoint>> {
    let Some((object_id, _, _)) = current_checkpoint(store, project_id)? else {
        return Ok(Vec::new());
    };
    let mut out = Vec::new();
    for (_, _, payload) in store.history(&object_id)? {
        if let Ok(RecordPayload::ReviewCheckpoint(c)) = RecordPayload::from_json(&payload) {
            out.push(c);
        }
    }
    Ok(out)
}

/// Advance the project's checkpoint to `through_seq` (defaults to the
/// vault's current transaction head). Refuses a `through_seq` lower than
/// the checkpoint's current value — use [`reset_checkpoint`] for that,
/// explicitly and with a reason.
///
/// `expected_revision_id` must be `None` for the very first mark on a
/// project (there is no prior revision to conflict with) and `Some` for
/// every subsequent one (F21/I05: a stale caller cannot silently clobber a
/// newer mark).
pub fn mark_reviewed_through(
    store: &mut CanonicalStore,
    actor: &str,
    project_id: &str,
    expected_revision_id: Option<&str>,
    through_seq: Option<i64>,
) -> Result<(CommandOutcome, ReviewCheckpoint)> {
    project::require_project(store, project_id)?;
    let (head_seq, _) = store.transaction_head()?;
    let through_seq = through_seq.unwrap_or(head_seq);
    if !(0..=head_seq).contains(&through_seq) {
        return Err(Error::Project(format!(
            "through_seq {through_seq} is out of range for this vault's current head {head_seq}"
        )));
    }

    let existing = current_checkpoint(store, project_id)?;
    match (&existing, expected_revision_id) {
        (None, Some(_)) => {
            return Err(Error::Project(
                "no checkpoint exists yet for this project; omit --expect for the first mark"
                    .into(),
            ));
        }
        (Some(_), None) => {
            return Err(Error::Project(
                "a checkpoint already exists for this project; supply --expect".into(),
            ));
        }
        _ => {}
    }
    if let Some((_, _, current)) = &existing {
        if through_seq < current.reviewed_through_seq {
            return Err(Error::Project(format!(
                "through_seq {through_seq} would move the checkpoint backward from {}; use reset_checkpoint explicitly with a reason",
                current.reviewed_through_seq
            )));
        }
    }

    let record = RecordPayload::ReviewCheckpoint(ReviewCheckpoint {
        payload_schema_version: project::RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: project_id.to_string(),
        owner: actor.to_string(),
        reviewed_through_seq: through_seq,
        reset_reason: None,
        unknown: JsonMap::new(),
    });

    let mut writer = store.writer()?;
    let (outcome, checkpoint) = match &existing {
        None => commit_create(&mut writer, actor, record)?,
        Some((object_id, _, _)) => commit_update(
            &mut writer,
            actor,
            object_id,
            expected_revision_id.expect("checked above: Some when existing is Some"),
            record,
        )?,
    };
    Ok((outcome, checkpoint))
}

/// Explicitly move the checkpoint to `through_seq`, which may be lower
/// than its current value — the only function in this module that may
/// lower it. `reason` is mandatory and non-empty, and is persisted on the
/// resulting revision's `reset_reason`.
pub fn reset_checkpoint(
    store: &mut CanonicalStore,
    actor: &str,
    project_id: &str,
    expected_revision_id: &str,
    through_seq: i64,
    reason: &str,
) -> Result<(CommandOutcome, ReviewCheckpoint)> {
    if reason.trim().is_empty() {
        return Err(Error::Project(
            "resetting a checkpoint requires an explicit, non-empty reason".into(),
        ));
    }
    project::require_project(store, project_id)?;
    let (head_seq, _) = store.transaction_head()?;
    if !(0..=head_seq).contains(&through_seq) {
        return Err(Error::Project(format!(
            "through_seq {through_seq} is out of range for this vault's current head {head_seq}"
        )));
    }
    let Some((object_id, _, _)) = current_checkpoint(store, project_id)? else {
        return Err(Error::Project(
            "no checkpoint exists yet for this project to reset".into(),
        ));
    };

    let record = RecordPayload::ReviewCheckpoint(ReviewCheckpoint {
        payload_schema_version: project::RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: project_id.to_string(),
        owner: actor.to_string(),
        reviewed_through_seq: through_seq,
        reset_reason: Some(reason.to_string()),
        unknown: JsonMap::new(),
    });
    let mut writer = store.writer()?;
    commit_update(&mut writer, actor, &object_id, expected_revision_id, record)
}

fn commit_create(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    record: RecordPayload,
) -> Result<(CommandOutcome, ReviewCheckpoint)> {
    let (outcome, record) = project::commit_create(writer, actor, RecordOrigin::User, record)?;
    Ok((
        outcome,
        match record {
            RecordPayload::ReviewCheckpoint(c) => c,
            _ => unreachable!(),
        },
    ))
}

fn commit_update(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    object_id: &str,
    expected_revision_id: &str,
    record: RecordPayload,
) -> Result<(CommandOutcome, ReviewCheckpoint)> {
    let (outcome, record) = project::commit_update(
        writer,
        actor,
        RecordOrigin::User,
        object_id,
        expected_revision_id,
        record,
    )?;
    Ok((
        outcome,
        match record {
            RecordPayload::ReviewCheckpoint(c) => c,
            _ => unreachable!(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-checkpoint-{}", uuid::Uuid::now_v7()))
    }

    fn cleanup(p: &Path) {
        let _ = std::fs::remove_dir_all(p);
    }

    fn open_store() -> (PathBuf, CanonicalStore) {
        let root = tmp();
        let store = CanonicalStore::create(&root).unwrap();
        (root, store)
    }

    fn make_project(store: &mut CanonicalStore) -> String {
        let mut writer = store.writer().unwrap();
        let (outcome, _p) = project::create_project(&mut writer, "owner", "P", None).unwrap();
        outcome.object_id
    }

    #[test]
    fn no_checkpoint_before_any_mark() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        assert_eq!(current_checkpoint(&store, &project_id).unwrap(), None);
        assert!(checkpoint_history(&store, &project_id).unwrap().is_empty());
        cleanup(&root);
    }

    #[test]
    fn first_mark_creates_a_checkpoint_without_expect() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let head = store.transaction_head().unwrap().0;
        let (_outcome, checkpoint) =
            mark_reviewed_through(&mut store, "owner", &project_id, None, None).unwrap();
        assert_eq!(checkpoint.reviewed_through_seq, head);
        assert_eq!(checkpoint.reset_reason, None);
        cleanup(&root);
    }

    #[test]
    fn second_mark_without_expect_is_refused() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        mark_reviewed_through(&mut store, "owner", &project_id, None, None).unwrap();
        let err = mark_reviewed_through(&mut store, "owner", &project_id, None, None).unwrap_err();
        assert!(format!("{err}").contains("already exists"));
        cleanup(&root);
    }

    #[test]
    fn mark_backward_is_refused_without_an_explicit_reset() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        // Advance the head so there is a later sequence to mark through.
        make_project(&mut store);
        let head_after = store.transaction_head().unwrap().0;
        let (outcome, checkpoint) =
            mark_reviewed_through(&mut store, "owner", &project_id, None, Some(head_after))
                .unwrap();
        assert_eq!(checkpoint.reviewed_through_seq, head_after);

        let err = mark_reviewed_through(
            &mut store,
            "owner",
            &project_id,
            Some(&outcome.revision_id),
            Some(0),
        )
        .unwrap_err();
        assert!(format!("{err}").contains("backward"));
        cleanup(&root);
    }

    #[test]
    fn reset_explicitly_moves_the_checkpoint_backward_with_a_reason() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let head = store.transaction_head().unwrap().0;
        let (outcome, _) =
            mark_reviewed_through(&mut store, "owner", &project_id, None, Some(head)).unwrap();

        let (_outcome2, checkpoint2) = reset_checkpoint(
            &mut store,
            "owner",
            &project_id,
            &outcome.revision_id,
            0,
            "owner wants to re-review everything",
        )
        .unwrap();
        assert_eq!(checkpoint2.reviewed_through_seq, 0);
        assert_eq!(
            checkpoint2.reset_reason.as_deref(),
            Some("owner wants to re-review everything")
        );

        let history = checkpoint_history(&store, &project_id).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].reviewed_through_seq, head);
        assert_eq!(history[1].reviewed_through_seq, 0);
        cleanup(&root);
    }

    #[test]
    fn reset_requires_a_non_empty_reason() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (outcome, _) =
            mark_reviewed_through(&mut store, "owner", &project_id, None, None).unwrap();
        let err = reset_checkpoint(
            &mut store,
            "owner",
            &project_id,
            &outcome.revision_id,
            0,
            "",
        )
        .unwrap_err();
        assert!(format!("{err}").contains("reason"));
        cleanup(&root);
    }

    #[test]
    fn reset_before_any_mark_is_refused() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let err = reset_checkpoint(&mut store, "owner", &project_id, "not-a-real-rev", 0, "r")
            .unwrap_err();
        assert!(format!("{err}").contains("no checkpoint exists"));
        cleanup(&root);
    }

    #[test]
    fn out_of_range_through_seq_is_refused() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let head = store.transaction_head().unwrap().0;
        let err = mark_reviewed_through(&mut store, "owner", &project_id, None, Some(head + 100))
            .unwrap_err();
        assert!(format!("{err}").contains("out of range"));
        cleanup(&root);
    }

    #[test]
    fn stale_expect_is_refused() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (outcome, _) =
            mark_reviewed_through(&mut store, "owner", &project_id, None, None).unwrap();
        make_project(&mut store); // advance head so a second mark has something new to say
        mark_reviewed_through(
            &mut store,
            "owner",
            &project_id,
            Some(&outcome.revision_id),
            None,
        )
        .unwrap();
        // Reusing the now-stale first revision id must be refused.
        let err = mark_reviewed_through(
            &mut store,
            "owner",
            &project_id,
            Some(&outcome.revision_id),
            None,
        )
        .unwrap_err();
        assert!(
            format!("{err}")
                .to_lowercase()
                .contains("expected revision")
                || format!("{err}").to_lowercase().contains("conflict")
        );
        cleanup(&root);
    }
}
