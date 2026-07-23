//! Bounded asynchronous TCP and UDP helpers backed by Tokio.

#![forbid(unsafe_code)]

use std::{io, time::Duration};
use thiserror::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::{TcpStream, ToSocketAddrs, UdpSocket},
    time,
};

mod compat;

pub use compat::{
    AioAcceptHandler, AioClient, AioServer, AioSession, ChannelHandler, ChannelUtil, IoAction,
    MsgDecoder, MsgEncoder, NioAcceptHandler, NioClient, NioServer, NioUtil, Operation, Protocol,
    ReadHandler, SimpleIoAction, SocketConfig, SocketRuntimeException, SocketUtil,
};

/// Hutool-aligned AIO namespace.
pub mod aio {
    pub use crate::{
        AioAcceptHandler as AcceptHandler, AioClient, AioServer, AioSession, IoAction, ReadHandler,
        SimpleIoAction,
    };
}

/// Hutool-aligned NIO namespace.
pub mod nio {
    pub use crate::{
        ChannelHandler, NioAcceptHandler as AcceptHandler, NioClient, NioServer, NioUtil, Operation,
    };
}

/// Hutool-aligned protocol namespace.
pub mod protocol {
    pub use crate::{MsgDecoder, MsgEncoder, Protocol};
}

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
    #[cfg(not(test))]
    return connect_tcp_inner(address, config).await;
    #[cfg(test)]
    connect_tcp_inner(address, config, ConnectFaults::default()).await
}

#[cfg(test)]
#[derive(Clone, Copy, Default)]
struct ConnectFaults {
    timeout: bool,
    policy: bool,
}

async fn connect_tcp_inner(
    address: impl ToSocketAddrs,
    config: TcpConfig,
    #[cfg(test)] faults: ConnectFaults,
) -> Result<TcpStream, SocketError> {
    if config.connect_timeout.is_zero() {
        return Err(SocketError::ConnectTimeout);
    }
    #[cfg(test)]
    let timed = if faults.timeout {
        None
    } else {
        time::timeout(config.connect_timeout, TcpStream::connect(address))
            .await
            .ok()
    };
    #[cfg(not(test))]
    let timed = time::timeout(config.connect_timeout, TcpStream::connect(address))
        .await
        .ok();
    let stream = timed.ok_or(SocketError::ConnectTimeout)??;
    #[cfg(test)]
    let policy = if faults.policy {
        Err(io::Error::other("injected socket policy failure"))
    } else {
        stream.set_nodelay(config.no_delay)
    };
    #[cfg(not(test))]
    let policy = stream.set_nodelay(config.no_delay);
    policy?;
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
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use tokio::io::ReadBuf;

    use super::*;

    #[derive(Default)]
    struct TestIo {
        input: Vec<u8>,
        offset: usize,
        output: Vec<u8>,
        read_error: bool,
        write_error: bool,
        flush_error: bool,
    }

    impl TestIo {
        fn reader(input: &[u8]) -> Self {
            Self {
                input: input.to_vec(),
                ..Self::default()
            }
        }
    }

    impl AsyncRead for TestIo {
        fn poll_read(
            mut self: Pin<&mut Self>,
            _context: &mut Context<'_>,
            buffer: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            if self.read_error {
                return Poll::Ready(Err(io::Error::other("read failed")));
            }
            let remaining = &self.input[self.offset..];
            let count = remaining.len().min(buffer.remaining());
            buffer.put_slice(&remaining[..count]);
            self.offset += count;
            Poll::Ready(Ok(()))
        }
    }

    impl AsyncWrite for TestIo {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _context: &mut Context<'_>,
            data: &[u8],
        ) -> Poll<io::Result<usize>> {
            if self.write_error {
                return Poll::Ready(Err(io::Error::other("write failed")));
            }
            self.output.extend_from_slice(data);
            Poll::Ready(Ok(data.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<()>> {
            if self.flush_error {
                Poll::Ready(Err(io::Error::other("flush failed")))
            } else {
                Poll::Ready(Ok(()))
            }
        }

        fn poll_shutdown(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }

    #[tokio::test]
    async fn framed_io_is_bounded() {
        let mut input = TestIo::reader(b"hello\nrest");
        assert_eq!(read_frame(&mut input, b'\n', 6).await.unwrap(), b"hello\n");
        let mut input = TestIo::reader(b"too-long\n");
        assert!(read_frame(&mut input, b'\n', 4).await.is_err());
        let mut input = TestIo {
            read_error: true,
            ..TestIo::default()
        };
        assert!(read_frame(&mut input, b'\n', 4).await.is_err());

        let mut output = TestIo::default();
        write_frame(&mut output, b"hello").await.unwrap();
        assert_eq!(output.output, b"hello");
        output.shutdown().await.unwrap();
        let mut output = TestIo {
            write_error: true,
            ..TestIo::default()
        };
        assert!(write_frame(&mut output, b"hello").await.is_err());
        let mut output = TestIo {
            flush_error: true,
            ..TestIo::default()
        };
        assert!(write_frame(&mut output, b"hello").await.is_err());
    }

    #[tokio::test]
    async fn tcp_udp_and_default_policy_use_real_loopback_sockets() {
        let config = TcpConfig::default();
        assert_eq!(config.connect_timeout, Duration::from_secs(10));
        assert!(config.no_delay);

        let udp = bind_udp("127.0.0.1:0").await.unwrap();
        assert!(udp.local_addr().unwrap().port() > 0);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let accepted = tokio::spawn(async move { listener.accept().await.unwrap() });
        let stream = connect_tcp(address, config).await.unwrap();
        assert!(stream.nodelay().unwrap());
        drop(accepted.await.unwrap());

        assert!(
            connect_tcp(
                address,
                TcpConfig {
                    connect_timeout: Duration::ZERO,
                    no_delay: false,
                }
            )
            .await
            .is_err()
        );
        assert!(connect_tcp(address, config).await.is_err());
        assert!(bind_udp("not a socket address").await.is_err());

        assert!(
            connect_tcp_inner(
                address,
                config,
                ConnectFaults {
                    timeout: true,
                    policy: false,
                },
            )
            .await
            .is_err()
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let accepted = tokio::spawn(async move { listener.accept().await.unwrap() });
        assert!(
            connect_tcp_inner(
                address,
                config,
                ConnectFaults {
                    timeout: false,
                    policy: true,
                },
            )
            .await
            .is_err()
        );
        drop(accepted.await.unwrap());
    }
}
