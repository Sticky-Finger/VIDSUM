//! 云端 ASR 客户端
//!
//! 提供 OpenAI Whisper API 兼容接口
//!
//! TODO: Task 3 实现

use thiserror::Error;

/// 云端 ASR 配置
#[derive(Debug, Clone)]
pub struct CloudAsrConfig {
    /// API 地址
    pub api_url: String,
    /// API Key
    pub api_key: String,
    /// 模型名称
    pub model: String,
}

impl Default for CloudAsrConfig {
    fn default() -> Self {
        Self {
            api_url: "https://api.openai.com/v1/audio/transcriptions".to_string(),
            api_key: String::new(),
            model: "whisper-1".to_string(),
        }
    }
}

/// 云端 ASR 客户端
pub struct CloudAsrClient {
    config: CloudAsrConfig,
}

/// 云端 ASR 错误类型
#[derive(Error, Debug)]
pub enum CloudAsrError {
    #[error("未实现")]
    NotImplemented,
}

impl CloudAsrClient {
    /// 创建新的云端 ASR 客户端
    pub fn new(config: CloudAsrConfig) -> Self {
        Self { config }
    }

    /// 获取当前配置
    pub fn config(&self) -> &CloudAsrConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CloudAsrConfig::default();
        assert_eq!(
            config.api_url,
            "https://api.openai.com/v1/audio/transcriptions"
        );
        assert_eq!(config.model, "whisper-1");
        assert!(config.api_key.is_empty());
    }
}
