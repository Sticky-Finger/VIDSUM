//! LLM (大语言模型) 相关命令
//!
//! 提供配置管理和 LLM 调用的 Tauri 命令接口

use serde::{Deserialize, Serialize};
use tauri::Manager;

use vidsum_lib::llm::{LlmConfig, LlmClient};
use vidsum_lib::llm::client::ChatMessage;

/// 配置数据（前端 ↔ 后端传递）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfigPayload {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    /// 自定义系统提示（可选）
    pub system_prompt: Option<String>,
    /// 自定义用户提示模板（可选）
    pub user_prompt_template: Option<String>,
}

impl From<LlmConfig> for LlmConfigPayload {
    fn from(config: LlmConfig) -> Self {
        Self {
            base_url: config.base_url,
            api_key: config.api_key,
            model: config.model,
            system_prompt: config.system_prompt,
            user_prompt_template: config.user_prompt_template,
        }
    }
}

impl From<LlmConfigPayload> for LlmConfig {
    fn from(payload: LlmConfigPayload) -> Self {
        Self {
            base_url: payload.base_url,
            api_key: payload.api_key,
            model: payload.model,
            system_prompt: payload.system_prompt,
            user_prompt_template: payload.user_prompt_template,
        }
    }
}

/// 获取 app data 目录路径
fn get_app_data_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|e| format!("获取应用数据目录失败：{}", e))
}

/// 加载 LLM 配置
///
/// 从本地 JSON 文件读取配置，如果文件不存在返回默认值
#[tauri::command]
pub async fn load_llm_config(app: tauri::AppHandle) -> Result<LlmConfigPayload, String> {
    let data_dir = get_app_data_dir(&app)?;
    let config = vidsum_lib::llm::config::load_config(&data_dir);
    Ok(LlmConfigPayload::from(config))
}

/// 保存 LLM 配置
///
/// 将配置写入本地 JSON 文件持久化
#[tauri::command]
pub async fn save_llm_config(
    app: tauri::AppHandle,
    config: LlmConfigPayload,
) -> Result<String, String> {
    let llm_config = LlmConfig::from(config);

    // 验证配置
    llm_config.validate()?;

    let data_dir = get_app_data_dir(&app)?;
    vidsum_lib::llm::config::save_config(&data_dir, &llm_config)?;

    Ok("配置已保存".to_string())
}

/// 测试 LLM API 连接
///
/// 发送一条简单消息验证 API 配置是否正确
#[tauri::command]
pub async fn test_llm_connection(
    config: LlmConfigPayload,
) -> Result<String, String> {
    let llm_config = LlmConfig::from(config);
    llm_config.validate()?;

    let client = LlmClient::new(llm_config);

    let messages = vec![ChatMessage {
        role: "user".to_string(),
        content: "请回复\"连接成功\"两个字。".to_string(),
    }];

    let response = client
        .chat_completions(messages)
        .await
        .map_err(|e| format!("连接测试失败：{}", e))?;

    Ok(response.content)
}

/// 字幕片段数据（接收前端字幕数据）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleSegmentPayload {
    /// 开始时间（秒）
    pub start: f64,
    /// 结束时间（秒）
    pub end: f64,
    /// 文本内容
    pub text: String,
}

/// 格式化字幕片段为带时间戳的文本
fn format_subtitle_segments(segments: &[SubtitleSegmentPayload]) -> String {
    segments
        .iter()
        .map(|seg| {
            let start_ts = format_timestamp(seg.start);
            format!("[{}] {}", start_ts, seg.text)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// 将秒数格式化为 [HH:MM:SS]
fn format_timestamp(seconds: f64) -> String {
    let total = seconds as u64;
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let secs = total % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

/// 获取默认 Prompt
///
/// 返回系统提示和用户提示模板的默认值
#[tauri::command]
pub async fn get_default_prompt() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "system_prompt": vidsum_lib::llm::prompt::get_default_system_prompt(),
        "user_prompt_template": vidsum_lib::llm::prompt::get_default_user_prompt_template()
    }))
}

/// 生成总结
///
/// 使用配置的 LLM 和自定义/默认 Prompt 对字幕进行总结
#[tauri::command]
pub async fn generate_summary(
    config: LlmConfigPayload,
    segments: Vec<SubtitleSegmentPayload>,
    system_prompt: String,
    user_prompt_template: String,
) -> Result<String, String> {
    let llm_config = LlmConfig::from(config);
    llm_config.validate()?;

    // 格式化字幕文本
    let subtitles_text = format_subtitle_segments(&segments);

    // 替换占位符
    let user_content = user_prompt_template.replace("{subtitles}", &subtitles_text);

    // 构建消息列表
    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: system_prompt,
        },
        ChatMessage {
            role: "user".to_string(),
            content: user_content,
        },
    ];

    let client = LlmClient::new(llm_config);

    let response = client
        .chat_completions(messages)
        .await
        .map_err(|e| format!("生成总结失败：{}", e))?;

    Ok(response.content)
}
