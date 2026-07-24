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

/// Rendering-engine identification rule.
#[derive(Debug, Clone)]
pub struct Engine {
    info: UserAgentInfo,
    version_name: Option<String>,
}

impl Engine {
    /// Creates an engine rule and its conventional version matcher.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        let name = name.into();
        let version_name = (name != UNKNOWN_NAME).then(|| name.clone());
        Ok(Self {
            info: UserAgentInfo::new(name, regex)?,
            version_name,
        })
    }

    /// Returns the engine name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Extracts the engine version.
    #[must_use]
    pub fn version(&self, user_agent: &str) -> Option<String> {
        engine_version(self.version_name.as_deref()?, user_agent)
    }

    /// Returns whether this engine is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for Engine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}

impl PartialEq for Engine {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

impl Eq for Engine {}

impl Hash for Engine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.hash(state);
    }
}

pub(crate) const UNKNOWN_NAME: &str = "Unknown";

fn engine_version(name: &str, user_agent: &str) -> Option<String> {
    let start = user_agent
        .to_ascii_lowercase()
        .find(&name.to_ascii_lowercase())?
        + name.len();
    let value = user_agent[start..].strip_prefix(['/', '-', ' '])?;
    let end = value
        .find(|character: char| {
            !character.is_ascii_alphanumeric() && !matches!(character, '.' | '-')
        })
        .unwrap_or(value.len());
    (end > 0).then(|| value[..end].to_owned())
}
