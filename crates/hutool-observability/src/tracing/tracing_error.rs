//! Reloadable tracing configuration owned by the final application.

use std::{env, ffi::OsString};

use thiserror::Error;
pub use tracing::{Level, debug, error, event, info, instrument, span, trace, warn};
use tracing_subscriber::{
    EnvFilter, Registry, fmt,
    layer::SubscriberExt,
    reload::{self, Handle},
    util::SubscriberInitExt,
};

use super::filter_reload_handle::FilterReloadHandle;
use super::tracing_config::TracingConfig;

/// Tracing setup failures.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TracingError {
    /// The filter expression is invalid.
    #[error("invalid tracing filter: {0}")]
    Filter(String),
    /// Another global subscriber is already installed.
    #[error("failed to install tracing subscriber: {0}")]
    Install(String),
    /// The reload layer is no longer active.
    #[error("failed to reload tracing filter: {0}")]
    Reload(String),
}

pub(crate) fn install(config: &TracingConfig) -> Result<FilterReloadHandle, TracingError> {
    let (filter, handle) = reloadable_filter(config)?;
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().compact().with_ansi(config.ansi))
        .try_init()
        .map_err(|error| TracingError::Install(error.to_string()))?;
    Ok(handle)
}
