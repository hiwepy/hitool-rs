use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, ReentrantMutex, RwLock};

use super::cache_obj::CacheObj;

/// Cache implementation that deliberately stores nothing.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoCache<K, V>(PhantomData<fn(K) -> V>);

impl<K, V> NoCache<K, V> {
    /// Creates a no-op cache.
    pub const fn new() -> Self {
        Self(PhantomData)
    }

    /// No-op insert.
    pub fn put(&self, _key: K, _value: V) {}

    /// No-op insert with timeout.
    pub fn put_with_timeout(&self, _key: K, _value: V, _timeout: Duration) {}

    /// Always misses.
    pub const fn get(&self, _key: &K) -> Option<Arc<V>> {
        None
    }

    /// Produces but does not store a value.
    pub fn get_or_insert_with<F>(&self, _key: K, factory: F) -> Arc<V>
    where
        F: FnOnce() -> V,
    {
        Arc::new(factory())
    }

    /// Always returns false.
    pub const fn contains_key(&self, _key: &K) -> bool {
        false
    }

    /// Returns an empty iterator.
    pub fn values(&self) -> std::iter::Empty<Arc<V>> {
        std::iter::empty()
    }

    /// Returns an empty cache-object iterator.
    pub fn cache_objects(&self) -> std::iter::Empty<CacheObj<K, V>> {
        std::iter::empty()
    }

    /// Removes nothing.
    pub const fn remove(&self, _key: &K) {}

    /// Clears nothing.
    pub const fn clear(&self) {}

    /// Prunes nothing.
    pub const fn prune(&self) -> usize {
        0
    }

    /// A no-op cache is never full.
    pub const fn is_full(&self) -> bool {
        false
    }

    /// Capacity is zero.
    pub const fn capacity(&self) -> usize {
        0
    }

    /// Timeout is absent.
    pub const fn timeout(&self) -> Option<Duration> {
        None
    }

    /// Size is zero.
    pub const fn size(&self) -> usize {
        0
    }

    /// A no-op cache is always empty.
    pub const fn is_empty(&self) -> bool {
        true
    }
}
