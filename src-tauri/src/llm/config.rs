//! LLM API 配置管理
//!
//! 提供 OpenAI 兼容 API 的配置结构体和 JSON 文件持久化

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// LLM API 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// API Base URL（如 https://api.openai.com/v1）
    pub base_url: String,
    /// API Key
    pub api_key: String,
    /// 模型 ID（如 gpt-4o-mini）
    pub model: String,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: String::new(),
            model: "gpt-4o-mini".to_string(),
        }
    }
}

impl LlmConfig {
    /// 获取 Chat Completions 端点完整 URL
    pub fn chat_completions_url(&self) -> String {
        let base = self.base_url.trim_end_matches('/');
        format!("{}/chat/completions", base)
    }

    /// 验证配置是否完整
    pub fn validate(&self) -> Result<(), String> {
        if self.base_url.trim().is_empty() {
            return Err("Base URL 不能为空".to_string());
        }
        if self.api_key.trim().is_empty() {
            return Err("API Key 不能为空".to_string());
        }
        if self.model.trim().is_empty() {
            return Err("Model ID 不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取配置文件路径
///
/// 使用 Tauri app data 目录: {app_data_dir}/llm_config.json
fn config_file_path(app_data_dir: &PathBuf) -> PathBuf {
    app_data_dir.join("llm_config.json")
}

/// 从 JSON 文件加载配置
///
/// 如果文件不存在或解析失败，返回默认配置
pub fn load_config(app_data_dir: &PathBuf) -> LlmConfig {
    let path = config_file_path(app_data_dir);
    if !path.exists() {
        return LlmConfig::default();
    }

    match std::fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str::<LlmConfig>(&content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("[llm_config] 解析配置文件失败，使用默认配置: {}", e);
                LlmConfig::default()
            }
        },
        Err(e) => {
            eprintln!("[llm_config] 读取配置文件失败，使用默认配置: {}", e);
            LlmConfig::default()
        }
    }
}

/// 保存配置到 JSON 文件
///
/// 自动创建父目录（如果不存在）
pub fn save_config(app_data_dir: &PathBuf, config: &LlmConfig) -> Result<(), String> {
    let path = config_file_path(app_data_dir);

    // 确保目录存在
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建配置目录失败：{}", e))?;
    }

    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("序列化配置失败：{}", e))?;

    std::fs::write(&path, json)
        .map_err(|e| format!("写入配置文件失败：{}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LlmConfig::default();
        assert_eq!(config.base_url, "https://api.openai.com/v1");
        assert_eq!(config.model, "gpt-4o-mini");
        assert!(config.api_key.is_empty());
    }

    #[test]
    fn test_chat_completions_url() {
        let config = LlmConfig {
            base_url: "https://api.openai.com/v1/".to_string(),
            api_key: "sk-test".to_string(),
            model: "gpt-4o".to_string(),
        };
        assert_eq!(
            config.chat_completions_url(),
            "https://api.openai.com/v1/chat/completions"
        );
    }

    #[test]
    fn test_chat_completions_url_no_trailing_slash() {
        let config = LlmConfig {
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: "sk-test".to_string(),
            model: "gpt-4o".to_string(),
        };
        assert_eq!(
            config.chat_completions_url(),
            "https://api.openai.com/v1/chat/completions"
        );
    }

    #[test]
    fn test_validate_empty_api_key() {
        let config = LlmConfig::default();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_success() {
        let config = LlmConfig {
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: "sk-test".to_string(),
            model: "gpt-4o".to_string(),
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_base_url() {
        let config = LlmConfig {
            base_url: "  ".to_string(),
            api_key: "sk-test".to_string(),
            model: "gpt-4o".to_string(),
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_load_nonexistent_file() {
        let dir = PathBuf::from("/tmp/vidsum_test_nonexistent");
        let config = load_config(&dir);
        // 应返回默认配置
        assert_eq!(config.model, "gpt-4o-mini");
    }

    #[test]
    fn test_save_and_load_config() {
        let dir = PathBuf::from("/tmp/vidsum_test_config");
        let _ = std::fs::remove_dir_all(&dir);

        let config = LlmConfig {
            base_url: "https://custom.api.com/v1".to_string(),
            api_key: "sk-custom-key".to_string(),
            model: "custom-model".to_string(),
        };

        save_config(&dir, &config).unwrap();
        let loaded = load_config(&dir);

        assert_eq!(loaded.base_url, "https://custom.api.com/v1");
        assert_eq!(loaded.api_key, "sk-custom-key");
        assert_eq!(loaded.model, "custom-model");

        // 清理
        let _ = std::fs::remove_dir_all(&dir);
    }
}
