use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

/// Hutool's five portable logging levels.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    /// Highly detailed diagnostic events.
    #[default]
    Trace,
    /// Developer-oriented diagnostic events.
    Debug,
    /// Normal application events.
    Info,
    /// Recoverable or potentially harmful events.
    Warn,
    /// Failed operations.
    Error,
}

/// A backend-neutral logging event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogRecord {
    /// Logger/category name.
    pub name: String,
    /// Event severity.
    pub level: LogLevel,
    /// Rendered event message.
    pub message: String,
    /// Optional error description.
    pub error: Option<String>,
    /// Optional fully-qualified facade or caller name.
    pub fqcn: Option<String>,
}

impl LogRecord {
    /// Creates a record with owned data suitable for asynchronous sinks.
    #[must_use]
    pub fn new(name: &str, level: LogLevel, message: &str) -> Self {
        Self {
            name: name.to_owned(),
            level,
            message: message.to_owned(),
            error: None,
            fqcn: None,
        }
    }

    /// Attaches an error description.
    #[must_use]
    pub fn with_error(mut self, error: &str) -> Self {
        self.error = Some(error.to_owned());
        self
    }

    /// Attaches the original facade/caller name.
    #[must_use]
    pub fn with_fqcn(mut self, fqcn: &str) -> Self {
        self.fqcn = Some(fqcn.to_owned());
        self
    }
}

/// Injectable destination used by all compatibility dialects.
pub trait LogSink: Send + Sync {
    /// Returns whether the destination accepts this category and level.
    fn enabled(&self, _name: &str, _level: LogLevel) -> bool {
        true
    }
    /// Emits one accepted record.
    fn emit(&self, record: &LogRecord);
}

/// Production sink backed by the mature `tracing` ecosystem.
#[derive(Debug, Default)]
pub struct TracingSink;

impl LogSink for TracingSink {
    fn enabled(&self, _name: &str, level: LogLevel) -> bool {
        match level {
            LogLevel::Trace => tracing::enabled!(target: "hitool", tracing::Level::TRACE),
            LogLevel::Debug => tracing::enabled!(target: "hitool", tracing::Level::DEBUG),
            LogLevel::Info => tracing::enabled!(target: "hitool", tracing::Level::INFO),
            LogLevel::Warn => tracing::enabled!(target: "hitool", tracing::Level::WARN),
            LogLevel::Error => tracing::enabled!(target: "hitool", tracing::Level::ERROR),
        }
    }

    fn emit(&self, record: &LogRecord) {
        let error = record.error.as_deref().unwrap_or_default();
        let fqcn = record.fqcn.as_deref().unwrap_or_default();
        match record.level {
            LogLevel::Trace => {
                tracing::trace!(target: "hitool", logger = %record.name, error, fqcn, "{}", record.message);
            }
            LogLevel::Debug => {
                tracing::debug!(target: "hitool", logger = %record.name, error, fqcn, "{}", record.message);
            }
            LogLevel::Info => {
                tracing::info!(target: "hitool", logger = %record.name, error, fqcn, "{}", record.message);
            }
            LogLevel::Warn => {
                tracing::warn!(target: "hitool", logger = %record.name, error, fqcn, "{}", record.message);
            }
            LogLevel::Error => {
                tracing::error!(target: "hitool", logger = %record.name, error, fqcn, "{}", record.message);
            }
        }
    }
}

/// Formats Hutool-style sequential `{}` placeholders.
#[must_use]
pub fn format_message(template: &str, arguments: &[&dyn fmt::Display]) -> String {
    let mut result = String::with_capacity(template.len());
    let mut remaining = template;
    let mut arguments = arguments.iter();
    while let Some(index) = remaining.find("{}") {
        result.push_str(&remaining[..index]);
        if let Some(argument) = arguments.next() {
            result.push_str(&argument.to_string());
        } else {
            result.push_str("{}");
        }
        remaining = &remaining[index + 2..];
    }
    result.push_str(remaining);
    for argument in arguments {
        result.push(' ');
        result.push_str(&argument.to_string());
    }
    result
}

/// Object-safe Hutool-compatible logger contract.
pub trait Log: Send + Sync {
    /// Returns the logger/category name.
    fn name(&self) -> &str;
    /// Returns whether the logger accepts this level.
    fn is_enabled(&self, level: LogLevel) -> bool;
    /// Emits a fully-built record when enabled.
    fn log_record(&self, record: LogRecord);

    /// Logs a plain message at an arbitrary level.
    fn log(&self, level: LogLevel, message: &str) {
        self.log_record(LogRecord::new(self.name(), level, message));
    }
    /// Logs a message and error description.
    fn log_error(&self, level: LogLevel, error: &str, message: &str) {
        self.log_record(LogRecord::new(self.name(), level, message).with_error(error));
    }
    /// Logs a message with facade/caller metadata and an optional error.
    fn log_fqcn(&self, fqcn: &str, level: LogLevel, error: Option<&str>, message: &str) {
        let mut record = LogRecord::new(self.name(), level, message).with_fqcn(fqcn);
        if let Some(error) = error {
            record = record.with_error(error);
        }
        self.log_record(record);
    }
    /// Logs a trace message.
    fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
    /// Logs a debug message.
    fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    /// Logs an informational message.
    fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    /// Logs a warning message.
    fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }
    /// Logs an error message.
    fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

/// Shared implementation corresponding to Hutool's `AbstractLog`.
#[derive(Clone)]
pub struct AbstractLog {
    name: String,
    sink: Arc<dyn LogSink>,
}

impl fmt::Debug for AbstractLog {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AbstractLog")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl AbstractLog {
    /// Creates a logger backed by an explicitly owned sink.
    #[must_use]
    pub fn new(name: &str, sink: Arc<dyn LogSink>) -> Self {
        Self {
            name: name.to_owned(),
            sink,
        }
    }

    /// Returns whether one level is accepted by the sink.
    #[must_use]
    pub fn is_trace_enabled(&self) -> bool {
        self.is_enabled(LogLevel::Trace)
    }
    /// Returns whether debug events are enabled.
    #[must_use]
    pub fn is_debug_enabled(&self) -> bool {
        self.is_enabled(LogLevel::Debug)
    }
    /// Returns whether informational events are enabled.
    #[must_use]
    pub fn is_info_enabled(&self) -> bool {
        self.is_enabled(LogLevel::Info)
    }
    /// Returns whether warning events are enabled.
    #[must_use]
    pub fn is_warn_enabled(&self) -> bool {
        self.is_enabled(LogLevel::Warn)
    }
    /// Returns whether error events are enabled.
    #[must_use]
    pub fn is_error_enabled(&self) -> bool {
        self.is_enabled(LogLevel::Error)
    }
}

impl Log for AbstractLog {
    fn name(&self) -> &str {
        &self.name
    }
    fn is_enabled(&self, level: LogLevel) -> bool {
        self.sink.enabled(&self.name, level)
    }
    fn log_record(&self, record: LogRecord) {
        if self.is_enabled(record.level) {
            self.sink.emit(&record);
        }
    }
}

/// The native `HiTool` logger; compatibility dialect names are aliases of this type.
pub type TracingLog = AbstractLog;

/// Logger factory with thread-safe instance caching and no implicit global state.
#[derive(Clone)]
pub struct LogFactory {
    name: String,
    sink: Arc<dyn LogSink>,
    cache: Arc<RwLock<HashMap<String, Arc<dyn Log>>>>,
}

impl fmt::Debug for LogFactory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LogFactory")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl Default for LogFactory {
    fn default() -> Self {
        Self::new("tracing", Arc::new(TracingSink))
    }
}

impl LogFactory {
    /// Creates an independently owned factory.
    #[must_use]
    pub fn new(name: &str, sink: Arc<dyn LogSink>) -> Self {
        Self {
            name: name.to_owned(),
            sink,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    /// Returns the factory/backend name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Gets or creates one named logger.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the internal logger-cache lock.
    #[must_use]
    pub fn get(&self, name: &str) -> Arc<dyn Log> {
        if let Some(logger) = self
            .cache
            .read()
            .expect("log cache read lock poisoned")
            .get(name)
        {
            return Arc::clone(logger);
        }
        let mut cache = self.cache.write().expect("log cache write lock poisoned");
        Arc::clone(
            cache
                .entry(name.to_owned())
                .or_insert_with(|| Arc::new(AbstractLog::new(name, Arc::clone(&self.sink)))),
        )
    }
    /// Creates an uncached logger.
    #[must_use]
    pub fn create(&self, name: &str) -> Arc<dyn Log> {
        Arc::new(AbstractLog::new(name, Arc::clone(&self.sink)))
    }
    #[must_use]
    /// Returns the number of cached logger instances.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the internal logger-cache lock.
    pub fn cached_len(&self) -> usize {
        self.cache
            .read()
            .expect("log cache read lock poisoned")
            .len()
    }
    /// Removes all cached logger instances.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the internal logger-cache lock.
    pub fn clear(&self) {
        self.cache
            .write()
            .expect("log cache write lock poisoned")
            .clear();
    }
}

/// Explicit compatibility access to Hutool's process-wide factory.
pub struct GlobalLogFactory;

fn global_slot() -> &'static RwLock<LogFactory> {
    static FACTORY: OnceLock<RwLock<LogFactory>> = OnceLock::new();
    FACTORY.get_or_init(|| RwLock::new(LogFactory::default()))
}

impl GlobalLogFactory {
    /// Returns a handle sharing the current global factory's cache.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the global factory lock.
    #[must_use]
    pub fn get() -> LogFactory {
        global_slot()
            .read()
            .expect("global log factory read lock poisoned")
            .clone()
    }
    /// Replaces the compatibility global and returns its previous value.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the global factory lock.
    pub fn set(factory: LogFactory) -> LogFactory {
        std::mem::replace(
            &mut *global_slot()
                .write()
                .expect("global log factory write lock poisoned"),
            factory,
        )
    }
    /// Restores the production tracing factory.
    pub fn reset() -> LogFactory {
        Self::set(LogFactory::default())
    }
}

/// Static facade matching Hutool's convenience entry points.
pub struct StaticLog;

impl StaticLog {
    /// Gets or creates a logger from the compatibility global.
    #[must_use]
    pub fn get(name: &str) -> Arc<dyn Log> {
        GlobalLogFactory::get().get(name)
    }
    /// Logs a message at an arbitrary level.
    pub fn log(level: LogLevel, message: &str) {
        Self::get("static").log(level, message);
    }
    /// Logs a trace message.
    pub fn trace(message: &str) {
        Self::log(LogLevel::Trace, message);
    }
    /// Logs a debug message.
    pub fn debug(message: &str) {
        Self::log(LogLevel::Debug, message);
    }
    /// Logs an informational message.
    pub fn info(message: &str) {
        Self::log(LogLevel::Info, message);
    }
    /// Logs a warning message.
    pub fn warn(message: &str) {
        Self::log(LogLevel::Warn, message);
    }
    /// Logs an error message.
    pub fn error(message: &str) {
        Self::log(LogLevel::Error, message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    struct RecordingSink {
        minimum: Mutex<LogLevel>,
        records: Mutex<Vec<LogRecord>>,
    }

    impl RecordingSink {
        fn with_minimum(minimum: LogLevel) -> Self {
            Self {
                minimum: Mutex::new(minimum),
                records: Mutex::new(Vec::new()),
            }
        }
        fn records(&self) -> Vec<LogRecord> {
            self.records.lock().expect("records lock poisoned").clone()
        }
    }

    impl LogSink for RecordingSink {
        fn enabled(&self, _name: &str, level: LogLevel) -> bool {
            let rank = |level| match level {
                LogLevel::Trace => 0,
                LogLevel::Debug => 1,
                LogLevel::Info => 2,
                LogLevel::Warn => 3,
                LogLevel::Error => 4,
            };
            rank(level) >= rank(*self.minimum.lock().expect("minimum lock poisoned"))
        }
        fn emit(&self, record: &LogRecord) {
            self.records
                .lock()
                .expect("records lock poisoned")
                .push(record.clone());
        }
    }

    struct DefaultEnabledSink;

    impl LogSink for DefaultEnabledSink {
        fn emit(&self, _record: &LogRecord) {}
    }

    #[test]
    fn records_and_formats_every_level() {
        let sink = Arc::new(RecordingSink::default());
        let logger = AbstractLog::new("demo", sink.clone());
        assert_eq!(logger.name(), "demo");
        assert!(logger.is_trace_enabled());
        assert!(logger.is_debug_enabled());
        assert!(logger.is_info_enabled());
        assert!(logger.is_warn_enabled());
        assert!(logger.is_error_enabled());
        logger.trace("trace");
        logger.debug("debug");
        logger.info("info");
        logger.warn("warn");
        logger.error("error");
        logger.log_error(LogLevel::Error, "boom", "failed");
        logger.log_fqcn("demo::caller", LogLevel::Warn, Some("bad"), "caller failed");
        logger.log_fqcn("demo::caller", LogLevel::Info, None, "caller ok");
        let records = sink.records();
        assert_eq!(records.len(), 8);
        assert_eq!(records[5].error.as_deref(), Some("boom"));
        assert_eq!(records[6].fqcn.as_deref(), Some("demo::caller"));
        assert_eq!(format!("{logger:?}"), "AbstractLog { name: \"demo\", .. }");
        assert_eq!(format_message("{} + {} = {}", &[&1, &2, &3]), "1 + 2 = 3");
        assert_eq!(format_message("{} {}", &[&1]), "1 {}");
        assert_eq!(format_message("value", &[&1, &2]), "value 1 2");
    }

    #[test]
    fn disabled_levels_are_discarded() {
        let sink = Arc::new(RecordingSink::with_minimum(LogLevel::Warn));
        let logger = AbstractLog::new("filtered", sink.clone());
        assert!(!logger.is_trace_enabled());
        logger.info("discarded");
        logger.warn("kept");
        assert_eq!(sink.records().len(), 1);
        assert!(DefaultEnabledSink.enabled("default", LogLevel::Info));
        DefaultEnabledSink.emit(&LogRecord::new("default", LogLevel::Info, "message"));
    }

    #[test]
    fn factory_caches_and_creates_loggers() {
        let sink = Arc::new(RecordingSink::default());
        let factory = LogFactory::new("recording", sink);
        assert_eq!(factory.name(), "recording");
        let first = factory.get("one");
        let second = factory.get("one");
        assert!(Arc::ptr_eq(&first, &second));
        assert!(!Arc::ptr_eq(&first, &factory.create("one")));
        assert_eq!(factory.cached_len(), 1);
        assert_eq!(
            format!("{factory:?}"),
            "LogFactory { name: \"recording\", .. }"
        );
        factory.clear();
        assert_eq!(factory.cached_len(), 0);
    }

    #[test]
    fn static_facade_uses_replaceable_global() {
        let sink = Arc::new(RecordingSink::default());
        let previous = GlobalLogFactory::set(LogFactory::new("test", sink.clone()));
        assert_eq!(GlobalLogFactory::get().name(), "test");
        StaticLog::trace("trace");
        StaticLog::debug("debug");
        StaticLog::info("info");
        StaticLog::warn("warn");
        StaticLog::error("error");
        assert_eq!(sink.records().len(), 5);
        let test_factory = GlobalLogFactory::set(previous);
        assert_eq!(test_factory.name(), "test");
        let _ = GlobalLogFactory::reset();
    }

    #[test]
    fn tracing_sink_supports_all_levels() {
        let sink = TracingSink;
        for level in [
            LogLevel::Trace,
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
        ] {
            let record = LogRecord::new("native", level, "message")
                .with_error("error")
                .with_fqcn("caller");
            let _ = sink.enabled("native", level);
            sink.emit(&record);
        }
        assert_eq!(format!("{sink:?}"), "TracingSink");
        assert_eq!(LogFactory::default().name(), "tracing");
    }
}
