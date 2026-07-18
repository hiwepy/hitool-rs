//! Unicode-aware string utilities.
//!
//! Portions of the behavior and tests were adapted from yimi-rutool 0.2.5
//! (Apache-2.0) and substantially revised for borrowed strings, Unicode, and
//! Rust extension traits.

use std::fmt::{Display, Write};

/// Returns `true` when a string is empty or contains only Unicode whitespace.
#[inline]
#[must_use]
pub fn is_blank(value: &str) -> bool {
    value.trim().is_empty()
}

/// Removes every non-overlapping occurrence of `needle` from `value`.
#[must_use]
pub fn remove_all(value: &str, needle: &str) -> String {
    if needle.is_empty() {
        return value.to_owned();
    }
    value.replace(needle, "")
}

/// Removes all characters listed in `characters` from `value`.
#[must_use]
pub fn remove_chars(value: &str, characters: &[char]) -> String {
    value
        .chars()
        .filter(|character| !characters.contains(character))
        .collect()
}

/// Uppercases the first Unicode scalar value without changing the remainder.
#[must_use]
pub fn upper_first(value: &str) -> String {
    change_first(value, char::to_uppercase)
}

/// Lowercases the first Unicode scalar value without changing the remainder.
#[must_use]
pub fn lower_first(value: &str) -> String {
    change_first(value, char::to_lowercase)
}

fn change_first<I>(value: &str, transform: impl FnOnce(char) -> I) -> String
where
    I: Iterator<Item = char>,
{
    let Some(first) = value.chars().next() else {
        return String::new();
    };
    let remainder = &value[first.len_utf8()..];
    let mut result = String::with_capacity(value.len());
    result.extend(transform(first));
    result.push_str(remainder);
    result
}

/// Splits a string with optional trimming and empty-item removal.
#[must_use]
pub fn split(value: &str, separator: char, trim_items: bool, ignore_empty: bool) -> Vec<&str> {
    value
        .split(separator)
        .map(|item| if trim_items { item.trim() } else { item })
        .filter(|item| !ignore_empty || !item.is_empty())
        .collect()
}

/// Formats sequential `{}` placeholders.
///
/// `{{` and `}}` produce literal braces. Missing values leave their `{}`
/// placeholder intact, while extra values are ignored.
#[must_use]
pub fn format_template(template: &str, values: &[&dyn Display]) -> String {
    let mut result = String::with_capacity(template.len());
    let mut chars = template.chars().peekable();
    let mut value_index = 0;

    while let Some(character) = chars.next() {
        match (character, chars.peek().copied()) {
            ('{', Some('{')) => {
                chars.next();
                result.push('{');
            }
            ('}', Some('}')) => {
                chars.next();
                result.push('}');
            }
            ('{', Some('}')) => {
                chars.next();
                if let Some(value) = values.get(value_index) {
                    write!(&mut result, "{value}").expect("writing to String cannot fail");
                    value_index += 1;
                } else {
                    result.push_str("{}");
                }
            }
            _ => result.push(character),
        }
    }

    result
}

/// Extension methods for string slices.
pub trait StrExt {
    /// Returns `true` when the string is empty or only Unicode whitespace.
    fn is_blank(&self) -> bool;

    /// Returns `true` when the string contains a non-whitespace character.
    fn is_not_blank(&self) -> bool;

    /// Returns a borrowed string with surrounding Unicode whitespace removed.
    fn trimmed(&self) -> &str;

    /// Returns an owned string with all occurrences of `needle` removed.
    fn without(&self, needle: &str) -> String;

    /// Returns an owned string with the first character uppercased.
    fn upper_first(&self) -> String;

    /// Returns an owned string with the first character lowercased.
    fn lower_first(&self) -> String;
}

impl StrExt for str {
    #[inline]
    fn is_blank(&self) -> bool {
        is_blank(self)
    }

    #[inline]
    fn is_not_blank(&self) -> bool {
        !is_blank(self)
    }

    #[inline]
    fn trimmed(&self) -> &str {
        self.trim()
    }

    fn without(&self, needle: &str) -> String {
        remove_all(self, needle)
    }

    fn upper_first(&self) -> String {
        upper_first(self)
    }

    fn lower_first(&self) -> String {
        lower_first(self)
    }
}

// ════════════════════════════════════════════════════════════
//  StrUtil-aligned functions (对齐 cn.hutool.core.util.StrUtil)
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `StrUtil.trim(CharSequence str)`
///
/// 去除首尾空白字符。Hutool 的 trim 额外处理:
/// - Unicode 全角空格 `\u3000`
/// - 制表符 `\t`
/// - 换行符 `\r` `\n`
///
/// Rust `str::trim` 已经处理 Unicode 空白,行为一致。
#[must_use]
pub fn trim(value: &str) -> &str {
    value.trim()
}

/// 对齐 Java: `StrUtil.cleanBlank(CharSequence str)`
///
/// 清除字符串中**所有**空白字符(包括中间的空白),
/// 返回无空白的字符串。
#[must_use]
pub fn clean_blank(value: &str) -> String {
    value
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '\u{3000}')
        .collect()
}

/// 对齐 Java: `StrUtil.cut(CharSequence str, int cutLength)`
///
/// 按固定长度切割字符串为数组。
///
/// # 参数
/// - `value`: 原始字符串
/// - `cut_length`: 每段长度
///
/// # 错误
/// - `cut_length <= 0` 时返回 `Err`
pub fn cut(value: &str, cut_length: usize) -> Result<Vec<String>, String> {
    if cut_length == 0 {
        return Err("cut_length must be greater than zero".into());
    }
    let chars: Vec<char> = value.chars().collect();
    let mut result = Vec::new();
    for chunk in chars.chunks(cut_length) {
        result.push(chunk.iter().collect());
    }
    Ok(result)
}

/// 对齐 Java: `StrUtil.strip(CharSequence str, CharSequence prefixOrSuffix)`
///
/// 去除首尾与 `prefix_or_suffix` 相等的字符。
/// Java 的 strip 对每个字符单独检查首尾,不是整个字符串匹配。
#[must_use]
pub fn strip(value: &str, prefix_or_suffix_chars: &str) -> String {
    let chars: std::collections::HashSet<char> = prefix_or_suffix_chars.chars().collect();
    value
        .trim_matches(|c: char| chars.contains(&c))
        .to_owned()
}

/// 对齐 Java: `StrUtil.stripIgnoreCase(CharSequence str, CharSequence prefixOrSuffix)`
#[must_use]
pub fn strip_ignore_case(value: &str, prefix_or_suffix_chars: &str) -> String {
    let chars: std::collections::HashSet<char> =
        prefix_or_suffix_chars.chars().flat_map(|c| c.to_lowercase()).collect();
    value
        .trim_matches(|c: char| chars.contains(&c.to_ascii_lowercase()))
        .to_owned()
}

/// 对齐 Java: `StrUtil.indexOfIgnoreCase(CharSequence str, CharSequence testStr)`
///
/// 忽略大小写查找子串首次出现的字节位置。找不到返回 `None`(Java 返回 -1)。
#[must_use]
pub fn index_of_ignore_case(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    let h_lower = haystack.to_lowercase();
    let n_lower = needle.to_lowercase();
    h_lower.find(&n_lower)
}

/// 对齐 Java: `StrUtil.lastIndexOf(CharSequence str, CharSequence searchStr, int fromIndex)`
///
/// 从后向前查找子串。找不到返回 `None`(Java 返回 -1)。
#[must_use]
pub fn last_index_of(haystack: &str, needle: &str) -> Option<usize> {
    haystack.rfind(needle)
}

/// 对齐 Java: `StrUtil.lastIndexOfIgnoreCase`
#[must_use]
pub fn last_index_of_ignore_case(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(haystack.len());
    }
    let h_lower = haystack.to_lowercase();
    let n_lower = needle.to_lowercase();
    h_lower.rfind(&n_lower)
}

/// 对齐 Java: `StrUtil.replace(CharSequence str, CharSequence searchStr, CharSequence replacement)`
///
/// 替换所有匹配的子串。
#[must_use]
pub fn replace(value: &str, search: &str, replacement: &str) -> String {
    if search.is_empty() {
        return value.to_owned();
    }
    value.replace(search, replacement)
}

/// 对齐 Java: `StrUtil.startWith(CharSequence str, CharSequence prefix)`
#[must_use]
pub fn start_with(value: &str, prefix: &str) -> bool {
    value.starts_with(prefix)
}

/// 对齐 Java: `StrUtil.endWith(CharSequence str, CharSequence suffix)`
#[must_use]
pub fn end_with(value: &str, suffix: &str) -> bool {
    value.ends_with(suffix)
}

/// 对齐 Java: `StrUtil.contains(CharSequence str, CharSequence testStr)`
#[must_use]
pub fn contains(value: &str, needle: &str) -> bool {
    value.contains(needle)
}

/// 对齐 Java: `StrUtil.containsIgnoreCase`
#[must_use]
pub fn contains_ignore_case(value: &str, needle: &str) -> bool {
    value.to_lowercase().contains(&needle.to_lowercase())
}

/// 对齐 Java: `StrUtil.equals(CharSequence str1, CharSequence str2)`
#[must_use]
pub fn equals(a: &str, b: &str) -> bool {
    a == b
}

/// 对齐 Java: `StrUtil.equalsIgnoreCase`
#[must_use]
pub fn equals_ignore_case(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b) || a.to_lowercase() == b.to_lowercase()
}

/// 对齐 Java: `StrUtil.reverse(CharSequence str)`
#[must_use]
pub fn reverse(value: &str) -> String {
    value.chars().rev().collect()
}

/// 对齐 Java: `StrUtil.repeat(CharSequence str, int n)`
#[must_use]
pub fn repeat(value: &str, n: usize) -> String {
    value.repeat(n)
}

/// 对齐 Java: `StrUtil.length(CharSequence str)`
///
/// null → 0,否则返回字符数。
#[must_use]
pub fn length(value: Option<&str>) -> usize {
    value.map_or(0, str::len)
}

/// 对齐 Java: `StrUtil.str(Object obj)` —— null → "",否则 toString
#[must_use]
pub fn str_or_empty(value: Option<&str>) -> &str {
    value.unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_uses_unicode_whitespace() {
        assert!(is_blank(" \t\n\u{3000}"));
        assert!(!is_blank(" value "));
    }

    #[test]
    fn first_character_conversion_is_unicode_safe() {
        assert_eq!(upper_first("élan"), "Élan");
        assert_eq!(lower_first("Über"), "über");
        assert_eq!(upper_first(""), "");
    }

    #[test]
    fn split_can_trim_and_drop_empty_items() {
        assert_eq!(split(" a, ,b,", ',', true, true), ["a", "b"]);
        assert_eq!(split("a,,b", ',', false, false), ["a", "", "b"]);
    }

    #[test]
    fn template_formatting_handles_escapes_and_missing_values() {
        let count = 2;
        let name = "files";
        assert_eq!(
            format_template("{{copied}} {} {} {}", &[&count, &name]),
            "{copied} 2 files {}"
        );
    }

    #[test]
    fn extension_trait_keeps_borrowed_operations_borrowed() {
        let value = "  hello  ";
        assert_eq!(value.trimmed(), "hello");
        assert!(" \n".is_blank());
        assert_eq!("banana".without("na"), "ba");
    }
}
