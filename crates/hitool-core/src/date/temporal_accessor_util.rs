//! 对齐: `cn.hutool.core.date.TemporalAccessorUtil`
//! chrono `NaiveDateTime` / `DateTime` 字段读取与格式化。

use chrono::{Datelike, NaiveDateTime, Timelike};

use crate::date::date_pattern::{self, DatePattern};

/// 对齐 Java: `cn.hutool.core.date.TemporalAccessorUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct TemporalAccessorUtil;

impl TemporalAccessorUtil {
    /// 对齐 Java: `TemporalAccessorUtil.get` — 常用字段快捷读取。
    pub fn get_year(dt: NaiveDateTime) -> i32 {
        dt.year()
    }

    /// 取月（1-12）。
    pub fn get_month(dt: NaiveDateTime) -> u32 {
        dt.month()
    }

    /// 取日。
    pub fn get_day(dt: NaiveDateTime) -> u32 {
        dt.day()
    }

    /// 取小时。
    pub fn get_hour(dt: NaiveDateTime) -> u32 {
        dt.hour()
    }

    /// 对齐 Java: `TemporalAccessorUtil.format(TemporalAccessor)` 默认 ISO 风格。
    pub fn format(dt: NaiveDateTime) -> String {
        Self::format_iso(dt)
    }

    /// 对齐 Java: `TemporalAccessorUtil.format(TemporalAccessor, String)`
    pub fn format_pattern(dt: NaiveDateTime, pattern: &str) -> String {
        let chrono_pat = date_pattern::to_chrono_format(pattern);
        dt.format(&chrono_pat).to_string()
    }

    /// 对齐 Java: `TemporalAccessorUtil.format` ISO 风格别名。
    pub fn format_iso(dt: NaiveDateTime) -> String {
        dt.format("%Y-%m-%dT%H:%M:%S").to_string()
    }

    /// 对齐 Java: `TemporalAccessorUtil.toEpochMilli`
    pub fn to_epoch_milli(dt: NaiveDateTime) -> i64 {
        dt.and_utc().timestamp_millis()
    }

    /// 对齐 Java: `TemporalAccessorUtil.isIn`
    pub fn is_in(date: NaiveDateTime, begin: NaiveDateTime, end: NaiveDateTime) -> bool {
        Self::is_in_bounds(date, begin, end, true, true)
    }

    /// 对齐 Java: `TemporalAccessorUtil.isIn(..., includeBegin, includeEnd)`
    pub fn is_in_bounds(
        date: NaiveDateTime,
        begin: NaiveDateTime,
        end: NaiveDateTime,
        include_begin: bool,
        include_end: bool,
    ) -> bool {
        let after_begin = if include_begin {
            date >= begin
        } else {
            date > begin
        };
        let before_end = if include_end { date <= end } else { date < end };
        after_begin && before_end
    }

    /// 通过 DatePattern 工厂格式化。
    pub fn format_with_pattern(dt: NaiveDateTime, pattern: &str) -> String {
        let fmt = DatePattern::create_formatter(pattern);
        dt.format(&fmt).to_string()
    }
}
