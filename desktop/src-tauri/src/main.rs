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
        ])
        .run(tauri::generate_context!())
        .expect("error while running the Flake desktop shell");
}
