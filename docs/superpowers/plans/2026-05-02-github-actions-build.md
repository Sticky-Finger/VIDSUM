# GitHub Actions 构建 Tauri 应用计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task.

**目标**: 在 GitHub Actions 上实现 VidSum Tauri 2 应用的三端（Windows/macOS/Linux）手动触发构建，产出各平台安装包

**架构**: 创建一个 workflow 文件，使用矩阵策略在四个 runner 上并行构建（含 Intel 和 Apple Silicon 双 macOS 架构）。通过 `tauri-apps/tauri-action@v0` 调用 `pnpm tauri build`，各平台安装独立的系统依赖，Whisper 模型从 HuggingFace 下载并缓存

**核心文件**:
- 新建: `.github/workflows/build.yml`

**关键背景**:
- 包管理：pnpm 9.x，lockfile v9；前端构建：`pnpm build` = `tsc && vite build`
- Tauri 配置中已设 `beforeBuildCommand: "pnpm build"`、`bundle.targets: "all"`、`resources: {"models/*": "models/"}`
- Whisper 模型 `ggml-tiny.bin`（77MB）未提交到 Git（.gitignore 排除），CI 需从 HuggingFace 下载
- whisper-rs 需要 cmake 和 C++ 编译器
- 现有 workflow 只有 gitleaks.yml，无构建流水线

---

## Tasks

### Task 1: 创建 GitHub Actions 构建 workflow ✅

**新建文件**: `.github/workflows/build.yml`

**触发条件**：`push`（任意分支推送自动触发），`timeout-minutes: 45`

**平台矩阵**（4 项）：
- `windows-latest` → `x86_64-pc-windows-msvc`
- `macos-latest` → `aarch64-apple-darwin`（Apple Silicon）
- `macos-13` → `x86_64-apple-darwin`（Intel）
- `ubuntu-22.04` → `x86_64-unknown-linux-gnu`

**步骤**：
- [ ] 配置 workflow 基本结构和 3 平台矩阵（fail-fast: false）
- [ ] 安装系统依赖：Ubuntu 装 cmake + GTK3 + WebKit2GTK + librsvg2 + patchelf；macOS 装 cmake（brew）；Windows 装 cmake（choco）+ 配置 MSVC 环境（ilammy/msvc-dev-cmd@v1）
- [ ] 安装 pnpm 9（pnpm/action-setup@v4）→ Node.js 20（actions/setup-node@v4，cache: pnpm）→ `pnpm install --frozen-lockfile`
- [ ] 安装 Rust stable + 矩阵 target（dtolnay/rust-toolchain@stable），缓存 Rust 依赖（Swatinem/rust-cache@v2，workspaces: src-tauri）
- [ ] 缓存 Whisper 模型（actions/cache@v4），未命中时从 HuggingFace 下载 `ggml-tiny.bin` 到 `src-tauri/models/`
- [ ] 构建 Tauri 应用（tauri-apps/tauri-action@v0，tauriScript: "pnpm tauri"）
- [ ] 上传各平台构建产物（actions/upload-artifact@v4），覆盖 msi/exe/dmg/app/AppImage/deb/rpm

---

## 测试清单

- [ ] 提交 workflow 文件并推送到 GitHub
- [ ] 在 Actions 页面手动触发 "构建发布" workflow
- [ ] 确认四个平台 job 都成功完成
- [ ] 从 artifact 下载各平台安装包并验证能正常安装运行

## 验收标准

1. 任意推送后四个平台构建任务并行启动
2. Windows 产出 `.msi` / `.exe`，macOS 产出 `.dmg`，Linux 产出 `.AppImage` / `.deb`
3. Whisper 模型文件被正确打包到各平台安装包内
