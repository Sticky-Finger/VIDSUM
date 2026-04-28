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

> 这次开启会话前，使用了新的模型：默认deepseek-v4-flash，sonnet为GLM 5.1，opus为deepseek-v4-pro 【之前的会话用的全是qwen3.5-plus】

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

### 3.6 使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的 Task 6

对话过程记录: [2026-04-26-1706-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-26-1706-caveat-the-messages-below-were-generated-by-the-u.txt)


**要点内容**：

- 遇到要输入图片时，切换为多模态模型去理解：
  - 运行测试软件GUI时，出现报错，于是给软件报错界面截图，使用`/model qwen3.5-plus`切换成这个多模态模型，把截图发给它然后问：`[Image #1]这上面有什么？在报什么错误？ `。
  - 接下来再`/model default`Set model to Default (deepseek-v4-flash)

### 合并plan分支feature/asr-audio-transcription到版本分支1.0-mvp/task-start-entry

> 此次任务使用的模型：开始时使用的 qwen3.5-plus，结果执行第一个指令，就选错了superpowers技能；于是后面就切换并全程时使用 deepseek-v4-flash

对话过程记录: [2026-04-26-1804-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-26-1804-caveat-the-messages-below-were-generated-by-the-u.txt)

**要点内容**：

- **新会话cc意识不到superpowers工作流已改**，还在基于git worktree行动
  - 解决办法：
    - 当前会话中直接提示cc用的superpowers工作流已改：`你已经把我当前的feature分支合并到那个1.0-mvp分支了吗？我这边其实改了工作流，没有使用worktree去跑，是直接在当前主代码库开一个分支去实现superpowers下面的plan, 具体你可以参考这里的个人记录 @docs/tutorial/Tutorial.md 的52行这个段落 `
    - 固定为项目记忆，使得下次新的会话默认使用改了的superpowers流程——让cc给项目级CLAUDE.md添加更改流程的说明内容，提示词为：`好的，你现在先帮我看一下上面那个对superpowers不使用worktree而换成branch实现的那个流程，应该怎样写到我的claude.md或者其他的全局记忆文件里面。我不想每次开一个新会话，然后用superpowers时都会让你们以为要去合并worktree或者是新建worktree之类的操作。对了，我的全局文件里面好像也没说要安装superpowers，这个要怎么标注？ `

## 4 实现TODO.md中第三个任务【字幕确认与预览】

### 4.1 生成第三个任务的superpowers的任务的plan文档

> 此次任务使用的模型：deepseek-v4-flash[1m]

对话过程记录: [2026-04-27-1609-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-27-1609-caveat-the-messages-below-were-generated-by-the-u.txt)

**要点内容**：

- 对话开始让cc实现TODO.md里面的第三个任务时，触发了superpowers:brainstorm流程，实际当前项目做过整体详细规划（在README.md、prd.md和TODO.md中），superpowers:brainstorm的项目规划流程完全多余。只需要使用 superpowers:writing-plans 给具体TODO任务写plan文档，以及之后用 superpowers:executing-plans 去实现具体TODO任务的plan文档里的task。

### 3.2 使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的Task 1~4

> 此次任务使用的模型：deepseek-v4-flash[1m]、阿里云的deepseek-v4-flash

对话过程记录: [2026-04-27-1828-superpowersexecuting-plan-docssuperpowers.txt](../../chat-logs/2026-04-27-1828-superpowersexecuting-plan-docssuperpowers.txt)

**要点内容**：

- 再次会话后期，deepseek官方的v4-flash模型出现报错：
  ```bash
  > /model 
    ⎿  Set model to deepseek-v4-flash

  > 继续上面中断的任务 
    ⎿ API Error: 400 {"error":{"message":"{\"error\":{\"message\":\"The 
      `content[].thinking` in the thinking mode must be passed back to the 
      API.\",\"type\":\"invalid_request_error\",\"param\":null,\"code\":\"invalid
      _request_error\"}}. Received Model Group=deepseek-v4-flash\nAvailable Model
      Group Fallbacks=None","type":"None","param":"None","code":"400"}}
  ```
  弄不好，于是 `/model ali-deepseek-v4-flash` 切到阿里的deepseek-v4-flash大模型服务，能够正常用下去了

### 合并plan分支版本分支1.0-mvp/task-start-entry

> 此次任务使用的模型：deepseek-v4-flash、deepseek-v4-pro

对话过程记录: [2026-04-28-1635-docssuperpowersplans2026-04-27-subtitle-previe.txt)](../../chat-logs/2026-04-28-1635-docssuperpowersplans2026-04-27-subtitle-previe.txt)

## 5 实现TODO.md中第4个任务的第一个子任务【大模型总结 > 用户配置 OpenAI 兼容 API（Base URL、API Key、Model ID）】

### 5.1 生成第三个任务的superpowers的任务的plan文档 -> 完成plan中所所有task并验收更新文档进度

> 此次任务使用的模型：小米mimo大模型平台 token plan 的 mimo-v2.5-pro 模型

对话过程记录: [2026-04-28-1911-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-28-1911-caveat-the-messages-below-were-generated-by-the-u.txt)

**要点内容**
- 在任务快要结束时，由于上下文过长，使用了 `/compact` 命令，导致了上面的‘对话过程记录’被 `/export` 出来时特别短

#### 注意：在claude code里使用了`/compact`命令后，之前的对话记录`/export`都导不出

### 合并plan分支版本分支1.0-mvp/task-start-entry，并在相关文档上更新进度

> 此次任务使用的模型：mimo-v2.5-pro

对话过程记录: [2026-04-28-2006-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-28-2006-caveat-the-messages-below-were-generated-by-the-u.txt)

**要点内容**
- 整理更新进度到 TODO.md 和 01-prd.md 上的时候，才发现这次ai实现任务plan只是**TODO中一级任务的一个子任务**

## 6 实现TODO.md中第4个任务的剩余子任务

### 6.1 生成该任务superpowers的plan文档

> 此次任务使用的模型：mimo-v2.5-pro

对话过程记录: [2026-04-28-2056-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-28-2056-caveat-the-messages-below-were-generated-by-the-u.txt)

### 6.2 使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的Task 1

> 此次任务使用的模型：mimo-v2.5-pro

对话过程记录: [2026-04-29-0028-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-29-0028-caveat-the-messages-below-were-generated-by-the-u.txt)

### 6.3 使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的Task 2~4

> 此次任务使用的模型：deepseek-v4-flash

对话过程记录: [2026-04-29-0119-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-29-0119-caveat-the-messages-below-were-generated-by-the-u.txt)

**要点内容**

- **下一个任务的实现是和当前会话在同一个会话里**