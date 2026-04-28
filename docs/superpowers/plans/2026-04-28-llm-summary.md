# 大模型总结功能实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task.

**目标**: 用户点击"生成总结"后，系统使用可编辑的默认 Prompt 将字幕全文提交给 LLM，输出包含 `[HH:MM:SS]` 时间戳锚点的分层 Markdown。用户可在生成前查看和修改 Prompt。

**架构**: Rust 后端新增 `generate_summary` Tauri 命令，调用 `LlmClient` 发送请求；前端 `LlmConfig` 组件增加 Prompt 编辑区域，新增 `SummaryResult` 组件展示 Markdown 结果，`App.tsx` 新增 `summarizing` 和 `summary` 模式

**核心文件**:
- `src-tauri/src/llm/prompt.rs` - 默认 Prompt 模板（系统提示 + 用户提示模板）
- `src-tauri/src/llm/config.rs` - 扩展 LlmConfig 结构体，增加 prompt 字段
- `src-tauri/src/llm/mod.rs` - 导出 prompt 模块
- `src-tauri/src/commands/llm.rs` - 扩展命令，支持 prompt 持久化和生成总结
- `src-tauri/src/main.rs` - 注册新命令
- `src/components/LlmConfig.tsx` - 增加 Prompt 编辑区域
- `src/components/SummaryResult.tsx` - Markdown 总结结果展示组件
- `src/App.tsx` - 新增 `summarizing` 和 `summary` 模式

---

### Task 1: 创建 Rust Prompt 模块并扩展 LlmConfig ✅

**创建文件**:
- `src-tauri/src/llm/prompt.rs` - 定义默认 Prompt 模板，包含系统提示和用户提示模板

**修改文件**:
- `src-tauri/src/llm/config.rs`:
  - `LlmConfig` 结构体新增可选字段：`system_prompt: Option<String>`, `user_prompt_template: Option<String>`
  - `Default` 实现中这两个字段默认为 `None`
  - `load_config()` 加载时，如果 JSON 中存在则读取，否则为 `None`
  - `save_config()` 保存时，包含这两个字段
- `src-tauri/src/llm/mod.rs` - 新增 `pub mod prompt` 和导出

**Prompt 设计要求**:
- 系统提示（`DEFAULT_SYSTEM_PROMPT`）：指示 LLM 作为视频内容总结助手，输出分层 Markdown，每个段落必须包含 `[HH:MM:SS]` 时间戳锚点
- 用户提示模板（`DEFAULT_USER_PROMPT_TEMPLATE`）：包含 `{subtitles}` 占位符，用于插入字幕全文
- 提供 `fn get_default_system_prompt() -> String` 和 `fn get_default_user_prompt_template() -> String` 供前端获取默认值

**测试**: `cargo check` 通过 ✅

---

### Task 2: 扩展 Tauri 命令支持 Prompt 持久化和生成总结

**修改文件**:
- `src-tauri/src/commands/llm.rs`:
  - `LlmConfigPayload` 新增可选字段：`system_prompt: Option<String>`, `user_prompt_template: Option<String>`
  - 修改 `load_llm_config`：返回时包含已保存的 prompt（如果没有则为 null）
  - 修改 `save_llm_config`：保存时包含 prompt 字段
  - 新增 `SubtitleSegmentPayload` 结构体（接收前端字幕数据）
  - 新增 `get_default_prompt` Tauri 命令：
    - 参数：无
    - 返回：`Result<serde_json::Value, String>`（包含 `system_prompt` 和 `user_prompt_template`）
    - 逻辑：调用 `prompt::get_default_system_prompt()` 和 `prompt::get_default_user_prompt_template()`
  - 新增 `generate_summary` Tauri 命令：
    - 参数：`config: LlmConfigPayload`, `segments: Vec<SubtitleSegmentPayload>`, `system_prompt: String`, `user_prompt_template: String`
    - 逻辑：将 segments 格式化为带时间戳的文本，用 user_prompt_template 替换 `{subtitles}` 占位符，构建 ChatMessage 列表（system + user），调用 `LlmClient.chat_completions()`
    - 返回：`Result<String, String>`（总结 Markdown 文本）

- `src-tauri/src/main.rs` - 注册 `get_default_prompt` 和 `generate_summary` 命令到 `invoke_handler`

**测试**: `cargo check` 通过

---

### Task 3: 修改 LlmConfig 组件增加 Prompt 编辑区域

**修改文件**:
- `src/components/LlmConfig.tsx`:
  - 新增状态：`systemPrompt: string`, `userPromptTemplate: string`, `showPromptEditor: boolean`
  - 组件加载时：
    1. 调用 `invoke('load_llm_config')` 加载已保存的配置（含 prompt）
    2. 调用 `invoke('get_default_prompt')` 获取默认 Prompt
    3. 优先使用已保存的 prompt，如果没有则使用默认值
  - 新增"编辑 Prompt"折叠区域（默认折叠）：
    - 系统提示 Textarea（可编辑）
    - 用户提示模板 Textarea（可编辑，显示 `{subtitles}` 占位符说明）
    - "恢复默认"按钮：重置为 `get_default_prompt` 返回的默认值
  - 修改"保存配置并继续"按钮：将 `systemPrompt` 和 `userPromptTemplate` 一起传递给 `save_llm_config` 和 `onConfigured` 回调
  - 修改 `onConfigured` 签名：`(config: LlmConfigData, systemPrompt: string, userPromptTemplate: string) => void`

**测试**: `pnpm build` 通过

---

### Task 4: 创建前端 SummaryResult 组件

**创建文件**:
- `src/components/SummaryResult.tsx`:
  - Props: `summary: string`, `onBack: () => void`, `onExport: () => void`
  - 展示区域：使用 `prose` 类渲染 Markdown（支持标题层级、列表、代码块）
  - 按钮：返回（回到 llm_config）、导出 .md 文件
  - Markdown 渲染：使用 `dangerouslySetInnerHTML` 或轻量 Markdown 解析库

**测试**: `pnpm build` 通过

---

### Task 5: 集成到 App.tsx 流程

**修改文件**:
- `src/App.tsx`:
  - `AppMode` 新增 `'summarizing'` 和 `'summary'` 模式
  - 导入 `SummaryResult` 组件
  - 新增状态：`summaryResult: string | null`, `currentSystemPrompt: string`, `currentUserPromptTemplate: string`
  - 修改 `handleLlmConfigured` 签名：接收 `(config, systemPrompt, userPromptTemplate)`
    - 保存 prompt 到状态
    - 设置 `currentMode('summarizing')`
    - 调用 `invoke('generate_summary', { config, segments: confirmedSubtitle, systemPrompt, userPromptTemplate })`
    - 成功后设置 `summaryResult` 和 `currentMode('summary')`
    - 失败后显示错误提示，返回 `llm_config` 模式
  - 新增 `handleExportSummary`：调用 `downloadFile` 导出 .md 文件
  - `handleBack` 中处理 `summary` → `llm_config` 返回
  - 渲染 `SummaryResult` 组件

**测试**: `pnpm tauri dev` 启动后完整流程验证

---

## 验收标准

1. LLM 配置页面显示可折叠的"编辑 Prompt"区域
2. 点击"编辑 Prompt"可查看和修改系统提示、用户提示模板
3. 用户提示模板包含 `{subtitles}` 占位符说明，支持自定义修改
4. "恢复默认"按钮可重置 Prompt 为默认值
5. 点击"保存配置并继续"时，将自定义 Prompt 一起持久化到 `llm_config.json`
6. 关闭应用后重新打开，Prompt 编辑区域显示用户最后一次保存的内容
7. 首次使用（无已保存配置）时，显示默认 Prompt
8. 系统使用用户自定义 Prompt（或默认 Prompt）将字幕全文提交给 LLM API
9. 总结过程中显示加载状态
10. 总结完成后展示分层 Markdown，每个段落包含 `[HH:MM:SS]` 时间戳锚点
11. 支持导出 `.md` 文件保存至本地
12. 错误处理：API 调用失败时显示错误提示，可返回重新配置
