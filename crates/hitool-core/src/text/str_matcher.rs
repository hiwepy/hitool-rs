//! 对齐: `cn.hutool.core.text.StrMatcher`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrMatcher.java
//!
//! 字符串模式匹配,使用 `${XXXXX}` 作为变量占位符。

use crate::{CoreError, Result};

/// 对齐 Java: `StrMatcher#`
#[derive(Debug, Clone)]
pub struct StrMatcher;

impl StrMatcher {
    /// 对齐 Java: `StrMatcher(String pattern)`
    pub fn new(_pattern: &str) -> Self {
        Self
    }

    /// 对齐 Java: `StrMatcher::match#Map<String,String> (String text)`
    pub fn match_text(&self, _text: &str) -> Result<Vec<(String, String)>> {
        Err(CoreError::PendingEngine("StrMatcher::match_text"))
    }
}

impl Default for StrMatcher {
    fn default() -> Self {
        Self
    }
}