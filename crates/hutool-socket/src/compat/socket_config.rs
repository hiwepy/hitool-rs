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

use super::aio_server::AioServer;
use super::socket_runtime_exception::SocketRuntimeException;

/// Socket communication limits and deadlines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketConfig {
    /// Hutool `threadPoolSize`: worker concurrency / max concurrent accept sessions
    /// processed by Tokio session tasks via a `Semaphore` in [`AioServer::start`].
    thread_pool_size: usize,
    read_timeout: Duration,
    write_timeout: Duration,
    read_buffer_size: usize,
    write_buffer_size: usize,
}

impl SocketConfig {
    /// Creates validated defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the requested worker concurrency (also the accept-loop session cap).
    #[must_use]
    pub const fn thread_pool_size(&self) -> usize {
        self.thread_pool_size
    }

    /// Sets worker concurrency / max concurrent sessions (`1..=1024`).
    pub fn set_thread_pool_size(
        &mut self,
        size: usize,
    ) -> Result<&mut Self, SocketRuntimeException> {
        if size == 0 || size > 1_024 {
            return Err(SocketRuntimeException::new(
                "thread pool size must be 1..=1024",
            ));
        }
        self.thread_pool_size = size;
        Ok(self)
    }

    /// Returns the read deadline; zero disables the deadline.
    #[must_use]
    pub const fn read_timeout(&self) -> Duration {
        self.read_timeout
    }

    /// Sets the read deadline.
    pub fn set_read_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.read_timeout = timeout;
        self
    }

    /// Returns the write deadline; zero disables the deadline.
    #[must_use]
    pub const fn write_timeout(&self) -> Duration {
        self.write_timeout
    }

    /// Sets the write deadline.
    pub fn set_write_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.write_timeout = timeout;
        self
    }

    /// Returns the maximum bytes read per callback.
    #[must_use]
    pub const fn read_buffer_size(&self) -> usize {
        self.read_buffer_size
    }

    /// Sets the bounded read buffer size.
    pub fn set_read_buffer_size(
        &mut self,
        size: usize,
    ) -> Result<&mut Self, SocketRuntimeException> {
        validate_buffer(size)?;
        self.read_buffer_size = size;
        Ok(self)
    }

    /// Returns the maximum accepted write size.
    #[must_use]
    pub const fn write_buffer_size(&self) -> usize {
        self.write_buffer_size
    }

    /// Sets the bounded write buffer size.
    pub fn set_write_buffer_size(
        &mut self,
        size: usize,
    ) -> Result<&mut Self, SocketRuntimeException> {
        validate_buffer(size)?;
        self.write_buffer_size = size;
        Ok(self)
    }
}

impl Default for SocketConfig {
    fn default() -> Self {
        Self {
            thread_pool_size: std::thread::available_parallelism().map_or(1, usize::from),
            read_timeout: Duration::ZERO,
            write_timeout: Duration::ZERO,
            read_buffer_size: 8_192,
            write_buffer_size: 8_192,
        }
    }
}

fn validate_buffer(size: usize) -> Result<(), SocketRuntimeException> {
    if !(1..=16 * 1024 * 1024).contains(&size) {
        return Err(SocketRuntimeException::new(
            "socket buffer size must be 1..=16777216",
        ));
    }
    Ok(())
}
