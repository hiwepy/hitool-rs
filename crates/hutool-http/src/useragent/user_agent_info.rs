//! Browser, rendering-engine, operating-system, and platform detection.

use regex::{Regex, RegexBuilder};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::{OnceLock, RwLock},
};
use woothee::parser::Parser as WootheeParser;

use super::rule_error::RuleError;

/// Named User-Agent matching rule.
#[derive(Debug, Clone)]
pub struct UserAgentInfo {
    name: String,
    pattern: Option<Regex>,
}

impl UserAgentInfo {
    /// Creates a case-insensitive matching rule.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        Ok(Self {
            name: name.into(),
            pattern: regex.map(case_insensitive_regex).transpose()?,
        })
    }

    /// Creates an information object from an already compiled pattern.
    #[must_use]
    pub fn from_pattern(name: impl Into<String>, pattern: Option<Regex>) -> Self {
        Self {
            name: name.into(),
            pattern,
        }
    }

    /// Returns the display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the compiled matching pattern, when one exists.
    #[must_use]
    pub const fn pattern(&self) -> Option<&Regex> {
        self.pattern.as_ref()
    }

    /// Returns whether this rule occurs in `content`.
    #[must_use]
    pub fn is_match(&self, content: &str) -> bool {
        self.pattern
            .as_ref()
            .is_some_and(|pattern| pattern.is_match(content))
    }

    /// Returns whether this represents an unidentified component.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.name == UNKNOWN_NAME
    }
}

impl PartialEq for UserAgentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for UserAgentInfo {}

impl Hash for UserAgentInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Display for UserAgentInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

fn case_insensitive_regex(pattern: &str) -> Result<Regex, RuleError> {
    RegexBuilder::new(pattern).case_insensitive(true).build()
}

pub(crate) const UNKNOWN_NAME: &str = "Unknown";
