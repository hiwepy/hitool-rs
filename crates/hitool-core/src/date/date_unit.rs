//! 对齐: `cn.hutool.core.date.DateUnit`

#![allow(dead_code)]

/// 对齐 Java: `cn.hutool.core.date.DateUnit`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateUnit {
    /// 毫秒
    Ms,
    /// 秒
    Second,
    /// 分
    Minute,
    /// 小时
    Hour,
    /// 天
    Day,
    /// 周
    Week,
}

impl DateUnit {
    /// 该单位对应的毫秒数。
    pub fn get_millis(self) -> i64 {
        match self {
            Self::Ms => 1,
            Self::Second => 1_000,
            Self::Minute => 60_000,
            Self::Hour => 3_600_000,
            Self::Day => 86_400_000,
            Self::Week => 604_800_000,
        }
    }
}
