use std::{fmt, sync::Arc};

use subtle::ConstantTimeEq;
use thiserror::Error;
use zeroize::Zeroizing;

/// Diagnostic capability protected by runtime authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticAction {
    /// Start or read an in-process CPU profile.
    CpuProfile,
    /// Start the Tokio console telemetry server.
    TokioConsole,
    /// Start an in-process heap profile.
    HeapProfile,
}

/// Application-supplied authorization policy for diagnostic operations.
pub trait DiagnosticsAuthorizer: Send + Sync + 'static {
    /// Returns `true` only when the credential may perform `action`.
    fn authorize(&self, action: DiagnosticAction, credential: &[u8]) -> bool;
}

#[derive(Debug, Default)]
struct DenyAll;

impl DiagnosticsAuthorizer for DenyAll {
    fn authorize(&self, _action: DiagnosticAction, _credential: &[u8]) -> bool {
        false
    }
}

/// Constant-time shared-token authorizer suitable for protected management
/// endpoints and local diagnostic tooling.
pub struct StaticTokenAuthorizer {
    token: Zeroizing<Vec<u8>>,
}

impl fmt::Debug for StaticTokenAuthorizer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StaticTokenAuthorizer")
            .field("token", &"[REDACTED]")
            .finish()
    }
}

impl StaticTokenAuthorizer {
    /// Creates an authorizer. Tokens shorter than 16 bytes are rejected.
    pub fn new(token: impl Into<Vec<u8>>) -> Result<Self, AuthorizationError> {
        let token = token.into();
        if token.len() < 16 {
            return Err(AuthorizationError::WeakToken);
        }
        Ok(Self {
            token: Zeroizing::new(token),
        })
    }
}

impl DiagnosticsAuthorizer for StaticTokenAuthorizer {
    fn authorize(&self, _action: DiagnosticAction, credential: &[u8]) -> bool {
        self.token.len() == credential.len() && bool::from(self.token.as_slice().ct_eq(credential))
    }
}

/// Runtime access controller. Its default policy denies every diagnostic
/// operation.
#[derive(Clone)]
pub struct DiagnosticsAccess {
    authorizer: Arc<dyn DiagnosticsAuthorizer>,
}

impl fmt::Debug for DiagnosticsAccess {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DiagnosticsAccess")
            .field("authorizer", &"<policy>")
            .finish()
    }
}

impl Default for DiagnosticsAccess {
    fn default() -> Self {
        Self {
            authorizer: Arc::new(DenyAll),
        }
    }
}

impl DiagnosticsAccess {
    /// Creates an access controller from an application-owned policy.
    #[must_use]
    pub fn new(authorizer: impl DiagnosticsAuthorizer) -> Self {
        Self {
            authorizer: Arc::new(authorizer),
        }
    }

    /// Authorizes one diagnostic action and returns an unforgeable permit.
    pub fn authorize(
        &self,
        action: DiagnosticAction,
        credential: &[u8],
    ) -> Result<DiagnosticPermit, AuthorizationError> {
        if self.authorizer.authorize(action, credential) {
            Ok(DiagnosticPermit { action })
        } else {
            Err(AuthorizationError::Denied(action))
        }
    }
}

/// Proof that an application authorization policy approved one diagnostic
/// action.
#[derive(Debug)]
pub struct DiagnosticPermit {
    action: DiagnosticAction,
}

impl DiagnosticPermit {
    /// Returns the diagnostic action authorized by this permit.
    #[must_use]
    pub fn action(&self) -> DiagnosticAction {
        self.action
    }

    #[cfg_attr(
        not(any(
            feature = "pprof",
            feature = "tokio-console",
            feature = "heap-profiler"
        )),
        allow(dead_code)
    )]
    pub(crate) fn require(&self, action: DiagnosticAction) -> Result<(), AuthorizationError> {
        if self.action == action {
            Ok(())
        } else {
            Err(AuthorizationError::WrongAction {
                expected: action,
                actual: self.action,
            })
        }
    }
}

/// Diagnostic authorization failures.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationError {
    /// Static diagnostic tokens must contain at least 128 bits of input.
    #[error("diagnostic token must contain at least 16 bytes")]
    WeakToken,
    /// The configured policy rejected the requested action.
    #[error("diagnostic action {0:?} was not authorized")]
    Denied(DiagnosticAction),
    /// A permit was presented for a different diagnostic action.
    #[error("permit authorizes {actual:?}, not {expected:?}")]
    WrongAction {
        /// Action required by the operation.
        expected: DiagnosticAction,
        /// Action contained in the permit.
        actual: DiagnosticAction,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN: &[u8] = b"0123456789abcdef0123456789abcdef";

    #[test]
    fn default_access_denies_diagnostics() {
        assert_eq!(
            DiagnosticsAccess::default()
                .authorize(DiagnosticAction::CpuProfile, TOKEN)
                .unwrap_err(),
            AuthorizationError::Denied(DiagnosticAction::CpuProfile)
        );
    }

    #[test]
    fn static_token_authorizes_only_matching_credentials() {
        let access = DiagnosticsAccess::new(StaticTokenAuthorizer::new(TOKEN.to_vec()).unwrap());
        let permit = access
            .authorize(DiagnosticAction::TokioConsole, TOKEN)
            .unwrap();
        permit.require(DiagnosticAction::TokioConsole).unwrap();
        assert!(matches!(
            permit.require(DiagnosticAction::HeapProfile),
            Err(AuthorizationError::WrongAction { .. })
        ));
        assert!(
            access
                .authorize(DiagnosticAction::TokioConsole, b"wrong credential")
                .is_err()
        );
    }

    #[test]
    fn weak_tokens_and_debug_secret_exposure_are_rejected() {
        assert_eq!(
            StaticTokenAuthorizer::new(b"short".to_vec()).unwrap_err(),
            AuthorizationError::WeakToken
        );
        let authorizer = StaticTokenAuthorizer::new(TOKEN.to_vec()).unwrap();
        let rendered = format!("{authorizer:?}");
        assert!(rendered.contains("[REDACTED]"));
        assert!(!rendered.contains("0123456789abcdef"));
    }
}
