//! WebService (SOAP) facade，对齐 hutool 的 `cn.hutool.http.webservice.*`。
//!
//! 提供 SOAP 客户端和工具的 trait 抽象。
//! 具体实现依赖 `javax.xml.soap.SOAPMessage` / Jakarta SOAP，属于 unsafe-to-copy。
//! Rust 用户推荐使用 `soap-rs` 或 `xml-rs` 等替代。

use std::collections::HashMap;

use crate::HttpException;

/// SOAP 协议枚举，对齐 `cn.hutool.http.webservice.SoapProtocol`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoapProtocol {
    /// SOAP 1.1
    Soap11,
    /// SOAP 1.2
    Soap12,
}

impl Default for SoapProtocol {
    fn default() -> Self {
        SoapProtocol::Soap11
    }
}

/// Jakarta SOAP 协议枚举，对齐 `cn.hutool.http.webservice.JakartaSoapProtocol`。
pub type JakartaSoapProtocol = SoapProtocol;

/// SOAP 运行时异常，对齐 `cn.hutool.http.webservice.SoapRuntimeException`。
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct SoapRuntimeException {
    message: String,
}

impl SoapRuntimeException {
    /// 对齐 `SoapRuntimeException(String)`
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// 对齐 `SoapRuntimeException(Throwable)`
    pub fn from_error<E: std::error::Error>(e: E) -> Self {
        Self {
            message: e.to_string(),
        }
    }

    /// 获取消息
    pub fn get_message(&self) -> &str {
        &self.message
    }
}

/// SOAP 客户端，对齐 `cn.hutool.http.webservice.SoapClient`。
///
/// Java 基于 `javax.xml.soap.SOAPConnection`；Rust 用 trait 抽象。
/// 注意：所有 setter 返回 `()`（不返回 `&mut Self`）以保持 dyn-compatible。
pub trait SoapClient: Send + Sync {
    /// 对齐 `SoapClient.setCharset(Charset)`
    fn set_charset(&mut self, charset: &str);

    /// 对齐 `SoapClient.getCharset()`
    fn get_charset(&self) -> &str;

    /// 对齐 `SoapClient.setUrl(String)`
    fn set_url(&mut self, url: &str);

    /// 对齐 `SoapClient.getUrl()`
    fn get_url(&self) -> &str;

    /// 对齐 `SoapClient.setMethod(String)`
    fn set_method(&mut self, method: &str);

    /// 对齐 `SoapClient.getMethod()`
    fn get_method(&self) -> &str;

    /// 对齐 `SoapClient.setTimeout(int)`
    fn set_timeout(&mut self, timeout_ms: u64);

    /// 对齐 `SoapClient.getTimeout()`
    fn get_timeout(&self) -> u64;

    /// 对齐 `SoapClient.setProtocol(SoapProtocol)`
    fn set_protocol(&mut self, protocol: SoapProtocol);

    /// 对齐 `SoapClient.getProtocol()`
    fn get_protocol(&self) -> SoapProtocol;

    /// 对齐 `SoapClient.addBody(Map<String, Object>)`
    fn add_body(&mut self, params: HashMap<String, String>);

    /// 对齐 `SoapClient.setBody(Map<String, Object>)`
    fn set_body(&mut self, params: HashMap<String, String>);

    /// 对齐 `SoapClient.getBody()`
    fn get_body(&self) -> &HashMap<String, String>;

    /// 对齐 `SoapClient.addHeader(String, Object)`
    fn add_header(&mut self, name: &str, value: &str);

    /// 对齐 `SoapClient.getHeaders()`
    fn get_headers(&self) -> &HashMap<String, String>;

    /// 对齐 `SoapClient.addSOAPHeader(Map<String, String>)`
    fn add_soap_header(&mut self, headers: HashMap<String, String>);

    /// 对齐 `SoapClient.getSOAPHeaders()`
    fn get_soap_headers(&self) -> &HashMap<String, String>;

    /// 对齐 `SoapClient.setParameters(Map<String, Object>)`
    fn set_parameters(&mut self, params: HashMap<String, String>);

    /// 对齐 `SoapClient.getParameters()`
    fn get_parameters(&self) -> &HashMap<String, String>;

    /// 对齐 `SoapClient.addParameter(String, Object)`
    fn add_parameter(&mut self, name: &str, value: &str);

    /// 对齐 `SoapClient.getMessages(boolean pretty)`
    fn get_messages(&self, pretty: bool) -> Result<String, HttpException>;

    /// 对齐 `SoapClient.send()`
    fn send(&self) -> Result<String, HttpException>;

    /// 对齐 `SoapClient.send(boolean pretty)`
    fn send_pretty(&self, pretty: bool) -> Result<String, HttpException>;

    /// 对齐 `SoapClient.sendForString()`
    fn send_for_string(&self) -> Result<String, HttpException> {
        self.send()
    }

    /// 对齐 `SoapClient.sendForDocument()` — Rust 版返回 String（Java 返回 org.w3c.dom.Document）
    fn send_for_document(&self) -> Result<String, HttpException> {
        self.send()
    }
}

/// Jakarta SOAP 客户端，对齐 `cn.hutool.http.webservice.JakartaSoapClient`。
///
/// Java 9+ Jakarta EE 命名空间版本（javax.xml.soap → jakarta.xml.soap）。
/// Rust 中与 `SoapClient` 等价，无 javax/jakarta 区分。
pub trait JakartaSoapClient: SoapClient {}

/// SOAP 工具类，对齐 `cn.hutool.http.webservice.SoapUtil`。
pub struct SoapUtil;

impl SoapUtil {
    /// 对齐 `SoapUtil.createClient(String url)`
    pub fn create_client(_url: &str) -> Result<Box<dyn SoapClient>, HttpException> {
        Err(HttpException::new(
            "SoapUtil::create_client requires javax.xml.soap; consider soap-rs crate in Rust",
        ))
    }

    /// 对齐 `SoapUtil.createClient(String url, String method)`
    pub fn create_client_with_method(
        _url: &str,
        _method: &str,
    ) -> Result<Box<dyn SoapClient>, HttpException> {
        Self::create_client(_url)
    }
}

/// Jakarta SOAP 工具类，对齐 `cn.hutool.http.webservice.JakartaSoapUtil`。
pub type JakartaSoapUtil = SoapUtil;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soap_protocol_default() {
        let p = SoapProtocol::default();
        assert_eq!(p, SoapProtocol::Soap11);
    }

    #[test]
    fn test_soap_runtime_exception() {
        let e = SoapRuntimeException::new("test error");
        assert_eq!(e.get_message(), "test error");
    }

    #[test]
    fn test_soap_runtime_exception_from_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "io fail");
        let e = SoapRuntimeException::from_error(io_err);
        assert!(e.get_message().contains("io fail"));
    }

    #[test]
    fn test_soap_util_create_client_unsupported() {
        let r = SoapUtil::create_client("http://example.com/soap");
        assert!(r.is_err());
    }
}