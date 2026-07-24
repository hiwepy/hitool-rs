//! Browser, rendering-engine, operating-system, and platform detection.

use regex::{Regex, RegexBuilder};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::{OnceLock, RwLock},
};
use woothee::parser::Parser as WootheeParser;

mod rule_error;
mod user_agent_info;
mod browser;
mod engine;
mod operating_system;
mod platform;
mod user_agent;
mod user_agent_parser;
mod user_agent_util;

pub use rule_error::RuleError;
pub use user_agent_info::UserAgentInfo;
pub use browser::Browser;
pub use engine::Engine;
pub use operating_system::OperatingSystem;
pub use platform::Platform;
pub use user_agent::UserAgent;
pub use user_agent_parser::UserAgentParser;
pub use user_agent_util::UserAgentUtil;
pub use user_agent_info::UNKNOWN_NAME;
