//! 对齐: `cn.hutool.core.util.ReUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ReUtil.java
//!
//! Rust 版本使用 `regex` crate 提供正则表达式工具。

use regex::Regex;

/// 对齐 Java: `cn.hutool.core.util.ReUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ReUtil;

impl ReUtil {
    // ── 匹配判断 ──

    /// 对齐 Java: `ReUtil.isMatch(Pattern, CharSequence)`
    pub fn is_match(pattern: &str, text: &str) -> bool {
        Regex::new(pattern).map_or(false, |re| re.is_match(text))
    }

    /// 对齐 Java: `ReUtil.isMatch(String, CharSequence)`
    pub fn is_match_str(pattern: &str, text: &str) -> bool {
        Self::is_match(pattern, text)
    }

    // ── 提取操作 ──

    /// 对齐 Java: `ReUtil.get(Pattern, CharSequence, int)`
    pub fn find(pattern: &str, text: &str) -> Option<String> {
        Regex::new(pattern)
            .ok()?
            .find(text)
            .map(|m| m.as_str().to_string())
    }

    /// 对齐 Java: `ReUtil.getGroup(Pattern, CharSequence, int)`
    pub fn group(pattern: &str, text: &str, group: usize) -> Option<String> {
        Regex::new(pattern)
            .ok()?
            .captures(text)?
            .get(group)
            .map(|m| m.as_str().to_string())
    }

    /// 对齐 Java: `ReUtil.getGroup0(Pattern, CharSequence)`
    pub fn group0(pattern: &str, text: &str) -> Option<String> {
        Self::group(pattern, text, 0)
    }

    /// 对齐 Java: `ReUtil.getGroup1(Pattern, CharSequence)`
    pub fn group1(pattern: &str, text: &str) -> Option<String> {
        Self::group(pattern, text, 1)
    }

    // ── 提取所有匹配 ──

    /// 对齐 Java: `ReUtil.findAll(Pattern, CharSequence, int)`
    pub fn find_all(pattern: &str, text: &str) -> Vec<String> {
        Regex::new(pattern)
            .map(|re| re.find_iter(text).map(|m| m.as_str().to_string()).collect())
            .unwrap_or_default()
    }

    /// 对齐 Java: `ReUtil.findAllGroup(Pattern, CharSequence, int)`
    pub fn find_all_groups(pattern: &str, text: &str, group: usize) -> Vec<String> {
        Regex::new(pattern)
            .map(|re| {
                re.captures_iter(text)
                    .filter_map(|caps| caps.get(group).map(|m| m.as_str().to_string()))
                    .collect()
            })
            .unwrap_or_default()
    }

    // ── 替换操作 ──

    /// 对齐 Java: `ReUtil.replaceAll(CharSequence, Pattern, String)`
    pub fn replace_all(pattern: &str, text: &str, replacement: &str) -> String {
        Regex::new(pattern)
            .map(|re| re.replace_all(text, replacement).into_owned())
            .unwrap_or_else(|_| text.to_string())
    }

    /// 对齐 Java: `ReUtil.replaceFirst(CharSequence, Pattern, String)`
    pub fn replace_first(pattern: &str, text: &str, replacement: &str) -> String {
        Regex::new(pattern)
            .map(|re| re.replace(text, replacement).into_owned())
            .unwrap_or_else(|_| text.to_string())
    }

    // ── 分割操作 ──

    /// 对齐 Java: `ReUtil.split(CharSequence, Pattern)`
    pub fn split(pattern: &str, text: &str) -> Vec<String> {
        Regex::new(pattern)
            .map(|re| re.split(text).map(String::from).collect())
            .unwrap_or_else(|_| vec![text.to_string()])
    }

    // ── 常用正则 ──

    /// 对齐 Java: `ReUtil.isEmail(CharSequence)`
    pub fn is_email(text: &str) -> bool {
        Self::is_match(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$", text)
    }

    /// 对齐 Java: `ReUtil.isIpv4(CharSequence)`
    pub fn is_ipv4(text: &str) -> bool {
        Self::is_match(r"^(\d{1,3}\.){3}\d{1,3}$", text)
    }

    /// 对齐 Java: `ReUtil.isIpv6(CharSequence)`
    pub fn is_ipv6(text: &str) -> bool {
        Self::is_match(r"^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$", text)
    }

    /// 对齐 Java: `ReUtil.isUrl(CharSequence)`
    pub fn is_url(text: &str) -> bool {
        Self::is_match(r"^https?://[^\s/$.?#].[^\s]*$", text)
    }

    /// 对齐 Java: `ReUtil.isChinese(CharSequence)`
    pub fn is_chinese(text: &str) -> bool {
        Self::is_match(r"^[\u4e00-\u9fa5]+$", text)
    }

    /// 对齐 Java: `ReUtil.isMobile(CharSequence)`
    pub fn is_mobile(text: &str) -> bool {
        Self::is_match(r"^1[3-9]\d{9}$", text)
    }

    /// 对齐 Java: `ReUtil.isIdCard(CharSequence)`
    pub fn is_id_card(text: &str) -> bool {
        Self::is_match(r"^\d{17}[\dX]$", text)
    }

    // ── 提取数字 ──

    /// 提取字符串中的第一个数字
    pub fn extract_number(text: &str) -> Option<i64> {
        Self::find(r"\d+", text)?.parse().ok()
    }

    /// 提取字符串中的所有数字
    pub fn extract_numbers(text: &str) -> Vec<i64> {
        Self::find_all(r"\d+", text)
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect()
    }

    // ── 转义 ──

    /// 对齐 Java: `ReUtil.escape(CharSequence)`
    pub fn escape_special(text: &str) -> String {
        let mut result = String::with_capacity(text.len() * 2);
        for c in text.chars() {
            if r"[\^$.|?*+(){}\\".contains(c) {
                result.push('\\');
            }
            result.push(c);
        }
        result
    }
}
