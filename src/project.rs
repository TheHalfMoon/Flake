//! Typed `Project`/`Note`/`Action`/`Decision`/`Relation` records on top of
//! the format-2 command/transaction API (`T02-01`..`T02-03`).
//!
//! **Architecture.** This module adds no new column and no new
//! `CommandTarget` variant to `crate::canonical` — every typed record is an
//! ordinary opaque-payload object (`CreateObject`/`UpdateObject`), whose
//! `payload` is this module's own JSON serialization of one of the structs
//! below, tagged with a `kind` field this module reads back to dispatch.
//! This keeps `T01-02`–`T01-07`'s already-audited, 600-fault-schedule-proven
//! transaction core completely unchanged: every typed mutation still goes
//! through the identical atomic commit path, identical idempotency/
//! expected-revision-conflict rules, and identical recovery/backup/
//! migration machinery, with zero new surface for those to re-prove.
//!
//! **`T02-03` — what changed here.** `T02-01` deliberately left `Action`'s
//! full state machine and `Decision`'s evidence linkage/acceptance/
//! supersession/valid-time semantics unimplemented (see its own doc comment
//! below, preserved for provenance). This task implements all of it:
//!
//! - `Action` gains `dependency_ids` (ordered, cycle-checked), a completion
//!   summary/actor/timestamp, and `last_transition` (the "event" every
//!   transition, including reopening, must produce — see
//!   [`ActionTransition`]).
//! - `Decision` gains `basis`/`verification`, a half-open `[valid_from,
//!   valid_to)` interval, and owner-only-acceptance bookkeeping
//!   (`accepted_by`/`accepted_at`).
//! - A new sixth `RecordPayload` kind, [`crate::relation::Relation`]
//!   (defined in its own module, exactly like `T02-02`'s `Source` — see
//!   `relation.rs` module docs), is how a `Decision` cites a `Source` as
//!   evidence and how one `Decision` supersedes another. Nothing on
//!   `Decision`/`Action` itself duplicates that link (§15: "Evidence is a
//!   source revision/artifact plus a relation, not a second copy").
//!
//! **"Visible history" — how this task satisfies it without a new event
//! store.** Every transition is an ordinary `commit_update`, so it already
//! produces a new immutable revision through the exact same mechanism
//! `project::archive_project`/`capture::deactivate_source` already proved
//! ("history preserved" — see each one's own test). What `T02-03` adds is
//! making each revision *self-describing*: `Action::last_transition` records
//! exactly which transition produced *this* revision (from/to/reason/actor/
//! at), so reading `CanonicalStore::history(action_id)` and parsing each
//! revision's payload directly reconstructs the full transition history —
//! current state, every previous state, and the reason for each reopen or
//! override — without inventing a second, parallel canonical mechanism.
//!
//! **Scope boundary — read before extending.** Deliberately **not**
//! implemented here (recorded, not silently dropped):
//!
//! - Any indexed/searchable project-scoped query (`T02-04`'s objective).
//!   [`CanonicalStore::list_current_objects`] is a full scan, not an index.
//! - `Decision` tombstone/delete (§15 lists `tombstoned` among possible
//!   lifecycle values; `T02-03`'s own task contract names only evidence
//!   linkage, owner-only acceptance, override/supersession and valid-time).
//!   `T04-03` ("archive/tombstone confirmation" scope) covers `Decision`
//!   soft-removal via the already-existing `withdraw_decision` lifecycle
//!   transition (§15's `Withdrawn` is exactly this record kind's own
//!   tombstone-equivalent state — no separate boolean flag was added,
//!   since `DecisionLifecycle` already has one) and adds `Note::tombstoned`'s
//!   own setter (`tombstone_note`/`untombstone_note`, mirroring
//!   `archive_project`/`unarchive_project`'s shape), since `Note` had the
//!   field from `T02-01` on but no way to set it until now.
//! - A floating (always-resolve-to-current) `Relation` endpoint policy —
//!   see `relation.rs` module docs, "Endpoint revision policy".
//! - CLI polish beyond the plain subcommands this task's own acceptance
//!   criteria requires.
//!
//! **Unknown-field preservation (§15 "unknown fields... retained").** Every
//! struct below carries `#[serde(flatten)] pub unknown: serde_json::Map<...>`.
//! A payload written by a newer reader with fields this build does not know
//! about round-trips those fields unchanged through a read-then-write by
//! this build, exactly like `identity::Frontmatter::unknown` already does
//! for format-1. Fields this task adds to already-existing structs
//! (`Action`, `Decision`) carry `#[serde(default)]` so they degrade safely
//! reading a payload written before this task existed.

use crate::canonical::{
    CanonicalStore, CanonicalWriter, CommandInput, CommandOutcome, CommandTarget, RecordOrigin,
};
use crate::relation::Relation;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Map as JsonMap;
use std::collections::{HashMap, HashSet};

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
/// §27 "Normal inputs": event metadata ≤16 KiB. Reused here (not redefined)
/// for `Action`/`Decision` transition reasons and override justifications —
/// they are exactly "event metadata", the same category `crate::limits`
/// already names.
pub const MAX_TRANSITION_REASON_BYTES: usize = crate::limits::MAX_EVENT_BYTES;

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

/// The "event" `T02-03`'s own task row requires every reopening (and every
/// other transition) to produce — see module docs, "visible history".
/// Recorded as part of the revision it belongs to, not a parallel log.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionTransition {
    pub from: ActionState,
    pub to: ActionState,
    /// Mandatory for reopening (`Done`/`Cancelled` -> `Doing`) and for
    /// completing an action with unmet dependencies (the "reasoned
    /// override" `T02-03`'s implementation requirements name); optional
    /// metadata for every other transition.
    pub reason: Option<String>,
    pub actor: String,
    pub at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionLifecycle {
    Draft,
    Accepted,
    Superseded,
    Withdrawn,
}

/// §15 "Basis = evidence/user-judgment/agent-proposal".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionBasis {
    Evidence,
    UserJudgment,
    AgentProposal,
}

/// §15 "verification = unreviewed/user-reviewed".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionVerification {
    Unreviewed,
    UserReviewed,
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
    /// Ordered, explicit dependency object IDs (other `Action`s in the same
    /// project). Cycle-checked at every point this list can change — see
    /// `validate_action_dependency_graph`. Distinct from a generic
    /// `Relation{type: DependsOn}` (`relation.rs` module docs): this field
    /// is the one structural dependency graph a completion actually
    /// consults; a generic `depends-on` `Relation` is a descriptive link
    /// with no such consequence.
    #[serde(default)]
    pub dependency_ids: Vec<String>,
    #[serde(default)]
    pub completion_summary: Option<String>,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub completed_by: Option<String>,
    /// The transition that produced this exact revision. `None` only for
    /// the original `create_action` revision (which has no "from" state).
    #[serde(default)]
    pub last_transition: Option<ActionTransition>,
    #[serde(flatten)]
    pub unknown: JsonMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Decision {
    pub payload_schema_version: u32,
    pub project_id: String,
    /// Stable, user-supplied decision key (§15). Not required to be unique
    /// across decisions in the same project — competing decisions may
    /// intentionally share a key (§12 "Conflict"). `T02-03`'s
    /// `supersede_decision` requires the superseding decision to share the
    /// superseded one's key exactly, matching "Override: ... accepts an
    /// explicit superseding revision/decision" for the same question.
    pub decision_key: String,
    pub statement: String,
    pub rationale: Option<String>,
    pub basis: DecisionBasis,
    pub verification: DecisionVerification,
    pub lifecycle: DecisionLifecycle,
    /// Half-open `[valid_from, valid_to)` UTC RFC3339 (§15). `None` means no
    /// explicit valid-time claim has been recorded — never coerced into an
    /// invented unbounded assertion (I11).
    #[serde(default)]
    pub valid_from: Option<String>,
    #[serde(default)]
    pub valid_to: Option<String>,
    #[serde(default)]
    pub accepted_by: Option<String>,
    #[serde(default)]
    pub accepted_at: Option<String>,
    /// Set only by `withdraw_decision`. Not reused for supersession — a
    /// superseding decision's own reason lives on the `Relation` that
    /// records the supersession edge (§15: evidence is a relation, not a
    /// second copy; the same rule applies to override rationale).
    #[serde(default)]
    pub withdrawal_reason: Option<String>,
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
    /// `T02-03`. Defined in `crate::relation`, not here, for the same
    /// reason `Source` lives in `crate::capture`.
    Relation(Relation),
    /// `T03-01`. Defined in `crate::source_check`, not here, for the same
    /// reason `Source`/`Relation` live in their own modules.
    SourceCheck(crate::source_check::SourceCheck),
    /// `T03-03`. Defined in `crate::checkpoint`, not here, for the same
    /// reason `Source`/`Relation`/`SourceCheck` live in their own modules.
    ReviewCheckpoint(crate::checkpoint::ReviewCheckpoint),
    /// `T03-04`. Defined in `crate::grant`, not here, for the same reason
    /// every other kind lives in its own module.
    ExportGrant(crate::grant::ExportGrant),
    /// `T03-04`. Defined in `crate::disclosure`, not here. Unlike every
    /// other kind, this one is never updated after creation — a receipt
    /// is immutable by construction (§15 "Disclosure receipt"), not merely
    /// by convention: no function anywhere in this crate ever issues a
    /// `CommandTarget::UpdateObject` against one.
    DisclosureReceipt(crate::disclosure::DisclosureReceipt),
    /// `T03-05`. Defined in `crate::proposal`, not here, for the same
    /// reason every other kind lives in its own module.
    AgentProposal(crate::proposal::AgentProposal),
}

impl RecordPayload {
    pub(crate) fn kind_str(&self) -> &'static str {
        match self {
            RecordPayload::Project(_) => "project",
            RecordPayload::Note(_) => "note",
            RecordPayload::Action(_) => "action",
            RecordPayload::Decision(_) => "decision",
            RecordPayload::Source(_) => "source",
            RecordPayload::Relation(_) => "relation",
            RecordPayload::SourceCheck(_) => "source_check",
            RecordPayload::ReviewCheckpoint(_) => "review_checkpoint",
            RecordPayload::ExportGrant(_) => "export_grant",
            RecordPayload::DisclosureReceipt(_) => "disclosure_receipt",
            RecordPayload::AgentProposal(_) => "agent_proposal",
        }
    }

    /// The owning project of any project-scoped record, or `None` for
    /// `Project` itself (which has no owner). Used by `relation.rs` to
    /// enforce "relations stay within one project" without duplicating a
    /// per-kind match at every call site.
    pub(crate) fn project_id_of(&self) -> Option<&str> {
        match self {
            RecordPayload::Project(_) => None,
            RecordPayload::Note(n) => Some(&n.project_id),
            RecordPayload::Action(a) => Some(&a.project_id),
            RecordPayload::Decision(d) => Some(&d.project_id),
            RecordPayload::Source(s) => Some(&s.project_id),
            RecordPayload::Relation(r) => Some(&r.project_id),
            RecordPayload::SourceCheck(c) => Some(&c.project_id),
            RecordPayload::ReviewCheckpoint(c) => Some(&c.project_id),
            RecordPayload::ExportGrant(g) => Some(&g.project_id),
            RecordPayload::DisclosureReceipt(r) => Some(&r.project_id),
            RecordPayload::AgentProposal(p) => Some(&p.project_id),
        }
    }

    pub(crate) fn to_json(&self) -> Result<String> {
        let mut value = match self {
            RecordPayload::Project(p) => serde_json::to_value(p),
            RecordPayload::Note(n) => serde_json::to_value(n),
            RecordPayload::Action(a) => serde_json::to_value(a),
            RecordPayload::Decision(d) => serde_json::to_value(d),
            RecordPayload::Source(s) => serde_json::to_value(s),
            RecordPayload::Relation(r) => serde_json::to_value(r),
            RecordPayload::SourceCheck(c) => serde_json::to_value(c),
            RecordPayload::ReviewCheckpoint(c) => serde_json::to_value(c),
            RecordPayload::ExportGrant(g) => serde_json::to_value(g),
            RecordPayload::DisclosureReceipt(r) => serde_json::to_value(r),
            RecordPayload::AgentProposal(p) => serde_json::to_value(p),
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
            "relation" => Ok(RecordPayload::Relation(
                serde_json::from_value(value)
                    .map_err(|e| Error::Project(format!("malformed relation record: {e}")))?,
            )),
            "source_check" => Ok(RecordPayload::SourceCheck(
                serde_json::from_value(value)
                    .map_err(|e| Error::Project(format!("malformed source_check record: {e}")))?,
            )),
            "review_checkpoint" => Ok(RecordPayload::ReviewCheckpoint(
                serde_json::from_value(value).map_err(|e| {
                    Error::Project(format!("malformed review_checkpoint record: {e}"))
                })?,
            )),
            "export_grant" => Ok(RecordPayload::ExportGrant(
                serde_json::from_value(value)
                    .map_err(|e| Error::Project(format!("malformed export_grant record: {e}")))?,
            )),
            "disclosure_receipt" => Ok(RecordPayload::DisclosureReceipt(
                serde_json::from_value(value).map_err(|e| {
                    Error::Project(format!("malformed disclosure_receipt record: {e}"))
                })?,
            )),
            "agent_proposal" => Ok(RecordPayload::AgentProposal(
                serde_json::from_value(value)
                    .map_err(|e| Error::Project(format!("malformed agent_proposal record: {e}")))?,
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

    pub fn as_note(&self) -> Result<&Note> {
        match self {
            RecordPayload::Note(n) => Ok(n),
            other => Err(Error::Project(format!(
                "expected a note record, found {}",
                other.kind_str()
            ))),
        }
    }

    pub fn as_action(&self) -> Result<&Action> {
        match self {
            RecordPayload::Action(a) => Ok(a),
            other => Err(Error::Project(format!(
                "expected an action record, found {}",
                other.kind_str()
            ))),
        }
    }

    pub fn as_decision(&self) -> Result<&Decision> {
        match self {
            RecordPayload::Decision(d) => Ok(d),
            other => Err(Error::Project(format!(
                "expected a decision record, found {}",
                other.kind_str()
            ))),
        }
    }

    pub fn as_relation(&self) -> Result<&Relation> {
        match self {
            RecordPayload::Relation(r) => Ok(r),
            other => Err(Error::Project(format!(
                "expected a relation record, found {}",
                other.kind_str()
            ))),
        }
    }

    pub fn as_source_check(&self) -> Result<&crate::source_check::SourceCheck> {
        match self {
            RecordPayload::SourceCheck(c) => Ok(c),
            other => Err(Error::Project(format!(
                "expected a source_check record, found {}",
                other.kind_str()
            ))),
        }
    }

    pub fn as_review_checkpoint(&self) -> Result<&crate::checkpoint::ReviewCheckpoint> {
        match self {
            RecordPayload::ReviewCheckpoint(c) => Ok(c),
            other => Err(Error::Project(format!(
                "expected a review_checkpoint record, found {}",
                other.kind_str()
            ))),
        }
    }

    pub fn as_export_grant(&self) -> Result<&crate::grant::ExportGrant> {
        match self {
            RecordPayload::ExportGrant(g) => Ok(g),
            other => Err(Error::Project(format!(
                "expected an export_grant record, found {}",
                other.kind_str()
            ))),
        }
    }

    pub fn as_disclosure_receipt(&self) -> Result<&crate::disclosure::DisclosureReceipt> {
        match self {
            RecordPayload::DisclosureReceipt(r) => Ok(r),
            other => Err(Error::Project(format!(
                "expected a disclosure_receipt record, found {}",
                other.kind_str()
            ))),
        }
    }

    pub fn as_agent_proposal(&self) -> Result<&crate::proposal::AgentProposal> {
        match self {
            RecordPayload::AgentProposal(p) => Ok(p),
            other => Err(Error::Project(format!(
                "expected an agent_proposal record, found {}",
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
) -> Result<(CommandOutcome, RecordPayload)> {
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
) -> Result<(CommandOutcome, RecordPayload)> {
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
) -> Result<(CommandOutcome, Project)> {
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
) -> Result<(CommandOutcome, Project)> {
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
) -> Result<(CommandOutcome, Project)> {
    set_project_active(store, actor, project_id, false)
}

/// Reverse of [`archive_project`] — also a new revision, not a rewrite of
/// the archiving revision.
pub fn unarchive_project(
    store: &mut CanonicalStore,
    actor: &str,
    project_id: &str,
) -> Result<(CommandOutcome, Project)> {
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
) -> Result<(CommandOutcome, Note)> {
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
) -> Result<(CommandOutcome, Note)> {
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

/// Tombstone a note (`T04`'s "archive/tombstone confirmation" scope,
/// deferred here from `T02-03` — see this module's own "Scope boundary"
/// doc comment above). Mirrors `archive_project`'s shape: a new revision
/// records `tombstoned: true`; the prior revision remains in immutable
/// history (I05) — this is a lifecycle flag flip, never a destructive
/// rewrite or content deletion. `expected_revision_id` follows `update_note`'s
/// own convention (never a silent last-writer-wins).
pub fn tombstone_note(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    note_id: &str,
    expected_revision_id: &str,
) -> Result<(CommandOutcome, Note)> {
    set_note_tombstoned(writer, actor, note_id, expected_revision_id, true)
}

/// Reverse of [`tombstone_note`] — also a new revision, not a rewrite of
/// the tombstoning revision.
pub fn untombstone_note(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    note_id: &str,
    expected_revision_id: &str,
) -> Result<(CommandOutcome, Note)> {
    set_note_tombstoned(writer, actor, note_id, expected_revision_id, false)
}

fn set_note_tombstoned(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    note_id: &str,
    expected_revision_id: &str,
    tombstoned: bool,
) -> Result<(CommandOutcome, Note)> {
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
    let record = RecordPayload::Note(Note {
        payload_schema_version: RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: existing.project_id.clone(),
        title: existing.title.clone(),
        body: existing.body.clone(),
        tombstoned,
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

// ---------------------------------------------------------------------
// Action: creation, dependency graph, and the full state machine (T02-03).
// ---------------------------------------------------------------------

/// Each dependency must name an existing `Action` in the same project. Used
/// at creation time, when a brand-new object ID cannot yet participate in
/// any existing cycle (nothing can reference it before it exists) — see
/// `validate_action_dependency_graph` for the full cycle-checked version
/// used once an action already exists and can be re-pointed.
fn validate_action_dependencies_exist(
    store: &CanonicalStore,
    project_id: &str,
    dependency_ids: &[String],
) -> Result<()> {
    for dep in dependency_ids {
        let (_, payload) = store.read_current(dep)?.ok_or_else(|| {
            Error::Project(format!(
                "invalid action dependency: no object exists with id {dep}"
            ))
        })?;
        match RecordPayload::from_json(&payload)?.as_action() {
            Ok(a) if a.project_id == project_id => {}
            Ok(_) => {
                return Err(Error::Project(format!(
                    "action dependency {dep} belongs to a different project"
                )))
            }
            Err(_) => {
                return Err(Error::Project(format!(
                    "action dependency {dep} is not an action"
                )))
            }
        }
    }
    Ok(())
}

/// Would setting `action_id`'s dependencies to `proposed_deps` create a
/// self-reference or a cycle among the project's `Action` dependency graph?
/// Builds the graph from every other action's *currently stored*
/// `dependency_ids`, substitutes `action_id`'s own edges with
/// `proposed_deps` (the edit under consideration, not yet committed), then
/// walks from `action_id` to see whether it can reach itself — "ordered
/// dependency-cycle rejection" (§15), checked before any transaction opens.
fn validate_action_dependency_graph(
    store: &CanonicalStore,
    project_id: &str,
    action_id: &str,
    proposed_deps: &[String],
) -> Result<()> {
    for dep in proposed_deps {
        if dep == action_id {
            return Err(Error::Project(format!(
                "action {action_id} cannot depend on itself"
            )));
        }
    }
    validate_action_dependencies_exist(store, project_id, proposed_deps)?;

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for (id, record) in list_project_records(store, project_id)? {
        if let RecordPayload::Action(a) = record {
            graph.insert(id, a.dependency_ids);
        }
    }
    graph.insert(action_id.to_string(), proposed_deps.to_vec());

    if reaches_self(&graph, action_id) {
        return Err(Error::Project(format!(
            "dependency cycle rejected: setting {action_id}'s dependencies to {proposed_deps:?} would create a cycle"
        )));
    }
    Ok(())
}

fn reaches_self(graph: &HashMap<String, Vec<String>>, start: &str) -> bool {
    let mut seen = HashSet::new();
    let mut stack: Vec<String> = graph.get(start).cloned().unwrap_or_default();
    while let Some(cur) = stack.pop() {
        if cur == start {
            return true;
        }
        if !seen.insert(cur.clone()) {
            continue;
        }
        if let Some(next) = graph.get(&cur) {
            stack.extend(next.iter().cloned());
        }
    }
    false
}

/// Which of `dependency_ids` are not currently `Done`. Missing/non-action
/// dependency IDs are treated as unmet too (fail closed) rather than
/// silently ignored, since a completion gate that quietly skips an
/// unreadable dependency would defeat its own purpose.
fn unmet_dependencies(store: &CanonicalStore, dependency_ids: &[String]) -> Result<Vec<String>> {
    let mut unmet = Vec::new();
    for dep in dependency_ids {
        let is_done = match store.read_current(dep)? {
            Some((_, payload)) => match RecordPayload::from_json(&payload) {
                Ok(RecordPayload::Action(a)) => a.state == ActionState::Done,
                _ => false,
            },
            None => false,
        };
        if !is_done {
            unmet.push(dep.clone());
        }
    }
    Ok(unmet)
}

/// Create an action in `project_id`, always starting in `ActionState::Open`
/// (§15 "Open→Doing/Blocked/Done/Cancelled"). `dependency_ids` may be
/// nonempty at creation — a brand-new action can declare dependencies on
/// existing actions immediately; existence/project-membership is checked,
/// but a full cycle walk is unnecessary here (see
/// `validate_action_dependencies_exist`'s own doc comment).
pub fn create_action(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    project_id: &str,
    title: &str,
    body: Option<&str>,
    dependency_ids: &[String],
) -> Result<(CommandOutcome, Action)> {
    require_project(writer.store(), project_id)?;
    check_len("action title", title, MAX_TITLE_BYTES)?;
    if let Some(b) = body {
        check_len("action body", b, MAX_BODY_BYTES)?;
    }
    validate_action_dependencies_exist(writer.store(), project_id, dependency_ids)?;
    let record = RecordPayload::Action(Action {
        payload_schema_version: RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: project_id.to_string(),
        title: title.to_string(),
        body: body.map(str::to_string),
        state: ActionState::Open,
        dependency_ids: dependency_ids.to_vec(),
        completion_summary: None,
        completed_at: None,
        completed_by: None,
        last_transition: None,
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

/// Replace an existing action's dependency list, cycle-checked against
/// every other action currently in the project.
pub fn set_action_dependencies(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    action_id: &str,
    expected_revision_id: &str,
    dependency_ids: &[String],
) -> Result<(CommandOutcome, Action)> {
    let (_, existing_payload) = writer
        .store()
        .read_current(action_id)?
        .ok_or_else(|| Error::Project(format!("no action exists with id {action_id}")))?;
    let mut action = match RecordPayload::from_json(&existing_payload)? {
        RecordPayload::Action(a) => a,
        other => {
            return Err(Error::Project(format!(
                "object {action_id} is not an action, found {}",
                other.kind_str()
            )))
        }
    };
    let project_id = action.project_id.clone();
    validate_action_dependency_graph(writer.store(), &project_id, action_id, dependency_ids)?;
    action.dependency_ids = dependency_ids.to_vec();
    let (outcome, record) = commit_update(
        writer,
        actor,
        RecordOrigin::User,
        action_id,
        expected_revision_id,
        RecordPayload::Action(action),
    )?;
    Ok((
        outcome,
        match record {
            RecordPayload::Action(a) => a,
            _ => unreachable!(),
        },
    ))
}

/// The exact allowed-transition table (§15 "Open→Doing/Blocked/Done/
/// Cancelled; any reopening requires an event... no autonomous
/// transitions"). Not in this table means rejected, including any
/// self-transition.
fn action_allowed_transition(from: ActionState, to: ActionState) -> bool {
    use ActionState::*;
    matches!(
        (from, to),
        (Open, Doing)
            | (Open, Blocked)
            | (Open, Done)
            | (Open, Cancelled)
            | (Doing, Blocked)
            | (Doing, Done)
            | (Doing, Cancelled)
            | (Blocked, Doing)
            | (Blocked, Done)
            | (Blocked, Cancelled)
            | (Done, Doing)
            | (Cancelled, Doing)
    )
}

fn is_reopen(from: ActionState, to: ActionState) -> bool {
    matches!(from, ActionState::Done | ActionState::Cancelled) && to == ActionState::Doing
}

/// The shared transition engine every public `*_action` function below
/// calls. Loads the current revision, validates the transition against
/// [`action_allowed_transition`], enforces the reopen-requires-a-reason and
/// completion-requires-a-summary rules, records [`ActionTransition`] on the
/// new revision, and commits — a single choke point so every transition
/// produces identically-shaped history.
fn transition_action(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    action_id: &str,
    expected_revision_id: &str,
    to: ActionState,
    reason: Option<&str>,
    completion_summary: Option<&str>,
) -> Result<(CommandOutcome, Action)> {
    let (_, existing_payload) = writer
        .store()
        .read_current(action_id)?
        .ok_or_else(|| Error::Project(format!("no action exists with id {action_id}")))?;
    let mut action = match RecordPayload::from_json(&existing_payload)? {
        RecordPayload::Action(a) => a,
        other => {
            return Err(Error::Project(format!(
                "object {action_id} is not an action, found {}",
                other.kind_str()
            )))
        }
    };
    let from = action.state;
    if !action_allowed_transition(from, to) {
        return Err(Error::Project(format!(
            "invalid action transition: {from:?} -> {to:?} is not an allowed transition"
        )));
    }

    if let Some(r) = reason {
        check_len("action transition reason", r, MAX_TRANSITION_REASON_BYTES)?;
    }
    if is_reopen(from, to) && reason.map(str::trim).unwrap_or("").is_empty() {
        return Err(Error::Project(
            "reopening a completed or cancelled action requires an explicit, non-empty reason (\"any reopening requires an event\")"
                .into(),
        ));
    }

    if to == ActionState::Done {
        let summary = completion_summary.unwrap_or("");
        if summary.trim().is_empty() {
            return Err(Error::Project(
                "completing an action requires a non-empty summary".into(),
            ));
        }
        check_len("action completion summary", summary, MAX_BODY_BYTES)?;
        action.completion_summary = Some(summary.to_string());
        action.completed_at = Some(crate::capture::now_rfc3339_utc());
        action.completed_by = Some(actor.to_string());
    } else if is_reopen(from, to) {
        // The prior completion remains fully visible in `store.history` (it
        // is the previous revision, immutable); clearing these fields on
        // the *current* revision only means the current view honestly
        // stops claiming the action is still done.
        action.completion_summary = None;
        action.completed_at = None;
        action.completed_by = None;
    }

    action.state = to;
    action.last_transition = Some(ActionTransition {
        from,
        to,
        reason: reason.map(str::to_string),
        actor: actor.to_string(),
        at: crate::capture::now_rfc3339_utc(),
    });

    let (outcome, record) = commit_update(
        writer,
        actor,
        RecordOrigin::User,
        action_id,
        expected_revision_id,
        RecordPayload::Action(action),
    )?;
    Ok((
        outcome,
        match record {
            RecordPayload::Action(a) => a,
            _ => unreachable!(),
        },
    ))
}

pub fn start_action(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    action_id: &str,
    expected_revision_id: &str,
) -> Result<(CommandOutcome, Action)> {
    transition_action(
        writer,
        actor,
        action_id,
        expected_revision_id,
        ActionState::Doing,
        None,
        None,
    )
}

pub fn block_action(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    action_id: &str,
    expected_revision_id: &str,
    reason: Option<&str>,
) -> Result<(CommandOutcome, Action)> {
    transition_action(
        writer,
        actor,
        action_id,
        expected_revision_id,
        ActionState::Blocked,
        reason,
        None,
    )
}

pub fn cancel_action(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    action_id: &str,
    expected_revision_id: &str,
    reason: Option<&str>,
) -> Result<(CommandOutcome, Action)> {
    transition_action(
        writer,
        actor,
        action_id,
        expected_revision_id,
        ActionState::Cancelled,
        reason,
        None,
    )
}

/// `Done`/`Cancelled` -> `Doing`. `reason` is mandatory and non-empty — "any
/// reopening requires an event" (§15), enforced by `transition_action`.
pub fn reopen_action(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    action_id: &str,
    expected_revision_id: &str,
    reason: &str,
) -> Result<(CommandOutcome, Action)> {
    transition_action(
        writer,
        actor,
        action_id,
        expected_revision_id,
        ActionState::Doing,
        Some(reason),
        None,
    )
}

/// Complete an action. `summary` is mandatory ("Completion requires
/// summary"). If any of the action's current `dependency_ids` is not
/// `Done`, completion is refused unless `dependency_override_reason` is an
/// explicit, non-empty reason ("blocked-dependency completion requires a
/// reasoned override") — the reason is recorded as this revision's
/// [`ActionTransition::reason`], so the override stays visible in history,
/// not silently applied.
pub fn complete_action(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    action_id: &str,
    expected_revision_id: &str,
    summary: &str,
    dependency_override_reason: Option<&str>,
) -> Result<(CommandOutcome, Action)> {
    let (_, payload) = writer
        .store()
        .read_current(action_id)?
        .ok_or_else(|| Error::Project(format!("no action exists with id {action_id}")))?;
    let current = match RecordPayload::from_json(&payload)? {
        RecordPayload::Action(a) => a,
        other => {
            return Err(Error::Project(format!(
                "object {action_id} is not an action, found {}",
                other.kind_str()
            )))
        }
    };
    let unmet = unmet_dependencies(writer.store(), &current.dependency_ids)?;
    if !unmet.is_empty() {
        let has_reason = dependency_override_reason
            .map(str::trim)
            .is_some_and(|r| !r.is_empty());
        if !has_reason {
            return Err(Error::Project(format!(
                "action {action_id} has unmet dependencies {unmet:?}; completing it anyway requires an explicit reasoned override"
            )));
        }
    }
    transition_action(
        writer,
        actor,
        action_id,
        expected_revision_id,
        ActionState::Done,
        dependency_override_reason,
        Some(summary),
    )
}

// ---------------------------------------------------------------------
// Decision: creation, valid-time validation, and lifecycle (T02-03).
// ---------------------------------------------------------------------

/// Half-open `[valid_from, valid_to)` (§15). Each bound, if present, must
/// parse as this crate's own strict UTC RFC3339 form
/// (`capture::parse_rfc3339_utc`); if both are present, `valid_from` must be
/// strictly before `valid_to` — an equal or inverted pair would describe an
/// empty or backwards interval, never a fabricated fix-up (I11).
fn validate_valid_interval(valid_from: Option<&str>, valid_to: Option<&str>) -> Result<()> {
    let from = valid_from
        .map(crate::capture::parse_rfc3339_utc)
        .transpose()
        .map_err(|e| Error::Project(format!("invalid valid_from: {e}")))?;
    let to = valid_to
        .map(crate::capture::parse_rfc3339_utc)
        .transpose()
        .map_err(|e| Error::Project(format!("invalid valid_to: {e}")))?;
    if let (Some(f), Some(t)) = (from, to) {
        if f >= t {
            return Err(Error::Project(format!(
                "invalid valid-time interval: valid_from ({}) must be strictly before valid_to ({}) — a half-open [from,to) interval cannot be empty or inverted",
                valid_from.unwrap_or_default(),
                valid_to.unwrap_or_default()
            )));
        }
    }
    Ok(())
}

/// Create a decision in `project_id`, always starting in
/// `DecisionLifecycle::Draft` regardless of `basis` — agent-origin content
/// never self-authorizes acceptance (S05/S06): reaching `Accepted` is only
/// ever `accept_decision`'s job, an explicit, separate owner action, exactly
/// mirroring `create_action` always starting `Open`.
#[allow(clippy::too_many_arguments)]
pub fn create_decision(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    project_id: &str,
    decision_key: &str,
    statement: &str,
    rationale: Option<&str>,
    basis: DecisionBasis,
    verification: DecisionVerification,
    valid_from: Option<&str>,
    valid_to: Option<&str>,
) -> Result<(CommandOutcome, Decision)> {
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
    validate_valid_interval(valid_from, valid_to)?;
    let record = RecordPayload::Decision(Decision {
        payload_schema_version: RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: project_id.to_string(),
        decision_key: decision_key.to_string(),
        statement: statement.to_string(),
        rationale: rationale.map(str::to_string),
        basis,
        verification,
        lifecycle: DecisionLifecycle::Draft,
        valid_from: valid_from.map(str::to_string),
        valid_to: valid_to.map(str::to_string),
        accepted_by: None,
        accepted_at: None,
        withdrawal_reason: None,
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

fn read_decision(store: &CanonicalStore, decision_id: &str) -> Result<Decision> {
    let (_, payload) = store
        .read_current(decision_id)?
        .ok_or_else(|| Error::Project(format!("no decision exists with id {decision_id}")))?;
    Ok(RecordPayload::from_json(&payload)?.as_decision()?.clone())
}

/// Owner-only acceptance (S05/S06): `Draft` -> `Accepted` only. F20: no
/// other path can reach `Accepted` — not import, not an agent proposal, not
/// a side effect of any other transition.
pub fn accept_decision(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    decision_id: &str,
    expected_revision_id: &str,
) -> Result<(CommandOutcome, Decision)> {
    let mut decision = read_decision(writer.store(), decision_id)?;
    if decision.lifecycle != DecisionLifecycle::Draft {
        return Err(Error::Project(format!(
            "decision {decision_id} cannot be accepted from lifecycle {:?}; only a draft decision may be accepted",
            decision.lifecycle
        )));
    }
    decision.lifecycle = DecisionLifecycle::Accepted;
    decision.accepted_by = Some(actor.to_string());
    decision.accepted_at = Some(crate::capture::now_rfc3339_utc());
    let (outcome, record) = commit_update(
        writer,
        actor,
        RecordOrigin::User,
        decision_id,
        expected_revision_id,
        RecordPayload::Decision(decision),
    )?;
    Ok((
        outcome,
        match record {
            RecordPayload::Decision(d) => d,
            _ => unreachable!(),
        },
    ))
}

/// `Draft` or `Accepted` -> `Withdrawn`. `reason` is mandatory — a
/// withdrawal without a stated reason would hide exactly the "explicit
/// judgment... remain visible" this task's acceptance criteria requires.
pub fn withdraw_decision(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    decision_id: &str,
    expected_revision_id: &str,
    reason: &str,
) -> Result<(CommandOutcome, Decision)> {
    if reason.trim().is_empty() {
        return Err(Error::Project(
            "withdrawing a decision requires an explicit, non-empty reason".into(),
        ));
    }
    check_len(
        "decision withdrawal reason",
        reason,
        MAX_TRANSITION_REASON_BYTES,
    )?;
    let mut decision = read_decision(writer.store(), decision_id)?;
    if !matches!(
        decision.lifecycle,
        DecisionLifecycle::Draft | DecisionLifecycle::Accepted
    ) {
        return Err(Error::Project(format!(
            "decision {decision_id} cannot be withdrawn from lifecycle {:?}",
            decision.lifecycle
        )));
    }
    decision.lifecycle = DecisionLifecycle::Withdrawn;
    decision.withdrawal_reason = Some(reason.to_string());
    let (outcome, record) = commit_update(
        writer,
        actor,
        RecordOrigin::User,
        decision_id,
        expected_revision_id,
        RecordPayload::Decision(decision),
    )?;
    Ok((
        outcome,
        match record {
            RecordPayload::Decision(d) => d,
            _ => unreachable!(),
        },
    ))
}

/// Supersede `old_decision_id` with `new_decision_id` (§15: "A supersession
/// must be owner-accepted, acyclic and within project"; §12 "Override: User
/// sees competing evidence, enters a reason and accepts an explicit
/// superseding revision/decision"). Both decisions must already be
/// `Accepted` and share the same `decision_key` (they must answer the same
/// question — see [`Decision::decision_key`]'s own doc comment).
///
/// This performs **two separate atomic commands**, not one joint
/// transaction: `crate::canonical::CommandTarget` admits exactly one object
/// per command (see its own doc comment) — there is no lower-level
/// multi-object primitive to invent one from without exactly the kind of
/// new canonical mechanism `AGENTS.md` §6/§7 forbids. Step 1 creates the
/// canonical `Supersedes` `Relation` (cycle/kind-checked by
/// `relation::create_relation` itself). Step 2 flips the old decision's own
/// `lifecycle` to `Superseded`, conflict-checked against
/// `expected_old_revision_id` exactly like any other update. If the process
/// fails between the two steps, the `Relation` from step 1 is already a
/// complete, valid, inspectable record on its own — a caller sees a
/// `Supersedes` edge naming a decision whose lifecycle has not yet caught
/// up, an honest partial state, never silent corruption, and step 2 alone
/// can be retried to complete it.
pub fn supersede_decision(
    store: &mut CanonicalStore,
    actor: &str,
    new_decision_id: &str,
    old_decision_id: &str,
    expected_old_revision_id: &str,
    reason: &str,
) -> Result<(CommandOutcome, CommandOutcome, Decision)> {
    if reason.trim().is_empty() {
        return Err(Error::Project(
            "superseding a decision requires an explicit, non-empty reason".into(),
        ));
    }
    check_len(
        "decision supersession reason",
        reason,
        MAX_TRANSITION_REASON_BYTES,
    )?;

    let new_decision = read_decision(store, new_decision_id)?;
    let mut old_decision = read_decision(store, old_decision_id)?;
    if new_decision.lifecycle != DecisionLifecycle::Accepted {
        return Err(Error::InvalidSupersession(format!(
            "superseding decision {new_decision_id} must already be accepted (found {:?})",
            new_decision.lifecycle
        )));
    }
    if old_decision.lifecycle != DecisionLifecycle::Accepted {
        return Err(Error::InvalidSupersession(format!(
            "superseded decision {old_decision_id} must currently be accepted (found {:?})",
            old_decision.lifecycle
        )));
    }
    if new_decision.decision_key != old_decision.decision_key {
        return Err(Error::InvalidSupersession(format!(
            "supersession requires the same decision_key ({:?} vs {:?})",
            new_decision.decision_key, old_decision.decision_key
        )));
    }

    let relation_outcome = {
        let mut writer = store.writer()?;
        crate::relation::create_relation(
            &mut writer,
            actor,
            &old_decision.project_id,
            crate::relation::RelationType::Supersedes,
            new_decision_id,
            old_decision_id,
            Some(reason),
        )?
        .0
    };

    old_decision.lifecycle = DecisionLifecycle::Superseded;
    let (update_outcome, updated) = {
        let mut writer = store.writer()?;
        commit_update(
            &mut writer,
            actor,
            RecordOrigin::User,
            old_decision_id,
            expected_old_revision_id,
            RecordPayload::Decision(old_decision),
        )?
    };
    Ok((
        relation_outcome,
        update_outcome,
        match updated {
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

/// Every work record (`Note`/`Action`/`Decision`, never `Project`,
/// `Source`, or `Relation`) currently belonging to `project_id`. A full
/// scan — see [`CanonicalStore::list_current_objects`]'s own documented
/// limitation.
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
            // `Source` (`capture::list_project_sources`), `Relation`
            // (`relation::list_project_relations`), `SourceCheck`
            // (`source_check::list_project_source_checks`),
            // `ReviewCheckpoint` (`checkpoint::current_checkpoint`),
            // `ExportGrant` (`grant::current_grant`, via a caller-known ID
            // — grants have no dedicated project-scoped listing function
            // since nothing yet needs "every grant for a project") and
            // `AgentProposal` (`proposal::list_project_proposals`) each
            // have their own dedicated listing function.
            RecordPayload::Source(_)
            | RecordPayload::Relation(_)
            | RecordPayload::Project(_)
            | RecordPayload::SourceCheck(_)
            | RecordPayload::ReviewCheckpoint(_)
            | RecordPayload::ExportGrant(_)
            | RecordPayload::DisclosureReceipt(_)
            | RecordPayload::AgentProposal(_) => false,
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

    fn new_project(store: &mut CanonicalStore) -> String {
        let mut writer = store.writer().unwrap();
        create_project(&mut writer, "owner", "P", None)
            .unwrap()
            .0
            .object_id
    }

    fn new_action(store: &mut CanonicalStore, project_id: &str) -> (String, String) {
        let mut writer = store.writer().unwrap();
        let (outcome, _) = create_action(&mut writer, "owner", project_id, "A", None, &[]).unwrap();
        (outcome.object_id, outcome.revision_id)
    }

    fn new_decision(store: &mut CanonicalStore, project_id: &str, key: &str) -> (String, String) {
        let mut writer = store.writer().unwrap();
        let (outcome, _) = create_decision(
            &mut writer,
            "owner",
            project_id,
            key,
            "statement",
            None,
            DecisionBasis::UserJudgment,
            DecisionVerification::Unreviewed,
            None,
            None,
        )
        .unwrap();
        (outcome.object_id, outcome.revision_id)
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
    fn note_tombstone_untombstone_round_trips_and_preserves_history() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
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
        assert!(!note.tombstoned);

        let (tombstone_outcome, tombstoned) = {
            let mut writer = store.writer().unwrap();
            tombstone_note(
                &mut writer,
                "owner",
                &note_outcome.object_id,
                &note_outcome.revision_id,
            )
            .unwrap()
        };
        assert!(tombstoned.tombstoned);
        // Content is preserved, never cleared, by a tombstone -- it is a
        // lifecycle flag flip, not a content deletion.
        assert_eq!(tombstoned.title, Some("Title".to_string()));
        assert_eq!(tombstoned.body, "Body text");

        // Stale expected_revision_id conflicts exactly like `update_note`.
        let err = {
            let mut writer = store.writer().unwrap();
            tombstone_note(
                &mut writer,
                "owner",
                &note_outcome.object_id,
                &note_outcome.revision_id, // stale: superseded by tombstone_outcome
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("expected revision conflict"));

        let (_, untombstoned) = {
            let mut writer = store.writer().unwrap();
            untombstone_note(
                &mut writer,
                "owner",
                &note_outcome.object_id,
                &tombstone_outcome.revision_id,
            )
            .unwrap()
        };
        assert!(!untombstoned.tombstoned);

        // History preserved: create, tombstone, untombstone -- three
        // distinct revisions, none rewritten.
        let history = store.history(&note_outcome.object_id).unwrap();
        assert_eq!(history.len(), 3);

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
                DecisionBasis::UserJudgment,
                DecisionVerification::Unreviewed,
                None,
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
                &[],
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
                DecisionBasis::UserJudgment,
                DecisionVerification::Unreviewed,
                None,
                None,
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
            create_action(&mut writer, "owner", &p1.object_id, "a1", None, &[]).unwrap();
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

    // -------------------------------------------------------------
    // T02-03: Action state machine
    // -------------------------------------------------------------

    #[test]
    fn action_lifecycle_table_allows_every_documented_transition() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);

        // Open -> Doing -> Blocked -> Doing -> Done -> (reopen) Doing -> Cancelled
        let (id, mut rev) = new_action(&mut store, &project_id);
        {
            let mut writer = store.writer().unwrap();
            let (o, a) = start_action(&mut writer, "owner", &id, &rev).unwrap();
            assert_eq!(a.state, ActionState::Doing);
            rev = o.revision_id;
        }
        {
            let mut writer = store.writer().unwrap();
            let (o, a) = block_action(&mut writer, "owner", &id, &rev, Some("waiting")).unwrap();
            assert_eq!(a.state, ActionState::Blocked);
            rev = o.revision_id;
        }
        {
            let mut writer = store.writer().unwrap();
            let (o, a) = start_action(&mut writer, "owner", &id, &rev).unwrap();
            assert_eq!(a.state, ActionState::Doing);
            rev = o.revision_id;
        }
        {
            let mut writer = store.writer().unwrap();
            let (o, a) = complete_action(&mut writer, "owner", &id, &rev, "done!", None).unwrap();
            assert_eq!(a.state, ActionState::Done);
            assert_eq!(a.completion_summary.as_deref(), Some("done!"));
            assert!(a.completed_at.is_some());
            assert_eq!(a.completed_by.as_deref(), Some("owner"));
            rev = o.revision_id;
        }
        {
            let mut writer = store.writer().unwrap();
            let (o, a) = reopen_action(&mut writer, "owner", &id, &rev, "found a bug").unwrap();
            assert_eq!(a.state, ActionState::Doing);
            assert!(
                a.completion_summary.is_none(),
                "reopen must clear the prior completion fields on the current revision"
            );
            rev = o.revision_id;
        }
        {
            let mut writer = store.writer().unwrap();
            let (_, a) =
                cancel_action(&mut writer, "owner", &id, &rev, Some("no longer needed")).unwrap();
            assert_eq!(a.state, ActionState::Cancelled);
        }

        // Full history is reconstructible and self-describing.
        let history = store.history(&id).unwrap();
        // create, start, block, start, complete, reopen, cancel = 7 revisions
        assert_eq!(history.len(), 7);
        let transitions: Vec<Option<ActionTransition>> = history
            .iter()
            .map(|(_, _, payload)| {
                RecordPayload::from_json(payload)
                    .unwrap()
                    .as_action()
                    .unwrap()
                    .last_transition
                    .clone()
            })
            .collect();
        assert!(
            transitions[0].is_none(),
            "the create revision has no transition"
        );
        assert_eq!(
            transitions[5].as_ref().unwrap().reason.as_deref(),
            Some("found a bug")
        );
        assert_eq!(transitions[5].as_ref().unwrap().from, ActionState::Done);
        assert_eq!(transitions[5].as_ref().unwrap().to, ActionState::Doing);

        cleanup(&root);
    }

    #[test]
    fn forbidden_action_transitions_are_rejected() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (id, rev) = new_action(&mut store, &project_id);

        // Open -> Open (self-transition) is not in the table.
        let err = {
            let mut writer = store.writer().unwrap();
            transition_action(
                &mut writer,
                "owner",
                &id,
                &rev,
                ActionState::Open,
                None,
                None,
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("not an allowed transition"));

        // Done -> Blocked is not in the table (must reopen through Doing).
        let rev_after_done = {
            let mut writer = store.writer().unwrap();
            complete_action(&mut writer, "owner", &id, &rev, "s", None)
                .unwrap()
                .0
                .revision_id
        };
        let err = {
            let mut writer = store.writer().unwrap();
            block_action(&mut writer, "owner", &id, &rev_after_done, None).unwrap_err()
        };
        assert!(format!("{err}").contains("not an allowed transition"));

        cleanup(&root);
    }

    #[test]
    fn reopening_without_a_reason_is_rejected() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (id, rev) = new_action(&mut store, &project_id);
        let rev = {
            let mut writer = store.writer().unwrap();
            complete_action(&mut writer, "owner", &id, &rev, "s", None)
                .unwrap()
                .0
                .revision_id
        };
        let err = {
            let mut writer = store.writer().unwrap();
            reopen_action(&mut writer, "owner", &id, &rev, "   ").unwrap_err()
        };
        assert!(format!("{err}").contains("requires an explicit, non-empty reason"));
        // Head unchanged: no partial/half-applied reopen was recorded.
        let history = store.history(&id).unwrap();
        assert_eq!(history.len(), 2); // create, complete only
        cleanup(&root);
    }

    #[test]
    fn completing_without_a_summary_is_rejected() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (id, rev) = new_action(&mut store, &project_id);
        let err = {
            let mut writer = store.writer().unwrap();
            complete_action(&mut writer, "owner", &id, &rev, "  ", None).unwrap_err()
        };
        assert!(format!("{err}").contains("requires a non-empty summary"));
        cleanup(&root);
    }

    #[test]
    fn completion_with_unmet_dependency_requires_an_override_reason() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (dep_id, _) = new_action(&mut store, &project_id);

        let (id, rev) = {
            let mut writer = store.writer().unwrap();
            let (o, _) = create_action(
                &mut writer,
                "owner",
                &project_id,
                "depends on dep",
                None,
                std::slice::from_ref(&dep_id),
            )
            .unwrap();
            (o.object_id, o.revision_id)
        };

        // dep is still Open (not Done): refused without an override reason.
        let err = {
            let mut writer = store.writer().unwrap();
            complete_action(&mut writer, "owner", &id, &rev, "done anyway", None).unwrap_err()
        };
        assert!(format!("{err}").contains("unmet dependencies"));

        // With an explicit override reason, completion succeeds and the
        // reason is recorded, visibly, on this revision's transition.
        let (_, action) = {
            let mut writer = store.writer().unwrap();
            complete_action(
                &mut writer,
                "owner",
                &id,
                &rev,
                "done anyway",
                Some("dependency turned out unnecessary"),
            )
            .unwrap()
        };
        assert_eq!(action.state, ActionState::Done);
        assert_eq!(
            action.last_transition.unwrap().reason.as_deref(),
            Some("dependency turned out unnecessary")
        );

        cleanup(&root);
    }

    #[test]
    fn completion_succeeds_without_override_once_dependency_is_done() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (dep_id, dep_rev) = new_action(&mut store, &project_id);
        {
            let mut writer = store.writer().unwrap();
            complete_action(&mut writer, "owner", &dep_id, &dep_rev, "dep done", None).unwrap();
        }

        let (id, rev) = {
            let mut writer = store.writer().unwrap();
            let (o, _) = create_action(
                &mut writer,
                "owner",
                &project_id,
                "depends on dep",
                None,
                std::slice::from_ref(&dep_id),
            )
            .unwrap();
            (o.object_id, o.revision_id)
        };
        let (_, action) = {
            let mut writer = store.writer().unwrap();
            complete_action(&mut writer, "owner", &id, &rev, "done", None).unwrap()
        };
        assert_eq!(action.state, ActionState::Done);

        cleanup(&root);
    }

    #[test]
    fn action_dependency_must_exist_and_be_in_the_same_project() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let other_project = new_project(&mut store);
        let (other_action, _) = new_action(&mut store, &other_project);
        let missing = uuid::Uuid::now_v7().to_string();

        let err = {
            let mut writer = store.writer().unwrap();
            create_action(
                &mut writer,
                "owner",
                &project_id,
                "x",
                None,
                std::slice::from_ref(&missing),
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("no object exists"));

        let err = {
            let mut writer = store.writer().unwrap();
            create_action(
                &mut writer,
                "owner",
                &project_id,
                "x",
                None,
                std::slice::from_ref(&other_action),
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("different project"));

        cleanup(&root);
    }

    #[test]
    fn action_dependency_direct_and_transitive_cycles_are_rejected() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (a, a_rev) = new_action(&mut store, &project_id);
        let (b, _) = {
            let mut writer = store.writer().unwrap();
            let (o, _) = create_action(
                &mut writer,
                "owner",
                &project_id,
                "b",
                None,
                std::slice::from_ref(&a),
            )
            .unwrap();
            (o.object_id, o.revision_id)
        };

        // a -> b would close a direct 2-cycle (b already depends on a).
        let err = {
            let mut writer = store.writer().unwrap();
            set_action_dependencies(&mut writer, "owner", &a, &a_rev, std::slice::from_ref(&b))
                .unwrap_err()
        };
        assert!(format!("{err}").contains("cycle"));

        // Self-reference is rejected too.
        let err = {
            let mut writer = store.writer().unwrap();
            set_action_dependencies(&mut writer, "owner", &a, &a_rev, std::slice::from_ref(&a))
                .unwrap_err()
        };
        assert!(format!("{err}").contains("cannot depend on itself"));

        cleanup(&root);
    }

    #[test]
    fn action_transition_conflicts_on_a_stale_expected_revision() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (id, rev) = new_action(&mut store, &project_id);
        {
            let mut writer = store.writer().unwrap();
            start_action(&mut writer, "owner", &id, &rev).unwrap();
        }
        // rev is now stale (superseded by the start_action revision above).
        let err = {
            let mut writer = store.writer().unwrap();
            block_action(&mut writer, "owner", &id, &rev, None).unwrap_err()
        };
        assert!(format!("{err}").contains("expected revision conflict"));
        cleanup(&root);
    }

    // -------------------------------------------------------------
    // T02-03: Decision valid-time, acceptance, override/supersession
    // -------------------------------------------------------------

    #[test]
    fn valid_interval_rejects_malformed_and_inverted_bounds() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);

        let err = {
            let mut writer = store.writer().unwrap();
            create_decision(
                &mut writer,
                "owner",
                &project_id,
                "k",
                "s",
                None,
                DecisionBasis::UserJudgment,
                DecisionVerification::Unreviewed,
                Some("not-a-timestamp"),
                None,
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("invalid valid_from"));

        let err = {
            let mut writer = store.writer().unwrap();
            create_decision(
                &mut writer,
                "owner",
                &project_id,
                "k",
                "s",
                None,
                DecisionBasis::UserJudgment,
                DecisionVerification::Unreviewed,
                Some("2024-06-01T00:00:00Z"),
                Some("2024-01-01T00:00:00Z"), // to before from
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("must be strictly before"));

        // Equal bounds describe an empty interval — also rejected.
        let err = {
            let mut writer = store.writer().unwrap();
            create_decision(
                &mut writer,
                "owner",
                &project_id,
                "k",
                "s",
                None,
                DecisionBasis::UserJudgment,
                DecisionVerification::Unreviewed,
                Some("2024-06-01T00:00:00Z"),
                Some("2024-06-01T00:00:00Z"),
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("must be strictly before"));

        cleanup(&root);
    }

    #[test]
    fn valid_interval_accepts_a_well_formed_half_open_range() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (_, decision) = {
            let mut writer = store.writer().unwrap();
            create_decision(
                &mut writer,
                "owner",
                &project_id,
                "k",
                "s",
                None,
                DecisionBasis::Evidence,
                DecisionVerification::UserReviewed,
                Some("2024-01-01T00:00:00Z"),
                Some("2024-06-01T00:00:00Z"),
            )
            .unwrap()
        };
        assert_eq!(decision.valid_from.as_deref(), Some("2024-01-01T00:00:00Z"));
        assert_eq!(decision.valid_to.as_deref(), Some("2024-06-01T00:00:00Z"));
        cleanup(&root);
    }

    #[test]
    fn only_a_draft_decision_can_be_accepted() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (id, rev) = new_decision(&mut store, &project_id, "k");

        let rev = {
            let mut writer = store.writer().unwrap();
            let (o, d) = accept_decision(&mut writer, "owner", &id, &rev).unwrap();
            assert_eq!(d.lifecycle, DecisionLifecycle::Accepted);
            assert_eq!(d.accepted_by.as_deref(), Some("owner"));
            assert!(d.accepted_at.is_some());
            o.revision_id
        };

        // Already accepted: accepting again is refused, not silently reapplied.
        let err = {
            let mut writer = store.writer().unwrap();
            accept_decision(&mut writer, "owner", &id, &rev).unwrap_err()
        };
        assert!(format!("{err}").contains("only a draft decision may be accepted"));

        cleanup(&root);
    }

    #[test]
    fn withdraw_requires_a_reason_and_preserves_prior_history() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (id, rev) = new_decision(&mut store, &project_id, "k");

        let err = {
            let mut writer = store.writer().unwrap();
            withdraw_decision(&mut writer, "owner", &id, &rev, "").unwrap_err()
        };
        assert!(format!("{err}").contains("requires an explicit, non-empty reason"));

        let (_, withdrawn) = {
            let mut writer = store.writer().unwrap();
            withdraw_decision(&mut writer, "owner", &id, &rev, "no longer relevant").unwrap()
        };
        assert_eq!(withdrawn.lifecycle, DecisionLifecycle::Withdrawn);
        assert_eq!(
            withdrawn.withdrawal_reason.as_deref(),
            Some("no longer relevant")
        );

        let history = store.history(&id).unwrap();
        assert_eq!(history.len(), 2); // create (Draft), withdraw (Withdrawn)
        let first = RecordPayload::from_json(&history[0].2).unwrap();
        assert_eq!(
            first.as_decision().unwrap().lifecycle,
            DecisionLifecycle::Draft
        );

        cleanup(&root);
    }

    #[test]
    fn supersede_requires_both_decisions_accepted_and_same_key() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (old_id, old_rev) = new_decision(&mut store, &project_id, "k1");
        let (new_id, new_rev) = new_decision(&mut store, &project_id, "k1");

        // Neither is accepted yet.
        let err = supersede_decision(&mut store, "owner", &new_id, &old_id, &old_rev, "because")
            .unwrap_err();
        assert!(format!("{err}").contains("must already be accepted"));

        {
            let mut writer = store.writer().unwrap();
            accept_decision(&mut writer, "owner", &new_id, &new_rev).unwrap();
        }
        // new is accepted, old is not.
        let err = supersede_decision(&mut store, "owner", &new_id, &old_id, &old_rev, "because")
            .unwrap_err();
        assert!(format!("{err}").contains("must currently be accepted"));

        let old_rev = {
            let mut writer = store.writer().unwrap();
            accept_decision(&mut writer, "owner", &old_id, &old_rev)
                .unwrap()
                .0
                .revision_id
        };

        // Different key: rejected.
        let (other_key_id, other_key_rev) = new_decision(&mut store, &project_id, "different-key");
        {
            let mut writer = store.writer().unwrap();
            accept_decision(&mut writer, "owner", &other_key_id, &other_key_rev).unwrap();
        }
        let err = supersede_decision(
            &mut store,
            "owner",
            &other_key_id,
            &old_id,
            &old_rev,
            "because",
        )
        .unwrap_err();
        assert!(format!("{err}").contains("same decision_key"));

        cleanup(&root);
    }

    #[test]
    fn supersede_preserves_prior_accepted_history_and_links_a_relation() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (old_id, old_rev) = new_decision(&mut store, &project_id, "k1");
        let (new_id, new_rev) = new_decision(&mut store, &project_id, "k1");
        let old_rev = {
            let mut writer = store.writer().unwrap();
            accept_decision(&mut writer, "owner", &old_id, &old_rev)
                .unwrap()
                .0
                .revision_id
        };
        {
            let mut writer = store.writer().unwrap();
            accept_decision(&mut writer, "owner", &new_id, &new_rev).unwrap();
        }

        let (relation_outcome, _update_outcome, updated_old) = supersede_decision(
            &mut store,
            "owner",
            &new_id,
            &old_id,
            &old_rev,
            "new evidence arrived",
        )
        .unwrap();
        assert_eq!(updated_old.lifecycle, DecisionLifecycle::Superseded);

        // Prior Accepted revision of the old decision remains in history.
        let history = store.history(&old_id).unwrap();
        assert_eq!(history.len(), 3); // create(Draft), accept(Accepted), supersede(Superseded)
        let accepted_revision = RecordPayload::from_json(&history[1].2).unwrap();
        assert_eq!(
            accepted_revision.as_decision().unwrap().lifecycle,
            DecisionLifecycle::Accepted
        );

        // The Supersedes relation is discoverable from either endpoint and
        // carries the reason — no second copy on the Decision itself.
        let relation = crate::relation::list_relations_for_object(&store, &old_id).unwrap();
        assert_eq!(relation.len(), 1);
        assert_eq!(relation[0].0, relation_outcome.object_id);
        assert_eq!(
            relation[0].1.relation_type,
            crate::relation::RelationType::Supersedes
        );
        assert_eq!(relation[0].1.note.as_deref(), Some("new evidence arrived"));
        assert_eq!(relation[0].1.from_object_id, new_id);
        assert_eq!(relation[0].1.to_object_id, old_id);

        cleanup(&root);
    }

    #[test]
    fn supersede_rejects_a_cycle_between_two_decisions() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (a, a_rev) = new_decision(&mut store, &project_id, "k1");
        let (b, b_rev) = new_decision(&mut store, &project_id, "k1");
        let a_rev = {
            let mut writer = store.writer().unwrap();
            accept_decision(&mut writer, "owner", &a, &a_rev)
                .unwrap()
                .0
                .revision_id
        };
        let b_rev = {
            let mut writer = store.writer().unwrap();
            accept_decision(&mut writer, "owner", &b, &b_rev)
                .unwrap()
                .0
                .revision_id
        };

        // b supersedes a.
        supersede_decision(&mut store, "owner", &b, &a, &a_rev, "b replaces a").unwrap();

        // a is now Superseded, not Accepted — re-accepting it to attempt the
        // reverse edge is refused by the acceptance gate itself, but even a
        // hypothetical re-acceptance must not let a supersede b: prove the
        // underlying relation-layer cycle check directly.
        let err = {
            let mut writer = store.writer().unwrap();
            crate::relation::create_relation(
                &mut writer,
                "owner",
                &project_id,
                crate::relation::RelationType::Supersedes,
                &a,
                &b,
                None,
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("cycle"));
        let _ = b_rev;

        cleanup(&root);
    }
}
