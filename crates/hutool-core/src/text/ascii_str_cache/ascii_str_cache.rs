//! 对齐: `cn.hutool.core.text.ASCIIStrCache`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/ASCIIStrCache.java
//!
//! ASCII 字符对应的字符串缓存。Java 的静态类对应 Rust 的零大小标记类型 + 关联函数。

use std::sync::OnceLock;

use crate::Result;

use super::ascii_str_cache::AsciiStrCache;

/// Java 类名别名（`ASCIIStrCache`）。
pub type ASCIIStrCache = AsciiStrCache;
