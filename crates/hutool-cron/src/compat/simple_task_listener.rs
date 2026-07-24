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

use super::task_listener::TaskListener;

/// No-op listener convenient for selective overrides.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleTaskListener;

impl TaskListener for SimpleTaskListener {}
