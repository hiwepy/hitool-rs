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

mod filter_reload_handle;
mod tracing_config;
mod tracing_error;

pub use filter_reload_handle::FilterReloadHandle;
pub use tracing_config::TracingConfig;
pub use tracing_error::TracingError;
pub use tracing_error::install;
pub use filter_reload_handle::reloadable_filter;
pub use filter_reload_handle::reload_filter;
