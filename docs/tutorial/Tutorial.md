# 过程记录

## 1 初始化项目时，写项目级文档
### 1.1 和AI讨论需求得到精炼的README.md
### 1.2 和AI讨论得到MVP产品文档01-prd.md
### 1.3 和AI讨论并将01-prd.md中需求写到TODO.md

## 2.开始实现TODO.md中第一个任务，生成该任务的superpowers的任务的plan文档

对话过程记录: [2026-04-23-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-23-caveat-the-messages-below-were-generated-by-the-u.txt)

期间cc自动调用了插件命令 superpowers:writing-plans，生成了任务的plan文档 [2026-04-22-input-task-creation.md](../superpowers/plans/2026-04-22-input-task-creation.md)

### 补充：开始生成plan文档时遇到了生成的文档达到1k多行过于冗长的问题

**解决办法**-提示词：

```bash
你看下你用'/superpowers:write-plan'写出来的 
@docs/superpowers/plans/2026-04-22-input-task-creation.md 
，有什么问题？我觉得这个plan文档也太长了 
```
然后，ai就提供了一个精简缩短版本的plan文档内容

## 3.使用 superpowers:excuting-plan 写代码实现上述plan文档内容里的Task 1

对话过程记录：[2026-04-23-1503-caveat-the-messages-below-were-generated-by-the-u.txt](../../chat-logs/2026-04-23-1503-caveat-the-messages-below-were-generated-by-the-u.txt)

- 让ai写代码实现Task 1的提示词：`superpowers:executing-plan 
@docs/superpowers/plans/2026-04-22-input-task-creation.md `
- 实现期间 superpowers 用 superpowers:using-git-worktrees
  创建隔离的工作空间
- ai自动测试 pnpm tauri dev 是否能启动时，出现了问题且ai自己一直解决不了卡住了。此时手动停止ai运行的 pnpm tauri dev 命令，并且自己手动运行测试和解决问题直至运行成功
- 手动测试调试debug成功后，告诉AI“我本地手动调试并运行验证了task 1，你查看下”，给AI发消息让它更新plan文档中task 1的进度，并且把项目源码git提交：
    - 提示词：`完成了task1，先更新superpowers生成的相关plan文档 @.worktrees/input-task-c
    reation/docs/superpowers/plans/2026-04-22-input-task-creation.md 标明进度，并且git提交当前代码文件么？`