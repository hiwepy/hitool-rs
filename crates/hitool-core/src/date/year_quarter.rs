//! 对齐: `cn.hutool.core.date.YearQuarter`

#![allow(dead_code)]

use chrono::{Datelike, NaiveDate};

use crate::date::date_time::DateTime;
use crate::date::month::Month;
use crate::date::quarter::Quarter;
use crate::{CoreError, Result};

/// Java Year 范围近似。
const YEAR_MIN: i32 = -999_999_999;
const YEAR_MAX: i32 = 999_999_999;

/// 对齐 Java: `cn.hutool.core.date.YearQuarter`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YearQuarter {
    year: i32,
    quarter: Quarter,
}

impl YearQuarter {
    /// 对齐 Java: `YearQuarter.of(year, quarterValue)`。
    pub fn of(year: i32, quarter: i32) -> Result<Self> {
        if !(YEAR_MIN..=YEAR_MAX).contains(&year) {
            return Err(CoreError::InvalidArgument {
                name: "year",
                reason: "year out of range",
            });
        }
        let q = Quarter::of(quarter).ok_or(CoreError::InvalidArgument {
            name: "quarter",
            reason: "quarter must be 1..=4",
        })?;
        Ok(Self { year, quarter: q })
    }

    /// 对齐 Java: `YearQuarter.of(year, Quarter)`。
    pub fn of_quarter(year: i32, quarter: Quarter) -> Result<Self> {
        Self::of(year, quarter.get_value())
    }

    /// 从 LocalDate / DateTime。
    pub fn from_date(date: DateTime) -> Self {
        Self {
            year: date.year(),
            quarter: date.quarter_enum(),
        }
    }

    /// 从年月日。
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Self> {
        let _ = NaiveDate::from_ymd_opt(year, month, day).ok_or(CoreError::DateOverflow)?;
        let q = Quarter::from_month_value(month as i32)?;
        Self::of_quarter(year, q)
    }

    /// 从 YearMonth（year + month 1-12）。
    pub fn from_year_month(year: i32, month: u32) -> Result<Self> {
        let q = Quarter::from_month_value(month as i32)?;
        Self::of_quarter(year, q)
    }

    pub fn get_year(self) -> i32 {
        self.year
    }
    pub fn get_quarter(self) -> Quarter {
        self.quarter
    }
    pub fn get_quarter_value(self) -> i32 {
        self.quarter.get_value()
    }

    pub fn get_first_date(self) -> NaiveDate {
        let (m, d) = self.quarter.first_month_day();
        NaiveDate::from_ymd_opt(self.year, m, d).unwrap()
    }
    pub fn get_last_date(self) -> NaiveDate {
        let (m, d) = self.quarter.last_month_day();
        // leap year Feb in Q1? Q1 last is Mar 31
        let day = if m == 2 {
            Month::February.get_last_day(NaiveDate::from_ymd_opt(self.year, 1, 1).unwrap().leap_year())
                as u32
        } else {
            d
        };
        NaiveDate::from_ymd_opt(self.year, m, day).unwrap_or_else(|| {
            NaiveDate::from_ymd_opt(self.year, m, self.quarter.last_month().get_last_day(false) as u32)
                .unwrap()
        })
    }

    pub fn first_year_month(self) -> (i32, u32) {
        (self.year, self.quarter.first_month_value() as u32)
    }
    pub fn last_year_month(self) -> (i32, u32) {
        (self.year, self.quarter.last_month_value() as u32)
    }
    pub fn first_month(self) -> Month {
        self.quarter.first_month()
    }
    pub fn last_month(self) -> Month {
        self.quarter.last_month()
    }

    pub fn plus_quarters(self, quarters: i64) -> Result<Self> {
        let total = self.year as i64 * 4 + (self.quarter.get_value() as i64 - 1) + quarters;
        let y = total.div_euclid(4) as i32;
        let q = (total.rem_euclid(4) + 1) as i32;
        Self::of(y, q)
    }
    pub fn minus_quarters(self, quarters: i64) -> Result<Self> {
        self.plus_quarters(-quarters)
    }
    pub fn next_quarter(self) -> Result<Self> {
        self.plus_quarters(1)
    }
    pub fn last_quarter(self) -> Result<Self> {
        self.plus_quarters(-1)
    }
    pub fn plus_years(self, years: i64) -> Result<Self> {
        Self::of(self.year + years as i32, self.quarter.get_value())
    }
    pub fn minus_years(self, years: i64) -> Result<Self> {
        self.plus_years(-years)
    }
    pub fn next_year(self) -> Result<Self> {
        self.plus_years(1)
    }
    pub fn last_year(self) -> Result<Self> {
        self.plus_years(-1)
    }

    pub fn compare_to(self, other: Self) -> i32 {
        (self.year, self.quarter.get_value()).cmp(&(other.year, other.quarter.get_value())) as i32
    }
    pub fn is_before(self, other: Self) -> bool {
        self.compare_to(other) < 0
    }
    pub fn is_after(self, other: Self) -> bool {
        self.compare_to(other) > 0
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}

impl PartialOrd for YearQuarter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for YearQuarter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.year
            .cmp(&other.year)
            .then(self.quarter.get_value().cmp(&other.quarter.get_value()))
    }
}
