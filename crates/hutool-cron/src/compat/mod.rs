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

mod task;
mod runnable_task;
mod invoke_registry;
mod invoke_task;
mod cron_task;
mod task_table;
mod task_listener;
mod simple_task_listener;
mod task_listener_manager;
mod task_executor;
mod task_executor_manager;
mod cron_config;
mod scheduler;
mod cron_setting_entry;
mod task_launcher;
mod task_launcher_manager;
mod cron_util;
mod cron_timer;

pub use task::Task;
pub use runnable_task::RunnableTask;
pub use invoke_registry::InvokeRegistry;
pub use invoke_task::InvokeTask;
pub use cron_task::CronTask;
pub use task_table::TaskTable;
pub use task_listener::TaskListener;
pub use simple_task_listener::SimpleTaskListener;
pub use task_listener_manager::TaskListenerManager;
pub use task_executor::TaskExecutor;
pub use task_executor_manager::TaskExecutorManager;
pub use cron_config::CronConfig;
pub use scheduler::Scheduler;
pub use cron_setting_entry::CronSettingEntry;
pub use task_launcher::TaskLauncher;
pub use task_launcher_manager::TaskLauncherManager;
pub use cron_util::CronUtil;
pub use cron_timer::CronTimer;
