//! 对齐: `cn.hutool.core.thread.AsyncUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/AsyncUtil.java
//!
//! # 功能说明
//!
//! Java 侧基于 [`CompletableFuture`](https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/util/concurrent/CompletableFuture.html)
//! 提供阻塞式 `waitAll` / `waitAny` / `get`。Rust 侧对应 Tokio 的
//! [`JoinHandle`] / [`Future`]:任务已由 `tokio::spawn` 调度并发执行,
//! 本工具负责等待完成并把失败统一包装为 [`ThreadException`]。
//!
//! # Feature
//!
//! 需要启用 crate feature `async`(引入可选依赖 `tokio`),默认不启用,
//! 以保持 hutool-core 同步用户无强制拉取 runtime。
//!
//! # API 映射
//!
//! | Java | Rust |
//! |------|------|
//! | `AsyncUtil.waitAll(CompletableFuture<?>...)` | [`AsyncUtil::wait_all`] |
//! | `AsyncUtil.waitAny(CompletableFuture<?>...)` | [`AsyncUtil::wait_any`] |
//! | `AsyncUtil.get(CompletableFuture<T>)` | [`AsyncUtil::get`] |

#![allow(dead_code, clippy::new_without_default)]

#[cfg(feature = "async")]
use crate::thread::thread_exception::ThreadException;
#[cfg(feature = "async")]
use std::future::Future;
#[cfg(feature = "async")]
use std::pin::Pin;
#[cfg(feature = "async")]
use std::task::{Context, Poll};
#[cfg(feature = "async")]
use tokio::task::{JoinError, JoinHandle};

/// 对齐 Java 类: `cn.hutool.core.thread.AsyncUtil`
///
/// 静态工具类在 Rust 中通过零字节 ZST + 关联函数表达。
/// 完整异步 API 需启用 feature `async`。
#[derive(Debug, Clone, Default)]
pub struct AsyncUtil;

#[cfg(feature = "async")]
impl AsyncUtil {
    /// 等待所有任务执行完毕,并将异常包装为 [`ThreadException`]。
    ///
    /// 对齐 Java: `AsyncUtil.waitAll(CompletableFuture<?>... tasks)`
    ///
    /// # 语义
    ///
    /// - 对应 `CompletableFuture.allOf(tasks).get()`。
    /// - 入参为已 `spawn` 的 [`JoinHandle`];任务本身并发运行,此处按序 `await`
    ///   仅收集完成态(与 `allOf` 的"全部完成"语义一致)。
    /// - 空迭代器立即成功返回(对齐 `allOf()` 无参时立刻完成)。
    ///
    /// # 错误
    ///
    /// 任一任务 panic / 被取消时,映射为 [`ThreadException`]
    /// (对齐 Java 将 `InterruptedException` / `ExecutionException` 包进 `ThreadException`)。
    pub async fn wait_all<T>(
        tasks: impl IntoIterator<Item = JoinHandle<T>>,
    ) -> Result<(), ThreadException> {
        // 逐个 await:JoinHandle 代表已在运行的任务,不会串行化执行体
        for handle in tasks {
            Self::await_join(handle).await?;
        }
        Ok(())
    }

    /// 等待任意一个任务执行完毕,返回其结果,并将异常包装为 [`ThreadException`]。
    ///
    /// 对齐 Java: `AsyncUtil.waitAny(CompletableFuture<?>... tasks)`
    ///
    /// # 语义
    ///
    /// - 对应 `CompletableFuture.anyOf(tasks).get()`。
    /// - 首个成功完成的任务值作为返回值;其余 handle 被 drop 后任务继续运行
    ///   (Tokio 默认 detach,对齐 Java `anyOf` 不取消其余 future)。
    /// - 空迭代器返回错误(Java `anyOf()` 空数组行为未定义/易挂起,Rust 侧显式失败)。
    ///
    /// # 类型
    ///
    /// Java `anyOf` 返回 `Object` 再强制转型;Rust 要求各任务结果类型同为 `T`。
    pub async fn wait_any<T>(
        tasks: impl IntoIterator<Item = JoinHandle<T>>,
    ) -> Result<T, ThreadException> {
        let handles: Vec<JoinHandle<T>> = tasks.into_iter().collect();
        if handles.is_empty() {
            return Err(ThreadException::new("wait_any: no tasks"));
        }
        WaitAny { handles }.await
    }

    /// 获取异步任务结果,并将异常包装为 [`ThreadException`]。
    ///
    /// 对齐 Java: `AsyncUtil.get(CompletableFuture<T> task)`
    ///
    /// # 语义
    ///
    /// 对应 `task.get()`:等待单个 [`JoinHandle`] 完成并取出 `T`。
    pub async fn get<T>(task: JoinHandle<T>) -> Result<T, ThreadException> {
        Self::await_join(task).await
    }

    /// 获取任意 [`Future`] 的结果(非 JoinHandle 便捷重载)。
    ///
    /// 对齐 Java: `AsyncUtil.get(CompletableFuture<T> task)` 的 Future 形态;
    /// Java 的 `CompletableFuture` 既可表示已提交任务也可表示组合 future,
    /// 故额外提供直接 await 任意 Future 的入口。
    pub async fn get_future<T, F>(task: F) -> Result<T, ThreadException>
    where
        F: Future<Output = T>,
    {
        Ok(task.await)
    }

    /// 将若干尚未 spawn 的 Future 提交到当前 runtime 后等待全部完成。
    ///
    /// 便捷方法:内部 `tokio::spawn` 再委托 [`Self::wait_all`],
    /// 便于从闭包/`async` 块直接对齐 Java `supplyAsync` + `waitAll` 写法。
    pub async fn wait_all_futures<T, F>(
        tasks: impl IntoIterator<Item = F>,
    ) -> Result<(), ThreadException>
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
    {
        let handles = tasks.into_iter().map(tokio::spawn).collect::<Vec<_>>();
        Self::wait_all(handles).await
    }

    /// 将若干尚未 spawn 的 Future 提交到当前 runtime 后等待任一完成。
    ///
    /// 便捷方法:内部 `tokio::spawn` 再委托 [`Self::wait_any`]。
    pub async fn wait_any_futures<T, F>(
        tasks: impl IntoIterator<Item = F>,
    ) -> Result<T, ThreadException>
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
    {
        let handles = tasks.into_iter().map(tokio::spawn).collect::<Vec<_>>();
        Self::wait_any(handles).await
    }

    /// 等待单个 [`JoinHandle`],把 [`JoinError`] 转为 [`ThreadException`]。
    async fn await_join<T>(handle: JoinHandle<T>) -> Result<T, ThreadException> {
        handle.await.map_err(Self::join_error_to_thread_exception)
    }

    /// 对齐 Java:将 `ExecutionException` / `InterruptedException` 包进 `ThreadException`。
    fn join_error_to_thread_exception(err: JoinError) -> ThreadException {
        if err.is_cancelled() {
            ThreadException::new(format!("task cancelled: {err}"))
        } else if err.is_panic() {
            ThreadException::new(format!("task panicked: {err}"))
        } else {
            ThreadException::new(format!("task join failed: {err}"))
        }
    }
}

/// 并发轮询多个 [`JoinHandle`],首个 `Ready` 即返回(对齐 `CompletableFuture.anyOf`)。
#[cfg(feature = "async")]
struct WaitAny<T> {
    handles: Vec<JoinHandle<T>>,
}

#[cfg(feature = "async")]
impl<T> Future for WaitAny<T> {
    type Output = Result<T, ThreadException>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut().get_mut();

        // 轮询所有 handle,任一完成则立即返回
        let mut index = 0;
        while index < this.handles.len() {
            match Pin::new(&mut this.handles[index]).poll(cx) {
                Poll::Ready(Ok(value)) => {
                    // 移除已完成项,其余 JoinHandle drop → detach,不取消(对齐 anyOf)
                    let _ = this.handles.swap_remove(index);
                    this.handles.clear();
                    return Poll::Ready(Ok(value));
                }
                Poll::Ready(Err(err)) => {
                    return Poll::Ready(Err(AsyncUtil::join_error_to_thread_exception(err)));
                }
                Poll::Pending => {
                    index += 1;
                }
            }
        }

        if this.handles.is_empty() {
            return Poll::Ready(Err(ThreadException::new("wait_any: no tasks")));
        }
        Poll::Pending
    }
}
