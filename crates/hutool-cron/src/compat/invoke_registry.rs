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

/// Explicit method registry replacing Java reflection and classpath lookup.
#[derive(Clone, Default)]
pub struct InvokeRegistry {
    methods: Arc<RwLock<std::collections::HashMap<String, Arc<dyn Task>>>>,
}

impl fmt::Debug for InvokeRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InvokeRegistry")
            .field(
                "method_count",
                &self.methods.read().expect("invoke registry poisoned").len(),
            )
            .finish()
    }
}

impl InvokeRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers or replaces a named task.
    pub fn register<T>(&self, name: impl Into<String>, task: T) -> Option<Arc<dyn Task>>
    where
        T: Task,
    {
        self.methods
            .write()
            .expect("invoke registry poisoned")
            .insert(name.into(), Arc::new(task))
    }

    /// Resolves a named task.
    #[must_use]
    pub fn resolve(&self, name: &str) -> Option<Arc<dyn Task>> {
        self.methods
            .read()
            .expect("invoke registry poisoned")
            .get(name)
            .cloned()
    }
}
