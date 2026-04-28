`chat-logs/`、`chat-logs-sanitized/` 和 `docs/tutorial/` 是教程素材，不是项目源码，日常开发时请忽略这些目录。

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

## 包管理规范
- **强制使用 pnpm**（版本 ≥8.0）作为唯一的 Node.js 包管理器。
- 禁止使用 npm 或 yarn 安装依赖。
- 所有前端依赖安装命令必须为 `pnpm add <package>`，开发依赖为 `pnpm add -D <package>`。
- 若 AI 需要生成脚手架（如 `create-react-app` 或 `vite`），请使用 `pnpm create` 而非 `npm create`。

## AI 协作规范
- **superpowers 插件已安装**，AI 应使用 `Skill` 工具调用相关技能
- **不使用 worktree**，直接在项目主目录建分支开发
  - 从目标分支建 feature 分支 → 实现 → `git merge --ff-only` 合回目标分支 → 删除 feature 分支
  - 原因：避免 worktree 重复安装依赖和编译 Rust 代码

# CLAUDE.md - AI 编码行为约束

## 核心流程

本项目编码实现严格遵循 `docs/WORKFLOW.md` 中定义的流程。你只负责 **步骤3：编码实现**。

## 强制性规则

1. **禁止使用 `brainstorming` 技能**  
   除非用户明确要求，否则不得调用 brainstorming、不得全项目扫描、不得生成 `specs/` 目录。

2. **仅处理 `docs/TODO.md` 中未完成的任务**  
   开始任何实现前，先确认对应条目未被勾选。

3. **必须使用 `writing-plans` + `executing-plan`**（按 Task 粒度执行）  
   - **步骤 A**：用 `writing-plans` 生成计划（存于 `docs/superpowers/plans/`）  
     计划文件结构：  
     - 以 `### Task N: 任务名称` 为最小工作单元  
     - 每个 Task 内可包含子步骤（`- [ ]`），但完成标志是 **Task 标题后添加 ✅**（例如 `### Task 1: 初始化 ✅`）  
     - 未完成的 Task 标题后无 ✅，且内部子步骤应为 `- [ ]` 或混合状态  
   - **步骤 B**：用 `executing-plan` 执行  
     - **一个会话至少完整完成一个 Task**（即从 `- [ ]` 做到 `✅` 并更新内部子步骤）  
     - 如果无法一次性完成某个 Task（例如会话中断），**下次必须继续该 Task**，直到它被标记 ✅，再进入下一个 Task  
     - 完成一个 Task 后，同步更新 `docs/TODO.md` 和 `docs/PRD.md` 中的对应进度  

4. **支持断点续传**  
   如果会话中断，用户说“继续执行计划”时，读取最近的计划文件，找到**第一个未标记 ✅ 的 Task**，从中断点继续完成该 Task。

5. **完成后同步进度**  
   所有 Task 都标记 ✅ 后，更新 `docs/TODO.md` 和 `docs/PRD.md` 中对应的任务状态。

## 例外情况

- 用户明确说“使用 brainstorming”或“讨论设计”时，才可临时启用 brainstorming。
- 用户要求扫描项目时，需明确指定扫描范围（如 `src/` 下的某个子目录），不得全根扫描。