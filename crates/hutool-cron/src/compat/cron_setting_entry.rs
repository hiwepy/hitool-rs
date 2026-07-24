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

use super::cron_config::CronConfig;
use super::cron_task::CronTask;
use super::scheduler::Scheduler;
use super::task::Task;
use super::task_executor_manager::TaskExecutorManager;
use super::task_listener::TaskListener;
use super::task_listener_manager::TaskListenerManager;
use super::task_table::TaskTable;

/// One validated task entry used for explicit batch scheduling.
#[derive(Clone)]
pub struct CronSettingEntry {
    /// Stable task ID.
    pub id: String,
    /// Parsed pattern.
    pub pattern: CronPattern,
    /// Injected task implementation.
    pub task: Arc<dyn Task>,
}

impl fmt::Debug for CronSettingEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CronSettingEntry")
            .field("id", &self.id)
            .field("pattern", &self.pattern)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for Scheduler {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Scheduler")
            .field("config", &self.config)
            .field("daemon", &self.daemon)
            .field("started", &self.is_started())
            .field("task_count", &self.len())
            .finish()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    /// Creates a stopped scheduler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: CronConfig::default(),
            daemon: false,
            runtime: None,
            task_table: Arc::new(RwLock::new(TaskTable::new())),
            listeners: TaskListenerManager::default(),
            worker: None,
            next_id: 1,
        }
    }

    /// Sets the timezone used for configuration/reporting.
    pub fn set_timezone(&mut self, timezone: chrono::FixedOffset) -> &mut Self {
        self.config.set_timezone(timezone);
        self
    }

    /// Returns the configured timezone.
    #[must_use]
    pub const fn timezone(&self) -> chrono::FixedOffset {
        self.config.timezone()
    }

    /// Sets daemon shutdown semantics metadata.
    pub fn set_daemon(&mut self, daemon: bool) -> &mut Self {
        self.daemon = daemon;
        self
    }

    /// Returns daemon mode.
    #[must_use]
    pub const fn is_daemon(&self) -> bool {
        self.daemon
    }

    /// Injects the Tokio runtime used for scheduling and blocking tasks.
    pub fn set_runtime(&mut self, runtime: tokio::runtime::Handle) -> Result<&mut Self, CronError> {
        if self.is_started() {
            return Err(CronError::SchedulerAlreadyStarted);
        }
        self.runtime = Some(runtime);
        Ok(self)
    }

    /// Returns whether seconds are matched.
    #[must_use]
    pub const fn is_match_second(&self) -> bool {
        self.config.is_match_second()
    }

    /// Sets second matching.
    pub fn set_match_second(&mut self, value: bool) -> Result<&mut Self, CronError> {
        if self.is_started() {
            return Err(CronError::SchedulerAlreadyStarted);
        }
        self.config.set_match_second(value);
        Ok(self)
    }

    /// Adds a listener.
    pub fn add_listener(&self, listener: Arc<dyn TaskListener>) -> &Self {
        self.listeners.add_listener(listener);
        self
    }

    /// Removes a listener.
    pub fn remove_listener(&self, listener: &Arc<dyn TaskListener>) -> bool {
        self.listeners.remove_listener(listener)
    }

    /// Schedules an auto-ID task.
    pub fn schedule<T>(&mut self, pattern: &str, task: T) -> Result<String, CronError>
    where
        T: Task,
    {
        self.schedule_arc(pattern, Arc::new(task))
    }

    fn schedule_arc(&mut self, pattern: &str, task: Arc<dyn Task>) -> Result<String, CronError> {
        let id = format!("hutool-cron-{}", self.next_id);
        self.next_id = self.next_id.wrapping_add(1);
        self.schedule_owned(id.clone(), CronPattern::parse(pattern)?, task)?;
        Ok(id)
    }

    /// Schedules an explicit-ID task.
    pub fn schedule_with_id(
        &self,
        id: impl Into<String>,
        pattern: CronPattern,
        task: Arc<dyn Task>,
    ) -> Result<&Self, CronError> {
        self.schedule_owned(id.into(), pattern, task)
    }

    fn schedule_owned(
        &self,
        id: String,
        pattern: CronPattern,
        task: Arc<dyn Task>,
    ) -> Result<&Self, CronError> {
        self.task_table
            .write()
            .expect("task table poisoned")
            .add(CronTask::new(id, pattern, task))?;
        Ok(self)
    }

    /// Adds every entry from an explicitly parsed setting.
    pub fn schedule_setting(
        &self,
        entries: impl IntoIterator<Item = CronSettingEntry>,
    ) -> Result<&Self, CronError> {
        for entry in entries {
            self.schedule_owned(entry.id, entry.pattern, entry.task)?;
        }
        Ok(self)
    }

    /// Removes a task and ignores absence.
    pub fn deschedule(&self, id: &str) -> &Self {
        self.deschedule_with_status(id);
        self
    }

    /// Removes a task and reports whether it existed.
    pub fn deschedule_with_status(&self, id: &str) -> bool {
        self.task_table
            .write()
            .expect("task table poisoned")
            .remove(id)
    }

    /// Updates a task pattern.
    pub fn update_pattern(&self, id: &str, pattern: CronPattern) -> bool {
        self.task_table
            .read()
            .expect("task table poisoned")
            .update_pattern(id, pattern)
    }

    /// Returns the shared task table for read-only inspection.
    #[must_use]
    pub fn task_table(&self) -> Arc<RwLock<TaskTable>> {
        Arc::clone(&self.task_table)
    }

    /// Returns a task pattern.
    #[must_use]
    pub fn pattern(&self, id: &str) -> Option<CronPattern> {
        self.task_table
            .read()
            .expect("task table poisoned")
            .get_pattern(id)
    }

    /// Returns a scheduled task.
    #[must_use]
    pub fn task(&self, id: &str) -> Option<Arc<CronTask>> {
        self.task_table
            .read()
            .expect("task table poisoned")
            .get_task(id)
    }

    /// Returns whether the schedule is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the scheduled task count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.task_table.read().expect("task table poisoned").len()
    }

    /// Clears all tasks.
    pub fn clear(&self) -> &Self {
        *self.task_table.write().expect("task table poisoned") = TaskTable::new();
        self
    }

    /// Returns whether the scheduler worker is active.
    #[must_use]
    pub const fn is_started(&self) -> bool {
        self.worker.is_some()
    }

    /// Starts on the injected runtime or the current Tokio runtime.
    pub fn start(&mut self) -> Result<&mut Self, CronError> {
        if self.is_started() {
            return Err(CronError::SchedulerAlreadyStarted);
        }
        let runtime = self
            .runtime
            .clone()
            .or_else(|| tokio::runtime::Handle::try_current().ok())
            .ok_or(CronError::MissingRuntime)?;
        let table = Arc::clone(&self.task_table);
        let manager = TaskExecutorManager::new(self.listeners.clone());
        let match_second = self.config.is_match_second();
        let worker_runtime = runtime.clone();
        let worker = runtime.spawn(async move {
            let period = if match_second {
                Duration::from_secs(1)
            } else {
                Duration::from_secs(60)
            };
            let mut ticks = time::interval(period);
            loop {
                ticks.tick().await;
                let tasks = table
                    .read()
                    .expect("task table poisoned")
                    .matching(Utc::now().timestamp_millis(), match_second);
                for task in tasks {
                    let executor = manager.spawn_executor(task);
                    let completed = manager.clone();
                    worker_runtime.spawn_blocking(move || {
                        let _ = executor.run();
                        completed.notify_executor_completed(&executor);
                    });
                }
            }
        });
        self.worker = Some(worker);
        Ok(self)
    }

    /// Stops the scheduler, optionally clearing tasks.
    pub fn stop(&mut self, clear_tasks: bool) -> &mut Self {
        if let Some(worker) = self.worker.take() {
            worker.abort();
        }
        if clear_tasks {
            self.clear();
        }
        self
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        self.stop(false);
    }
}
