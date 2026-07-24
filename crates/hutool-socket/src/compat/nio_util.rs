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

use super::operation::Operation;
use super::socket_runtime_exception::SocketRuntimeException;

/// Tokio registers readiness with the runtime, so registration is an explicit validation operation.
pub struct NioUtil;

impl NioUtil {
    /// Validates that a stream can participate in the requested operation.
    pub fn register_channel(
        stream: &TcpStream,
        _operation: Operation,
    ) -> Result<(), SocketRuntimeException> {
        stream.local_addr().map(|_| ()).map_err(Into::into)
    }
}
