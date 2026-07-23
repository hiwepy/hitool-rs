//! 对齐: `cn.hutool.core.io.unit.DataSize`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/unit/DataSize.java
//!
//! Spring-style 数据大小值对象；解析委托 [`super::data_size_util::DataSizeUtil`]。

use super::data_size_util::DataSizeUtil;
use super::data_unit::DataUnit;
use std::cmp::Ordering;
use std::fmt;

const BYTES_PER_KB: i64 = 1024;
const BYTES_PER_MB: i64 = BYTES_PER_KB * 1024;
const BYTES_PER_GB: i64 = BYTES_PER_MB * 1024;
const BYTES_PER_TB: i64 = BYTES_PER_GB * 1024;

/// 对齐 Java 类: `cn.hutool.core.io.unit.DataSize`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataSize {
    bytes: i64,
}

impl DataSize {
    /// 对齐 Java: `DataSize.ofBytes(long)`
    pub fn of_bytes(bytes: i64) -> Self {
        Self { bytes }
    }

    /// 对齐 Java: `DataSize.ofKilobytes(long)`
    pub fn of_kilobytes(kilobytes: i64) -> Self {
        Self {
            bytes: kilobytes.saturating_mul(BYTES_PER_KB),
        }
    }

    /// 对齐 Java: `DataSize.ofMegabytes(long)`
    pub fn of_megabytes(megabytes: i64) -> Self {
        Self {
            bytes: megabytes.saturating_mul(BYTES_PER_MB),
        }
    }

    /// 对齐 Java: `DataSize.ofGigabytes(long)`
    pub fn of_gigabytes(gigabytes: i64) -> Self {
        Self {
            bytes: gigabytes.saturating_mul(BYTES_PER_GB),
        }
    }

    /// 对齐 Java: `DataSize.ofTerabytes(long)`
    pub fn of_terabytes(terabytes: i64) -> Self {
        Self {
            bytes: terabytes.saturating_mul(BYTES_PER_TB),
        }
    }

    /// 对齐 Java: `DataSize.of(long, DataUnit)`
    pub fn of(amount: i64, unit: DataUnit) -> Self {
        let mult = match unit {
            DataUnit::Bytes => 1,
            DataUnit::Kilobytes => BYTES_PER_KB,
            DataUnit::Megabytes => BYTES_PER_MB,
            DataUnit::Gigabytes => BYTES_PER_GB,
            DataUnit::Terabytes => BYTES_PER_TB,
        };
        Self {
            bytes: amount.saturating_mul(mult),
        }
    }

    /// 对齐 Java: `DataSize.of(BigDecimal, DataUnit)` — f64 金额近似。
    pub fn of_f64(amount: f64, unit: DataUnit) -> Self {
        let mult = match unit {
            DataUnit::Bytes => 1.0,
            DataUnit::Kilobytes => BYTES_PER_KB as f64,
            DataUnit::Megabytes => BYTES_PER_MB as f64,
            DataUnit::Gigabytes => BYTES_PER_GB as f64,
            DataUnit::Terabytes => BYTES_PER_TB as f64,
        };
        Self {
            bytes: (amount * mult) as i64,
        }
    }

    /// 对齐 Java: `DataSize.parse(CharSequence)`
    pub fn parse(text: &str) -> Result<Self, String> {
        DataSizeUtil::parse(text).map(Self::of_bytes)
    }

    /// 对齐 Java: `DataSize.parse(CharSequence, DataUnit)` — 缺省单位覆盖。
    pub fn parse_with_default(text: &str, default_unit: DataUnit) -> Result<Self, String> {
        let trimmed = text.trim();
        if trimmed
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_alphabetic())
            .count()
            == 0
        {
            // 纯数字：套用默认单位
            let amount: i64 = trimmed
                .parse()
                .map_err(|_| format!("'{text}' is not a valid data size"))?;
            return Ok(Self::of(amount, default_unit));
        }
        Self::parse(text)
    }

    /// 对齐 Java: `DataSize.isNegative()`
    pub fn is_negative(self) -> bool {
        self.bytes < 0
    }

    /// 对齐 Java: `DataSize.toBytes()`
    pub fn to_bytes(self) -> i64 {
        self.bytes
    }

    /// 对齐 Java: `DataSize.toKilobytes()`
    pub fn to_kilobytes(self) -> i64 {
        self.bytes / BYTES_PER_KB
    }

    /// 对齐 Java: `DataSize.toMegabytes()`
    pub fn to_megabytes(self) -> i64 {
        self.bytes / BYTES_PER_MB
    }

    /// 对齐 Java: `DataSize.toGigabytes()`
    pub fn to_gigabytes(self) -> i64 {
        self.bytes / BYTES_PER_GB
    }

    /// 对齐 Java: `DataSize.toTerabytes()`
    pub fn to_terabytes(self) -> i64 {
        self.bytes / BYTES_PER_TB
    }
}

impl PartialOrd for DataSize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DataSize {
    /// 对齐 Java: `DataSize.compareTo`
    fn cmp(&self, other: &Self) -> Ordering {
        self.bytes.cmp(&other.bytes)
    }
}

impl fmt::Display for DataSize {
    /// 对齐 Java: `DataSize.toString()` — 可读格式。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", DataSizeUtil::format(self.bytes))
    }
}
