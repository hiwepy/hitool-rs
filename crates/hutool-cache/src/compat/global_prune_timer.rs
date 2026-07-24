use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, ReentrantMutex, RwLock};

use super::prune_handle::PruneHandle;

/// Factory for explicit, per-cache prune workers.
pub struct GlobalPruneTimer;

impl GlobalPruneTimer {
    /// Creates an explicit repeating worker.
    pub fn schedule<F>(task: F, delay: Duration) -> PruneHandle
    where
        F: FnMut() + Send + 'static,
    {
        Self::schedule_boxed(Box::new(task), delay)
    }

    fn schedule_boxed(mut task: Box<dyn FnMut() + Send>, delay: Duration) -> PruneHandle {
        let delay = if delay.is_zero() {
            Duration::from_millis(1)
        } else {
            delay
        };
        let (stop, receiver) = mpsc::channel();
        let worker = thread::spawn(move || {
            loop {
                match receiver.recv_timeout(delay) {
                    Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(mpsc::RecvTimeoutError::Timeout) => task(),
                }
            }
        });
        PruneHandle {
            stop: Some(stop),
            worker: Some(worker),
        }
    }

    /// No-op compatibility hook; workers are created explicitly by `schedule`.
    pub const fn create() {}

    /// No-op compatibility hook; dropping a `PruneHandle` performs shutdown.
    pub const fn shutdown() {}

    /// Returns no orphan tasks because workers are owned by handles.
    pub fn shutdown_now() -> Vec<JoinHandle<()>> {
        Vec::new()
    }
}
