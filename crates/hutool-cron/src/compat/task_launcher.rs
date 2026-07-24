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
use super::task_executor::TaskExecutor;
use super::task_listener_manager::TaskListenerManager;
use super::task_table::TaskTable;

/// Executes every task matching one timestamp.
#[derive(Debug, Clone)]
pub struct TaskLauncher {
    table: Arc<RwLock<TaskTable>>,
    listeners: TaskListenerManager,
    millis: i64,
    match_second: bool,
}

impl TaskLauncher {
    /// Creates a one-shot launcher.
    #[must_use]
    pub fn new(scheduler: &Scheduler, millis: i64) -> Self {
        Self {
            table: scheduler.task_table(),
            listeners: scheduler.listeners.clone(),
            millis,
            match_second: scheduler.is_match_second(),
        }
    }

    /// Executes all matching tasks and returns their results.
    #[must_use]
    pub fn run(&self) -> Vec<Result<(), CronError>> {
        self.table
            .read()
            .expect("task table poisoned")
            .matching(self.millis, self.match_second)
            .into_iter()
            .map(|task| TaskExecutor::new(task, self.listeners.clone()).run())
            .collect()
    }
}
