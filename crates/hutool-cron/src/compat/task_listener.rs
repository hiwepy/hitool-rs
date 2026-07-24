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

use super::task_executor::TaskExecutor;

/// Listener for task lifecycle events.
pub trait TaskListener: Send + Sync + 'static {
    /// Called immediately before execution.
    fn on_start(&self, _executor: &TaskExecutor) {}
    /// Called after successful execution.
    fn on_succeeded(&self, _executor: &TaskExecutor) {}
    /// Called after failed execution.
    fn on_failed(&self, _executor: &TaskExecutor, _error: &CronError) {}
}
