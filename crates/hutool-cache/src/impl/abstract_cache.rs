//! AbstractCache + CachePolicy + ReentrantCache + StampedCache — 对齐 cn.hutool.cache.impl.AbstractCache。缓存引擎核心实现。

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::{Mutex, ReentrantMutex, RwLock};
use crate::compat::{CacheListener, CacheObj, CachePolicy, Entry};

pub struct AbstractCache<K, V> {
    inner: Arc<CacheInner<K, V>>,
}

impl<K, V> Clone for AbstractCache<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K, V> fmt::Debug for AbstractCache<K, V>
where
    K: Eq + Hash,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AbstractCache")
            .field("capacity", &self.inner.capacity)
            .field("timeout", &self.inner.timeout)
            .field("policy", &self.inner.policy)
            .field("size", &self.inner.state.lock().entries.len())
            .finish()
    }
}

impl<K, V> AbstractCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    /// Creates a cache. Capacity zero means unlimited.
    #[must_use]
    pub fn new(capacity: usize, timeout: Option<Duration>, policy: CachePolicy) -> Self {
        Self {
            inner: Arc::new(CacheInner {
                capacity,
                timeout: timeout.filter(|value| !value.is_zero()),
                policy,
                state: Mutex::new(State {
                    entries: HashMap::new(),
                }),
                listener: RwLock::new(None),
                factory_lock: ReentrantMutex::new(()),
                sequence: AtomicU64::new(0),
                hits: AtomicU64::new(0),
                misses: AtomicU64::new(0),
            }),
        }
    }

    fn next_sequence(&self) -> u64 {
        self.inner.sequence.fetch_add(1, Ordering::Relaxed)
    }

    fn notify(&self, removed: Vec<(K, Arc<V>)>) {
        let listener = self.inner.listener.read().clone();
        if let Some(listener) = listener {
            for (key, value) in removed {
                listener.on_remove(&key, value.as_ref());
            }
        }
    }

    fn victim_key(&self, state: &State<K, V>) -> Option<K> {
        state
            .entries
            .iter()
            .min_by_key(|(_, entry)| match self.inner.policy {
                CachePolicy::Fifo => (entry.created_seq, 0),
                CachePolicy::Lfu => (entry.accesses, entry.created_seq),
                CachePolicy::Lru | CachePolicy::Timed => (entry.last_access_seq, 0),
            })
            .map(|(key, _)| key.clone())
    }

    fn remove_expired_locked(state: &mut State<K, V>, now: Instant) -> Vec<(K, Arc<V>)> {
        let keys: Vec<_> = state
            .entries
            .iter()
            .filter(|(_, entry)| entry.is_expired_at(now))
            .map(|(key, _)| key.clone())
            .collect();
        keys.into_iter()
            .filter_map(|key| state.entries.remove(&key).map(|entry| (key, entry.value)))
            .collect()
    }

    /// Hutool `LFUCache.pruneCache`: subtract the minimum access count from every
    /// entry and remove those that reach zero so new inserts share a fair counter.
    fn prune_lfu_when_full_locked(state: &mut State<K, V>) -> Vec<(K, Arc<V>)> {
        let Some(min_access) = state.entries.values().map(|entry| entry.accesses).min() else {
            return Vec::new();
        };
        let keys: Vec<_> = state.entries.keys().cloned().collect();
        let mut removed = Vec::new();
        for key in keys {
            let Some(entry) = state.entries.get_mut(&key) else {
                continue;
            };
            entry.accesses = entry.accesses.saturating_sub(min_access);
            if entry.accesses == 0 {
                let entry = state
                    .entries
                    .remove(&key)
                    .expect("LFU victim key was present under exclusive lock");
                removed.push((key, entry.value));
            }
        }
        removed
    }

    /// Inserts using the default timeout.
    pub fn put(&self, key: K, value: V) {
        self.put_arc_with_timeout(key, Arc::new(value), self.inner.timeout);
    }

    /// Inserts using an entry-specific time-to-idle. Zero disables expiration.
    pub fn put_with_timeout(&self, key: K, value: V, timeout: Duration) {
        self.put_arc_with_timeout(
            key,
            Arc::new(value),
            Some(timeout).filter(|value| !value.is_zero()),
        );
    }

    /// Inserts a shared value using the default timeout.
    pub fn put_arc(&self, key: K, value: Arc<V>) {
        self.put_arc_with_timeout(key, value, self.inner.timeout);
    }

    fn put_arc_with_timeout(&self, key: K, value: Arc<V>, ttl: Option<Duration>) {
        let now = Instant::now();
        let sequence = self.next_sequence();
        let mut state = self.inner.state.lock();
        let mut removed = Self::remove_expired_locked(&mut state, now);
        if let Some(previous) = state.entries.remove(&key) {
            removed.push((key.clone(), previous.value));
        }
        // Hutool: when full, prune before insert. LFU subtracts min access and
        // removes every entry that reaches zero (not a single victim).
        if self.inner.capacity > 0 && state.entries.len() >= self.inner.capacity {
            match self.inner.policy {
                CachePolicy::Lfu => {
                    removed.extend(Self::prune_lfu_when_full_locked(&mut state));
                }
                _ => {
                    let victim = self
                        .victim_key(&state)
                        .expect("a cache at positive capacity has an eviction victim");
                    let entry = state
                        .entries
                        .remove(&victim)
                        .expect("the selected eviction victim is present");
                    removed.push((victim, entry.value));
                }
            }
        }
        state.entries.insert(
            key,
            Entry {
                value,
                ttl,
                created_at: now,
                last_access: now,
                created_seq: sequence,
                last_access_seq: sequence,
                accesses: 0,
            },
        );
        drop(state);
        self.notify(removed);
    }

    fn get_internal(&self, key: &K, refresh: bool) -> Option<Arc<V>> {
        let now = Instant::now();
        let sequence = self.next_sequence();
        let mut state = self.inner.state.lock();
        if state
            .entries
            .get(key)
            .is_some_and(|entry| entry.is_expired_at(now))
        {
            let entry = state.entries.remove(key).expect("entry existed above");
            self.inner.misses.fetch_add(1, Ordering::Relaxed);
            drop(state);
            self.notify(vec![(key.clone(), entry.value)]);
            return None;
        }
        let result = state.entries.get_mut(key).map(|entry| {
            entry.accesses = entry.accesses.saturating_add(1);
            if refresh {
                entry.last_access = now;
                entry.last_access_seq = sequence;
            }
            Arc::clone(&entry.value)
        });
        if result.is_some() {
            self.inner.hits.fetch_add(1, Ordering::Relaxed);
        } else {
            self.inner.misses.fetch_add(1, Ordering::Relaxed);
        }
        result
    }

    /// Gets and refreshes an entry's last-access time.
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        self.get_internal(key, true)
    }

    /// Gets without refreshing expiration.
    pub fn get_without_refresh(&self, key: &K) -> Option<Arc<V>> {
        self.get_internal(key, false)
    }

    /// Gets or computes and stores a value with the default timeout.
    pub fn get_or_insert_with<F>(&self, key: K, factory: F) -> Arc<V>
    where
        F: FnOnce() -> V,
    {
        self.get_or_insert_with_timeout(key, self.inner.timeout, factory)
    }

    /// Gets or computes and stores a value with an explicit optional timeout.
    pub fn get_or_insert_with_timeout<F>(
        &self,
        key: K,
        timeout: Option<Duration>,
        factory: F,
    ) -> Arc<V>
    where
        F: FnOnce() -> V,
    {
        let _factory_guard = self.inner.factory_lock.lock();
        if let Some(value) = self.get(&key) {
            return value;
        }
        let value = Arc::new(factory());
        self.put_arc_with_timeout(key, Arc::clone(&value), timeout);
        value
    }

    /// Returns whether a live entry exists without changing hit/miss counters.
    ///
    /// # Panics
    ///
    /// Panics only if the internal map changes while its exclusive lock is held.
    pub fn contains_key(&self, key: &K) -> bool {
        let now = Instant::now();
        let mut state = self.inner.state.lock();
        if state
            .entries
            .get(key)
            .is_some_and(|entry| entry.is_expired_at(now))
        {
            let entry = state
                .entries
                .remove(key)
                .expect("the expired entry was observed while holding the cache lock");
            drop(state);
            self.notify(vec![(key.clone(), entry.value)]);
            false
        } else {
            state.entries.contains_key(key)
        }
    }

    /// Removes one entry and returns its value.
    pub fn remove(&self, key: &K) -> Option<Arc<V>> {
        let removed = self.inner.state.lock().entries.remove(key);
        removed.map(|entry| {
            let value = Arc::clone(&entry.value);
            self.notify(vec![(key.clone(), entry.value)]);
            value
        })
    }

    /// Removes expired entries and returns the number removed.
    pub fn prune(&self) -> usize {
        let mut state = self.inner.state.lock();
        let removed = Self::remove_expired_locked(&mut state, Instant::now());
        let count = removed.len();
        drop(state);
        self.notify(removed);
        count
    }

    /// Clears all entries.
    pub fn clear(&self) {
        let mut state = self.inner.state.lock();
        let removed = state
            .entries
            .drain()
            .map(|(key, entry)| (key, entry.value))
            .collect();
        drop(state);
        self.notify(removed);
    }

    /// Installs or replaces a removal listener.
    pub fn set_listener<L>(&self, listener: L) -> &Self
    where
        L: CacheListener<K, V> + 'static,
    {
        *self.inner.listener.write() = Some(Arc::new(listener));
        self
    }

    /// Removes the listener.
    pub fn clear_listener(&self) {
        *self.inner.listener.write() = None;
    }

    /// Returns the configured capacity; zero means unlimited.
    pub fn capacity(&self) -> usize {
        self.inner.capacity
    }

    /// Returns the default timeout.
    pub fn timeout(&self) -> Option<Duration> {
        self.inner.timeout
    }

    /// Returns whether a bounded cache has reached capacity.
    pub fn is_full(&self) -> bool {
        self.inner.capacity > 0 && self.size() >= self.inner.capacity
    }

    /// Returns the current entry count.
    pub fn size(&self) -> usize {
        self.inner.state.lock().entries.len()
    }

    /// Returns whether there are no entries.
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Returns the hit count.
    pub fn hit_count(&self) -> u64 {
        self.inner.hits.load(Ordering::Relaxed)
    }

    /// Returns the miss count.
    pub fn miss_count(&self) -> u64 {
        self.inner.misses.load(Ordering::Relaxed)
    }

    /// Returns a snapshot of keys.
    pub fn key_set(&self) -> HashSet<K> {
        self.inner.state.lock().entries.keys().cloned().collect()
    }

    /// Returns shared value snapshots.
    pub fn values(&self) -> Vec<Arc<V>> {
        self.inner
            .state
            .lock()
            .entries
            .values()
            .map(|entry| Arc::clone(&entry.value))
            .collect()
    }

    /// Returns cache-object snapshots.
    pub fn cache_objects(&self) -> Vec<CacheObj<K, V>> {
        self.inner
            .state
            .lock()
            .entries
            .iter()
            .map(|(key, entry)| CacheObj {
                key: key.clone(),
                value: Arc::clone(&entry.value),
                ttl: entry.ttl,
                created_at: entry.created_at,
                last_access: entry.last_access,
            })
            .collect()
    }
}
