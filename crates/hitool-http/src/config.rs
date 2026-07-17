//! Hutool-aligned HTTP configuration backed by Reqwest and Rustls.

use reqwest::{Method, StatusCode, Url, header::HeaderMap, tls::Version};
use std::{fmt, sync::Arc, time::Duration};
use thiserror::Error;

/// Explicit hostname-verification policy for TLS connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HostnameVerification {
    /// Verify certificate hostnames using Rustls and WebPKI.
    #[default]
    Strict,
    /// Accept invalid certificate hostnames. This is dangerous outside tests.
    DangerousAcceptInvalid,
}

/// TLS protocol versions supported by the Rustls transport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsProtocol {
    /// TLS 1.2 only.
    Tls12,
    /// TLS 1.3 only.
    Tls13,
}

impl TlsProtocol {
    pub(crate) const fn reqwest(self) -> Version {
        match self {
            Self::Tls12 => Version::TLS_1_2,
            Self::Tls13 => Version::TLS_1_3,
        }
    }
}

/// Errors returned while building Hutool-compatible HTTP configuration.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HttpConfigError {
    /// Timeout values must be non-negative.
    #[error("HTTP timeout must be non-negative, got {0} ms")]
    NegativeTimeout(i64),
    /// A proxy URL could not be accepted by Reqwest.
    #[error("invalid HTTP proxy URL: {0}")]
    InvalidProxy(String),
    /// A TLS protocol name was blank.
    #[error("TLS protocol must not be blank")]
    BlankTlsProtocol,
    /// Rustls intentionally does not support the requested protocol.
    #[error("unsupported or insecure TLS protocol: {0}")]
    UnsupportedTlsProtocol(String),
    /// Cookie persistence was requested without compiling the cookie engine.
    #[error("redirect cookie persistence requires the `cookies` Cargo feature")]
    CookiesFeatureDisabled,
}

/// Error returned by a configured request or response interceptor.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("HTTP interceptor rejected operation: {message}")]
pub struct HttpInterceptorError {
    message: String,
}

impl HttpInterceptorError {
    /// Creates an interceptor failure with a bounded owned message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Returns the failure message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Mutable request metadata exposed to application interceptors.
#[derive(Debug, Clone)]
pub struct HttpRequestContext {
    method: Method,
    url: Url,
    headers: HeaderMap,
}

impl HttpRequestContext {
    pub(crate) fn new(method: Method, url: Url, headers: HeaderMap) -> Self {
        Self {
            method,
            url,
            headers,
        }
    }

    /// Returns the request method.
    #[must_use]
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Replaces the request method.
    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    /// Returns the destination URL.
    #[must_use]
    pub const fn url(&self) -> &Url {
        &self.url
    }

    /// Replaces the destination URL.
    pub fn set_url(&mut self, url: Url) -> &mut Self {
        self.url = url;
        self
    }

    /// Returns request headers.
    #[must_use]
    pub const fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns mutable request headers.
    pub const fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub(crate) fn into_parts(self) -> (Method, Url, HeaderMap) {
        (self.method, self.url, self.headers)
    }
}

/// Mutable response metadata exposed to application interceptors.
#[derive(Debug, Clone)]
pub struct HttpResponseContext {
    status: StatusCode,
    headers: HeaderMap,
}

impl HttpResponseContext {
    pub(crate) fn new(status: StatusCode, headers: HeaderMap) -> Self {
        Self { status, headers }
    }

    /// Returns the immutable response status.
    #[must_use]
    pub const fn status(&self) -> StatusCode {
        self.status
    }

    /// Returns response headers.
    #[must_use]
    pub const fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns mutable response headers.
    pub const fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub(crate) fn into_headers(self) -> HeaderMap {
        self.headers
    }
}

/// Shared request-interceptor callback.
pub type RequestInterceptor = Arc<
    dyn Fn(&mut HttpRequestContext) -> Result<(), HttpInterceptorError> + Send + Sync + 'static,
>;
/// Shared response-interceptor callback.
pub type ResponseInterceptor = Arc<
    dyn Fn(&mut HttpResponseContext) -> Result<(), HttpInterceptorError> + Send + Sync + 'static,
>;

/// HTTP client construction and Hutool compatibility settings.
#[derive(Clone)]
pub struct HttpConfig {
    /// TCP/TLS connection establishment timeout.
    pub connect_timeout: Duration,
    /// Entire request/read timeout.
    pub timeout: Duration,
    /// Maximum response body accepted by convenience methods.
    pub max_response_bytes: usize,
    /// User-Agent header value.
    pub user_agent: String,
    /// Maximum redirects followed by the client.
    pub redirect_limit: usize,
    /// Whether request-cache headers are disabled explicitly.
    pub disable_cache: bool,
    /// Streaming upload block size; `None` lets Reqwest choose.
    pub block_size: Option<usize>,
    /// Whether compatibility response readers may tolerate premature EOF.
    pub ignore_eof_error: bool,
    /// Whether compatibility request builders normalize pre-encoded URLs.
    pub decode_url: bool,
    /// Whether interceptors should also run for compatibility redirects.
    pub interceptor_on_redirect: bool,
    /// Whether the owned Reqwest client persists redirect cookies.
    pub follow_redirects_cookie: bool,
    /// Whether compatibility requests synthesize form content type when absent.
    pub use_default_content_type_if_null: bool,
    /// Whether compatibility readers ignore a declared Content-Length.
    pub ignore_content_length: bool,
    pub(crate) proxy_url: Option<String>,
    pub(crate) hostname_verification: HostnameVerification,
    pub(crate) tls_identity: Option<reqwest::Identity>,
    pub(crate) root_certificates: Vec<reqwest::Certificate>,
    pub(crate) tls_protocol: Option<TlsProtocol>,
    pub(crate) request_interceptors: Vec<RequestInterceptor>,
    pub(crate) response_interceptors: Vec<ResponseInterceptor>,
}

impl fmt::Debug for HttpConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HttpConfig")
            .field("connect_timeout", &self.connect_timeout)
            .field("timeout", &self.timeout)
            .field("max_response_bytes", &self.max_response_bytes)
            .field("user_agent", &self.user_agent)
            .field("redirect_limit", &self.redirect_limit)
            .field("disable_cache", &self.disable_cache)
            .field("block_size", &self.block_size)
            .field("ignore_eof_error", &self.ignore_eof_error)
            .field("decode_url", &self.decode_url)
            .field("interceptor_on_redirect", &self.interceptor_on_redirect)
            .field("follow_redirects_cookie", &self.follow_redirects_cookie)
            .field(
                "use_default_content_type_if_null",
                &self.use_default_content_type_if_null,
            )
            .field("ignore_content_length", &self.ignore_content_length)
            .field("proxy_url", &self.proxy_url.as_ref().map(|_| "<redacted>"))
            .field("hostname_verification", &self.hostname_verification)
            .field(
                "tls_identity",
                &self.tls_identity.as_ref().map(|_| "configured"),
            )
            .field("root_certificates", &self.root_certificates.len())
            .field("tls_protocol", &self.tls_protocol)
            .field("request_interceptors", &self.request_interceptors.len())
            .field("response_interceptors", &self.response_interceptors.len())
            .finish()
    }
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            timeout: Duration::from_secs(30),
            max_response_bytes: 8 * 1024 * 1024,
            user_agent: concat!("hitool-http/", env!("CARGO_PKG_VERSION")).to_owned(),
            redirect_limit: 5,
            disable_cache: false,
            block_size: None,
            ignore_eof_error: false,
            decode_url: false,
            interceptor_on_redirect: false,
            follow_redirects_cookie: false,
            use_default_content_type_if_null: true,
            ignore_content_length: false,
            proxy_url: None,
            hostname_verification: HostnameVerification::Strict,
            tls_identity: None,
            root_certificates: Vec::new(),
            tls_protocol: None,
            request_interceptors: Vec::new(),
            response_interceptors: Vec::new(),
        }
    }
}

impl HttpConfig {
    /// Creates the default Hutool-aligned configuration.
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// Sets both connection and read timeouts in milliseconds.
    pub fn timeout_millis(&mut self, milliseconds: i64) -> Result<&mut Self, HttpConfigError> {
        let timeout = duration(milliseconds)?;
        self.connect_timeout = timeout;
        self.timeout = timeout;
        Ok(self)
    }

    /// Sets the connection timeout in milliseconds.
    pub fn set_connection_timeout_millis(
        &mut self,
        milliseconds: i64,
    ) -> Result<&mut Self, HttpConfigError> {
        self.connect_timeout = duration(milliseconds)?;
        Ok(self)
    }

    /// Sets the request/read timeout in milliseconds.
    pub fn set_read_timeout_millis(
        &mut self,
        milliseconds: i64,
    ) -> Result<&mut Self, HttpConfigError> {
        self.timeout = duration(milliseconds)?;
        Ok(self)
    }

    /// Disables caches through standard request headers.
    pub const fn disable_cache(&mut self) -> &mut Self {
        self.disable_cache = true;
        self
    }

    /// Sets the redirect limit, clamping negative values to zero.
    pub fn set_max_redirect_count(&mut self, count: i32) -> &mut Self {
        self.redirect_limit = usize::try_from(count.max(0)).expect("non-negative i32 fits usize");
        self
    }

    /// Sets the TLS hostname-verification policy.
    pub const fn set_hostname_verifier(&mut self, verification: HostnameVerification) -> &mut Self {
        self.hostname_verification = verification;
        self
    }

    /// Configures an HTTP proxy from a host and port.
    pub fn set_http_proxy(&mut self, host: &str, port: u16) -> Result<&mut Self, HttpConfigError> {
        self.set_proxy(format!("http://{}:{port}", host.trim()))
    }

    /// Configures an HTTP, HTTPS, or SOCKS proxy URL.
    pub fn set_proxy(
        &mut self,
        proxy_url: impl Into<String>,
    ) -> Result<&mut Self, HttpConfigError> {
        let proxy_url = proxy_url.into();
        reqwest::Proxy::all(&proxy_url)
            .map_err(|_| HttpConfigError::InvalidProxy(proxy_url.clone()))?;
        self.proxy_url = Some(proxy_url);
        Ok(self)
    }

    /// Configures a PKCS#12 or PEM client identity instead of a Java socket factory.
    pub fn set_ssl_identity(&mut self, identity: reqwest::Identity) -> &mut Self {
        self.tls_identity = Some(identity);
        self
    }

    /// Adds a trusted PEM or DER root certificate.
    pub fn add_root_certificate(&mut self, certificate: reqwest::Certificate) -> &mut Self {
        self.root_certificates.push(certificate);
        self
    }

    /// Selects TLS 1.2 or TLS 1.3; obsolete protocols are rejected.
    pub fn set_ssl_protocol(&mut self, protocol: &str) -> Result<&mut Self, HttpConfigError> {
        let protocol = protocol.trim();
        if protocol.is_empty() {
            return Err(HttpConfigError::BlankTlsProtocol);
        }
        self.tls_protocol = Some(match protocol.to_ascii_uppercase().as_str() {
            "TLSV1.2" | "TLS1.2" => TlsProtocol::Tls12,
            "TLSV1.3" | "TLS1.3" => TlsProtocol::Tls13,
            _ => return Err(HttpConfigError::UnsupportedTlsProtocol(protocol.to_owned())),
        });
        Ok(self)
    }

    /// Sets a streaming block size; non-positive values disable the override.
    pub fn set_block_size(&mut self, block_size: i32) -> &mut Self {
        self.block_size = usize::try_from(block_size).ok().filter(|size| *size > 0);
        self
    }

    /// Sets premature-EOF compatibility handling.
    pub const fn set_ignore_eof_error(&mut self, ignore: bool) -> &mut Self {
        self.ignore_eof_error = ignore;
        self
    }

    /// Sets mixed encoded/unencoded URL normalization.
    pub const fn set_decode_url(&mut self, decode: bool) -> &mut Self {
        self.decode_url = decode;
        self
    }

    /// Adds an owned request interceptor.
    pub fn add_request_interceptor<F>(&mut self, interceptor: F) -> &mut Self
    where
        F: Fn(&mut HttpRequestContext) -> Result<(), HttpInterceptorError> + Send + Sync + 'static,
    {
        self.request_interceptors.push(Arc::new(interceptor));
        self
    }

    /// Adds an owned response interceptor.
    pub fn add_response_interceptor<F>(&mut self, interceptor: F) -> &mut Self
    where
        F: Fn(&mut HttpResponseContext) -> Result<(), HttpInterceptorError> + Send + Sync + 'static,
    {
        self.response_interceptors.push(Arc::new(interceptor));
        self
    }

    /// Sets whether compatibility redirects invoke interceptors again.
    pub const fn set_interceptor_on_redirect(&mut self, enabled: bool) -> &mut Self {
        self.interceptor_on_redirect = enabled;
        self
    }

    /// Sets whether the owned client persists redirect cookies.
    pub const fn set_follow_redirects_cookie(&mut self, enabled: bool) -> &mut Self {
        self.follow_redirects_cookie = enabled;
        self
    }

    /// Sets whether missing request content types default to form encoding.
    pub const fn set_use_default_content_type_if_null(&mut self, enabled: bool) -> &mut Self {
        self.use_default_content_type_if_null = enabled;
        self
    }

    /// Sets whether compatibility readers disregard Content-Length.
    pub const fn set_ignore_content_length(&mut self, ignore: bool) -> &mut Self {
        self.ignore_content_length = ignore;
        self
    }

    pub(crate) fn intercept_request(
        &self,
        context: &mut HttpRequestContext,
    ) -> Result<(), HttpInterceptorError> {
        for interceptor in &self.request_interceptors {
            interceptor(context)?;
        }
        Ok(())
    }

    pub(crate) fn intercept_response(
        &self,
        context: &mut HttpResponseContext,
    ) -> Result<(), HttpInterceptorError> {
        for interceptor in &self.response_interceptors {
            interceptor(context)?;
        }
        Ok(())
    }
}

fn duration(milliseconds: i64) -> Result<Duration, HttpConfigError> {
    let milliseconds =
        u64::try_from(milliseconds).map_err(|_| HttpConfigError::NegativeTimeout(milliseconds))?;
    Ok(Duration::from_millis(milliseconds))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rcgen::generate_simple_self_signed;
    use reqwest::header::{HeaderValue, USER_AGENT};

    #[derive(Default)]
    struct CountingWriter {
        writes: usize,
    }

    impl fmt::Write for CountingWriter {
        fn write_str(&mut self, _value: &str) -> fmt::Result {
            self.writes += 1;
            Ok(())
        }
    }

    struct FailingWriter {
        fail_at: usize,
        writes: usize,
    }

    impl fmt::Write for FailingWriter {
        fn write_str(&mut self, _value: &str) -> fmt::Result {
            if self.writes == self.fail_at {
                return Err(fmt::Error);
            }
            self.writes += 1;
            Ok(())
        }
    }

    #[test]
    fn configuration_setters_validate_every_hutool_shape_and_redact_secrets() {
        let mut config = HttpConfig::create();
        config
            .timeout_millis(500)
            .unwrap()
            .set_connection_timeout_millis(200)
            .unwrap()
            .set_read_timeout_millis(300)
            .unwrap()
            .disable_cache()
            .set_max_redirect_count(-1)
            .set_hostname_verifier(HostnameVerification::DangerousAcceptInvalid)
            .set_block_size(-1)
            .set_ignore_eof_error(true)
            .set_decode_url(true)
            .set_interceptor_on_redirect(true)
            .set_follow_redirects_cookie(true)
            .set_use_default_content_type_if_null(false)
            .set_ignore_content_length(true);
        assert_eq!(config.connect_timeout, Duration::from_millis(200));
        assert_eq!(config.timeout, Duration::from_millis(300));
        assert_eq!(config.redirect_limit, 0);
        assert_eq!(config.block_size, None);
        config.set_block_size(4096).set_max_redirect_count(3);
        assert_eq!(config.block_size, Some(4096));
        assert_eq!(config.redirect_limit, 3);
        assert_eq!(
            config.timeout_millis(-1).unwrap_err(),
            HttpConfigError::NegativeTimeout(-1)
        );
        assert_eq!(
            config.set_connection_timeout_millis(-2).unwrap_err(),
            HttpConfigError::NegativeTimeout(-2)
        );
        assert_eq!(
            config.set_read_timeout_millis(-3).unwrap_err(),
            HttpConfigError::NegativeTimeout(-3)
        );
        assert_eq!(
            config.set_proxy("not a proxy").unwrap_err(),
            HttpConfigError::InvalidProxy("not a proxy".to_owned())
        );
        config.set_http_proxy("127.0.0.1", 8080).unwrap();
        let debug = format!("{config:?}");
        assert!(debug.contains("<redacted>"));
        assert!(!debug.contains("127.0.0.1"));
        assert_eq!(
            config.set_ssl_protocol(" ").unwrap_err(),
            HttpConfigError::BlankTlsProtocol
        );
        assert_eq!(
            config.set_ssl_protocol("SSLv3").unwrap_err(),
            HttpConfigError::UnsupportedTlsProtocol("SSLv3".to_owned())
        );
        config.set_ssl_protocol("tls1.2").unwrap();
        assert_eq!(config.tls_protocol, Some(TlsProtocol::Tls12));
        config.set_ssl_protocol("TLSv1.3").unwrap();
        assert_eq!(config.tls_protocol, Some(TlsProtocol::Tls13));
        assert_eq!(TlsProtocol::Tls12.reqwest(), Version::TLS_1_2);
        assert_eq!(TlsProtocol::Tls13.reqwest(), Version::TLS_1_3);

        let generated = generate_simple_self_signed(vec!["localhost".to_owned()]).unwrap();
        let certificate_pem = generated.cert.pem();
        let identity_pem = format!("{certificate_pem}{}", generated.key_pair.serialize_pem());
        config
            .set_ssl_identity(reqwest::Identity::from_pem(identity_pem.as_bytes()).unwrap())
            .add_root_certificate(
                reqwest::Certificate::from_pem(certificate_pem.as_bytes()).unwrap(),
            );
        let mut counter = CountingWriter::default();
        fmt::write(&mut counter, format_args!("{config:?}")).unwrap();
        for fail_at in 0..counter.writes {
            let mut writer = FailingWriter { fail_at, writes: 0 };
            assert!(fmt::write(&mut writer, format_args!("{config:?}")).is_err());
        }
        assert!(crate::HttpClient::new(&config).is_ok());
        #[cfg(feature = "blocking")]
        assert!(crate::blocking::HttpClient::new(&config).is_ok());

        let mut corrupted = HttpConfig::default();
        corrupted.proxy_url = Some("not a proxy".to_owned());
        assert!(matches!(
            crate::HttpClient::new(&corrupted),
            Err(crate::HttpError::Config(HttpConfigError::InvalidProxy(_)))
        ));
        #[cfg(feature = "blocking")]
        assert!(matches!(
            crate::blocking::HttpClient::new(&corrupted),
            Err(crate::HttpError::Config(HttpConfigError::InvalidProxy(_)))
        ));
    }

    #[test]
    fn request_and_response_interceptors_mutate_contexts_and_propagate_errors() {
        let mut config = HttpConfig::default();
        config
            .add_request_interceptor(|context| {
                context
                    .set_method(Method::POST)
                    .set_url(Url::parse("https://example.com/changed").unwrap());
                context
                    .headers_mut()
                    .insert(USER_AGENT, HeaderValue::from_static("intercepted"));
                Ok(())
            })
            .add_response_interceptor(|context| {
                assert_eq!(context.status(), StatusCode::CREATED);
                assert!(context.headers().is_empty());
                context
                    .headers_mut()
                    .insert("x-response", HeaderValue::from_static("yes"));
                Ok(())
            });
        let mut request = HttpRequestContext::new(
            Method::GET,
            Url::parse("https://example.com/original").unwrap(),
            HeaderMap::new(),
        );
        config.intercept_request(&mut request).unwrap();
        assert_eq!(request.method(), Method::POST);
        assert_eq!(request.url().path(), "/changed");
        assert_eq!(request.headers()[USER_AGENT], "intercepted");
        let (method, url, headers) = request.into_parts();
        assert_eq!(method, Method::POST);
        assert_eq!(url.path(), "/changed");
        assert_eq!(headers[USER_AGENT], "intercepted");

        let mut response = HttpResponseContext::new(StatusCode::CREATED, HeaderMap::new());
        config.intercept_response(&mut response).unwrap();
        assert_eq!(response.into_headers()["x-response"], "yes");

        let failure = HttpInterceptorError::new("denied");
        assert_eq!(failure.message(), "denied");
        let mut rejecting = HttpConfig::default();
        rejecting
            .add_request_interceptor(move |_| Err(failure.clone()))
            .add_response_interceptor(|_| Err(HttpInterceptorError::new("response denied")));
        let mut request = HttpRequestContext::new(
            Method::GET,
            Url::parse("https://example.com").unwrap(),
            HeaderMap::new(),
        );
        assert_eq!(
            rejecting.intercept_request(&mut request).unwrap_err(),
            HttpInterceptorError::new("denied")
        );
        let mut response = HttpResponseContext::new(StatusCode::OK, HeaderMap::new());
        assert_eq!(
            rejecting.intercept_response(&mut response).unwrap_err(),
            HttpInterceptorError::new("response denied")
        );
    }
}
