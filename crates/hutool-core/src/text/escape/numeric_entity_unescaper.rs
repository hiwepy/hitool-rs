//! 对齐: `cn.hutool.core.text.escape.NumericEntityUnescaper`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/escape/NumericEntityUnescaper.java
//!
//! 反转义形如 `&#39;` / `&#x27;` 的数值实体。

use crate::Result;

/// 对齐 Java: `NumericEntityUnescaper#`
#[derive(Debug, Clone, Copy, Default)]
pub struct NumericEntityUnescaper;

impl NumericEntityUnescaper {
    /// 对齐 Java: `NumericEntityUnescaper()` — 构造数值实体反转义器。
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: `StrReplacer.replace(CharSequence)` — 全串扫描替换。
    pub fn replace_text(&self, text: &str) -> Result<String> {
        let chars: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(text.len());
        let mut i = 0usize;
        while i < chars.len() {
            if let Some((ch, consumed)) = Self::try_entity(&chars, i) {
                out.push(ch);
                i += consumed;
            } else {
                out.push(chars[i]);
                i += 1;
            }
        }
        Ok(out)
    }

    /// 尝试从 `pos` 解析 `&#...;` 实体；成功返回 `(字符, 消耗码点数)`。
    fn try_entity(chars: &[char], pos: usize) -> Option<(char, usize)> {
        let len = chars.len();
        if chars[pos] != '&' || pos + 2 >= len || chars[pos + 1] != '#' {
            return None;
        }
        let mut start = pos + 2;
        let mut is_hex = false;
        if start < len && (chars[start] == 'x' || chars[start] == 'X') {
            start += 1;
            is_hex = true;
        }
        if start >= len {
            return None;
        }
        let mut end = start;
        while end < len && Self::is_hex_digit(chars[end]) {
            // 十进制实体也允许 0-9；十六进制允许 a-f
            if !is_hex && !chars[end].is_ascii_digit() {
                break;
            }
            end += 1;
        }
        if end >= len || chars[end] != ';' || end == start {
            return None;
        }
        let digits: String = chars[start..end].iter().collect();
        let value = if is_hex {
            u32::from_str_radix(&digits, 16).ok()?
        } else {
            digits.parse::<u32>().ok()?
        };
        let ch = char::from_u32(value)?;
        let consumed = end - pos + 1;
        Some((ch, consumed))
    }

    /// 十六进制字符判断（对齐 Hutool `CharUtil.isHexChar`）。
    fn is_hex_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }
}
