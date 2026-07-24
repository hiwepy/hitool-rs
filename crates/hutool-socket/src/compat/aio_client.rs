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

use super::aio_session::AioSession;
use super::channel_util::ChannelUtil;
use super::io_action::IoAction;
use super::socket_config::SocketConfig;
use super::socket_runtime_exception::SocketRuntimeException;

/// AIO-shaped Tokio client.
pub struct AioClient {
    session: AioSession,
}

impl AioClient {
    /// Connects and invokes the accept callback.
    pub async fn connect(
        address: impl ToSocketAddrs,
        action: Arc<dyn IoAction>,
        config: SocketConfig,
    ) -> Result<Self, SocketRuntimeException> {
        let stream = ChannelUtil::connect(address, Duration::from_secs(10)).await?;
        let session = AioSession::new(stream, action, config);
        session.action.accept(&session);
        Ok(Self { session })
    }
    /// Returns the session.
    #[must_use]
    pub const fn session(&self) -> &AioSession {
        &self.session
    }
    /// Reads and dispatches one chunk.
    pub async fn read(&self) -> Result<usize, SocketRuntimeException> {
        self.session.read().await
    }
    /// Writes bytes.
    pub async fn write(&self, data: &[u8]) -> Result<usize, SocketRuntimeException> {
        self.session.write(data).await
    }
    /// Closes the client.
    pub async fn close(&self) -> Result<(), SocketRuntimeException> {
        self.session.close().await
    }
}
