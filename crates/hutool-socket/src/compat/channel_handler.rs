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

/// Handler used by the NIO-shaped client/server facades.
pub trait ChannelHandler: Send + Sync {
    /// Handles one established session.
    fn handle(&self, session: AioSession) -> Result<(), SocketRuntimeException>;
}
