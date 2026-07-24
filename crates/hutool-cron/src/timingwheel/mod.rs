//! Explicitly owned timer and timing-wheel primitives.

#![allow(clippy::missing_panics_doc)]

use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt,
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::CronError;

mod timer_task;
mod timer_task_list;
mod timing_wheel;
mod system_timer;

pub use timer_task::TimerTask;
pub use timer_task_list::TimerTaskList;
pub use timing_wheel::TimingWheel;
pub use system_timer::SystemTimer;
