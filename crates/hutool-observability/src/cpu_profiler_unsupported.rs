use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

/// CPU sampling configuration retained on unsupported platforms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuProfileConfig {
    /// Requested samples per second.
    pub frequency: i32,
    /// Requested native-frame blocklist.
    pub blocklist: Vec<String>,
}

impl Default for CpuProfileConfig {
    fn default() -> Self {
        Self {
            frequency: 99,
            blocklist: Vec::new(),
        }
    }
}

/// Uninhabited CPU profile session on non-Unix targets.
#[derive(Debug)]
pub struct CpuProfileSession {
    _private: (),
}

impl CpuProfileSession {
    /// Checks authorization and returns an unsupported-platform error.
    pub fn start(
        _config: &CpuProfileConfig,
        permit: &DiagnosticPermit,
    ) -> Result<Self, CpuProfileError> {
        permit.require(DiagnosticAction::CpuProfile)?;
        Err(CpuProfileError::UnsupportedPlatform)
    }

    /// Always returns an unsupported-platform error.
    pub fn protobuf(&self, permit: &DiagnosticPermit) -> Result<Vec<u8>, CpuProfileError> {
        permit.require(DiagnosticAction::CpuProfile)?;
        Err(CpuProfileError::UnsupportedPlatform)
    }
}

/// CPU profiling failures on non-Unix targets.
#[derive(Debug, Error)]
pub enum CpuProfileError {
    /// The operation was not authorized.
    #[error(transparent)]
    Authorization(#[from] AuthorizationError),
    /// `pprof-rs` uses POSIX sampling and is unavailable on this target.
    #[error("in-process pprof CPU sampling is supported only on Unix targets")]
    UnsupportedPlatform,
}
