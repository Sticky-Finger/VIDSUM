//! 文件选择相关命令

use serde::{Deserialize, Serialize};
use tauri_plugin_dialog::DialogExt;

/// 文件类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    Media,
    Subtitle,
}

/// 选择文件命令
///
/// # Arguments
/// * `file_type` - 文件类型 (media 或 subtitle)
///
/// # Returns
/// 返回选中的文件路径，如果用户取消则返回空字符串
#[tauri::command]
pub async fn select_file(
    app: tauri::AppHandle,
    file_type: FileType,
) -> Result<String, String> {
    // 使用 tauri-plugin-dialog 打开文件选择对话框
    let mut dialog_builder = app
        .dialog()
        .file()
        .set_title("选择文件");

    // 根据文件类型添加过滤器
    match file_type {
        FileType::Media => {
            dialog_builder = dialog_builder.add_filter(
                "音视频文件",
                &[
                    "mp4", "mkv", "mov", "avi", "webm", "flv",
                    "mp3", "wav", "m4a", "aac", "ogg", "flac",
                ],
            );
        }
        FileType::Subtitle => {
            dialog_builder = dialog_builder.add_filter(
                "字幕文件",
                &["srt", "vtt", "ass", "ssa"],
            );
        }
    }

    // 使用 blocking_pick_file 在命令中打开文件对话框
    let file_path = dialog_builder.blocking_pick_file();

    match file_path {
        Some(path) => Ok(path.to_string()),
        None => Ok(String::new()), // 用户取消选择
    }
}
