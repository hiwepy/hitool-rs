//! HttpConnection facade，对齐 hutool 的 `cn.hutool.http.HttpConnection`。
//!
//! 提供 `java.net.HttpURLConnection` 的包装抽象。
//! 具体实现依赖 JDK HttpURLConnection，Rust 用 reqwest 替代。

use std::collections::HashMap;
use std::io;

use crate::HttpException;

/// HTTP 连接，对齐 `cn.hutool.http.HttpConnection`。
///
/// Java 包装 `java.net.HttpURLConnection`；Rust 用 trait 提供形状。
pub trait HttpConnection: Send + Sync {
    /// 对齐 `HttpConnection.initConn()`
    fn init_conn(&mut self) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.getMethod()`
    fn get_method(&self) -> &str;

    /// 对齐 `HttpConnection.setMethod(Method)`
    fn set_method(&mut self, method: &str) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.getUrl()`
    fn get_url(&self) -> &str;

    /// 对齐 `HttpConnection.getProxy()`
    fn get_proxy(&self) -> Option<&str>;

    /// 对齐 `HttpConnection.getHttpURLConnection()` — Rust 版返回 Option
    fn get_raw_connection(&self) -> Option<Box<dyn std::any::Any>>;

    /// 对齐 `HttpConnection.header(String, String, boolean)`
    fn header_override(
        &mut self,
        name: &str,
        value: &str,
        is_override: bool,
    ) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.header(String)` 单参数读取
    fn header(&self, name: &str) -> Option<String>;

    /// 对齐 `HttpConnection.headers()`
    fn headers(&self) -> HashMap<String, Vec<String>>;

    /// 对齐 `HttpConnection.setHttpsInfo(HostnameVerifier, SSLSocketFactory)`
    fn set_https_info(
        &mut self,
        verifier: &dyn std::any::Any,
        ssl_factory: &dyn std::any::Any,
    ) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.disableCache()`
    fn disable_cache(&mut self) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setConnectTimeout(int)`
    fn set_connect_timeout(&mut self, timeout_ms: u32) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setReadTimeout(int)`
    fn set_read_timeout(&mut self, timeout_ms: u32) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setConnectionAndReadTimeout(int)`
    fn set_connection_and_read_timeout(&mut self, timeout_ms: u32) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setCookie(String)`
    fn set_cookie(&mut self, cookie: &str) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setFixedLengthStreamingMode(long)`
    fn set_fixed_length_streaming_mode(&mut self, content_length: u64) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setChunkedStreamingMode(int)`
    fn set_chunked_streaming_mode(&mut self, block_size: i32) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setInstanceFollowRedirects(boolean)`
    fn set_instance_follow_redirects(&mut self, follow: bool) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.connect()`
    fn connect(&mut self) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.disconnectQuietly()`
    fn disconnect_quietly(&mut self);

    /// 对齐 `HttpConnection.disconnect()`
    fn disconnect(&mut self) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.getInputStream()`
    fn get_input_stream(&self) -> Result<Box<dyn io::Read>, HttpException>;

    /// 对齐 `HttpConnection.getOutputStream()`
    fn get_output_stream(&self) -> Result<Box<dyn io::Write>, HttpException>;

    /// 对齐 `HttpConnection.getResponseCode()`
    fn get_response_code(&self) -> Result<i32, HttpException>;

    /// 对齐 `HttpConnection.getResponse()`
    fn get_response(&self) -> Result<String, HttpException>;

    /// 对齐 `HttpConnection.getHeader(String)` (在 response 中查找)
    fn get_header(&self, name: &str) -> Option<String>;
}

/// 工厂函数（独立 module function 而非 trait 关联方法，保证 dyn-compatible）
pub fn http_connection_create(
    _url: &str,
    _proxy: Option<&str>,
) -> Result<Box<dyn HttpConnection>, HttpException> {
    Err(HttpException::new(
        "http_connection_create requires java.net.HttpURLConnection; use reqwest in Rust",
    ))
}

/// 简单的 Stub 实现，所有方法返回 unsupported
pub struct StubHttpConnection {
    url: String,
    method: String,
    proxy: Option<String>,
}

impl StubHttpConnection {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: "GET".to_string(),
            proxy: None,
        }
    }
}

impl HttpConnection for StubHttpConnection {
    fn init_conn(&mut self) -> Result<(), HttpException> {
        Err(HttpException::new("init_conn requires HttpURLConnection"))
    }

    fn get_method(&self) -> &str {
        &self.method
    }

    fn set_method(&mut self, method: &str) -> Result<(), HttpException> {
        self.method = method.to_string();
        Ok(())
    }

    fn get_url(&self) -> &str {
        &self.url
    }

    fn get_proxy(&self) -> Option<&str> {
        self.proxy.as_deref()
    }

    fn get_raw_connection(&self) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn header_override(
        &mut self,
        _name: &str,
        _value: &str,
        _is_override: bool,
    ) -> Result<(), HttpException> {
        Err(HttpException::new("header requires HttpURLConnection"))
    }

    fn header(&self, _name: &str) -> Option<String> {
        None
    }

    fn headers(&self) -> HashMap<String, Vec<String>> {
        HashMap::new()
    }

    fn set_https_info(
        &mut self,
        _verifier: &dyn std::any::Any,
        _ssl_factory: &dyn std::any::Any,
    ) -> Result<(), HttpException> {
        Err(HttpException::new("set_https_info requires HttpURLConnection"))
    }

    fn disable_cache(&mut self) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_connect_timeout(&mut self, _timeout_ms: u32) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_read_timeout(&mut self, _timeout_ms: u32) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_connection_and_read_timeout(&mut self, _timeout_ms: u32) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_cookie(&mut self, _cookie: &str) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_fixed_length_streaming_mode(
        &mut self,
        _content_length: u64,
    ) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_chunked_streaming_mode(&mut self, _block_size: i32) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_instance_follow_redirects(&mut self, _follow: bool) -> Result<(), HttpException> {
        Ok(())
    }

    fn connect(&mut self) -> Result<(), HttpException> {
        Err(HttpException::new("connect requires HttpURLConnection"))
    }

    fn disconnect_quietly(&mut self) {}

    fn disconnect(&mut self) -> Result<(), HttpException> {
        Ok(())
    }

    fn get_input_stream(&self) -> Result<Box<dyn io::Read>, HttpException> {
        Err(HttpException::new("get_input_stream requires HttpURLConnection"))
    }

    fn get_output_stream(&self) -> Result<Box<dyn io::Write>, HttpException> {
        Err(HttpException::new("get_output_stream requires HttpURLConnection"))
    }

    fn get_response_code(&self) -> Result<i32, HttpException> {
        Err(HttpException::new("get_response_code requires HttpURLConnection"))
    }

    fn get_response(&self) -> Result<String, HttpException> {
        Err(HttpException::new("get_response requires HttpURLConnection"))
    }

    fn get_header(&self, _name: &str) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub_connection_basic() {
        let conn = StubHttpConnection::new("http://example.com");
        assert_eq!(conn.get_url(), "http://example.com");
        assert_eq!(conn.get_method(), "GET");
        assert_eq!(conn.get_proxy(), None);
    }

    #[test]
    fn test_stub_connection_set_method() {
        let mut conn = StubHttpConnection::new("http://example.com");
        conn.set_method("POST").unwrap();
        assert_eq!(conn.get_method(), "POST");
    }

    #[test]
    fn test_stub_connection_no_op_methods() {
        let mut conn = StubHttpConnection::new("http://example.com");
        assert!(conn.disable_cache().is_ok());
        assert!(conn.set_connect_timeout(5000).is_ok());
        assert!(conn.set_read_timeout(5000).is_ok());
        assert!(conn.disconnect().is_ok());
        assert!(conn.set_cookie("foo=bar").is_ok());
    }

    #[test]
    fn test_stub_connection_io_methods_unsupported() {
        let conn = StubHttpConnection::new("http://example.com");
        assert!(conn.get_input_stream().is_err());
        assert!(conn.get_output_stream().is_err());
        assert!(conn.get_response_code().is_err());
        let mut conn = conn;
        assert!(conn.connect().is_err());
    }

    #[test]
    fn test_stub_connection_create_unsupported() {
        let r = http_connection_create("http://x", None);
        assert!(r.is_err());
    }
}