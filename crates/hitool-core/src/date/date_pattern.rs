//! 对齐: `cn.hutool.core.date.DatePattern`
//!
//! # Timezone note
//! Hutool `DateUtilTest` 依赖 `TZ=Asia/Shanghai`。本 crate 对无时区字符串解析默认按
//! **UTC+08:00** 解释墙钟时间，以保证与 Hutool Asia/Shanghai 用例向量一致。

#![allow(dead_code)]

/// 标准日期格式：yyyy-MM-dd
pub const NORM_DATE_PATTERN: &str = "yyyy-MM-dd";
/// 标准时间格式：HH:mm:ss
pub const NORM_TIME_PATTERN: &str = "HH:mm:ss";
/// 标准日期时间：yyyy-MM-dd HH:mm:ss
pub const NORM_DATETIME_PATTERN: &str = "yyyy-MM-dd HH:mm:ss";
/// 标准日期时间（毫秒）：yyyy-MM-dd HH:mm:ss.SSS
pub const NORM_DATETIME_MS_PATTERN: &str = "yyyy-MM-dd HH:mm:ss.SSS";
/// 纯日期：yyyyMMdd
pub const PURE_DATE_PATTERN: &str = "yyyyMMdd";
/// 纯日期时间：yyyyMMddHHmmss
pub const PURE_DATETIME_PATTERN: &str = "yyyyMMddHHmmss";
/// HTTP 日期：EEE, dd MMM yyyy HH:mm:ss z
pub const HTTP_DATETIME_PATTERN: &str = "EEE, dd MMM yyyy HH:mm:ss z";
/// ISO8601：yyyy-MM-dd'T'HH:mm:ss
pub const UTC_SIMPLE_PATTERN: &str = "yyyy-MM-dd'T'HH:mm:ss";
/// 年月：yyyy-MM
pub const NORM_MONTH_PATTERN: &str = "yyyy-MM";
/// 简单年月：yyyyMM
pub const SIMPLE_MONTH_PATTERN: &str = "yyyyMM";

/// 将 Hutool/Java SimpleDateFormat 风格 pattern 转为 chrono strftime（子集）。
pub fn to_chrono_format(pattern: &str) -> String {
    let mut out = String::with_capacity(pattern.len() + 8);
    let bytes = pattern.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            '\'' => {
                // quoted literal
                i += 1;
                while i < bytes.len() {
                    if bytes[i] as char == '\'' {
                        i += 1;
                        break;
                    }
                    out.push(bytes[i] as char);
                    i += 1;
                }
            }
            'y' => {
                let start = i;
                while i < bytes.len() && bytes[i] as char == 'y' {
                    i += 1;
                }
                out.push_str(if i - start >= 4 { "%Y" } else { "%y" });
            }
            'M' => {
                let start = i;
                while i < bytes.len() && bytes[i] as char == 'M' {
                    i += 1;
                }
                let n = i - start;
                out.push_str(match n {
                    1 | 2 => "%m",
                    3 => "%b",
                    _ => "%B",
                });
            }
            'd' => {
                while i < bytes.len() && bytes[i] as char == 'd' {
                    i += 1;
                }
                out.push_str("%d");
            }
            'H' => {
                while i < bytes.len() && bytes[i] as char == 'H' {
                    i += 1;
                }
                out.push_str("%H");
            }
            'h' => {
                while i < bytes.len() && bytes[i] as char == 'h' {
                    i += 1;
                }
                out.push_str("%I");
            }
            'm' => {
                while i < bytes.len() && bytes[i] as char == 'm' {
                    i += 1;
                }
                out.push_str("%M");
            }
            's' => {
                while i < bytes.len() && bytes[i] as char == 's' {
                    i += 1;
                }
                out.push_str("%S");
            }
            'S' => {
                let start = i;
                while i < bytes.len() && bytes[i] as char == 'S' {
                    i += 1;
                }
                let n = i - start;
                // chrono: %f = nanoseconds; pad later in formatter for SSS
                out.push_str(if n >= 3 { "%.3f" } else { "%f" });
            }
            'E' => {
                let start = i;
                while i < bytes.len() && bytes[i] as char == 'E' {
                    i += 1;
                }
                out.push_str(if i - start >= 4 { "%A" } else { "%a" });
            }
            'a' => {
                i += 1;
                out.push_str("%p");
            }
            'z' | 'Z' => {
                i += 1;
                out.push_str("%z");
            }
            '%' => {
                i += 1;
                out.push_str("%%");
            }
            _ => {
                out.push(c);
                i += 1;
            }
        }
    }
    // fix "%.3f" which is not valid chrono — handled specially in format path
    out.replace("%.3f", "%f")
}

/// 对齐 Java: `DatePattern` 工具入口（常量 + formatter 工厂）。
#[derive(Debug, Clone, Copy, Default)]
pub struct DatePattern;

impl DatePattern {
    /// 对齐 Java: `DatePattern.createFormatter(String)` — 返回 chrono 可用的 strftime 子集。
    ///
    /// Java 返回 `DateTimeFormatter`；Rust 侧产出可传入 `chrono`/`DateUtil::format` 的 pattern。
    pub fn create_formatter(pattern: &str) -> String {
        to_chrono_format(pattern)
    }
}
