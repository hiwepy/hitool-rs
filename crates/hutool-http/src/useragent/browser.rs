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

/// Browser identification rule.
#[derive(Debug, Clone)]
pub struct Browser {
    info: UserAgentInfo,
    version_pattern: Option<Regex>,
    mobile: bool,
}

impl Browser {
    /// Creates a browser rule.
    pub fn new(
        name: impl Into<String>,
        regex: Option<&str>,
        version_regex: Option<&str>,
    ) -> Result<Self, RuleError> {
        let name = name.into();
        Ok(Self {
            mobile: is_mobile_browser_name(&name),
            info: UserAgentInfo::new(name, regex)?,
            version_pattern: version_regex.map(case_insensitive_regex).transpose()?,
        })
    }

    /// Registers a browser rule after the built-in Hutool-compatible rules.
    pub fn add_custom_browser(
        name: impl Into<String>,
        regex: &str,
        version_regex: &str,
    ) -> Result<(), RuleError> {
        let browser = Self::new(name, Some(regex), Some(version_regex))?;
        write_rules(custom_browsers()).push(browser);
        Ok(())
    }

    /// Returns the browser name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Extracts the browser version.
    #[must_use]
    pub fn version(&self, user_agent: &str) -> Option<String> {
        capture(self.version_pattern.as_ref(), user_agent)
    }

    /// Returns whether the browser itself represents a mobile client.
    #[must_use]
    pub const fn is_mobile(&self) -> bool {
        self.mobile
    }

    /// Returns whether this browser is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for Browser {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}

impl PartialEq for Browser {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

impl Eq for Browser {}

impl Hash for Browser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.hash(state);
    }
}

fn capture(pattern: Option<&Regex>, content: &str) -> Option<String> {
    pattern?
        .captures(content)?
        .get(1)
        .map(|value| value.as_str().to_owned())
}

fn is_mobile_browser_name(name: &str) -> bool {
    matches!(
        name,
        "PSP"
            | "Yammer Mobile"
            | "Android Browser"
            | "IEMobile"
            | "MicroMessenger"
            | "miniProgram"
            | "DingTalk"
    )
}

fn case_insensitive_regex(pattern: &str) -> Result<Regex, RuleError> {
    RegexBuilder::new(pattern).case_insensitive(true).build()
}

fn write_rules<T>(rules: &'static RwLock<Vec<T>>) -> std::sync::RwLockWriteGuard<'static, Vec<T>> {
    rules
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn custom_browsers() -> &'static RwLock<Vec<Browser>> {
    static RULES: OnceLock<RwLock<Vec<Browser>>> = OnceLock::new();
    RULES.get_or_init(|| RwLock::new(Vec::new()))
}
