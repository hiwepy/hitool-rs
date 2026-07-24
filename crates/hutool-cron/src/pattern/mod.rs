//! Hutool-aligned cron patterns, builders, parsers, and matchers.

#![allow(clippy::missing_panics_doc)]

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

mod part;
mod cron_pattern_builder;
mod cron_pattern;
mod cron_pattern_util;
mod part_matcher;
mod always_true_matcher;
mod bool_array_matcher;
mod year_value_matcher;
mod day_of_month_matcher;
mod pattern_matcher;
mod part_parser;
mod pattern_parser;

pub use part::Part;
pub use cron_pattern_builder::CronPatternBuilder;
pub use cron_pattern::CronPattern;
pub use cron_pattern_util::CronPatternUtil;
pub use part_matcher::PartMatcher;
pub use always_true_matcher::AlwaysTrueMatcher;
pub use bool_array_matcher::BoolArrayMatcher;
pub use year_value_matcher::YearValueMatcher;
pub use day_of_month_matcher::DayOfMonthMatcher;
pub use pattern_matcher::PatternMatcher;
pub use part_parser::PartParser;
pub use pattern_parser::PatternParser;
pub use cron_pattern_builder::fields;
