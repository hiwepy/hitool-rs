use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;
use thiserror::Error;

/// Health state ordered from best to worst.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    /// The component is fully operational.
    Healthy,
    /// The component is operational with reduced capability.
    Degraded,
    /// The component cannot serve its intended workload.
    Unhealthy,
}

impl HealthStatus {
    #[cfg(feature = "metrics")]
    fn metric_value(self) -> f64 {
        match self {
            Self::Healthy => 1.0,
            Self::Degraded => 0.5,
            Self::Unhealthy => 0.0,
        }
    }
}

/// Snapshot of one named health check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HealthCheck {
    /// Current check status.
    pub status: HealthStatus,
    /// Optional operator-safe detail. Never put credentials or payloads here.
    pub detail: Option<String>,
    /// Last update time as milliseconds since the Unix epoch.
    pub updated_at_unix_ms: u64,
}

/// Serializable aggregate health report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HealthReport {
    /// Application or service name.
    pub service: String,
    /// Worst status among all checks.
    pub status: HealthStatus,
    /// Individual checks ordered by name.
    pub checks: BTreeMap<String, HealthCheck>,
    /// Report creation time as milliseconds since the Unix epoch.
    pub generated_at_unix_ms: u64,
}

/// Thread-safe application-owned health registry.
#[derive(Debug, Clone)]
pub struct HealthRegistry {
    service: Arc<str>,
    checks: Arc<RwLock<BTreeMap<String, HealthCheck>>>,
}

impl HealthRegistry {
    /// Creates an empty registry.
    pub fn new(service: impl Into<String>) -> Result<Self, HealthError> {
        let service = service.into();
        if service.trim().is_empty() {
            return Err(HealthError::EmptyService);
        }
        Ok(Self {
            service: Arc::from(service),
            checks: Arc::new(RwLock::new(BTreeMap::new())),
        })
    }

    /// Inserts or replaces one bounded-cardinality health check.
    pub fn set(
        &self,
        name: impl Into<String>,
        status: HealthStatus,
        detail: Option<String>,
    ) -> Result<(), HealthError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(HealthError::EmptyCheck);
        }
        let check = HealthCheck {
            status,
            detail,
            updated_at_unix_ms: unix_time_ms(),
        };
        self.checks
            .write()
            .map_err(|_| HealthError::Poisoned)?
            .insert(name.clone(), check);

        #[cfg(feature = "metrics")]
        metrics::gauge!(
            "hutool_health_check_status",
            "service" => self.service.to_string(),
            "check" => name.clone()
        )
        .set(status.metric_value());

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: "hutool.observability.health",
            service = %self.service,
            check = %name,
            ?status,
            "health check updated"
        );

        Ok(())
    }

    /// Removes one check.
    pub fn remove(&self, name: &str) -> Result<Option<HealthCheck>, HealthError> {
        Ok(self
            .checks
            .write()
            .map_err(|_| HealthError::Poisoned)?
            .remove(name))
    }

    /// Returns a consistent health snapshot.
    pub fn report(&self) -> Result<HealthReport, HealthError> {
        let checks = self
            .checks
            .read()
            .map_err(|_| HealthError::Poisoned)?
            .clone();
        let status = checks
            .values()
            .map(|check| check.status)
            .max()
            .unwrap_or(HealthStatus::Healthy);
        Ok(HealthReport {
            service: self.service.to_string(),
            status,
            checks,
            generated_at_unix_ms: unix_time_ms(),
        })
    }

    /// Returns `true` when no check is unhealthy.
    pub fn is_ready(&self) -> Result<bool, HealthError> {
        Ok(self.report()?.status != HealthStatus::Unhealthy)
    }
}

fn unix_time_ms() -> u64 {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    u64::try_from(millis).unwrap_or(u64::MAX)
}

/// Health registry failures.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum HealthError {
    /// Service names must be explicit.
    #[error("health service name cannot be empty")]
    EmptyService,
    /// Check names must be explicit and bounded by the application.
    #[error("health check name cannot be empty")]
    EmptyCheck,
    /// Another thread panicked while holding the registry lock.
    #[error("health registry lock is poisoned")]
    Poisoned,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_uses_worst_status_and_stable_names() {
        let registry = HealthRegistry::new("orders").unwrap();
        registry
            .set("database", HealthStatus::Healthy, None)
            .unwrap();
        registry
            .set(
                "cache",
                HealthStatus::Degraded,
                Some("fallback active".to_owned()),
            )
            .unwrap();
        let report = registry.report().unwrap();
        assert_eq!(report.status, HealthStatus::Degraded);
        assert_eq!(
            report.checks.keys().cloned().collect::<Vec<_>>(),
            vec!["cache", "database"]
        );
        assert!(registry.is_ready().unwrap());

        registry
            .set("database", HealthStatus::Unhealthy, None)
            .unwrap();
        assert!(!registry.is_ready().unwrap());
    }

    #[test]
    fn empty_names_are_rejected() {
        assert_eq!(
            HealthRegistry::new(" ").unwrap_err(),
            HealthError::EmptyService
        );
        let registry = HealthRegistry::new("orders").unwrap();
        assert_eq!(
            registry.set("", HealthStatus::Healthy, None).unwrap_err(),
            HealthError::EmptyCheck
        );
    }
}
