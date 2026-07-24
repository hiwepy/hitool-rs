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

/// Interest operation corresponding to Hutool's NIO enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Operation {
    /// Read readiness.
    Read = 1,
    /// Write readiness.
    Write = 4,
    /// Connect readiness.
    Connect = 8,
    /// Accept readiness.
    Accept = 16,
}

impl Operation {
    /// Returns the Java NIO-compatible bit value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }
}
