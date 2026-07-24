use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, ReentrantMutex, RwLock};

use super::no_cache::NoCache;
use super::scheduled_timed_cache::ScheduledTimedCache;
use super::timed_cache::TimedCache;
use super::weak_cache::WeakCache;

/// Hutool-named constructors.
pub struct CacheUtil;

impl CacheUtil {
    /// Creates FIFO cache without expiration.
    pub fn new_fifo_cache<K, V>(capacity: usize) -> FIFOCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        FIFOCache::new(capacity)
    }

    /// Creates FIFO cache with expiration.
    pub fn new_fifo_cache_with_timeout<K, V>(capacity: usize, timeout: Duration) -> FIFOCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        FIFOCache::with_timeout(capacity, timeout)
    }

    /// Creates LFU cache without expiration.
    pub fn new_lfu_cache<K, V>(capacity: usize) -> LFUCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        LFUCache::new(capacity)
    }

    /// Creates LFU cache with expiration.
    pub fn new_lfu_cache_with_timeout<K, V>(capacity: usize, timeout: Duration) -> LFUCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        LFUCache::with_timeout(capacity, timeout)
    }

    /// Creates LRU cache without expiration.
    pub fn new_lru_cache<K, V>(capacity: usize) -> LRUCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        LRUCache::new(capacity)
    }

    /// Creates LRU cache with expiration.
    pub fn new_lru_cache_with_timeout<K, V>(capacity: usize, timeout: Duration) -> LRUCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        LRUCache::with_timeout(capacity, timeout)
    }

    /// Creates an unscheduled timed cache.
    pub fn new_timed_cache<K, V>(timeout: Duration) -> TimedCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        TimedCache::new(timeout)
    }

    /// Creates and schedules a timed cache.
    pub fn new_scheduled_timed_cache<K, V>(
        timeout: Duration,
        delay: Duration,
    ) -> Result<ScheduledTimedCache<K, V>, &'static str>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        let cache = TimedCache::new(timeout);
        cache.schedule_prune(delay)?;
        Ok(ScheduledTimedCache { cache })
    }

    /// Creates a weak-value cache.
    pub fn new_weak_cache<K, V>(timeout: Option<Duration>) -> WeakCache<K, V>
    where
        K: Eq + Hash + Clone,
    {
        WeakCache::new(timeout)
    }

    /// Creates a no-op cache.
    pub const fn new_no_cache<K, V>() -> NoCache<K, V> {
        NoCache::new()
    }
}
