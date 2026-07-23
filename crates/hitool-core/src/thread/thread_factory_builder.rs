//! 对齐: `cn.hutool.core.thread.ThreadFactoryBuilder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ThreadFactoryBuilder.java

use crate::thread::named_thread_factory::NamedThreadFactory;
use std::thread;

type ExHandler = Option<Box<dyn Fn(&thread::Thread, Box<dyn std::any::Any + Send>) + Send + Sync>>;

/// 对齐 Java 类: `cn.hutool.core.thread.ThreadFactoryBuilder`
#[derive(Default)]
pub struct ThreadFactoryBuilder {
    name_prefix: Option<String>,
    daemon: bool,
    priority: Option<u8>,
    handler: ExHandler,
}

impl ThreadFactoryBuilder {
    /// 对齐 `ThreadFactoryBuilder.create()`。
    pub fn create() -> Self {
        Self::default()
    }

    /// 对齐 `setNamePrefix`。
    pub fn set_name_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.name_prefix = Some(prefix.into());
        self
    }

    /// 对齐 `setDaemon`。
    pub fn set_daemon(mut self, daemon: bool) -> Self {
        self.daemon = daemon;
        self
    }

    /// 对齐 `setPriority` — Rust 线程优先级不可移植，仅保留字段。
    pub fn set_priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }

    /// 对齐 `setThreadFactory` — 忽略 backing factory，仅用前缀重建。
    pub fn set_thread_factory(self, _backing: NamedThreadFactory) -> Self {
        self
    }

    /// 对齐 `setUncaughtExceptionHandler`。
    pub fn set_uncaught_exception_handler(
        mut self,
        handler: Box<dyn Fn(&thread::Thread, Box<dyn std::any::Any + Send>) + Send + Sync>,
    ) -> Self {
        self.handler = Some(handler);
        self
    }

    /// 对齐 `build()`。
    pub fn build(self) -> NamedThreadFactory {
        let _ = self.priority;
        let prefix = self.name_prefix.unwrap_or_else(|| "hitool-".to_string());
        NamedThreadFactory::with_handler(prefix, self.daemon, self.handler)
    }
}
