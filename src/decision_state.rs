//! Deterministic resolution of "what decision is currently accepted" for a
//! given `(project, decision_key)`, without hiding conflict or negative
//! evidence (`T03-02`).
//!
//! **Why a new module, not `temporal.rs`/`memory.rs`.** The task's own plan
//! row names `src/temporal.rs and memory values or successor decision
//! resolver` — read together with §15's actual `Decision` entity (lifecycle
//! `draft/accepted/superseded/withdrawn/tombstoned`, a half-open
//! `[valid_from, valid_to)` interval, and *explicit* owner
//! acceptance/supersession) rather than the historical Phase T `Memory`
//! type's five-rung confidence ladder, "successor decision resolver" is
//! this new, standalone module — not a reuse or edit of `temporal.rs`.
//! `temporal.rs`/`memory.rs` are immutable Phase T evidence
//! (`AGENTS.md` §3); this crate's own established precedent
//! (`relation.rs`'s non-reuse of `temporal::validate_supersession`, and
//! `source_check.rs`'s non-reuse of `capture::capture_file`) is to
//! reimplement the *algorithm shape* standalone when the surrounding
//! contract differs, never to couple a live Flake-v1 feature to historical
//! code. The shape actually reused here is narrow: `temporal.rs`'s own
//! resolver terminates ambiguity in `Contradiction`, never a manufactured
//! confidence number ("there is no rung 6") — this module applies the same
//! discipline, deliberately without reimplementing any of Phase T's five
//! ranking rungs, because `Decision`'s lifecycle already makes basis/
//! verification/recency ranking unnecessary (see below).
//!
//! **No ranking ladder — a deliberate simplification, not a missing
//! feature.** Phase T's resolver breaks ties between candidates by
//! verification, basis, scope specificity, then recency. This task's own
//! acceptance criterion is narrower and stricter: "Resolve only explicit
//! accepted supersession; incomparable overlapping decisions with same key
//! produce Conflict" — no other tie-break exists. This is also structurally
//! forced: `project::supersede_decision` (`T02-03`) always demotes the
//! superseded decision's lifecycle away from `Accepted` in the same atomic
//! step that creates the `Supersedes` relation, so two decisions sharing a
//! key can never both be `Accepted` *and* connected by an explicit
//! supersession edge at the same time — by the time a supersession is
//! recorded, the loser is no longer a candidate. Consequently, whenever
//! more than one `Accepted` decision for the same key is simultaneously
//! valid, that is unconditionally an unresolved conflict (`NeedsReview`),
//! never something this resolver silently breaks a tie on (I06/I08: no
//! latest-wins, no model adjudication).
//!
//! **Negative evidence stays visible.** [`resolve_decision_state`] does not
//! return only the winner: `considered` lists *every* decision object that
//! shares the requested `(project, decision_key)`, whether or not it was
//! admitted, each carrying `exclusion_reason` when it was not (its
//! lifecycle, or the requested valid-time point falling outside its
//! recorded `[valid_from, valid_to)`). A withdrawn decision's own
//! `withdrawal_reason` is folded into its `exclusion_reason` rather than
//! silently dropped.
//!
//! **As-of is one deterministic reconstruction, not two code paths.**
//! Resolving "now" and resolving a past point both walk
//! [`crate::canonical::CanonicalStore::all_revisions`] and keep, per
//! object, the latest revision with `recorded_seq <= as_of_recorded` —
//! `as_of_recorded` defaults to the vault's current transaction head, so
//! "current state" is not a special case with its own separate logic that
//! could silently drift from the historical path.

use crate::canonical::CanonicalStore;
use crate::capture::{now_rfc3339_utc, parse_rfc3339_utc};
use crate::project::{self, Decision, DecisionLifecycle, RecordPayload};
use crate::{Error, Result};
use serde::Serialize;
use std::collections::HashMap;

/// One decision object sharing the requested `(project, decision_key)`,
/// reconstructed as of the resolution's `as_of_recorded` cutoff.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConsideredDecision {
    pub decision_id: String,
    /// The decision's own fields as of `as_of_recorded` — not necessarily
    /// its current live state if a later revision exists beyond the cutoff.
    pub decision: Decision,
    /// `true` only if this decision is `Accepted` and `as_of_valid` falls
    /// within its recorded `[valid_from, valid_to)`.
    pub admitted: bool,
    /// Always `Some` when `admitted` is `false`; always `None` when it is
    /// `true`.
    pub exclusion_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionOutcome {
    /// Exactly one admitted decision. Carries its `decision_id`.
    CurrentSet(String),
    /// Two or more admitted decisions, none resolved by explicit
    /// supersession — a genuine, surfaced conflict.
    NeedsReview,
    /// No decision for this key is currently accepted and valid at
    /// `as_of_valid`. `considered` still names every candidate that was
    /// ruled out, and why.
    NoAcceptedDecision,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DecisionResolution {
    pub project_id: String,
    pub decision_key: String,
    /// The exact RFC 3339 UTC instant actually used (echoes the caller's
    /// `as_of_valid` when given; otherwise the resolution's own
    /// observation time).
    pub as_of_valid: String,
    /// The exact recorded-sequence cutoff actually used (echoes the
    /// caller's `as_of_recorded` when given; otherwise the vault's current
    /// transaction head).
    pub as_of_recorded: i64,
    pub outcome: DecisionOutcome,
    /// Every decision object sharing `(project_id, decision_key)`,
    /// admitted or not, in stable `decision_id`-ascending order — the same
    /// order on every process/run (V05 "stable deterministic ordering").
    pub considered: Vec<ConsideredDecision>,
}

/// Resolve the current accepted state of one `(project_id, decision_key)`
/// question as of `as_of_valid` (an RFC 3339 UTC instant; `None` means
/// "now") and `as_of_recorded` (a recorded-sequence cutoff; `None` means
/// the vault's current transaction head).
pub fn resolve_decision_state(
    store: &CanonicalStore,
    project_id: &str,
    decision_key: &str,
    as_of_valid: Option<&str>,
    as_of_recorded: Option<i64>,
) -> Result<DecisionResolution> {
    project::require_project(store, project_id)?;
    if decision_key.is_empty() || decision_key.len() > 128 {
        return Err(Error::Project(format!(
            "decision_key must be 1-128 bytes, got {}",
            decision_key.len()
        )));
    }

    let as_of_valid_str = match as_of_valid {
        Some(s) => {
            parse_rfc3339_utc(s)?;
            s.to_string()
        }
        None => now_rfc3339_utc(),
    };
    let as_of_valid_secs = parse_rfc3339_utc(&as_of_valid_str)?;

    let (head_seq, _) = store.transaction_head()?;
    let cutoff = match as_of_recorded {
        Some(s) => {
            if s < 0 || s > head_seq {
                return Err(Error::Project(format!(
                    "as_of_recorded {s} is out of range for this vault's current head {head_seq}"
                )));
            }
            s
        }
        None => head_seq,
    };

    let mut considered: Vec<ConsideredDecision> = decisions_as_of(store, cutoff)?
        .into_iter()
        .filter(|(_, d)| d.project_id == project_id && d.decision_key == decision_key)
        .map(|(decision_id, decision)| {
            let (admitted, exclusion_reason) =
                admission(&decision, as_of_valid_secs, &as_of_valid_str);
            ConsideredDecision {
                decision_id,
                decision,
                admitted,
                exclusion_reason,
            }
        })
        .collect();
    considered.sort_by(|a, b| a.decision_id.cmp(&b.decision_id));

    let admitted_count = considered.iter().filter(|c| c.admitted).count();
    let outcome = match admitted_count {
        0 => DecisionOutcome::NoAcceptedDecision,
        1 => DecisionOutcome::CurrentSet(
            considered
                .iter()
                .find(|c| c.admitted)
                .expect("admitted_count == 1 implies exactly one admitted entry")
                .decision_id
                .clone(),
        ),
        _ => DecisionOutcome::NeedsReview,
    };

    Ok(DecisionResolution {
        project_id: project_id.to_string(),
        decision_key: decision_key.to_string(),
        as_of_valid: as_of_valid_str,
        as_of_recorded: cutoff,
        outcome,
        considered,
    })
}

/// Is `decision` admitted at `as_of_valid_secs`, and if not, why?
fn admission(
    decision: &Decision,
    as_of_valid_secs: i64,
    as_of_valid_str: &str,
) -> (bool, Option<String>) {
    if decision.lifecycle != DecisionLifecycle::Accepted {
        let mut reason = format!("lifecycle is {:?}, not Accepted", decision.lifecycle);
        if let Some(r) = &decision.withdrawal_reason {
            reason.push_str(&format!(" (withdrawal reason: {r:?})"));
        }
        return (false, Some(reason));
    }
    if let Some(from) = &decision.valid_from {
        match parse_rfc3339_utc(from) {
            Ok(from_secs) if as_of_valid_secs < from_secs => {
                return (
                    false,
                    Some(format!(
                        "as_of_valid {as_of_valid_str} is before valid_from {from}"
                    )),
                );
            }
            Ok(_) => {}
            Err(_) => {
                return (
                    false,
                    Some(format!("valid_from {from:?} could not be parsed")),
                );
            }
        }
    }
    if let Some(to) = &decision.valid_to {
        match parse_rfc3339_utc(to) {
            Ok(to_secs) if as_of_valid_secs >= to_secs => {
                return (
                    false,
                    Some(format!(
                        "as_of_valid {as_of_valid_str} is at or after valid_to {to} (half-open [from, to) interval)"
                    )),
                );
            }
            Ok(_) => {}
            Err(_) => {
                return (false, Some(format!("valid_to {to:?} could not be parsed")));
            }
        }
    }
    (true, None)
}

/// Every `Decision` object in the vault, reconstructed as of `cutoff`: for
/// each `object_id`, the payload of its latest revision with
/// `recorded_seq <= cutoff`, if that payload is a `Decision`. A single pass
/// over [`CanonicalStore::all_revisions`] (ascending `recorded_seq`, per
/// its own doc comment), so the last write for each `object_id` still
/// admitted by the cutoff is always the one retained.
fn decisions_as_of(store: &CanonicalStore, cutoff: i64) -> Result<Vec<(String, Decision)>> {
    let mut latest: HashMap<String, String> = HashMap::new();
    for rev in store.all_revisions()? {
        if rev.recorded_seq > cutoff {
            continue;
        }
        latest.insert(rev.object_id, rev.payload);
    }
    let mut out = Vec::new();
    for (object_id, payload) in latest {
        if let Ok(record) = RecordPayload::from_json(&payload) {
            if let Ok(decision) = record.as_decision() {
                out.push((object_id, decision.clone()));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::CanonicalStore;
    use crate::project::{self, DecisionBasis, DecisionVerification};
    use std::path::{Path, PathBuf};

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-decision-state-{}", uuid::Uuid::now_v7()))
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

    #[allow(clippy::too_many_arguments)]
    fn make_decision(
        store: &mut CanonicalStore,
        project_id: &str,
        key: &str,
        statement: &str,
        valid_from: Option<&str>,
        valid_to: Option<&str>,
    ) -> (String, String) {
        let mut writer = store.writer().unwrap();
        let (outcome, _d) = project::create_decision(
            &mut writer,
            "owner",
            project_id,
            key,
            statement,
            None,
            DecisionBasis::UserJudgment,
            DecisionVerification::Unreviewed,
            valid_from,
            valid_to,
        )
        .unwrap();
        (outcome.object_id, outcome.revision_id)
    }

    fn accept(store: &mut CanonicalStore, id: &str, expect: &str) -> String {
        let mut writer = store.writer().unwrap();
        let (outcome, _d) = project::accept_decision(&mut writer, "owner", id, expect).unwrap();
        outcome.revision_id
    }

    #[test]
    fn no_accepted_decision_when_nothing_exists_for_the_key() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let resolution =
            resolve_decision_state(&store, &project_id, "unused-key", None, None).unwrap();
        assert_eq!(resolution.outcome, DecisionOutcome::NoAcceptedDecision);
        assert!(resolution.considered.is_empty());
        cleanup(&root);
    }

    #[test]
    fn no_accepted_decision_when_only_a_draft_exists() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        make_decision(&mut store, &project_id, "k", "s", None, None);
        let resolution = resolve_decision_state(&store, &project_id, "k", None, None).unwrap();
        assert_eq!(resolution.outcome, DecisionOutcome::NoAcceptedDecision);
        assert_eq!(resolution.considered.len(), 1);
        assert!(!resolution.considered[0].admitted);
        assert!(resolution.considered[0]
            .exclusion_reason
            .as_deref()
            .unwrap()
            .contains("Draft"));
        cleanup(&root);
    }

    #[test]
    fn current_set_when_exactly_one_decision_is_accepted() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (id, rev) = make_decision(&mut store, &project_id, "k", "s", None, None);
        accept(&mut store, &id, &rev);
        let resolution = resolve_decision_state(&store, &project_id, "k", None, None).unwrap();
        assert_eq!(resolution.outcome, DecisionOutcome::CurrentSet(id.clone()));
        assert_eq!(resolution.considered.len(), 1);
        assert!(resolution.considered[0].admitted);
        assert!(resolution.considered[0].exclusion_reason.is_none());
        cleanup(&root);
    }

    #[test]
    fn needs_review_when_two_independently_accepted_decisions_share_a_key_and_overlap() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (id_a, rev_a) = make_decision(&mut store, &project_id, "k", "a", None, None);
        let (id_b, rev_b) = make_decision(&mut store, &project_id, "k", "b", None, None);
        accept(&mut store, &id_a, &rev_a);
        accept(&mut store, &id_b, &rev_b);
        let resolution = resolve_decision_state(&store, &project_id, "k", None, None).unwrap();
        assert_eq!(resolution.outcome, DecisionOutcome::NeedsReview);
        assert_eq!(
            resolution.considered.iter().filter(|c| c.admitted).count(),
            2
        );
        let mut ids: Vec<&str> = resolution
            .considered
            .iter()
            .map(|c| c.decision_id.as_str())
            .collect();
        ids.sort();
        let mut expected = vec![id_a.as_str(), id_b.as_str()];
        expected.sort();
        assert_eq!(ids, expected);
        cleanup(&root);
    }

    #[test]
    fn non_overlapping_valid_intervals_for_the_same_key_are_not_a_conflict() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (id_a, rev_a) = make_decision(
            &mut store,
            &project_id,
            "k",
            "a",
            Some("2024-01-01T00:00:00Z"),
            Some("2024-06-01T00:00:00Z"),
        );
        let (id_b, rev_b) = make_decision(
            &mut store,
            &project_id,
            "k",
            "b",
            Some("2024-06-01T00:00:00Z"),
            None,
        );
        accept(&mut store, &id_a, &rev_a);
        accept(&mut store, &id_b, &rev_b);

        let before =
            resolve_decision_state(&store, &project_id, "k", Some("2024-03-01T00:00:00Z"), None)
                .unwrap();
        assert_eq!(before.outcome, DecisionOutcome::CurrentSet(id_a.clone()));

        // Exactly the boundary instant: [from, to) means A has ended and B has
        // begun (half-open) — B alone is current, not both, not neither.
        let boundary =
            resolve_decision_state(&store, &project_id, "k", Some("2024-06-01T00:00:00Z"), None)
                .unwrap();
        assert_eq!(boundary.outcome, DecisionOutcome::CurrentSet(id_b.clone()));
        cleanup(&root);
    }

    #[test]
    fn explicit_supersession_leaves_exactly_one_candidate_never_a_conflict() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (old_id, old_rev) = make_decision(&mut store, &project_id, "k", "old", None, None);
        let old_rev = accept(&mut store, &old_id, &old_rev);
        let (new_id, new_rev) = make_decision(&mut store, &project_id, "k", "new", None, None);
        let new_rev = accept(&mut store, &new_id, &new_rev);

        project::supersede_decision(&mut store, "owner", &new_id, &old_id, &old_rev, "because")
            .unwrap();
        let _ = new_rev; // superseding decision's own revision id not needed further

        let resolution = resolve_decision_state(&store, &project_id, "k", None, None).unwrap();
        assert_eq!(resolution.outcome, DecisionOutcome::CurrentSet(new_id));
        assert_eq!(resolution.considered.len(), 2);
        let old_entry = resolution
            .considered
            .iter()
            .find(|c| c.decision_id == old_id)
            .unwrap();
        assert!(!old_entry.admitted);
        assert!(old_entry
            .exclusion_reason
            .as_deref()
            .unwrap()
            .contains("Superseded"));
        cleanup(&root);
    }

    #[test]
    fn withdrawal_reason_is_surfaced_as_negative_evidence() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (id, rev) = make_decision(&mut store, &project_id, "k", "s", None, None);
        let rev = accept(&mut store, &id, &rev);
        {
            let mut writer = store.writer().unwrap();
            project::withdraw_decision(&mut writer, "owner", &id, &rev, "changed our mind")
                .unwrap();
        }
        let resolution = resolve_decision_state(&store, &project_id, "k", None, None).unwrap();
        assert_eq!(resolution.outcome, DecisionOutcome::NoAcceptedDecision);
        assert!(resolution.considered[0]
            .exclusion_reason
            .as_deref()
            .unwrap()
            .contains("changed our mind"));
        cleanup(&root);
    }

    #[test]
    fn different_projects_with_the_same_key_never_collide() {
        let (root, mut store) = open_store();
        let project_a = make_project(&mut store);
        let project_b = make_project(&mut store);
        let (id_a, rev_a) = make_decision(&mut store, &project_a, "k", "a", None, None);
        accept(&mut store, &id_a, &rev_a);
        let (id_b, rev_b) = make_decision(&mut store, &project_b, "k", "b", None, None);
        accept(&mut store, &id_b, &rev_b);

        let resolution_a = resolve_decision_state(&store, &project_a, "k", None, None).unwrap();
        assert_eq!(resolution_a.outcome, DecisionOutcome::CurrentSet(id_a));
        assert_eq!(resolution_a.considered.len(), 1);

        let resolution_b = resolve_decision_state(&store, &project_b, "k", None, None).unwrap();
        assert_eq!(resolution_b.outcome, DecisionOutcome::CurrentSet(id_b));
        assert_eq!(resolution_b.considered.len(), 1);
        cleanup(&root);
    }

    #[test]
    fn as_of_recorded_reconstructs_a_past_state_before_a_later_acceptance() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (id, rev) = make_decision(&mut store, &project_id, "k", "s", None, None);
        let cutoff_before_accept = store.transaction_head().unwrap().0;
        accept(&mut store, &id, &rev);

        let past =
            resolve_decision_state(&store, &project_id, "k", None, Some(cutoff_before_accept))
                .unwrap();
        assert_eq!(past.outcome, DecisionOutcome::NoAcceptedDecision);
        assert!(!past.considered[0].admitted);

        let now = resolve_decision_state(&store, &project_id, "k", None, None).unwrap();
        assert_eq!(now.outcome, DecisionOutcome::CurrentSet(id));
        cleanup(&root);
    }

    #[test]
    fn as_of_recorded_out_of_range_is_refused() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let head = store.transaction_head().unwrap().0;
        let err =
            resolve_decision_state(&store, &project_id, "k", None, Some(head + 1000)).unwrap_err();
        assert!(format!("{err}").contains("out of range"));
        cleanup(&root);
    }

    #[test]
    fn resolution_ordering_is_stable_across_repeated_calls() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (id_a, rev_a) = make_decision(&mut store, &project_id, "k", "a", None, None);
        let (id_b, rev_b) = make_decision(&mut store, &project_id, "k", "b", None, None);
        accept(&mut store, &id_a, &rev_a);
        accept(&mut store, &id_b, &rev_b);

        let first = resolve_decision_state(&store, &project_id, "k", None, None).unwrap();
        let second = resolve_decision_state(&store, &project_id, "k", None, None).unwrap();
        let first_ids: Vec<&str> = first
            .considered
            .iter()
            .map(|c| c.decision_id.as_str())
            .collect();
        let second_ids: Vec<&str> = second
            .considered
            .iter()
            .map(|c| c.decision_id.as_str())
            .collect();
        assert_eq!(first_ids, second_ids);
        let mut sorted = first_ids.clone();
        sorted.sort();
        assert_eq!(
            first_ids, sorted,
            "considered must already be decision_id-sorted"
        );
        cleanup(&root);
    }

    // ---- Property test: an independently-written reference oracle -------
    //
    // §29 V05 / this task's own "Verification method": a *pure reference
    // oracle separate from production resolver*, cross-checked over
    // randomly generated scenarios with a recorded seed. `reference_oracle`
    // below deliberately does not call `resolve_decision_state` or
    // `decisions_as_of`/`admission` — it re-derives the same DecisionOutcome
    // from the same committed vault using its own, structurally different
    // route: per-object binary search over that object's own
    // `CanonicalStore::history` (never `all_revisions`'s single global
    // fold), and its own independent half-open-interval/lifecycle check
    // written from scratch rather than calling `admission`.
    mod reference_oracle {
        use super::*;

        /// Recorded-sequence position of one historical revision, looked up
        /// independently of `all_revisions` by scanning
        /// `CanonicalStore::revisions_since(0)` once (a different query
        /// shape/tuple than `all_revisions`'s full envelope) purely to
        /// build a `revision_id -> recorded_seq` map this oracle can use
        /// while walking each object's own `history()` — which does not
        /// itself carry `recorded_seq`.
        fn recorded_seq_by_revision(store: &CanonicalStore) -> HashMap<String, i64> {
            let mut map = HashMap::new();
            for (seq, _object_id, revision_id, _payload) in store.revisions_since(0).unwrap() {
                map.insert(revision_id, seq);
            }
            map
        }

        /// For one object, the payload of the latest revision (via
        /// `CanonicalStore::history`, oldest-first) with `recorded_seq <=
        /// cutoff` — found by walking from the end backwards (a linear
        /// reverse scan, not the production path's forward fold), stopping
        /// at the first admissible revision.
        fn latest_payload_as_of(
            store: &CanonicalStore,
            object_id: &str,
            cutoff: i64,
            seqs: &HashMap<String, i64>,
        ) -> Option<String> {
            let history = store.history(object_id).unwrap();
            for (revision_id, _parent, payload) in history.into_iter().rev() {
                let seq = *seqs
                    .get(&revision_id)
                    .expect("every history revision_id must appear in revisions_since(0)");
                if seq <= cutoff {
                    return Some(payload);
                }
            }
            None
        }

        pub fn resolve(
            store: &CanonicalStore,
            project_id: &str,
            decision_key: &str,
            as_of_valid_secs: i64,
            cutoff: i64,
        ) -> DecisionOutcome {
            let seqs = recorded_seq_by_revision(store);
            let mut object_ids: Vec<String> = store
                .list_current_objects()
                .unwrap()
                .into_iter()
                .map(|(object_id, _rev, _payload)| object_id)
                .collect();
            object_ids.sort();

            let mut admitted_ids: Vec<String> = Vec::new();
            for object_id in object_ids {
                let Some(payload) = latest_payload_as_of(store, &object_id, cutoff, &seqs) else {
                    continue;
                };
                let Ok(record) = RecordPayload::from_json(&payload) else {
                    continue;
                };
                let Ok(decision) = record.as_decision() else {
                    continue;
                };
                if decision.project_id != project_id || decision.decision_key != decision_key {
                    continue;
                }
                if decision.lifecycle != DecisionLifecycle::Accepted {
                    continue;
                }
                let starts_ok = match &decision.valid_from {
                    None => true,
                    Some(f) => parse_rfc3339_utc(f)
                        .map(|s| s <= as_of_valid_secs)
                        .unwrap_or(false),
                };
                let ends_ok = match &decision.valid_to {
                    None => true,
                    Some(t) => parse_rfc3339_utc(t)
                        .map(|s| as_of_valid_secs < s)
                        .unwrap_or(false),
                };
                if starts_ok && ends_ok {
                    admitted_ids.push(object_id);
                }
            }

            match admitted_ids.len() {
                0 => DecisionOutcome::NoAcceptedDecision,
                1 => DecisionOutcome::CurrentSet(admitted_ids.remove(0)),
                _ => DecisionOutcome::NeedsReview,
            }
        }
    }

    /// A tiny, dependency-free, seeded PRNG (SplitMix64) — this crate has no
    /// `rand`/`proptest` dependency, and one is not worth adding for a
    /// single bounded generator (§: "prove necessity before adding a
    /// runtime dependency"). Deterministic and reproducible from `seed`
    /// alone, which is what "record generated seed" requires.
    struct SplitMix64(u64);

    impl SplitMix64 {
        fn next_u64(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
            z ^ (z >> 31)
        }

        fn next_range(&mut self, n: u64) -> u64 {
            self.next_u64() % n
        }

        fn next_bool(&mut self) -> bool {
            self.next_u64().is_multiple_of(2)
        }
    }

    /// Generates one random decision-lifecycle scenario for a fixed
    /// `(project_id, "k")`, then asserts the production resolver and the
    /// independently-written `reference_oracle` agree on every one of a
    /// fixed grid of `(as_of_valid, as_of_recorded)` probe points.
    ///
    /// `seed = 20260915` (this task's own evidence-capture date, chosen for
    /// reproducibility, not tuned to pass) drives 200 generated scenarios;
    /// the exact seed and count are recorded here and in the evidence
    /// report so a failure is reproducible without external state.
    #[test]
    fn production_resolver_agrees_with_the_reference_oracle_across_random_scenarios() {
        const SEED: u64 = 20260915;
        const SCENARIOS: usize = 200;
        let mut rng = SplitMix64(SEED);

        for scenario in 0..SCENARIOS {
            let (root, mut store) = open_store();
            let project_id = make_project(&mut store);
            let decision_count = 1 + rng.next_range(4) as usize; // 1..=4
            let mut decision_ids = Vec::new();
            let mut probe_cutoffs = Vec::new();

            for _ in 0..decision_count {
                let has_from = rng.next_bool();
                let has_to = rng.next_bool();
                let base = 1_700_000_000i64 + rng.next_range(20) as i64 * 86_400;
                let from = if has_from {
                    Some(crate::capture::rfc3339_utc_from_unix_seconds(base))
                } else {
                    None
                };
                let to = if has_to {
                    let extra = 1 + rng.next_range(20) as i64;
                    Some(crate::capture::rfc3339_utc_from_unix_seconds(
                        base + extra * 86_400,
                    ))
                } else {
                    None
                };
                let (id, rev) = make_decision(
                    &mut store,
                    &project_id,
                    "k",
                    "s",
                    from.as_deref(),
                    to.as_deref(),
                );
                probe_cutoffs.push(store.transaction_head().unwrap().0);
                // Randomly accept, leave as draft, or accept-then-withdraw.
                match rng.next_range(3) {
                    0 => {
                        accept(&mut store, &id, &rev);
                    }
                    1 => {
                        let rev = accept(&mut store, &id, &rev);
                        let mut writer = store.writer().unwrap();
                        project::withdraw_decision(&mut writer, "owner", &id, &rev, "r").unwrap();
                    }
                    _ => {}
                }
                probe_cutoffs.push(store.transaction_head().unwrap().0);
                decision_ids.push(id);
            }

            let as_of_valid_probes = [
                1_700_000_000i64 - 86_400,
                1_700_000_000i64 + 10 * 86_400,
                1_700_000_000i64 + 40 * 86_400,
            ];
            for &as_of_valid_secs in &as_of_valid_probes {
                for &cutoff in &probe_cutoffs {
                    let as_of_valid_str =
                        crate::capture::rfc3339_utc_from_unix_seconds(as_of_valid_secs);
                    let production = resolve_decision_state(
                        &store,
                        &project_id,
                        "k",
                        Some(&as_of_valid_str),
                        Some(cutoff),
                    )
                    .unwrap();
                    let oracle = reference_oracle::resolve(
                        &store,
                        &project_id,
                        "k",
                        as_of_valid_secs,
                        cutoff,
                    );
                    assert_eq!(
                        production.outcome, oracle,
                        "scenario {scenario} disagreed at as_of_valid_secs={as_of_valid_secs} cutoff={cutoff} decisions={decision_ids:?}"
                    );
                }
            }
            cleanup(&root);
        }
    }
}
