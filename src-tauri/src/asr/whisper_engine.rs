//! Whisper ASR 引擎实现
//!
//! 使用 whisper-rs 进行本地音频转写

use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

/// Whisper 模型类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhisperModel {
    /// Tiny 模型 - 最小最快，精度最低
    Tiny,
    /// Base 模型 - 平衡性能和精度
    Base,
}

impl WhisperModel {
    /// 获取模型文件名
    pub fn filename(&self) -> &'static str {
        match self {
            WhisperModel::Tiny => "ggml-tiny.bin",
            WhisperModel::Base => "ggml-base.bin",
        }
    }

    /// 从文件名字符串解析模型类型
    pub fn from_filename(name: &str) -> Option<Self> {
        match name {
            "ggml-tiny.bin" | "tiny" => Some(WhisperModel::Tiny),
            "ggml-base.bin" | "base" => Some(WhisperModel::Base),
            _ => None,
        }
    }
}

/// 转写进度信息
#[derive(Debug, Clone)]
pub struct TranscriptionProgress {
    /// 当前处理的段落索引
    pub segment_index: i32,
    /// 总段落数估计
    pub total_segments: i32,
    /// 当前转写的文本
    pub text: String,
    /// 开始时间 (毫秒)
    pub start_time: i64,
    /// 结束时间 (毫秒)
    pub end_time: i64,
}

/// 转写结果
#[derive(Debug, Clone)]
pub struct TranscriptionResult {
    /// 完整转写文本
    pub full_text: String,
    /// 带时间戳的段落
    pub segments: Vec<TranscriptionSegment>,
}

/// 转写段落
#[derive(Debug, Clone)]
pub struct TranscriptionSegment {
    /// 段落文本
    pub text: String,
    /// 开始时间 (毫秒)
    pub start_time: i64,
    /// 结束时间 (毫秒)
    pub end_time: i64,
    /// 格式化后的时间戳 [HH:MM:SS]
    pub timestamp: String,
}

/// Whisper 引擎错误类型
#[derive(Error, Debug)]
pub enum WhisperError {
    #[error("模型加载失败：{0}")]
    ModelLoadError(String),
    #[error("音频加载失败：{0}")]
    AudioLoadError(String),
    #[error("转写失败：{0}")]
    TranscriptionError(String),
    #[error("模型文件不存在：{0}")]
    ModelNotFound(PathBuf),
}

/// Whisper ASR 引擎
pub struct WhisperEngine {
    /// 模型上下文
    ctx: Arc<WhisperContext>,
    /// 模型类型
    model: WhisperModel,
}

impl WhisperEngine {
    /// 创建新的 Whisper 引擎实例
    ///
    /// # 参数
    /// * `model` - 模型类型
    /// * `model_dir` - 模型文件目录
    ///
    /// # 返回
    /// 成功返回引擎实例，失败返回错误
    pub fn new(model: WhisperModel, model_dir: PathBuf) -> Result<Self, WhisperError> {
        let model_path = model_dir.join(model.filename());

        if !model_path.exists() {
            return Err(WhisperError::ModelNotFound(model_path));
        }

        let ctx = WhisperContext::new_with_params(
            model_path.to_str().unwrap(),
            whisper_rs::WhisperContextParameters::default(),
        )
        .map_err(|e| WhisperError::ModelLoadError(e.to_string()))?;

        Ok(Self {
            ctx: Arc::new(ctx),
            model,
        })
    }

    /// 获取模型类型
    pub fn model(&self) -> WhisperModel {
        self.model
    }

    /// 执行转写
    ///
    /// # 参数
    /// * `audio_path` - 音频文件路径
    /// * `progress_callback` - 进度回调函数
    ///
    /// # 返回
    /// 转写结果
    pub fn transcribe<F>(
        &self,
        audio_path: PathBuf,
        mut progress_callback: F,
    ) -> Result<TranscriptionResult, WhisperError>
    where
        F: FnMut(TranscriptionProgress),
    {
        // 加载音频文件
        let audio_data = Self::load_audio(&audio_path)?;

        // 创建状态
        let mut state = self
            .ctx
            .create_state()
            .map_err(|e| WhisperError::TranscriptionError(e.to_string()))?;

        // 设置参数
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_print_special(false);
        params.set_print_progress(true);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_suppress_blank(true);
        params.set_suppress_nst(true);
        params.set_token_timestamps(true);
        params.set_single_segment(false);
        params.set_max_initial_ts(10000.0);

        // 执行转写
        state
            .full(params, &audio_data)
            .map_err(|e| WhisperError::TranscriptionError(e.to_string()))?;

        // 收集结果
        let num_segments = state.full_n_segments();
        let mut segments = Vec::new();
        let mut full_text = String::new();

        for i in 0..num_segments {
            let segment = state
                .get_segment(i)
                .ok_or_else(|| WhisperError::TranscriptionError(format!("Segment {} not found", i)))?;

            let text = segment
                .to_str_lossy()
                .map_err(|e| WhisperError::TranscriptionError(e.to_string()))?
                .to_string();

            let start_time = segment.start_timestamp();
            let end_time = segment.end_timestamp();

            let timestamp = Self::format_timestamp(start_time); // 时间单位已是厘秒

            // 发送进度回调
            progress_callback(TranscriptionProgress {
                segment_index: i,
                total_segments: num_segments,
                text: text.clone(),
                start_time: start_time * 10, // 转换为毫秒
                end_time: end_time * 10,
            });

            segments.push(TranscriptionSegment {
                text: text.clone(),
                start_time: start_time * 10,
                end_time: end_time * 10,
                timestamp: timestamp.clone(),
            });

            full_text.push_str(&format!("[{}] {}\n", timestamp, text));
        }

        Ok(TranscriptionResult {
            full_text,
            segments,
        })
    }

    /// 加载音频文件并转换为 Whisper 需要的格式
    ///
    /// Whisper 需要 16kHz 单声道 16 位 PCM 数据
    fn load_audio(path: &PathBuf) -> Result<Vec<f32>, WhisperError> {
        // 使用 hound 读取 WAV 文件
        let reader = hound::WavReader::open(path).map_err(|e| {
            WhisperError::AudioLoadError(format!("无法打开音频文件：{}", e))
        })?;

        let samples: Vec<i16> = reader
            .into_samples::<i16>()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| WhisperError::AudioLoadError(format!("读取音频采样失败：{}", e)))?;

        // 转换为 f32 并归一化到 [-1.0, 1.0]
        let audio_data: Vec<f32> = samples
            .into_iter()
            .map(|s| s as f32 / i16::MAX as f32)
            .collect();

        // TODO: 如果采样率不是 16kHz，需要进行重采样
        // 目前假设输入是 16kHz

        Ok(audio_data)
    }

    /// 格式化时间戳为 [HH:MM:SS] 格式
    fn format_timestamp(time_ms: i64) -> String {
        let total_seconds = time_ms / 1000;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_filename() {
        assert_eq!(WhisperModel::Tiny.filename(), "ggml-tiny.bin");
        assert_eq!(WhisperModel::Base.filename(), "ggml-base.bin");
    }

    #[test]
    fn test_model_from_filename() {
        assert_eq!(
            WhisperModel::from_filename("ggml-tiny.bin"),
            Some(WhisperModel::Tiny)
        );
        assert_eq!(
            WhisperModel::from_filename("ggml-base.bin"),
            Some(WhisperModel::Base)
        );
        assert_eq!(WhisperModel::from_filename("unknown.bin"), None);
    }

    #[test]
    fn test_format_timestamp() {
        assert_eq!(WhisperEngine::format_timestamp(0), "00:00:00");
        assert_eq!(WhisperEngine::format_timestamp(1000), "00:00:01");
        assert_eq!(WhisperEngine::format_timestamp(60000), "00:01:00");
        assert_eq!(WhisperEngine::format_timestamp(3600000), "01:00:00");
        assert_eq!(WhisperEngine::format_timestamp(3661000), "01:01:01");
    }
}
