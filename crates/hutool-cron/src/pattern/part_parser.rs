//! Hutool-aligned cron patterns, builders, parsers, and matchers.

#![allow(clippy::missing_panics_doc)]

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::always_true_matcher::AlwaysTrueMatcher;
use super::bool_array_matcher::BoolArrayMatcher;
use super::day_of_month_matcher::DayOfMonthMatcher;
use super::part::Part;
use super::part_matcher::PartMatcher;
use super::year_value_matcher::YearValueMatcher;

/// Parses a single cron part into a matcher.
#[derive(Debug, Clone, Copy)]
pub struct PartParser {
    part: Part,
}

impl PartParser {
    /// Creates a parser for `part`.
    #[must_use]
    pub const fn new(part: Part) -> Self {
        Self { part }
    }

    /// Parses wildcards, lists, ranges, steps, `L`, negatives, and wrapping ranges.
    pub fn parse(&self, value: &str) -> Result<Box<dyn PartMatcher>, CronError> {
        if matches!(value, "*" | "?") {
            return Ok(Box::new(AlwaysTrueMatcher));
        }
        let mut values = Vec::new();
        for item in value.split(',') {
            let (base, step) = item.split_once('/').map_or((item, 1), |(base, step)| {
                (base, step.parse::<i32>().unwrap_or(0))
            });
            if step <= 0 {
                return Err(CronError::InvalidPattern(value.to_owned()));
            }
            let range_max = if self.part == Part::DayOfMonth {
                31
            } else {
                self.part.max()
            };
            let collected = if base == "*" {
                expand_range(self.part, self.part.min(), range_max, step)?
            } else if let Some((begin, end)) = base
                .split_once('-')
                .filter(|(b, e)| !b.is_empty() && !e.is_empty())
            {
                let begin = apply_negative(self.part, parse_alias(self.part, begin)?)?;
                let end = apply_negative(self.part, parse_alias(self.part, end)?)?;
                expand_range(self.part, begin, end, step)?
            } else {
                let begin = apply_negative(self.part, parse_alias(self.part, base)?)?;
                if step > 1 {
                    expand_range(self.part, begin, range_max, step)?
                } else if self.part == Part::DayOfMonth && begin == 32 {
                    vec![32]
                } else {
                    vec![checked_schedule_value(self.part, begin)?]
                }
            };
            values.extend(collected);
        }
        if self.part == Part::Year {
            Ok(Box::new(YearValueMatcher::from_values(values).expect(
                "year values were validated while parsing the cron field",
            )))
        } else if self.part == Part::DayOfMonth {
            Ok(Box::new(DayOfMonthMatcher::from_values(values).expect(
                "day-of-month values were validated while parsing the cron field",
            )))
        } else {
            Ok(Box::new(BoolArrayMatcher::from_values(values).expect(
                "a successfully parsed finite cron field contains a value",
            )))
        }
    }
}

fn expand_range(part: Part, begin: i32, end: i32, step: i32) -> Result<Vec<i32>, CronError> {
    let step = usize::try_from(step).expect("positive step fits usize");
    let max = if part == Part::DayOfMonth {
        31
    } else {
        part.max()
    };
    let min = part.min();
    // For DOM, allow 32 only as standalone L, not in numeric ranges beyond 31.
    let begin = if part == Part::DayOfMonth && begin == 32 {
        max
    } else {
        begin
    };
    let end = if part == Part::DayOfMonth && end == 32 {
        max
    } else {
        end
    };
    if part == Part::DayOfMonth {
        if !(1..=31).contains(&begin) || !(1..=31).contains(&end) {
            return Err(CronError::InvalidPartValue {
                part,
                value: begin.max(end),
            });
        }
    } else {
        part.check_value(begin)?;
        part.check_value(end)?;
    }
    let mut values = Vec::new();
    if begin <= end {
        values.extend((begin..=end).step_by(step));
    } else {
        // Hutool wrap: 22-2 → 22..=max then min..=2
        values.extend((begin..=max).step_by(step));
        values.extend((min..=end).step_by(step));
    }
    Ok(values)
}

fn checked_schedule_value(part: Part, value: i32) -> Result<i32, CronError> {
    if part == Part::DayOfMonth {
        if value == 32 {
            return Ok(32);
        }
        if !(1..=31).contains(&value) {
            return Err(CronError::InvalidPartValue { part, value });
        }
        return Ok(value);
    }
    part.check_value(value)
}

fn parse_alias(part: Part, value: &str) -> Result<i32, CronError> {
    let lowercase = value.to_ascii_lowercase();
    // Hutool: `L` means the field maximum (day-of-month sentinel 32, Saturday=6).
    if lowercase == "l" {
        return Ok(match part {
            Part::DayOfMonth => 32,
            Part::DayOfWeek => 6,
            _ => part.max(),
        });
    }
    let alias = match part {
        Part::Month => [
            "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
        ]
        .iter()
        .position(|candidate| *candidate == lowercase)
        .and_then(|index| i32::try_from(index).ok())
        .map(|index| index + 1),
        Part::DayOfWeek => ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
            .iter()
            .position(|candidate| *candidate == lowercase)
            .and_then(|index| i32::try_from(index).ok()),
        _ => None,
    };
    alias
        .or_else(|| value.parse().ok())
        .ok_or_else(|| CronError::InvalidPattern(value.to_owned()))
}

fn apply_negative(part: Part, value: i32) -> Result<i32, CronError> {
    if value >= 0 {
        return Ok(value);
    }
    // Hutool: `i += part.getMax()` — hour `-4` → 19; DOM uses max 32.
    Ok(value + part.max())
}
