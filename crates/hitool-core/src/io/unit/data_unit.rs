//! 对齐: `cn.hutool.core.io.unit.DataUnit`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/unit/DataUnit.java

/// 对齐 Java enum: `cn.hutool.core.io.unit.DataUnit`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataUnit {
    /// 对齐 Java 枚举常量: `BYTES`
    Bytes,
    /// 对齐 Java 枚举常量: `KILOBYTES`
    Kilobytes,
    /// 对齐 Java 枚举常量: `MEGABYTES`
    Megabytes,
    /// 对齐 Java 枚举常量: `GIGABYTES`
    Gigabytes,
    /// 对齐 Java 枚举常量: `TERABYTES`
    Terabytes,
}

impl DataUnit {
    /// 对齐 Java: `DataUnit.UNIT_NAMES`
    pub const UNIT_NAMES: [&'static str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];

    /// 对齐 Java: `DataUnit.getSuffix()`
    pub fn suffix(self) -> &'static str {
        match self {
            Self::Bytes => "B",
            Self::Kilobytes => "KB",
            Self::Megabytes => "MB",
            Self::Gigabytes => "GB",
            Self::Terabytes => "TB",
        }
    }

    /// 对齐 Java: `DataUnit.fromSuffix(String)`
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        match suffix.trim().to_ascii_uppercase().as_str() {
            "B" | "BYTE" | "BYTES" => Some(Self::Bytes),
            "K" | "KB" | "KIB" | "KILOBYTE" | "KILOBYTES" => Some(Self::Kilobytes),
            "M" | "MB" | "MIB" | "MEGABYTE" | "MEGABYTES" => Some(Self::Megabytes),
            "G" | "GB" | "GIB" | "GIGABYTE" | "GIGABYTES" => Some(Self::Gigabytes),
            "T" | "TB" | "TIB" | "TERABYTE" | "TERABYTES" => Some(Self::Terabytes),
            _ => None,
        }
    }

    /// 对齐 Java: `ArrayUtil.indexOf(DataUnit.UNIT_NAMES, suffix)` 的索引。
    pub fn digit_group(self) -> usize {
        match self {
            Self::Bytes => 0,
            Self::Kilobytes => 1,
            Self::Megabytes => 2,
            Self::Gigabytes => 3,
            Self::Terabytes => 4,
        }
    }
}
