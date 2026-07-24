//! Hutool-aligned cron patterns, builders, parsers, and matchers.

#![allow(clippy::missing_panics_doc)]

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::part::Part;
use super::part_parser::PartParser;
use super::pattern_matcher::PatternMatcher;

/// Parses full expressions into field matchers.
pub struct PatternParser;

impl PatternParser {
    /// Parses every `|`-separated expression.
    pub fn parse(expression: &str) -> Result<Vec<PatternMatcher>, CronError> {
        expression
            .split('|')
            .map(str::trim)
            .map(|alternative| {
                let fields = pad_fields(alternative, true)?;
                let fields = [
                    PartParser::new(Part::Second).parse(&fields[0])?,
                    PartParser::new(Part::Minute).parse(&fields[1])?,
                    PartParser::new(Part::Hour).parse(&fields[2])?,
                    PartParser::new(Part::DayOfMonth).parse(&fields[3])?,
                    PartParser::new(Part::Month).parse(&fields[4])?,
                    PartParser::new(Part::DayOfWeek).parse(&fields[5])?,
                    PartParser::new(Part::Year).parse(&fields[6])?,
                ];
                Ok(PatternMatcher::new(fields))
            })
            .collect()
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

fn pad_fields(expression: &str, match_second: bool) -> Result<[String; 7], CronError> {
    let mut fields = expression
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    match fields.len() {
        5 => {
            fields.insert(0, "0".to_owned());
            fields.push("*".to_owned());
        }
        6 => fields.push("*".to_owned()),
        7 => {}
        _ => return Err(CronError::InvalidPattern(expression.to_owned())),
    }
    if !match_second {
        fields[0] = "0".to_owned();
    }
    Ok([
        fields[0].clone(),
        fields[1].clone(),
        fields[2].clone(),
        fields[3].clone(),
        fields[4].clone(),
        fields[5].clone(),
        fields[6].clone(),
    ])
}
