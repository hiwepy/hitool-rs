use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, ReentrantMutex, RwLock};

/// Snapshot of one cached object and its expiration/access metadata.
#[derive(Clone)]
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
