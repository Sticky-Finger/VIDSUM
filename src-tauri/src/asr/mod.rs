//! ASR (自动语音识别) 模块
//!
//! 提供本地 Whisper 转写和云端 ASR 接口

pub mod whisper_engine;
pub mod cloud_asr;

pub use whisper_engine::{WhisperModel, WhisperEngine, TranscriptionProgress, TranscriptionResult, TranscriptionSegment, WhisperError};
pub use cloud_asr::{CloudAsrConfig, CloudAsrClient, CloudAsrError};
