//! 对齐: `cn.hutool.core.util.EscapeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/EscapeUtil.java
//!
//! Rust 版本提供字符转义的 idiomatic 实现。

/// 对齐 Java: `cn.hutool.core.util.EscapeUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct EscapeUtil;

impl EscapeUtil {
    // ── HTML 转义 ──

    /// 对齐 Java: `EscapeUtil.escapeHtml(CharSequence)`
    pub fn escape_html(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                '\'' => result.push_str("&#39;"),
                _ => result.push(c),
            }
        }
        result
    }

    /// 对齐 Java: `EscapeUtil.unescapeHtml(CharSequence)`
    pub fn unescape_html(input: &str) -> String {
        input
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
    }

    // ── XML 转义 ──

    /// 对齐 Java: `EscapeUtil.escapeXml(CharSequence)`
    pub fn escape_xml(input: &str) -> String {
        Self::escape_html(input)
    }

    /// 对齐 Java: `EscapeUtil.unescapeXml(CharSequence)`
    pub fn unescape_xml(input: &str) -> String {
        Self::unescape_html(input)
    }

    // ── Java/JavaScript 转义 ──

    /// 对齐 Java: `EscapeUtil.escapeJava(CharSequence)`
    pub fn escape_java(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),
                '\\' => result.push_str("\\\\"),
                '"' => result.push_str("\\\""),
                '\'' => result.push_str("\\'"),
                _ => result.push(c),
            }
        }
        result
    }

    /// 对齐 Java: `EscapeUtil.unescapeJava(CharSequence)`
    pub fn unescape_java(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('t') => result.push('\t'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('\'') => result.push('\''),
                    Some(other) => {
                        result.push('\\');
                        result.push(other);
                    }
                    None => result.push('\\'),
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    // ── SQL 转义 ──

    /// 对齐 Java: `EscapeUtil.escapeSql(CharSequence)`
    pub fn escape_sql(input: &str) -> String {
        input.replace('\'', "''")
    }

    // ── 通用转义 ──

    /// 对齐 Java: `EscapeUtil.escape(CharSequence)`
    pub fn escape(input: &str) -> String {
        Self::escape_html(input)
    }

    /// 对齐 Java: `EscapeUtil.unescape(CharSequence)`
    pub fn unescape(input: &str) -> String {
        Self::unescape_html(input)
    }
}
