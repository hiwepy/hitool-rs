use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, ReentrantMutex, RwLock};

use super::abstract_cache::AbstractCache;
use super::cache_policy::CachePolicy;
use super::global_prune_timer::GlobalPruneTimer;
use super::prune_handle::PruneHandle;

/// Unbounded expiring cache with optional scheduled pruning.
pub struct TimedCache<K, V> {
    cache: AbstractCache<K, V>,
    prune_handle: Arc<Mutex<Option<PruneHandle>>>,
}

impl<K, V> Clone for TimedCache<K, V> {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
            prune_handle: Arc::clone(&self.prune_handle),
        }
    }
}

impl<K, V> fmt::Debug for TimedCache<K, V>
where
    K: Eq + Hash,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("TimedCache")
            .field(&self.cache)
            .finish()
    }
}

impl<K, V> TimedCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    /// Creates an unbounded timed cache.
    #[must_use]
    pub fn new(timeout: Duration) -> Self {
        Self {
            cache: AbstractCache::new(0, Some(timeout), CachePolicy::Timed),
            prune_handle: Arc::new(Mutex::new(None)),
        }
    }

    /// Starts per-cache scheduled pruning, replacing an existing schedule.
    pub fn schedule_prune(&self, delay: Duration) -> Result<(), &'static str> {
        if delay.is_zero() {
            return Err("prune delay must be greater than zero");
        }
        self.cancel_prune_schedule();
        let cache = self.cache.clone();
        *self.prune_handle.lock() = Some(GlobalPruneTimer::schedule(
            move || {
                cache.prune();
            },
            delay,
        ));
        Ok(())
    }

    /// Cancels scheduled pruning.
    pub fn cancel_prune_schedule(&self) -> bool {
        self.prune_handle.lock().take().is_some()
    }
}

impl<K, V> std::ops::Deref for TimedCache<K, V> {
    type Target = AbstractCache<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}
