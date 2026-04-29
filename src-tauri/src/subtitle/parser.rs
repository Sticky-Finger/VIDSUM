//! SRT/VTT 字幕文件解析器
//!
//! 纯手动解析，不依赖第三方库。
//! SRT 格式：序号 + 时间行 `HH:MM:SS,mmm --> HH:MM:SS,mmm` + 文本（空行分隔）
//! VTT 格式：`WEBVTT` 头部 + 时间行 `HH:MM:SS.mmm --> HH:MM:SS.mmm` + 文本（空行分隔）

use serde::{Deserialize, Serialize};
use std::path::Path;

/// 字幕条目（时间单位为毫秒，与前端 SubtitleEntry 兼容）
#[derive(Debug, Clone, Serialize, Deserialize)]
/// 注意：使用 camelCase 以与前端的 SubtitleEntry 接口字段名一致
#[serde(rename_all = "camelCase")]
pub struct SubtitleEntry {
    pub index: i32,
    pub text: String,
    pub start_time: i64,
    pub end_time: i64,
    pub timestamp: String,
}

/// 字幕格式
#[derive(Debug, Clone, PartialEq)]
pub enum SubtitleFormat {
    Srt,
    Vtt,
}

/// 根据文件扩展名判断字幕格式
fn detect_format(path: &Path) -> Result<SubtitleFormat, String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("srt") => Ok(SubtitleFormat::Srt),
        Some("vtt") => Ok(SubtitleFormat::Vtt),
        Some(ext) => Err(format!("不支持的字幕格式: .{}（仅支持 .srt / .vtt）", ext)),
        None => Err("无法识别文件扩展名".to_string()),
    }
}

/// 解析 VTT 时间字符串 "HH:MM:SS.mmm" 为毫秒数
fn parse_vtt_time(s: &str) -> Option<i64> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let hours: i64 = parts[0].parse().ok()?;
    let minutes: i64 = parts[1].parse().ok()?;
    let sec_parts: Vec<&str> = parts[2].split('.').collect();
    let seconds: i64 = sec_parts[0].parse().ok()?;
    let millis: i64 = if sec_parts.len() > 1 {
        let padded = format!("{:0<3}", sec_parts[1]);
        padded[..3].parse().unwrap_or(0)
    } else {
        0
    };
    Some(hours * 3_600_000 + minutes * 60_000 + seconds * 1_000 + millis)
}

/// 格式化毫秒为 HH:MM:SS（用于前端 SubtitleEntry.timestamp）
fn format_timestamp(ms: i64) -> String {
    let total_seconds = ms / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

/// 解析 SRT 时间行 "HH:MM:SS,mmm --> HH:MM:SS,mmm" 为 (start_ms, end_ms)
fn parse_srt_time_line(line: &str) -> Option<(i64, i64)> {
    let line = line.trim();
    let arrow = line.find("-->")?;
    let start_str = line[..arrow].trim();
    let end_str = line[arrow + 3..].trim();
    // SRT 使用逗号分隔毫秒，转为点号后复用 VTT 解析
    let start_str = start_str.replace(',', ".");
    let end_str = end_str.replace(',', ".");
    Some((parse_vtt_time(&start_str)?, parse_vtt_time(&end_str)?))
}

/// 解析 VTT 时间行 "HH:MM:SS.mmm --> HH:MM:SS.mmm"，可能包含尾部设置如 "A:start"
fn parse_vtt_time_line(line: &str) -> Option<(i64, i64)> {
    let line = line.trim();
    let arrow = line.find("-->")?;
    let start_str = line[..arrow].trim();
    let end_part = line[arrow + 3..].trim();
    // 去掉尾部设置（空格后的部分）
    let end_str = end_part.split_whitespace().next().unwrap_or(end_part);
    Some((parse_vtt_time(start_str)?, parse_vtt_time(end_str)?))
}

/// 解析 SRT 内容
fn parse_srt(content: &str) -> Result<Vec<SubtitleEntry>, String> {
    let mut entries = Vec::new();
    // SRT 格式：序号\n时间\n文本\n空行
    let blocks: Vec<&str> = content.split("\n\n").collect();

    for block in blocks {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        let lines: Vec<&str> = block.lines().collect();
        if lines.len() < 3 {
            continue;
        }

        // 第1行：序号
        let index: i32 = match lines[0].trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        // 第2行：时间
        let (start_time, end_time) = match parse_srt_time_line(lines[1]) {
            Some(t) => t,
            None => continue,
        };

        // 剩余行：文本
        let text = lines[2..].join("\n").trim().to_string();
        if text.is_empty() {
            continue;
        }

        entries.push(SubtitleEntry {
            index,
            text,
            start_time,
            end_time,
            timestamp: format_timestamp(start_time),
        });
    }

    if entries.is_empty() {
        return Err("未解析到任何有效字幕条目，请检查文件格式".to_string());
    }

    Ok(entries)
}

/// 解析 VTT 内容
fn parse_vtt(content: &str) -> Result<Vec<SubtitleEntry>, String> {
    let mut index_counter = 0;

    // VTT 文件以 "WEBVTT" 头部开始，跳过头部到第一个空行之后
    let body = if let Some(pos) = content.find("\n\n") {
        &content[pos + 2..]
    } else {
        content
    };

    let blocks: Vec<&str> = body.split("\n\n").collect();
    let mut entries = Vec::new();

    for block in blocks {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        let lines: Vec<&str> = block.lines().collect();
        if lines.is_empty() {
            continue;
        }

        // 跳过 STYLE 和 NOTE 块
        let first_line = lines[0].trim();
        if first_line.eq_ignore_ascii_case("STYLE") || first_line.starts_with("NOTE") {
            continue;
        }

        // 查找包含 "-->" 的时间行
        let time_line_idx = match lines.iter().position(|l| l.contains("-->")) {
            Some(idx) => idx,
            None => continue,
        };

        let (start_time, end_time) = match parse_vtt_time_line(lines[time_line_idx]) {
            Some(t) => t,
            None => continue,
        };

        // 时间行之后为文本
        let text_lines = &lines[time_line_idx + 1..];
        if text_lines.is_empty() {
            continue;
        }
        let text = text_lines.join("\n").trim().to_string();
        if text.is_empty() {
            continue;
        }

        index_counter += 1;
        entries.push(SubtitleEntry {
            index: index_counter,
            text,
            start_time,
            end_time,
            timestamp: format_timestamp(start_time),
        });
    }

    if entries.is_empty() {
        return Err("未解析到任何有效字幕条目，请检查文件格式".to_string());
    }

    Ok(entries)
}

/// 解析字幕文件入口函数
///
/// # Arguments
/// * `file_path` - 字幕文件的完整路径
///
/// # Returns
/// 解析后的字幕条目列表
pub fn parse_subtitle_file(file_path: &str) -> Result<Vec<SubtitleEntry>, String> {
    let path = Path::new(file_path);
    let format = detect_format(path)?;

    let content =
        std::fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))?;

    match format {
        SubtitleFormat::Srt => parse_srt(&content),
        SubtitleFormat::Vtt => parse_vtt(&content),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ====== 时间解析 ======

    #[test]
    fn test_parse_vtt_time_normal() {
        assert_eq!(parse_vtt_time("00:00:01.000"), Some(1000));
        assert_eq!(parse_vtt_time("00:01:30.500"), Some(90_500));
        assert_eq!(parse_vtt_time("01:00:00.000"), Some(3_600_000));
    }

    #[test]
    fn test_parse_vtt_time_short_millis() {
        // 毫秒数不足3位，补齐
        assert_eq!(parse_vtt_time("00:00:01.5"), Some(1500));
        assert_eq!(parse_vtt_time("00:00:01.50"), Some(1500));
    }

    #[test]
    fn test_parse_vtt_time_zero() {
        assert_eq!(parse_vtt_time("00:00:00.000"), Some(0));
    }

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp(0), "00:00:00");
        assert_eq!(format_timestamp(1000), "00:00:01");
        assert_eq!(format_timestamp(60_000), "00:01:00");
        assert_eq!(format_timestamp(3_661_000), "01:01:01");
    }

    // ====== 格式检测 ======

    #[test]
    fn test_detect_format_srt() {
        assert_eq!(
            detect_format(Path::new("test.srt")).unwrap(),
            SubtitleFormat::Srt
        );
    }

    #[test]
    fn test_detect_format_vtt() {
        assert_eq!(
            detect_format(Path::new("test.vtt")).unwrap(),
            SubtitleFormat::Vtt
        );
    }

    #[test]
    fn test_detect_format_unsupported() {
        assert!(detect_format(Path::new("test.txt")).is_err());
        assert!(detect_format(Path::new("test")).is_err());
    }

    // ====== SRT 解析 ======

    #[test]
    fn test_parse_srt_simple() {
        let srt = "\
1
00:00:01,000 --> 00:00:04,000
Hello world

2
00:00:05,000 --> 00:00:08,500
This is a test
";
        let entries = parse_srt(srt).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].index, 1);
        assert_eq!(entries[0].text, "Hello world");
        assert_eq!(entries[0].start_time, 1000);
        assert_eq!(entries[0].end_time, 4000);
        assert_eq!(entries[0].timestamp, "00:00:01");
        assert_eq!(entries[1].text, "This is a test");
        assert_eq!(entries[1].start_time, 5000);
        assert_eq!(entries[1].end_time, 8500);
    }

    #[test]
    fn test_parse_srt_multiline_text() {
        let srt = "\
1
00:00:01,000 --> 00:00:04,000
Line one
Line two
";
        let entries = parse_srt(srt).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].text, "Line one\nLine two");
    }

    #[test]
    fn test_parse_srt_chinese() {
        let srt = "\
1
00:00:01,500 --> 00:00:05,000
你好世界

2
00:00:06,000 --> 00:00:10,000
这是测试文本
";
        let entries = parse_srt(srt).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].text, "你好世界");
        assert_eq!(entries[1].text, "这是测试文本");
    }

    #[test]
    fn test_parse_srt_empty_result() {
        let result = parse_srt("this is not srt content");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("未解析到"));
    }

    // ====== VTT 解析 ======

    #[test]
    fn test_parse_vtt_simple() {
        let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000
Hello world

00:00:05.000 --> 00:00:08.500
This is a test
";
        let entries = parse_vtt(vtt).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].text, "Hello world");
        assert_eq!(entries[0].start_time, 1000);
        assert_eq!(entries[1].start_time, 5000);
    }

    #[test]
    fn test_parse_vtt_with_options() {
        // VTT 时间行可能包含尾部设置
        let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000 A:start
Hello world
";
        let entries = parse_vtt(vtt).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].text, "Hello world");
        assert_eq!(entries[0].start_time, 1000);
        assert_eq!(entries[0].end_time, 4000);
    }

    #[test]
    fn test_parse_vtt_skip_style_note() {
        let vtt = "\
WEBVTT

STYLE
::cue {
  color: white;
}

NOTE
This is a comment

00:00:01.000 --> 00:00:04.000
Hello world
";
        let entries = parse_vtt(vtt).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].text, "Hello world");
    }

    #[test]
    fn test_parse_vtt_empty_result() {
        let vtt = "WEBVTT\n\ninvalid content without arrow\n";
        let result = parse_vtt(vtt);
        assert!(result.is_err());
    }

    // ====== 时间行解析 ======

    #[test]
    fn test_parse_srt_time_line() {
        let (start, end) = parse_srt_time_line("00:00:01,000 --> 00:00:04,500").unwrap();
        assert_eq!(start, 1000);
        assert_eq!(end, 4500);
    }

    #[test]
    fn test_parse_vtt_time_line_with_options() {
        let (start, end) =
            parse_vtt_time_line("00:00:01.000 --> 00:00:04.500 A:start").unwrap();
        assert_eq!(start, 1000);
        assert_eq!(end, 4500);
    }

    #[test]
    fn test_parse_srt_time_line_invalid() {
        assert!(parse_srt_time_line("invalid").is_none());
    }
}
