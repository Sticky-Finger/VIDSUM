//! LLM Prompt 模板模块
//!
//! 定义视频字幕总结的默认 Prompt 模板

/// 默认系统提示
///
/// 指示 LLM 作为视频内容总结助手，输出分层 Markdown，
/// 每个段落必须包含 `[HH:MM:SS]` 时间戳锚点
pub const DEFAULT_SYSTEM_PROMPT: &str = r#"你是一个专业的视频内容总结助手。你的任务是根据用户提供的视频字幕文本，生成结构清晰、层次分明的 Markdown 总结。

## 输出要求

1. **分层结构**：使用标题层级（# ## ### 等）组织内容，体现视频的逻辑结构
2. **时间戳锚点**：每个段落或要点必须包含 `[HH:MM:SS]` 格式的时间戳，指向该内容在视频中的位置
3. **关键信息提取**：提取核心观点、重要结论、关键数据和行动项
4. **简洁准确**：保持总结简洁，避免冗余，忠实于原始内容
5. **Markdown 格式**：使用 Markdown 语法，支持标题、列表、加粗、代码块等

## 输出格式示例

```markdown
# 视频标题/主题概述

## 第一部分：[00:00:00] 引言

主要内容概述...

## 第二部分：[00:05:30] 核心观点

- 要点一 [00:06:15]
- 要点二 [00:08:45]

## 第三部分：[00:15:00] 详细分析

具体内容...

## 总结

[00:25:00] 核心结论和行动建议...
```

请根据用户提供的字幕文本生成总结。"#;

/// 默认用户提示模板
///
/// 包含 `{subtitles}` 占位符，用于插入字幕全文
pub const DEFAULT_USER_PROMPT_TEMPLATE: &str = r#"请根据以下视频字幕文本生成总结：

---

{subtitles}

---

请按照系统提示的要求，生成包含时间戳锚点的分层 Markdown 总结。"#;

/// 获取默认系统提示
pub fn get_default_system_prompt() -> String {
    DEFAULT_SYSTEM_PROMPT.to_string()
}

/// 获取默认用户提示模板
pub fn get_default_user_prompt_template() -> String {
    DEFAULT_USER_PROMPT_TEMPLATE.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_system_prompt_not_empty() {
        let prompt = get_default_system_prompt();
        assert!(!prompt.is_empty());
        assert!(prompt.contains("时间戳"));
        assert!(prompt.contains("HH:MM:SS"));
    }

    #[test]
    fn test_default_user_prompt_template_contains_placeholder() {
        let template = get_default_user_prompt_template();
        assert!(template.contains("{subtitles}"));
    }

    #[test]
    fn test_default_functions_match_constants() {
        assert_eq!(get_default_system_prompt(), DEFAULT_SYSTEM_PROMPT);
        assert_eq!(
            get_default_user_prompt_template(),
            DEFAULT_USER_PROMPT_TEMPLATE
        );
    }
}
