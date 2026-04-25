# 过程记录

## 1 初始化项目时，写项目级文档
### 1.1 和AI讨论需求得到精炼的README.md
### 1.2 和AI讨论得到MVP产品文档01-prd.md
### 1.3 和AI讨论并将01-prd.md中需求写到TODO.md

## 2 实现TODO.md中第一个任务

### 2.1 生成第一个任务的superpowers的任务的plan文档

对话过程记录: [2026-04-23-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-23-caveat-the-messages-below-were-generated-by-the-u.txt)

期间cc自动调用了插件命令 superpowers:writing-plans，生成了任务的plan文档 [2026-04-22-input-task-creation.md](../superpowers/plans/2026-04-22-input-task-creation.md)

#### 补充：开始生成plan文档时遇到了生成的文档达到1k多行过于冗长的问题

**解决办法**-提示词：

```bash
你看下你用'/superpowers:write-plan'写出来的 
@docs/superpowers/plans/2026-04-22-input-task-creation.md 
，有什么问题？我觉得这个plan文档也太长了 
```
然后，ai就提供了一个精简缩短版本的plan文档内容

### 2.2 使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的Task 1

对话过程记录：[2026-04-23-1503-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-23-1503-caveat-the-messages-below-were-generated-by-the-u.txt)

- 让ai写代码实现Task 1的提示词：`superpowers:executing-plan 
@docs/superpowers/plans/2026-04-22-input-task-creation.md `
- 实现期间 superpowers 用 superpowers:using-git-worktrees
  创建隔离的工作空间
- ai自动测试 pnpm tauri dev 是否能启动时，出现了问题且ai自己一直解决不了卡住了。此时手动停止ai运行的 pnpm tauri dev 命令，并且自己手动运行测试和解决问题直至运行成功
- 手动测试调试debug成功后，告诉AI“我本地手动调试并运行验证了task 1，你查看下”，给AI发消息让它更新plan文档中task 1的进度，并且把项目源码git提交：
    - 提示词：`完成了task1，先更新superpowers生成的相关plan文档 @.worktrees/input-task-c
    reation/docs/superpowers/plans/2026-04-22-input-task-creation.md 标明进度，并且git提交当前代码文件么？`

### 2.3 使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的Task 2

对话过程记录：[2026-04-23-1629-superpowersexecuting-plan-worktreesinput.txt](../../chat-logs/2026-04-23-1629-superpowersexecuting-plan-worktreesinput.txt)

### 2.4 使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的Task 3

对话过程记录：[2026-04-23-1805-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-23-1805-caveat-the-messages-below-were-generated-by-the-u.txt)

完成了第一个任务，合并worktree到住代码库，然后在 docs/ 下的 01-prd.md 和 TODO.md 文件中更新任务进度

## 3 实现TODO.md中第二个任务

### 3.1 生成第二个任务的superpowers的任务的plan文档

对话过程记录: [2026-04-24-0051-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-24-0051-caveat-the-messages-below-were-generated-by-the-u.txt)

- **不使用worktree而是在主代码库新建分支去实现TODO.md里的任务**：
  - 提示词：`不要使用worktree，在当前跟目录的代码库这里新建一个分支，然后在这个分支上
去开发`
- 又遇到了生成plan文档太长的问题，解决该问题的提示词：`@docs/superpowers/plans/2026-04-24-asr-audio-transcription.md 
这个plan文件内容太长了，你参考下 
@docs/superpowers/plans/2026-04-22-input-task-creation.md 
这个计划的风格，修改下。不要把具体的代码实现写到plan里 `

#### 补充：不使用worktree开发的原因：

- 1 为了避免新的worktree目录下的代码要再次重新安装node.js依赖，以及重新编译tuari的rust代码
- 2 避免：在功能开发完worktree合并回主代码库时，随着代码更新，又要在主代码库再来一次上面的依赖安装和rust代码编译

### 3.2 使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的Task 1、Task 2

对话过程记录: [2026-04-24-0236-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-24-0236-caveat-the-messages-below-were-generated-by-the-u.txt)

- Task 1，AI自动添加所需的 Rust 依赖时，出现了有不少依赖包版本不是最新稳定版的问题
  - 如何发现：把AI写的rust依赖配置文件内容复制发给网页端AI工具Deepseek，让它去审核是否时最新的稳定版依赖
  - 解决办法：自己通过网页端AI工具Deepseek查找出最新的稳定版本，然后直接对话告诉AI让它改

### 3.3 使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的 Task 3

对话过程记录: [](../../chat-logs/2026-04-24-1508-caveat-the-messages-below-were-generated-by-the-u.txt)

**要点内容**：

-  遇到的问题：cc完成当前 Task 3 的编码和测试后，cc想要直接完结在文档里写整个plan已完成（实际上plan文档里还有未完成的Task 4，5 。。。）：

```bash
⏺ 所有 8 个测试通过（包括 5 个云端 ASR 测试和 3 个 Whisper
  引擎测试）。让我更新 TodoWrite 标记 Task 3 完成：

───────────────────────────────────────────────────────────────────────────────
 Use skill "superpowers:finishing-a-development-branch"?
 Claude may use instructions, code, or files from this Skill.

   Use when implementation is complete, all tests pass, and you need to 
   decide how to integrate the work - guides completion of development work 
   by presenting structured options for merge, PR, or cleanup 
   (plugin:superpowers@superpowers-marketplace)

 Do you want to proceed?
 ❯ 1. Yes
  2. Yes, and don't ask again for superpowers:finishing-a-development-branch in
   /Users/a1/Codes/VidSum
   3. No, and tell Claude what to do differently
```
解决办法：**选择选型3**，然后和cc说：`@docs/superpowers/plans/2026-04-24-asr-audio-transcription.md 更新这个的进度`

> **补充**：finishing-a-development-branch 这个技能，并不会自动更新你的计划文档。它是一个Git 分支管理工作流，只处理代码和分支的最终状态，不负责文档状态同步。它被触发的时机，是 AI 认为整个计划（Plan）已全部完成的时候。

### 3.4 使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的 Task 4

> 这次开启会话前，使用了新的模型：默认deepseek-v4-flash，sonnet为GLM 5.1，opus为deepseek-v4-pro

对话过程记录: [2026-04-25-0103-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-25-0103-caveat-the-messages-below-were-generated-by-the-u.txt)

**要点内容**：

- **claude code在实现我的指令途中，突然切换了模型为sonnet代表的GLM 5.1**，cc切换模型是隐式的没有提示的

### 3.5 使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的 Task 5

> 这次任务全程使用了模型：deepseek-v4-pro

对话过程记录: [2026-04-26-0336-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-26-0336-caveat-the-messages-below-were-generated-by-the-u.txt)

**要点内容**：

- **调整plan文档，调换Task5和6的顺序**：
  - 原因：task4和task6联系紧密，且task6实现难度比较大，先实现task5会妨碍task6的实现
  - 实现的提示词：`@docs/superpowers/plans/2026-04-24-asr-audio-transcription.md 这个是我用superpowers:write-plan命令制订的plan，而且已经用superpowers:executing-plan 实现、更新这个文档中的进度以及git提交了前4个任务。但是我现在觉得接下来的任务顺序不合理，我觉得应该先把task6实现，因为task4实现后并没有真实推理过whisper模型，而且我预计task6会比较难。现在我们讨论一下要不要调整一下这个plan文档`
