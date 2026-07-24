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
use super::task_listener_manager::TaskListenerManager;
use super::task_table::TaskTable;

/// Explicitly owned scheduler; it never creates a hidden runtime or global.
pub struct Scheduler {
    config: CronConfig,
    daemon: bool,
    runtime: Option<tokio::runtime::Handle>,
    task_table: Arc<RwLock<TaskTable>>,
    listeners: TaskListenerManager,
    worker: Option<JoinHandle<()>>,
    next_id: u64,
}
