//! 对齐: `cn.hutool.core.text.replacer.StrReplacer`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/replacer/StrReplacer.java

use crate::{CoreError, Result};

/// 对齐 Java: `StrReplacer#`
#[derive(Debug, Clone)]
pub struct StrReplacer;

impl StrReplacer {
    /// 对齐 Java: `StrReplacer::replace#String (CharSequence)`
    pub fn replace(&self, _text: &str) -> Result<String> {
        Err(CoreError::PendingEngine("StrReplacer::replace"))
    }
}