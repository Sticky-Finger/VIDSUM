# 音视频总结 VidSum

一款将音视频内容智能提炼为带时间戳分层 Markdown 总结的本地桌面工具。

## 技术栈
- 桌面框架：Tauri (Rust) + React 19 + TypeScript
- 前端工具链：Vite
- 样式方案：Tailwind CSS + shadcn/ui
- 核心引擎：Rust (Tokio + reqwest)
- AI 模型与引擎：
  - 语音识别抽象层：支持本地 Whisper.cpp（通过 `whisper-rs`）与第三方兼容 OpenAI 格式的 ASR 服务
  - 说话人分割：pyannote-rs
- 辅助工具 (Sidecar 模式)：yt-dlp (Python 独立可执行文件)
- 大模型接口：兼容 OpenAI 格式的 API (本地或云端)

## 核心功能
1. 音视频输入双通道（URL 链接 + 本地文件路径）
2. 智能字幕获取：
   - 优先下载官方字幕
   - 降级自动语音转文字，并支持说话人区分
   - 可灵活切换本地引擎与第三方 ASR 服务
3. 大模型结构化总结：
   - 自动生成层次分明、带时间戳锚点的 Markdown 文案
   - 支持接入本地或云端兼容 OpenAI 格式的 API
4. 模型自由选配：
   - 本地 STT：可选不同规格模型（tiny/base/small），并支持说话人分割开关
   - 总结服务：可切换不同的大模型 API 端点

## 开发规范
- 所有代码、注释、文档必须使用中文
- 前端命名：组件 PascalCase，变量函数 camelCase
- 后端命名：文件 snake_case，结构体/枚举 PascalCase
- Git规范：
    - **分支命名**：main/develop/feature功能名/bugfix问题描述/hotfix紧急修复
    - **提交格式**：`<type>(<scope>): <subject>`
        - 类型：feat/fix/docs/style/refactor/perf/test/chore
        - 示例：`feat(search): 添加多模态搜索功能`

### 敏感信息检测

#### 本地检测
- 安装 [Gitleaks](https://github.com/gitleaks/gitleaks)：`brew install gitleaks`（macOS）,`winget install gitleaks`（Windows 11）
- 全盘扫描：`gitleaks detect --source . -v`
- 配合 pre-commit 自动拦截含敏感信息的提交：
   ```bash
   pip install pre-commit
   # 在项目根目录创建 .pre-commit-config.yaml（示例见仓库）
   pre-commit install
   ```
   此后每次 git commit 会自动扫描暂存区，发现敏感信息则提交失败

#### 仓库层面
- 通过 GitHub Actions 自动扫描 push 和 PR（配置文件：.github/workflows/gitleaks.yml）

## 运行与构建

### 前置条件

#### macOS 15 (x86_64)
- Node.js v18+
- pnpm v8.0+（唯一推荐的 Node.js 包管理器）
- Rust（通过 [rustup](https://rustup.rs/) 安装）
- 系统依赖：参考 [Tauri v2 前置条件](https://v2.tauri.app/start/prerequisites/)

#### Windows 11
- Node.js v18+
- pnpm v8.0+（唯一推荐的 Node.js 包管理器）
- Rust（通过 [rustup](https://rustup.rs/) 安装）
- 系统依赖：参考 [Tauri v2 前置条件](https://v2.tauri.app/start/prerequisites/)
- 额外依赖与常见问题：[Windows 11 开发环境搭建指南](docs/win11-dev-setup.md)

### 安装依赖
项目根目录下运行
```bash
pnpm install
```

### 开发模式
```bash
cd src-tauri
pnpm tauri dev
```
- 前端开发服务器运行在 http://localhost:1420
- Rust 后端增量编译，修改代码后自动重载

### 生产构建
```bash
cd src-tauri
pnpm tauri build
```
构建产物输出至 `src-tauri/target/release/bundle/`，包含各平台安装包（.app、.dmg、.msi、.deb 等）。