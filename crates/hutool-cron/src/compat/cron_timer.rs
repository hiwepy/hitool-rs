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

use super::scheduler::Scheduler;

/// Thin owned timer facade for compatibility with Hutool's `CronTimer`.
#[derive(Debug)]
pub struct CronTimer<'a> {
    scheduler: &'a mut Scheduler,
}

impl<'a> CronTimer<'a> {
    /// Creates a timer for a scheduler.
    pub fn new(scheduler: &'a mut Scheduler) -> Self {
        Self { scheduler }
    }

    /// Starts the scheduler.
    pub fn run(&mut self) -> Result<(), CronError> {
        self.scheduler.start().map(|_| ())
    }

    /// Stops the scheduler without clearing tasks.
    pub fn stop_timer(&mut self) {
        self.scheduler.stop(false);
    }
}
