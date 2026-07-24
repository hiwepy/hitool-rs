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

/// A task together with its stable ID and mutable schedule.
pub struct CronTask {
    id: String,
    pattern: RwLock<CronPattern>,
    task: Arc<dyn Task>,
}

impl fmt::Debug for CronTask {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CronTask")
            .field("id", &self.id)
            .field(
                "pattern",
                &self.pattern.read().expect("cron pattern poisoned"),
            )
            .finish_non_exhaustive()
    }
}

impl CronTask {
    /// Creates a scheduled task.
    #[must_use]
    pub fn new(id: impl Into<String>, pattern: CronPattern, task: Arc<dyn Task>) -> Self {
        Self {
            id: id.into(),
            pattern: RwLock::new(pattern),
            task,
        }
    }

    /// Executes the raw task.
    pub fn execute(&self) -> Result<(), CronError> {
        self.task.execute()
    }

    /// Returns the task ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns a snapshot of the current pattern.
    #[must_use]
    pub fn pattern(&self) -> CronPattern {
        self.pattern.read().expect("cron pattern poisoned").clone()
    }

    /// Replaces the schedule.
    pub fn set_pattern(&self, pattern: CronPattern) -> &Self {
        *self.pattern.write().expect("cron pattern poisoned") = pattern;
        self
    }

    /// Returns the underlying task.
    #[must_use]
    pub fn raw(&self) -> Arc<dyn Task> {
        Arc::clone(&self.task)
    }
}
