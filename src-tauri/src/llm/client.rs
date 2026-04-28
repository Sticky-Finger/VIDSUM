//! OpenAI 兼容 Chat Completions 客户端
//!
//! 支持非流式调用

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::config::LlmConfig;

/// LLM 调用错误
#[derive(Error, Debug)]
pub enum LlmError {
    #[error("HTTP 请求失败：{0}")]
    HttpError(#[from] reqwest::Error),
    #[error("API 响应错误：{0}")]
    ApiError(String),
    #[error("配置错误：{0}")]
    ConfigError(String),
    #[error("响应解析失败：{0}")]
    ParseError(String),
}

/// Chat 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Chat Completions 请求体
#[derive(Debug, Serialize)]
struct ChatCompletionsRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    stream: bool,
}

/// Chat Completions 响应 - 选择
#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessageResponse,
}

/// Chat Completions 响应 - 消息
#[derive(Debug, Deserialize)]
struct ChatMessageResponse {
    content: Option<String>,
}

/// Chat Completions 响应体
#[derive(Debug, Deserialize)]
struct ChatCompletionsResponse {
    choices: Vec<ChatChoice>,
}

/// LLM 调用结果
#[derive(Debug, Clone)]
pub struct LlmResponse {
    /// 生成的文本内容
    pub content: String,
}

/// OpenAI 兼容 LLM 客户端
pub struct LlmClient {
    config: LlmConfig,
    http_client: reqwest::Client,
}

impl LlmClient {
    /// 创建新的 LLM 客户端
    pub fn new(config: LlmConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    /// 获取当前配置
    pub fn config(&self) -> &LlmConfig {
        &self.config
    }

    /// 调用 Chat Completions API
    ///
    /// # 参数
    /// - `messages`: 对话消息列表
    ///
    /// # 返回
    /// 返回 LLM 生成的文本内容
    pub async fn chat_completions(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<LlmResponse, LlmError> {
        // 验证配置
        self.config.validate().map_err(LlmError::ConfigError)?;

        let url = self.config.chat_completions_url();

        let request = ChatCompletionsRequest {
            model: self.config.model.clone(),
            messages,
            temperature: 0.3,
            stream: false,
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.config.api_key))
                .map_err(|e| LlmError::ConfigError(format!("无效的 API Key：{}", e)))?,
        );

        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "未知错误".to_string());
            return Err(LlmError::ApiError(format!(
                "API 错误 ({}): {}",
                status, error_text
            )));
        }

        let response_text = response.text().await?;
        let chat_response: ChatCompletionsResponse =
            serde_json::from_str(&response_text)
                .map_err(|e| LlmError::ParseError(format!("解析响应失败：{}", e)))?;

        let content = chat_response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or_else(|| LlmError::ParseError("响应中没有生成内容".to_string()))?;

        Ok(LlmResponse { content })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = LlmConfig {
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: "sk-test".to_string(),
            model: "gpt-4o".to_string(),
            system_prompt: None,
            user_prompt_template: None,
        };
        let client = LlmClient::new(config);
        assert_eq!(client.config().model, "gpt-4o");
    }

    #[test]
    fn test_chat_completions_url() {
        let config = LlmConfig {
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: "sk-test".to_string(),
            model: "gpt-4o".to_string(),
            system_prompt: None,
            user_prompt_template: None,
        };
        let client = LlmClient::new(config);
        assert_eq!(
            client.config().chat_completions_url(),
            "https://api.openai.com/v1/chat/completions"
        );
    }
}
