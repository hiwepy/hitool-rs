//! 对齐: `cn.hutool.core.thread.lock.NoLock`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/lock/NoLock.java

use std::time::Duration;

/// 对齐 Java 类: `cn.hutool.core.thread.lock.NoLock` — 空操作锁。
#[derive(Debug, Clone, Copy, Default)]
pub struct NoLock;

static INSTANCE: NoLock = NoLock;

impl NoLock {
    /// 单例，对齐 `NoLock.INSTANCE`。
    pub fn instance() -> &'static NoLock {
        &INSTANCE
    }

    /// 对齐 `lock()`。
    pub fn lock(&self) {}

    /// 对齐 `lockInterruptibly()`。
    pub fn lock_interruptibly(&self) {}

    /// 对齐 `tryLock()`。
    pub fn try_lock(&self) -> bool {
        true
    }

    /// 对齐 `tryLock(long, TimeUnit)`。
    pub fn try_lock_timeout(&self, _time: Duration) -> bool {
        true
    }

    /// 对齐 `unlock()`。
    pub fn unlock(&self) {}
}
