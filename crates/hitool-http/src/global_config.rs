//! Process-scoped HTTP defaults aligned with Hutool `HttpGlobalConfig`.
//!
//! Values are stored for Hutool callers that opt in; [`HttpRequest`] /
//! [`HttpClient`] do **not** auto-apply them unless the caller reads and
//! overlays them (avoids silent process-global timeout injection).

use crate::cookie::{CookieManagerHandle, GlobalCookieManager};
use crate::HostnameVerification;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

/// Default multipart boundary used when none is configured.
pub const DEFAULT_BOUNDARY: &str = "----HiToolHttpBoundary";

/// Hutool-aligned global HTTP settings store.
///
/// Java: `cn.hutool.http.HttpGlobalConfig`
#[derive(Debug, Clone)]
pub struct HttpGlobalConfigState {
    /// Connect/read timeout in milliseconds (`-1` = unset / infinite in Hutool).
    pub timeout_ms: i32,
    /// Multipart boundary string.
    pub boundary: String,
    /// Maximum redirect hops.
    pub max_redirect_count: i32,
    /// Whether to ignore EOF / truncated body errors.
    pub ignore_eof_error: bool,
    /// Whether to decode URLs before sending.
    pub decode_url: bool,
    /// Whether PATCH is allowed (always true for reqwest; flag retained for parity).
    pub allow_patch: bool,
    /// Whether to trust any host (Dangerous hostname verification when applied).
    pub trust_any_host: bool,
}

impl Default for HttpGlobalConfigState {
    fn default() -> Self {
        Self {
            timeout_ms: -1,
            boundary: DEFAULT_BOUNDARY.to_string(),
            max_redirect_count: 0,
            ignore_eof_error: true,
            decode_url: false,
            allow_patch: true,
            trust_any_host: false,
        }
    }
}

fn state() -> &'static Mutex<HttpGlobalConfigState> {
    static STATE: OnceLock<Mutex<HttpGlobalConfigState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(HttpGlobalConfigState::default()))
}

/// Static accessors matching Hutool `HttpGlobalConfig`.
pub struct HttpGlobalConfig;

impl HttpGlobalConfig {
    /// Returns a snapshot of the current global settings.
    #[must_use]
    pub fn snapshot() -> HttpGlobalConfigState {
        state()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    /// Resets all global settings to defaults (tests).
    pub fn reset() {
        *state().lock().unwrap_or_else(|e| e.into_inner()) = HttpGlobalConfigState::default();
        GlobalCookieManager::close_cookie();
    }

    /// Java: `HttpGlobalConfig.getTimeout()`
    #[must_use]
    pub fn get_timeout() -> i32 {
        Self::snapshot().timeout_ms
    }

    /// Java: `HttpGlobalConfig.setTimeout(int)`
    pub fn set_timeout(timeout_ms: i32) {
        state().lock().unwrap_or_else(|e| e.into_inner()).timeout_ms = timeout_ms;
    }

    /// Timeout as [`Duration`] when a non-negative millisecond value is set.
    #[must_use]
    pub fn timeout_duration() -> Option<Duration> {
        let ms = Self::get_timeout();
        if ms < 0 {
            None
        } else {
            Some(Duration::from_millis(ms as u64))
        }
    }

    /// Java: `HttpGlobalConfig.getBoundary()`
    #[must_use]
    pub fn get_boundary() -> String {
        Self::snapshot().boundary
    }

    /// Java: `HttpGlobalConfig.setBoundary(String)`
    pub fn set_boundary(boundary: impl Into<String>) {
        state().lock().unwrap_or_else(|e| e.into_inner()).boundary = boundary.into();
    }

    /// Java: `HttpGlobalConfig.getMaxRedirectCount()`
    #[must_use]
    pub fn get_max_redirect_count() -> i32 {
        Self::snapshot().max_redirect_count
    }

    /// Java: `HttpGlobalConfig.setMaxRedirectCount(int)`
    pub fn set_max_redirect_count(count: i32) {
        state()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .max_redirect_count = count;
    }

    /// Java: `HttpGlobalConfig.isIgnoreEOFError()`
    #[must_use]
    pub fn is_ignore_eof_error() -> bool {
        Self::snapshot().ignore_eof_error
    }

    /// Java: `HttpGlobalConfig.setIgnoreEOFError(boolean)`
    pub fn set_ignore_eof_error(ignore: bool) {
        state()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .ignore_eof_error = ignore;
    }

    /// Java: `HttpGlobalConfig.isDecodeUrl()`
    #[must_use]
    pub fn is_decode_url() -> bool {
        Self::snapshot().decode_url
    }

    /// Java: `HttpGlobalConfig.setDecodeUrl(boolean)`
    pub fn set_decode_url(decode: bool) {
        state().lock().unwrap_or_else(|e| e.into_inner()).decode_url = decode;
    }

    /// Java: `HttpGlobalConfig.getCookieManager()` — returns the shared handle.
    #[must_use]
    pub fn get_cookie_manager() -> CookieManagerHandle {
        GlobalCookieManager::get_cookie_manager()
    }

    /// Java: `HttpGlobalConfig.setCookieManager(CookieManager)`
    pub fn set_cookie_manager(manager: CookieManagerHandle) {
        GlobalCookieManager::set_cookie_manager(manager);
    }

    /// Java: `HttpGlobalConfig.closeCookie()`
    pub fn close_cookie() {
        GlobalCookieManager::close_cookie();
    }

    /// Java: `HttpGlobalConfig.allowPatch()` — PATCH is always available via reqwest.
    pub fn allow_patch() {
        state().lock().unwrap_or_else(|e| e.into_inner()).allow_patch = true;
    }

    /// Java: `HttpGlobalConfig.isTrustAnyHost()`
    #[must_use]
    pub fn is_trust_any_host() -> bool {
        Self::snapshot().trust_any_host
    }

    /// Java: `HttpGlobalConfig.setTrustAnyHost(boolean)`
    pub fn set_trust_any_host(trust: bool) {
        state()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .trust_any_host = trust;
    }

    /// Maps [`Self::is_trust_any_host`] to a hostname verification mode.
    #[must_use]
    pub fn hostname_verification() -> HostnameVerification {
        if Self::is_trust_any_host() {
            HostnameVerification::DangerousAcceptInvalid
        } else {
            HostnameVerification::Strict
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_config_roundtrip_and_reset() {
        HttpGlobalConfig::reset();
        HttpGlobalConfig::set_timeout(3_000);
        HttpGlobalConfig::set_boundary("----Test");
        HttpGlobalConfig::set_max_redirect_count(5);
        HttpGlobalConfig::set_ignore_eof_error(false);
        HttpGlobalConfig::set_decode_url(true);
        HttpGlobalConfig::allow_patch();
        HttpGlobalConfig::set_trust_any_host(true);
        assert_eq!(HttpGlobalConfig::get_timeout(), 3_000);
        assert_eq!(HttpGlobalConfig::get_boundary(), "----Test");
        assert_eq!(HttpGlobalConfig::get_max_redirect_count(), 5);
        assert!(!HttpGlobalConfig::is_ignore_eof_error());
        assert!(HttpGlobalConfig::is_decode_url());
        assert!(HttpGlobalConfig::is_trust_any_host());
        assert!(matches!(
            HttpGlobalConfig::hostname_verification(),
            HostnameVerification::DangerousAcceptInvalid
        ));
        assert!(HttpGlobalConfig::timeout_duration().is_some());
        HttpGlobalConfig::reset();
        assert_eq!(HttpGlobalConfig::get_timeout(), -1);
        assert!(!HttpGlobalConfig::is_trust_any_host());
    }
}
