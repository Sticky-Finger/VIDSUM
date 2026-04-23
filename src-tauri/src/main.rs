// Prevent the unused library warning
#![allow(clippy::extra_unused_lifetimes)]

mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![commands::file::select_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
