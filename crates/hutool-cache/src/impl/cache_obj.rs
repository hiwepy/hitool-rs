//! CacheObj + Entry — 对齐 cn.hutool.cache.impl.CacheObj。缓存条目元数据及内部 Entry 类型。

use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::compat::CacheListener;

pub struct CacheObj<K, V> {
    key: K,
    value: Arc<V>,
    ttl: Option<Duration>,
    created_at: Instant,
    last_access: Instant,
}

impl<K, V> CacheObj<K, V> {
    /// Returns the key.
    pub const fn key(&self) -> &K {
        &self.key
    }

    /// Returns a shared value handle.
    pub fn value(&self) -> Arc<V> {
        Arc::clone(&self.value)
    }

    /// Returns the entry time-to-idle, if configured.
    pub const fn ttl(&self) -> Option<Duration> {
        self.ttl
    }

    /// Returns the insertion instant.
    pub const fn created_at(&self) -> Instant {
        self.created_at
    }

    /// Returns the last refreshed access instant.
    pub const fn last_access(&self) -> Instant {
        self.last_access
    }

    /// Returns the calculated expiration instant.
    pub fn expired_time(&self) -> Option<Instant> {
        self.ttl.and_then(|ttl| self.last_access.checked_add(ttl))
    }

    /// Returns whether the snapshot is expired now.
    pub fn is_expired(&self) -> bool {
        self.ttl
            .is_some_and(|ttl| self.last_access.elapsed() >= ttl)
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for CacheObj<K, V> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CacheObj")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("ttl", &self.ttl)
            .finish_non_exhaustive()
    }
}

struct State<K, V> {
    entries: HashMap<K, Entry<V>>,
}

struct CacheInner<K, V> {
    capacity: usize,
    timeout: Option<Duration>,
    policy: CachePolicy,
    state: Mutex<State<K, V>>,
    listener: RwLock<Option<Arc<dyn CacheListener<K, V>>>>,
    factory_lock: ReentrantMutex<()>,
    sequence: AtomicU64,
    hits: AtomicU64,
    misses: AtomicU64,
}

/// Thread-safe Hutool-compatible cache engine with deterministic eviction.
