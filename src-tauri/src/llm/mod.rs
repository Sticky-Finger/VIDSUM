//! LLM (大语言模型) 模块
//!
//! 提供 OpenAI 兼容 API 的配置管理和调用接口

pub mod config;
pub mod client;
pub mod prompt;

pub use config::LlmConfig;
pub use client::LlmClient;
