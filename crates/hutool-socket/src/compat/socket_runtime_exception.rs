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

/// Hutool-compatible socket runtime error with optional source context.
#[derive(Debug)]
pub struct SocketRuntimeException {
    message: String,
    source: Option<io::Error>,
}

impl SocketRuntimeException {
    /// Creates an error from a message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// Creates an error with an I/O source.
    #[must_use]
    pub fn with_source(message: impl Into<String>, source: io::Error) -> Self {
        Self {
            message: message.into(),
            source: Some(source),
        }
    }

    /// Formats sequential `{}` placeholders.
    #[must_use]
    pub fn formatted(template: &str, values: &[&dyn fmt::Display]) -> Self {
        let mut message = String::with_capacity(template.len());
        let mut rest = template;
        for value in values {
            if let Some(index) = rest.find("{}") {
                message.push_str(&rest[..index]);
                message.push_str(&value.to_string());
                rest = &rest[index + 2..];
            } else {
                break;
            }
        }
        message.push_str(rest);
        Self::new(message)
    }
}

impl fmt::Display for SocketRuntimeException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for SocketRuntimeException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source as &(dyn std::error::Error + 'static))
    }
}

impl From<io::Error> for SocketRuntimeException {
    fn from(error: io::Error) -> Self {
        Self::with_source(error.to_string(), error)
    }
}

impl From<SocketError> for SocketRuntimeException {
    fn from(error: SocketError) -> Self {
        Self::new(error.to_string())
    }
}
