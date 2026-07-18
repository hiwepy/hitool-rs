//! 对齐: `cn.hutool.core.text.finder.CharFinder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/finder/CharFinder.java

use crate::{CoreError, Result};

/// 对齐 Java: `CharFinder#`
#[derive(Debug, Clone)]
pub struct CharFinder;

impl CharFinder {
    /// 对齐 Java: `CharFinder(char c)`
    pub fn new(_c: char) -> Self {
        Self
    }

    /// 对齐 Java: `CharFinder::start#int (int)`
    pub fn start(&self, _from: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("CharFinder::start"))
    }
}