//! 对齐: `cn.hutool.core.text.ASCIIStrCache`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/ASCIIStrCache.java
//!
//! ASCII 字符对应的字符串缓存。Java 的静态类对应 Rust 的零大小标记类型 + 关联函数。

use crate::{CoreError, Result};

/// 对齐 Java: `ASCIIStrCache#`
#[derive(Debug, Clone, Copy, Default)]
pub struct AsciiStrCache;

impl AsciiStrCache {
    /// 对齐 Java: `ASCIIStrCache::toString#String (char c)`
    pub fn to_string(c: char) -> Result<String> {
        Err(CoreError::PendingEngine("AsciiStrCache::to_string"))
    }
}