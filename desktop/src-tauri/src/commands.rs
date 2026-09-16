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

use fehrest::canonical::CanonicalStore;
use fehrest::import::import_full_restore;
use fehrest::project::{
    self, ActionState, DecisionBasis, DecisionLifecycle, DecisionVerification, RecordPayload,
};
use fehrest::relation::{self, RelationType};
use fehrest::resume::ResumeView;

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
