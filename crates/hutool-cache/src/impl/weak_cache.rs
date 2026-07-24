//! `WeakCache`，对齐 `cn.hutool.cache.impl.WeakCache`。
//!
//! 弱引用缓存——value 被 `Weak` 引用持有，无强引用时自动被回收。
//! Java 版用 `WeakReference<V>` + GC，Rust 用 `Arc::downgrade` + 引用计数。
//!
//! Java 方法对照：
//! - `WeakCache(long timeout)` → `WeakCache::new(Option<Duration>)`
//! - `put(K, V)` → `put(&self, key, &Arc<V>)`
//! - `get(K)` → `get(&self, &K) -> Option<Arc<V>>`
//! - `get(K, Func0<V>)` → `get_or_insert_with(&self, K, F)`
//! - `remove(K)` → `remove(&self, &K)`
//! - `prune()` → `prune(&self) -> usize`
//! - `clear()` → `clear(&self)`
//! - `size()` → `size(&self) -> usize`
//! - `isFull()` / `capacity()` → `capacity(&self) -> usize`

use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, RwLock};

use crate::compat::CacheListener;

/// 弱引用缓存条目（内部类型，对齐 hutool 的 `CacheObj` 的弱引用版本）。
struct WeakEntry<V> {
    value: Weak<V>,
    ttl: Option<Duration>,
    last_access: Instant,
}

/// 弱引用缓存，对齐 `cn.hutool.cache.impl.WeakCache`。
pub struct WeakCache<K, V> {
    timeout: Option<Duration>,
    entries: Mutex<HashMap<K, WeakEntry<V>>>,
    listener: RwLock<Option<Arc<dyn CacheListener<K, V>>>>,
    key_locks: Mutex<HashMap<K, Arc<Mutex<()>>>>,
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for WeakCache<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WeakCache")
            .field("timeout", &self.timeout)
            .field("size", &self.entries.lock().len())
            .finish()
    }
}

impl<K, V> WeakCache<K, V>
where
    K: Eq + Hash + Clone,
{
    #[must_use]
    pub fn new(timeout: Option<Duration>) -> Self {
        Self {
            timeout: timeout.filter(|value| !value.is_zero()),
            entries: Mutex::new(HashMap::new()),
            listener: RwLock::new(None),
            key_locks: Mutex::new(HashMap::new()),
        }
    }

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

    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        let now = Instant::now();
        let mut entries = self.entries.lock();
        let expired = entries.get(key).is_some_and(|entry| {
            entry.value.strong_count() == 0
                || entry.ttl.is_some_and(|ttl| now.saturating_duration_since(entry.last_access) >= ttl)
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
            Arc::clone(locks.entry(key.clone()).or_insert_with(|| Arc::new(Mutex::new(()))))
        };
        let _guard = lock.lock();
        if let Some(value) = self.get(&key) {
            return value;
        }
        let value = Arc::new(factory());
        self.put(key, &value);
        value
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&self, key: &K) -> Option<Arc<V>> {
        let value = self.entries.lock().remove(key)?.value.upgrade();
        if let (Some(listener), Some(value)) = (self.listener.read().clone(), value.as_ref()) {
            listener.on_remove(key, value.as_ref());
        }
        value
    }

    pub fn prune(&self) -> usize {
        let before = self.entries.lock().len();
        let keys: Vec<_> = self.entries.lock().keys().cloned().collect();
        for key in keys {
            let _ = self.get(&key);
        }
        before.saturating_sub(self.entries.lock().len())
    }

    pub fn clear(&self) {
        let keys: Vec<_> = self.entries.lock().keys().cloned().collect();
        for key in keys {
            let _ = self.remove(&key);
        }
    }

    pub fn set_listener<L>(&self, listener: L) -> &Self
    where
        L: CacheListener<K, V> + 'static,
    {
        *self.listener.write() = Some(Arc::new(listener));
        self
    }

    pub fn size(&self) -> usize {
        self.entries.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub const fn capacity(&self) -> usize {
        0
    }

    pub const fn timeout(&self) -> Option<Duration> {
        self.timeout
    }
}
