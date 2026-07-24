//! Hutool-aligned cron patterns, builders, parsers, and matchers.

#![allow(clippy::missing_panics_doc)]

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::part::Part;
use super::part_matcher::PartMatcher;
use super::pattern_parser::PatternParser;

/// Seven-field matcher assembled by `PatternParser`.
#[derive(Debug)]
pub struct PatternMatcher {
    fields: [Box<dyn PartMatcher>; 7],
}

impl PatternMatcher {
    /// Creates a matcher from all seven fields.
    #[must_use]
    pub fn new(fields: [Box<dyn PartMatcher>; 7]) -> Self {
        Self { fields }
    }

    /// Returns a field matcher.
    #[must_use]
    pub fn get(&self, part: Part) -> &dyn PartMatcher {
        self.fields[part.calendar_field()].as_ref()
    }

    /// Matches `[second, minute, hour, day, month, weekday, year]`.
    #[must_use]
    pub fn matches(&self, fields: [i32; 7]) -> bool {
        self.fields
            .iter()
            .zip(fields)
            .all(|(matcher, value)| matcher.matches(value))
    }

    /// Matches Java/Hutool weekday numbering, treating 0 and 7 as Sunday.
    #[must_use]
    pub fn matches_week(&self, weekday: i32) -> bool {
        let matcher = self.get(Part::DayOfWeek);
        matcher.matches(weekday) || (weekday == 0 && matcher.matches(7))
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
