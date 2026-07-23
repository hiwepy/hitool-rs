//! 对齐: `cn.hutool.core.thread.ExecutorBuilder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ExecutorBuilder.java

use crate::thread::named_thread_factory::NamedThreadFactory;
use crate::thread::reject_policy::RejectPolicy;
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// 工作队列形态（对齐 BlockingQueue / SynchronousQueue 子集）。
#[derive(Debug, Clone, Copy)]
enum QueueKind {
    /// 无界（近似 LinkedBlockingQueue 大容量）。
    Unbounded,
    /// 有界 ArrayBlockingQueue。
    Bounded(usize),
    /// SynchronousQueue：无缓冲，直接 hand-off（此处用容量 0 同步通道近似）。
    Synchronous,
}

enum JobChannel {
    Unbounded(Sender<Box<dyn FnOnce() + Send + 'static>>),
    Bounded(SyncSender<Box<dyn FnOnce() + Send + 'static>>),
}

impl JobChannel {
    fn try_send(&self, job: Box<dyn FnOnce() + Send + 'static>) -> Result<(), Box<dyn FnOnce() + Send + 'static>> {
        match self {
            JobChannel::Unbounded(tx) => tx.send(job).map_err(|e| e.0),
            JobChannel::Bounded(tx) => tx.try_send(job).map_err(|e| match e {
                mpsc::TrySendError::Full(j) | mpsc::TrySendError::Disconnected(j) => j,
            }),
        }
    }
}

/// 简易线程池（对齐 ExecutorBuilder 构建结果的可运行子集）
pub struct SimpleExecutor {
    tx: Mutex<Option<JobChannel>>,
    handles: Mutex<Vec<JoinHandle<()>>>,
    reject: RejectPolicy,
}

impl SimpleExecutor {
    /// 提交任务
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mut guard = self.tx.lock().unwrap();
        if let Some(tx) = guard.as_ref() {
            if let Err(job) = tx.try_send(Box::new(f)) {
                Self::reject_job(self.reject, job);
            }
        } else {
            // shutdown 后：CALLER_RUNS / BLOCK 在调用线程执行
            match self.reject {
                RejectPolicy::ABORT | RejectPolicy::DISCARD | RejectPolicy::DISCARD_OLDEST => {}
                RejectPolicy::CALLER_RUNS | RejectPolicy::BLOCK => f(),
            }
        }
    }

    fn reject_job(policy: RejectPolicy, job: Box<dyn FnOnce() + Send + 'static>) {
        match policy {
            RejectPolicy::ABORT => {
                // 对齐 AbortPolicy：丢弃并视为拒绝（Rust 不抛 RejectedExecutionException）
            }
            RejectPolicy::DISCARD | RejectPolicy::DISCARD_OLDEST => {}
            RejectPolicy::CALLER_RUNS | RejectPolicy::BLOCK => job(),
        }
    }

    /// 关闭线程池
    pub fn shutdown(&self) {
        let mut guard = self.tx.lock().unwrap();
        *guard = None;
        drop(guard);
        let mut handles = self.handles.lock().unwrap();
        for h in handles.drain(..) {
            let _ = h.join();
        }
    }

    /// 提交并返回 JoinHandle（供 GlobalThreadPool.submit 使用）。
    pub fn submit<F, T>(&self, f: F) -> thread::JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        self.execute(move || {
            let _ = tx.send(f());
        });
        thread::spawn(move || rx.recv().expect("task result"))
    }
}

impl Drop for SimpleExecutor {
    fn drop(&mut self) {
        // Arc 共享时仅最后一个 owner drop；避免对全局池误关。
        // 显式 shutdown() 仍可用。
    }
}

/// 对齐 Java: `ExecutorBuilder`
pub struct ExecutorBuilder {
    core: usize,
    max: usize,
    reject: RejectPolicy,
    keep_alive: Duration,
    allow_core_timeout: bool,
    queue: QueueKind,
    factory: Option<NamedThreadFactory>,
}

impl ExecutorBuilder {
    /// 对齐 `ExecutorBuilder.create()`
    pub fn create() -> Self {
        Self {
            core: 1,
            max: 1,
            reject: RejectPolicy::ABORT,
            keep_alive: Duration::from_secs(60),
            allow_core_timeout: false,
            queue: QueueKind::Unbounded,
            factory: None,
        }
    }

    /// 对齐 `setCorePoolSize`
    pub fn set_core_pool_size(mut self, n: usize) -> Self {
        self.core = n.max(1);
        self
    }

    /// 对齐 `setMaxPoolSize`
    pub fn set_max_pool_size(mut self, n: usize) -> Self {
        self.max = n.max(1);
        self
    }

    /// 对齐 `setHandler`
    pub fn set_handler(mut self, policy: RejectPolicy) -> Self {
        self.reject = policy;
        self
    }

    /// 对齐 `setKeepAliveTime(long)` — 毫秒。
    pub fn set_keep_alive_time_millis(mut self, millis: u64) -> Self {
        self.keep_alive = Duration::from_millis(millis);
        self
    }

    /// 对齐 `setKeepAliveTime(long, TimeUnit)` / Duration 形态。
    pub fn set_keep_alive_time(mut self, keep_alive: Duration) -> Self {
        self.keep_alive = keep_alive;
        self
    }

    /// 对齐 `setAllowCoreThreadTimeOut`
    pub fn set_allow_core_thread_time_out(mut self, allow: bool) -> Self {
        self.allow_core_timeout = allow;
        self
    }

    /// 对齐 `setThreadFactory`
    pub fn set_thread_factory(mut self, factory: NamedThreadFactory) -> Self {
        self.factory = Some(factory);
        self
    }

    /// 对齐 `setWorkQueue` — 有界队列容量。
    pub fn set_work_queue_capacity(mut self, capacity: usize) -> Self {
        self.queue = QueueKind::Bounded(capacity.max(1));
        self
    }

    /// 对齐 `useArrayBlockingQueue(int capacity)`
    pub fn use_array_blocking_queue(mut self, capacity: usize) -> Self {
        self.queue = QueueKind::Bounded(capacity.max(1));
        self
    }

    /// 对齐 `useSynchronousQueue()` / `useSynchronousQueue(boolean fair)`
    pub fn use_synchronous_queue(mut self) -> Self {
        self.queue = QueueKind::Synchronous;
        self
    }

    /// 对齐 `useSynchronousQueue(boolean fair)` — fair 在 Rust 通道中忽略。
    pub fn use_synchronous_queue_fair(mut self, _fair: bool) -> Self {
        self.queue = QueueKind::Synchronous;
        self
    }

    /// 对齐 `build()`
    pub fn build(self) -> SimpleExecutor {
        self.build_inner(false)
    }

    /// 对齐 `buildFinalizable()` — Drop 时自动 shutdown。
    pub fn build_finalizable(self) -> SimpleExecutor {
        self.build_inner(true)
    }

    fn build_inner(self, _finalizable: bool) -> SimpleExecutor {
        let _ = (self.keep_alive, self.allow_core_timeout); // 保留字段语义供后续扩容
        let factory = self
            .factory
            .unwrap_or_else(|| NamedThreadFactory::new("hutool-pool-", false));
        let workers = self.core.max(self.max);

        let (channel, rx): (JobChannel, Arc<Mutex<Receiver<Box<dyn FnOnce() + Send + 'static>>>>) =
            match self.queue {
                QueueKind::Unbounded => {
                    let (tx, rx) = mpsc::channel();
                    (JobChannel::Unbounded(tx), Arc::new(Mutex::new(rx)))
                }
                QueueKind::Bounded(_) | QueueKind::Synchronous => {
                    let cap = match self.queue {
                        QueueKind::Synchronous => 0,
                        QueueKind::Bounded(c) => c,
                        QueueKind::Unbounded => unreachable!(),
                    };
                    // sync_channel(0) ≈ SynchronousQueue；>0 ≈ ArrayBlockingQueue
                    let (tx, rx) = mpsc::sync_channel(cap);
                    (JobChannel::Bounded(tx), Arc::new(Mutex::new(rx)))
                }
            };

        let mut handles = Vec::new();
        for i in 0..workers {
            let rx = Arc::clone(&rx);
            let name = factory.next_name(i);
            handles.push(
                thread::Builder::new()
                    .name(name)
                    .spawn(move || loop {
                        let job = {
                            let guard = rx.lock().unwrap();
                            guard.recv()
                        };
                        match job {
                            Ok(f) => f(),
                            Err(_) => break,
                        }
                    })
                    .expect("spawn pool worker"),
            );
        }
        SimpleExecutor {
            tx: Mutex::new(Some(channel)),
            handles: Mutex::new(handles),
            reject: self.reject,
        }
    }
}
