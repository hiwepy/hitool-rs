use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use parking_lot::Mutex;

/// File-cache eviction policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileCachePolicy {
    /// Least frequently used file first.
    Lfu,
    /// Least recently used file first.
    Lru,
}

#[derive(Debug)]
struct FileEntry {
    bytes: Arc<[u8]>,
    last_access: Instant,
    sequence: u64,
    accesses: u64,
}

#[derive(Debug)]
struct FileState {
    entries: HashMap<PathBuf, FileEntry>,
    used_size: usize,
}

/// Thread-safe file cache bounded by total bytes and per-file bytes.
pub struct AbstractFileCache {
    capacity: usize,
    max_file_size: usize,
    timeout: Option<Duration>,
    policy: FileCachePolicy,
    sequence: AtomicU64,
    state: Mutex<FileState>,
}

impl std::fmt::Debug for AbstractFileCache {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AbstractFileCache")
            .field("capacity", &self.capacity)
            .field("max_file_size", &self.max_file_size)
            .field("timeout", &self.timeout)
            .field("policy", &self.policy)
            .field("sequence", &self.sequence.load(Ordering::Relaxed))
            .field("state", &self.state.lock())
            .finish()
    }
}

impl AbstractFileCache {
    /// Creates a bounded file cache. Zero max-file-size means no per-file limit.
    #[must_use]
    pub fn new(
        capacity: usize,
        max_file_size: usize,
        timeout: Option<Duration>,
        policy: FileCachePolicy,
    ) -> Self {
        Self {
            capacity,
            max_file_size,
            timeout: timeout.filter(|value| !value.is_zero()),
            policy,
            sequence: AtomicU64::new(0),
            state: Mutex::new(FileState {
                entries: HashMap::new(),
                used_size: 0,
            }),
        }
    }

    fn next_sequence(&self) -> u64 {
        self.sequence.fetch_add(1, Ordering::Relaxed)
    }

    fn prune_expired(&self, state: &mut FileState, now: Instant) {
        let Some(timeout) = self.timeout else {
            return;
        };
        let expired: Vec<_> = state
            .entries
            .iter()
            .filter(|(_, entry)| now.saturating_duration_since(entry.last_access) >= timeout)
            .map(|(path, _)| path.clone())
            .collect();
        for path in expired {
            let entry = state
                .entries
                .remove(&path)
                .expect("an expired file remains present while the cache lock is held");
            state.used_size = state.used_size.saturating_sub(entry.bytes.len());
        }
    }

    fn victim(&self, state: &FileState) -> Option<PathBuf> {
        state
            .entries
            .iter()
            .min_by_key(|(_, entry)| match self.policy {
                FileCachePolicy::Lfu => (entry.accesses, entry.sequence),
                FileCachePolicy::Lru => (entry.sequence, 0),
            })
            .map(|(path, _)| path.clone())
    }

    /// Reads a path, returning cached bytes when possible.
    ///
    /// # Panics
    ///
    /// Panics only if the internal map changes while its exclusive lock is held.
    pub fn get_file_bytes(&self, path: impl AsRef<Path>) -> io::Result<Arc<[u8]>> {
        let path = path.as_ref().to_path_buf();
        let now = Instant::now();
        let sequence = self.next_sequence();
        {
            let mut state = self.state.lock();
            self.prune_expired(&mut state, now);
            if let Some(entry) = state.entries.get_mut(&path) {
                entry.last_access = now;
                entry.sequence = sequence;
                entry.accesses = entry.accesses.saturating_add(1);
                return Ok(Arc::clone(&entry.bytes));
            }
        }

        let bytes: Arc<[u8]> = fs::read(&path)?.into();
        if (self.max_file_size > 0 && bytes.len() > self.max_file_size)
            || (self.capacity > 0 && bytes.len() > self.capacity)
        {
            return Ok(bytes);
        }

        let mut state = self.state.lock();
        self.prune_expired(&mut state, now);
        while self.capacity > 0
            && state.used_size.saturating_add(bytes.len()) > self.capacity
            && !state.entries.is_empty()
        {
            let victim = self
                .victim(&state)
                .expect("a non-empty file cache has an eviction victim");
            let entry = state
                .entries
                .remove(&victim)
                .expect("the selected file-cache victim is present");
            state.used_size = state.used_size.saturating_sub(entry.bytes.len());
        }
        state.entries.insert(
            path,
            FileEntry {
                bytes: Arc::clone(&bytes),
                last_access: now,
                sequence,
                accesses: 0,
            },
        );
        state.used_size = state.entries.values().map(|entry| entry.bytes.len()).sum();
        Ok(bytes)
    }

    /// Returns byte capacity.
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns bytes currently cached.
    pub fn used_size(&self) -> usize {
        self.state.lock().used_size
    }

    /// Returns maximum cacheable file size; zero means unlimited.
    pub const fn max_file_size(&self) -> usize {
        self.max_file_size
    }

    /// Returns number of cached files.
    pub fn cached_files_count(&self) -> usize {
        self.state.lock().entries.len()
    }

    /// Returns expiration timeout.
    pub const fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// Clears cached files.
    pub fn clear(&self) {
        let mut state = self.state.lock();
        state.entries.clear();
        state.used_size = 0;
    }
}

macro_rules! file_cache {
    ($name:ident, $policy:expr) => {
        #[doc = concat!(stringify!($name), " bounded file cache.")]
        #[derive(Debug)]
        pub struct $name(AbstractFileCache);

        impl $name {
            /// Creates a cache using capacity as the per-file limit.
            #[must_use]
            pub fn new(capacity: usize) -> Self {
                Self(AbstractFileCache::new(capacity, capacity, None, $policy))
            }

            /// Creates a cache with a per-file limit.
            #[must_use]
            pub fn with_max_file_size(capacity: usize, max_file_size: usize) -> Self {
                Self(AbstractFileCache::new(
                    capacity,
                    max_file_size,
                    None,
                    $policy,
                ))
            }

            /// Creates a cache with per-file limit and expiration.
            #[must_use]
            pub fn with_timeout(capacity: usize, max_file_size: usize, timeout: Duration) -> Self {
                Self(AbstractFileCache::new(
                    capacity,
                    max_file_size,
                    Some(timeout),
                    $policy,
                ))
            }
        }

        impl std::ops::Deref for $name {
            type Target = AbstractFileCache;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

file_cache!(LFUFileCache, FileCachePolicy::Lfu);
file_cache!(LRUFileCache, FileCachePolicy::Lru);

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, sync::Arc, thread};
    use tempfile::tempdir;

    fn file(dir: &Path, name: &str, bytes: &[u8]) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, bytes).unwrap();
        path
    }

    #[test]
    fn file_cache_reuses_bytes_reports_limits_and_clears() {
        let dir = tempdir().unwrap();
        let path = file(dir.path(), "a", b"ab");
        let cache = LRUFileCache::with_max_file_size(8, 4);
        let first = cache.get_file_bytes(&path).unwrap();
        let second = cache.get_file_bytes(&path).unwrap();
        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(cache.capacity(), 8);
        assert_eq!(cache.max_file_size(), 4);
        assert_eq!(cache.used_size(), 2);
        assert_eq!(cache.cached_files_count(), 1);
        assert_eq!(cache.timeout(), None);
        assert!(format!("{cache:?}").contains("AbstractFileCache"));
        cache.clear();
        assert_eq!(cache.used_size(), 0);
        assert_eq!(cache.cached_files_count(), 0);
    }

    #[test]
    fn lru_and_lfu_evict_by_byte_capacity() {
        let dir = tempdir().unwrap();
        let a = file(dir.path(), "a", b"aa");
        let b = file(dir.path(), "b", b"bb");
        let c = file(dir.path(), "c", b"cc");

        let lru = LRUFileCache::new(4);
        let old_a = lru.get_file_bytes(&a).unwrap();
        let old_b = lru.get_file_bytes(&b).unwrap();
        assert!(Arc::ptr_eq(&old_a, &lru.get_file_bytes(&a).unwrap()));
        lru.get_file_bytes(&c).unwrap();
        assert!(!Arc::ptr_eq(&old_b, &lru.get_file_bytes(&b).unwrap()));

        let lfu = LFUFileCache::new(4);
        let old_a = lfu.get_file_bytes(&a).unwrap();
        let old_b = lfu.get_file_bytes(&b).unwrap();
        lfu.get_file_bytes(&a).unwrap();
        lfu.get_file_bytes(&a).unwrap();
        lfu.get_file_bytes(&b).unwrap();
        lfu.get_file_bytes(&c).unwrap();
        assert!(Arc::ptr_eq(&old_a, &lfu.get_file_bytes(&a).unwrap()));
        assert!(!Arc::ptr_eq(&old_b, &lfu.get_file_bytes(&b).unwrap()));
    }

    #[test]
    fn oversized_files_bypass_cache_and_missing_files_return_io_error() {
        let dir = tempdir().unwrap();
        let path = file(dir.path(), "large", b"large");
        let per_file = LFUFileCache::with_max_file_size(20, 2);
        let first = per_file.get_file_bytes(&path).unwrap();
        let second = per_file.get_file_bytes(&path).unwrap();
        assert!(!Arc::ptr_eq(&first, &second));
        assert_eq!(per_file.cached_files_count(), 0);

        let capacity = LRUFileCache::with_max_file_size(2, 20);
        assert_eq!(capacity.get_file_bytes(&path).unwrap().as_ref(), b"large");
        assert_eq!(capacity.cached_files_count(), 0);
        let missing = dir.path().join("missing");
        assert!(capacity.get_file_bytes(&missing).is_err());
    }

    #[test]
    fn timeout_expires_cached_file_and_all_constructors_work() {
        let dir = tempdir().unwrap();
        let path = file(dir.path(), "a", b"a");
        let cache = LRUFileCache::with_timeout(4, 4, Duration::from_millis(5));
        let first = cache.get_file_bytes(&path).unwrap();
        thread::sleep(Duration::from_millis(12));
        let second = cache.get_file_bytes(&path).unwrap();
        assert!(!Arc::ptr_eq(&first, &second));
        assert_eq!(cache.timeout(), Some(Duration::from_millis(5)));

        let lfu = LFUFileCache::with_timeout(4, 3, Duration::from_secs(1));
        assert_eq!(lfu.max_file_size(), 3);
    }
}
