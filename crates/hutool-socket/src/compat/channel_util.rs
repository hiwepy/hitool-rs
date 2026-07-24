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

use super::socket_runtime_exception::SocketRuntimeException;

/// Connection helpers corresponding to Hutool's `ChannelUtil`.
pub struct ChannelUtil;

impl ChannelUtil {
    /// Validates a requested Tokio worker-group size.
    pub fn create_fixed_group(pool_size: usize) -> Result<usize, SocketRuntimeException> {
        if pool_size == 0 || pool_size > 1_024 {
            return Err(SocketRuntimeException::new(
                "thread pool size must be 1..=1024",
            ));
        }
        Ok(pool_size)
    }

    /// Connects a Tokio TCP channel.
    pub async fn connect(
        address: impl ToSocketAddrs,
        timeout: Duration,
    ) -> Result<TcpStream, SocketRuntimeException> {
        connect_tcp(
            address,
            TcpConfig {
                connect_timeout: timeout,
                no_delay: true,
            },
        )
        .await
        .map_err(Into::into)
    }
}
