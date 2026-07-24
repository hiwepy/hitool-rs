//! Hutool-aligned cron patterns, builders, parsers, and matchers.

#![allow(clippy::missing_panics_doc)]

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::part_matcher::PartMatcher;

/// Sorted finite-value matcher used for most cron fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolArrayMatcher {
    values: Vec<i32>,
}

impl BoolArrayMatcher {
    /// Creates a matcher from a non-empty value collection.
    pub fn new(values: impl IntoIterator<Item = i32>) -> Result<Self, CronError> {
        Self::from_values(values.into_iter().collect())
    }

    fn from_values(mut values: Vec<i32>) -> Result<Self, CronError> {
        values.sort_unstable();
        values.dedup();
        if values.is_empty() {
            return Err(CronError::EmptyMatcher);
        }
        Ok(Self { values })
    }

    /// Returns the minimum represented value.
    #[must_use]
    pub fn min_value(&self) -> i32 {
        self.values[0]
    }

    /// Returns the maximum represented value.
    #[must_use]
    pub fn max_value(&self) -> i32 {
        self.values[self.values.len() - 1]
    }
}

impl PartMatcher for BoolArrayMatcher {
    fn matches(&self, value: i32) -> bool {
        self.values.binary_search(&value).is_ok()
    }

    fn next_after(&self, value: i32) -> i32 {
        self.values
            .iter()
            .copied()
            .find(|candidate| *candidate >= value)
            .unwrap_or_else(|| self.min_value())
    }
}

impl fmt::Display for BoolArrayMatcher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values = self.values.iter().map(i32::to_string).collect::<Vec<_>>();
        formatter.write_str(&values.join(","))
    }
}

pub(crate) fn fields<Tz: TimeZone>(value: &DateTime<Tz>, match_second: bool) -> [i32; 7] {
    [
        if match_second {
            i32::try_from(value.second()).unwrap_or_default()
        } else {
            0
        },
        i32::try_from(value.minute()).unwrap_or_default(),
        i32::try_from(value.hour()).unwrap_or_default(),
        i32::try_from(value.day()).unwrap_or_default(),
        i32::try_from(value.month()).unwrap_or_default(),
        i32::try_from(value.weekday().num_days_from_sunday()).unwrap_or_default(),
        value.year(),
    ]
}
