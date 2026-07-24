//! `StampedCache` — 对齐 `cn.hutool.cache.impl.StampedCache`。带版本戳缓存。
use crate::r#impl::abstract_cache::AbstractCache;
/// 带版本戳缓存。与 `AbstractCache` 完全一致，Rust 用 type alias。
pub type StampedCache<K, V> = AbstractCache<K, V>;
