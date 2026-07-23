//! 对齐: `cn.hutool.core.thread.SyncFinisher`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/SyncFinisher.java

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Barrier, Mutex};
use std::thread::{self, JoinHandle};

type ExHandler = Arc<dyn Fn(&thread::Thread, Box<dyn std::any::Any + Send>) + Send + Sync>;

/// 对齐 Java: `SyncFinisher`
pub struct SyncFinisher {
    thread_size: usize,
    workers: Mutex<Vec<Box<dyn FnOnce() + Send + 'static>>>,
    exception_handler: Mutex<Option<ExHandler>>,
    started: AtomicBool,
    begin_at_same_time: AtomicBool,
    finished: AtomicU64,
}

impl SyncFinisher {
    /// 对齐 `new SyncFinisher(threadSize)`
    pub fn new(thread_size: usize) -> Self {
        Self {
            thread_size: thread_size.max(1),
            workers: Mutex::new(Vec::new()),
            exception_handler: Mutex::new(None),
            started: AtomicBool::new(false),
            begin_at_same_time: AtomicBool::new(false),
            finished: AtomicU64::new(0),
        }
    }

    /// 对齐 `addWorker(Runnable)`
    pub fn add_worker<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.workers.lock().unwrap().push(Box::new(f));
    }

    /// 对齐 `addRepeatWorker(Runnable)` — 按 threadSize 重复添加同一逻辑。
    pub fn add_repeat_worker<F>(&self, f: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        let f = Arc::new(f);
        for _ in 0..self.thread_size {
            let f = Arc::clone(&f);
            self.add_worker(move || f());
        }
    }

    /// 对齐 `clearWorker()`。
    pub fn clear_worker(&self) {
        self.workers.lock().unwrap().clear();
    }

    /// 对齐 `setBeginAtSameTime`。
    pub fn set_begin_at_same_time(&self, yes: bool) -> &Self {
        self.begin_at_same_time.store(yes, Ordering::SeqCst);
        self
    }

    /// 对齐 `setExceptionHandler`
    pub fn set_exception_handler<H>(&self, handler: H) -> &Self
    where
        H: Fn(&thread::Thread, Box<dyn std::any::Any + Send>) + Send + Sync + 'static,
    {
        *self.exception_handler.lock().unwrap() = Some(Arc::new(handler));
        self
    }

    /// 对齐 `setExecutorService` — Rust 侧忽略外部 Executor，自管线程。
    pub fn set_executor_service(&self, _ignored: ()) -> &Self {
        self
    }

    /// 对齐 `count()` — 已完成 worker 数。
    pub fn count(&self) -> u64 {
        self.finished.load(Ordering::SeqCst)
    }

    /// 对齐 `start()`
    pub fn start(&self) {
        self.start_sync(true);
    }

    /// 对齐 `start(boolean sync)`
    pub fn start_sync(&self, sync: bool) {
        if self.started.swap(true, Ordering::SeqCst) {
            return;
        }
        let workers = std::mem::take(&mut *self.workers.lock().unwrap());
        let handler = self.exception_handler.lock().unwrap().clone();
        let same_time = self.begin_at_same_time.load(Ordering::SeqCst);
        let barrier = if same_time {
            Some(Arc::new(Barrier::new(workers.len().max(1))))
        } else {
            None
        };
        let finished = &self.finished;
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        for worker in workers {
            let handler = handler.clone();
            let barrier = barrier.clone();
            handles.push(thread::spawn(move || {
                if let Some(b) = barrier {
                    b.wait();
                }
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(worker));
                if let Err(payload) = result {
                    if let Some(h) = handler.as_ref() {
                        h(&thread::current(), payload);
                    }
                }
            }));
            if handles.len() >= self.thread_size {
                if let Some(h) = handles.pop() {
                    let _ = h.join();
                    finished.fetch_add(1, Ordering::SeqCst);
                }
            }
        }
        if sync {
            for h in handles {
                let _ = h.join();
                finished.fetch_add(1, Ordering::SeqCst);
            }
        } else {
            // 异步：detach 剩余句柄
            for h in handles {
                std::mem::forget(h);
            }
        }
    }

    /// 对齐 `stop()` / `stopNow()`。
    pub fn stop(&self) {
        self.clear_worker();
    }

    /// 对齐 `stopNow()`。
    pub fn stop_now(&self) {
        self.stop();
    }

    /// 对齐 `close()`
    pub fn close(&self) {
        self.stop();
    }
}
