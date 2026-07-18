//! 对齐: `cn.hutool.core.text.escape.NumericEntityUnescaper`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/escape/NumericEntityUnescaper.java

use crate::{CoreError, Result};

/// 对齐 Java: `NumericEntityUnescaper#`
#[derive(Debug, Clone, Copy, Default)]
pub struct NumericEntityUnescaper;

impl NumericEntityUnescaper {
    /// 对齐 Java: `NumericEntityUnescaper::replace#String (CharSequence)`
    pub fn replace_text(&self, _text: &str) -> Result<String> {
        Err(CoreError::PendingEngine(
            "NumericEntityUnescaper::replace_text",
        ))
    }
}