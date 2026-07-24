//! Browser, rendering-engine, operating-system, and platform detection.

use regex::{Regex, RegexBuilder};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::{OnceLock, RwLock},
};
use woothee::parser::Parser as WootheeParser;

use super::rule_error::RuleError;
use super::user_agent_info::UserAgentInfo;

/// Operating-system identification rule.
#[derive(Debug, Clone)]
pub struct OperatingSystem {
    info: UserAgentInfo,
    version_pattern: Option<Regex>,
}

impl OperatingSystem {
    /// Creates an operating-system rule without version extraction.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        Self::with_version(name, regex, None)
    }

    /// Creates an operating-system rule with version extraction.
    pub fn with_version(
        name: impl Into<String>,
        regex: Option<&str>,
        version_regex: Option<&str>,
    ) -> Result<Self, RuleError> {
        Ok(Self {
            info: UserAgentInfo::new(name, regex)?,
            version_pattern: version_regex.map(case_insensitive_regex).transpose()?,
        })
    }

    /// Registers an operating-system rule after the built-in rules.
    pub fn add_custom_os(
        name: impl Into<String>,
        regex: &str,
        version_regex: &str,
    ) -> Result<(), RuleError> {
        let os = Self::with_version(name, Some(regex), Some(version_regex))?;
        write_rules(custom_operating_systems()).push(os);
        Ok(())
    }

    /// Returns the operating-system name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Extracts the operating-system version.
    #[must_use]
    pub fn version(&self, user_agent: &str) -> Option<String> {
        capture(self.version_pattern.as_ref(), user_agent).map(|version| version.replace('_', "."))
    }

    /// Returns whether this is macOS.
    #[must_use]
    pub fn is_macos(&self) -> bool {
        matches!(self.name(), "OSX" | "macOS" | "Mac OS X")
    }

    /// Returns whether this operating system is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}

impl PartialEq for OperatingSystem {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

impl Eq for OperatingSystem {}

impl Hash for OperatingSystem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.hash(state);
    }
}

fn custom_operating_systems() -> &'static RwLock<Vec<OperatingSystem>> {
    static RULES: OnceLock<RwLock<Vec<OperatingSystem>>> = OnceLock::new();
    RULES.get_or_init(|| RwLock::new(Vec::new()))
}

fn capture(pattern: Option<&Regex>, content: &str) -> Option<String> {
    pattern?
        .captures(content)?
        .get(1)
        .map(|value| value.as_str().to_owned())
}

fn write_rules<T>(rules: &'static RwLock<Vec<T>>) -> std::sync::RwLockWriteGuard<'static, Vec<T>> {
    rules
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn case_insensitive_regex(pattern: &str) -> Result<Regex, RuleError> {
    RegexBuilder::new(pattern).case_insensitive(true).build()
}
