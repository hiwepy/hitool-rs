//! Metrics facade and an explicitly installed Prometheus recorder.

use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use thiserror::Error;

pub use metrics::{
    Counter, Gauge, Histogram, Unit, counter, describe_counter, describe_gauge, describe_histogram,
    gauge, histogram,
};

/// Handle for rendering the process-wide Prometheus recorder.
#[derive(Debug, Clone)]
pub struct PrometheusMetrics {
    handle: PrometheusHandle,
}

impl PrometheusMetrics {
    /// Installs the process-wide recorder.
    ///
    /// This function is intentionally explicit because the metrics recorder
    /// can only be installed once. It does not start a scrape server.
    pub fn install() -> Result<Self, MetricsError> {
        let handle = PrometheusBuilder::new()
            .with_recommended_naming(true)
            .install_recorder()
            .map_err(|error| MetricsError::Install(error.to_string()))?;
        Ok(Self { handle })
    }

    /// Renders a Prometheus text exposition payload.
    #[must_use]
    pub fn render(&self) -> String {
        self.handle.render()
    }
}

/// Metrics initialization failures.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum MetricsError {
    /// Another recorder is already installed or the recorder could not start.
    #[error("failed to install Prometheus recorder: {0}")]
    Install(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recorder_is_installed_explicitly_and_renders_metrics() {
        let metrics = PrometheusMetrics::install().unwrap();
        counter!("hutool_observability_test_counter").increment(2);
        let rendered = metrics.render();
        assert!(rendered.contains("hutool_observability_test_counter_total 2"));
    }
}
