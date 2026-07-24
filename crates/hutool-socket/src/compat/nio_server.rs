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
use super::channel_handler::ChannelHandler;
use super::socket_config::SocketConfig;
use super::socket_runtime_exception::SocketRuntimeException;

/// NIO-shaped Tokio server.
pub struct NioServer {
    server: AioServer,
}

impl NioServer {
    /// Binds a NIO facade.
    pub async fn bind(
        address: impl ToSocketAddrs,
        config: SocketConfig,
    ) -> Result<Self, SocketRuntimeException> {
        Ok(Self {
            server: AioServer::bind(address, config).await?,
        })
    }

    /// Sets the channel handler.
    pub async fn set_channel_handler(&self, handler: Arc<dyn ChannelHandler>) {
        self.server
            .set_io_action(Arc::new(HandlerAction(handler)))
            .await;
    }

    /// Returns the bound address as the Tokio selector identity.
    pub fn selector(&self) -> Result<SocketAddr, SocketRuntimeException> {
        self.server.local_address()
    }

    /// Starts listening.
    pub fn start(&self) -> JoinHandle<Result<(), SocketRuntimeException>> {
        self.server.start()
    }

    /// Alias for `start`.
    pub fn listen(&self) -> JoinHandle<Result<(), SocketRuntimeException>> {
        self.start()
    }

    /// Closes the server.
    pub fn close(&self) {
        self.server.close();
    }
}

struct HandlerAction(Arc<dyn ChannelHandler>);
