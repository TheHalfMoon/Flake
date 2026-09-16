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
use fehrest::project::{self, RecordPayload};

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
