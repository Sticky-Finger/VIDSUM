# 大模型 API 配置 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task.

**目标**: 用户可以配置 OpenAI 兼容 API（Base URL、API Key、Model ID），配置持久化到本地 JSON 文件

**架构**: Rust 后端新建 `llm` 模块管理配置持久化，前端新建 `LlmConfig` 组件提供配置表单 UI

**核心文件**:
- `src-tauri/src/llm/config.rs` - LLM 配置结构体 + JSON 持久化
- `src-tauri/src/llm/client.rs` - OpenAI 兼容 Chat Completions 客户端
- `src-tauri/src/llm/mod.rs` - 模块入口
- `src-tauri/src/commands/llm.rs` - Tauri 命令（load/save/test）
- `src/components/ui/input.tsx` - shadcn/ui Input 组件
- `src/components/ui/label.tsx` - shadcn/ui Label 组件
- `src/components/LlmConfig.tsx` - LLM 配置表单组件
- `src/App.tsx` - 新增 `llm_config` 模式

---

### Task 1: 创建 Rust LLM 配置模块 ✅

**创建文件**:
- `src-tauri/src/llm/config.rs` - `LlmConfig` 结构体（base_url/api_key/model）、`Default` 实现、`validate()` 校验、`load_config(app_data_dir)` 从 JSON 读取、`save_config(app_data_dir, config)` 写入 JSON
- `src-tauri/src/llm/mod.rs` - 导出 config 和 client 模块

**修改文件**:
- `src-tauri/src/lib.rs` - 新增 `pub mod llm`

**测试**: `cargo check` 编译通过

---

### Task 2: 创建 Rust LLM 客户端 ✅

**创建文件**:
- `src-tauri/src/llm/client.rs` - `LlmClient` 结构体，封装 reqwest 调用 OpenAI 兼容 `/chat/completions` 端点，支持 `ChatMessage` 消息列表输入，返回生成文本

**测试**: `cargo check` 编译通过

---

### Task 3: 创建 Tauri LLM 命令 ✅

**创建文件**:
- `src-tauri/src/commands/llm.rs` - 三个命令：
  - `load_llm_config` - 从本地文件加载配置
  - `save_llm_config` - 保存配置到本地文件
  - `test_llm_connection` - 发送测试消息验证 API 可用性

**修改文件**:
- `src-tauri/src/commands/mod.rs` - 新增 `pub mod llm`
- `src-tauri/src/main.rs` - 注册三个新命令到 `invoke_handler`

**测试**: `cargo check` 编译通过

---

### Task 4: 创建前端 shadcn/ui 基础组件 ✅

**创建文件**:
- `src/components/ui/input.tsx` - Input 组件（基于 shadcn/ui 规范）
- `src/components/ui/label.tsx` - Label 组件（基于 shadcn/ui 规范）

**测试**: `pnpm build` 构建通过

---

### Task 5: 创建 LlmConfig 前端组件 ✅

**创建文件**:
- `src/components/LlmConfig.tsx` - 配置表单组件，包含：
  - Base URL 输入框（默认 https://api.openai.com/v1）
  - API Key 输入框（密码类型）
  - Model ID 输入框（默认 gpt-4o-mini）
  - "保存配置并继续"按钮 → 调用 `save_llm_config`
  - "测试连接"按钮 → 调用 `test_llm_connection`
  - "返回"按钮
  - 组件加载时自动调用 `load_llm_config` 读取已保存配置

**测试**: `pnpm build` 构建通过

---

### Task 6: 集成到 App.tsx 流程 ✅

**修改文件**:
- `src/App.tsx`:
  - `AppMode` 新增 `'llm_config'` 模式
  - 导入 `LlmConfig` 组件
  - 修改 `handlePreviewConfirm`：字幕确认后进入 `llm_config` 模式（替换原 alert 占位）
  - 新增 `handleLlmConfigured` 回调（暂 alert 占位，后续接入总结）
  - `handleBack` 中处理 `llm_config` → `preview` 返回
  - 渲染 `LlmConfig` 组件

**测试**: `pnpm tauri dev` 启动后完整流程验证

---

## 验收标准

1. 字幕确认后进入 LLM API 配置页面（Base URL、API Key、Model ID 三个输入框）
2. 配置保存到本地 JSON 文件（`{app_data_dir}/llm_config.json`）
3. 重新打开应用后自动加载已保存的配置
4. "测试连接"按钮能验证 API 是否可用
5. 配置完成后点击"保存配置并继续"保存并进入下一步（暂时 alert 占位）
6. 所有输入框有 placeholder 提示和说明文字
