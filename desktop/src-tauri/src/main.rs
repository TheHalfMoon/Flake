// Prevents an additional console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use tauri_plugin_dialog::DialogExt;

/// The only native filesystem-path-selection surface in this shell: an
/// owner-mediated native folder picker. This never reads or writes
/// anything itself -- it returns a path string (or `None` if the owner
/// cancelled) that the caller then hands to one of `commands`' own
/// bounded Core-backed commands. This is the "native dialog-mediated
/// selection as a bounded Core capability" the T04-01 task contract
/// requires, and it is the only path-producing surface in this crate.
#[tauri::command]
async fn pick_directory(app: tauri::AppHandle) -> Option<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    app.dialog().file().pick_folder(move |folder| {
        let _ = tx.send(folder.map(|f| f.to_string()));
    });
    rx.recv().ok().flatten()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            pick_directory,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running the Flake desktop shell");
}
