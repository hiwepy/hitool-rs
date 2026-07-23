//! 对齐: `cn.hutool.core.net.URLEncoder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/net/URLEncoder.java
//!
//! 轻量 percent-encoder，委托 RFC3986 安全集。

use crate::net::rfc3986::Rfc3986;

/// 对齐 Java 类: `cn.hutool.core.net.URLEncoder`
#[derive(Debug, Clone, Copy, Default)]
pub struct UrlEncoder;

impl UrlEncoder {
    /// 对齐 Java: `URLEncoder.DEFAULT.encode(...)`（PATH）
    pub fn encode(content: &str) -> String {
        Rfc3986::encode_path(content)
    }

    /// 对齐 Java: `URLEncoder.QUERY.encode(...)`
    pub fn encode_query(content: &str) -> String {
        Rfc3986::encode_query(content)
    }

    /// 对齐 Java: `URLEncoder.FRAGMENT.encode(...)`
    pub fn encode_fragment(content: &str) -> String {
        Rfc3986::encode_fragment(content)
    }

    /// 对齐 Java: `URLEncoder.ALL.encode(...)`
    pub fn encode_all(content: &str) -> String {
        Rfc3986::encode_all(content)
    }

    /// 对齐 Java: `URLEncoder.createDefault()` 返回值的 encode 语义。
    pub fn create_default_encode(content: &str) -> String {
        Self::encode(content)
    }
}
