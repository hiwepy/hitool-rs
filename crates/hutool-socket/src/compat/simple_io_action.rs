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

/// Convenience action with no-op accept/failure callbacks.
pub struct SimpleIoAction<F>(pub F);

impl<F> IoAction for SimpleIoAction<F>
where
    F: Fn(&AioSession, &[u8]) + Send + Sync,
{
    fn do_action(&self, session: &AioSession, data: &[u8]) {
        (self.0)(session, data);
    }
}
