# 输入与任务创建功能实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task.

**目标**: 实现 VidSum MVP 的"输入与任务创建"功能，允许用户选择本地音视频文件或字幕文件

**架构**: Tauri 2.0 + React 19，前端提供 UI，Rust 后端处理文件对话框

**核心文件**:
- `src/components/InputModeSelect.tsx` - 输入模式选择界面
- `src/components/FileSelector.tsx` - 文件选择器组件
- `src-tauri/src/commands/file.rs` - Rust 文件选择命令

---

## Tasks

### Task 1: 项目初始化 ✅

**状态**: 已完成 (2026-04-23)

创建基础配置文件和 Tauri 后端骨架。

**创建文件**:
- `package.json` - React 19, Tauri 2.0, Tailwind 依赖
- `tsconfig.json`, `tsconfig.node.json` - TypeScript 配置
- `vite.config.ts` - Vite 配置，端口 1420
- `tailwind.config.js` - Tailwind 配置，shadcn 主题变量
- `postcss.config.js` - PostCSS 配置
- `index.html` - HTML 入口
- `src/index.css` - 全局样式，CSS 变量定义
- `src/main.tsx` - React 入口
- `src/App.tsx` - 根组件（暂时返回空）
- `src-tauri/Cargo.toml` - Rust 依赖 (tauri 2, tauri-plugin-shell, tauri-plugin-dialog)
- `src-tauri/tauri.conf.json` - Tauri 配置，窗口 1200x800
- `src-tauri/src/main.rs` - Rust 入口
- `src-tauri/capabilities/default.json` - 权限配置
- `src-tauri/build.rs` - Tauri 构建脚本 (额外创建)
- `src-tauri/src/lib.rs` - Rust 库入口 (额外创建)

**测试**: `pnpm tauri dev` 能启动空白窗口 ✅ 已验证

---

### Task 2: UI 组件和输入模式选择 ✅

**状态**: 已完成 (2026-04-23)

**创建文件**:
- `src/lib/utils.ts` - cn 工具函数
- `src/components/ui/button.tsx` - Button 组件 (variant: default/secondary/outline/ghost)
- `src/components/ui/card.tsx` - Card 组件 (Card/CardHeader/CardTitle/CardDescription/CardContent/CardFooter)
- `src/components/InputModeSelect.tsx` - 两个按钮：音视频文件转写 / 已有字幕文件

**修改文件**:
- `src/App.tsx` - 使用 InputModeSelect 组件
- `vite.config.ts` - 添加路径别名 `@` 配置

**测试**: `pnpm tauri dev` 启动后看到居中的卡片和两个按钮 ✅ 已验证

---

### Task 3: 文件选择功能 ✅

**状态**: 已完成 (2026-04-23)

**创建文件**:
- `src/components/FileSelector.tsx` - 文件选择 UI，调用 Rust 命令，显示文件信息

**修改文件**:
- `src-tauri/src/commands/file.rs` - `select_file(file_type)` 命令，使用 tauri-plugin-dialog
- `src-tauri/src/commands/mod.rs` - 导出 commands
- `src-tauri/src/main.rs` - 注册 select_file 命令
- `src/App.tsx` - 集成 FileSelector 组件，处理模式切换和返回

**测试清单** (`pnpm tauri dev`):
- [x] 应用启动显示"选择输入方式"界面
- [x] 点击"音视频文件转写"进入文件选择，对话框过滤音视频格式
- [x] 点击"已有字幕文件"进入文件选择，对话框过滤.srt/.vtt
- [x] 选择文件后显示名称、类型、大小、路径
- [x] 可以重新选择文件
- [x] 返回按钮回到模式选择

---

## 验收标准

1. 两个输入选项：音视频文件转写 / 已有字幕文件 ✅
2. 文件对话框正确过滤文件类型 (mp4/mkv/mov/mp3/wav/m4a vs srt/vtt) ✅
3. 选择文件后显示详细信息 ✅
4. 支持重新选择文件和返回上一级 ✅
