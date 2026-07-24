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

use super::channel_util::ChannelUtil;
use super::socket_runtime_exception::SocketRuntimeException;

/// General socket helpers.
pub struct SocketUtil;

impl SocketUtil {
    /// Connects with the default ten-second timeout.
    pub async fn connect(address: impl ToSocketAddrs) -> Result<TcpStream, SocketRuntimeException> {
        ChannelUtil::connect(address, Duration::from_secs(10)).await
    }

    /// Connects with an explicit timeout.
    pub async fn connect_timeout(
        address: impl ToSocketAddrs,
        timeout: Duration,
    ) -> Result<TcpStream, SocketRuntimeException> {
        ChannelUtil::connect(address, timeout).await
    }

    /// Returns the peer address.
    pub fn remote_address(stream: &TcpStream) -> Result<SocketAddr, SocketRuntimeException> {
        stream.peer_addr().map_err(Into::into)
    }

    /// Checks whether the stream has no pending socket error.
    pub fn is_connected(stream: &TcpStream) -> bool {
        stream.take_error().is_ok_and(|e| e.is_none())
    }
}
