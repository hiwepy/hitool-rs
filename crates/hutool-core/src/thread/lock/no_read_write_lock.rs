//! 对齐: `cn.hutool.core.thread.lock.NoReadWriteLock`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/lock/NoReadWriteLock.java

use crate::thread::lock::no_lock::NoLock;

/// 对齐 Java 类: `cn.hutool.core.thread.lock.NoReadWriteLock`
#[derive(Debug, Clone, Copy, Default)]
pub struct NoReadWriteLock;

static INSTANCE: NoReadWriteLock = NoReadWriteLock;

impl NoReadWriteLock {
    /// 单例。
    pub fn instance() -> &'static NoReadWriteLock {
        &INSTANCE
    }

    /// 对齐 `readLock()`。
    pub fn read_lock(&self) -> &'static NoLock {
        NoLock::instance()
    }

    /// 对齐 `writeLock()`。
    pub fn write_lock(&self) -> &'static NoLock {
        NoLock::instance()
    }
}
