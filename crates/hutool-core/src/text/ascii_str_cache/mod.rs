//! 对齐: `cn.hutool.core.text.ASCIIStrCache`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/ASCIIStrCache.java
//!
//! ASCII 字符对应的字符串缓存。Java 的静态类对应 Rust 的零大小标记类型 + 关联函数。

use std::sync::OnceLock;

use crate::Result;

mod ascii_str_cache;
mod ascii_str_cache;

pub use ascii_str_cache::AsciiStrCache;
pub use ascii_str_cache::ASCIIStrCache;
