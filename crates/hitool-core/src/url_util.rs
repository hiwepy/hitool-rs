//! 对齐: `cn.hutool.core.util.URLUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/URLUtil.java
//!
//! Rust 版本提供 URL 操作的 idiomatic 实现。

/// 对齐 Java: `cn.hutool.core.util.URLUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct UrlUtil;

impl UrlUtil {
    // ── URL 验证 ──

    /// 对齐 Java: `URLUtil.isUrl(CharSequence)`
    pub fn is_url(value: &str) -> bool {
        value.starts_with("http://") || value.starts_with("https://") || value.starts_with("ftp://")
    }

    /// 对齐 Java: `URLUtil.isHttp(CharSequence)`
    pub fn is_http(value: &str) -> bool {
        value.starts_with("http://")
    }

    /// 对齐 Java: `URLUtil.isHttps(CharSequence)`
    pub fn is_https(value: &str) -> bool {
        value.starts_with("https://")
    }

    // ── URL 解析 ──

    /// 对齐 Java: `URLUtil.getHost(URL)`
    pub fn get_host(url: &str) -> Option<&str> {
        let url = url.trim_start_matches("http://").trim_start_matches("https://").trim_start_matches("ftp://");
        url.split('/').next().and_then(|host| {
            if host.is_empty() {
                None
            } else {
                Some(host)
            }
        })
    }

    /// 对齐 Java: `URLUtil.getPath(URL)`
    pub fn get_path(url: &str) -> &str {
        let url = url.trim_start_matches("http://").trim_start_matches("https://").trim_start_matches("ftp://");
        if let Some(pos) = url.find('/') {
            &url[pos..]
        } else {
            "/"
        }
    }

    /// 对齐 Java: `URLUtil.getProtocol(URL)`
    pub fn get_protocol(url: &str) -> Option<&str> {
        if url.starts_with("https://") {
            Some("https")
        } else if url.starts_with("http://") {
            Some("http")
        } else if url.starts_with("ftp://") {
            Some("ftp")
        } else {
            None
        }
    }

    // ── URL 编码 ──

    /// 对齐 Java: `URLUtil.encode(String)`
    pub fn encode(value: &str) -> String {
        let mut result = String::with_capacity(value.len() * 3);
        for byte in value.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(byte as char);
                }
                b' ' => {
                    result.push('+');
                }
                _ => {
                    result.push('%');
                    result.push_str(&format!("{:02X}", byte));
                }
            }
        }
        result
    }

    /// 对齐 Java: `URLUtil.decode(String)`
    pub fn decode(value: &str) -> String {
        let mut result = String::with_capacity(value.len());
        let mut chars = value.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '+' => result.push(' '),
                '%' => {
                    let hex: String = chars.by_ref().take(2).collect();
                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                        result.push(byte as char);
                    } else {
                        result.push('%');
                        result.push_str(&hex);
                    }
                }
                _ => result.push(c),
            }
        }
        result
    }

    // ── URL 构建 ──

    /// 对齐 Java: `URLUtil.url(String)`
    pub fn normalize(url: &str) -> String {
        let url = url.trim();
        if url.starts_with("http://") || url.starts_with("https://") || url.starts_with("ftp://") {
            url.to_string()
        } else {
            format!("http://{}", url)
        }
    }

    /// 对齐 Java: `URLUtil.completeUrl(String, String)`
    pub fn complete_url(base: &str, path: &str) -> String {
        let base = base.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        format!("{}/{}", base, path)
    }
}
