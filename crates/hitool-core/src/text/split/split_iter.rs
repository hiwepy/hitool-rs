//! 对齐: `cn.hutool.core.text.split.SplitIter`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/split/SplitIter.java

use crate::{CoreError, Result};

/// 对齐 Java: `SplitIter#`
#[derive(Debug, Clone)]
pub struct SplitIter;

impl SplitIter {
    /// 对齐 Java: `SplitIter(CharSequence str, char separator)`
    pub fn new(_str: &str, _separator: char) -> Self {
        Self
    }

    /// 对齐 Java: `SplitIter::next#String ()`
    pub fn next(&mut self) -> Result<Option<String>> {
        Err(CoreError::PendingEngine("SplitIter::next"))
    }
}