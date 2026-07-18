//! 对齐: `cn.hutool.core.text.escape.XmlUnescape`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/escape/XmlUnescape.java

use crate::{CoreError, Result};

/// 对齐 Java: `XmlUnescape#`
#[derive(Debug, Clone, Copy, Default)]
pub struct XmlUnescape;

impl XmlUnescape {
    /// 对齐 Java: `XmlUnescape::unescape#String (CharSequence)`
    pub fn unescape(_text: &str) -> Result<String> {
        Err(CoreError::PendingEngine("XmlUnescape::unescape"))
    }
}