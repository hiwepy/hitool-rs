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

/// Synchronous task contract. Scheduler execution happens on Tokio's blocking
/// pool so a task cannot block the scheduling loop.
pub trait Task: Send + Sync + 'static {
    /// Executes one task invocation.
    fn execute(&self) -> Result<(), CronError>;
}

impl<F> Task for F
where
    F: Fn() -> Result<(), CronError> + Send + Sync + 'static,
{
    fn execute(&self) -> Result<(), CronError> {
        self()
    }
}
