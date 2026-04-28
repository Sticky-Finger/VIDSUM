// Prevent the unused library warning
#![allow(clippy::extra_unused_lifetimes)]

mod commands;

use crate::commands::asr::AppState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState {
            whisper_engine: std::sync::Arc::new(std::sync::Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            commands::file::select_file,
            commands::asr::init_whisper_engine,
            commands::asr::start_transcription,
            commands::asr::transcribe_with_cloud,
            commands::llm::load_llm_config,
            commands::llm::save_llm_config,
            commands::llm::test_llm_connection,
            commands::llm::get_default_prompt,
            commands::llm::generate_summary,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
