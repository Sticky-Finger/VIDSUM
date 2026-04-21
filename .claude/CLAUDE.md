`chat-logs/` 和 `docs/tutorial/` 是教程素材，不是项目源码，日常开发时请忽略这些目录。

# VidSum 项目上下文 (MVP 阶段)

## 项目定位
VidSum 是一款隐私优先的本地桌面工具，专注于将本地音视频文件转写并总结为带时间戳的 Markdown 笔记。

## 当前技术栈 (仅 MVP 涉及)
- **桌面框架**：Tauri (Rust) + React 19 + TypeScript
- **前端构建**：Vite + Tailwind CSS + shadcn/ui
- **Rust 后端依赖**：Tokio, reqwest, whisper-rs (Whisper.cpp 绑定)
- **外部依赖**：无 Sidecar 程序，无 Python 运行时依赖

## 项目结构 (预期)

## 开发规范
- 所有代码、注释、文档必须使用中文
- 前端命名：组件 PascalCase，变量函数 camelCase
- 后端命名：文件 snake_case，结构体/枚举 PascalCase
- Git规范：
    - **分支命名**：main/develop/feature功能名/bugfix问题描述/hotfix紧急修复
    - **提交格式**：`<type>(<scope>): <subject>`
        - 类型：feat/fix/docs/style/refactor/perf/test/chore
        - 示例：`feat(search): 添加多模态搜索功能`