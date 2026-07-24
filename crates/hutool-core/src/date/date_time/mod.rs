//! 对齐: `cn.hutool.core.date.DateTime`
//!
//! # Timezone note
//! 默认按 UTC+08:00（Asia/Shanghai 无夏令时偏移）解释/格式化墙钟时间，
//! 与 Hutool `DateUtilTest` 的 `TZ=Asia/Shanghai` 约定一致。

#![allow(dead_code)]

use chrono::{Datelike, Duration, FixedOffset, NaiveDate, NaiveDateTime, Timelike, Weekday};

use crate::date::date_field::DateField;
use crate::date::date_pattern::{self, NORM_DATETIME_MS_PATTERN, NORM_DATETIME_PATTERN};
use crate::date::date_unit::DateUnit;
use crate::date::month::Month;
use crate::date::quarter::Quarter;
use crate::date::week::Week;
use crate::{CoreError, Result};

mod date_time;
mod hutool_date_time;

pub use date_time::DateTime;
pub use hutool_date_time::HutoolDateTime;
pub use hutool_date_time::parity_zone;
pub use hutool_date_time::week_of_year_mon_min1;
pub use hutool_date_time::format_with_pattern;
pub use hutool_date_time::between_unit;
