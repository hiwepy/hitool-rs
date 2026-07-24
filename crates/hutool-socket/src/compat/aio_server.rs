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
use super::channel_handler::ChannelHandler;
use super::io_action::IoAction;
use super::socket_config::SocketConfig;
use super::socket_runtime_exception::SocketRuntimeException;

/// AIO-shaped Tokio server with explicit task shutdown.
pub struct AioServer {
    listener: Arc<TcpListener>,
    action: Arc<RwLock<Option<Arc<dyn IoAction>>>>,
    config: SocketConfig,
    shutdown: watch::Sender<bool>,
    #[cfg(test)]
    fail_accept: bool,
}

impl AioServer {
    /// Binds a server.
    pub async fn bind(
        address: impl ToSocketAddrs,
        config: SocketConfig,
    ) -> Result<Self, SocketRuntimeException> {
        let listener = TcpListener::bind(address).await?;
        let (shutdown, _) = watch::channel(false);
        Ok(Self {
            listener: Arc::new(listener),
            action: Arc::new(RwLock::new(None)),
            config,
            shutdown,
            #[cfg(test)]
            fail_accept: false,
        })
    }
    /// Sets the singleton I/O action.
    pub async fn set_io_action(&self, action: Arc<dyn IoAction>) {
        *self.action.write().await = Some(action);
    }
    /// Returns the bound address.
    pub fn local_address(&self) -> Result<SocketAddr, SocketRuntimeException> {
        self.listener.local_addr().map_err(Into::into)
    }
    /// Returns whether shutdown has not been requested.
    #[must_use]
    pub fn is_open(&self) -> bool {
        !*self.shutdown.borrow()
    }
    /// Starts the accept loop in a managed Tokio task.
    ///
    /// Concurrent sessions are capped by [`SocketConfig::thread_pool_size`] using a
    /// `Semaphore`. When no permit is available the loop waits (backpressure) while
    /// still honouring shutdown via `select!`; the permit is held for the session
    /// task lifetime and released on drop.
    pub fn start(&self) -> JoinHandle<Result<(), SocketRuntimeException>> {
        let listener = Arc::clone(&self.listener);
        let actions = Arc::clone(&self.action);
        let config = self.config;
        let mut shutdown = self.shutdown.subscribe();
        let semaphore = Arc::new(Semaphore::new(config.thread_pool_size));
        #[cfg(test)]
        let fail_accept = self.fail_accept;
        tokio::spawn(async move {
            let mut sessions = JoinSet::new();
            loop {
                tokio::select! {
                    changed = shutdown.changed() => {
                        changed.map_err(|_| SocketRuntimeException::new("shutdown channel closed"))?;
                        sessions.shutdown().await;
                        return Ok(());
                    }
                    accepted = accept_connection(&listener, #[cfg(test)] fail_accept) => {
                        let (stream, _) = accepted?;
                        if let Some(action) = actions.read().await.clone() {
                            // Backpressure: wait for a free slot, or exit on shutdown.
                            let permit = tokio::select! {
                                changed = shutdown.changed() => {
                                    changed.map_err(|_| {
                                        SocketRuntimeException::new("shutdown channel closed")
                                    })?;
                                    drop(stream);
                                    sessions.shutdown().await;
                                    return Ok(());
                                }
                                permit = Arc::clone(&semaphore).acquire_owned() => {
                                    permit.map_err(|_| {
                                        SocketRuntimeException::new("connection semaphore closed")
                                    })?
                                }
                            };
                            let session = AioSession::new(stream, action, config);
                            session.action.accept(&session);
                            sessions.spawn(async move {
                                let _permit = permit;
                                let _ = session.read().await;
                            });
                        }
                    }
                }
            }
        })
    }
    /// Requests shutdown.
    pub fn close(&self) {
        let _ = self.shutdown.send(true);
    }
}

impl IoAction for HandlerAction {
    fn accept(&self, session: &AioSession) {
        let _ = self.0.handle(session.clone());
    }

    fn do_action(&self, _session: &AioSession, _data: &[u8]) {}
}

struct HandlerAction(Arc<dyn ChannelHandler>);
