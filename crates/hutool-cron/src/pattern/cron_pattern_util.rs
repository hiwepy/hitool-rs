//! Hutool-aligned cron patterns, builders, parsers, and matchers.

#![allow(clippy::missing_panics_doc)]

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::cron_pattern::CronPattern;

/// Helpers for calculating matching dates with explicit bounds.
pub struct CronPatternUtil;

impl CronPatternUtil {
    /// Returns the next date after `start` using second matching.
    #[must_use]
    pub fn next_date_after(pattern: &CronPattern, start: DateTime<Utc>) -> Option<DateTime<Utc>> {
        pattern.next_match_after(start, true)
    }

    /// Returns the next date after `start` using the requested precision.
    #[must_use]
    pub fn next_date_after_with_precision(
        pattern: &CronPattern,
        start: DateTime<Utc>,
        match_second: bool,
    ) -> Option<DateTime<Utc>> {
        pattern.next_match_after(start, match_second)
    }

    /// Returns at most `count` matching dates in the inclusive time window.
    pub fn matched_dates(
        pattern: &CronPattern,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        count: usize,
        match_second: bool,
    ) -> Result<Vec<DateTime<Utc>>, CronError> {
        if end < start {
            return Err(CronError::InvalidDateRange);
        }
        let mut result = Vec::with_capacity(count.min(64));
        let mut cursor = start - ChronoDuration::seconds(1);
        while result.len() < count {
            let Some(next) = pattern.next_match_after(cursor, match_second) else {
                break;
            };
            if next > end {
                break;
            }
            result.push(next);
            cursor = next;
        }
        Ok(result)
    }

    /// Hutool `matchedDates(pattern, start, count, matchSecond)` — end defaults to year-end.
    pub fn matched_dates_count(
        pattern: &str,
        start: DateTime<Utc>,
        count: usize,
        match_second: bool,
    ) -> Result<Vec<DateTime<Utc>>, CronError> {
        let parsed = CronPattern::parse(pattern)?;
        let end = end_of_year(start);
        Self::matched_dates(&parsed, start, end, count, match_second)
    }

    /// Hutool `matchedDates(pattern, start, end, count, matchSecond)`.
    pub fn matched_dates_str(
        pattern: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        count: usize,
        match_second: bool,
    ) -> Result<Vec<DateTime<Utc>>, CronError> {
        let parsed = CronPattern::parse(pattern)?;
        Self::matched_dates(&parsed, start, end, count, match_second)
    }
}

fn end_of_year(start: DateTime<Utc>) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(start.year(), 12, 31, 23, 59, 59)
        .single()
        .unwrap_or(start)
}
