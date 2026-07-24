//! Hutool-aligned cron patterns, builders, parsers, and matchers.

#![allow(clippy::missing_panics_doc)]

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::day_of_month_matcher::DayOfMonthMatcher;
use super::part::Part;

/// Hutool-style cron pattern with support for `|` alternatives.
#[derive(Debug, Clone)]
pub struct CronPattern {
    expression: String,
    second_schedules: Vec<Schedule>,
    minute_schedules: Vec<Schedule>,
    /// Per-alternative flag: day-of-month field used Hutool `L` (last day).
    dom_last: Vec<bool>,
}

impl CronPattern {
    /// Parses a five-, six-, or seven-part expression.
    pub fn parse(expression: impl Into<String>) -> Result<Self, CronError> {
        let expression = expression.into();
        let alternatives = expression.split('|').map(str::trim).collect::<Vec<_>>();
        if alternatives.is_empty() || alternatives.iter().any(|part| part.is_empty()) {
            return Err(CronError::InvalidPattern(expression));
        }
        let mut second_schedules = Vec::with_capacity(alternatives.len());
        let mut minute_schedules = Vec::with_capacity(alternatives.len());
        let mut dom_last = Vec::with_capacity(alternatives.len());
        for alternative in alternatives {
            let (second_expr, last) = normalize_expanded(alternative, true)?;
            let (minute_expr, _) = normalize_expanded(alternative, false)?;
            second_schedules.push(Schedule::from_str(&second_expr)?);
            minute_schedules.push(
                Schedule::from_str(&minute_expr)
                    .expect("replacing a valid seconds field with zero remains valid"),
            );
            dom_last.push(last);
        }
        Ok(Self {
            expression,
            second_schedules,
            minute_schedules,
            dom_last,
        })
    }

    /// Alias matching Hutool's `of` constructor.
    pub fn of(expression: impl Into<String>) -> Result<Self, CronError> {
        Self::parse(expression)
    }

    /// Returns whether the UTC instant matches this pattern.
    #[must_use]
    pub fn matches(&self, instant: DateTime<Utc>, match_second: bool) -> bool {
        let instant = if match_second {
            instant
                .with_nanosecond(0)
                .expect("zero nanoseconds is always a valid timestamp")
        } else {
            instant
                .with_second(0)
                .expect("zero seconds is always valid")
                .with_nanosecond(0)
                .expect("zero nanoseconds is always valid")
        };
        self.schedules(match_second)
            .iter()
            .zip(self.dom_last.iter().copied())
            .any(|(schedule, last)| {
                let hits = schedule
                    .after(&(instant - ChronoDuration::seconds(1)))
                    .next()
                    == Some(instant);
                hits && (!last || is_last_day_of_month(instant))
            })
    }

    /// Returns whether a millisecond timestamp matches this pattern.
    pub fn matches_millis(&self, millis: i64, match_second: bool) -> Result<bool, CronError> {
        let Some(instant) = Utc.timestamp_millis_opt(millis).single() else {
            return Err(CronError::InvalidTimestamp);
        };
        Ok(self.matches(instant, match_second))
    }

    /// Returns the first matching instant at or after `start`.
    #[must_use]
    pub fn next_match(&self, start: DateTime<Utc>, match_second: bool) -> Option<DateTime<Utc>> {
        if self.matches(start, match_second) {
            Some(start)
        } else {
            self.next_match_after(start, match_second)
        }
    }

    /// Returns the first matching instant strictly after `start`.
    #[must_use]
    pub fn next_match_after(
        &self,
        start: DateTime<Utc>,
        match_second: bool,
    ) -> Option<DateTime<Utc>> {
        self.schedules(match_second)
            .iter()
            .zip(self.dom_last.iter().copied())
            .filter_map(|(schedule, last)| next_after_filtered(schedule, start, last))
            .min()
    }

    fn schedules(&self, match_second: bool) -> &[Schedule] {
        if match_second {
            &self.second_schedules
        } else {
            &self.minute_schedules
        }
    }
}

impl fmt::Display for CronPattern {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.expression)
    }
}

fn is_last_day_of_month(instant: DateTime<Utc>) -> bool {
    let leap = instant.year() % 4 == 0 && (instant.year() % 100 != 0 || instant.year() % 400 == 0);
    instant.day() == DayOfMonthMatcher::last_day(instant.month(), leap)
}

fn next_after_filtered(

fn normalize_expanded(expression: &str, match_second: bool) -> Result<(String, bool), CronError> {
    let mut fields = expression.split_whitespace().collect::<Vec<_>>();
    match fields.len() {
        5 => {
            fields.insert(0, "0");
            fields.push("*");
        }
        6 => fields.push("*"),
        7 => {}
        _ => return Err(CronError::InvalidPattern(expression.to_owned())),
    }
    if !match_second {
        fields[0] = "0";
    }
    let parts = [
        Part::Second,
        Part::Minute,
        Part::Hour,
        Part::DayOfMonth,
        Part::Month,
        Part::DayOfWeek,
        Part::Year,
    ];
    let mut dom_last = false;
    let mut expanded = Vec::with_capacity(7);
    for (part, field) in parts.into_iter().zip(fields.iter().copied()) {
        let (field_expr, last) = expand_field(part, field)?;
        if part == Part::DayOfMonth {
            dom_last = last;
        }
        expanded.push(field_expr);
    }
    Ok((expanded.join(" "), dom_last))
}
