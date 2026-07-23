//! 对齐: `cn.hutool.core.date.DateField`
//! 来源: hutool-core/.../DateField.java
//!
//! 与 Java `Calendar` 字段常量对应（MONTH 从 0 起）。

#![allow(dead_code)]

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.date.DateField`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DateField {
    /// 纪元（Calendar.ERA）
    Era = 0,
    /// 年
    Year = 1,
    /// 月（0=一月）
    Month = 2,
    /// 一年中第几周
    WeekOfYear = 3,
    /// 一月中第几周
    WeekOfMonth = 4,
    /// 一月中的第几天
    DayOfMonth = 5,
    /// 一年中的第几天
    DayOfYear = 6,
    /// 周几（1=周日）
    DayOfWeek = 7,
    /// 天所在的周是这个月的第几周
    DayOfWeekInMonth = 8,
    /// 上午 / 下午
    AmPm = 9,
    /// 12 小时制小时
    Hour = 10,
    /// 24 小时制小时
    HourOfDay = 11,
    /// 分钟
    Minute = 12,
    /// 秒
    Second = 13,
    /// 毫秒
    Millisecond = 14,
}

impl DateField {
    /// 对齐 Java: `DateField.of(int)` — 按 Calendar 常量取值。
    pub fn of(calendar_value: i32) -> Option<Self> {
        match calendar_value {
            0 => Some(Self::Era),
            1 => Some(Self::Year),
            2 => Some(Self::Month),
            3 => Some(Self::WeekOfYear),
            4 => Some(Self::WeekOfMonth),
            5 => Some(Self::DayOfMonth),
            6 => Some(Self::DayOfYear),
            7 => Some(Self::DayOfWeek),
            8 => Some(Self::DayOfWeekInMonth),
            9 => Some(Self::AmPm),
            10 => Some(Self::Hour),
            11 => Some(Self::HourOfDay),
            12 => Some(Self::Minute),
            13 => Some(Self::Second),
            14 => Some(Self::Millisecond),
            _ => None,
        }
    }

    /// 返回对应 Calendar int 值。
    pub fn get_value(self) -> i32 {
        self as i32
    }

    /// 兼容旧 sentinel 调用点（已实现，返回 Ok）。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}

impl std::str::FromStr for DateField {
    type Err = CoreError;

    /// 按枚举名解析（忽略大小写）。
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_uppercase().as_str() {
            "ERA" => Ok(Self::Era),
            "YEAR" => Ok(Self::Year),
            "MONTH" => Ok(Self::Month),
            "WEEK_OF_YEAR" => Ok(Self::WeekOfYear),
            "WEEK_OF_MONTH" => Ok(Self::WeekOfMonth),
            "DAY_OF_MONTH" => Ok(Self::DayOfMonth),
            "DAY_OF_YEAR" => Ok(Self::DayOfYear),
            "DAY_OF_WEEK" => Ok(Self::DayOfWeek),
            "DAY_OF_WEEK_IN_MONTH" => Ok(Self::DayOfWeekInMonth),
            "AM_PM" => Ok(Self::AmPm),
            "HOUR" => Ok(Self::Hour),
            "HOUR_OF_DAY" => Ok(Self::HourOfDay),
            "MINUTE" => Ok(Self::Minute),
            "SECOND" => Ok(Self::Second),
            "MILLISECOND" => Ok(Self::Millisecond),
            _ => Err(CoreError::InvalidArgument {
                name: "date_field",
                reason: "unknown DateField name",
            }),
        }
    }
}
