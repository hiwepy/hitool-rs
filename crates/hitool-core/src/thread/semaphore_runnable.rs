//! 对齐: `cn.hutool.core.thread.SemaphoreRunnable`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/SemaphoreRunnable.java
//!
//! 带有信号量控制的任务包装: 在执行前 `acquire`, 结束后(含 panic) `release`,
//! 从而限制可同时执行该逻辑的线程数。
//!
//! # 设计选择 (sync, 非 Tokio)
//!
//! `hitool-core` 刻意不引入 async runtime (`lib.rs` / Cargo.toml 均无 tokio)。
//! Java `java.util.concurrent.Semaphore` 本身是阻塞式计数信号量, 与 Hutool
//! 线程模型一致。因此本模块用 `parking_lot::{Mutex, Condvar}` 实现同步
//! [`Semaphore`], 而非 `tokio::sync::Semaphore`。

use std::sync::Arc;

use parking_lot::{Condvar, Mutex};

/// 同步计数信号量, 对齐 Java `java.util.concurrent.Semaphore` 的核心语义。
///
/// 供 [`SemaphoreRunnable`] 与后续 `SegmentLock` / `LockUtil` 分段信号量复用。
/// 多个线程必须共享同一 [`Arc<Semaphore>`](Semaphore)。
#[derive(Debug)]
pub struct Semaphore {
    /// 当前可用许可数。
    permits: Mutex<usize>,
    /// 许可变为可用时唤醒等待者。
    available: Condvar,
}

impl Semaphore {
    /// 创建具有 `permits` 个初始许可的信号量。
    ///
    /// 对齐 Java: `new Semaphore(int permits)`。
    #[must_use]
    pub fn new(permits: usize) -> Self {
        Self {
            permits: Mutex::new(permits),
            available: Condvar::new(),
        }
    }

    /// 获取一个许可; 无可用许可时阻塞当前线程。
    ///
    /// 对齐 Java: `Semaphore#acquire()` (Rust 无线程中断, 故不抛出
    /// `InterruptedException`; 阻塞直至获得许可)。
    pub fn acquire(&self) {
        let mut permits = self.permits.lock();
        while *permits == 0 {
            self.available.wait(&mut permits);
        }
        *permits -= 1;
    }

    /// 尝试获取一个许可; 成功返回 `true`, 否则立即返回 `false`。
    ///
    /// 对齐 Java: `Semaphore#tryAcquire()`。
    #[must_use]
    pub fn try_acquire(&self) -> bool {
        let mut permits = self.permits.lock();
        if *permits == 0 {
            return false;
        }
        *permits -= 1;
        true
    }

    /// 释放一个许可, 可能唤醒一个等待中的获取者。
    ///
    /// 对齐 Java: `Semaphore#release()`。许可数可超过初始值(与 JDK 一致)。
    pub fn release(&self) {
        let mut permits = self.permits.lock();
        *permits += 1;
        self.available.notify_one();
    }

    /// 返回当前可用许可数。
    ///
    /// 对齐 Java: `Semaphore#availablePermits()`。
    #[must_use]
    pub fn available_permits(&self) -> usize {
        *self.permits.lock()
    }
}

/// RAII 许可守卫: 在 Drop 时释放许可, 等价于 Java `try/finally` 中的 `release`。
struct PermitGuard<'a> {
    semaphore: &'a Semaphore,
}

impl Drop for PermitGuard<'_> {
    fn drop(&mut self) {
        self.semaphore.release();
    }
}

/// 对齐 Java 类: `cn.hutool.core.thread.SemaphoreRunnable`
///
/// 包装一个可执行任务: 若信号量非空, 则先获取许可再执行, 结束后释放许可。
/// 若信号量为 `None`(对齐 Java `null`), `run` 为空操作且**不**执行任务
/// (与 Hutool 源码一致)。
pub struct SemaphoreRunnable<F> {
    /// 实际执行的逻辑 — 对齐 Java 字段 `runnable`。
    runnable: F,
    /// 信号量 — 对齐 Java 字段 `semaphore`; `None` 对应 Java `null`。
    semaphore: Option<Arc<Semaphore>>,
}

impl<F> SemaphoreRunnable<F> {
    /// 构造带可选信号量的包装任务。
    ///
    /// 对齐 Java: `SemaphoreRunnable(Runnable runnable, Semaphore semaphore)`。
    ///
    /// # 参数
    /// - `runnable`: 实际执行逻辑
    /// - `semaphore`: 多个线程必须共享同一信号量; 传 `None` 对齐 Java `null`
    #[must_use]
    pub fn new(runnable: F, semaphore: Option<Arc<Semaphore>>) -> Self {
        Self {
            runnable,
            semaphore,
        }
    }

    /// 使用非空共享信号量构造包装任务(常见路径)。
    ///
    /// # 参数
    /// - `runnable`: 实际执行逻辑
    /// - `semaphore`: 共享信号量(`Arc`), 对齐 Java「多个线程必须共享同一信号量」
    #[must_use]
    pub fn with_semaphore(runnable: F, semaphore: Arc<Semaphore>) -> Self {
        Self::new(runnable, Some(semaphore))
    }

    /// 获得信号量引用。
    ///
    /// 对齐 Java: `getSemaphore()`。
    #[must_use]
    pub fn semaphore(&self) -> Option<&Arc<Semaphore>> {
        self.semaphore.as_ref()
    }

    /// 获得信号量引用(Java 命名别名)。
    ///
    /// 对齐 Java: `getSemaphore()`。
    #[must_use]
    pub fn get_semaphore(&self) -> Option<&Arc<Semaphore>> {
        self.semaphore()
    }
}

impl<F> SemaphoreRunnable<F>
where
    F: FnMut(),
{
    /// 在信号量控制下执行任务。
    ///
    /// 对齐 Java: `run()`。
    ///
    /// 逻辑:
    /// 1. `semaphore == None` → 直接返回(不执行任务, 对齐 Hutool)
    /// 2. `acquire` → 执行 `runnable` → `release`(经 RAII, panic 时也会释放)
    pub fn run(&mut self) {
        let Some(semaphore) = self.semaphore.as_ref() else {
            return;
        };

        // 获取许可; 无可用许可时阻塞
        semaphore.acquire();
        // RAII 保证 runnable panic 时仍释放许可(对齐 Java finally)
        let _guard = PermitGuard {
            semaphore: semaphore.as_ref(),
        };
        (self.runnable)();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;

    /// 验证 acquire / release 改变 available_permits。
    #[test]
    fn acquire_release_updates_permits() {
        let sem = Semaphore::new(2);
        assert_eq!(sem.available_permits(), 2);

        sem.acquire();
        assert_eq!(sem.available_permits(), 1);

        sem.acquire();
        assert_eq!(sem.available_permits(), 0);
        assert!(!sem.try_acquire());

        sem.release();
        assert_eq!(sem.available_permits(), 1);
        assert!(sem.try_acquire());
        assert_eq!(sem.available_permits(), 0);
    }

    /// 验证 SemaphoreRunnable 在运行前后正确占用/释放许可。
    #[test]
    fn runnable_acquires_and_releases() {
        let sem = Arc::new(Semaphore::new(1));
        let ran = Arc::new(AtomicUsize::new(0));
        let ran_clone = Arc::clone(&ran);
        let sem_inside = Arc::clone(&sem);

        let mut task = SemaphoreRunnable::with_semaphore(
            move || {
                assert_eq!(sem_inside.available_permits(), 0);
                ran_clone.fetch_add(1, Ordering::SeqCst);
            },
            Arc::clone(&sem),
        );

        assert_eq!(sem.available_permits(), 1);
        task.run();
        assert_eq!(sem.available_permits(), 1);
        assert_eq!(ran.load(Ordering::SeqCst), 1);
    }

    /// 验证信号量为 None 时 run 为空操作(对齐 Java null semaphore)。
    #[test]
    fn null_semaphore_is_noop() {
        let ran = Arc::new(AtomicUsize::new(0));
        let ran_clone = Arc::clone(&ran);
        let mut task = SemaphoreRunnable::new(move || {
            ran_clone.fetch_add(1, Ordering::SeqCst);
        }, None);

        task.run();
        assert_eq!(ran.load(Ordering::SeqCst), 0);
        assert!(task.get_semaphore().is_none());
    }

    /// 验证 permits=2 时最多两个线程同时进入临界区。
    #[test]
    fn concurrency_limit_enforced() {
        let permits = 2usize;
        let sem = Arc::new(Semaphore::new(permits));
        let in_critical = Arc::new(AtomicUsize::new(0));
        let max_in_critical = Arc::new(AtomicUsize::new(0));
        let completed = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::new();
        for _ in 0..8 {
            let sem = Arc::clone(&sem);
            let in_critical = Arc::clone(&in_critical);
            let max_in_critical = Arc::clone(&max_in_critical);
            let completed = Arc::clone(&completed);

            handles.push(thread::spawn(move || {
                let mut task = SemaphoreRunnable::with_semaphore(
                    move || {
                        let now = in_critical.fetch_add(1, Ordering::SeqCst) + 1;
                        max_in_critical.fetch_max(now, Ordering::SeqCst);
                        thread::sleep(Duration::from_millis(30));
                        in_critical.fetch_sub(1, Ordering::SeqCst);
                        completed.fetch_add(1, Ordering::SeqCst);
                    },
                    sem,
                );
                task.run();
            }));
        }

        for h in handles {
            h.join().expect("worker thread panicked");
        }

        assert_eq!(completed.load(Ordering::SeqCst), 8);
        assert!(
            max_in_critical.load(Ordering::SeqCst) <= permits,
            "observed concurrent entries exceeded semaphore permits"
        );
        assert_eq!(sem.available_permits(), permits);
    }

    /// 验证 runnable panic 后许可仍被释放。
    #[test]
    fn release_on_panic() {
        let sem = Arc::new(Semaphore::new(1));
        let sem_for_task = Arc::clone(&sem);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut task = SemaphoreRunnable::with_semaphore(
                || panic!("boom"),
                sem_for_task,
            );
            task.run();
        }));

        assert!(result.is_err());
        assert_eq!(sem.available_permits(), 1);
    }
}
