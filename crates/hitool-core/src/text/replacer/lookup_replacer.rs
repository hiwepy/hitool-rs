//! 对齐: `cn.hutool.core.text.replacer.LookupReplacer`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/replacer/LookupReplacer.java

use crate::{CoreError, Result};

/// 对齐 Java: `LookupReplacer#`
#[derive(Debug, Clone)]
pub struct LookupReplacer;

impl LookupReplacer {
    /// 对齐 Java: `LookupReplacer(String[] lookup)`
    pub fn new(_lookup: &[&str]) -> Self {
        Self
    }

    /// 对齐 Java: `LookupReplacer::replace#String (CharSequence)`
    pub fn replace(&self, _text: &str) -> Result<String> {
        Err(CoreError::PendingEngine("LookupReplacer::replace"))
    }
}