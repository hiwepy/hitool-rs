//! 对齐: `cn.hutool.core.thread.lock.SegmentLock`
//! 使用 `parking_lot` 实现强引用分段锁 / 信号量 / 读写锁。

use crate::thread::semaphore_runnable::Semaphore;
use parking_lot::{Mutex, RwLock};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

const ALL_SET: usize = !0;

/// 分段锁（对齐 Java `SegmentLock<L>`）
pub struct SegmentLock<L> {
    segments: Vec<L>,
    mask: usize,
}

impl SegmentLock<Arc<Mutex<()>>> {
    /// 对齐 `SegmentLock.lock(stripes)`
    pub fn lock(stripes: usize) -> Self {
        Self::custom(stripes, || Arc::new(Mutex::new(())))
    }

    /// 对齐 `SegmentLock.lazyWeakLock`（Rust 用强引用语义近似）
    pub fn lazy_weak_lock(stripes: usize) -> Self {
        Self::lock(stripes)
    }
}

impl SegmentLock<Arc<Semaphore>> {
    /// 对齐 `SegmentLock.semaphore(stripes, permits)`
    pub fn semaphore(stripes: usize, permits: usize) -> Self {
        Self::custom(stripes, || Arc::new(Semaphore::new(permits)))
    }

    /// 对齐 `SegmentLock.lazyWeakSemaphore` — 强引用近似。
    pub fn lazy_weak_semaphore(stripes: usize, permits: usize) -> Self {
        Self::semaphore(stripes, permits)
    }
}

impl SegmentLock<Arc<RwLock<()>>> {
    /// 对齐 `SegmentLock.readWriteLock(stripes)`
    pub fn read_write_lock(stripes: usize) -> Self {
        Self::custom(stripes, || Arc::new(RwLock::new(())))
    }

    /// 对齐 `SegmentLock.lazyWeakReadWriteLock` — 强引用近似。
    pub fn lazy_weak_read_write_lock(stripes: usize) -> Self {
        Self::read_write_lock(stripes)
    }
}

impl<L> SegmentLock<L> {
    /// 对齐 `SegmentLock.custom`
    pub fn custom<F>(stripes: usize, mut supplier: F) -> Self
    where
        F: FnMut() -> L,
    {
        assert!(stripes > 0, "stripes must be positive");
        let size = ceil_to_power_of_two(stripes);
        let mask = if stripes > usize::MAX / 2 {
            ALL_SET
        } else {
            size - 1
        };
        let mut segments = Vec::with_capacity(size);
        for _ in 0..size {
            segments.push(supplier());
        }
        Self { segments, mask }
    }

    /// 对齐 `size()`
    pub fn size(&self) -> usize {
        self.segments.len()
    }

    fn index_for_hash(&self, hash: u64) -> usize {
        (smear(hash as u32) as usize) & self.mask
    }

    /// 对齐 `get(Object key)`
    pub fn get<K: Hash>(&self, key: &K) -> &L {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = self.index_for_hash(hasher.finish());
        &self.segments[idx]
    }

    /// 对齐 `getAt(int)`
    pub fn get_at(&self, index: usize) -> &L {
        assert!(index < self.segments.len(), "Index out of bounds");
        &self.segments[index]
    }

    /// 对齐 `bulkGet` — 按段索引升序
    pub fn bulk_get<K: Hash>(&self, keys: &[K]) -> Vec<&L> {
        let mut pairs: Vec<(usize, &L)> = keys
            .iter()
            .map(|k| {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                k.hash(&mut hasher);
                let idx = self.index_for_hash(hasher.finish());
                (idx, &self.segments[idx])
            })
            .collect();
        pairs.sort_by_key(|(i, _)| *i);
        pairs.into_iter().map(|(_, l)| l).collect()
    }

    /// 测试辅助：按指针找段索引
    pub fn find_index_by_ptr(&self, target: *const L) -> Option<usize> {
        self.segments
            .iter()
            .position(|s| std::ptr::eq(s as *const L, target))
    }
}

fn ceil_to_power_of_two(x: usize) -> usize {
    if x <= 1 {
        return 1;
    }
    1usize << (usize::BITS - (x - 1).leading_zeros())
}

fn smear(mut hash_code: u32) -> u32 {
    hash_code ^= (hash_code >> 20) ^ (hash_code >> 12);
    hash_code ^ (hash_code >> 7) ^ (hash_code >> 4)
}
