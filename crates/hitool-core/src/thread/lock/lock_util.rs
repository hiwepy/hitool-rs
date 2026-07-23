//! 对齐: `cn.hutool.core.thread.lock.LockUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/lock/LockUtil.java

use crate::thread::lock::no_lock::NoLock;
use crate::thread::lock::no_read_write_lock::NoReadWriteLock;
use crate::thread::lock::segment_lock::SegmentLock;
use crate::thread::semaphore_runnable::Semaphore;
use parking_lot::{Mutex, RwLock};
use std::sync::Arc;

/// 对齐 Java 类: `cn.hutool.core.thread.lock.LockUtil`
#[derive(Debug, Clone, Default)]
pub struct LockUtil;

impl LockUtil {
    /// 对齐 `getNoLock()`。
    pub fn get_no_lock() -> &'static NoLock {
        NoLock::instance()
    }

    /// 对齐 `createReadWriteLock(boolean fair)` — parking_lot RwLock（fair 忽略）。
    pub fn create_read_write_lock(_fair: bool) -> Arc<RwLock<()>> {
        Arc::new(RwLock::new(()))
    }

    /// 对齐 `createStampLock()` — 用 RwLock 近似 StampedLock 的读写语义。
    pub fn create_stamp_lock() -> Arc<RwLock<()>> {
        Arc::new(RwLock::new(()))
    }

    /// 对齐 `createSegmentLock(int)`。
    pub fn create_segment_lock(segments: usize) -> SegmentLock<Arc<Mutex<()>>> {
        SegmentLock::lock(segments)
    }

    /// 对齐 `createSegmentReadWriteLock(int)`。
    pub fn create_segment_read_write_lock(segments: usize) -> SegmentLock<Arc<RwLock<()>>> {
        SegmentLock::read_write_lock(segments)
    }

    /// 对齐 `createSegmentSemaphore(int, int)`。
    pub fn create_segment_semaphore(
        segments: usize,
        permits: usize,
    ) -> SegmentLock<Arc<Semaphore>> {
        SegmentLock::semaphore(segments, permits)
    }

    /// 对齐 `createLazySegmentLock(int)` — 强引用近似弱引用懒加载。
    pub fn create_lazy_segment_lock(segments: usize) -> SegmentLock<Arc<Mutex<()>>> {
        SegmentLock::lazy_weak_lock(segments)
    }

    /// 对齐 `getSegmentLock(int, Object)`。
    pub fn get_segment_lock(segments: usize, key: &impl std::hash::Hash) -> Arc<Mutex<()>> {
        Arc::clone(Self::create_segment_lock(segments).get(key))
    }

    /// 对齐 `getSegmentReadLock` / write — 返回分段 RwLock。
    pub fn get_segment_read_write_lock(
        segments: usize,
        key: &impl std::hash::Hash,
    ) -> Arc<RwLock<()>> {
        Arc::clone(Self::create_segment_read_write_lock(segments).get(key))
    }

    /// 对齐 `getSegmentSemaphore`。
    pub fn get_segment_semaphore(
        segments: usize,
        permits: usize,
        key: &impl std::hash::Hash,
    ) -> Arc<Semaphore> {
        Arc::clone(Self::create_segment_semaphore(segments, permits).get(key))
    }

    /// 对齐 `getLazySegmentLock`。
    pub fn get_lazy_segment_lock(segments: usize, key: &impl std::hash::Hash) -> Arc<Mutex<()>> {
        Arc::clone(Self::create_lazy_segment_lock(segments).get(key))
    }

    /// 返回无操作读写锁单例。
    pub fn get_no_read_write_lock() -> &'static NoReadWriteLock {
        NoReadWriteLock::instance()
    }
}
