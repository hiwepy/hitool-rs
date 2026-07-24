//! `TimedCache` — 对齐 `cn.hutool.cache.impl.TimedCache`。定时过期缓存，每 key 独立超时。

use std::fmt;
use std::hash::Hash;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::compat::{AbstractCache, CachePolicy};
use crate::global_prune_timer::{GlobalPruneTimer, PruneHandle};

pub struct TimedCache<K, V> {
    cache: AbstractCache<K, V>,
    prune_handle: Arc<Mutex<Option<PruneHandle>>>,
}

impl<K, V> Clone for TimedCache<K, V> {
    fn clone(&self) -> Self {
        Self { cache: self.cache.clone(), prune_handle: Arc::clone(&self.prune_handle) }
    }
}

impl<K: Eq + Hash, V> fmt::Debug for TimedCache<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TimedCache").field(&self.cache).finish()
    }
}

impl<K, V> TimedCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    #[must_use]
    pub fn new(timeout: std::time::Duration) -> Self {
        Self {
            cache: AbstractCache::new(0, Some(timeout), CachePolicy::Timed),
            prune_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn schedule_prune(&self, delay: std::time::Duration) -> Result<(), &'static str> {
        if delay.is_zero() { return Err("prune delay must be greater than zero"); }
        self.cancel_prune_schedule();
        let cache = self.cache.clone();
        *self.prune_handle.lock() = Some(GlobalPruneTimer::schedule(move || { cache.prune(); }, delay));
        Ok(())
    }

    pub fn cancel_prune_schedule(&self) -> bool {
        self.prune_handle.lock().take().is_some()
    }
}

impl<K, V> std::ops::Deref for TimedCache<K, V> {
    type Target = AbstractCache<K, V>;
    fn deref(&self) -> &Self::Target { &self.cache }
}
