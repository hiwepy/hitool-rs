use std::fmt;

use chrono::{Datelike, Local, NaiveDate};
use thiserror::Error;

/// Parsed information returned for ten-character Taiwan, Macao, and Hong Kong cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card10Info {
    region: &'static str,
    gender: char,
    valid: bool,
}

impl Card10Info {
    /// Returns the card's region name.
    #[must_use]
    pub const fn region(self) -> &'static str {
        self.region
    }

    /// Returns `M`, `F`, or `N`, matching Hutool's information array.
    #[must_use]
    pub const fn gender(self) -> char {
        self.gender
    }

    /// Returns whether the regional checksum or syntax is valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.valid
    }
}
