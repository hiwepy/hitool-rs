//! Hutool-named socket facade backed by Tokio.

use std::fmt;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::sync::{Mutex, Notify, RwLock, Semaphore, watch};
use tokio::task::{JoinHandle, JoinSet};
use tokio::time;

use crate::{SocketError, TcpConfig, connect_tcp};

use super::io_action::IoAction;
use super::socket_config::SocketConfig;
use super::socket_runtime_exception::SocketRuntimeException;

/// Shared Tokio TCP session.
#[derive(Clone)]
pub struct AioSession {
    /// `Option` enables take/restore so I/O `.await` never holds `MutexGuard`.
    stream: Arc<Mutex<Option<TcpStream>>>,
    /// Wakes waiters after the stream is restored (serializes concurrent I/O).
    stream_available: Arc<Notify>,
    action: Arc<dyn IoAction>,
    config: SocketConfig,
    remote: SocketAddr,
}

impl fmt::Debug for AioSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AioSession")
            .field("remote", &self.remote)
            .finish_non_exhaustive()
    }
}

impl AioSession {
    fn new(stream: TcpStream, action: Arc<dyn IoAction>, config: SocketConfig) -> Self {
        let remote = stream
            .peer_addr()
            .expect("Tokio TcpStream instances are connected");
        Self {
            stream: Arc::new(Mutex::new(Some(stream))),
            stream_available: Arc::new(Notify::new()),
            action,
            config,
            remote,
        }
    }

    /// Takes the stream for one I/O op without holding `MutexGuard` across `.await`.
    /// 取出 TCP 流以执行单次 I/O，避免在 await 期间持有 MutexGuard（rust-async-patterns 修复）。
    async fn take_stream(&self) -> TcpStream {
        loop {
            // Register before checking so a restore notification cannot be missed.
            let wait = self.stream_available.notified();
            if let Some(stream) = self.stream.lock().await.take() {
                return stream;
            }
            wait.await;
        }
    }

    /// Restores the stream after I/O and wakes any waiter.
    /// I/O 完成后归还 TCP 流并唤醒等待者。
    async fn restore_stream(&self, stream: TcpStream) {
        *self.stream.lock().await = Some(stream);
        self.stream_available.notify_waiters();
    }

    /// Returns the configured read-buffer capacity.
    #[must_use]
    pub const fn read_buffer_size(&self) -> usize {
        self.config.read_buffer_size
    }
    /// Returns the configured write-buffer capacity.
    #[must_use]
    pub const fn write_buffer_size(&self) -> usize {
        self.config.write_buffer_size
    }
    /// Returns the action callback.
    #[must_use]
    pub fn io_action(&self) -> &dyn IoAction {
        self.action.as_ref()
    }
    /// Returns the peer address.
    #[must_use]
    pub const fn remote_address(&self) -> SocketAddr {
        self.remote
    }

    /// Reads one bounded chunk and dispatches it to the action.
    /// 读取一块有界数据并回调；I/O await 时不持有 MutexGuard。
    pub async fn read(&self) -> Result<usize, SocketRuntimeException> {
        let mut buffer = vec![0; self.config.read_buffer_size];
        // Anti-pattern fix: never `lock().await.read(...).await` (MutexGuard across await).
        let mut stream = self.take_stream().await;
        let result = with_timeout(self.config.read_timeout, stream.read(&mut buffer)).await;
        self.restore_stream(stream).await;
        let count = match result {
            Ok(count) => count,
            Err(error) => {
                self.action.failed(&error, self);
                return Err(error);
            }
        };
        buffer.truncate(count);
        self.action.do_action(self, &buffer);
        Ok(count)
    }

    /// Writes one bounded byte slice.
    /// 写入有界字节；I/O await 时不持有 MutexGuard。
    pub async fn write(&self, data: &[u8]) -> Result<usize, SocketRuntimeException> {
        if data.len() > self.config.write_buffer_size {
            return Err(SocketRuntimeException::new(
                "write exceeds configured buffer size",
            ));
        }
        // Anti-pattern fix: never `lock().await.write_all(...).await` (MutexGuard across await).
        let mut stream = self.take_stream().await;
        let result = with_timeout(self.config.write_timeout, stream.write_all(data)).await;
        self.restore_stream(stream).await;
        result?;
        Ok(data.len())
    }

    /// Writes bytes then shuts down the stream.
    pub async fn write_and_close(&self, data: &[u8]) -> Result<usize, SocketRuntimeException> {
        finish_write_and_close(self.write(data).await, self.close().await)
    }

    /// Returns whether the stream has no pending socket error.
    /// 流临时取出进行 I/O 时视为仍打开。
    pub async fn is_open(&self) -> bool {
        match self.stream.lock().await.as_ref() {
            Some(stream) => stream.take_error().is_ok_and(|e| e.is_none()),
            // Stream is checked out for in-flight I/O — session is still open.
            None => true,
        }
    }

    /// Shuts down the session. Tokio exposes a full async shutdown instead of Java half-close methods.
    /// 关闭会话；shutdown await 时不持有 MutexGuard。
    pub async fn close(&self) -> Result<(), SocketRuntimeException> {
        let mut stream = self.take_stream().await;
        let result = stream.shutdown().await;
        self.restore_stream(stream).await;
        result.map_err(Into::into)
    }

    /// Rust compatibility alias for closing input.
    pub async fn close_in(&self) -> Result<(), SocketRuntimeException> {
        self.close().await
    }
    /// Rust compatibility alias for closing output.
    pub async fn close_out(&self) -> Result<(), SocketRuntimeException> {
        self.close().await
    }
}

fn finish_write_and_close(
