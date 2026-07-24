//! Browser, rendering-engine, operating-system, and platform detection.

use regex::{Regex, RegexBuilder};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::{OnceLock, RwLock},
};
use woothee::parser::Parser as WootheeParser;

use super::user_agent::UserAgent;
use super::user_agent_parser::UserAgentParser;

/// Convenience facade matching Hutool's `UserAgentUtil` role.
#[derive(Debug, Clone, Copy, Default)]
pub struct UserAgentUtil;

impl UserAgentUtil {
    /// Parses a User-Agent string.
    #[must_use]
    pub fn parse(user_agent: &str) -> Option<UserAgent> {
        UserAgentParser::parse(user_agent)
    }
}
