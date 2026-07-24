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
use super::task_launcher::TaskLauncher;
use super::task_listener_manager::TaskListenerManager;
use super::task_table::TaskTable;

/// Factory for launchers.
#[derive(Debug, Clone)]
pub struct TaskLauncherManager {
    table: Arc<RwLock<TaskTable>>,
    listeners: TaskListenerManager,
    match_second: bool,
}

impl TaskLauncherManager {
    /// Captures a scheduler's shared resources.
    #[must_use]
    pub fn new(scheduler: &Scheduler) -> Self {
        Self {
            table: scheduler.task_table(),
            listeners: scheduler.listeners.clone(),
            match_second: scheduler.is_match_second(),
        }
    }

    /// Creates a launcher for one timestamp.
    #[must_use]
    pub fn launcher(&self, millis: i64) -> TaskLauncher {
        TaskLauncher {
            table: Arc::clone(&self.table),
            listeners: self.listeners.clone(),
            millis,
            match_second: self.match_second,
        }
    }
}
