//! 对齐: `cn.hutool.core.text.escape.XmlEscape`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/escape/XmlEscape.java

use crate::{CoreError, Result};

/// 对齐 Java: `XmlEscape#`
#[derive(Debug, Clone, Copy, Default)]
pub struct XmlEscape;

impl XmlEscape {
    /// 对齐 Java: `XmlEscape::escape#String (CharSequence)`
    pub fn escape(_text: &str) -> Result<String> {
        Err(CoreError::PendingEngine("XmlEscape::escape"))
    }
}