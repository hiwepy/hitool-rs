//! 对齐: `cn.hutool.core.text.finder.TextFinder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/finder/TextFinder.java

use crate::{CoreError, Result};

/// 对齐 Java: `TextFinder#`
#[derive(Debug, Clone)]
pub struct TextFinder;

impl TextFinder {
    /// 对齐 Java: `TextFinder::start#int (int)`
    pub fn start(&self, _from: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("TextFinder::start"))
    }

    /// 对齐 Java: `TextFinder::end#int (int)`
    pub fn end(&self, _from: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("TextFinder::end"))
    }
}