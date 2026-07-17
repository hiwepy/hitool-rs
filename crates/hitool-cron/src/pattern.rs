//! Hutool-aligned cron patterns, builders, parsers, and matchers.

#![allow(clippy::missing_panics_doc)]

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

/// A field in a Hutool cron expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Part {
    /// Seconds, `0..=59`.
    Second,
    /// Minutes, `0..=59`.
    Minute,
    /// Hours, `0..=23`.
    Hour,
    /// Day of month, `1..=31`.
    DayOfMonth,
    /// Month, `1..=12`.
    Month,
    /// Day of week, `0..=7`.
    DayOfWeek,
    /// Year, `1970..=2099`.
    Year,
}

impl Part {
    /// Returns the Hutool/Calendar field index.
    #[must_use]
    pub const fn calendar_field(self) -> usize {
        match self {
            Self::Second => 0,
            Self::Minute => 1,
            Self::Hour => 2,
            Self::DayOfMonth => 3,
            Self::Month => 4,
            Self::DayOfWeek => 5,
            Self::Year => 6,
        }
    }

    /// Returns the inclusive minimum.
    #[must_use]
    pub const fn min(self) -> i32 {
        match self {
            Self::Second | Self::Minute | Self::Hour | Self::DayOfWeek => 0,
            Self::DayOfMonth | Self::Month => 1,
            Self::Year => 1970,
        }
    }

    /// Returns the inclusive maximum.
    #[must_use]
    pub const fn max(self) -> i32 {
        match self {
            Self::Second | Self::Minute => 59,
            Self::Hour => 23,
            Self::DayOfMonth => 31,
            Self::Month => 12,
            Self::DayOfWeek => 7,
            Self::Year => 2099,
        }
    }

    /// Validates and returns a field value.
    pub fn check_value(self, value: i32) -> Result<i32, CronError> {
        if (self.min()..=self.max()).contains(&value) {
            Ok(value)
        } else {
            Err(CronError::InvalidPartValue { part: self, value })
        }
    }

    /// Resolves a field by its zero-based expression index.
    pub fn of(index: usize) -> Result<Self, CronError> {
        [
            Self::Second,
            Self::Minute,
            Self::Hour,
            Self::DayOfMonth,
            Self::Month,
            Self::DayOfWeek,
            Self::Year,
        ]
        .get(index)
        .copied()
        .ok_or(CronError::InvalidPartIndex(index))
    }
}

/// Incrementally builds a seven-part cron expression.
#[derive(Debug, Clone)]
pub struct CronPatternBuilder {
    parts: [String; 7],
}

impl Default for CronPatternBuilder {
    fn default() -> Self {
        Self {
            parts: std::array::from_fn(|_| "*".to_owned()),
        }
    }
}

impl CronPatternBuilder {
    /// Creates an all-wildcard builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a comma-separated collection of values.
    pub fn set_values(&mut self, part: Part, values: &[i32]) -> Result<&mut Self, CronError> {
        if values.is_empty() {
            return Err(CronError::EmptyPartValues(part));
        }
        let values = values
            .iter()
            .map(|value| part.check_value(*value).map(|value| value.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        self.parts[part.calendar_field()] = values.join(",");
        Ok(self)
    }

    /// Sets an inclusive value range.
    pub fn set_range(&mut self, part: Part, begin: i32, end: i32) -> Result<&mut Self, CronError> {
        part.check_value(begin)?;
        part.check_value(end)?;
        if begin > end {
            return Err(CronError::InvalidPartRange { part, begin, end });
        }
        self.parts[part.calendar_field()] = format!("{begin}-{end}");
        Ok(self)
    }

    /// Sets a raw field after validating it with the parser engine.
    pub fn set(&mut self, part: Part, value: impl Into<String>) -> Result<&mut Self, CronError> {
        let value = value.into();
        let mut candidate = self.clone();
        candidate.parts[part.calendar_field()] = value;
        CronPattern::parse(candidate.build())?;
        *self = candidate;
        Ok(self)
    }

    /// Builds a seven-part expression.
    #[must_use]
    pub fn build(&self) -> String {
        self.parts.join(" ")
    }
}

/// Hutool-style cron pattern with support for `|` alternatives.
#[derive(Debug, Clone)]
pub struct CronPattern {
    expression: String,
    second_schedules: Vec<Schedule>,
    minute_schedules: Vec<Schedule>,
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
        for alternative in alternatives {
            second_schedules.push(Schedule::from_str(&normalize(alternative, true))?);
            minute_schedules.push(
                Schedule::from_str(&normalize(alternative, false))
                    .expect("replacing a valid seconds field with zero remains valid"),
            );
        }
        Ok(Self {
            expression,
            second_schedules,
            minute_schedules,
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
        self.schedules(match_second).iter().any(|schedule| {
            schedule
                .after(&(instant - ChronoDuration::seconds(1)))
                .next()
                == Some(instant)
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
            .filter_map(|schedule| schedule.after(&start).next())
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

fn normalize(expression: &str, match_second: bool) -> String {
    let mut fields = expression.split_whitespace().collect::<Vec<_>>();
    match fields.len() {
        5 => {
            fields.insert(0, "0");
            fields.push("*");
        }
        6 => fields.push("*"),
        7 => {}
        _ => return expression.to_owned(),
    }
    if !match_second {
        fields[0] = "0";
    }
    fields.join(" ")
}

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
}

/// Common behavior of one cron field matcher.
pub trait PartMatcher: fmt::Debug + Send + Sync {
    /// Returns whether `value` matches.
    fn matches(&self, value: i32) -> bool;
    /// Returns the first represented value at or after `value`, wrapping to the minimum.
    fn next_after(&self, value: i32) -> i32;
}

/// Matcher that accepts every value.
#[derive(Debug, Clone, Copy, Default)]
pub struct AlwaysTrueMatcher;

impl PartMatcher for AlwaysTrueMatcher {
    fn matches(&self, _value: i32) -> bool {
        true
    }

    fn next_after(&self, value: i32) -> i32 {
        value
    }
}

impl fmt::Display for AlwaysTrueMatcher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("*")
    }
}

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

/// Year matcher with Hutool's lower-bound behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YearValueMatcher(BoolArrayMatcher);

impl YearValueMatcher {
    /// Creates a year matcher.
    pub fn new(values: impl IntoIterator<Item = i32>) -> Result<Self, CronError> {
        Self::from_values(values.into_iter().collect())
    }

    fn from_values(values: Vec<i32>) -> Result<Self, CronError> {
        let mut checked = Vec::new();
        for value in values {
            checked.push(Part::Year.check_value(value)?);
        }
        Ok(Self(BoolArrayMatcher::new(checked)?))
    }
}

impl PartMatcher for YearValueMatcher {
    fn matches(&self, value: i32) -> bool {
        self.0.matches(value)
    }

    fn next_after(&self, value: i32) -> i32 {
        self.0.next_after(value)
    }
}

/// Day-of-month matcher supporting Hutool's `L` sentinel (`32`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayOfMonthMatcher {
    values: BoolArrayMatcher,
    last: bool,
}

impl DayOfMonthMatcher {
    /// Creates a day matcher. Value `32` represents the last day.
    pub fn new(values: impl IntoIterator<Item = i32>) -> Result<Self, CronError> {
        Self::from_values(values.into_iter().collect())
    }

    fn from_values(values: Vec<i32>) -> Result<Self, CronError> {
        let mut last = false;
        let mut concrete = Vec::new();
        for value in values {
            if value == 32 {
                last = true;
            } else {
                concrete.push(Part::DayOfMonth.check_value(value)?);
            }
        }
        if concrete.is_empty() {
            concrete.push(32);
        }
        Ok(Self {
            values: BoolArrayMatcher::new(concrete)
                .expect("a day matcher always contains a concrete value or the last-day sentinel"),
            last,
        })
    }

    /// Returns whether the last-day sentinel is enabled.
    #[must_use]
    pub const fn is_last(&self) -> bool {
        self.last
    }

    /// Returns the number of days in a month.
    #[must_use]
    pub const fn last_day(month: u32, leap_year: bool) -> u32 {
        match month {
            2 if leap_year => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    /// Matches a day with month/leap-year context.
    #[must_use]
    pub fn matches_day(&self, day: u32, month: u32, leap_year: bool) -> bool {
        self.values.matches(i32::try_from(day).unwrap_or(i32::MAX))
            || (self.last && day == Self::last_day(month, leap_year))
    }

    /// Returns the next matching day within the month, or the minimum match.
    #[must_use]
    pub fn next_day(&self, day: u32, month: u32, leap_year: bool) -> u32 {
        let last_day = Self::last_day(month, leap_year);
        (day..=last_day)
            .find(|candidate| self.matches_day(*candidate, month, leap_year))
            .or_else(|| {
                (1..=last_day).find(|candidate| self.matches_day(*candidate, month, leap_year))
            })
            .unwrap_or(day)
    }

    /// Returns the minimum concrete match for the month.
    #[must_use]
    pub fn min_value(&self, month: u32, leap_year: bool) -> u32 {
        self.next_day(1, month, leap_year)
    }

    /// Returns the maximum concrete match for the month.
    #[must_use]
    pub fn max_value(&self, month: u32, leap_year: bool) -> u32 {
        let last_day = Self::last_day(month, leap_year);
        (1..=last_day)
            .rev()
            .find(|candidate| self.matches_day(*candidate, month, leap_year))
            .unwrap_or(last_day)
    }
}

impl PartMatcher for DayOfMonthMatcher {
    fn matches(&self, value: i32) -> bool {
        self.values.matches(value) || (self.last && value == 32)
    }

    fn next_after(&self, value: i32) -> i32 {
        self.values.next_after(value)
    }
}

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

    /// Parses wildcards, lists, ranges, and steps.
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
            let (begin, end) = if base == "*" {
                (self.part.min(), self.part.max())
            } else if let Some((begin, end)) = base.split_once('-') {
                (parse_alias(self.part, begin)?, parse_alias(self.part, end)?)
            } else {
                let begin = parse_alias(self.part, base)?;
                (begin, begin)
            };
            self.part.check_value(begin)?;
            self.part.check_value(end)?;
            if begin > end {
                return Err(CronError::InvalidPartRange {
                    part: self.part,
                    begin,
                    end,
                });
            }
            let step = usize::try_from(step)
                .expect("a positive i32 step is representable as usize on supported targets");
            values.extend((begin..=end).step_by(step));
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

/// Parses full expressions into field matchers.
pub struct PatternParser;

impl PatternParser {
    /// Parses every `|`-separated expression.
    pub fn parse(expression: &str) -> Result<Vec<PatternMatcher>, CronError> {
        expression
            .split('|')
            .map(str::trim)
            .map(|alternative| {
                let normalized = normalize(alternative, true);
                let fields = normalized.split_whitespace().collect::<Vec<_>>();
                if fields.len() != 7 {
                    return Err(CronError::InvalidPattern(alternative.to_owned()));
                }
                let fields = [
                    PartParser::new(Part::Second).parse(fields[0])?,
                    PartParser::new(Part::Minute).parse(fields[1])?,
                    PartParser::new(Part::Hour).parse(fields[2])?,
                    PartParser::new(Part::DayOfMonth).parse(fields[3])?,
                    PartParser::new(Part::Month).parse(fields[4])?,
                    PartParser::new(Part::DayOfWeek).parse(fields[5])?,
                    PartParser::new(Part::Year).parse(fields[6])?,
                ];
                Ok(PatternMatcher::new(fields))
            })
            .collect()
    }
}

fn parse_alias(part: Part, value: &str) -> Result<i32, CronError> {
    let lowercase = value.to_ascii_lowercase();
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

/// Extracts Hutool's seven field values from a UTC date.
#[must_use]
pub fn fields<Tz: TimeZone>(value: &DateTime<Tz>, match_second: bool) -> [i32; 7] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts_builders_and_parsers_validate_every_field_shape() {
        assert_eq!(Part::of(0).unwrap(), Part::Second);
        assert!(Part::of(7).is_err());
        assert_eq!(Part::Hour.check_value(23).unwrap(), 23);
        assert!(Part::Hour.check_value(24).is_err());
        let all_parts = [
            Part::Second,
            Part::Minute,
            Part::Hour,
            Part::DayOfMonth,
            Part::Month,
            Part::DayOfWeek,
            Part::Year,
        ];
        for (index, part) in all_parts.into_iter().enumerate() {
            assert_eq!(part.calendar_field(), index);
            assert!(part.min() <= part.max());
        }

        let mut builder = CronPatternBuilder::new();
        builder
            .set_values(Part::Minute, &[5, 10])
            .unwrap()
            .set_range(Part::Hour, 8, 18)
            .unwrap()
            .set(Part::DayOfWeek, "mon-fri")
            .unwrap();
        assert_eq!(builder.build(), "* 5,10 8-18 * * mon-fri *");
        assert!(builder.set_values(Part::Minute, &[]).is_err());
        assert!(builder.set_values(Part::Minute, &[60]).is_err());
        assert!(builder.set_range(Part::Hour, 18, 8).is_err());
        assert!(builder.set_range(Part::Hour, -1, 8).is_err());
        assert!(builder.set_range(Part::Hour, 8, 24).is_err());
        assert!(builder.set(Part::Minute, "invalid").is_err());

        let parsed = PatternParser::parse("*/5 * * * * | 0 0 * * sun").unwrap();
        assert_eq!(parsed.len(), 2);
        assert!(parsed[0].matches([0, 10, 1, 1, 1, 1, 2026]));
        assert!(parsed[1].matches_week(0));
        assert!(PartParser::new(Part::Minute).parse("*/0").is_err());
        assert!(PartParser::new(Part::Minute).parse("10-2").is_err());
        assert!(PartParser::new(Part::Minute).parse("bad-2").is_err());
        assert!(PartParser::new(Part::Minute).parse("2-bad").is_err());
        assert!(PartParser::new(Part::Minute).parse("60").is_err());
        assert!(PartParser::new(Part::Minute).parse("1-60").is_err());
        assert!(PartParser::new(Part::Minute).parse("*/-1").is_err());
        assert!(
            PartParser::new(Part::Year)
                .parse("2026,2027")
                .unwrap()
                .matches(2026)
        );
        assert!(
            PartParser::new(Part::DayOfMonth)
                .parse("1,2")
                .unwrap()
                .matches(2)
        );
        assert!(
            PartParser::new(Part::Month)
                .parse("jan-mar")
                .unwrap()
                .matches(2)
        );
        assert!(
            PartParser::new(Part::DayOfWeek)
                .parse("sun")
                .unwrap()
                .matches(0)
        );
        assert!(
            PartParser::new(Part::Month)
                .parse("12")
                .unwrap()
                .matches(12)
        );
        assert!(
            PartParser::new(Part::DayOfWeek)
                .parse("7")
                .unwrap()
                .matches(7)
        );
        assert!(PartParser::new(Part::Month).parse("no-month").is_err());
        assert!(PartParser::new(Part::DayOfWeek).parse("no-day").is_err());
        assert!(PartParser::new(Part::Year).parse("1900").is_err());
        assert!(PartParser::new(Part::DayOfMonth).parse("0").is_err());
        assert!(PatternParser::parse("one two three four").is_err());
        for expression in [
            "bad 0 0 1 1 0 2026",
            "0 bad 0 1 1 0 2026",
            "0 0 bad 1 1 0 2026",
            "0 0 0 bad 1 0 2026",
            "0 0 0 1 bad 0 2026",
            "0 0 0 1 1 bad 2026",
            "0 0 0 1 1 0 bad",
        ] {
            assert!(PatternParser::parse(expression).is_err());
        }
        let sunday_seven = PatternParser::parse("0 0 0 * * 7 2026").unwrap();
        assert!(sunday_seven[0].matches_week(0));
    }

    #[test]
    fn patterns_match_advance_and_bound_results() {
        let pattern = CronPattern::of("*/5 * * * * | 30 * * * * *").unwrap();
        let start = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        assert!(pattern.matches(start, false));
        assert!(
            pattern
                .matches_millis(start.timestamp_millis(), false)
                .unwrap()
        );
        assert_eq!(pattern.next_match(start, false), Some(start));
        assert_eq!(
            pattern.next_match(start + ChronoDuration::seconds(1), true),
            Some(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 30).unwrap())
        );
        assert_eq!(
            pattern.next_match_after(start, true).unwrap(),
            Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 30).unwrap()
        );
        let dates = CronPatternUtil::matched_dates(
            &pattern,
            start,
            start + ChronoDuration::minutes(11),
            3,
            false,
        )
        .unwrap();
        assert_eq!(dates.len(), 3);
        assert_eq!(
            CronPatternUtil::next_date_after(&pattern, start).unwrap(),
            Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 30).unwrap()
        );
        assert_eq!(
            CronPatternUtil::next_date_after_with_precision(&pattern, start, false).unwrap(),
            Utc.with_ymd_and_hms(2026, 1, 1, 0, 1, 0).unwrap()
        );
        assert!(
            CronPatternUtil::matched_dates(
                &CronPattern::parse("0 0 0 1 1 * 1970").unwrap(),
                start,
                start + ChronoDuration::days(1),
                1,
                true,
            )
            .unwrap()
            .is_empty()
        );
        assert!(
            CronPatternUtil::matched_dates(
                &pattern,
                start + ChronoDuration::seconds(1),
                start + ChronoDuration::seconds(2),
                2,
                true,
            )
            .unwrap()
            .is_empty()
        );
        assert!(
            CronPatternUtil::matched_dates(
                &pattern,
                start,
                start - ChronoDuration::seconds(1),
                1,
                true
            )
            .is_err()
        );
        assert_eq!(pattern.to_string(), "*/5 * * * * | 30 * * * * *");
    }

    #[test]
    fn finite_day_year_and_wildcard_matchers_cover_boundaries() {
        let wildcard = AlwaysTrueMatcher;
        assert!(wildcard.matches(999));
        assert_eq!(wildcard.next_after(4), 4);
        assert_eq!(wildcard.to_string(), "*");

        let finite = BoolArrayMatcher::new([2, 4, 4]).unwrap();
        assert!(finite.matches(4));
        assert_eq!(finite.next_after(3), 4);
        assert_eq!(finite.next_after(5), 2);
        assert_eq!((finite.min_value(), finite.max_value()), (2, 4));
        assert_eq!(finite.to_string(), "2,4");
        assert!(BoolArrayMatcher::new([]).is_err());

        let year = YearValueMatcher::new([2026, 2028]).unwrap();
        assert!(year.matches(2026));
        assert_eq!(year.next_after(2027), 2028);
        assert!(YearValueMatcher::new([1900]).is_err());
        assert!(YearValueMatcher::new([]).is_err());

        let days = DayOfMonthMatcher::new([1, 32]).unwrap();
        assert!(days.is_last());
        assert!(days.matches_day(29, 2, true));
        assert_eq!(days.next_day(2, 2, false), 28);
        assert_eq!(days.min_value(2, false), 1);
        assert_eq!(days.max_value(2, false), 28);
        assert_eq!(DayOfMonthMatcher::last_day(4, false), 30);
        assert_eq!(DayOfMonthMatcher::last_day(2, false), 28);
        assert_eq!(DayOfMonthMatcher::last_day(3, false), 31);
        assert!(days.matches(32));
        assert_eq!(days.next_after(2), 1);
        let only_last = DayOfMonthMatcher::new([32]).unwrap();
        assert!(only_last.matches_day(31, 1, false));
        let impossible = DayOfMonthMatcher {
            values: BoolArrayMatcher::new([32]).unwrap(),
            last: false,
        };
        assert_eq!(impossible.next_day(3, 2, false), 3);
        assert_eq!(impossible.max_value(2, false), 28);
        assert!(DayOfMonthMatcher::new([0]).is_err());
    }

    #[test]
    fn field_extraction_and_alias_errors_are_explicit() {
        let instant = Utc.with_ymd_and_hms(2026, 7, 5, 6, 7, 8).unwrap();
        assert_eq!(fields(&instant, true), [8, 7, 6, 5, 7, 0, 2026]);
        assert_eq!(fields(&instant, false)[0], 0);
        assert!(CronPattern::parse("bad").is_err());
        assert!(
            CronPattern::parse("* * * * *")
                .unwrap()
                .matches_millis(i64::MAX, true)
                .is_err()
        );
        assert!(CronPattern::parse("* * * * * |").is_err());
        assert!(PatternParser::parse("0 x * * *").is_err());
    }
}
