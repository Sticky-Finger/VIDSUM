//! Whisper ASR 引擎实现
//!
//! 使用 whisper-rs 进行本地音频转写

use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// Whisper 模型类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhisperModel {
    /// Tiny 模型 (77MB) - 最小最快，仅适合英文
    Tiny,
    /// Base 模型 (147MB) - 多语言基础支持
    Base,
    /// Small 模型 (488MB) - 较好的多语言精度
    Small,
    /// Medium 模型 (1.5GB) - 高精度多语言
    Medium,
    /// Large 模型 (3.1GB) - 最高精度
    Large,
}

impl WhisperModel {
    /// 获取模型文件名
    pub fn filename(&self) -> &'static str {
        match self {
            WhisperModel::Tiny => "ggml-tiny.bin",
            WhisperModel::Base => "ggml-base.bin",
            WhisperModel::Small => "ggml-small.bin",
            WhisperModel::Medium => "ggml-medium.bin",
            WhisperModel::Large => "ggml-large.bin",
        }
    }

    /// 从文件名字符串解析模型类型
    pub fn from_filename(name: &str) -> Option<Self> {
        match name {
            "ggml-tiny.bin" | "tiny" => Some(WhisperModel::Tiny),
            "ggml-base.bin" | "base" => Some(WhisperModel::Base),
            "ggml-small.bin" | "small" => Some(WhisperModel::Small),
            "ggml-medium.bin" | "medium" => Some(WhisperModel::Medium),
            "ggml-large.bin" | "large" => Some(WhisperModel::Large),
            _ => None,
        }
    }

    /// 建议的语言参数 (基于模型能力)
    pub fn default_language(&self) -> &'static str {
        match self {
            WhisperModel::Tiny => "en",   // Tiny 仅支持英文
            _ => "auto",                   // 其他模型支持自动检测
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
#[derive(Clone)]
pub struct WhisperEngine {
    /// 模型上下文
    ctx: Arc<WhisperContext>,
    /// 模型类型
    model: WhisperModel,
    /// 转写语言 ("zh", "en", "ja", "auto" 等)
    language: String,
}

impl WhisperEngine {
    /// 创建新的 Whisper 引擎实例
    ///
    /// # 参数
    /// * `model` - 模型类型
    /// * `model_dir` - 模型文件目录
    /// * `language` - 转写语言代码 ("zh" / "en" / "ja" / "auto")
    ///
    /// # 返回
    /// 成功返回引擎实例，失败返回错误
    pub fn new(
        model: WhisperModel,
        model_dir: PathBuf,
        language: Option<String>,
    ) -> Result<Self, WhisperError> {
        let model_path = model_dir.join(model.filename());

        if !model_path.exists() {
            return Err(WhisperError::ModelNotFound(model_path));
        }

        let ctx = WhisperContext::new_with_params(
            model_path.to_str().unwrap(),
            whisper_rs::WhisperContextParameters::default(),
        )
        .map_err(|e| WhisperError::ModelLoadError(e.to_string()))?;

        let language = language.unwrap_or_else(|| model.default_language().to_string());

        Ok(Self {
            ctx: Arc::new(ctx),
            model,
            language,
        })
    }

    /// 获取模型类型
    pub fn model(&self) -> WhisperModel {
        self.model
    }

    /// 获取语言设置
    pub fn language(&self) -> &str {
        &self.language
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
        params.set_language(Some(&self.language));
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
    /// 支持 MP3、AAC (MP4)、WAV 等格式，自动重采样到 16kHz 单声道
    fn load_audio(path: &PathBuf) -> Result<Vec<f32>, WhisperError> {
        // 打开音频文件
        let file = std::fs::File::open(path).map_err(|e| {
            WhisperError::AudioLoadError(format!("无法打开音频文件：{}", e))
        })?;

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // 根据文件扩展名提示格式
        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        // 探测格式并创建解码器
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
            .map_err(|e| WhisperError::AudioLoadError(format!("无法识别音频格式：{}", e)))?;

        let mut format = probed.format;
        let track = format
            .default_track()
            .ok_or_else(|| WhisperError::AudioLoadError("未找到音频轨道".to_string()))?;

        let sample_rate = track
            .codec_params
            .sample_rate
            .unwrap_or(16000);

        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &DecoderOptions::default())
            .map_err(|e| WhisperError::AudioLoadError(format!("无法创建解码器：{}", e)))?;

        let track_id = track.id;

        // 读取并累积所有音频采样
        let mut raw_samples: Vec<f32> = Vec::new();

        loop {
            let packet = match format.next_packet() {
                Ok(p) => p,
                Err(e) => {
                    if e.to_string().contains("end of stream") {
                        break;
                    }
                    return Err(WhisperError::AudioLoadError(format!("读取音频包失败：{}", e)));
                }
            };

            // 跳过非目标轨道的包
            if packet.track_id() != track_id {
                continue;
            }

            let decoded = decoder
                .decode(&packet)
                .map_err(|e| WhisperError::AudioLoadError(format!("解码音频失败：{}", e)))?;

            // 将解码后的采样追加到缓冲区（自动混音为单声道）
            match &decoded {
                AudioBufferRef::F32(buf) => {
                    let spec = *buf.spec();
                    let num_frames = buf.frames();
                    let num_channels = spec.channels.count();
                    for i in 0..num_frames {
                        let mut sum = 0.0f64;
                        for ch in 0..num_channels {
                            sum += buf.chan(ch)[i] as f64;
                        }
                        raw_samples.push((sum / num_channels as f64) as f32);
                    }
                }
                AudioBufferRef::S16(buf) => {
                    let spec = *buf.spec();
                    let num_frames = buf.frames();
                    let num_channels = spec.channels.count();
                    for i in 0..num_frames {
                        let mut sum = 0.0f64;
                        for ch in 0..num_channels {
                            sum += buf.chan(ch)[i] as f64 / i16::MAX as f64;
                        }
                        raw_samples.push((sum / num_channels as f64) as f32);
                    }
                }
                AudioBufferRef::U8(buf) => {
                    let spec = *buf.spec();
                    let num_frames = buf.frames();
                    let num_channels = spec.channels.count();
                    for i in 0..num_frames {
                        let mut sum = 0.0f64;
                        for ch in 0..num_channels {
                            sum += (buf.chan(ch)[i] as f64 - 128.0) / 128.0;
                        }
                        raw_samples.push((sum / num_channels as f64) as f32);
                    }
                }
                _ => {
                    // 跳过不支持的采样格式
                }
            }
        }

        if raw_samples.is_empty() {
            return Err(WhisperError::AudioLoadError("未读取到有效音频数据".to_string()));
        }

        // 重采样到 16kHz（数据已经是单声道）
        let audio_data = Self::resample_to_16khz(&raw_samples, sample_rate);

        Ok(audio_data)
    }

    /// 线性插值重采样到 16kHz
    ///
    /// * `samples` - 单声道 f32 采样
    /// * `from_rate` - 原始采样率
    fn resample_to_16khz(samples: &[f32], from_rate: u32) -> Vec<f32> {
        const TARGET_RATE: u32 = 16000;

        if from_rate == TARGET_RATE {
            return samples.to_vec();
        }

        let ratio = from_rate as f64 / TARGET_RATE as f64;
        let output_len = (samples.len() as f64 / ratio).ceil() as usize;

        let mut output = Vec::with_capacity(output_len);

        for out_idx in 0..output_len {
            let src_pos = out_idx as f64 * ratio;
            let src_idx = src_pos as usize;
            let frac = src_pos - src_idx as f64;

            let curr = samples[src_idx.min(samples.len().saturating_sub(1))] as f64;
            let next = samples[(src_idx + 1).min(samples.len().saturating_sub(1))] as f64;

            let interpolated = curr + (next - curr) * frac;
            output.push(interpolated as f32);
        }

        output
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

    /// 集成测试：验证模型文件能被 whisper-rs 加载
    ///
    /// 此测试需要提前下载模型文件到 src-tauri/models/ 目录
    /// 使用 `cargo test -- --ignored` 来运行
    #[test]
    #[ignore]
    fn test_load_model_file() {
        let model_dir = std::path::PathBuf::from("models");
        let language = Some("en".to_string());
        let engine = WhisperEngine::new(WhisperModel::Tiny, model_dir, language);
        assert!(engine.is_ok(), "模型文件加载失败：{:?}", engine.err());
    }

    /// 端到端转写测试：使用指定模型转写音频文件，输出 SRT 字幕
    ///
    /// 字幕文件会生成在音频文件的同级目录，与音频文件同名、扩展名为 .srt
    ///
    /// 环境变量：
    /// - AUDIO_FILE (必填): 音频文件路径
    /// - AUDIO_MODEL (可选): 模型名称, tiny/base/small/medium/large, 默认 base
    /// - AUDIO_LANG (可选): 语言代码, zh/en/ja/auto, 默认 auto
    ///
    /// 用法：
    /// ```bash
    /// cd src-tauri
    /// # 中文音频 (推荐 Base 以上模型)
    /// AUDIO_FILE=/path/to/audio.mp3 AUDIO_MODEL=base AUDIO_LANG=zh cargo test --lib asr -- test_transcribe_sample --ignored --nocapture
    /// ```
    ///
    /// 前置条件：对应模型文件已下载到 src-tauri/models/ 目录
    #[test]
    #[ignore]
    fn test_transcribe_sample() {
        let model_dir = std::path::PathBuf::from("models");

        let model_name = std::env::var("AUDIO_MODEL").unwrap_or_else(|_| "base".to_string());
        let model = WhisperModel::from_filename(&model_name)
            .expect("无效的模型名称，可选: tiny/base/small/medium/large");

        let language = std::env::var("AUDIO_LANG").ok();

        let engine = WhisperEngine::new(model, model_dir, language)
            .expect("模型加载失败");

        let audio_path_str = std::env::var("AUDIO_FILE")
            .expect("请设置 AUDIO_FILE 环境变量指向音频文件路径");

        let audio_path = std::path::PathBuf::from(&audio_path_str);
        assert!(audio_path.exists(), "音频文件不存在：{}", audio_path.display());

        // 生成同目录同名的 .srt 字幕文件路径
        let srt_path = audio_path.with_extension("srt");

        println!("\n========== 开始转写 ==========");
        println!("模型：{}", engine.model().filename());
        println!("音频：{}", audio_path.display());
        println!("字幕：{}", srt_path.display());
        println!("");

        let result = engine.transcribe(audio_path, |progress| {
            println!(
                "[{}/{}] {} -- {}",
                progress.segment_index + 1,
                progress.total_segments,
                &progress.text,
                &WhisperEngine::format_timestamp(progress.start_time)
            );
        }).expect("转写失败");

        // 生成 SRT 格式字幕
        let mut srt_content = String::new();
        for (i, seg) in result.segments.iter().enumerate() {
            srt_content.push_str(&format!("{}\n", i + 1));
            srt_content.push_str(&format!(
                "{} --> {}\n",
                format_srt_time(seg.start_time),
                format_srt_time(seg.end_time),
            ));
            srt_content.push_str(&format!("{}\n\n", seg.text.trim()));
        }

        std::fs::write(&srt_path, &srt_content)
            .expect("写入字幕文件失败");

        println!("\n========== 转写完成 ==========");
        println!("共 {} 个段落", result.segments.len());
        println!("字幕已保存到：{}", srt_path.display());
    }

    /// 格式化毫秒时间为 SRT 时间格式 HH:MM:SS,mmm
    fn format_srt_time(time_ms: i64) -> String {
        let total_seconds = time_ms / 1000;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        let millis = time_ms % 1000;
        format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, millis)
    }
}
