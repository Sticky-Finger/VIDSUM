# 字幕确认与预览 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task.

**目标**: 转写完成后展示带编辑功能的时间轴字幕预览，支持 SRT/VTT 导出和确认进入总结

**架构**: 纯前端实现。AsrProgress 扩展出 SubtitlePreview 组件（左右分栏+编辑功能），导出通过前端生成 Blob 下载

**核心文件**:
- Create: `src/lib/subtitle-export.ts` - SRT/VTT 生成 + 文件下载工具
- Create: `src/components/SubtitlePreview.tsx` - 字幕预览组件
- Modify: `src/components/AsrProgress.tsx` - 转写完成后增加预览入口
- Modify: `src/App.tsx` - 新增 preview 模式 + 预留 confirmedSubtitle 状态

---

### Task 1: 创建导出工具函数 ✅

**Files**:
- Create: `src/lib/subtitle-export.ts`

**内容**: 导出 `SubtitleEntry` 接口、`generateSrt()`、`generateVtt()`、`downloadFile()` 四个导出项。

- [x] 创建 `src/lib/subtitle-export.ts`，包含 `SubtitleEntry` 接口、`generateSrt()`、`generateVtt()`、`downloadFile()`

---

### Task 2: 创建 SubtitlePreview 组件 ✅

**Files**:
- Create: `src/components/SubtitlePreview.tsx`

**内容**: 左右分栏布局——左侧时间轴列表（点击选中高亮），右侧段落详情（点击文本进入 textarea 编辑模式 + 保存/取消）。顶部工具栏（导出 SRT/VTT + 复制全文），底部"确认字幕并进入总结"按钮（暂 alert 占位）。

- [x] 创建 `src/components/SubtitlePreview.tsx`，包含左右分栏、编辑功能、导出工具集成、确认按钮

---

### Task 3: 改造 AsrProgress ✅

**Files**:
- Modify: `src/components/AsrProgress.tsx`

**内容**: 转写完成的结果区域新增"进入字幕预览"按钮，点击后隐藏原结果区，渲染 SubtitlePreview。转写完成时自动将 result.segments 映射为 SubtitleEntry[] 传给预览组件。导出 SRT/VTT 按钮也放在转写完成结果区。

- [x] 新增 onPreview prop、showPreview 状态、subtitleEntries memo
- [x] 转写完成结果区增加导出 SRT/VTT 和"进入字幕预览"按钮
- [x] 预览模式渲染 SubtitlePreview，确认回调 onPreview，返回回到结果区

---

### Task 4: 改造 App.tsx ✅

**Files**:
- Modify: `src/App.tsx`

**内容**: 新增 `AppMode` 的 `preview` 模式。新增 `confirmedSubtitle` 状态，通过 `onPreviewConfirm` 回调传入 AsrProgress，为后续大模型总结预留数据。导入 `SubtitleEntry` 类型。

- [x] 导入 SubtitleEntry，AppMode 新增 `preview`
- [x] 新增 confirmedSubtitle 状态、handlePreviewConfirm 回调、preview 模式渲染
- [x] AsrProgress 传入 onPreview prop

---

## 验收标准

1. 转写完成后可以看到字幕时间轴列表（左侧）和选中段详情（右侧）
2. 点击文本进入编辑模式，可修改文字，保存后左侧列表同步更新
3. 导出 SRT / VTT 格式正确的字幕文件
4. "确认字幕并进入总结"按钮弹出占位提示

---

## 已知问题 / Bug 追踪

### Bug 1: 左侧时间轴点击后右侧文字对不上

- **描述**: 在 SubtitlePreview 中，点击左侧时间轴列表中的段落，右侧显示的详情段落与左侧选中的段落不匹配
- **原因**: 待排查
- **状态**: ⚠️ 未修复
