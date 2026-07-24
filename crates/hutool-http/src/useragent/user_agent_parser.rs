//! Browser, rendering-engine, operating-system, and platform detection.

use regex::{Regex, RegexBuilder};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::{OnceLock, RwLock},
};
use woothee::parser::Parser as WootheeParser;

use super::browser::Browser;
use super::engine::Engine;
use super::operating_system::OperatingSystem;
use super::platform::Platform;
use super::user_agent::UserAgent;

/// Stateless User-Agent parser facade.
#[derive(Debug, Clone, Copy, Default)]
pub struct UserAgentParser;

impl UserAgentParser {
    /// Parses a non-blank User-Agent string.
    #[must_use]
    pub fn parse(user_agent: &str) -> Option<UserAgent> {
        if user_agent.trim().is_empty() {
            return None;
        }

        let browser = find_browser(user_agent);
        let engine = find_engine(user_agent);
        let os = find_operating_system(user_agent);
        let platform = find_platform(user_agent);
        let mobile = (platform.is_mobile() || browser.is_mobile()) && !os.is_macos();
        Some(UserAgent {
            version: browser.version(user_agent),
            engine_version: engine.version(user_agent),
            os_version: os.version(user_agent),
            mobile,
            browser,
            platform,
            os,
            engine,
        })
    }
}

fn find_browser(user_agent: &str) -> Browser {
    built_in_browsers()
        .iter()
        .chain(read_rules(custom_browsers()).iter())
        .find(|browser| browser.info.is_match(user_agent))
        .cloned()
        .or_else(|| woothee_browser(user_agent))
        .unwrap_or_else(unknown_browser)
}

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

fn find_platform(user_agent: &str) -> Platform {
    built_in_platforms()
        .iter()
        .find(|platform| platform.info.is_match(user_agent))
        .cloned()
        .unwrap_or_else(unknown_platform)
}

fn find_engine(user_agent: &str) -> Engine {
    built_in_engines()
        .iter()
        .find(|engine| engine.info.is_match(user_agent))
        .cloned()
        .unwrap_or_else(unknown_engine)
}

fn find_operating_system(user_agent: &str) -> OperatingSystem {
    built_in_operating_systems()
        .iter()
        .chain(read_rules(custom_operating_systems()).iter())
        .find(|os| os.info.is_match(user_agent))
        .cloned()
        .or_else(|| woothee_operating_system(user_agent))
        .unwrap_or_else(unknown_operating_system)
}
