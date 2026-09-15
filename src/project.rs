//! Typed `Project`/`Note`/`Action`/`Decision` records on top of the
//! format-2 command/transaction API (`T02-01`).
//!
//! **Architecture.** This module adds no new column and no new
//! `CommandTarget` variant to `crate::canonical` — every typed record is an
//! ordinary opaque-payload object (`CreateObject`/`UpdateObject`), whose
//! `payload` is this module's own JSON serialization of one of the four
//! structs below, tagged with a `kind` field this module reads back to
//! dispatch. This keeps `T01-02`–`T01-07`'s already-audited, 600-fault-
//! schedule-proven transaction core completely unchanged: every typed
//! mutation still goes through the identical atomic commit path, identical
//! idempotency/expected-revision-conflict rules, and identical recovery/
//! backup/migration machinery, with zero new surface for those to re-prove.
//!
//! **Scope boundary — read before extending.** `T02-01`'s own objective is
//! "create/open/archive a project with stable typed work records," not the
//! full product model. Deliberately **not** implemented here (recorded, not
//! silently dropped — the canonical plan assigns each to a later task):
//!
//! - `Note`'s source/evidence/artifact linkage (`T02-02`'s objective).
//! - `Action`'s full state-machine ("any reopening requires an event",
//!   ordered dependency-cycle rejection) and `Decision`'s evidence linkage,
//!   override/supersession semantics, and valid-time intervals (`T02-03`'s
//!   objective — "record decisions and complete actions with visible
//!   history"). This task's `Action`/`Decision` support minimal field
//!   admission and a single, ownership-checked field update, not the richer
//!   lifecycle transitions §12/§15 describe in full.
//! - Any indexed/searchable project-scoped query (`T02-04`'s objective).
//!   [`CanonicalStore::list_current_objects`] is a full scan, not an index.
//! - CLI polish beyond the plain subcommands this task's own acceptance
//!   criteria requires ("...work through CLI").
//!
//! **Unknown-field preservation (§15 "unknown fields... retained").** Every
//! struct below carries `#[serde(flatten)] pub unknown: serde_json::Map<...>`.
//! A payload written by a newer reader with fields this build does not know
//! about round-trips those fields unchanged through a read-then-write by
//! this build, exactly like `identity::Frontmatter::unknown` already does
//! for format-1.

use crate::canonical::{
    CanonicalStore, CanonicalWriter, CommandInput, CommandTarget, RecordOrigin,
};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Map as JsonMap;

/// §27 "Normal inputs": title ≤1 KiB.
pub const MAX_TITLE_BYTES: usize = 1024;
/// A reasonable, explicitly chosen bound for a project name — no such limit
/// is separately named in §27, so this reuses the same title bound.
pub const MAX_PROJECT_NAME_BYTES: usize = MAX_TITLE_BYTES;
/// §27 "Normal inputs": text body ≤1 MiB. Re-exported from `crate::limits`
/// rather than redefined, so the two never drift.
pub const MAX_BODY_BYTES: usize = crate::limits::MAX_OBJECT_BYTES;
/// §27 "Normal inputs": decision statement ≤8 KiB.
pub const MAX_DECISION_STATEMENT_BYTES: usize = 8192;

/// The current version of this module's own JSON payload shape — distinct
/// from `canonical::CANONICAL_SCHEMA_VERSION` (the SQL schema) and from
/// `RecordOrigin` (who declared the record). "Record schema capability
/// compatibility": a payload declaring a `payload_schema_version` this
/// build does not implement is refused on read, not guessed at.
pub const RECORD_PAYLOAD_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionState {
    Open,
    Doing,
    Blocked,
    Done,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionLifecycle {
    Draft,
    Accepted,
    Superseded,
    Withdrawn,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub payload_schema_version: u32,
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
    #[serde(flatten)]
    pub unknown: JsonMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub payload_schema_version: u32,
    pub project_id: String,
    pub title: Option<String>,
    pub body: String,
    pub tombstoned: bool,
    #[serde(flatten)]
    pub unknown: JsonMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Action {
    pub payload_schema_version: u32,
    pub project_id: String,
    pub title: String,
    pub body: Option<String>,
    pub state: ActionState,
    #[serde(flatten)]
    pub unknown: JsonMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Decision {
    pub payload_schema_version: u32,
    pub project_id: String,
    /// Stable, user-supplied decision key (§15). Not required to be unique
    /// across decisions in the same project — competing decisions may
    /// intentionally share a key (§12 "Conflict"); this task does not
    /// implement conflict computation over shared keys (`T02-03`'s job).
    pub decision_key: String,
    pub statement: String,
    pub rationale: Option<String>,
    pub lifecycle: DecisionLifecycle,
    #[serde(flatten)]
    pub unknown: JsonMap<String, serde_json::Value>,
}

/// One typed record, dispatched by its own `kind` field rather than serde's
/// enum tagging (avoids interaction edge cases between internally-tagged
/// enums and each variant's own `#[serde(flatten)]` unknown-fields map).
#[derive(Debug, Clone, PartialEq)]
pub enum RecordPayload {
    Project(Project),
    Note(Note),
    Action(Action),
    Decision(Decision),
    /// `T02-02`. Defined in `crate::capture`, not here — this enum only
    /// dispatches on `kind`; source/artifact admission's own validation
    /// rules live with the module that owns that security boundary.
    Source(crate::capture::Source),
}

impl RecordPayload {
    fn kind_str(&self) -> &'static str {
        match self {
            RecordPayload::Project(_) => "project",
            RecordPayload::Note(_) => "note",
            RecordPayload::Action(_) => "action",
            RecordPayload::Decision(_) => "decision",
            RecordPayload::Source(_) => "source",
        }
    }

    pub(crate) fn to_json(&self) -> Result<String> {
        let mut value = match self {
            RecordPayload::Project(p) => serde_json::to_value(p),
            RecordPayload::Note(n) => serde_json::to_value(n),
            RecordPayload::Action(a) => serde_json::to_value(a),
            RecordPayload::Decision(d) => serde_json::to_value(d),
            RecordPayload::Source(s) => serde_json::to_value(s),
        }
        .map_err(|e| Error::Project(format!("cannot serialize record: {e}")))?;
        // The `kind` tag is Core-assigned here, at the one serialization
        // choke point — never taken from caller-supplied JSON (S04/S06:
        // "type tags... cannot bypass admission").
        value
            .as_object_mut()
            .expect("record payloads always serialize to a JSON object")
            .insert(
                "kind".to_string(),
                serde_json::Value::String(self.kind_str().to_string()),
            );
        serde_json::to_string(&value)
            .map_err(|e| Error::Project(format!("cannot serialize record: {e}")))
    }

    /// Parse a stored payload, dispatching on its own `kind` field. Refuses
    /// a `payload_schema_version` this build does not implement, rather
    /// than guessing at forward compatibility.
    pub fn from_json(json: &str) -> Result<Self> {
        let mut value: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| Error::Project(format!("record payload is not valid JSON: {e}")))?;
        let object = value
            .as_object_mut()
            .ok_or_else(|| Error::Project("record payload is not a JSON object".into()))?;
        // `kind` is a dispatch tag, not a named field on any concrete
        // struct (it is written only by `to_json`'s one serialization
        // choke point) — it must be removed here before deserializing into
        // a concrete struct, or `#[serde(flatten)] unknown` would capture
        // it as if it were an unrecognized field from a newer schema.
        let kind = object
            .remove("kind")
            .and_then(|k| k.as_str().map(str::to_string))
            .ok_or_else(|| Error::Project("record payload has no kind field".into()))?;
        let schema_version = object
            .get("payload_schema_version")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| {
                Error::Project("record payload has no payload_schema_version field".into())
            })?;
        if schema_version > RECORD_PAYLOAD_SCHEMA_VERSION as u64 {
            return Err(Error::Project(format!(
                "record payload_schema_version {schema_version} is newer than this build supports ({RECORD_PAYLOAD_SCHEMA_VERSION}); refusing rather than guessing compatibility"
            )));
        }
        match kind.as_str() {
            "project" => Ok(RecordPayload::Project(
                serde_json::from_value(value)
                    .map_err(|e| Error::Project(format!("malformed project record: {e}")))?,
            )),
            "note" => Ok(RecordPayload::Note(serde_json::from_value(value).map_err(
                |e| Error::Project(format!("malformed note record: {e}")),
            )?)),
            "action" => Ok(RecordPayload::Action(
                serde_json::from_value(value)
                    .map_err(|e| Error::Project(format!("malformed action record: {e}")))?,
            )),
            "decision" => Ok(RecordPayload::Decision(
                serde_json::from_value(value)
                    .map_err(|e| Error::Project(format!("malformed decision record: {e}")))?,
            )),
            "source" => Ok(RecordPayload::Source(
                serde_json::from_value(value)
                    .map_err(|e| Error::Project(format!("malformed source record: {e}")))?,
            )),
            other => Err(Error::Project(format!("unrecognized record kind: {other}"))),
        }
    }

    pub fn as_project(&self) -> Result<&Project> {
        match self {
            RecordPayload::Project(p) => Ok(p),
            other => Err(Error::Project(format!(
                "expected a project record, found {}",
                other.kind_str()
            ))),
        }
    }

    pub fn as_source(&self) -> Result<&crate::capture::Source> {
        match self {
            RecordPayload::Source(s) => Ok(s),
            other => Err(Error::Project(format!(
                "expected a source record, found {}",
                other.kind_str()
            ))),
        }
    }
}

pub(crate) fn check_len(what: &'static str, s: &str, limit: usize) -> Result<()> {
    if s.len() > limit {
        return Err(Error::Project(format!(
            "{what} exceeds limit: {} > {limit} bytes",
            s.len()
        )));
    }
    Ok(())
}

/// Reads `project_id`, confirms it names a `Project` record (not a
/// nonexistent object and not some other kind — "invalid cross-project
/// references... reject"). Does not require the project to be active:
/// archiving hides a project from a default view, it does not retroactively
/// invalidate the identity every one of its work records still legitimately
/// carries (I05: archiving preserves history; §12 "project archive is
/// reversible").
pub(crate) fn require_project(store: &CanonicalStore, project_id: &str) -> Result<Project> {
    let (_, payload) = store.read_current(project_id)?.ok_or_else(|| {
        Error::Project(format!(
            "invalid project reference: no object exists with id {project_id}"
        ))
    })?;
    RecordPayload::from_json(&payload)?
        .as_project()
        .cloned()
        .map_err(|_| {
            Error::Project(format!(
                "invalid project reference: object {project_id} is not a project"
            ))
        })
}

pub(crate) fn commit_create(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    origin: RecordOrigin,
    record: RecordPayload,
) -> Result<(crate::canonical::CommandOutcome, RecordPayload)> {
    let payload = record.to_json()?;
    let outcome = writer.commit(CommandInput {
        command_id: uuid::Uuid::now_v7().to_string(),
        actor: actor.to_string(),
        origin,
        target: CommandTarget::CreateObject { payload },
    })?;
    Ok((outcome, record))
}

pub(crate) fn commit_update(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    origin: RecordOrigin,
    object_id: &str,
    expected_revision_id: &str,
    record: RecordPayload,
) -> Result<(crate::canonical::CommandOutcome, RecordPayload)> {
    let payload = record.to_json()?;
    let outcome = writer.commit(CommandInput {
        command_id: uuid::Uuid::now_v7().to_string(),
        actor: actor.to_string(),
        origin,
        target: CommandTarget::UpdateObject {
            object_id: object_id.to_string(),
            expected_revision_id: expected_revision_id.to_string(),
            payload,
        },
    })?;
    Ok((outcome, record))
}

/// Create a new project. F05/F07/F20: field limits are checked before any
/// transaction opens; invalid input leaves no trace.
pub fn create_project(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    name: &str,
    description: Option<&str>,
) -> Result<(crate::canonical::CommandOutcome, Project)> {
    check_len("project name", name, MAX_PROJECT_NAME_BYTES)?;
    if let Some(d) = description {
        check_len("project description", d, MAX_BODY_BYTES)?;
    }
    let record = RecordPayload::Project(Project {
        payload_schema_version: RECORD_PAYLOAD_SCHEMA_VERSION,
        name: name.to_string(),
        description: description.map(str::to_string),
        active: true,
        unknown: JsonMap::new(),
    });
    let (outcome, record) = commit_create(writer, actor, RecordOrigin::User, record)?;
    Ok((outcome, record.as_project()?.clone()))
}

fn set_project_active(
    store: &mut CanonicalStore,
    actor: &str,
    project_id: &str,
    active: bool,
) -> Result<(crate::canonical::CommandOutcome, Project)> {
    let (revision_id, payload) = store
        .read_current(project_id)?
        .ok_or_else(|| Error::Project(format!("no project exists with id {project_id}")))?;
    let mut project = RecordPayload::from_json(&payload)?.as_project()?.clone();
    project.active = active;
    let mut writer = store.writer()?;
    let (outcome, record) = commit_update(
        &mut writer,
        actor,
        RecordOrigin::User,
        project_id,
        &revision_id,
        RecordPayload::Project(project),
    )?;
    Ok((outcome, record.as_project()?.clone()))
}

/// Archive a project. A new revision records `active: false`; the prior
/// (active) revision remains in immutable history (I05).
pub fn archive_project(
    store: &mut CanonicalStore,
    actor: &str,
    project_id: &str,
) -> Result<(crate::canonical::CommandOutcome, Project)> {
    set_project_active(store, actor, project_id, false)
}

/// Reverse of [`archive_project`] — also a new revision, not a rewrite of
/// the archiving revision.
pub fn unarchive_project(
    store: &mut CanonicalStore,
    actor: &str,
    project_id: &str,
) -> Result<(crate::canonical::CommandOutcome, Project)> {
    set_project_active(store, actor, project_id, true)
}

/// Read a project's current state (no lock, no mutation).
pub fn open_project(store: &CanonicalStore, project_id: &str) -> Result<Project> {
    require_project(store, project_id)
}

/// Create a note in `project_id`. Rejects if `project_id` does not name an
/// existing project (I03/I05/I07: "one owning project per work record; no
/// orphaned canonical references").
pub fn create_note(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    project_id: &str,
    title: Option<&str>,
    body: &str,
) -> Result<(crate::canonical::CommandOutcome, Note)> {
    require_project(writer.store(), project_id)?;
    if let Some(t) = title {
        check_len("note title", t, MAX_TITLE_BYTES)?;
    }
    check_len("note body", body, MAX_BODY_BYTES)?;
    let record = RecordPayload::Note(Note {
        payload_schema_version: RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: project_id.to_string(),
        title: title.map(str::to_string),
        body: body.to_string(),
        tombstoned: false,
        unknown: JsonMap::new(),
    });
    let (outcome, record) = commit_create(writer, actor, RecordOrigin::User, record)?;
    Ok((
        outcome,
        match record {
            RecordPayload::Note(n) => n,
            _ => unreachable!(),
        },
    ))
}

/// Replace a note's title/body with a new revision. `expected_revision_id`
/// must be the note's actual current revision (I07: never a silent
/// last-writer-wins) — the same conflict rule every other typed record's
/// update shares, inherited directly from `CanonicalWriter::commit`.
pub fn update_note(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    note_id: &str,
    expected_revision_id: &str,
    title: Option<&str>,
    body: &str,
) -> Result<(crate::canonical::CommandOutcome, Note)> {
    let (_, existing_payload) = writer
        .store()
        .read_current(note_id)?
        .ok_or_else(|| Error::Project(format!("no note exists with id {note_id}")))?;
    let existing = match RecordPayload::from_json(&existing_payload)? {
        RecordPayload::Note(n) => n,
        other => {
            return Err(Error::Project(format!(
                "object {note_id} is not a note, found {}",
                other.kind_str()
            )))
        }
    };
    if let Some(t) = title {
        check_len("note title", t, MAX_TITLE_BYTES)?;
    }
    check_len("note body", body, MAX_BODY_BYTES)?;
    let record = RecordPayload::Note(Note {
        payload_schema_version: RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: existing.project_id.clone(),
        title: title.map(str::to_string),
        body: body.to_string(),
        tombstoned: existing.tombstoned,
        unknown: existing.unknown.clone(),
    });
    let (outcome, record) = commit_update(
        writer,
        actor,
        RecordOrigin::User,
        note_id,
        expected_revision_id,
        record,
    )?;
    Ok((
        outcome,
        match record {
            RecordPayload::Note(n) => n,
            _ => unreachable!(),
        },
    ))
}

/// Create an action in `project_id`, always starting in `ActionState::Open`
/// (§15 "Open→Doing/Blocked/Done/Cancelled"). Reaching a later state is
/// `T02-03`'s full state-machine objective, not this task's.
pub fn create_action(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    project_id: &str,
    title: &str,
    body: Option<&str>,
) -> Result<(crate::canonical::CommandOutcome, Action)> {
    require_project(writer.store(), project_id)?;
    check_len("action title", title, MAX_TITLE_BYTES)?;
    if let Some(b) = body {
        check_len("action body", b, MAX_BODY_BYTES)?;
    }
    let record = RecordPayload::Action(Action {
        payload_schema_version: RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: project_id.to_string(),
        title: title.to_string(),
        body: body.map(str::to_string),
        state: ActionState::Open,
        unknown: JsonMap::new(),
    });
    let (outcome, record) = commit_create(writer, actor, RecordOrigin::User, record)?;
    Ok((
        outcome,
        match record {
            RecordPayload::Action(a) => a,
            _ => unreachable!(),
        },
    ))
}

/// Create a decision in `project_id`, always starting in
/// `DecisionLifecycle::Draft`. Evidence linkage, owner acceptance, and
/// override/supersession semantics are `T02-03`'s objective.
pub fn create_decision(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    project_id: &str,
    decision_key: &str,
    statement: &str,
    rationale: Option<&str>,
) -> Result<(crate::canonical::CommandOutcome, Decision)> {
    require_project(writer.store(), project_id)?;
    if decision_key.is_empty() || decision_key.len() > 128 {
        return Err(Error::Project(format!(
            "decision_key must be 1-128 bytes, got {}",
            decision_key.len()
        )));
    }
    check_len(
        "decision statement",
        statement,
        MAX_DECISION_STATEMENT_BYTES,
    )?;
    if let Some(r) = rationale {
        check_len("decision rationale", r, MAX_BODY_BYTES)?;
    }
    let record = RecordPayload::Decision(Decision {
        payload_schema_version: RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: project_id.to_string(),
        decision_key: decision_key.to_string(),
        statement: statement.to_string(),
        rationale: rationale.map(str::to_string),
        lifecycle: DecisionLifecycle::Draft,
        unknown: JsonMap::new(),
    });
    let (outcome, record) = commit_create(writer, actor, RecordOrigin::User, record)?;
    Ok((
        outcome,
        match record {
            RecordPayload::Decision(d) => d,
            _ => unreachable!(),
        },
    ))
}

/// Read any typed record by `object_id`, whatever its kind.
pub fn read_record(store: &CanonicalStore, object_id: &str) -> Result<Option<RecordPayload>> {
    match store.read_current(object_id)? {
        Some((_, payload)) => Ok(Some(RecordPayload::from_json(&payload)?)),
        None => Ok(None),
    }
}

/// Every work record (`Note`/`Action`/`Decision`, never `Project` itself)
/// currently belonging to `project_id`. A full scan — see
/// [`CanonicalStore::list_current_objects`]'s own documented limitation.
pub fn list_project_records(
    store: &CanonicalStore,
    project_id: &str,
) -> Result<Vec<(String, RecordPayload)>> {
    let mut out = Vec::new();
    for (object_id, _revision_id, payload) in store.list_current_objects()? {
        let record = match RecordPayload::from_json(&payload) {
            Ok(r) => r,
            Err(_) => continue, // not a typed record (e.g. a migrated legacy object); not in scope
        };
        let belongs = match &record {
            RecordPayload::Note(n) => n.project_id == project_id,
            RecordPayload::Action(a) => a.project_id == project_id,
            RecordPayload::Decision(d) => d.project_id == project_id,
            // A `Source` also carries `project_id`, but is not a "work
            // record" in this function's own documented sense (`T02-02`
            // gives it a dedicated `capture::list_project_sources`).
            RecordPayload::Source(_) | RecordPayload::Project(_) => false,
        };
        if belongs {
            out.push((object_id, record));
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
        std::env::temp_dir().join(format!("fehrest-project-{}", uuid::Uuid::now_v7()))
    }

    fn cleanup(p: &Path) {
        let _ = std::fs::remove_dir_all(p);
    }

    #[test]
    fn create_open_archive_unarchive_project_round_trips() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let (outcome, project) = {
            let mut writer = store.writer().unwrap();
            create_project(&mut writer, "owner", "My Project", Some("a description")).unwrap()
        };
        assert!(project.active);
        assert_eq!(project.name, "My Project");

        let opened = open_project(&store, &outcome.object_id).unwrap();
        assert_eq!(opened, project);

        let (_, archived) = archive_project(&mut store, "owner", &outcome.object_id).unwrap();
        assert!(!archived.active);
        let (_, unarchived) = unarchive_project(&mut store, "owner", &outcome.object_id).unwrap();
        assert!(unarchived.active);

        // History preserved: two prior revisions (create, archive) plus
        // the current (unarchive) one all still exist.
        let history = store.history(&outcome.object_id).unwrap();
        assert_eq!(history.len(), 3);

        cleanup(&root);
    }

    #[test]
    fn note_requires_an_existing_project_and_rejects_wrong_kind_reference() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();

        let missing_project = uuid::Uuid::now_v7().to_string();
        let err = {
            let mut writer = store.writer().unwrap();
            create_note(&mut writer, "owner", &missing_project, None, "body").unwrap_err()
        };
        assert!(format!("{err}").contains("invalid project reference"));

        let (project_outcome, _) = {
            let mut writer = store.writer().unwrap();
            create_project(&mut writer, "owner", "P", None).unwrap()
        };
        let (note_outcome, note) = {
            let mut writer = store.writer().unwrap();
            create_note(
                &mut writer,
                "owner",
                &project_outcome.object_id,
                Some("Title"),
                "Body text",
            )
            .unwrap()
        };
        assert_eq!(note.project_id, project_outcome.object_id);

        // A note's own ID is not a project — referencing it as one refuses.
        let err = {
            let mut writer = store.writer().unwrap();
            create_note(&mut writer, "owner", &note_outcome.object_id, None, "x").unwrap_err()
        };
        assert!(format!("{err}").contains("is not a project"));

        cleanup(&root);
    }

    #[test]
    fn note_update_conflicts_exactly_like_any_other_typed_record() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let (project_outcome, _) = {
            let mut writer = store.writer().unwrap();
            create_project(&mut writer, "owner", "P", None).unwrap()
        };
        let (note_outcome, _) = {
            let mut writer = store.writer().unwrap();
            create_note(&mut writer, "owner", &project_outcome.object_id, None, "v1").unwrap()
        };
        let (updated_outcome, updated) = {
            let mut writer = store.writer().unwrap();
            update_note(
                &mut writer,
                "owner",
                &note_outcome.object_id,
                &note_outcome.revision_id,
                None,
                "v2",
            )
            .unwrap()
        };
        assert_eq!(updated.body, "v2");

        // Stale expected_revision_id conflicts, exactly like the untyped path.
        let err = {
            let mut writer = store.writer().unwrap();
            update_note(
                &mut writer,
                "owner",
                &note_outcome.object_id,
                &note_outcome.revision_id, // stale: superseded by updated_outcome
                None,
                "v3-conflicting",
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("expected revision conflict"));
        let (_, current) = store
            .read_current(&note_outcome.object_id)
            .unwrap()
            .unwrap();
        assert!(current.contains("v2"));
        let _ = updated_outcome;

        cleanup(&root);
    }

    #[test]
    fn field_limits_are_rejected_before_any_transaction_opens() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let huge = "x".repeat(MAX_PROJECT_NAME_BYTES + 1);
        let head_before = store.transaction_head().unwrap();
        let err = {
            let mut writer = store.writer().unwrap();
            create_project(&mut writer, "owner", &huge, None).unwrap_err()
        };
        assert!(format!("{err}").contains("exceeds limit"));
        assert_eq!(store.transaction_head().unwrap(), head_before);

        let (project_outcome, _) = {
            let mut writer = store.writer().unwrap();
            create_project(&mut writer, "owner", "P", None).unwrap()
        };
        let head_before = store.transaction_head().unwrap();
        let huge_statement = "x".repeat(MAX_DECISION_STATEMENT_BYTES + 1);
        let err = {
            let mut writer = store.writer().unwrap();
            create_decision(
                &mut writer,
                "owner",
                &project_outcome.object_id,
                "key",
                &huge_statement,
                None,
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("exceeds limit"));
        assert_eq!(store.transaction_head().unwrap(), head_before);

        cleanup(&root);
    }

    #[test]
    fn action_starts_open_and_decision_starts_draft() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let (project_outcome, _) = {
            let mut writer = store.writer().unwrap();
            create_project(&mut writer, "owner", "P", None).unwrap()
        };
        let (_, action) = {
            let mut writer = store.writer().unwrap();
            create_action(
                &mut writer,
                "owner",
                &project_outcome.object_id,
                "Do the thing",
                None,
            )
            .unwrap()
        };
        assert_eq!(action.state, ActionState::Open);

        let (_, decision) = {
            let mut writer = store.writer().unwrap();
            create_decision(
                &mut writer,
                "owner",
                &project_outcome.object_id,
                "question-1",
                "We will do X",
                Some("because Y"),
            )
            .unwrap()
        };
        assert_eq!(decision.lifecycle, DecisionLifecycle::Draft);

        cleanup(&root);
    }

    #[test]
    fn list_project_records_scopes_correctly_and_excludes_other_projects() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let (p1, _) = {
            let mut writer = store.writer().unwrap();
            create_project(&mut writer, "owner", "P1", None).unwrap()
        };
        let (p2, _) = {
            let mut writer = store.writer().unwrap();
            create_project(&mut writer, "owner", "P2", None).unwrap()
        };
        {
            let mut writer = store.writer().unwrap();
            create_note(&mut writer, "owner", &p1.object_id, None, "n1").unwrap();
        }
        {
            let mut writer = store.writer().unwrap();
            create_action(&mut writer, "owner", &p1.object_id, "a1", None).unwrap();
        }
        {
            let mut writer = store.writer().unwrap();
            create_note(&mut writer, "owner", &p2.object_id, None, "n2").unwrap();
        }

        let p1_records = list_project_records(&store, &p1.object_id).unwrap();
        assert_eq!(p1_records.len(), 2);
        let p2_records = list_project_records(&store, &p2.object_id).unwrap();
        assert_eq!(p2_records.len(), 1);

        cleanup(&root);
    }

    #[test]
    fn unknown_fields_survive_a_read_then_write_round_trip() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let (project_outcome, _) = {
            let mut writer = store.writer().unwrap();
            create_project(&mut writer, "owner", "P", None).unwrap()
        };

        // Simulate a payload written by a newer build carrying an extra
        // field this build does not know about.
        {
            let mut project = RecordPayload::Project(Project {
                payload_schema_version: RECORD_PAYLOAD_SCHEMA_VERSION,
                name: "P".to_string(),
                description: None,
                active: true,
                unknown: JsonMap::new(),
            });
            if let RecordPayload::Project(p) = &mut project {
                p.unknown
                    .insert("future_field".to_string(), serde_json::json!("kept"));
            }
            let payload = project.to_json().unwrap();
            let mut writer = store.writer().unwrap();
            writer
                .commit_with_fault(
                    CommandInput {
                        command_id: uuid::Uuid::now_v7().to_string(),
                        actor: "owner".into(),
                        origin: RecordOrigin::User,
                        target: CommandTarget::UpdateObject {
                            object_id: project_outcome.object_id.clone(),
                            expected_revision_id: project_outcome.revision_id.clone(),
                            payload,
                        },
                    },
                    None,
                )
                .unwrap();
        }

        let (revision_id, payload) = store
            .read_current(&project_outcome.object_id)
            .unwrap()
            .unwrap();
        let record = RecordPayload::from_json(&payload).unwrap();
        let project = record.as_project().unwrap();
        assert_eq!(
            project.unknown.get("future_field"),
            Some(&serde_json::json!("kept"))
        );

        // Read-then-write (archive) must preserve the unknown field.
        let (_, archived) =
            archive_project(&mut store, "owner", &project_outcome.object_id).unwrap();
        assert_eq!(
            archived.unknown.get("future_field"),
            Some(&serde_json::json!("kept")),
            "unknown fields must survive a read-then-write round trip"
        );
        let _ = revision_id;

        cleanup(&root);
    }

    #[test]
    fn a_payload_schema_version_newer_than_supported_is_refused() {
        let json = serde_json::json!({
            "kind": "project",
            "payload_schema_version": RECORD_PAYLOAD_SCHEMA_VERSION + 1,
            "name": "future",
            "description": null,
            "active": true,
        })
        .to_string();
        let err = RecordPayload::from_json(&json).unwrap_err();
        assert!(format!("{err}").contains("newer than this build supports"));
    }
}
