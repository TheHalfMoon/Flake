//! The owner resume view (`T03-03`): "what changed, what's current, what
//! needs attention" for one project, composed from already-canonical reads
//! — never a new stored fact.
//!
//! §12's own "Resume" row: "Current accepted decisions, next actions,
//! relevant notes, and changes since the user's explicit checkpoint;
//! conflicts and stale evidence precede reassuring summaries." This module
//! is read-only start to finish: [`resume`] never writes a
//! [`crate::checkpoint::ReviewCheckpoint`] — only
//! [`crate::checkpoint::mark_reviewed_through`]/
//! [`crate::checkpoint::reset_checkpoint`] do, and only on an explicit
//! separate caller invocation. Opening or computing a resume view can
//! never itself advance the checkpoint (§12: "opening a page never implies
//! the user read it").
//!
//! **Ordering.** [`ResumeView`]'s own field order is this task's own
//! priority order, matching §18's identically-worded package priority
//! groups ("unresolved conflict/staleness, accepted decisions, active
//! actions, cited evidence, supporting notes; then stable recorded
//! sequence and ID"): [`ResumeView::conflicts`], then
//! [`ResumeView::stale_or_missing_evidence`], then
//! [`ResumeView::current_decisions`], then [`ResumeView::next_actions`],
//! then [`ResumeView::relevant_notes`], then
//! [`ResumeView::changes_since_checkpoint`] (the raw, unfiltered "what
//! happened" log). Every list within each group is itself sorted by
//! `object_id` (or `recorded_seq` for the changes log) — stable and
//! deterministic across repeated calls at the same snapshot.
//!
//! **One fixed snapshot per call.** `head_seq` is read once via
//! `CanonicalStore::transaction_head` and threaded through every
//! sub-computation (in particular, every
//! [`crate::decision_state::resolve_decision_state`] call is pinned to it
//! via `as_of_recorded`) — this task's own "Stable resume output at the
//! same snapshot" requirement, not a separate re-read per section that
//! could observe a concurrent commit mid-computation.
//!
//! **No new stored fact.** Every field here is recomputed from
//! already-canonical reads (`T02-01`-`T03-02`'s own already-audited
//! functions) on every call; nothing this module returns is itself
//! persisted (I06: derivation is disposable).

use crate::canonical::CanonicalStore;
use crate::checkpoint;
use crate::decision_state::{self, DecisionResolution};
use crate::project::{self, Action, ActionState, Note};
use crate::source_check::{self, CheckStatus, SourceCheck};
use crate::{project as project_mod, Result};
use serde::Serialize;

/// One source whose most recent check is not a clean `Match` — this
/// project's "missing/changed evidence" (§12).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StaleEvidence {
    pub source_id: String,
    pub latest_check: SourceCheck,
}

/// One revision recorded after the project's checkpoint — the raw
/// "changes since checkpoint" log, before any per-kind interpretation.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ChangeSummary {
    pub recorded_seq: i64,
    pub object_id: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ResumeView {
    pub project_id: String,
    /// `None` if this project has never been marked reviewed.
    pub reviewed_through_seq: Option<i64>,
    /// The exact recorded-sequence snapshot this entire view was computed
    /// at.
    pub head_seq: i64,
    /// Every `decision_key` currently `NeedsReview` (§12 "Conflict":
    /// incompatible accepted decisions with the same key and overlapping
    /// time remain visible together — never latest-wins).
    pub conflicts: Vec<DecisionResolution>,
    /// Every `Source` whose latest check is `Missing`, `Changed` or
    /// `Denied` — never silently omitted (§12 "Negative evidence").
    pub stale_or_missing_evidence: Vec<StaleEvidence>,
    /// Every `decision_key` currently resolved to exactly one accepted
    /// decision.
    pub current_decisions: Vec<DecisionResolution>,
    /// Every `Action` not in a terminal state (`Open`/`Doing`/`Blocked`).
    pub next_actions: Vec<(String, Action)>,
    /// Notes with a revision recorded after `reviewed_through_seq`
    /// (everything, if the project has never been marked reviewed).
    pub relevant_notes: Vec<(String, Note)>,
    /// Every revision (any kind) recorded after `reviewed_through_seq`,
    /// oldest first.
    pub changes_since_checkpoint: Vec<ChangeSummary>,
}

/// Compose the resume view for `project_id` at the vault's current
/// transaction head. Entirely read-only: never writes a checkpoint.
pub fn resume(store: &CanonicalStore, project_id: &str) -> Result<ResumeView> {
    project::require_project(store, project_id)?;
    let (head_seq, _) = store.transaction_head()?;
    let checkpoint = checkpoint::current_checkpoint(store, project_id)?;
    let reviewed_through_seq = checkpoint.as_ref().map(|(_, _, c)| c.reviewed_through_seq);
    let since_seq = reviewed_through_seq.unwrap_or(-1);

    let records = project::list_project_records(store, project_id)?;

    let mut decision_keys: Vec<&str> = records
        .iter()
        .filter_map(|(_, r)| r.as_decision().ok())
        .map(|d| d.decision_key.as_str())
        .collect();
    decision_keys.sort_unstable();
    decision_keys.dedup();

    let mut conflicts = Vec::new();
    let mut current_decisions = Vec::new();
    for key in decision_keys {
        let resolution =
            decision_state::resolve_decision_state(store, project_id, key, None, Some(head_seq))?;
        match resolution.outcome {
            decision_state::DecisionOutcome::NeedsReview => conflicts.push(resolution),
            decision_state::DecisionOutcome::CurrentSet(_) => current_decisions.push(resolution),
            decision_state::DecisionOutcome::NoAcceptedDecision => {}
        }
    }
    conflicts.sort_by(|a, b| a.decision_key.cmp(&b.decision_key));
    current_decisions.sort_by(|a, b| a.decision_key.cmp(&b.decision_key));

    let mut stale_or_missing_evidence = Vec::new();
    for (source_id, _source) in crate::capture::list_project_sources(store, project_id)? {
        if let Some((_, latest_check)) = source_check::list_checks_for_source(store, &source_id)?
            .into_iter()
            .last()
        {
            if latest_check.status != CheckStatus::Match {
                stale_or_missing_evidence.push(StaleEvidence {
                    source_id,
                    latest_check,
                });
            }
        }
    }
    stale_or_missing_evidence.sort_by(|a, b| a.source_id.cmp(&b.source_id));

    let mut next_actions: Vec<(String, Action)> = records
        .iter()
        .filter_map(|(id, r)| r.as_action().ok().map(|a| (id.clone(), a.clone())))
        .filter(|(_, a)| a.state != ActionState::Done && a.state != ActionState::Cancelled)
        .collect();
    next_actions.sort_by(|a, b| a.0.cmp(&b.0));

    // recorded_seq per object_id, built once from a single revisions_since
    // scan — reused for both "relevant notes" and the raw changes log
    // below, rather than re-scanning per note.
    let changed_since: Vec<(i64, String, String)> = store
        .revisions_since(since_seq)?
        .into_iter()
        .map(|(seq, object_id, _revision_id, payload)| {
            let kind = project_mod::RecordPayload::from_json(&payload)
                .map(|r| r.kind_str().to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            (seq, object_id, kind)
        })
        .collect();

    let changed_note_ids: std::collections::HashSet<&str> = changed_since
        .iter()
        .filter(|(_, _, kind)| kind == "note")
        .map(|(_, id, _)| id.as_str())
        .collect();
    let mut relevant_notes: Vec<(String, Note)> = records
        .iter()
        .filter_map(|(id, r)| r.as_note().ok().map(|n| (id.clone(), n.clone())))
        .filter(|(id, _)| changed_note_ids.contains(id.as_str()))
        .collect();
    relevant_notes.sort_by(|a, b| a.0.cmp(&b.0));

    let project_object_ids: std::collections::HashSet<String> =
        records.iter().map(|(id, _)| id.clone()).collect();
    let mut changes_since_checkpoint: Vec<ChangeSummary> = changed_since
        .into_iter()
        .filter(|(_, object_id, _)| {
            project_object_ids.contains(object_id) || object_id == project_id
        })
        .map(|(recorded_seq, object_id, kind)| ChangeSummary {
            recorded_seq,
            object_id,
            kind,
        })
        .collect();
    changes_since_checkpoint.sort_by(|a, b| {
        a.recorded_seq
            .cmp(&b.recorded_seq)
            .then_with(|| a.object_id.cmp(&b.object_id))
    });

    Ok(ResumeView {
        project_id: project_id.to_string(),
        reviewed_through_seq,
        head_seq,
        conflicts,
        stale_or_missing_evidence,
        current_decisions,
        next_actions,
        relevant_notes,
        changes_since_checkpoint,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::{DecisionBasis, DecisionVerification};
    use std::path::{Path, PathBuf};

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-resume-{}", uuid::Uuid::now_v7()))
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
    fn empty_project_has_an_empty_resume_view() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let view = resume(&store, &project_id).unwrap();
        assert_eq!(view.reviewed_through_seq, None);
        assert!(view.conflicts.is_empty());
        assert!(view.stale_or_missing_evidence.is_empty());
        assert!(view.current_decisions.is_empty());
        assert!(view.next_actions.is_empty());
        // The project's own creation is itself a change with no checkpoint yet.
        assert!(!view.changes_since_checkpoint.is_empty());
        cleanup(&root);
    }

    #[test]
    fn current_decisions_and_conflicts_are_separated() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            let (outcome, _) = project::create_decision(
                &mut writer,
                "owner",
                &project_id,
                "clear",
                "s",
                None,
                DecisionBasis::UserJudgment,
                DecisionVerification::Unreviewed,
                None,
                None,
            )
            .unwrap();
            project::accept_decision(
                &mut writer,
                "owner",
                &outcome.object_id,
                &outcome.revision_id,
            )
            .unwrap();
        }
        {
            let mut writer = store.writer().unwrap();
            let (a_out, _) = project::create_decision(
                &mut writer,
                "owner",
                &project_id,
                "disputed",
                "a",
                None,
                DecisionBasis::UserJudgment,
                DecisionVerification::Unreviewed,
                None,
                None,
            )
            .unwrap();
            project::accept_decision(&mut writer, "owner", &a_out.object_id, &a_out.revision_id)
                .unwrap();
            let (b_out, _) = project::create_decision(
                &mut writer,
                "owner",
                &project_id,
                "disputed",
                "b",
                None,
                DecisionBasis::UserJudgment,
                DecisionVerification::Unreviewed,
                None,
                None,
            )
            .unwrap();
            project::accept_decision(&mut writer, "owner", &b_out.object_id, &b_out.revision_id)
                .unwrap();
        }

        let view = resume(&store, &project_id).unwrap();
        assert_eq!(view.current_decisions.len(), 1);
        assert_eq!(view.current_decisions[0].decision_key, "clear");
        assert_eq!(view.conflicts.len(), 1);
        assert_eq!(view.conflicts[0].decision_key, "disputed");
        cleanup(&root);
    }

    #[test]
    fn stale_evidence_lists_only_non_match_latest_checks() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let file_path = root.join("evidence.txt");
        std::fs::write(&file_path, b"original").unwrap();
        let source_id = {
            let mut writer = store.writer().unwrap();
            let (outcome, _s) =
                crate::capture::import_file(&mut writer, "owner", &project_id, "label", &file_path)
                    .unwrap();
            outcome.object_id
        };
        source_check::check_source(&mut store, "owner", &source_id).unwrap();
        let view = resume(&store, &project_id).unwrap();
        assert!(
            view.stale_or_missing_evidence.is_empty(),
            "Match is not stale"
        );

        std::fs::write(&file_path, b"edited behind Flake's back").unwrap();
        source_check::check_source(&mut store, "owner", &source_id).unwrap();
        let view = resume(&store, &project_id).unwrap();
        assert_eq!(view.stale_or_missing_evidence.len(), 1);
        assert_eq!(view.stale_or_missing_evidence[0].source_id, source_id);
        assert_eq!(
            view.stale_or_missing_evidence[0].latest_check.status,
            CheckStatus::Changed
        );
        cleanup(&root);
    }

    #[test]
    fn next_actions_excludes_terminal_states() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let mut writer = store.writer().unwrap();
        let (open_out, _) =
            project::create_action(&mut writer, "owner", &project_id, "open one", None, &[])
                .unwrap();
        let (done_out, _) =
            project::create_action(&mut writer, "owner", &project_id, "done one", None, &[])
                .unwrap();
        drop(writer);
        {
            let mut writer = store.writer().unwrap();
            project::complete_action(
                &mut writer,
                "owner",
                &done_out.object_id,
                &done_out.revision_id,
                "finished",
                None,
            )
            .unwrap();
        }

        let view = resume(&store, &project_id).unwrap();
        let ids: Vec<&str> = view
            .next_actions
            .iter()
            .map(|(id, _)| id.as_str())
            .collect();
        assert!(ids.contains(&open_out.object_id.as_str()));
        assert!(!ids.contains(&done_out.object_id.as_str()));
        cleanup(&root);
    }

    #[test]
    fn checkpoint_narrows_relevant_notes_and_changes() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, Some("before"), "b").unwrap();
        }
        checkpoint::mark_reviewed_through(&mut store, "owner", &project_id, None, None).unwrap();
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, Some("after"), "a").unwrap();
        }

        let view = resume(&store, &project_id).unwrap();
        assert_eq!(view.relevant_notes.len(), 1);
        assert_eq!(view.relevant_notes[0].1.title.as_deref(), Some("after"));
        assert!(view
            .changes_since_checkpoint
            .iter()
            .all(|c| c.recorded_seq > view.reviewed_through_seq.unwrap()));
        cleanup(&root);
    }

    #[test]
    fn resume_never_writes_a_checkpoint() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        resume(&store, &project_id).unwrap();
        resume(&store, &project_id).unwrap();
        assert_eq!(
            checkpoint::current_checkpoint(&store, &project_id).unwrap(),
            None,
            "opening/computing resume must never itself mark reviewed (§12)"
        );
        cleanup(&root);
    }

    #[test]
    fn resume_output_is_stable_across_repeated_calls_at_the_same_snapshot() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let mut writer = store.writer().unwrap();
        project::create_action(&mut writer, "owner", &project_id, "a", None, &[]).unwrap();
        drop(writer);

        let first = resume(&store, &project_id).unwrap();
        let second = resume(&store, &project_id).unwrap();
        assert_eq!(first, second);
        cleanup(&root);
    }
}
