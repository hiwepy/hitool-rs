//! 对齐: `cn.hutool.core.util.NumberUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/NumberUtil.java
//!
//! Rust 版本提供算术、比较、最值、解析与 BigDecimal（`rust_decimal::Decimal`）对齐实现。

use crate::{CoreError, Result};
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use std::collections::HashSet;
use std::str::FromStr;

mod number_util;
mod parsed_number;

pub use number_util::NumberUtil;
pub use parsed_number::ParsedNumber;
