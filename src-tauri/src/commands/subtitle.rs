//! 字幕处理相关命令

use vidsum_lib::subtitle;

#[tauri::command]
pub fn parse_subtitle_file(file_path: String) -> Result<Vec<subtitle::SubtitleEntry>, String> {
    subtitle::parse_subtitle_file(&file_path)
}
