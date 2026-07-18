//! 对齐: `cn.hutool.core.text.finder.PatternFinder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/finder/PatternFinder.java

use crate::{CoreError, Result};

/// 对齐 Java: `PatternFinder#`
#[derive(Debug, Clone)]
pub struct PatternFinder;

impl PatternFinder {
    /// 对齐 Java: `PatternFinder(String regex)`
    pub fn new(_regex: &str) -> Self {
        Self
    }

    /// 对齐 Java: `PatternFinder::start#int (int)`
    pub fn start(&self, _from: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("PatternFinder::start"))
    }

    /// 对齐 Java: `PatternFinder::end#int (int)`
    pub fn end(&self, _from: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("PatternFinder::end"))
    }
}