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

/// Stable insertion-ordered scheduled-task table.
#[derive(Default)]
pub struct TaskTable {
    entries: Vec<Arc<CronTask>>,
}

impl fmt::Debug for TaskTable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(&self.entries).finish()
    }
}

impl fmt::Display for TaskTable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self
            .entries
            .iter()
            .map(|task| format!("{} {}", task.id(), task.pattern()))
            .collect::<Vec<_>>()
            .join("\n");
        formatter.write_str(&output)
    }
}

impl TaskTable {
    /// Creates an empty table.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Creates an empty table with reserved capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
        }
    }

    /// Adds a unique ID.
    pub fn add(&mut self, task: CronTask) -> Result<&mut Self, CronError> {
        if self.get_task(task.id()).is_some() {
            return Err(CronError::DuplicateTaskId(task.id().to_owned()));
        }
        self.entries.push(Arc::new(task));
        Ok(self)
    }

    /// Returns all IDs in insertion order.
    #[must_use]
    pub fn ids(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|task| task.id().to_owned())
            .collect()
    }

    /// Returns all pattern snapshots.
    #[must_use]
    pub fn patterns(&self) -> Vec<CronPattern> {
        self.entries.iter().map(|task| task.pattern()).collect()
    }

    /// Returns all raw task handles.
    #[must_use]
    pub fn tasks(&self) -> Vec<Arc<dyn Task>> {
        self.entries.iter().map(|task| task.raw()).collect()
    }

    /// Removes an ID.
    pub fn remove(&mut self, id: &str) -> bool {
        if let Some(index) = self.entries.iter().position(|task| task.id() == id) {
            self.entries.remove(index);
            true
        } else {
            false
        }
    }

    /// Replaces one pattern and reports whether the ID exists.
    pub fn update_pattern(&self, id: &str, pattern: CronPattern) -> bool {
        self.get_task(id).is_some_and(|task| {
            task.set_pattern(pattern);
            true
        })
    }

    /// Returns a task by index.
    #[must_use]
    pub fn task_at(&self, index: usize) -> Option<Arc<CronTask>> {
        self.entries.get(index).cloned()
    }

    /// Returns a task by ID.
    #[must_use]
    pub fn get_task(&self, id: &str) -> Option<Arc<CronTask>> {
        self.entries.iter().find(|task| task.id() == id).cloned()
    }

    /// Returns a pattern by index.
    #[must_use]
    pub fn pattern_at(&self, index: usize) -> Option<CronPattern> {
        self.task_at(index).map(|task| task.pattern())
    }

    /// Returns a pattern by ID.
    #[must_use]
    pub fn get_pattern(&self, id: &str) -> Option<CronPattern> {
        self.get_task(id).map(|task| task.pattern())
    }

    /// Returns the task count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the table is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn matching(&self, millis: i64, match_second: bool) -> Vec<Arc<CronTask>> {
        self.entries
            .iter()
            .filter(|task| {
                task.pattern()
                    .matches_millis(millis, match_second)
                    .unwrap_or(false)
            })
            .cloned()
            .collect()
    }
}
