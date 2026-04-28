## 开发工作流

本项目采用以下AI编程协作流程：

1. **需求与设计**（不在 Claude Code 中进行）  
   - 在网页端 DeepSeek 等AI聊天工具完成技术调研、PRD 讨论
   - 最终 PRD 维护在 `docs/PRD.md`

2. **任务拆解**  
   - 使用 `openspec` 等spec工具或者手动根据 PRD 制作具体的 TODO 清单（`docs/TODO.md`）

3. **编码实现**（在 Claude Code 中）  
   - 仅处理 `docs/TODO.md` 中未完成的任务
   - 使用 `superpowers` 的 `writing-plans` + `executing-plan` 技能，按原子步骤实现并标记进度 
   - 详细规范见 `CLAUDE.md`

4. **进度跟踪**  
   - 每个任务的实现计划存放在 `docs/superpowers/plans/`
   - 计划文件中的复选框记录了完成情况，支持任务中断后重新开始
   - 完成 `docs/superpowers/plans/`里的任务后，会更新 TODO.md 和 PRD.md 中相关的任务进度