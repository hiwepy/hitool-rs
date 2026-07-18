//! 对齐: `cn.hutool.core.text.escape.Html4Escape`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/escape/Html4Escape.java

use crate::{CoreError, Result};

/// 对齐 Java: `Html4Escape#`
#[derive(Debug, Clone, Copy, Default)]
pub struct Html4Escape;

impl Html4Escape {
    /// 对齐 Java: `Html4Escape::escape#String (CharSequence)`
    pub fn escape(_text: &str) -> Result<String> {
        Err(CoreError::PendingEngine("Html4Escape::escape"))
    }
}