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

mod socket_runtime_exception;
mod socket_config;
mod operation;
mod msg_decoder;
mod msg_encoder;
mod protocol;
mod io_action;
mod simple_io_action;
mod aio_session;
mod channel_util;
mod socket_util;
mod nio_util;
mod channel_handler;
mod aio_accept_handler;
mod read_handler;
mod nio_accept_handler;
mod aio_client;
mod aio_server;
mod nio_client;
mod nio_server;

pub use socket_runtime_exception::SocketRuntimeException;
pub use socket_config::SocketConfig;
pub use operation::Operation;
pub use msg_decoder::MsgDecoder;
pub use msg_encoder::MsgEncoder;
pub use protocol::Protocol;
pub use io_action::IoAction;
pub use simple_io_action::SimpleIoAction;
pub use aio_session::AioSession;
pub use channel_util::ChannelUtil;
pub use socket_util::SocketUtil;
pub use nio_util::NioUtil;
pub use channel_handler::ChannelHandler;
pub use aio_accept_handler::AioAcceptHandler;
pub use read_handler::ReadHandler;
pub use nio_accept_handler::NioAcceptHandler;
pub use aio_client::AioClient;
pub use aio_server::AioServer;
pub use nio_client::NioClient;
pub use nio_server::NioServer;
