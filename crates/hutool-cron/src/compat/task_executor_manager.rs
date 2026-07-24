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
use super::task_executor::TaskExecutor;
use super::task_listener_manager::TaskListenerManager;

/// Tracks currently spawned blocking executions.
#[derive(Debug, Clone)]
pub struct TaskExecutorManager {
    listeners: TaskListenerManager,
    executors: Arc<RwLock<Vec<TaskExecutor>>>,
}

impl TaskExecutorManager {
    /// Creates an empty manager.
    #[must_use]
    pub fn new(listeners: TaskListenerManager) -> Self {
        Self {
            listeners,
            executors: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Returns a snapshot of active executions.
    #[must_use]
    pub fn executors(&self) -> Vec<TaskExecutor> {
        self.executors
            .read()
            .expect("executor manager poisoned")
            .clone()
    }

    /// Creates and records an executor.
    pub fn spawn_executor(&self, task: Arc<CronTask>) -> TaskExecutor {
        let executor = TaskExecutor::new(task, self.listeners.clone());
        self.executors
            .write()
            .expect("executor manager poisoned")
            .push(executor.clone());
        executor
    }

    /// Removes a completed executor by task identity.
    pub fn notify_executor_completed(&self, executor: &TaskExecutor) -> bool {
        let mut executors = self.executors.write().expect("executor manager poisoned");
        if let Some(index) = executors
            .iter()
            .position(|candidate| Arc::ptr_eq(candidate.cron_task(), executor.cron_task()))
        {
            executors.remove(index);
            true
        } else {
            false
        }
    }
}
