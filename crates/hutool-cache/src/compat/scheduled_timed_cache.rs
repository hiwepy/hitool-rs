use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, ReentrantMutex, RwLock};

use super::timed_cache::TimedCache;

/// Timed cache bundled with a schedule guard when construction-time scheduling is desired.
pub struct ScheduledTimedCache<K, V> {
    /// Timed cache.
    pub cache: TimedCache<K, V>,
}

impl<K, V> fmt::Debug for ScheduledTimedCache<K, V>
where
    K: Eq + Hash,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ScheduledTimedCache")
            .field("cache", &self.cache)
            .finish()
    }
}
