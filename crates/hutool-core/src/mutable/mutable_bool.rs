//! Mutable value containers corresponding to Hutool's `core.lang.mutable` package.
//!
//! Rust normally prefers ordinary mutable bindings. These small wrappers are
//! useful when mutation itself must be passed around as a value, while keeping
//! ownership and borrowing explicit.

#![allow(

use super::mutable::Mutable;

/// Mutable boolean value.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutableBool {
    value: bool,
}

impl MutableBool {
    /// Creates a mutable boolean value.
    pub const fn new(value: bool) -> Self {
        Self { value }
    }

    /// Parses using Java `Boolean.parseBoolean` semantics.
    pub fn parse(value: &str) -> Self {
        Self::new(value.eq_ignore_ascii_case("true"))
    }

    /// Returns the current primitive value.
    pub const fn get(&self) -> bool {
        self.value
    }

    /// Replaces the current value.
    pub fn set(&mut self, value: bool) {
        self.value = value;
    }

    /// Returns Java's boolean wrapper hash code.
    pub const fn java_hash_code(&self) -> i32 {
        if self.value { 1231 } else { 1237 }
    }

    /// Consumes the wrapper and returns its value.
    pub const fn into_inner(self) -> bool {
        self.value
    }
}

impl Mutable<bool> for MutableBool {
    fn get(&self) -> &bool {
        &self.value
    }

    fn get_mut(&mut self) -> &mut bool {
        &mut self.value
    }

    fn set(&mut self, value: bool) {
        self.value = value;
    }
}

impl From<bool> for MutableBool {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl FromStr for MutableBool {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(value))
    }
}

impl fmt::Display for MutableBool {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}
