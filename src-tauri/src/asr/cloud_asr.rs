//! 云端 ASR 客户端
//!
//! 提供 OpenAI Whisper API 兼容接口

use reqwest::multipart;
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

/// 云端 ASR 错误类型
#[derive(Error, Debug)]
pub enum CloudAsrError {
    #[error("HTTP 请求失败：{0}")]
    HttpError(#[from] reqwest::Error),
    #[error("API 响应错误：{0}")]
    ApiError(String),
    #[error("无效的响应格式：{0}")]
    InvalidResponse(String),
    #[error("配置错误：{0}")]
    ConfigError(String),
}

/// 转写结果
#[derive(Debug, Clone)]
pub struct TranscriptionResult {
    /// 转写文本
    pub text: String,
    /// 语言检测
    pub language: Option<String>,
}

/// 云端 ASR 客户端
pub struct CloudAsrClient {
    config: CloudAsrConfig,
    http_client: reqwest::Client,
}

impl CloudAsrClient {
    /// 创建新的云端 ASR 客户端
    pub fn new(config: CloudAsrConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    /// 创建带自定义 HTTP 客户端的云端 ASR 客户端
    pub fn with_client(config: CloudAsrConfig, http_client: reqwest::Client) -> Self {
        Self {
            config,
            http_client,
        }
    }

    /// 获取当前配置
    pub fn config(&self) -> &CloudAsrConfig {
        &self.config
    }

    /// 验证配置是否完整
    pub fn validate(&self) -> Result<(), CloudAsrError> {
        if self.config.api_key.is_empty() {
            return Err(CloudAsrError::ConfigError("API Key 不能为空".to_string()));
        }
        if self.config.api_url.is_empty() {
            return Err(CloudAsrError::ConfigError("API URL 不能为空".to_string()));
        }
        Ok(())
    }

    /// 转写音频文件
    ///
    /// # 参数
    /// - `audio_path`: 音频文件路径
    ///
    /// # 返回
    /// 返回转写结果或错误
    pub async fn transcribe(&self, audio_path: &str) -> Result<TranscriptionResult, CloudAsrError> {
        // 验证配置
        self.validate()?;

        // 读取音频文件
        let audio_data = tokio::fs::read(audio_path)
            .await
            .map_err(|e| CloudAsrError::ApiError(format!("读取音频文件失败：{}", e)))?;

        // 构建 multipart 表单
        let form = multipart::Form::new()
            .text("model", self.config.model.clone())
            .part(
                "file",
                multipart::Part::bytes(audio_data)
                    .file_name("audio.wav")
                    .mime_str("audio/wav")
                    .map_err(|e| CloudAsrError::InvalidResponse(format!("设置 MIME 失败：{}", e)))?,
            );

        // 发送请求
        let response = self
            .http_client
            .post(&self.config.api_url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .multipart(form)
            .send()
            .await?;

        // 检查响应状态
        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "未知错误".to_string());
            return Err(CloudAsrError::ApiError(format!("API 错误：{}", error_text)));
        }

        // 解析响应
        let response_text = response.text().await?;

        // 尝试解析 JSON 响应 (OpenAI 格式)
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response_text) {
            if let Some(text) = json.get("text").and_then(|v| v.as_str()) {
                let language = json.get("language").and_then(|v| v.as_str()).map(String::from);
                return Ok(TranscriptionResult {
                    text: text.to_string(),
                    language,
                });
            }
        }

        // 如果不是 JSON，直接返回文本
        Ok(TranscriptionResult {
            text: response_text,
            language: None,
        })
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

    #[test]
    fn test_custom_config() {
        let config = CloudAsrConfig {
            api_url: "https://custom.api.com/v1/transcribe".to_string(),
            api_key: "sk-test-key".to_string(),
            model: "custom-whisper".to_string(),
        };
        let client = CloudAsrClient::new(config);
        assert_eq!(client.config().api_key, "sk-test-key");
        assert_eq!(client.config().model, "custom-whisper");
    }

    #[test]
    fn test_validate_missing_api_key() {
        let config = CloudAsrConfig {
            api_url: "https://api.example.com".to_string(),
            api_key: String::new(),
            model: "whisper-1".to_string(),
        };
        let client = CloudAsrClient::new(config);
        assert!(client.validate().is_err());
    }

    #[test]
    fn test_validate_success() {
        let config = CloudAsrConfig {
            api_url: "https://api.example.com".to_string(),
            api_key: "sk-valid-key".to_string(),
            model: "whisper-1".to_string(),
        };
        let client = CloudAsrClient::new(config);
        assert!(client.validate().is_ok());
    }

    #[tokio::test]
    async fn test_transcribe_missing_file() {
        let config = CloudAsrConfig {
            api_url: "https://api.example.com".to_string(),
            api_key: "sk-test-key".to_string(),
            model: "whisper-1".to_string(),
        };
        let client = CloudAsrClient::new(config);

        // 测试不存在的文件
        let result = client.transcribe("/nonexistent/file.wav").await;
        assert!(result.is_err());
    }
}
