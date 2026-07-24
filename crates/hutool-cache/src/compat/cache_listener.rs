use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, ReentrantMutex, RwLock};

/// Callback invoked after an entry leaves a cache.
pub trait CacheListener<K, V>: Send + Sync {
    /// Receives the removed key and value.
    fn on_remove(&self, key: &K, value: &V);
}

impl<K, V, F> CacheListener<K, V> for F
where
    F: Fn(&K, &V) + Send + Sync,
{
    fn on_remove(&self, key: &K, value: &V) {
        self(key, value);
    }
}

impl<V> Entry<V> {
    fn is_expired_at(&self, now: Instant) -> bool {
        self.ttl
            .is_some_and(|ttl| now.saturating_duration_since(self.last_access) >= ttl)
    }
}

struct Entry<V> {
    value: Arc<V>,
    ttl: Option<Duration>,
    created_at: Instant,
    last_access: Instant,
    created_seq: u64,
    last_access_seq: u64,
    accesses: u64,
}
