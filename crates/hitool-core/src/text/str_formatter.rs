//! 对齐: `cn.hutool.core.text.StrFormatter`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrFormatter.java
//!
//! 字符串占位符格式化(`{}` 与 `{varName}`),支持转义。

use crate::{CoreError, Result};

/// 对齐 Java: `StrFormatter#`
#[derive(Debug, Clone, Copy, Default)]
pub struct StrFormatter;

impl StrFormatter {
    /// 对齐 Java: `StrFormatter::format#String (String strPattern, Object... argArray)`
    pub fn format(_pattern: &str, _args: &[&str]) -> Result<String> {
        Err(CoreError::PendingEngine("StrFormatter::format"))
    }

    /// 对齐 Java: `StrFormatter::formatWith#String (String strPattern, String placeHolder, Object... argArray)`
    pub fn format_with(_pattern: &str, _placeholder: &str, _args: &[&str]) -> Result<String> {
        Err(CoreError::PendingEngine("StrFormatter::format_with"))
    }

    /// 对齐 Java: `StrFormatter::format#String (CharSequence template, Map<?,?> map, boolean ignoreNull)`
    pub fn format_map(
        _template: &str,
        _entries: &[(&str, &str)],
        _ignore_null: bool,
    ) -> Result<String> {
        Err(CoreError::PendingEngine("StrFormatter::format_map"))
    }
}