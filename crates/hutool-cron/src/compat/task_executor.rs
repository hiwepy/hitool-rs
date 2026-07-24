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

use super::cron_task::CronTask;
use super::task::Task;
use super::task_listener_manager::TaskListenerManager;

/// One concrete task execution.
#[derive(Clone)]
pub struct TaskExecutor {
    cron_task: Arc<CronTask>,
    listeners: TaskListenerManager,
}

impl fmt::Debug for TaskExecutor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TaskExecutor")
            .field("task_id", &self.cron_task.id())
            .finish_non_exhaustive()
    }
}

impl TaskExecutor {
    /// Creates an execution with an explicit listener manager.
    #[must_use]
    pub fn new(cron_task: Arc<CronTask>, listeners: TaskListenerManager) -> Self {
        Self {
            cron_task,
            listeners,
        }
    }

    /// Returns the raw task.
    #[must_use]
    pub fn task(&self) -> Arc<dyn Task> {
        self.cron_task.raw()
    }

    /// Returns the scheduled task.
    #[must_use]
    pub fn cron_task(&self) -> &Arc<CronTask> {
        &self.cron_task
    }

    /// Executes and emits lifecycle events.
    pub fn run(&self) -> Result<(), CronError> {
        self.listeners.notify_task_start(self);
        match self.cron_task.execute() {
            Ok(()) => {
                self.listeners.notify_task_succeeded(self);
                Ok(())
            }
            Err(error) => {
                self.listeners.notify_task_failed(self, &error);
                Err(error)
            }
        }
    }
}
