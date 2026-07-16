//! Bounded asynchronous TCP and UDP helpers backed by Tokio.

#![forbid(unsafe_code)]

use std::{io, time::Duration};
use thiserror::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::{TcpStream, ToSocketAddrs, UdpSocket},
    time,
};

/// Socket helper failures.
#[derive(Debug, Error)]
pub enum SocketError {
    /// An operating-system I/O operation failed.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// A connection attempt exceeded its deadline.
    #[error("socket connection timed out")]
    ConnectTimeout,
    /// An incoming frame exceeded the configured limit.
    #[error("frame contains {actual} bytes, exceeding limit {limit}")]
    FrameTooLarge {
        /// Configured limit.
        limit: usize,
        /// Observed bytes before termination.
        actual: usize,
    },
}

/// TCP connection policy.
#[derive(Debug, Clone, Copy)]
pub struct TcpConfig {
    /// Maximum connection establishment time.
    pub connect_timeout: Duration,
    /// Whether to disable Nagle's algorithm.
    pub no_delay: bool,
}

impl Default for TcpConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            no_delay: true,
        }
    }
}

/// Connects a TCP stream with timeout and socket policy.
pub async fn connect_tcp(
    address: impl ToSocketAddrs,
    config: TcpConfig,
) -> Result<TcpStream, SocketError> {
    let stream = time::timeout(config.connect_timeout, TcpStream::connect(address))
        .await
        .map_err(|_| SocketError::ConnectTimeout)??;
    stream.set_nodelay(config.no_delay)?;
    Ok(stream)
}

/// Binds an asynchronous UDP socket.
pub async fn bind_udp(address: impl ToSocketAddrs) -> Result<UdpSocket, SocketError> {
    Ok(UdpSocket::bind(address).await?)
}

/// Reads one delimiter-terminated frame while enforcing a byte limit.
pub async fn read_frame<R: AsyncRead + Unpin>(
    reader: R,
    delimiter: u8,
    max_bytes: usize,
) -> Result<Vec<u8>, SocketError> {
    let mut reader = BufReader::new(reader);
    let mut output = Vec::new();
    reader.read_until(delimiter, &mut output).await?;
    if output.len() > max_bytes {
        return Err(SocketError::FrameTooLarge {
            limit: max_bytes,
            actual: output.len(),
        });
    }
    Ok(output)
}

/// Writes all frame bytes and flushes the writer.
pub async fn write_frame<W: AsyncWrite + Unpin>(
    mut writer: W,
    frame: &[u8],
) -> Result<(), SocketError> {
    writer.write_all(frame).await?;
    writer.flush().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn framed_io_is_bounded() {
        assert_eq!(
            read_frame(&b"hello\nrest"[..], b'\n', 6).await.unwrap(),
            b"hello\n"
        );
        assert!(matches!(
            read_frame(&b"too-long\n"[..], b'\n', 4).await,
            Err(SocketError::FrameTooLarge { .. })
        ));
        let mut output = Vec::new();
        write_frame(&mut output, b"hello").await.unwrap();
        assert_eq!(output, b"hello");
    }
}
