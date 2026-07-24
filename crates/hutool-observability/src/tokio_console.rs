use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

/// Loopback-only Tokio console server configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokioConsoleConfig {
    /// Local address used by the console gRPC server.
    pub bind: SocketAddr,
    /// Retention window for completed async resources.
    pub retention: Duration,
}

impl Default for TokioConsoleConfig {
    fn default() -> Self {
        Self {
            bind: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6_669),
            retention: Duration::from_secs(60),
        }
    }
}

/// Creates authorized Tokio console parts without spawning a runtime task.
///
/// The layer always binds to a loopback address. Remote access must use an
/// authenticated tunnel or management proxy owned by the application.
///
/// Building applications with this feature also requires
/// `RUSTFLAGS="--cfg tokio_unstable"` so Tokio emits task instrumentation.
pub fn tokio_console_parts(
    config: &TokioConsoleConfig,
    permit: &DiagnosticPermit,
) -> Result<TokioConsoleParts, TokioConsoleError> {
    permit.require(DiagnosticAction::TokioConsole)?;
    if !config.bind.ip().is_loopback() {
        return Err(TokioConsoleError::NonLoopback(config.bind));
    }
    if config.retention.is_zero() {
        return Err(TokioConsoleError::ZeroRetention);
    }
    let (layer, server) = console_subscriber::ConsoleLayer::builder()
        .server_addr(config.bind)
        .retention(config.retention)
        .build();
    Ok(TokioConsoleParts { layer, server })
}

/// Authorized console components. The application must add `layer` to its
/// subscriber and spawn `server.serve()` on its own Tokio runtime.
