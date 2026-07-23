//! 对齐: `cn.hutool.core.thread.RecyclableBatchThreadPoolExecutor`

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

/// 对齐 Java: `RecyclableBatchThreadPoolExecutor`（可运行简化版）
pub struct RecyclableBatchThreadPoolExecutor {
    pool_size: usize,
    executed: AtomicUsize,
}

impl RecyclableBatchThreadPoolExecutor {
    /// 创建固定大小批处理池
    pub fn new(pool_size: usize) -> Self {
        Self {
            pool_size: pool_size.max(1),
            executed: AtomicUsize::new(0),
        }
    }

    /// 批量执行任务并等待完成
    pub fn process<T, R, F>(&self, tasks: Vec<T>, f: F) -> Vec<R>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: Fn(T) -> R + Send + Sync + 'static,
    {
        let f = Arc::new(f);
        let results = Arc::new(Mutex::new(Vec::with_capacity(tasks.len())));
        let mut handles = Vec::new();
        for (i, task) in tasks.into_iter().enumerate() {
            let f = Arc::clone(&f);
            let results = Arc::clone(&results);
            let executed = &self.executed;
            // 简单串行分批：限制并发为 pool_size
            handles.push(thread::spawn(move || {
                let r = f(task);
                results.lock().unwrap().push((i, r));
            }));
            if handles.len() >= self.pool_size {
                if let Some(h) = handles.pop() {
                    let _ = h.join();
                    executed.fetch_add(1, Ordering::SeqCst);
                }
            }
        }
        for h in handles {
            let _ = h.join();
            self.executed.fetch_add(1, Ordering::SeqCst);
        }
        let mut out = results.lock().unwrap();
        out.sort_by_key(|(i, _)| *i);
        out.drain(..).map(|(_, r)| r).collect()
    }

    /// 已执行批次数（近似）
    pub fn executed(&self) -> usize {
        self.executed.load(Ordering::SeqCst)
    }
}
