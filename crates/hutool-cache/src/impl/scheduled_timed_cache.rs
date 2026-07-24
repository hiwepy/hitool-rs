//! `ScheduledTimedCache` — 对齐 hutool 的调度定时缓存。带 schedule guard 的 TimedCache。
use std::fmt;
use std::hash::Hash;
use crate::compat::TimedCache;
#[derive(Debug, Clone, Copy, Default)]
pub struct ScheduledTimedCache<K, V> { pub cache: TimedCache<K, V> }
impl<K: Eq + Hash + fmt::Debug, V: fmt::Debug> fmt::Debug for ScheduledTimedCache<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("ScheduledTimedCache").field("cache", &self.cache).finish() }
}
