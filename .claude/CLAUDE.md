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

3. **必须使用 `writing-plans` + `executing-plan`**  
   - 步骤 A：用 `writing-plans` 生成计划（存于 `docs/superpowers/plans/`）  
     计划文件必须包含：原子化的 `- [ ]` 步骤、每个步骤的文件路径、验证方式  
   - 步骤 B：用 `executing-plan` 逐步骤执行，完成后将 `[ ]` 改为 `[x]`  

4. **支持断点续传**  
   如果会话中断，用户说“继续执行计划”时，读取最近的计划文件，从第一个未完成步骤继续。

5. **完成后同步进度**  
   所有步骤勾选完成后，更新 `docs/TODO.md` 和 `docs/PRD.md` 中对应的任务状态。

## 例外情况

- 用户明确说“使用 brainstorming”或“讨论设计”时，才可临时启用 brainstorming。
- 用户要求扫描项目时，需明确指定扫描范围（如 `src/` 下的某个子目录），不得全根扫描。