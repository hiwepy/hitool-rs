//! UTC date and date-time helpers built on `chrono`.
//!
//! Basic parsing and formatting behavior was adapted from yimi-rutool 0.2.5
//! (Apache-2.0) and revised to use checked arithmetic.

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};

use crate::{CoreError, Result};

/// UTC-focused date and date-time utilities.
pub struct DateUtil;

impl DateUtil {
    /// Returns the current UTC date-time.
    #[must_use]
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }

    /// Parses a date with a `chrono` format string.
    ///
    /// # Errors
    ///
    /// Returns an error when the value does not match `format`.
    pub fn parse_date(value: &str, format: &str) -> Result<NaiveDate> {
        Ok(NaiveDate::parse_from_str(value, format)?)
    }

    /// Parses a timezone-free date-time with a `chrono` format string.
    ///
    /// # Errors
    ///
    /// Returns an error when the value does not match `format`.
    pub fn parse_datetime(value: &str, format: &str) -> Result<NaiveDateTime> {
        Ok(NaiveDateTime::parse_from_str(value, format)?)
    }

    /// Formats a date with a `chrono` format string.
    #[must_use]
    pub fn format_date(value: NaiveDate, format: &str) -> String {
        value.format(format).to_string()
    }

    /// Formats a UTC date-time with a `chrono` format string.
    #[must_use]
    pub fn format_datetime(value: DateTime<Utc>, format: &str) -> String {
        value.format(format).to_string()
    }

    /// Adds signed days with overflow checking.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::DateOverflow`] if the resulting date is out of range.
    pub fn add_days(value: NaiveDate, days: i64) -> Result<NaiveDate> {
        value
            .checked_add_signed(Duration::days(days))
            .ok_or(CoreError::DateOverflow)
    }

    /// Returns Unix epoch milliseconds.
    #[must_use]
    pub fn timestamp_millis(value: DateTime<Utc>) -> i64 {
        value.timestamp_millis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_parse_format_and_offset_round_trip() {
        let date = DateUtil::parse_date("2026-07-17", "%Y-%m-%d").unwrap();
        assert_eq!(DateUtil::format_date(date, "%Y%m%d"), "20260717");
        assert_eq!(
            DateUtil::format_date(DateUtil::add_days(date, 1).unwrap(), "%Y-%m-%d"),
            "2026-07-18"
        );
    }
}
