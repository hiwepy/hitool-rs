//! 对齐: `cn.hutool.core.text.finder.CharMatcherFinder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/finder/CharMatcherFinder.java

use crate::{CoreError, Result};

/// 对齐 Java: `CharMatcherFinder#`
#[derive(Debug, Clone)]
pub struct CharMatcherFinder;

impl CharMatcherFinder {
    /// 对齐 Java: `CharMatcherFinder(CharMatcher matcher)`
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: `CharMatcherFinder::start#int (int)`
    pub fn start(&self, _from: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("CharMatcherFinder::start"))
    }
}

impl Default for CharMatcherFinder {
    fn default() -> Self {
        Self::new()
    }
}