//! T04-01's entire typed bridge surface. Every command here does nothing
//! Core doesn't already do: it opens/creates a [`fehrest::canonical::CanonicalStore`]
//! at an owner-chosen local path and calls an already-audited `fehrest::project`
//! function. No new authority model, no filesystem/shell/http/process access
//! beyond exactly what each function below names -- there is no general
//! "read file" or "run command" bridge command anywhere in this crate.
//!
//! Every path argument here originates only from the owner's own native
//! dialog selection (`pick_directory`) or their own typed vault/project
//! name -- never from imported or remote content, which this shell never
//! renders as executable in the first place (strict CSP, no `dangerouslySetInnerHTML`-
//! equivalent, no filesystem/http/shell/process plugins admitted).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use fehrest::backup::{self, BackupReport, RestoreReport};
use fehrest::canonical::CanonicalStore;
use fehrest::capture;
use fehrest::checkpoint::{self, ReviewCheckpoint};
use fehrest::disclosure::{self, DisclosureReceipt};
use fehrest::export::{self, ExportPreview, ExportReport};
use fehrest::grant::{self, ExportGrant};
use fehrest::import::{self, import_full_restore, ImportPreview, ImportReport};
use fehrest::project::{
    self, ActionState, DecisionBasis, DecisionLifecycle, DecisionVerification, RecordPayload,
};
use fehrest::proposal::{self, AgentProposal};
use fehrest::recovery::{self, RecoveryReport};
use fehrest::relation::{self, RelationType};
use fehrest::resume::ResumeView;
use fehrest::source_check::{self, CheckStatus, SourceCheck};

const DESKTOP_ACTOR: &str = "owner";
const MAX_NAME_BYTES: usize = 200;

#[derive(Debug, Clone, serde::Serialize)]
pub struct VaultInfo {
    pub path: String,
    pub vault_id: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProjectSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NoteInfo {
    pub id: String,
    pub project_id: String,
    pub title: Option<String>,
    pub body: String,
    /// The exact revision this note is at *after* this call. The frontend
    /// must send this back as `expectedRevisionId` on the next save -- T04-02's
    /// "conflicting editor revision shows review options, never silent
    /// overwrite" requirement is enforced entirely by Core's own
    /// expected-revision conflict check (`project::update_note`); this
    /// command adds no separate conflict logic of its own.
    pub revision_id: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ActionSummary {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub body: Option<String>,
    pub state: ActionState,
    pub dependency_ids: Vec<String>,
    pub completion_summary: Option<String>,
    pub revision_id: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DecisionSummary {
    pub id: String,
    pub project_id: String,
    pub decision_key: String,
    pub statement: String,
    pub rationale: Option<String>,
    pub basis: DecisionBasis,
    pub verification: DecisionVerification,
    pub lifecycle: DecisionLifecycle,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub revision_id: String,
}

/// Relations are create-only in Core (no `update_relation` exists) -- there
/// is deliberately no `revision_id` here for a caller to send back on a
/// future update, unlike every other summary type in this file.
#[derive(Debug, Clone, serde::Serialize)]
pub struct RelationEntry {
    pub id: String,
    pub project_id: String,
    pub relation_type: RelationType,
    pub from_object_id: String,
    pub to_object_id: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SupersedeResult {
    pub relation_id: String,
    pub old_decision: DecisionSummary,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchHit {
    pub kind: String,
    pub id: String,
    pub title: Option<String>,
    pub snippet: String,
}

fn snippet_around(haystack: &str, needle_lower: &str, radius: usize) -> String {
    let hay_lower = haystack.to_lowercase();
    let byte_idx = match hay_lower.find(needle_lower) {
        Some(i) => i,
        None => return haystack.chars().take(radius * 2).collect(),
    };
    // Walk to char boundaries; `haystack`/`hay_lower` share byte offsets
    // for any needle that only contains ASCII, which every caller here
    // guarantees by lowercasing an ASCII-cheap query -- non-ASCII match
    // positions still fall back correctly since `char_indices` below never
    // panics on any UTF-8 input.
    let start_char = haystack[..byte_idx.min(haystack.len())]
        .chars()
        .count()
        .saturating_sub(radius);
    let end_char = haystack[..byte_idx.min(haystack.len())].chars().count()
        + needle_lower.chars().count()
        + radius;
    let snippet: String = haystack
        .chars()
        .skip(start_char)
        .take(end_char.saturating_sub(start_char))
        .collect();
    snippet
}

fn bounded_name(field: &str, value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    if value.len() > MAX_NAME_BYTES {
        return Err(format!("{field} must be at most {MAX_NAME_BYTES} bytes"));
    }
    Ok(())
}

fn join_child(parent: &str, name: &str) -> Result<PathBuf, String> {
    bounded_name("name", name)?;
    if name.contains(['/', '\\']) || name == "." || name == ".." {
        return Err("name must be a single path segment, not a path".to_string());
    }
    Ok(Path::new(parent).join(name))
}

#[tauri::command]
pub fn vault_create(parent_dir: String, name: String) -> Result<VaultInfo, String> {
    let dest = join_child(&parent_dir, &name)?;
    let store = CanonicalStore::create(&dest).map_err(|e| e.to_string())?;
    Ok(VaultInfo {
        path: dest.to_string_lossy().to_string(),
        vault_id: store.vault_id().to_string(),
    })
}

#[tauri::command]
pub fn vault_open(path: String) -> Result<VaultInfo, String> {
    if path.trim().is_empty() {
        return Err("path must not be empty".to_string());
    }
    let store = CanonicalStore::open(&path).map_err(|e| e.to_string())?;
    Ok(VaultInfo {
        path,
        vault_id: store.vault_id().to_string(),
    })
}

#[tauri::command]
pub fn vault_restore(
    source_path: String,
    parent_dir: String,
    name: String,
) -> Result<VaultInfo, String> {
    if source_path.trim().is_empty() {
        return Err("source_path must not be empty".to_string());
    }
    let dest = join_child(&parent_dir, &name)?;
    import_full_restore(Path::new(&source_path), &dest).map_err(|e| e.to_string())?;
    let store = CanonicalStore::open(&dest).map_err(|e| e.to_string())?;
    Ok(VaultInfo {
        path: dest.to_string_lossy().to_string(),
        vault_id: store.vault_id().to_string(),
    })
}

#[tauri::command]
pub fn list_projects(vault_path: String) -> Result<Vec<ProjectSummary>, String> {
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let objects = store.list_current_objects().map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for (object_id, _revision_id, payload) in objects {
        let record = match RecordPayload::from_json(&payload) {
            Ok(r) => r,
            Err(_) => continue,
        };
        if let RecordPayload::Project(p) = record {
            out.push(ProjectSummary {
                id: object_id,
                name: p.name,
                description: p.description,
                active: p.active,
            });
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name).then(a.id.cmp(&b.id)));
    Ok(out)
}

#[tauri::command]
pub fn create_project(
    vault_path: String,
    name: String,
    description: Option<String>,
) -> Result<ProjectSummary, String> {
    bounded_name("project name", &name)?;
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, created) =
        project::create_project(&mut writer, DESKTOP_ACTOR, &name, description.as_deref())
            .map_err(|e| e.to_string())?;
    Ok(ProjectSummary {
        id: outcome.object_id,
        name: created.name,
        description: created.description,
        active: created.active,
    })
}

#[tauri::command]
pub fn list_notes(vault_path: String, project_id: String) -> Result<Vec<NoteInfo>, String> {
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let objects = store.list_current_objects().map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for (object_id, revision_id, payload) in objects {
        let record = match RecordPayload::from_json(&payload) {
            Ok(r) => r,
            Err(_) => continue,
        };
        if let RecordPayload::Note(n) = record {
            if n.project_id == project_id && !n.tombstoned {
                out.push(NoteInfo {
                    id: object_id,
                    project_id: n.project_id,
                    title: n.title,
                    body: n.body,
                    revision_id,
                });
            }
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

#[tauri::command]
pub fn note_create(
    vault_path: String,
    project_id: String,
    title: Option<String>,
    body: String,
) -> Result<NoteInfo, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, created) = project::create_note(
        &mut writer,
        DESKTOP_ACTOR,
        &project_id,
        title.as_deref(),
        &body,
    )
    .map_err(|e| e.to_string())?;
    Ok(NoteInfo {
        id: outcome.object_id,
        project_id: created.project_id,
        title: created.title,
        body: created.body,
        revision_id: outcome.revision_id,
    })
}

#[tauri::command]
pub fn note_update(
    vault_path: String,
    note_id: String,
    expected_revision_id: String,
    title: Option<String>,
    body: String,
) -> Result<NoteInfo, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, updated) = project::update_note(
        &mut writer,
        DESKTOP_ACTOR,
        &note_id,
        &expected_revision_id,
        title.as_deref(),
        &body,
    )
    .map_err(|e| e.to_string())?;
    Ok(NoteInfo {
        id: note_id,
        project_id: updated.project_id,
        title: updated.title,
        body: updated.body,
        revision_id: outcome.revision_id,
    })
}

#[tauri::command]
pub fn note_tombstone(
    vault_path: String,
    note_id: String,
    expected_revision_id: String,
) -> Result<NoteInfo, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, updated) =
        project::tombstone_note(&mut writer, DESKTOP_ACTOR, &note_id, &expected_revision_id)
            .map_err(|e| e.to_string())?;
    Ok(NoteInfo {
        id: note_id,
        project_id: updated.project_id,
        title: updated.title,
        body: updated.body,
        revision_id: outcome.revision_id,
    })
}

#[tauri::command]
pub fn note_untombstone(
    vault_path: String,
    note_id: String,
    expected_revision_id: String,
) -> Result<NoteInfo, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, updated) =
        project::untombstone_note(&mut writer, DESKTOP_ACTOR, &note_id, &expected_revision_id)
            .map_err(|e| e.to_string())?;
    Ok(NoteInfo {
        id: note_id,
        project_id: updated.project_id,
        title: updated.title,
        body: updated.body,
        revision_id: outcome.revision_id,
    })
}

#[tauri::command]
pub fn project_archive(vault_path: String, project_id: String) -> Result<ProjectSummary, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (_, updated) = project::archive_project(&mut store, DESKTOP_ACTOR, &project_id)
        .map_err(|e| e.to_string())?;
    Ok(ProjectSummary {
        id: project_id,
        name: updated.name,
        description: updated.description,
        active: updated.active,
    })
}

#[tauri::command]
pub fn project_unarchive(vault_path: String, project_id: String) -> Result<ProjectSummary, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (_, updated) = project::unarchive_project(&mut store, DESKTOP_ACTOR, &project_id)
        .map_err(|e| e.to_string())?;
    Ok(ProjectSummary {
        id: project_id,
        name: updated.name,
        description: updated.description,
        active: updated.active,
    })
}

#[tauri::command]
pub fn list_actions(vault_path: String, project_id: String) -> Result<Vec<ActionSummary>, String> {
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let objects = store.list_current_objects().map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for (object_id, revision_id, payload) in objects {
        let record = match RecordPayload::from_json(&payload) {
            Ok(r) => r,
            Err(_) => continue,
        };
        if let RecordPayload::Action(a) = record {
            if a.project_id == project_id {
                out.push(ActionSummary {
                    id: object_id,
                    project_id: a.project_id,
                    title: a.title,
                    body: a.body,
                    state: a.state,
                    dependency_ids: a.dependency_ids,
                    completion_summary: a.completion_summary,
                    revision_id,
                });
            }
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

#[tauri::command]
pub fn action_create(
    vault_path: String,
    project_id: String,
    title: String,
    body: Option<String>,
    depends_on: Vec<String>,
) -> Result<ActionSummary, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, created) = project::create_action(
        &mut writer,
        DESKTOP_ACTOR,
        &project_id,
        &title,
        body.as_deref(),
        &depends_on,
    )
    .map_err(|e| e.to_string())?;
    Ok(ActionSummary {
        id: outcome.object_id,
        project_id: created.project_id,
        title: created.title,
        body: created.body,
        state: created.state,
        dependency_ids: created.dependency_ids,
        completion_summary: created.completion_summary,
        revision_id: outcome.revision_id,
    })
}

fn action_summary_from(
    id: String,
    outcome_revision_id: String,
    a: project::Action,
) -> ActionSummary {
    ActionSummary {
        id,
        project_id: a.project_id,
        title: a.title,
        body: a.body,
        state: a.state,
        dependency_ids: a.dependency_ids,
        completion_summary: a.completion_summary,
        revision_id: outcome_revision_id,
    }
}

#[tauri::command]
pub fn action_start(
    vault_path: String,
    action_id: String,
    expected_revision_id: String,
) -> Result<ActionSummary, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, updated) = project::start_action(
        &mut writer,
        DESKTOP_ACTOR,
        &action_id,
        &expected_revision_id,
    )
    .map_err(|e| e.to_string())?;
    Ok(action_summary_from(action_id, outcome.revision_id, updated))
}

#[tauri::command]
pub fn action_block(
    vault_path: String,
    action_id: String,
    expected_revision_id: String,
    reason: Option<String>,
) -> Result<ActionSummary, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, updated) = project::block_action(
        &mut writer,
        DESKTOP_ACTOR,
        &action_id,
        &expected_revision_id,
        reason.as_deref(),
    )
    .map_err(|e| e.to_string())?;
    Ok(action_summary_from(action_id, outcome.revision_id, updated))
}

#[tauri::command]
pub fn action_cancel(
    vault_path: String,
    action_id: String,
    expected_revision_id: String,
    reason: Option<String>,
) -> Result<ActionSummary, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, updated) = project::cancel_action(
        &mut writer,
        DESKTOP_ACTOR,
        &action_id,
        &expected_revision_id,
        reason.as_deref(),
    )
    .map_err(|e| e.to_string())?;
    Ok(action_summary_from(action_id, outcome.revision_id, updated))
}

#[tauri::command]
pub fn action_reopen(
    vault_path: String,
    action_id: String,
    expected_revision_id: String,
    reason: String,
) -> Result<ActionSummary, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, updated) = project::reopen_action(
        &mut writer,
        DESKTOP_ACTOR,
        &action_id,
        &expected_revision_id,
        &reason,
    )
    .map_err(|e| e.to_string())?;
    Ok(action_summary_from(action_id, outcome.revision_id, updated))
}

#[tauri::command]
pub fn action_complete(
    vault_path: String,
    action_id: String,
    expected_revision_id: String,
    summary: String,
    override_reason: Option<String>,
) -> Result<ActionSummary, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, updated) = project::complete_action(
        &mut writer,
        DESKTOP_ACTOR,
        &action_id,
        &expected_revision_id,
        &summary,
        override_reason.as_deref(),
    )
    .map_err(|e| e.to_string())?;
    Ok(action_summary_from(action_id, outcome.revision_id, updated))
}

fn decision_summary_from(
    id: String,
    outcome_revision_id: String,
    d: project::Decision,
) -> DecisionSummary {
    DecisionSummary {
        id,
        project_id: d.project_id,
        decision_key: d.decision_key,
        statement: d.statement,
        rationale: d.rationale,
        basis: d.basis,
        verification: d.verification,
        lifecycle: d.lifecycle,
        valid_from: d.valid_from,
        valid_to: d.valid_to,
        revision_id: outcome_revision_id,
    }
}

#[tauri::command]
pub fn list_decisions(
    vault_path: String,
    project_id: String,
) -> Result<Vec<DecisionSummary>, String> {
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let objects = store.list_current_objects().map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for (object_id, revision_id, payload) in objects {
        let record = match RecordPayload::from_json(&payload) {
            Ok(r) => r,
            Err(_) => continue,
        };
        if let RecordPayload::Decision(d) = record {
            if d.project_id == project_id {
                out.push(decision_summary_from(object_id, revision_id, d));
            }
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

/// Bundled to keep `decision_create` under clippy's argument-count limit --
/// no new semantics, just a grouping of this Core function's own optional
/// valid-time/rationale parameters.
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionCreateExtra {
    pub rationale: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
}

#[tauri::command]
pub fn decision_create(
    vault_path: String,
    project_id: String,
    key: String,
    statement: String,
    basis: DecisionBasis,
    verification: DecisionVerification,
    extra: DecisionCreateExtra,
) -> Result<DecisionSummary, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, created) = project::create_decision(
        &mut writer,
        DESKTOP_ACTOR,
        &project_id,
        &key,
        &statement,
        extra.rationale.as_deref(),
        basis,
        verification,
        extra.valid_from.as_deref(),
        extra.valid_to.as_deref(),
    )
    .map_err(|e| e.to_string())?;
    Ok(decision_summary_from(
        outcome.object_id,
        outcome.revision_id,
        created,
    ))
}

#[tauri::command]
pub fn decision_accept(
    vault_path: String,
    decision_id: String,
    expected_revision_id: String,
) -> Result<DecisionSummary, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, updated) = project::accept_decision(
        &mut writer,
        DESKTOP_ACTOR,
        &decision_id,
        &expected_revision_id,
    )
    .map_err(|e| e.to_string())?;
    Ok(decision_summary_from(
        decision_id,
        outcome.revision_id,
        updated,
    ))
}

#[tauri::command]
pub fn decision_withdraw(
    vault_path: String,
    decision_id: String,
    expected_revision_id: String,
    reason: String,
) -> Result<DecisionSummary, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, updated) = project::withdraw_decision(
        &mut writer,
        DESKTOP_ACTOR,
        &decision_id,
        &expected_revision_id,
        &reason,
    )
    .map_err(|e| e.to_string())?;
    Ok(decision_summary_from(
        decision_id,
        outcome.revision_id,
        updated,
    ))
}

#[tauri::command]
pub fn decision_supersede(
    vault_path: String,
    new_decision_id: String,
    old_decision_id: String,
    expected_old_revision_id: String,
    reason: String,
) -> Result<SupersedeResult, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (relation_outcome, update_outcome, old_decision) = project::supersede_decision(
        &mut store,
        DESKTOP_ACTOR,
        &new_decision_id,
        &old_decision_id,
        &expected_old_revision_id,
        &reason,
    )
    .map_err(|e| e.to_string())?;
    Ok(SupersedeResult {
        relation_id: relation_outcome.object_id,
        old_decision: decision_summary_from(
            old_decision_id,
            update_outcome.revision_id,
            old_decision,
        ),
    })
}

#[tauri::command]
pub fn relation_create(
    vault_path: String,
    project_id: String,
    relation_type: RelationType,
    from: String,
    to: String,
    note: Option<String>,
) -> Result<RelationEntry, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, created) = relation::create_relation(
        &mut writer,
        DESKTOP_ACTOR,
        &project_id,
        relation_type,
        &from,
        &to,
        note.as_deref(),
    )
    .map_err(|e| e.to_string())?;
    Ok(RelationEntry {
        id: outcome.object_id,
        project_id: created.project_id,
        relation_type: created.relation_type,
        from_object_id: created.from_object_id,
        to_object_id: created.to_object_id,
        note: created.note,
    })
}

#[tauri::command]
pub fn list_relations_for_object(
    vault_path: String,
    object_id: String,
) -> Result<Vec<RelationEntry>, String> {
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let relations =
        relation::list_relations_for_object(&store, &object_id).map_err(|e| e.to_string())?;
    let mut out: Vec<RelationEntry> = relations
        .into_iter()
        .map(|(id, r)| RelationEntry {
            id,
            project_id: r.project_id,
            relation_type: r.relation_type,
            from_object_id: r.from_object_id,
            to_object_id: r.to_object_id,
            note: r.note,
        })
        .collect();
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

#[tauri::command]
pub fn resume_view(vault_path: String, project_id: String) -> Result<ResumeView, String> {
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    fehrest::resume::resume(&store, &project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_project(
    vault_path: String,
    project_id: String,
    query: String,
) -> Result<Vec<SearchHit>, String> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    let needle = trimmed.to_lowercase();
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let records = project::list_project_records(&store, &project_id).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for (id, record) in records {
        match record {
            RecordPayload::Note(n) => {
                let title_hit = n
                    .title
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&needle);
                let body_hit = n.body.to_lowercase().contains(&needle);
                if title_hit || body_hit {
                    let snippet = snippet_around(&n.body, &needle, 40);
                    out.push(SearchHit {
                        kind: "note".to_string(),
                        id,
                        title: n.title,
                        snippet,
                    });
                }
            }
            RecordPayload::Action(a) => {
                let hit = a.title.to_lowercase().contains(&needle)
                    || a.body
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&needle);
                if hit {
                    let snippet =
                        snippet_around(a.body.as_deref().unwrap_or(&a.title), &needle, 40);
                    out.push(SearchHit {
                        kind: "action".to_string(),
                        id,
                        title: Some(a.title),
                        snippet,
                    });
                }
            }
            RecordPayload::Decision(d) => {
                let hit = d.statement.to_lowercase().contains(&needle)
                    || d.rationale
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&needle);
                if hit {
                    let snippet = snippet_around(&d.statement, &needle, 40);
                    out.push(SearchHit {
                        kind: "decision".to_string(),
                        id,
                        title: Some(d.decision_key),
                        snippet,
                    });
                }
            }
            _ => {}
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

// --- T04-04: resumption/checkpoint/source-status/package/proposal review ---
// Every type below wraps an already-`Serialize` Core type with `#[serde(flatten)]`
// plus its object ID -- no new field, no re-derived semantics, no
// duplicated authority. `ResumeView` itself (used by `resume_view` above)
// needed no such wrapper since its own object IS the view, not a stored
// record with an ID of its own.

#[derive(Debug, Clone, serde::Serialize)]
pub struct CheckpointInfo {
    pub id: String,
    #[serde(flatten)]
    pub checkpoint: ReviewCheckpoint,
    pub revision_id: String,
}

#[tauri::command]
pub fn checkpoint_current(
    vault_path: String,
    project_id: String,
) -> Result<Option<CheckpointInfo>, String> {
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let found = checkpoint::current_checkpoint(&store, &project_id).map_err(|e| e.to_string())?;
    Ok(found.map(|(id, revision_id, checkpoint)| CheckpointInfo {
        id,
        checkpoint,
        revision_id,
    }))
}

#[tauri::command]
pub fn checkpoint_mark(
    vault_path: String,
    project_id: String,
    expected_revision_id: Option<String>,
    through: Option<i64>,
) -> Result<CheckpointInfo, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (outcome, checkpoint) = checkpoint::mark_reviewed_through(
        &mut store,
        DESKTOP_ACTOR,
        &project_id,
        expected_revision_id.as_deref(),
        through,
    )
    .map_err(|e| e.to_string())?;
    Ok(CheckpointInfo {
        id: outcome.object_id,
        checkpoint,
        revision_id: outcome.revision_id,
    })
}

#[tauri::command]
pub fn checkpoint_reset(
    vault_path: String,
    project_id: String,
    expected_revision_id: String,
    through: i64,
    reason: String,
) -> Result<CheckpointInfo, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (outcome, checkpoint) = checkpoint::reset_checkpoint(
        &mut store,
        DESKTOP_ACTOR,
        &project_id,
        &expected_revision_id,
        through,
        &reason,
    )
    .map_err(|e| e.to_string())?;
    Ok(CheckpointInfo {
        id: outcome.object_id,
        checkpoint,
        revision_id: outcome.revision_id,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SourceEntry {
    pub id: String,
    pub label: String,
    pub active: bool,
    /// The most recently recorded check for this source, if any has ever
    /// been run -- `None` is `T02-01`'s own documented "Unchecked" derived
    /// read-path label (zero rows in history), not a stored variant.
    pub latest_check_status: Option<CheckStatus>,
    pub latest_check_at: Option<String>,
}

#[tauri::command]
pub fn list_sources(vault_path: String, project_id: String) -> Result<Vec<SourceEntry>, String> {
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let sources = capture::list_project_sources(&store, &project_id).map_err(|e| e.to_string())?;
    // Every `SourceCheck` is its own append-only object (created fresh by
    // `check_source`, never updated) -- unlike `Note`/`Action`, there is no
    // single "current revision" per source to read, so this scans every
    // `SourceCheck` object and keeps the *most recently created* one (by
    // its own UUIDv7 object ID, which is sub-second time-ordered) per
    // `source_id`. `observed_at`'s own RFC 3339 UTC text truncates to
    // whole seconds, so it cannot break ties between two checks recorded
    // in the same second -- object ID can. This is the display-only
    // counterpart of `resume.rs`'s own stale-evidence scan, generalized to
    // show every source's status, not only the non-`Match` ones
    // `resume()` itself surfaces.
    let mut latest_checks: std::collections::HashMap<String, (String, SourceCheck)> =
        std::collections::HashMap::new();
    for (check_object_id, _, payload) in store.list_current_objects().map_err(|e| e.to_string())? {
        if let Ok(RecordPayload::SourceCheck(c)) = RecordPayload::from_json(&payload) {
            match latest_checks.get(&c.source_id) {
                Some((existing_id, _)) if *existing_id >= check_object_id => {}
                _ => {
                    latest_checks.insert(c.source_id.clone(), (check_object_id, c));
                }
            }
        }
    }
    let mut out = Vec::new();
    for (id, s) in sources {
        let latest = latest_checks.get(&id).map(|(_, c)| c);
        out.push(SourceEntry {
            id,
            label: s.label,
            active: s.active,
            latest_check_status: latest.map(|c| c.status),
            latest_check_at: latest.map(|c| c.observed_at.clone()),
        });
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

#[tauri::command]
pub fn source_check_now(vault_path: String, source_id: String) -> Result<SourceCheck, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (_, check) = source_check::check_source(&mut store, DESKTOP_ACTOR, &source_id)
        .map_err(|e| e.to_string())?;
    Ok(check)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GrantEntry {
    pub id: String,
    #[serde(flatten)]
    pub grant: ExportGrant,
    pub revision_id: String,
}

/// Bundled to keep `grant_issue` under clippy's argument-count limit.
/// Struct-level `#[serde(default)]` (not just `#[derive(Default)]`) is
/// required so a caller passing `{}` -- omitting every field, not just
/// this one -- deserializes via `Default::default()` rather than a
/// missing-field error; `#[derive(Default)]` alone only provides a Rust
/// value to fall back to, it does not by itself tell serde to use it for
/// absent JSON keys.
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GrantIssueOptions {
    pub allowed_object_ids: Option<Vec<String>>,
    pub privacy_exclusions: Vec<String>,
}

#[tauri::command]
pub fn grant_issue(
    vault_path: String,
    project_id: String,
    allowed_kinds: Vec<String>,
    byte_budget: u32,
    ttl_secs: i64,
    options: GrantIssueOptions,
) -> Result<GrantEntry, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut writer = store.writer().map_err(|e| e.to_string())?;
    let (outcome, g) = grant::issue_grant(
        &mut writer,
        DESKTOP_ACTOR,
        &project_id,
        &allowed_kinds,
        options.allowed_object_ids.as_deref(),
        &options.privacy_exclusions,
        byte_budget,
        ttl_secs,
    )
    .map_err(|e| e.to_string())?;
    Ok(GrantEntry {
        id: outcome.object_id,
        grant: g,
        revision_id: outcome.revision_id,
    })
}

#[tauri::command]
pub fn grant_revoke(
    vault_path: String,
    grant_id: String,
    expected_revision_id: String,
) -> Result<GrantEntry, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (outcome, g) =
        grant::revoke_grant(&mut store, DESKTOP_ACTOR, &grant_id, &expected_revision_id)
            .map_err(|e| e.to_string())?;
    Ok(GrantEntry {
        id: outcome.object_id,
        grant: g,
        revision_id: outcome.revision_id,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PackagePreviewResult {
    pub receipt: DisclosureReceipt,
    pub wire: String,
}

#[tauri::command]
pub fn package_preview(
    vault_path: String,
    grant_id: String,
    request_id: String,
    principal: Option<String>,
) -> Result<PackagePreviewResult, String> {
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (receipt, wire) = disclosure::preview_disclosure_package(
        &store,
        &grant_id,
        &request_id,
        principal.as_deref().unwrap_or("agent"),
    )
    .map_err(|e| e.to_string())?;
    Ok(PackagePreviewResult { receipt, wire })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PackageCompileResult {
    pub receipt_id: String,
    pub receipt: DisclosureReceipt,
    pub wire: String,
}

/// Compiles and *persists* the `DisclosureReceipt` (receipt-before-emission,
/// this task's own durability gate) without writing the package to disk --
/// unlike the CLI's `package-export`, this never touches the filesystem,
/// so it needs no native destination dialog (that capability, and the
/// file write itself, is `T04-05`'s own "expose... export safely" scope).
/// The returned `wire` is held only in memory for the owner to copy/export
/// through whatever channel they choose outside this command's own scope.
#[tauri::command]
pub fn package_compile(
    vault_path: String,
    grant_id: String,
    request_id: String,
    principal: Option<String>,
) -> Result<PackageCompileResult, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (receipt_id, receipt, wire) = disclosure::compile_disclosure_package(
        &mut store,
        &grant_id,
        &request_id,
        principal.as_deref().unwrap_or("agent"),
    )
    .map_err(|e| e.to_string())?;
    Ok(PackageCompileResult {
        receipt_id,
        receipt,
        wire,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProposalEntry {
    pub id: String,
    #[serde(flatten)]
    pub proposal: AgentProposal,
    pub revision_id: String,
}

#[tauri::command]
pub fn list_proposals(
    vault_path: String,
    project_id: String,
) -> Result<Vec<ProposalEntry>, String> {
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for (object_id, revision_id, payload) in
        store.list_current_objects().map_err(|e| e.to_string())?
    {
        if let Ok(RecordPayload::AgentProposal(p)) = RecordPayload::from_json(&payload) {
            if p.project_id == project_id {
                out.push(ProposalEntry {
                    id: object_id,
                    proposal: p,
                    revision_id,
                });
            }
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

#[tauri::command]
pub fn proposal_admit(
    vault_path: String,
    project_id: String,
    raw_text: String,
) -> Result<ProposalEntry, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (outcome, proposal) =
        proposal::admit_proposal(&mut store, DESKTOP_ACTOR, &project_id, raw_text.as_bytes())
            .map_err(|e| e.to_string())?;
    Ok(ProposalEntry {
        id: outcome.object_id,
        proposal,
        revision_id: outcome.revision_id,
    })
}

#[tauri::command]
pub fn proposal_accept(
    vault_path: String,
    proposal_id: String,
    expected_revision_id: String,
    selected_indices: Vec<usize>,
) -> Result<ProposalEntry, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (outcome, proposal) = proposal::accept_proposal(
        &mut store,
        DESKTOP_ACTOR,
        &proposal_id,
        &expected_revision_id,
        &selected_indices,
    )
    .map_err(|e| e.to_string())?;
    Ok(ProposalEntry {
        id: outcome.object_id,
        proposal,
        revision_id: outcome.revision_id,
    })
}

#[tauri::command]
pub fn proposal_reject(
    vault_path: String,
    proposal_id: String,
    expected_revision_id: String,
    reason: String,
) -> Result<ProposalEntry, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    let (outcome, proposal) = proposal::reject_proposal(
        &mut store,
        DESKTOP_ACTOR,
        &proposal_id,
        &expected_revision_id,
        &reason,
    )
    .map_err(|e| e.to_string())?;
    Ok(ProposalEntry {
        id: outcome.object_id,
        proposal,
        revision_id: outcome.revision_id,
    })
}

// --- T04-05: backup, recovery, import and export, exposed safely ---
//
// Every destination/source argument here still originates only from the
// owner's own native folder picker (`pick_directory`, T04-01) -- this
// task reuses that one native dialog for every new destination/source
// selection rather than adding a new native dialog capability, matching
// this task's own "native destination dialog bindings" requirement with
// the capability already admitted and audited, not a new one.
//
// `backup_to_new_root` is the only one of these five Core operations that
// accepts a cancellation callback -- `restore_from_backup`,
// `recover_to_new_root`, `export_to_new_root` and `import_full_restore`/
// `import_selected_merge` are synchronous, all-or-nothing Core operations
// with no such hook. This is not a bridge-level limitation papered over:
// it is exactly what Core exposes, so only `vault_backup` below runs on a
// blocking task with a real, checked-by-the-copy-loop cancellation flag;
// the others are plain commands (still off the UI thread, since every
// Tauri command already runs off it) that either complete or fail, never
// "cancel mid-write."

#[derive(Default)]
pub struct CancellationRegistry(Mutex<std::collections::HashMap<String, Arc<AtomicBool>>>);

impl CancellationRegistry {
    fn register(&self, operation_id: &str) -> Arc<AtomicBool> {
        let flag = Arc::new(AtomicBool::new(false));
        self.0
            .lock()
            .expect("cancellation registry mutex poisoned")
            .insert(operation_id.to_string(), flag.clone());
        flag
    }
    fn cancel(&self, operation_id: &str) -> bool {
        match self
            .0
            .lock()
            .expect("cancellation registry mutex poisoned")
            .get(operation_id)
        {
            Some(flag) => {
                flag.store(true, Ordering::SeqCst);
                true
            }
            None => false,
        }
    }
    fn unregister(&self, operation_id: &str) {
        self.0
            .lock()
            .expect("cancellation registry mutex poisoned")
            .remove(operation_id);
    }
}

/// Signal cancellation for an in-flight `vault_backup` call by the same
/// `operation_id` the caller passed to it. Returns `false` if no such
/// operation is currently registered (already finished, or never
/// started) -- never an error, since "there was nothing to cancel" is not
/// a failure.
#[tauri::command]
pub fn cancel_operation(
    registry: tauri::State<'_, CancellationRegistry>,
    operation_id: String,
) -> bool {
    registry.cancel(&operation_id)
}

#[tauri::command]
pub async fn vault_backup(
    registry: tauri::State<'_, CancellationRegistry>,
    vault_path: String,
    dest_parent_dir: String,
    dest_name: String,
    operation_id: String,
) -> Result<BackupReport, String> {
    let dest = join_child(&dest_parent_dir, &dest_name)?;
    let flag = registry.register(&operation_id);
    let flag_for_closure = flag.clone();
    let source = PathBuf::from(vault_path);
    let result = tauri::async_runtime::spawn_blocking(move || {
        backup::backup_to_new_root(&source, &dest, move || {
            flag_for_closure.load(Ordering::SeqCst)
        })
    })
    .await
    .map_err(|e| e.to_string())
    .and_then(|r| r.map_err(|e| e.to_string()));
    registry.unregister(&operation_id);
    result
}

#[tauri::command]
pub fn vault_restore_from_backup(
    backup_path: String,
    dest_parent_dir: String,
    dest_name: String,
) -> Result<RestoreReport, String> {
    let dest = join_child(&dest_parent_dir, &dest_name)?;
    backup::restore_from_backup(Path::new(&backup_path), &dest).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn vault_recover(
    original_path: String,
    dest_parent_dir: String,
    dest_name: String,
) -> Result<RecoveryReport, String> {
    let dest = join_child(&dest_parent_dir, &dest_name)?;
    recovery::recover_to_new_root(Path::new(&original_path), &dest).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_preview(
    vault_path: String,
    project_id: Option<String>,
) -> Result<ExportPreview, String> {
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    export::preview_export(&store, project_id.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn vault_export(
    vault_path: String,
    project_id: Option<String>,
    dest_parent_dir: String,
    dest_name: String,
) -> Result<ExportReport, String> {
    let dest = join_child(&dest_parent_dir, &dest_name)?;
    let store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    export::export_to_new_root(&store, project_id.as_deref(), &dest).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_preview(source_path: String) -> Result<ImportPreview, String> {
    import::preview_import(Path::new(&source_path)).map_err(|e| e.to_string())
}

/// Merge a portable package's selected/exported scope into an **existing**
/// vault -- distinct from `vault_restore` (T04-01, `import_full_restore`
/// into a brand-new empty vault). Never a "silent merge": the owner must
/// have already reviewed `import_preview`'s own conflict list before
/// choosing to call this.
#[tauri::command]
pub fn vault_import_selected(
    vault_path: String,
    source_path: String,
) -> Result<ImportReport, String> {
    let mut store = CanonicalStore::open(&vault_path).map_err(|e| e.to_string())?;
    import::import_selected_merge(&mut store, Path::new(&source_path)).map_err(|e| e.to_string())
}

/// `T05-04`: the desktop half of "About/help/distribution include license,
/// source, privacy and support/reporting route" -- reads the same
/// compile-time `fehrest::about::AboutInfo` the CLI's own `license` command
/// prints, so the two surfaces can never state diverging facts. No vault,
/// no filesystem, no network access.
#[tauri::command]
pub fn about_info() -> fehrest::about::AboutInfo {
    fehrest::about::about_info()
}
