//! `ReentrantCache` — 对齐 `cn.hutool.cache.impl.ReentrantCache`。可重入缓存。
use crate::r#impl::abstract_cache::AbstractCache;
/// 可重入缓存。与 `AbstractCache` 完全一致，Rust 用 type alias。
pub type ReentrantCache<K, V> = AbstractCache<K, V>;
