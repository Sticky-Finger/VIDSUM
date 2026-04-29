# 已有字幕文件流程实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 补全"选择已有字幕文件（.srt/.vtt）作为起点"的完整流程，跳过 ASR 转写和字幕预览步骤，解析后直接进入 LLM 总结配置。

**流程（对应 PRD 流程图）：**
```
选择字幕文件 → 后台解析 → 自动确认字幕 → 进入 LLM 总结配置
```

**Architecture:**

- **解析层**：Rust 后端新增 `subtitle/` 模块，纯手动解析 SRT/VTT（不引入第三方库），通过 Tauri command 返回字幕条目列表
- **状态层**：前端 App.tsx 状态机新增 `parsing` 状态，字幕文件确认后调用 Rust 解析命令，成功后自动设置 `confirmedSubtitle` 并进入 `llm_config`
- **复用层**：进入 `llm_config` 后完全复用现有 LLM 总结流程，不引入新组件

**Tech Stack:** Rust (Tauri command + serde), React 19 + TypeScript, shadcn/ui

---

### Task 1: Rust 后端 - 新建 subtitle 模块及 SRT/VTT 解析器

**Files:**
- Create: `src-tauri/src/subtitle/mod.rs` — 模块导出
- Create: `src-tauri/src/subtitle/parser.rs` — SRT/VTT 解析逻辑 + 单元测试
- Modify: `src-tauri/src/lib.rs` — 添加 `pub mod subtitle`
- Modify: `src-tauri/src/commands/mod.rs` — 添加 `pub mod subtitle`
- Create: `src-tauri/src/commands/subtitle.rs` — `parse_subtitle_file` 命令
- Modify: `src-tauri/src/main.rs` — 注册新命令到 `generate_handler!`

**实现要点：**
- 解析器输出结构：`SubtitleEntry { index, text, start_time, end_time, timestamp }`，时间单位毫秒
- 支持格式：SRT（序号+`HH:MM:SS,mmm -->`）、VTT（`WEBVTT` 头部+`HH:MM:SS.mmm -->`，含选项中继跳过）
- 错误处理：格式不支持、文件无法读取、未解析到有效条目时返回可读错误信息
- 单元测试覆盖：标准 SRT/VTT、多行文本、中文、时间格式边界、非法内容

**Sub-steps:**

- [ ] 创建 `src-tauri/src/subtitle/mod.rs` 导出 parser 模块
- [ ] 创建 `src-tauri/src/subtitle/parser.rs` 实现完整解析逻辑 + 单元测试
- [ ] 运行 `cargo test` 确认测试通过
- [ ] 修改 `src-tauri/src/lib.rs` 添加 `pub mod subtitle`
- [ ] 修改 `src-tauri/src/commands/mod.rs` 添加 `pub mod subtitle`
- [ ] 创建 `src-tauri/src/commands/subtitle.rs` 添加 `parse_subtitle_file` 命令
- [ ] 修改 `src-tauri/src/main.rs` 注册 `commands::subtitle::parse_subtitle_file`

---

### Task 2: React 前端 - 补全"已有字幕"流程状态机

**Files:**
- Modify: `src/App.tsx`

**改动要点：**

1. `AppMode` 联合类型新增 `'parsing'` 状态
2. `handleConfirm`：字幕文件类型不再报错，改为 `setCurrentMode('parsing')` 并调用解析
3. 新增 `handleParseSubtitle`：调用 `invoke('parse_subtitle_file')`，成功后直接调用 `handlePreviewConfirm(entries)` 进入 `llm_config`
4. `parsing` 渲染：加载动画 + "正在解析字幕文件..." 文案 + 返回按钮
5. `handleBack` 增加 `parsing → confirm` 分支
6. 错误处理：解析失败时回到 `confirm` 并展示错误信息

**状态机流转对比：**

| 步骤 | 音视频模式 | 字幕模式 |
|------|-----------|---------|
| 1 | select | select |
| 2 | file | file |
| 3 | confirm | confirm |
| 4 | model_select | **parsing**（新） |
| 5 | transcribing | **llm_config**（解析成功后自动进入） |
| 6 | preview（SubtitlePreview） | summarizing |
| 7 | llm_config | summary |
| 8 | summarizing | — |
| 9 | summary | — |

**Sub-steps:**

- [ ] 修改 `AppMode` 新增 `'parsing'` 状态
- [ ] 修改 `handleConfirm` 将字幕文件分支改为进入解析
- [ ] 新增 `handleParseSubtitle`：调用 Rust 解析命令，成功则自动确认字幕并进入 LLM 配置
- [ ] 修改 `handleBack` 支持 `parsing → confirm`
- [ ] 新增 `parsing` 状态的渲染（加载动画 + 返回按钮）

---

### Task 3: 更新进度文档

**Files:**
- Modify: `docs/TODO.md`

- [ ] 将"已有字幕文件处理流程"下的子任务标记为完成
