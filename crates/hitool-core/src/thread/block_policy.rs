//! 对齐: `cn.hutool.core.thread.BlockPolicy`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/BlockPolicy.java
//!
//! 队列满时在调用线程阻塞执行（近似 put 阻塞）；shutdown 后可选回调。

use std::sync::Arc;

/// 对齐 Java 类: `cn.hutool.core.thread.BlockPolicy`
pub struct BlockPolicy {
    when_shutdown: Option<Arc<dyn Fn(Box<dyn FnOnce() + Send + 'static>) + Send + Sync>>,
}

impl Default for BlockPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockPolicy {
    /// 对齐无参构造。
    pub fn new() -> Self {
        Self {
            when_shutdown: None,
        }
    }

    /// 对齐 `BlockPolicy(Consumer<Runnable> handlerWhenShutdown)`。
    pub fn with_shutdown_handler<F>(handler: F) -> Self
    where
        F: Fn(Box<dyn FnOnce() + Send + 'static>) + Send + Sync + 'static,
    {
        Self {
            when_shutdown: Some(Arc::new(handler)),
        }
    }

    /// 对齐 `rejectedExecution` — 直接在调用线程执行任务。
    pub fn rejected_execution<F>(&self, runnable: F, is_shutdown: bool)
    where
        F: FnOnce() + Send + 'static,
    {
        if is_shutdown {
            if let Some(h) = &self.when_shutdown {
                h(Box::new(runnable));
            }
            return;
        }
        runnable();
    }
}
