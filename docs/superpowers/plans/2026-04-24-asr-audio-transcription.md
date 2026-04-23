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

### Task 3: 实现云端 ASR 模块

**状态**: 已创建占位文件 (2026-04-24)

**创建文件**:
- `src-tauri/src/asr/cloud_asr.rs` - CloudAsrConfig 配置、CloudAsrClient 客户端（占位实现）

**功能**:
- 支持配置 API URL、API Key、Model
- 预留 OpenAI Whisper API 兼容接口
- 异步转写方法

**测试**: `cargo check` 通过

---

### Task 4: 创建 ASR 命令接口

**创建文件**:
- `src-tauri/src/commands/asr.rs` - init_whisper_engine、start_transcription、transcribe_with_cloud 命令

**修改文件**:
- `src-tauri/src/commands/mod.rs` - 导出 asr 命令
- `src-tauri/Capabilities/default.json` - 添加必要权限

**功能**:
- 初始化 Whisper 引擎命令
- 开始转写命令（后台线程执行）
- 通过 Tauri event 发送进度和结果

**测试**: `cargo check` 通过

---

### Task 5: 实现前端组件

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

### Task 6: 下载测试模型并验证

**执行**:
- 创建 `src-tauri/models/` 目录
- 下载 ggml-tiny.bin 模型文件

**测试清单** (`pnpm tauri dev`):
- [ ] 选择音频文件后显示模型选择
- [ ] 选择模型后开始转写
- [ ] 进度条实时更新
- [ ] 转写完成后显示带时间戳的文本

---

## 验收标准

1. 支持选择 Tiny 或 Base 模型
2. 转写过程显示实时进度条
3. 输出带 `[HH:MM:SS]` 时间戳的文本
4. 云端 ASR 接口预留完成
