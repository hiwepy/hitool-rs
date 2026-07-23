//! 对齐: `cn.hutool.core.net.URLEncodeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/net/URLEncodeUtil.java

use crate::net::rfc3986::Rfc3986;

/// 对齐 Java 类: `cn.hutool.core.net.URLEncodeUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct UrlEncodeUtil;

impl UrlEncodeUtil {
    /// 对齐 Java: `URLEncodeUtil.encode(String)` → PATH
    pub fn encode(content: &str) -> String {
        Rfc3986::encode_path(content)
    }

    /// 对齐 Java: `URLEncodeUtil.encodeQuery(String)`
    pub fn encode_query(content: &str) -> String {
        Rfc3986::encode_query(content)
    }

    /// 对齐 Java: `URLEncodeUtil.encodePathSegment(String)`
    pub fn encode_path_segment(content: &str) -> String {
        Rfc3986::encode_path_segment(content)
    }

    /// 对齐 Java: `URLEncodeUtil.encodeFragment(String)`
    pub fn encode_fragment(content: &str) -> String {
        Rfc3986::encode_fragment(content)
    }

    /// 对齐 Java: `URLEncodeUtil.encodeAll(String)`
    pub fn encode_all(content: &str) -> String {
        Rfc3986::encode_all(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_path_keeps_slash() {
        assert!(UrlEncodeUtil::encode("a/b").contains('/'));
        assert!(!UrlEncodeUtil::encode_all("a/b").contains('/'));
    }
}
