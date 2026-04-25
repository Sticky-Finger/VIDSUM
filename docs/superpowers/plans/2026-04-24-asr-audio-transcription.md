# 音频转文字 (ASR) 功能实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task.

**目标**: 实现 VidSum MVP 的"音频转文字"功能，支持本地 Whisper 转写（Tiny/Base 模型）和云端 ASR 预留接口

**架构**:
- Rust 后端使用 whisper-rs 实现本地转写，通过 Tauri event 推送进度
- React 前端显示模型选择和转写进度条
- 云端 ASR 预留 OpenAI 兼容 API 接口

**核心文件**:
- `src-tauri/src/asr/whisper_engine.rs` - Whisper 引擎封装
- `src-tauri/src/asr/cloud_asr.rs` - 云端 ASR 客户端
- `src-tauri/src/commands/asr.rs` - ASR 相关命令
- `src/components/ModelSelect.tsx` - 模型选择组件
- `src/components/AsrProgress.tsx` - 转写进度条组件

---

## Tasks

### Task 1: 添加 Rust 依赖 ✅

**状态**: 已完成 (2026-04-24)

**修改文件**:
- `src-tauri/Cargo.toml` - 添加 whisper-rs, hound, reqwest, tokio, thiserror

**测试**: `cargo check` 通过 ✅

---

### Task 2: 实现 Whisper ASR 引擎模块 ✅

**状态**: 已完成 (2026-04-24)

**创建文件**:
- `src-tauri/src/asr/mod.rs` - 模块导出
- `src-tauri/src/asr/whisper_engine.rs` - WhisperModel 枚举、WhisperEngine 结构体、transcribe 方法

**修改文件**:
- `src-tauri/src/lib.rs` - 添加 asr 模块声明

**功能**:
- 支持 Tiny/Base 两种模型
- 模型文件路径：`src-tauri/models/ggml-*.bin`
- 实现音频加载和转写逻辑
- 支持进度回调

**测试**:
- `cargo check` 通过 ✅
- `cargo test --lib asr` 通过，4 个单元测试全部通过 ✅

---

### Task 3: 实现云端 ASR 模块 ✅

**状态**: 已完成 (2026-04-24)

**创建文件**:
- `src-tauri/src/asr/cloud_asr.rs` - CloudAsrConfig 配置、CloudAsrClient 客户端、TranscriptionResult 结果

**修改文件**:
- `src-tauri/Cargo.toml` - 添加 reqwest multipart feature

**功能**:
- 支持配置 API URL、API Key、Model
- 预留 OpenAI Whisper API 兼容接口
- 异步转写方法（multipart 表单上传）
- 配置验证（validate 方法）
- 完整错误类型（HttpError、ApiError、InvalidResponse、ConfigError）

**测试**:
- `cargo check` 通过 ✅
- `cargo test --lib asr` 通过，5 个云端 ASR 单元测试全部通过 ✅

---

### Task 4: 创建 ASR 命令接口 ✅

**状态**: 已完成 (2026-04-25)

**创建文件**:
- `src-tauri/src/commands/asr.rs` - init_whisper_engine、start_transcription、transcribe_with_cloud 命令

**修改文件**:
- `src-tauri/src/commands/mod.rs` - 导出 asr 命令
- `src-tauri/src/main.rs` - 注册 ASR 命令到 invoke_handler，管理 AppState（Arc<Mutex<Option<WhisperEngine>>>）
- `src-tauri/src/asr/whisper_engine.rs` - 为 WhisperEngine 添加 #[derive(Clone)]
- `src-tauri/Capabilities/default.json` - 添加 fs:allow-exists 权限

**功能**:
- 初始化 Whisper 引擎命令
- 开始转写命令（后台线程执行）
- 云端 ASR 转写命令
- 通过 Tauri event 发送进度和结果
- 使用 Arc<Mutex<Option<WhisperEngine>>> 管理引擎状态，实现跨线程安全共享

**测试**:
- `cargo check` 通过 ✅
- `cargo test --lib asr` 通过，8 个单元测试全部通过 ✅

---

### Task 5: 下载测试模型并验证 ✅

**状态**: 已完成 (2026-04-26)

**执行**:
- 创建 `src-tauri/models/` 目录
- 从 `huggingface.co/ggerganov/whisper.cpp` 下载 ggml-tiny.bin (77MB) 和 ggml-base.bin (147MB)
- 添加 `src-tauri/models/` 到 `.gitignore`，不上传大模型文件
- 在 `tauri.conf.json` 中配置 `bundle.resources` 打包模型文件
- 用 `symphonia` 替换 `hound`，扩展音频解码支持 MP3 / AAC(MP4) / WAV + 自动重采样到 16kHz
- 扩展 `WhisperModel` 枚举支持 Small/Medium/Large（原来只有 Tiny/Base）
- 在 `WhisperEngine::new` 和 `init_whisper_engine` 中增加 `language` 参数（zh/en/ja/auto）
- 新增 `test_transcribe_sample` 端到端测试，输出 SRT 字幕文件

**修改文件**:
- `src-tauri/src/asr/whisper_engine.rs` - 新增集成测试；重写 `load_audio`；扩展 WhisperModel + language 参数
- `src-tauri/src/commands/asr.rs` - ModelName 扩展 + init_whisper_engine 增加 language
- `.gitignore` - 忽略 models 目录
- `src-tauri/tauri.conf.json` - 配置 bundle.resources
- `src-tauri/Cargo.toml` - 添加 `symphonia` (v0.5)，移除 `hound`

**验证清单**:
- [x] Tiny 模型 (77MB) 加载成功 (`cargo test --ignored` 通过)
- [x] Base 模型 (147MB) 下载并加载成功
- [x] 支持 MP3 / MP4 / WAV 音频输入 + 自动重采样到 16kHz 单声道
- [x] `cargo check` 编译通过
- [x] 全部 8 个单元测试通过
- [x] **端到端中文转写验证**：在 Claude Code 中通过 Bash 工具运行以下命令测试成功
  ```bash
  cd src-tauri && AUDIO_FILE=samples/scene1.mp3 AUDIO_MODEL=base AUDIO_LANG=zh \
    cargo test --lib asr -- test_transcribe_sample --ignored --nocapture
  ```
  输出 3 段中文转写 + SRT 字幕文件，结果准确（原文"我做了一个本地AI搜索工具，今天正式开源了..."）

---

### Task 6: 实现前端组件

**创建文件**:
- `src/components/ModelSelect.tsx` - Tiny/Base 模型选择按钮
- `src/components/AsrProgress.tsx` - 进度条和实时文本显示

**修改文件**:
- `src/App.tsx` - 集成模型选择和转写流程

**功能**:
- 模型选择并初始化后端引擎
- 监听转写进度事件
- 显示进度条和当前转写文本
- 处理完成和错误事件

**测试**: `pnpm tauri dev` 启动后 UI 正常渲染

---

## 验收标准

1. 支持选择 Tiny 或 Base 模型
2. 转写过程显示实时进度条
3. 输出带 `[HH:MM:SS]` 时间戳的文本
4. 云端 ASR 接口预留完成
