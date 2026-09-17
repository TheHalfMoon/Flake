// Prevents an additional console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

/// The only native filesystem-path-selection surface in this shell: an
/// owner-mediated native folder picker. This never reads or writes
/// anything itself -- it returns a path string (or `None` if the owner
/// cancelled) that the caller then hands to one of `commands`' own
/// bounded Core-backed commands. This is the "native dialog-mediated
/// selection as a bounded Core capability" the T04-01 task contract
/// requires, and it is the only path-producing surface in this crate.
///
/// `T05-03` (plan section 25: "Default data paths use the OS's per-user
/// application-data directory under Flake/vaults") opens the dialog at
/// that suggested starting location when the OS reports one -- a
/// starting point the owner can freely navigate away from, never a
/// forced or scanned location; `default_vault_parent_dir` below is the
/// read-only counterpart the frontend uses to *display* that same
/// suggestion before the owner has opened the dialog at all.
#[tauri::command]
async fn pick_directory(app: tauri::AppHandle) -> Option<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut builder = app.dialog().file();
    if let Some(dir) = default_vault_parent_dir_path(&app) {
        builder = builder.set_directory(dir);
    }
    builder.pick_folder(move |folder| {
        let _ = tx.send(folder.map(|f| f.to_string()));
    });
    rx.recv().ok().flatten()
}

/// The OS's own per-user application-data directory joined with
/// `vaults` (plan section 25's own exact phrase) -- never created or
/// written to by this function itself; it is purely informational,
/// resolved fresh on every call, and both callers below treat "the OS
/// reports no such directory" as an ordinary `None`, not an error.
fn default_vault_parent_dir_path(app: &tauri::AppHandle) -> Option<std::path::PathBuf> {
    app.path().app_data_dir().ok().map(|dir| dir.join("vaults"))
}

/// Read-only suggestion for the frontend's own "create a new vault"
/// screen to display as a default location *before* the owner opens the
/// native picker -- the owner can always pick a different location via
/// `pick_directory` instead. Returns `None` exactly when the OS-level
/// resolver itself has nothing to report, never a fabricated fallback
/// path.
#[tauri::command]
fn default_vault_parent_dir(app: tauri::AppHandle) -> Option<String> {
    default_vault_parent_dir_path(&app).map(|dir| dir.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(commands::CancellationRegistry::default())
        .invoke_handler(tauri::generate_handler![
            pick_directory,
            default_vault_parent_dir,
            commands::vault_create,
            commands::vault_open,
            commands::vault_restore,
            commands::list_projects,
            commands::create_project,
            commands::project_archive,
            commands::project_unarchive,
            commands::list_notes,
            commands::note_create,
            commands::note_update,
            commands::note_tombstone,
            commands::note_untombstone,
            commands::list_actions,
            commands::action_create,
            commands::action_start,
            commands::action_block,
            commands::action_cancel,
            commands::action_reopen,
            commands::action_complete,
            commands::list_decisions,
            commands::decision_create,
            commands::decision_accept,
            commands::decision_withdraw,
            commands::decision_supersede,
            commands::relation_create,
            commands::list_relations_for_object,
            commands::resume_view,
            commands::search_project,
            commands::checkpoint_current,
            commands::checkpoint_mark,
            commands::checkpoint_reset,
            commands::list_sources,
            commands::source_check_now,
            commands::grant_issue,
            commands::grant_revoke,
            commands::package_preview,
            commands::package_compile,
            commands::list_proposals,
            commands::proposal_admit,
            commands::proposal_accept,
            commands::proposal_reject,
            commands::cancel_operation,
            commands::vault_backup,
            commands::vault_restore_from_backup,
            commands::vault_recover,
            commands::export_preview,
            commands::vault_export,
            commands::import_preview,
            commands::vault_import_selected,
        ])
        .run(tauri::generate_context!())
        .expect("error while running the Flake desktop shell");
}
