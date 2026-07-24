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
use super::io_action::IoAction;
use super::socket_config::SocketConfig;
use super::socket_runtime_exception::SocketRuntimeException;

/// AIO accept completion adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct AioAcceptHandler;

impl AioAcceptHandler {
    /// Converts an accepted Tokio stream into a session and dispatches `accept`.
    pub fn completed(
        &self,
        stream: TcpStream,
        action: Arc<dyn IoAction>,
        config: SocketConfig,
    ) -> AioSession {
        let session = AioSession::new(stream, action, config);
        session.action.accept(&session);
        session
    }

    /// Dispatches an accept failure through the action callback.
    pub fn failed(&self, error: &SocketRuntimeException, session: &AioSession) {
        session.action.failed(error, session);
    }
}
