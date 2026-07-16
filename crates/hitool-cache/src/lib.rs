//! Concurrent in-process caching aligned with Hutool's cache module.

#![forbid(unsafe_code)]

use moka::sync::{Cache as MokaCache, CacheBuilder};
use std::{hash::Hash, sync::Arc, time::Duration};

/// Builder for a bounded, concurrent cache.
#[derive(Debug, Clone, Copy)]
pub struct CacheConfig {
    /// Maximum number of entries retained by the cache.
    pub max_capacity: u64,
    /// Optional time-to-live measured from insertion.
    pub time_to_live: Option<Duration>,
    /// Optional time-to-idle measured from the last access.
    pub time_to_idle: Option<Duration>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_capacity: 10_000,
            time_to_live: None,
            time_to_idle: None,
        }
    }
}

/// A cheap-to-clone handle to a bounded, thread-safe cache.
pub struct Cache<K, V> {
    inner: MokaCache<K, Arc<V>>,
}

impl<K, V> Clone for Cache<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    /// Creates a cache from explicit production limits.
    #[must_use]
    pub fn new(config: CacheConfig) -> Self {
        let mut builder: CacheBuilder<K, Arc<V>, MokaCache<K, Arc<V>>> =
            MokaCache::builder().max_capacity(config.max_capacity);
        if let Some(ttl) = config.time_to_live {
            builder = builder.time_to_live(ttl);
        }
        if let Some(tti) = config.time_to_idle {
            builder = builder.time_to_idle(tti);
        }
        Self {
            inner: builder.build(),
        }
    }

    /// Inserts or replaces a cached value.
    pub fn insert(&self, key: K, value: V) {
        self.inner.insert(key, Arc::new(value));
    }

    /// Gets a shared value handle when the key is present.
    #[must_use]
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        self.inner.get(key)
    }

    /// Invalidates one key.
    pub fn invalidate(&self, key: &K) {
        self.inner.invalidate(key);
    }

    /// Invalidates every entry.
    pub fn clear(&self) {
        self.inner.invalidate_all();
    }

    /// Returns the approximate number of entries.
    #[must_use]
    pub fn entry_count(&self) -> u64 {
        self.inner.entry_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_are_shared_and_invalidation_is_explicit() {
        let cache = Cache::new(CacheConfig {
            max_capacity: 10,
            ..CacheConfig::default()
        });
        cache.insert("answer", 42);
        assert_eq!(*cache.get(&"answer").unwrap(), 42);
        cache.invalidate(&"answer");
        assert!(cache.get(&"answer").is_none());
    }
}
