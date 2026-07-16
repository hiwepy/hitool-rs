//! Portable host and runtime information.

#![forbid(unsafe_code)]

use sysinfo::System;

/// A point-in-time view of the current host.
#[derive(Debug, Clone, PartialEq)]
pub struct SystemSnapshot {
    /// Host name reported by the operating system.
    pub host_name: Option<String>,
    /// Operating system name.
    pub os_name: Option<String>,
    /// Operating system version.
    pub os_version: Option<String>,
    /// Logical CPU count.
    pub cpu_count: usize,
    /// Average CPU usage percentage across logical CPUs.
    pub cpu_usage_percent: f32,
    /// Total memory in bytes.
    pub total_memory: u64,
    /// Used memory in bytes.
    pub used_memory: u64,
    /// System uptime in seconds.
    pub uptime_seconds: u64,
}

impl SystemSnapshot {
    /// Collects a fresh system snapshot.
    #[must_use]
    pub fn collect() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let cpus = system.cpus();
        Self {
            host_name: System::host_name(),
            os_name: System::name(),
            os_version: System::os_version(),
            cpu_count: cpus.len(),
            cpu_usage_percent: system.global_cpu_usage(),
            total_memory: system.total_memory(),
            used_memory: system.used_memory(),
            uptime_seconds: System::uptime(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_has_sane_resource_values() {
        let snapshot = SystemSnapshot::collect();
        assert!(snapshot.cpu_count > 0);
        assert!(snapshot.total_memory > 0);
        assert!(snapshot.used_memory <= snapshot.total_memory);
        assert!(snapshot.cpu_usage_percent >= 0.0);
    }
}
