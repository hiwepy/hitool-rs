//! `NoCache`，对齐 `cn.hutool.cache.impl.NoCache`。无缓存实现（测试/禁用缓存场景）。
//!
//! Java 方法对照：
//! - `new NoCache()` → `NoCache::new()`
//! - `put(K, V)` → `put(&self, K, V)` — 空操作
//! - `get(K)` → `get(&self, &K) -> None` — 永远缓存未命中
//! - `get(K, Func0<V>)` → `get_or_insert_with(&self, K, F) -> Arc<V>` — 执行但不缓存

use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use crate::compat::CacheObj;

/// 无缓存实现，对齐 `cn.hutool.cache.impl.NoCache`。所有操作都是空操作。
pub struct NoCache<K, V>(PhantomData<fn(K) -> V>);

impl<K, V> fmt::Debug for NoCache<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NoCache").finish()
    }
}

impl<K, V> Default for NoCache<K, V> {
    fn default() -> Self { Self::new() }
}

impl<K, V> NoCache<K, V> {
    pub const fn new() -> Self { Self(PhantomData) }
    pub fn put(&self, _key: K, _value: V) {}
    pub fn put_with_timeout(&self, _key: K, _value: V, _timeout: Duration) {}
    pub const fn get(&self, _key: &K) -> Option<Arc<V>> { None }
    pub fn get_or_insert_with<F>(&self, _key: K, factory: F) -> Arc<V>
    where F: FnOnce() -> V { Arc::new(factory()) }
    pub const fn contains_key(&self, _key: &K) -> bool { false }
    pub fn values(&self) -> std::iter::Empty<Arc<V>> { std::iter::empty() }
    pub fn cache_objects(&self) -> std::iter::Empty<CacheObj<K, V>> { std::iter::empty() }
    pub const fn remove(&self, _key: &K) {}
    pub const fn clear(&self) {}
    pub const fn prune(&self) -> usize { 0 }
    pub const fn is_full(&self) -> bool { false }
    pub const fn capacity(&self) -> usize { 0 }
    pub const fn timeout(&self) -> Option<Duration> { None }
    pub const fn size(&self) -> usize { 0 }
    pub const fn is_empty(&self) -> bool { true }
}
