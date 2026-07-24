//! 对齐: `cn.hutool.core.lang.PatternPool`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/PatternPool.java
//!
//! 编译正则缓存；flags 对齐 Java `Pattern` 位掩码的常用子集（CASE_INSENSITIVE=2）。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;
use regex::{Regex, RegexBuilder};

use super::regex_pool::RegexPool;

mod regex_with_flag;
mod pattern_pool;

pub use regex_with_flag::RegexWithFlag;
pub use pattern_pool::PatternPool;
pub use regex_with_flag::FLAG_CASE_INSENSITIVE;
