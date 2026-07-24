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
use super::socket_runtime_exception::SocketRuntimeException;

/// NIO accept completion adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct NioAcceptHandler;

impl NioAcceptHandler {
    /// Passes an accepted session to the configured channel handler.
    pub fn completed(
        &self,
        session: AioSession,
        handler: &dyn ChannelHandler,
    ) -> Result<(), SocketRuntimeException> {
        handler.handle(session)
    }

    /// Preserves the failure as a structured result.
    pub fn failed(&self, error: SocketRuntimeException) -> Result<(), SocketRuntimeException> {
        Err(error)
    }
}

impl<F> ChannelHandler for F
where
    F: Fn(AioSession) -> Result<(), SocketRuntimeException> + Send + Sync,
{
    fn handle(&self, session: AioSession) -> Result<(), SocketRuntimeException> {
        self(session)
    }
}
