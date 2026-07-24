use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, ReentrantMutex, RwLock};

/// Deterministic eviction policy used by Hutool-aligned caches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CachePolicy {
    /// First inserted, first removed.
    Fifo,
    /// Least frequently accessed, then oldest, first removed.
    Lfu,
    /// Least recently accessed first.
    Lru,
    /// Unbounded entries removed only by expiration.
    Timed,
}
