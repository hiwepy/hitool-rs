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

use super::invoke_registry::InvokeRegistry;
use super::task::Task;

/// A named invocation resolved through an injected registry.
#[derive(Clone)]
pub struct InvokeTask {
    name: String,
    task: Arc<dyn Task>,
}

impl fmt::Debug for InvokeTask {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InvokeTask")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl InvokeTask {
    /// Resolves a Hutool-style `type::method` name without reflection.
    pub fn new(name: impl Into<String>, registry: &InvokeRegistry) -> Result<Self, CronError> {
        let name = name.into();
        let task = registry
            .resolve(&name)
            .ok_or_else(|| CronError::UnknownInvokeTask(name.clone()))?;
        Ok(Self { name, task })
    }

    /// Returns the registered method name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Task for InvokeTask {
    fn execute(&self) -> Result<(), CronError> {
        self.task.execute()
    }
}
