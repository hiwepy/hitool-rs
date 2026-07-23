//! 对齐: `cn.hutool.core.thread.FinalizableDelegatedExecutorService`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/FinalizableDelegatedExecutorService.java

use crate::thread::delegated_executor_service::DelegatedExecutorService;
use crate::thread::executor_builder::SimpleExecutor;
use std::sync::Arc;

/// 对齐 Java 类: 析构时自动 shutdown 的委托执行器。
pub struct FinalizableDelegatedExecutorService {
    inner: DelegatedExecutorService,
}

impl FinalizableDelegatedExecutorService {
    /// 包装执行器。
    pub fn new(inner: Arc<SimpleExecutor>) -> Self {
        Self {
            inner: DelegatedExecutorService::new(inner),
        }
    }

    /// 访问内部委托。
    pub fn delegate(&self) -> &DelegatedExecutorService {
        &self.inner
    }
}

impl Drop for FinalizableDelegatedExecutorService {
    fn drop(&mut self) {
        self.inner.shutdown();
    }
}
