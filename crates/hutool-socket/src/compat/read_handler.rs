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
use super::socket_runtime_exception::SocketRuntimeException;

/// Read completion adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct ReadHandler;

impl ReadHandler {
    /// Dispatches completed bytes.
    pub fn completed(&self, session: &AioSession, data: &[u8]) {
        session.action.do_action(session, data);
    }

    /// Dispatches a read failure.
    pub fn failed(&self, error: &SocketRuntimeException, session: &AioSession) {
        session.action.failed(error, session);
    }
}
