//! Hutool-aligned cron patterns, builders, parsers, and matchers.

#![allow(clippy::missing_panics_doc)]

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

/// Common behavior of one cron field matcher.
pub trait PartMatcher: fmt::Debug + Send + Sync {
    /// Returns whether `value` matches.
    fn matches(&self, value: i32) -> bool;
    /// Returns the first represented value at or after `value`, wrapping to the minimum.
    fn next_after(&self, value: i32) -> i32;
}
