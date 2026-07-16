//! Structured application logging built on the Rust `tracing` ecosystem.

#![forbid(unsafe_code)]

use std::fmt as std_fmt;
pub use tracing::{Level, debug, error, event, info, instrument, span, trace, warn};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Installs a compact global text subscriber.
///
/// The `RUST_LOG` environment variable takes precedence over `default_filter`.
pub fn init(default_filter: &str) -> Result<(), tracing_subscriber::util::TryInitError> {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().compact())
        .try_init()
}

/// Creates an environment-aware filter without installing a global subscriber.
#[must_use]
pub fn env_filter(default_filter: &str) -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter))
}

/// A display/debug wrapper that never reveals the wrapped value.
pub struct Redacted<T>(pub T);

impl<T> std_fmt::Debug for Redacted<T> {
    fn fmt(&self, formatter: &mut std_fmt::Formatter<'_>) -> std_fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

impl<T> std_fmt::Display for Redacted<T> {
    fn fmt(&self, formatter: &mut std_fmt::Formatter<'_>) -> std_fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

impl<T> Redacted<T> {
    /// Borrows the wrapped secret for deliberate use.
    #[must_use]
    pub fn expose(&self) -> &T {
        &self.0
    }

    /// Consumes the wrapper and returns its value.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_filter_is_valid() {
        assert!(!env_filter("info,hitool=debug").to_string().is_empty());
    }

    #[test]
    fn redacted_values_do_not_leak_through_formatting() {
        let secret = Redacted("api-key");
        assert_eq!(format!("{secret}"), "[REDACTED]");
        assert_eq!(format!("{secret:?}"), "[REDACTED]");
        assert_eq!(secret.expose(), &"api-key");
    }
}
