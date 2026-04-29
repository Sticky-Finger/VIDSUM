//! 字幕解析模块
//! 支持解析 SRT 和 VTT 格式字幕文件

pub mod parser;
pub use parser::{parse_subtitle_file, SubtitleEntry, SubtitleFormat};
