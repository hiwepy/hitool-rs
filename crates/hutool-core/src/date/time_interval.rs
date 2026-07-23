//! 对齐: `cn.hutool.core.date.TimeInterval`
//! 来源: hutool-core TimeInterval（单段计时器）

use std::time::Instant;

use crate::date::between_formatter::{BetweenFormatter, Level};
use crate::date::date_unit::DateUnit;

/// 对齐 Java: `cn.hutool.core.date.TimeInterval`
#[derive(Debug, Clone)]
pub struct TimeInterval {
    start: Instant,
    is_nano: bool,
    group: bool,
}

impl Default for TimeInterval {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeInterval {
    /// 对齐 Java: `new TimeInterval()` — 毫秒精度。
    pub fn new() -> Self {
        Self::with_nano(false)
    }

    /// 对齐 Java: `new TimeInterval(boolean isNano)`
    pub fn with_nano(is_nano: bool) -> Self {
        Self {
            start: Instant::now(),
            is_nano,
            group: false,
        }
    }

    /// 分组计时入口（兼容旧 API / Hutool intervalGroup 场景）。
    pub fn new_group() -> Self {
        Self {
            start: Instant::now(),
            is_nano: false,
            group: true,
        }
    }

    /// 对齐 Java: `TimeInterval.start()` — 重新计时并返回当前间隔。
    pub fn start(&mut self) -> i64 {
        let interval = self.interval();
        self.start = Instant::now();
        interval
    }

    /// 对齐 Java: `TimeInterval.intervalRestart()`
    pub fn interval_restart(&mut self) -> i64 {
        self.start()
    }

    /// 对齐 Java: `TimeInterval.restart()`
    pub fn restart(&mut self) -> &mut Self {
        self.start = Instant::now();
        self
    }

    /// 对齐 Java: `TimeInterval.interval()` — 纳秒或毫秒取决于构造。
    pub fn interval(&self) -> i64 {
        if self.is_nano {
            self.start.elapsed().as_nanos() as i64
        } else {
            self.start.elapsed().as_millis() as i64
        }
    }

    /// 对齐 Java: `TimeInterval.intervalPretty()`
    pub fn interval_pretty(&self) -> String {
        BetweenFormatter::new(self.interval_ms(), Level::Millisecond, 0).format()
    }

    /// 对齐 Java: `TimeInterval.intervalMs()`
    pub fn interval_ms(&self) -> i64 {
        self.start.elapsed().as_millis() as i64
    }

    /// 对齐 Java: `TimeInterval.intervalSecond()`
    pub fn interval_second(&self) -> i64 {
        self.interval_ms() / DateUnit::Second.get_millis()
    }

    /// 对齐 Java: `TimeInterval.intervalMinute()`
    pub fn interval_minute(&self) -> i64 {
        self.interval_ms() / DateUnit::Minute.get_millis()
    }

    /// 对齐 Java: `TimeInterval.intervalHour()`
    pub fn interval_hour(&self) -> i64 {
        self.interval_ms() / DateUnit::Hour.get_millis()
    }

    /// 对齐 Java: `TimeInterval.intervalDay()`
    pub fn interval_day(&self) -> i64 {
        self.interval_ms() / DateUnit::Day.get_millis()
    }

    /// 对齐 Java: `TimeInterval.intervalWeek()`
    pub fn interval_week(&self) -> i64 {
        self.interval_ms() / DateUnit::Week.get_millis()
    }

    /// 是否纳秒模式。
    pub fn is_nano(&self) -> bool {
        self.is_nano
    }

    /// 是否分组模式。
    pub fn is_group(&self) -> bool {
        self.group
    }
}
