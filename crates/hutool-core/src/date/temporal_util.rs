//! 对齐: `cn.hutool.core.date.TemporalUtil`
//! chrono 实现 Java Temporal 偏移 / Duration / 单位换算的可移植子集。

use chrono::{Datelike, Duration, NaiveDateTime, Weekday};

use crate::date::date_unit::DateUnit;
use crate::date::stop_watch::TimeUnit;

/// 对齐 Java: `cn.hutool.core.date.TemporalUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct TemporalUtil;

impl TemporalUtil {
    /// 对齐 Java: `TemporalUtil.between` — 毫秒差。
    pub fn between_ms(start: NaiveDateTime, end: NaiveDateTime) -> i64 {
        (end - start).num_milliseconds()
    }

    /// 对齐 Java: `TemporalUtil.between(..., ChronoUnit)` — 按 DateUnit 计量。
    pub fn between(start: NaiveDateTime, end: NaiveDateTime, unit: DateUnit) -> i64 {
        let ms = Self::between_ms(start, end);
        ms / unit.get_millis().max(1)
    }

    /// 对齐 Java: `TemporalUtil.offset(T, long, TemporalUnit)`
    pub fn offset(dt: NaiveDateTime, unit: DateUnit, offset: i64) -> NaiveDateTime {
        match unit {
            DateUnit::Day => dt + Duration::days(offset),
            DateUnit::Hour => dt + Duration::hours(offset),
            DateUnit::Minute => dt + Duration::minutes(offset),
            DateUnit::Second => dt + Duration::seconds(offset),
            DateUnit::Ms => dt + Duration::milliseconds(offset),
            DateUnit::Week => dt + Duration::weeks(offset),
        }
    }

    /// 对齐 Java: `TemporalUtil.offset(..., DayOfWeek, boolean isPrevious)`
    pub fn offset_to_weekday(
        dt: NaiveDateTime,
        weekday: Weekday,
        is_previous: bool,
    ) -> NaiveDateTime {
        let mut cur = dt.date();
        for _ in 0..8 {
            if cur.weekday() == weekday {
                return cur.and_time(dt.time());
            }
            cur = if is_previous {
                cur.pred_opt().unwrap_or(cur)
            } else {
                cur.succ_opt().unwrap_or(cur)
            };
        }
        dt
    }

    /// 对齐 Java: `TemporalUtil.toChronoUnit(TimeUnit)` — Rust 侧映射到 DateUnit。
    pub fn to_date_unit(unit: TimeUnit) -> DateUnit {
        match unit {
            TimeUnit::Nanos | TimeUnit::Micros | TimeUnit::Millis => DateUnit::Ms,
            TimeUnit::Seconds => DateUnit::Second,
        }
    }

    /// 对齐 Java: `TemporalUtil.toTimeUnit(ChronoUnit)`
    pub fn to_time_unit(unit: DateUnit) -> TimeUnit {
        match unit {
            DateUnit::Ms => TimeUnit::Millis,
            DateUnit::Second => TimeUnit::Seconds,
            DateUnit::Minute | DateUnit::Hour | DateUnit::Day | DateUnit::Week => TimeUnit::Seconds,
        }
    }
}
