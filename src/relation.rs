//! `Relation`: typed, project-scoped links between two typed records
//! (`T02-03`).
//!
//! **Architecture.** Exactly like `Source` (`T02-02`), `Relation` is a sixth
//! [`crate::project::RecordPayload`] kind — an ordinary opaque-payload
//! canonical object committed through the existing
//! `CommandTarget::CreateObject` and `CanonicalWriter::commit`. No new
//! `CommandTarget` variant, no schema change, no event store, no graph
//! engine: the "graph" this module walks for cycle rejection is assembled at
//! call time from an ordinary full scan of already-admitted `Relation`
//! objects (`list_current_objects`), the same documented-limitation scan
//! `project::list_project_records`/`capture::list_project_sources` already
//! use.
//!
//! **What this owns (§15 `Relation` row).** "Project, typed endpoints,
//! relation type (supports, contradicts, depends-on, relates-to,
//! supersedes), endpoint revision policy... A supersession must be
//! owner-accepted, acyclic and within project; links do not imply current
//! truth." This module is the sole place a `Relation` is admitted, so it is
//! the sole enforcement point for all of that in one place.
//!
//! **Endpoint revision policy — the one this task implements.** Every
//! `Relation` pins both endpoints to the exact revision that was current at
//! the moment the relation was created (`from_revision_id`/`to_revision_id`).
//! This is a deliberate, documented choice among possible policies: a pinned
//! citation ("this exact decision text cited this exact source content") is
//! the more precise, more immutable-history-friendly reading of "endpoint
//! revision policy" than a floating "always resolve to current" policy would
//! be, and it costs nothing extra to record since the current revision is
//! already read to validate the endpoint exists. A floating-reference policy
//! is not implemented and is not needed by anything in this task's own
//! scope; recorded here as a deliberate deferral, not a silent omission.
//!
//! **Endpoint kind restriction.** Any two `Note`/`Action`/`Decision`/
//! `Source` objects in the *same* project may be linked by
//! `Supports`/`Contradicts`/`DependsOn`/`RelatesTo` — these are descriptive
//! links; §15 explicitly says "links do not imply current truth". `Project`
//! and `Relation` objects themselves may never be an endpoint (a relation
//! about a relation, or about the project container itself, is out of this
//! task's scope and not needed by anything here).
//!
//! `Supersedes` is further restricted to a `Decision` endpoint on **both**
//! sides. `T02-03`'s own task contract names only `Decision`
//! "override/supersession semantics" — nothing in this task's scope needs a
//! non-`Decision` supersession, and admitting one now would invent
//! semantics (e.g. "does superseding a `Source` deactivate it?") this plan
//! never specifies. This is a deliberate, narrow, documented scope limit,
//! not a structural ceiling: lifting it later needs no schema change, only
//! relaxing this one check.
//!
//! **Cycle rejection is `Supersedes`-only.** §15 names acyclicity only for
//! supersession ("A supersession must be ... acyclic"); the other four
//! relation types are plain descriptive edges with no such requirement. The
//! walk here is a direct, independent reimplementation of the same
//! depth-first "does the target already reach the source" shape
//! `temporal::validate_supersession`'s `reaches` helper already uses for the
//! historical Phase T `Memory` graph — deliberately **not** shared code,
//! because `temporal.rs` is coupled to `Memory` (a Phase T/Fehrest type this
//! plan's own product model does not use — see `AGENTS.md` §3, historical
//! evidence is immutable and not extended for later work). Reusing the
//! *algorithm shape* without coupling this task's new `Relation` type to an
//! unrelated historical data model is the deliberate choice.
//!
//! This module reuses [`crate::Error::InvalidSupersession`] for every
//! `Supersedes`-specific rejection (self-supersession, wrong endpoint kind,
//! cycle) — that variant already existed, unused by any other module, with
//! exactly this documented purpose ("An invalid supersession edge. Never
//! silently normalised").

use crate::canonical::{CanonicalStore, CanonicalWriter, CommandOutcome, RecordOrigin};
use crate::project::{self, RecordPayload};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Map as JsonMap;
use std::collections::HashSet;

/// No dedicated §27 limit is named for a relation note; reuses `project`'s
/// general body bound, exactly like `capture::MAX_SOURCE_LABEL_BYTES` reuses
/// `project::MAX_TITLE_BYTES` for the same reason.
pub const MAX_RELATION_NOTE_BYTES: usize = project::MAX_BODY_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    Supports,
    Contradicts,
    DependsOn,
    RelatesTo,
    Supersedes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relation {
    pub payload_schema_version: u32,
    pub project_id: String,
    pub relation_type: RelationType,
    pub from_object_id: String,
    /// Pinned at creation — see module docs, "Endpoint revision policy".
    pub from_revision_id: String,
    pub to_object_id: String,
    pub to_revision_id: String,
    pub note: Option<String>,
    #[serde(flatten)]
    pub unknown: JsonMap<String, serde_json::Value>,
}

/// Read `object_id`'s current revision, confirm it is a project-scoped work
/// record (`Note`/`Action`/`Decision`/`Source`, never `Project` or another
/// `Relation`) belonging to `project_id`. Shared validation for both
/// endpoints of a proposed relation.
fn read_endpoint(
    store: &CanonicalStore,
    project_id: &str,
    object_id: &str,
) -> Result<(String, RecordPayload)> {
    let (revision_id, payload) = store.read_current(object_id)?.ok_or_else(|| {
        Error::Project(format!(
            "invalid relation endpoint: no object exists with id {object_id}"
        ))
    })?;
    let record = RecordPayload::from_json(&payload)?;
    if matches!(
        record,
        RecordPayload::Project(_) | RecordPayload::Relation(_)
    ) {
        return Err(Error::Project(format!(
            "object {object_id} ({}) cannot be a relation endpoint",
            record.kind_str()
        )));
    }
    match record.project_id_of() {
        Some(owner) if owner == project_id => Ok((revision_id, record)),
        Some(_) => Err(Error::Project(format!(
            "relation endpoint {object_id} belongs to a different project; cross-project relations are not permitted in v1"
        ))),
        None => unreachable!("Project/Relation already excluded above"),
    }
}

/// Every currently-admitted `Relation` in `project_id` whose `relation_type`
/// is `Supersedes`. A full scan — same documented limitation every other
/// `list_project_*` function in this crate already carries.
fn supersedes_edges(store: &CanonicalStore, project_id: &str) -> Result<Vec<Relation>> {
    let mut out = Vec::new();
    for (_, _, payload) in store.list_current_objects()? {
        if let Ok(RecordPayload::Relation(r)) = RecordPayload::from_json(&payload) {
            if r.project_id == project_id && r.relation_type == RelationType::Supersedes {
                out.push(r);
            }
        }
    }
    Ok(out)
}

/// Does a path of existing `Supersedes` edges lead from `start` to `target`?
/// Used to reject `create_relation(Supersedes, from: new, to: old)` when
/// `old` already (directly or transitively) supersedes `new` — admitting the
/// new edge would close a cycle.
fn reaches(edges: &[Relation], start: &str, target: &str) -> bool {
    let mut seen = HashSet::new();
    let mut stack = vec![start.to_string()];
    while let Some(cur) = stack.pop() {
        if cur == target {
            return true;
        }
        if !seen.insert(cur.clone()) {
            continue;
        }
        for e in edges {
            if e.from_object_id == cur {
                stack.push(e.to_object_id.clone());
            }
        }
    }
    false
}

/// Create a `Relation` from `from_object_id` to `to_object_id` within
/// `project_id`. Rejects before any transaction opens: invalid/missing/
/// cross-project/wrong-kind endpoints, a self-loop, an over-limit note, or
/// (for `Supersedes` only) a non-`Decision` endpoint or a cycle.
#[allow(clippy::too_many_arguments)]
pub fn create_relation(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    project_id: &str,
    relation_type: RelationType,
    from_object_id: &str,
    to_object_id: &str,
    note: Option<&str>,
) -> Result<(CommandOutcome, Relation)> {
    project::require_project(writer.store(), project_id)?;
    if from_object_id == to_object_id {
        return Err(Error::Project(
            "relation endpoints must be different objects; a self-loop is not a relation".into(),
        ));
    }
    if let Some(n) = note {
        project::check_len("relation note", n, MAX_RELATION_NOTE_BYTES)?;
    }
    let (from_revision_id, from_record) =
        read_endpoint(writer.store(), project_id, from_object_id)?;
    let (to_revision_id, to_record) = read_endpoint(writer.store(), project_id, to_object_id)?;

    if relation_type == RelationType::Supersedes {
        from_record.as_decision().map_err(|_| {
            Error::InvalidSupersession(format!(
                "supersedes relation requires a decision as its 'from' endpoint, found {} ({})",
                from_object_id,
                from_record.kind_str()
            ))
        })?;
        to_record.as_decision().map_err(|_| {
            Error::InvalidSupersession(format!(
                "supersedes relation requires a decision as its 'to' endpoint, found {} ({})",
                to_object_id,
                to_record.kind_str()
            ))
        })?;
        let edges = supersedes_edges(writer.store(), project_id)?;
        if reaches(&edges, to_object_id, from_object_id) {
            return Err(Error::InvalidSupersession(format!(
                "cycle: {to_object_id} already (transitively) supersedes {from_object_id}"
            )));
        }
    }

    let relation = Relation {
        payload_schema_version: project::RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: project_id.to_string(),
        relation_type,
        from_object_id: from_object_id.to_string(),
        from_revision_id,
        to_object_id: to_object_id.to_string(),
        to_revision_id,
        note: note.map(str::to_string),
        unknown: JsonMap::new(),
    };
    let (outcome, record) = project::commit_create(
        writer,
        actor,
        RecordOrigin::User,
        RecordPayload::Relation(relation),
    )?;
    Ok((outcome, record.as_relation()?.clone()))
}

/// Every `Relation` in `project_id` — full scan, same documented limitation
/// as `project::list_project_records`.
pub fn list_project_relations(
    store: &CanonicalStore,
    project_id: &str,
) -> Result<Vec<(String, Relation)>> {
    let mut out = Vec::new();
    for (object_id, _, payload) in store.list_current_objects()? {
        if let Ok(RecordPayload::Relation(r)) = RecordPayload::from_json(&payload) {
            if r.project_id == project_id {
                out.push((object_id, r));
            }
        }
    }
    Ok(out)
}

/// Every `Relation` touching `object_id` as either endpoint — how a caller
/// discovers a `Decision`'s or `Action`'s linked evidence (§15: "Evidence is
/// a source revision/artifact plus a relation, not a second copy" — nothing
/// in this crate stores evidence a second time on the `Decision`/`Action`
/// itself; this scan is the read path for it).
pub fn list_relations_for_object(
    store: &CanonicalStore,
    object_id: &str,
) -> Result<Vec<(String, Relation)>> {
    let mut out = Vec::new();
    for (rid, _, payload) in store.list_current_objects()? {
        if let Ok(RecordPayload::Relation(r)) = RecordPayload::from_json(&payload) {
            if r.from_object_id == object_id || r.to_object_id == object_id {
                out.push((rid, r));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::CanonicalStore;
    use std::path::{Path, PathBuf};

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-relation-{}", uuid::Uuid::now_v7()))
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

    fn new_decision(store: &mut CanonicalStore, project_id: &str, key: &str) -> String {
        let mut writer = store.writer().unwrap();
        project::create_decision(
            &mut writer,
            "owner",
            project_id,
            key,
            "statement",
            None,
            project::DecisionBasis::UserJudgment,
            project::DecisionVerification::Unreviewed,
            None,
            None,
        )
        .unwrap()
        .0
        .object_id
    }

    fn new_source(store: &mut CanonicalStore, project_id: &str) -> String {
        let mut writer = store.writer().unwrap();
        crate::capture::create_manual_reference(
            &mut writer,
            "owner",
            project_id,
            "l",
            None,
            None,
            None,
        )
        .unwrap()
        .0
        .object_id
    }

    #[test]
    fn links_a_decision_to_a_source_as_supporting_evidence() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let decision_id = new_decision(&mut store, &project_id, "k1");
        let source_id = new_source(&mut store, &project_id);

        let (outcome, relation) = {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &project_id,
                RelationType::Supports,
                &decision_id,
                &source_id,
                Some("cited evidence"),
            )
            .unwrap()
        };
        assert_eq!(relation.from_object_id, decision_id);
        assert_eq!(relation.to_object_id, source_id);

        let linked = list_relations_for_object(&store, &decision_id).unwrap();
        assert_eq!(linked.len(), 1);
        assert_eq!(linked[0].0, outcome.object_id);
        let linked_from_source = list_relations_for_object(&store, &source_id).unwrap();
        assert_eq!(linked_from_source.len(), 1);

        cleanup(&root);
    }

    #[test]
    fn rejects_a_self_loop() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let decision_id = new_decision(&mut store, &project_id, "k1");
        let err = {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &project_id,
                RelationType::RelatesTo,
                &decision_id,
                &decision_id,
                None,
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("self-loop"));
        cleanup(&root);
    }

    #[test]
    fn rejects_a_cross_project_endpoint() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let p1 = new_project(&mut store);
        let p2 = new_project(&mut store);
        let d1 = new_decision(&mut store, &p1, "k1");
        let d2 = new_decision(&mut store, &p2, "k1");
        let err = {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &p1,
                RelationType::RelatesTo,
                &d1,
                &d2,
                None,
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("different project"));
        cleanup(&root);
    }

    #[test]
    fn rejects_a_project_or_relation_as_an_endpoint() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let decision_id = new_decision(&mut store, &project_id, "k1");
        let err = {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &project_id,
                RelationType::RelatesTo,
                &decision_id,
                &project_id,
                None,
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("cannot be a relation endpoint"));
        cleanup(&root);
    }

    #[test]
    fn supersedes_requires_decision_endpoints() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let decision_id = new_decision(&mut store, &project_id, "k1");
        let source_id = new_source(&mut store, &project_id);
        let err = {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &project_id,
                RelationType::Supersedes,
                &decision_id,
                &source_id,
                None,
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("requires a decision"));
        cleanup(&root);
    }

    #[test]
    fn supersedes_rejects_a_direct_cycle() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let a = new_decision(&mut store, &project_id, "k1");
        let b = new_decision(&mut store, &project_id, "k1");

        {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &project_id,
                RelationType::Supersedes,
                &b,
                &a,
                None,
            )
            .unwrap(); // b supersedes a
        }
        let err = {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &project_id,
                RelationType::Supersedes,
                &a,
                &b,
                None,
            )
            .unwrap_err() // a supersedes b would close a 2-cycle
        };
        assert!(format!("{err}").contains("cycle"));
        cleanup(&root);
    }

    #[test]
    fn supersedes_rejects_a_transitive_cycle() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let a = new_decision(&mut store, &project_id, "k1");
        let b = new_decision(&mut store, &project_id, "k1");
        let c = new_decision(&mut store, &project_id, "k1");

        {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &project_id,
                RelationType::Supersedes,
                &b,
                &a,
                None,
            )
            .unwrap(); // b -> a
        }
        {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &project_id,
                RelationType::Supersedes,
                &c,
                &b,
                None,
            )
            .unwrap(); // c -> b
        }
        // a -> c would close a -> c -> b -> a
        let err = {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &project_id,
                RelationType::Supersedes,
                &a,
                &c,
                None,
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("cycle"));
        cleanup(&root);
    }

    #[test]
    fn missing_endpoint_is_refused_before_any_mutation() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let decision_id = new_decision(&mut store, &project_id, "k1");
        let missing = uuid::Uuid::now_v7().to_string();
        let head_before = store.transaction_head().unwrap();
        let err = {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &project_id,
                RelationType::RelatesTo,
                &decision_id,
                &missing,
                None,
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("no object exists"));
        assert_eq!(store.transaction_head().unwrap(), head_before);
        cleanup(&root);
    }

    #[test]
    fn endpoints_are_pinned_to_the_revision_current_at_creation() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let decision_id = new_decision(&mut store, &project_id, "k1");
        let source_id = new_source(&mut store, &project_id);
        let (from_rev_before, _) = store.read_current(&decision_id).unwrap().unwrap();

        let (_, relation) = {
            let mut writer = store.writer().unwrap();
            create_relation(
                &mut writer,
                "owner",
                &project_id,
                RelationType::Supports,
                &decision_id,
                &source_id,
                None,
            )
            .unwrap()
        };
        assert_eq!(relation.from_revision_id, from_rev_before);

        // Mutate the decision afterwards; the already-created relation keeps
        // citing the old revision it was pinned to.
        {
            let mut writer = store.writer().unwrap();
            project::accept_decision(&mut writer, "owner", &decision_id, &from_rev_before).unwrap();
        }
        let (from_rev_after, _) = store.read_current(&decision_id).unwrap().unwrap();
        assert_ne!(from_rev_before, from_rev_after);
        assert_eq!(relation.from_revision_id, from_rev_before);

        cleanup(&root);
    }
}
