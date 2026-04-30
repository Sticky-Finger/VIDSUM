# 字幕入口预览流程实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task.

**目标**: 从"已有字幕文件"入口进入时，解析完成后显示字幕预览界面，并禁用"重新转写"按钮

**架构**: 修改 App.tsx 的 handleParseSubtitle 函数，使其解析完成后进入 preview 模式而非直接跳到 llm_config；为 SubtitlePreview 组件添加 `canRetranscribe` prop 来控制"返回"按钮的行为和文案

**核心文件**:
- `src/App.tsx` - 修改 handleParseSubtitle 和 preview 模式渲染逻辑
- `src/components/SubtitlePreview.tsx` - 添加 canRetranscribe prop
- `src/components/AsrProgress.tsx` - 传递 canRetranscribe prop

---

## Tasks

### Task 1: 修改 App.tsx - 字幕解析完成后进入预览界面 ✅

**修改文件**:
- `src/App.tsx:72-84` - handleParseSubtitle 函数
- `src/App.tsx:286-303` - preview 模式渲染
- `src/App.tsx:37-56` - handleBack 函数

- [x] 修改 handleParseSubtitle：解析完成后进入 preview 模式（调用 setCurrentMode('preview')），而非直接调用 handlePreviewConfirm
- [x] 修改 preview 模式渲染：渲染 SubtitlePreview 组件，传递 canRetranscribe={selectedInputMode === 'media'}
- [x] 更新 handleBack：添加 preview 模式的返回逻辑（返回到 confirm 模式）

**测试**:
- [x] `pnpm tauri dev` 编译通过

---

### Task 2: 修改 SubtitlePreview 组件 - 添加 canRetranscribe prop ✅

**修改文件**:
- `src/components/SubtitlePreview.tsx:18-23` - SubtitlePreviewProps 接口
- `src/components/SubtitlePreview.tsx:251-257` - 底部"返回"按钮

- [x] SubtitlePreviewProps 接口添加 canRetranscribe: boolean
- [x] 组件解构 canRetranscribe prop
- [x] 底部"返回"按钮根据 canRetranscribe 显示不同文案：true 显示"← 重新转写"，false 显示"← 返回"

**测试**:
- [x] `pnpm tauri dev` 编译通过

---

### Task 3: 更新 AsrProgress 组件 - 传递 canRetranscribe prop ✅

**修改文件**:
- `src/components/AsrProgress.tsx:232-242` - SubtitlePreview 使用位置

- [x] 找到 SubtitlePreview 组件的渲染代码，添加 canRetranscribe={true}

**测试**:
- [x] `pnpm tauri dev` 编译通过

---

## 测试清单

- [x] 选择"已有字幕文件"入口 → 选择 .srt 文件 → 点击"开始处理" → 解析完成后显示字幕预览界面
- [x] 字幕预览界面底部"返回"按钮显示"← 返回"（非"重新转写"）
- [x] 点击"← 返回"可回到文件确认页
- [x] 选择"转写字幕"入口 → 完成转写后进入字幕预览 → 底部"返回"按钮显示"← 重新转写"

---

## 验收标准

1. 从"已有字幕文件"入口进入时，解析完成后必须显示字幕预览界面（而非直接跳到 LLM 配置）
2. 从"已有字幕文件"入口进入时，字幕预览界面的"返回"按钮显示"← 返回"，点击后回到文件确认页
3. 从"转写字幕"入口进入时，字幕预览界面的"返回"按钮显示"← 重新转写"，点击后回到主页
