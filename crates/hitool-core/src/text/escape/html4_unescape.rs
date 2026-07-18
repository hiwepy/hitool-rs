//! 对齐: `cn.hutool.core.text.escape.Html4Unescape`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/escape/Html4Unescape.java

use crate::{CoreError, Result};

/// 对齐 Java: `Html4Unescape#`
#[derive(Debug, Clone, Copy, Default)]
pub struct Html4Unescape;

impl Html4Unescape {
    /// 对齐 Java: `Html4Unescape::unescape#String (CharSequence)`
    pub fn unescape(_text: &str) -> Result<String> {
        Err(CoreError::PendingEngine("Html4Unescape::unescape"))
    }
}