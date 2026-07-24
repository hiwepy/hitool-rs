use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, ReentrantMutex, RwLock};

use super::abstract_cache::AbstractCache;

/// Hutool's stamped-lock implementation maps to the same safe Rust engine.
pub type StampedCache<K, V> = AbstractCache<K, V>;

macro_rules! bounded_cache {
    ($name:ident, $policy:expr) => {
        #[doc = concat!(stringify!($name), " cache.")]
        #[derive(Clone)]
        pub struct $name<K, V>(AbstractCache<K, V>);

        impl<K, V> fmt::Debug for $name<K, V>
        where
            K: Eq + Hash,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&self.0)
                    .finish()
            }
        }

        impl<K, V> $name<K, V>
        where
            K: Eq + Hash + Clone + Send + Sync + 'static,
            V: Send + Sync + 'static,
        {
            /// Creates a cache without expiration.
            #[must_use]
            pub fn new(capacity: usize) -> Self {
                Self(AbstractCache::new(capacity, None, $policy))
            }

            /// Creates a cache with default time-to-idle.
            #[must_use]
            pub fn with_timeout(capacity: usize, timeout: Duration) -> Self {
                Self(AbstractCache::new(capacity, Some(timeout), $policy))
            }
        }

        impl<K, V> std::ops::Deref for $name<K, V> {
            type Target = AbstractCache<K, V>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
