//! ASR (自动语音识别) 相关命令
//!
//! 提供 Whisper 本地转写和云端 ASR 的 Tauri 命令接口

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager};

use vidsum_lib::asr::{
    CloudAsrConfig, CloudAsrClient, WhisperEngine, WhisperModel,
    TranscriptionProgress, TranscriptionSegment, WhisperError,
};
use vidsum_lib::asr::whisper_engine::TranscriptionResult;

/// 模型名称参数（前端传入）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelName {
    Tiny,
    Base,
    Small,
    Medium,
    Large,
}

impl From<ModelName> for WhisperModel {
    fn from(name: ModelName) -> Self {
        match name {
            ModelName::Tiny => WhisperModel::Tiny,
            ModelName::Base => WhisperModel::Base,
            ModelName::Small => WhisperModel::Small,
            ModelName::Medium => WhisperModel::Medium,
            ModelName::Large => WhisperModel::Large,
        }
    }
}

/// 转写进度事件payload（发送到前端）
#[derive(Debug, Clone, Serialize)]
pub struct ProgressPayload {
    /// 当前段落索引
    pub segment_index: i32,
    /// 总段落数
    pub total_segments: i32,
    /// 当前转写文本
    pub text: String,
    /// 开始时间 (毫秒)
    pub start_time: i64,
    /// 结束时间 (毫秒)
    pub end_time: i64,
}

impl From<TranscriptionProgress> for ProgressPayload {
    fn from(progress: TranscriptionProgress) -> Self {
        Self {
            segment_index: progress.segment_index,
            total_segments: progress.total_segments,
            text: progress.text,
            start_time: progress.start_time,
            end_time: progress.end_time,
        }
    }
}

/// 转写段落payload（发送到前端）
#[derive(Debug, Clone, Serialize)]
pub struct SegmentPayload {
    /// 段落文本
    pub text: String,
    /// 开始时间 (毫秒)
    pub start_time: i64,
    /// 结束时间 (毫秒)
    pub end_time: i64,
    /// 格式化时间戳 [HH:MM:SS]
    pub timestamp: String,
}

impl From<TranscriptionSegment> for SegmentPayload {
    fn from(segment: TranscriptionSegment) -> Self {
        Self {
            text: segment.text,
            start_time: segment.start_time,
            end_time: segment.end_time,
            timestamp: segment.timestamp,
        }
    }
}

/// 转写完成事件payload（发送到前端）
#[derive(Debug, Clone, Serialize)]
pub struct TranscriptionResultPayload {
    /// 完整转写文本
    pub full_text: String,
    /// 带时间戳的段落
    pub segments: Vec<SegmentPayload>,
}

impl From<TranscriptionResult> for TranscriptionResultPayload {
    fn from(result: TranscriptionResult) -> Self {
        Self {
            full_text: result.full_text,
            segments: result.segments.into_iter().map(SegmentPayload::from).collect(),
        }
    }
}

/// 转写错误事件payload（发送到前端）
#[derive(Debug, Clone, Serialize)]
pub struct ErrorPayload {
    /// 错误信息
    pub message: String,
}

/// 应用状态：Whisper 引擎
pub struct AppState {
    /// Whisper 引擎（初始化后使用）
    /// 使用 Arc<Mutex> 以便跨线程共享
    pub whisper_engine: Arc<Mutex<Option<WhisperEngine>>>,
}

/// 解析模型目录路径
///
/// 优先使用 Tauri 标准资源路径，开发模式下降级到源码目录
fn resolve_model_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    // 尝试 Tauri 标准资源路径
    if let Ok(resource_dir) = app.path().resource_dir() {
        let model_dir = resource_dir.join("models");
        eprintln!("[resolve_model_dir] 尝试 Tauri 资源目录: {}", model_dir.display());
        if model_dir.exists() {
            eprintln!("[resolve_model_dir] ✓ 使用 Tauri 资源目录: {}", model_dir.display());
            return Ok(model_dir);
        } else {
            eprintln!("[resolve_model_dir] ✗ 目录不存在");
        }
    } else {
        eprintln!("[resolve_model_dir] ✗ resource_dir() 获取失败");
    }

    // 开发模式降级：使用 CARGO_MANIFEST_DIR/models/
    let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("models");
    eprintln!("[resolve_model_dir] 尝试源码目录: {}", cargo_dir.display());
    if cargo_dir.exists() {
        eprintln!("[resolve_model_dir] ✓ 使用源码目录: {}", cargo_dir.display());
        return Ok(cargo_dir);
    } else {
        eprintln!("[resolve_model_dir] ✗ 源码目录也不存在");
    }

    // 都找不到时，给出详细的提示信息
    let expected = app
        .path()
        .resource_dir()
        .map(|d| d.join("models"))
        .unwrap_or_else(|_| PathBuf::from("models"));
    Err(format!(
        "模型目录不存在。请将模型文件放置在以下目录之一：\n\
         1. {}（Tauri 资源目录）\n\
         2. {}（源码目录）",
        expected.display(),
        cargo_dir.display(),
    ))
}

/// 初始化 Whisper 引擎
///
/// 前端选择模型后调用此命令，加载对应的模型文件
///
/// # 参数
/// - `model_name`: 模型名称 (tiny / base / small / medium / large)
/// - `language`: 转写语言代码 ("zh" / "en" / "ja" / "auto")，默认由模型决定
///
/// # 事件
/// 成功时发送 `asr:engine-initialized` 事件
/// 失败时发送 `asr:error` 事件
#[tauri::command]
pub async fn init_whisper_engine(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    model_name: ModelName,
    language: Option<String>,
) -> Result<String, String> {
    let model: WhisperModel = model_name.into();
    eprintln!(
        "[init_whisper_engine] 请求模型: {:?}, 文件名: {}",
        model,
        model.filename(),
    );

    // 解析模型目录（优先资源目录，降级到源码目录）
    let model_dir = resolve_model_dir(&app)?;
    eprintln!("[init_whisper_engine] 最终模型目录: {}", model_dir.display());

    // 初始化引擎
    let engine = WhisperEngine::new(model, model_dir, language.clone())
        .map_err(|e| {
            let msg = format!("初始化 Whisper 引擎失败：{}", e);
            app.emit("asr:error", ErrorPayload { message: msg.clone() })
                .ok();
            msg
        })?;

    // 存储到状态
    let mut engine_state = state.whisper_engine.lock()
        .map_err(|e| format!("获取状态锁失败：{}", e))?;
    *engine_state = Some(engine);

    let lang_display = language.as_deref().unwrap_or("auto");
    let result_msg = format!(
        "Whisper {} 模型已初始化（语言：{}）",
        model.filename(),
        lang_display,
    );
    app.emit("asr:engine-initialized", ErrorPayload { message: result_msg.clone() })
        .ok();

    Ok(result_msg)
}

/// 开始本地转写
///
/// 在后台线程执行转写，通过 Tauri event 推送进度和结果
///
/// # 参数
/// - `audio_path`: 音频文件路径
///
/// # 事件
/// 进度更新：`asr:progress`
/// 转写完成：`asr:transcription-completed`
/// 转写失败：`asr:error`
#[tauri::command]
pub async fn start_transcription(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    audio_path: String,
) -> Result<String, String> {
    // 验证引擎已初始化
    {
        let engine_state = state.whisper_engine.lock()
            .map_err(|e| format!("获取状态锁失败：{}", e))?;
        if engine_state.is_none() {
            let msg = "Whisper 引擎未初始化，请先调用 init_whisper_engine".to_string();
            app.emit("asr:error", ErrorPayload { message: msg.clone() }).ok();
            return Err(msg);
        }
    }

    // 获取 Arc 引用，以便在后台线程中使用
    let engine_arc: Arc<Mutex<Option<WhisperEngine>>> = state.whisper_engine.clone();
    let audio_path_buf = PathBuf::from(&audio_path);

    // 在后台线程执行转写（避免阻塞主线程）
    std::thread::spawn(move || {
        // 获取引擎并执行转写
        let engine: WhisperEngine = {
            let guard = engine_arc.lock().ok();
            match guard {
                Some(g) => match g.clone() {
                    Some(e) => e,
                    None => {
                        app.emit("asr:error", ErrorPayload { message: "Whisper 引擎未初始化".to_string() }).ok();
                        return;
                    }
                },
                None => {
                    app.emit("asr:error", ErrorPayload { message: "获取状态锁失败".to_string() }).ok();
                    return;
                }
            }
        };

        // 执行转写，进度通过事件发送
        let result: Result<TranscriptionResult, WhisperError> = engine.transcribe(audio_path_buf, |progress| {
            app.emit("asr:progress", ProgressPayload::from(progress)).ok();
        });

        match result {
            Ok(transcription_result) => {
                app.emit(
                    "asr:transcription-completed",
                    TranscriptionResultPayload::from(transcription_result),
                ).ok();
            }
            Err(e) => {
                app.emit(
                    "asr:error",
                    ErrorPayload { message: format!("转写失败：{}", e) },
                ).ok();
            }
        }
    });

    Ok(format!("开始转写：{}", audio_path))
}

/// 云端 ASR 转写
///
/// 使用云端 API 进行转写
///
/// # 参数
/// - `audio_path`: 音频文件路径
/// - `api_url`: API 地址（可选，默认 OpenAI）
/// - `api_key`: API Key
/// - `model`: 模型名称（可选，默认 whisper-1）
///
/// # 事件
/// 转写完成：`asr:cloud-transcription-completed`
/// 转写失败：`asr:error`
#[tauri::command]
pub async fn transcribe_with_cloud(
    app: tauri::AppHandle,
    audio_path: String,
    api_url: Option<String>,
    api_key: String,
    model: Option<String>,
) -> Result<String, String> {
    // 构建配置
    let config = CloudAsrConfig {
        api_url: api_url.unwrap_or_else(|| "https://api.openai.com/v1/audio/transcriptions".to_string()),
        api_key,
        model: model.unwrap_or_else(|| "whisper-1".to_string()),
    };

    let client = CloudAsrClient::new(config);

    // 验证配置
    client.validate().map_err(|e| {
        let msg = format!("云端 ASR 配置验证失败：{}", e);
        app.emit("asr:error", ErrorPayload { message: msg.clone() }).ok();
        msg
    })?;

    // 执行转写
    let result = client.transcribe(&audio_path).await.map_err(|e| {
            let msg = format!("云端转写失败：{}", e);
            app.emit("asr:error", ErrorPayload { message: msg.clone() }).ok();
            msg
        })?;

    // 发送完成事件
    app.emit(
        "asr:cloud-transcription-completed",
        serde_json::json!({
            "text": result.text,
            "language": result.language,
        }),
    ).ok();

    Ok("云端转写已完成".to_string())
}