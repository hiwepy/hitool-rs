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
use super::task_listener::TaskListener;

/// Thread-safe listener collection.
#[derive(Clone, Default)]
pub struct TaskListenerManager {
    listeners: Arc<RwLock<Vec<Arc<dyn TaskListener>>>>,
}

impl fmt::Debug for TaskListenerManager {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TaskListenerManager")
            .field(
                "listener_count",
                &self
                    .listeners
                    .read()
                    .expect("listener manager poisoned")
                    .len(),
            )
            .finish()
    }
}

impl TaskListenerManager {
    /// Adds a listener.
    pub fn add_listener(&self, listener: Arc<dyn TaskListener>) -> &Self {
        self.listeners
            .write()
            .expect("listener manager poisoned")
            .push(listener);
        self
    }

    /// Removes a listener by shared identity.
    pub fn remove_listener(&self, listener: &Arc<dyn TaskListener>) -> bool {
        let mut listeners = self.listeners.write().expect("listener manager poisoned");
        if let Some(index) = listeners
            .iter()
            .position(|candidate| Arc::ptr_eq(candidate, listener))
        {
            listeners.remove(index);
            true
        } else {
            false
        }
    }

    fn snapshot(&self) -> Vec<Arc<dyn TaskListener>> {
        self.listeners
            .read()
            .expect("listener manager poisoned")
            .clone()
    }

    /// Notifies start listeners.
    pub fn notify_task_start(&self, executor: &TaskExecutor) {
        for listener in self.snapshot() {
            listener.on_start(executor);
        }
    }

    /// Notifies success listeners.
    pub fn notify_task_succeeded(&self, executor: &TaskExecutor) {
        for listener in self.snapshot() {
            listener.on_succeeded(executor);
        }
    }

    /// Notifies failure listeners.
    pub fn notify_task_failed(&self, executor: &TaskExecutor, error: &CronError) {
        for listener in self.snapshot() {
            listener.on_failed(executor, error);
        }
    }
}
