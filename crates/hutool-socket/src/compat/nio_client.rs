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

use super::aio_client::AioClient;
use super::aio_session::AioSession;
use super::channel_handler::ChannelHandler;
use super::io_action::IoAction;
use super::socket_config::SocketConfig;
use super::socket_runtime_exception::SocketRuntimeException;

/// NIO-shaped Tokio client.
pub struct NioClient {
    client: AioClient,
    handler: Arc<dyn ChannelHandler>,
}

impl NioClient {
    /// Connects a client with a channel handler.
    pub async fn connect(
        address: impl ToSocketAddrs,
        handler: Arc<dyn ChannelHandler>,
        config: SocketConfig,
    ) -> Result<Self, SocketRuntimeException> {
        let action: Arc<dyn IoAction> = Arc::new(HandlerAction(Arc::clone(&handler)));
        let client = AioClient::connect(address, action, config).await?;
        Ok(Self { client, handler })
    }

    /// Dispatches the current session to the handler.
    pub fn listen(&self) -> Result<(), SocketRuntimeException> {
        self.handler.handle(self.client.session.clone())
    }

    /// Writes gathered byte slices in order.
    pub async fn write(&self, data: &[&[u8]]) -> Result<usize, SocketRuntimeException> {
        let total = data.iter().map(|part| part.len()).sum();
        if total > self.client.session.config.write_buffer_size {
            return Err(SocketRuntimeException::new(
                "write exceeds configured buffer size",
            ));
        }
        let mut bytes = Vec::with_capacity(total);
        for part in data {
            bytes.extend_from_slice(part);
        }
        self.client.write(&bytes).await
    }

    /// Returns the underlying session.
    #[must_use]
    pub const fn session(&self) -> &AioSession {
        self.client.session()
    }

    /// Closes the client.
    pub async fn close(&self) -> Result<(), SocketRuntimeException> {
        self.client.close().await
    }
}

struct HandlerAction(Arc<dyn ChannelHandler>);
