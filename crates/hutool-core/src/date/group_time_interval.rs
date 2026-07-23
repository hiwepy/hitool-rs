//! 对齐: `cn.hutool.core.date.GroupTimeInterval`
//! 来源: hutool-core GroupTimeInterval（按 id 分组的多段计时）

use std::collections::HashMap;
use std::time::Instant;

use crate::date::between_formatter::{BetweenFormatter, Level};
use crate::date::date_unit::DateUnit;

/// 对齐 Java: `cn.hutool.core.date.GroupTimeInterval`
#[derive(Debug, Clone)]
pub struct GroupTimeInterval {
    is_nano: bool,
    group_map: HashMap<String, Instant>,
}

impl Default for GroupTimeInterval {
    fn default() -> Self {
        Self::new(false)
    }
}

impl GroupTimeInterval {
    /// 对齐 Java: `new GroupTimeInterval(boolean isNano)`
    pub fn new(is_nano: bool) -> Self {
        Self {
            is_nano,
            group_map: HashMap::new(),
        }
    }

    /// 对齐 Java: `GroupTimeInterval.clear()`
    pub fn clear(&mut self) -> &mut Self {
        self.group_map.clear();
        self
    }

    /// 对齐 Java: `GroupTimeInterval.start(String)`
    pub fn start(&mut self, id: impl Into<String>) -> i64 {
        let id = id.into();
        let previous = self.interval(&id);
        self.group_map.insert(id, Instant::now());
        previous
    }

    /// 对齐 Java: `GroupTimeInterval.intervalRestart(String)`
    pub fn interval_restart(&mut self, id: impl Into<String>) -> i64 {
        self.start(id)
    }

    /// 对齐 Java: `GroupTimeInterval.interval(String)`
    pub fn interval(&self, id: &str) -> i64 {
        let Some(start) = self.group_map.get(id) else {
            return 0;
        };
        if self.is_nano {
            start.elapsed().as_nanos() as i64
        } else {
            start.elapsed().as_millis() as i64
        }
    }

    /// 对齐 Java: `GroupTimeInterval.interval(String, DateUnit)`
    pub fn interval_unit(&self, id: &str, unit: DateUnit) -> i64 {
        let ms = self.interval_ms(id);
        ms / unit.get_millis().max(1)
    }

    /// 对齐 Java: `GroupTimeInterval.intervalMs(String)`
    pub fn interval_ms(&self, id: &str) -> i64 {
        self.group_map
            .get(id)
            .map(|s| s.elapsed().as_millis() as i64)
            .unwrap_or(0)
    }

    /// 对齐 Java: `GroupTimeInterval.intervalSecond(String)`
    pub fn interval_second(&self, id: &str) -> i64 {
        self.interval_ms(id) / DateUnit::Second.get_millis()
    }

    /// 对齐 Java: `GroupTimeInterval.intervalMinute(String)`
    pub fn interval_minute(&self, id: &str) -> i64 {
        self.interval_ms(id) / DateUnit::Minute.get_millis()
    }

    /// 对齐 Java: `GroupTimeInterval.intervalHour(String)`
    pub fn interval_hour(&self, id: &str) -> i64 {
        self.interval_ms(id) / DateUnit::Hour.get_millis()
    }

    /// 对齐 Java: `GroupTimeInterval.intervalDay(String)`
    pub fn interval_day(&self, id: &str) -> i64 {
        self.interval_ms(id) / DateUnit::Day.get_millis()
    }

    /// 对齐 Java: `GroupTimeInterval.intervalWeek(String)`
    pub fn interval_week(&self, id: &str) -> i64 {
        self.interval_ms(id) / DateUnit::Week.get_millis()
    }

    /// 对齐 Java: `GroupTimeInterval.intervalPretty(String)`
    pub fn interval_pretty(&self, id: &str) -> String {
        BetweenFormatter::new(self.interval_ms(id), Level::Millisecond, 0).format()
    }
}
