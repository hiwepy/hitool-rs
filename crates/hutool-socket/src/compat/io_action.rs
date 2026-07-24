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

/// Session lifecycle callback.
pub trait IoAction: Send + Sync {
    /// Called after a connection is established.
    fn accept(&self, _session: &AioSession) {}
    /// Called after one bounded read completes.
    fn do_action(&self, session: &AioSession, data: &[u8]);
    /// Called when a background operation fails.
    fn failed(&self, _error: &SocketRuntimeException, _session: &AioSession) {}
}
