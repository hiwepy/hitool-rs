//! 对齐: `cn.hutool.core.text.finder.LengthFinder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/finder/LengthFinder.java

use crate::{CoreError, Result};

/// 对齐 Java: `LengthFinder#`
#[derive(Debug, Clone, Copy)]
pub struct LengthFinder;

impl LengthFinder {
    /// 对齐 Java: `LengthFinder(int length)`
    pub fn new(_length: i32) -> Self {
        Self
    }

    /// 对齐 Java: `LengthFinder::start#int (int)`
    pub fn start(&self, _from: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("LengthFinder::start"))
    }

    /// 对齐 Java: `LengthFinder::end#int (int)`
    pub fn end(&self, _from: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("LengthFinder::end"))
    }
}