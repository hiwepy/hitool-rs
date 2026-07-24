//! Hutool-aligned collection types with Rust-native ownership and concurrency.

use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt,
    hash::Hash,
    sync::{
        Arc,
        atomic::{AtomicU64, AtomicUsize, Ordering as AtomicOrdering},
    },
};

use indexmap::IndexMap;
use parking_lot::RwLock;

use crate::{CoreError, Result};

mod bounded_priority_queue;
mod concurrent_hash_set;
mod unique_key_set;

pub use bounded_priority_queue::BoundedPriorityQueue;
pub use concurrent_hash_set::ConcurrentHashSet;
pub use unique_key_set::UniqueKeySet;
pub use bounded_priority_queue::ring_next_index;
pub use bounded_priority_queue::ring_next_u64;
pub use bounded_priority_queue::ring_next_for_len;
