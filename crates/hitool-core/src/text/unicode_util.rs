//! 对齐: `cn.hutool.core.text.UnicodeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/UnicodeUtil.java
//!
//! Unicode 字符串(`\\uXXXX`)与普通字符串互转。

use crate::{CoreError, Result};

/// 对齐 Java: `UnicodeUtil#`
#[derive(Debug, Clone, Copy, Default)]
pub struct UnicodeUtil;

impl UnicodeUtil {
    /// 对齐 Java: `UnicodeUtil::toString#String (String unicode)`
    pub fn to_string(_unicode: &str) -> Result<String> {
        Err(CoreError::PendingEngine("UnicodeUtil::to_string"))
    }

    /// 对齐 Java: `UnicodeUtil::toUnicode#String (char c)`
    pub fn to_unicode_char(_c: char) -> Result<String> {
        Err(CoreError::PendingEngine("UnicodeUtil::to_unicode_char"))
    }

    /// 对齐 Java: `UnicodeUtil::toUnicode#String (int c)`
    pub fn to_unicode_int(_c: i32) -> Result<String> {
        Err(CoreError::PendingEngine("UnicodeUtil::to_unicode_int"))
    }

    /// 对齐 Java: `UnicodeUtil::toUnicode#String (String str)`
    pub fn to_unicode(_str: &str) -> Result<String> {
        Err(CoreError::PendingEngine("UnicodeUtil::to_unicode"))
    }

    /// 对齐 Java: `UnicodeUtil::toUnicode#String (String str, boolean isSkipAscii)`
    pub fn to_unicode_skip_ascii(_str: &str, _skip_ascii: bool) -> Result<String> {
        Err(CoreError::PendingEngine("UnicodeUtil::to_unicode_skip_ascii"))
    }
}