//! 对齐: `cn.hutool.core.text.finder.StrFinder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/finder/StrFinder.java

use crate::{CoreError, Result};

/// 对齐 Java: `StrFinder#`
#[derive(Debug, Clone)]
pub struct StrFinder;

impl StrFinder {
    /// 对齐 Java: `StrFinder(String strToFind)`
    pub fn new(_str: &str) -> Self {
        Self
    }

    /// 对齐 Java: `StrFinder::start#int (int)`
    pub fn start(&self, _from: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("StrFinder::start"))
    }

    /// 对齐 Java: `StrFinder::end#int (int)`
    pub fn end(&self, _from: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("StrFinder::end"))
    }
}