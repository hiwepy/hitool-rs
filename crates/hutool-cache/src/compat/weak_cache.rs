use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, ReentrantMutex, RwLock};

use super::cache_listener::CacheListener;

/// Explicit weak-value cache using Rust `Arc`/`Weak` ownership.
pub struct WeakCache<K, V> {
    timeout: Option<Duration>,
    entries: Mutex<HashMap<K, WeakEntry<V>>>,
    listener: RwLock<Option<Arc<dyn CacheListener<K, V>>>>,
    /// Per-key locks for Hutool-aligned `get(key, supplier)` double-checked loading.
    key_locks: Mutex<HashMap<K, Arc<Mutex<()>>>>,
}

impl<K, V> WeakCache<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Creates a weak-value cache.
    #[must_use]
    pub fn new(timeout: Option<Duration>) -> Self {
        Self {
            timeout: timeout.filter(|value| !value.is_zero()),
            entries: Mutex::new(HashMap::new()),
            listener: RwLock::new(None),
            key_locks: Mutex::new(HashMap::new()),
        }
    }

    /// Stores a weak reference to a shared value.
    pub fn put(&self, key: K, value: &Arc<V>) {
        self.entries.lock().insert(
            key,
            WeakEntry {
                value: Arc::downgrade(value),
                ttl: self.timeout,
                last_access: Instant::now(),
            },
        );
    }

    /// Gets a live value and refreshes its last access.
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        let now = Instant::now();
        let mut entries = self.entries.lock();
        let expired = entries.get(key).is_some_and(|entry| {
            entry.value.strong_count() == 0
                || entry
                    .ttl
                    .is_some_and(|ttl| now.saturating_duration_since(entry.last_access) >= ttl)
        });
        if expired {
            let value = entries.remove(key).and_then(|entry| entry.value.upgrade());
            drop(entries);
            if let (Some(listener), Some(value)) = (self.listener.read().clone(), value.as_ref()) {
                listener.on_remove(key, value.as_ref());
            }
            return None;
        }
        entries.get_mut(key).and_then(|entry| {
            entry.last_access = now;
            entry.value.upgrade()
        })
    }

    /// Hutool `Cache.get(key, supplier)`: load-or-compute under a per-key lock.
    ///
    /// The returned [`Arc`] must be retained by the caller; dropping every strong
    /// reference lets the weak entry evaporate (Rust ownership vs Java GC).
    pub fn get_or_insert_with<F>(&self, key: K, factory: F) -> Arc<V>
    where
        F: FnOnce() -> V,
        V: Send + Sync + 'static,
    {
        if let Some(value) = self.get(&key) {
            return value;
        }
        let lock = {
            let mut locks = self.key_locks.lock();
            Arc::clone(
                locks
                    .entry(key.clone())
                    .or_insert_with(|| Arc::new(Mutex::new(()))),
            )
        };
        let _guard = lock.lock();
        if let Some(value) = self.get(&key) {
            return value;
        }
        let value = Arc::new(factory());
        self.put(key, &value);
        value
    }

    /// Returns whether a live weak entry exists.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Removes one weak entry and notifies the listener if the value is alive.
    pub fn remove(&self, key: &K) -> Option<Arc<V>> {
        let value = self.entries.lock().remove(key)?.value.upgrade();
        if let (Some(listener), Some(value)) = (self.listener.read().clone(), value.as_ref()) {
            listener.on_remove(key, value.as_ref());
        }
        value
    }

    /// Removes expired or dropped values.
    pub fn prune(&self) -> usize {
        let before = self.entries.lock().len();
        let keys: Vec<_> = self.entries.lock().keys().cloned().collect();
        for key in keys {
            let _ = self.get(&key);
        }
        before.saturating_sub(self.entries.lock().len())
    }

    /// Clears all weak entries.
    pub fn clear(&self) {
        let keys: Vec<_> = self.entries.lock().keys().cloned().collect();
        for key in keys {
            let _ = self.remove(&key);
        }
    }

    /// Installs a removal listener.
    pub fn set_listener<L>(&self, listener: L) -> &Self
    where
        L: CacheListener<K, V> + 'static,
    {
        *self.listener.write() = Some(Arc::new(listener));
        self
    }

    /// Returns the number of tracked weak entries.
    pub fn size(&self) -> usize {
        self.entries.lock().len()
    }

    /// Returns whether no weak entries are tracked.
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Weak caches are unbounded.
    pub const fn capacity(&self) -> usize {
        0
    }

    /// Returns the default timeout.
    pub const fn timeout(&self) -> Option<Duration> {
        self.timeout
    }
}

struct WeakEntry<V> {
    value: Weak<V>,
    ttl: Option<Duration>,
    last_access: Instant,
}
