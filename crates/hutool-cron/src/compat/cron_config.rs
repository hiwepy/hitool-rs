//! Hutool-aligned task, listener, table, and scheduler facades.

#![allow(clippy::missing_fields_in_debug, clippy::missing_panics_doc)]

use std::{
    fmt,
    sync::{Arc, RwLock},
    time::Duration,
};

use chrono::Utc;
use tokio::{task::JoinHandle, time};

use crate::{CronError, CronPattern};

use super::scheduler::Scheduler;

/// Scheduler configuration.
#[derive(Debug, Clone)]
pub struct CronConfig {
    timezone: chrono::FixedOffset,
    match_second: bool,
}

impl Default for CronConfig {
    fn default() -> Self {
        Self {
            timezone: chrono::FixedOffset::east_opt(0).expect("UTC offset is valid"),
            match_second: false,
        }
    }
}

impl CronConfig {
    /// Sets the fixed timezone offset.
    pub fn set_timezone(&mut self, timezone: chrono::FixedOffset) -> &mut Self {
        self.timezone = timezone;
        self
    }

    /// Returns the fixed timezone offset.
    #[must_use]
    pub const fn timezone(&self) -> chrono::FixedOffset {
        self.timezone
    }

    /// Returns whether seconds are matched.
    #[must_use]
    pub const fn is_match_second(&self) -> bool {
        self.match_second
    }

    /// Sets second matching.
    pub fn set_match_second(&mut self, match_second: bool) -> &mut Self {
        self.match_second = match_second;
        self
    }
}
