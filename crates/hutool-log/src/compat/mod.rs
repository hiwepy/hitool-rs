use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

mod log_level;
mod log_record;
mod log_sink;
mod tracing_sink;
mod log;
mod abstract_log;
mod tracing_log;
mod log_factory;
mod global_log_factory;
mod static_log;

pub use log_level::LogLevel;
pub use log_record::LogRecord;
pub use log_sink::LogSink;
pub use tracing_sink::TracingSink;
pub use log::Log;
pub use abstract_log::AbstractLog;
pub use tracing_log::TracingLog;
pub use log_factory::LogFactory;
pub use global_log_factory::GlobalLogFactory;
pub use static_log::StaticLog;
pub use log::format_message;
