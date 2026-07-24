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

/// Encodes one application message into bounded output bytes.
pub trait MsgEncoder<T>: Send + Sync {
    /// Encodes a message.
    fn encode(&self, session: &AioSession, value: &T) -> Result<Vec<u8>, SocketRuntimeException>;
}
