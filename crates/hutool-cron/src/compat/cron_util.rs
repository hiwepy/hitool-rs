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

use super::cron_setting_entry::CronSettingEntry;
use super::scheduler::Scheduler;
use super::task::Task;

/// Owned facade corresponding to Hutool's static `CronUtil` surface.
#[derive(Debug, Default)]
pub struct CronUtil {
    scheduler: Scheduler,
}

impl CronUtil {
    /// Creates an isolated facade.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the owned scheduler.
    #[must_use]
    pub const fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    /// Returns the owned scheduler mutably.
    pub fn scheduler_mut(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }

    /// Configures second matching before start.
    pub fn set_match_second(&mut self, value: bool) -> Result<&mut Self, CronError> {
        self.scheduler.set_match_second(value)?;
        Ok(self)
    }

    /// Adds an auto-ID task.
    pub fn schedule<T>(&mut self, pattern: &str, task: T) -> Result<String, CronError>
    where
        T: Task,
    {
        self.scheduler.schedule(pattern, task)
    }

    /// Adds an explicit-ID task.
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
        self.scheduler.schedule_owned(id, pattern, task)?;
        Ok(self)
    }

    /// Adds an explicit batch setting.
    pub fn schedule_setting(
        &self,
        entries: impl IntoIterator<Item = CronSettingEntry>,
    ) -> Result<&Self, CronError> {
        self.scheduler.schedule_setting(entries)?;
        Ok(self)
    }

    /// Removes a task.
    pub fn remove(&self, id: &str) -> bool {
        self.scheduler.deschedule_with_status(id)
    }

    /// Replaces a task pattern.
    pub fn update_pattern(&self, id: &str, pattern: CronPattern) -> bool {
        self.scheduler.update_pattern(id, pattern)
    }

    /// Starts the scheduler.
    pub fn start(&mut self) -> Result<&mut Self, CronError> {
        self.scheduler.start()?;
        Ok(self)
    }

    /// Restarts without clearing tasks.
    pub fn restart(&mut self) -> Result<&mut Self, CronError> {
        self.scheduler.stop(false).start()?;
        Ok(self)
    }

    /// Stops and clears tasks.
    pub fn stop(&mut self) -> &mut Self {
        self.scheduler.stop(true);
        self
    }
}
