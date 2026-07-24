use pprof::{ProfilerGuard, ProfilerGuardBuilder, protos::Message};
use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

/// CPU sampling configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuProfileConfig {
    /// Samples per second. Values from 1 through 1,000 are accepted.
    pub frequency: i32,
    /// Native frames excluded from stack collection.
    pub blocklist: Vec<String>,
}

impl Default for CpuProfileConfig {
    fn default() -> Self {
        Self {
            frequency: 99,
            blocklist: vec![
                "libc".to_owned(),
                "libgcc".to_owned(),
                "pthread".to_owned(),
                "vdso".to_owned(),
            ],
        }
    }
}

/// Authorized in-process CPU profiling session.
pub struct CpuProfileSession {
    guard: ProfilerGuard<'static>,
}

impl std::fmt::Debug for CpuProfileSession {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CpuProfileSession")
            .field("guard", &"<active>")
            .finish()
    }
}

impl CpuProfileSession {
    /// Starts CPU sampling after checking the runtime permit.
    pub fn start(
        config: &CpuProfileConfig,
        permit: &DiagnosticPermit,
    ) -> Result<Self, CpuProfileError> {
        permit.require(DiagnosticAction::CpuProfile)?;
        if !(1..=1_000).contains(&config.frequency) {
            return Err(CpuProfileError::InvalidFrequency(config.frequency));
        }
        let blocklist = config
            .blocklist
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let guard = ProfilerGuardBuilder::default()
            .frequency(config.frequency)
            .blocklist(&blocklist)
            .build()
            .map_err(|error| CpuProfileError::Backend(error.to_string()))?;
        Ok(Self { guard })
    }

    /// Builds a standard pprof protobuf payload.
    pub fn protobuf(&self, permit: &DiagnosticPermit) -> Result<Vec<u8>, CpuProfileError> {
        permit.require(DiagnosticAction::CpuProfile)?;
        let report = self
            .guard
            .report()
            .build()
            .map_err(|error| CpuProfileError::Backend(error.to_string()))?;
        let profile = report
            .pprof()
            .map_err(|error| CpuProfileError::Backend(error.to_string()))?;
        let mut output = Vec::new();
        profile
            .write_to_vec(&mut output)
            .map_err(|error| CpuProfileError::Encode(error.to_string()))?;
        Ok(output)
    }
}

/// CPU profiling failures.
#[derive(Debug, Error)]
pub enum CpuProfileError {
    /// The operation was not authorized.
    #[error(transparent)]
    Authorization(#[from] AuthorizationError),
    /// Sampling frequencies outside the supported safety range are rejected.
    #[error("CPU profile frequency must be between 1 and 1000 Hz, got {0}")]
    InvalidFrequency(i32),
    /// The native sampler failed.
    #[error("CPU profiler failed: {0}")]
    Backend(String),
    /// The report could not be encoded.
    #[error("CPU profile encoding failed: {0}")]
    Encode(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DiagnosticsAccess, StaticTokenAuthorizer};

    const TOKEN: &[u8] = b"0123456789abcdef0123456789abcdef";

    #[test]
    fn profiler_requires_matching_action_and_safe_frequency() {
        let access = DiagnosticsAccess::new(StaticTokenAuthorizer::new(TOKEN.to_vec()).unwrap());
        let cpu = access
            .authorize(DiagnosticAction::CpuProfile, TOKEN)
            .unwrap();
        let heap = access
            .authorize(DiagnosticAction::HeapProfile, TOKEN)
            .unwrap();
        assert!(matches!(
            CpuProfileSession::start(&CpuProfileConfig::default(), &heap),
            Err(CpuProfileError::Authorization(_))
        ));
        assert!(matches!(
            CpuProfileSession::start(
                &CpuProfileConfig {
                    frequency: 0,
                    blocklist: Vec::new(),
                },
                &cpu
            ),
            Err(CpuProfileError::InvalidFrequency(0))
        ));
    }
}
