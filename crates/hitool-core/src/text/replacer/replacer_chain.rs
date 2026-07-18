//! 对齐: `cn.hutool.core.text.replacer.ReplacerChain`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/replacer/ReplacerChain.java

use crate::{CoreError, Result};

/// 对齐 Java: `ReplacerChain#`
#[derive(Debug, Clone)]
pub struct ReplacerChain;

impl ReplacerChain {
    /// 对齐 Java: `ReplacerChain()`
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: `ReplacerChain::add#ReplacerChain (StrReplacer)`
    pub fn add(&mut self, _replacer: ()) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("ReplacerChain::add"))
    }

    /// 对齐 Java: `ReplacerChain::replace#String (CharSequence)`
    pub fn replace(&self, _text: &str) -> Result<String> {
        Err(CoreError::PendingEngine("ReplacerChain::replace"))
    }
}

impl Default for ReplacerChain {
    fn default() -> Self {
        Self::new()
    }
}