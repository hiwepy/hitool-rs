//! 对齐: `cn.hutool.core.thread.ConcurrencyTester`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ConcurrencyTester.java

use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Instant;

/// 对齐 Java: `ConcurrencyTester`
pub struct ConcurrencyTester {
    thread_size: usize,
    interval_ms: u128,
}

impl ConcurrencyTester {
    /// 对齐 `new ConcurrencyTester(threadSize)`
    pub fn new(thread_size: usize) -> Self {
        Self {
            thread_size: thread_size.max(1),
            interval_ms: 0,
        }
    }

    /// 对齐 `test(Runnable)` — 多线程同时起跑并等待全部结束
    pub fn test<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        let f = Arc::new(f);
        let barrier = Arc::new(Barrier::new(self.thread_size));
        let start = Instant::now();
        let mut handles = Vec::new();
        for _ in 0..self.thread_size {
            let f = Arc::clone(&f);
            let barrier = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                barrier.wait();
                f();
            }));
        }
        for h in handles {
            let _ = h.join();
        }
        self.interval_ms = start.elapsed().as_millis();
        self
    }

    /// 对齐 `getInterval()`
    pub fn get_interval(&self) -> u128 {
        self.interval_ms
    }

    /// 对齐 `reset()`
    pub fn reset(&mut self) -> &mut Self {
        self.interval_ms = 0;
        self
    }

    /// 对齐 `close()` — AutoCloseable 空操作。
    pub fn close(&mut self) {}
}
