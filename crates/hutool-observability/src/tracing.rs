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

/// Runtime-reloadable filter handle.
pub type FilterReloadHandle = Handle<EnvFilter, Registry>;

/// Default text tracing configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TracingConfig {
    /// Filter used when `RUST_LOG` is absent or invalid.
    pub default_filter: String,
    /// Whether terminal output may contain ANSI color.
    pub ansi: bool,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            default_filter: "info".to_owned(),
            ansi: true,
        }
    }
}

/// Installs a compact global subscriber and returns its reload handle.
///
/// Call this only from the final application entry point. Libraries should
/// emit spans and let the application choose its subscriber.
pub fn install(config: &TracingConfig) -> Result<FilterReloadHandle, TracingError> {
    let (filter, handle) = reloadable_filter(config)?;
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().compact().with_ansi(config.ansi))
        .try_init()
        .map_err(|error| TracingError::Install(error.to_string()))?;
    Ok(handle)
}

/// Creates a reloadable filter layer without installing global state.
pub fn reloadable_filter(
    config: &TracingConfig,
) -> Result<(reload::Layer<EnvFilter, Registry>, FilterReloadHandle), TracingError> {
    let filter = filter_from_value(env::var_os("RUST_LOG"), &config.default_filter)?;
    Ok(reload::Layer::new(filter))
}

/// Updates a previously installed filter.
pub fn reload_filter(handle: &FilterReloadHandle, filter: &str) -> Result<(), TracingError> {
    let filter =
        EnvFilter::try_new(filter).map_err(|error| TracingError::Filter(error.to_string()))?;
    handle
        .reload(filter)
        .map_err(|error| TracingError::Reload(error.to_string()))
}

fn filter_from_value(
    value: Option<OsString>,
    default_filter: &str,
) -> Result<EnvFilter, TracingError> {
    if let Some(value) = value.and_then(|value| value.into_string().ok()) {
        if let Ok(filter) = EnvFilter::try_new(value) {
            return Ok(filter);
        }
    }
    EnvFilter::try_new(default_filter).map_err(|error| TracingError::Filter(error.to_string()))
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_layer_can_be_composed_and_reloaded() {
        let config = TracingConfig {
            default_filter: "hutool=info".to_owned(),
            ansi: false,
        };
        let (layer, handle) = reloadable_filter(&config).unwrap();
        reload_filter(&handle, "hutool=debug").unwrap();
        drop(layer);
        assert!(reload_filter(&handle, "hutool=trace").is_err());
    }

    #[test]
    fn invalid_default_filter_is_rejected() {
        assert!(filter_from_value(None, "[").is_err());
    }
}
