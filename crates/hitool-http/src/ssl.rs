//! SSL facade，对齐 hutool 的 `cn.hutool.http.ssl.*`。
//!
//! 提供定制协议的 SSLSocketFactory 抽象 + 信任任意主机名验证器。
//! 具体实现依赖 `javax.net.ssl.SSLSocketFactory`，Rust 用 reqwest 内置 rustls 替代。

use std::any::Any;
use std::io;

/// 自定义协议 SSL 工厂，对齐 `cn.hutool.http.ssl.CustomProtocolsSSLFactory`。
///
/// Java 继承 `SSLSocketFactory`；Rust 用 trait 提供形状。
pub trait CustomProtocolsSslFactory: Send + Sync {
    /// 对齐 `getDefaultCipherSuites()`
    fn get_default_cipher_suites(&self) -> Vec<String>;

    /// 对齐 `getSupportedCipherSuites()`
    fn get_supported_cipher_suites(&self) -> Vec<String>;

    /// 对齐 `createSocket()`
    fn create_socket(&self) -> io::Result<Box<dyn Any>>;

    /// 对齐 `createSocket(Socket, String, int, boolean)`
    fn create_socket_wrapped(
        &self,
        socket: Box<dyn Any>,
        host: &str,
        port: u16,
        auto_close: bool,
    ) -> io::Result<Box<dyn Any>>;

    /// 对齐 `createSocket(String, int)`
    fn create_socket_host(&self, host: &str, port: u16) -> io::Result<Box<dyn Any>>;

    /// 对齐 `createSocket(String, int, InetAddress, int)` (InetAddress 用 &str 表示)
    fn create_socket_host_with_client(
        &self,
        host: &str,
        port: u16,
        client_host: &str,
        client_port: u16,
    ) -> io::Result<Box<dyn Any>>;

    /// 对齐 `createSocket(InetAddress, int)` (InetAddress 用 &str 表示)
    fn create_socket_inet(&self, host: &str, port: u16) -> io::Result<Box<dyn Any>>;

    /// 对齐 `createSocket(InetAddress, int, InetAddress, int)` (InetAddress 用 &str 表示)
    fn create_socket_inet_with_client(
        &self,
        host: &str,
        port: u16,
        client_host: &str,
        client_port: u16,
    ) -> io::Result<Box<dyn Any>>;
}

/// 简单实现：仅记录 protocol 列表，不创建真实 SSLSocket
pub struct CustomProtocolsSslFactoryImpl {
    protocols: Vec<String>,
}

impl CustomProtocolsSslFactoryImpl {
    /// 对齐 `CustomProtocolsSSLFactory(String... protocols)`
    pub fn new(protocols: &[&str]) -> Self {
        Self {
            protocols: protocols.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// 获取配置的协议列表
    pub fn protocols(&self) -> &[String] {
        &self.protocols
    }
}

impl CustomProtocolsSslFactory for CustomProtocolsSslFactoryImpl {
    fn get_default_cipher_suites(&self) -> Vec<String> {
        // 返回常见 cipher suites（与 rustls 默认对齐）
        vec![
            "TLS_AES_128_GCM_SHA256".into(),
            "TLS_AES_256_GCM_SHA384".into(),
            "TLS_CHACHA20_POLY1305_SHA256".into(),
        ]
    }

    fn get_supported_cipher_suites(&self) -> Vec<String> {
        self.get_default_cipher_suites()
    }

    fn create_socket(&self) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "CustomProtocolsSslFactoryImpl::create_socket requires javax.net.ssl; use reqwest rustls in Rust",
        ))
    }

    fn create_socket_wrapped(
        &self,
        _socket: Box<dyn Any>,
        _host: &str,
        _port: u16,
        _auto_close: bool,
    ) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "CustomProtocolsSslFactoryImpl::create_socket_wrapped requires javax.net.ssl",
        ))
    }

    fn create_socket_host(&self, _host: &str, _port: u16) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "create_socket_host requires javax.net.ssl",
        ))
    }

    fn create_socket_host_with_client(
        &self,
        _host: &str,
        _port: u16,
        _client_host: &str,
        _client_port: u16,
    ) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "create_socket_host_with_client requires javax.net.ssl",
        ))
    }

    fn create_socket_inet(&self, _host: &str, _port: u16) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "create_socket_inet requires javax.net.ssl",
        ))
    }

    fn create_socket_inet_with_client(
        &self,
        _host: &str,
        _port: u16,
        _client_host: &str,
        _client_port: u16,
    ) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "create_socket_inet_with_client requires javax.net.ssl",
        ))
    }
}

/// 信任任意主机名验证器，对齐 `cn.hutool.http.ssl.TrustAnyHostnameVerifier`。
///
/// **安全警告**：此 trait 用于开发/测试，**禁止用于生产环境**。
/// Rust 用户应使用 reqwest 默认的 rustls 验证。
pub trait TrustAnyHostnameVerifier: Send + Sync {
    /// 对齐 `verify(String, SSLSession)`
    ///
    /// 始终返回 `true`，**不安全**。
    fn verify(&self, _hostname: &str, _session: &dyn Any) -> bool {
        true
    }
}

/// 桩实现
pub struct TrustAnyHostnameVerifierImpl;

impl TrustAnyHostnameVerifier for TrustAnyHostnameVerifierImpl {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_protocols() {
        let f = CustomProtocolsSslFactoryImpl::new(&["TLSv1.2", "TLSv1.3"]);
        assert_eq!(f.protocols(), &["TLSv1.2".to_string(), "TLSv1.3".to_string()]);
    }

    #[test]
    fn test_factory_cipher_suites() {
        let f = CustomProtocolsSslFactoryImpl::new(&["TLSv1.2"]);
        let ciphers = f.get_default_cipher_suites();
        assert!(!ciphers.is_empty());
        assert!(ciphers.iter().any(|c| c.contains("AES_128_GCM")));
    }

    #[test]
    fn test_factory_create_socket_unsupported() {
        let f = CustomProtocolsSslFactoryImpl::new(&["TLSv1.2"]);
        let r = f.create_socket();
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().kind(), io::ErrorKind::Unsupported);
    }

    #[test]
    fn test_trust_any_hostname_verifier() {
        let v = TrustAnyHostnameVerifierImpl;
        // 总是返回 true
        assert!(v.verify("any.host.com", &()));
    }
}