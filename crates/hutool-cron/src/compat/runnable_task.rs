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

use super::task::Task;

/// Adapts an infallible Rust closure to `Task`.
pub struct RunnableTask<F> {
    runnable: F,
}

impl<F> RunnableTask<F>
where
    F: Fn() + Send + Sync + 'static,
{
    /// Creates a task adapter.
    #[must_use]
    pub const fn new(runnable: F) -> Self {
        Self { runnable }
    }
}

impl<F> Task for RunnableTask<F>
where
    F: Fn() + Send + Sync + 'static,
{
    fn execute(&self) -> Result<(), CronError> {
        (self.runnable)();
        Ok(())
    }
}
