//! 对齐: `cn.hutool.core.util.URLUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/URLUtil.java
//!
//! Rust 版本提供 URL 操作的 idiomatic 实现。

use crate::net::rfc3986::Rfc3986;
use crate::net::url_decoder::UrlDecoder;
use crate::string::{is_blank, trim};
use crate::{CoreError, Result};

/// 对齐 Java `java.net.URI` 的轻量封装,用于 `URLUtil::to_uri` 返回值。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitUri {
    raw: String,
}

impl HitUri {
    /// 返回 URI 原始字符串。
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// 对齐 Java: `URI.getPath()`
    #[must_use]
    pub fn path(&self) -> Option<&str> {
        extract_path(&self.raw)
    }

    /// 对齐 Java: `URI.resolve(String)`
    #[must_use]
    pub fn resolve(&self, other: &str) -> Self {
        let base_path = self.path().unwrap_or("");
        let resolved = resolve_path(base_path, other);
        Self {
            raw: if resolved.starts_with('/') {
                resolved
            } else {
                format!("/{resolved}")
            },
        }
    }
}

impl std::fmt::Display for HitUri {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.raw)
    }
}

/// 对齐 Java: `cn.hutool.core.util.URLUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct UrlUtil;

impl UrlUtil {
    // ── URL 验证 ──

    /// 对齐 Java: `URLUtil.isUrl(CharSequence)`
    pub fn is_url(value: &str) -> bool {
        value.starts_with("http://")
            || value.starts_with("https://")
            || value.starts_with("ftp://")
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

    /// 对齐 Java: `URLUtil.getHost(URL)` 的字符串简化版。
    pub fn get_host(url: &str) -> Option<&str> {
        let url = url
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .trim_start_matches("ftp://");
        url.split('/').next().and_then(|host| {
            if host.is_empty() {
                None
            } else {
                Some(host)
            }
        })
    }

    /// 对齐 Java: `URLUtil.getPath(String uriStr)`
    pub fn get_path(uri_str: &str) -> String {
        Self::to_uri(uri_str)
            .ok()
            .and_then(|uri| uri.path().map(str::to_owned))
            .filter(|path| !path.is_empty())
            .unwrap_or_else(|| "/".to_string())
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

    /// 对齐 Java: `URLUtil.encode(String)` → `RFC3986.PATH`
    pub fn encode(value: &str) -> String {
        Rfc3986::encode_path(value)
    }

    /// 对齐 Java: `URLUtil.encodeQuery(String)` → `RFC3986.QUERY`
    pub fn encode_query(value: &str) -> String {
        Rfc3986::encode_query(value)
    }

    /// 对齐 Java: `URLUtil.decode(String)`
    pub fn decode(value: &str) -> String {
        UrlDecoder::decode(value)
    }

    // ── URI 转换 ──

    /// 对齐 Java: `URLUtil.toURI(String location)`
    pub fn to_uri(location: &str) -> Result<HitUri> {
        Self::to_uri_with_encode(location, false)
    }

    /// 对齐 Java: `URLUtil.toURI(String location, boolean isEncode)`
    pub fn to_uri_with_encode(location: &str, is_encode: bool) -> Result<HitUri> {
        let mut normalized = trim(location).to_string();
        if is_encode {
            normalized = Self::encode(&normalized);
        }
        validate_uri(&normalized)?;
        Ok(HitUri { raw: normalized })
    }

    // ── URL 构建 ──

    /// 对齐 Java: `URLUtil.normalize(String)`
    pub fn normalize(url: &str) -> String {
        Self::normalize_with_encode_path(url, false)
    }

    /// 对齐 Java: `URLUtil.normalize(String, boolean isEncodePath)`
    pub fn normalize_with_encode_path(url: &str, is_encode_path: bool) -> String {
        Self::normalize_full(url, is_encode_path, false)
    }

    /// 对齐 Java: `URLUtil.normalize(String, boolean isEncodePath, boolean replaceSlash)`
    pub fn normalize_full(url: &str, is_encode_path: bool, replace_slash: bool) -> String {
        if is_blank(url) {
            return url.to_string();
        }

        let (protocol, mut body) = split_protocol(url);

        let (params, body_without_params) = split_query(body.as_str());
        body = body_without_params;

        if !body.is_empty() {
            body = trim_leading_slashes(&body).to_string();
            body = body.replace('\\', "/");
            if replace_slash {
                body = collapse_slashes(&body);
            }
        }

        let (domain, path) = split_domain_and_path(&body);
        let encoded_path = if is_encode_path {
            path.as_ref().map(|segment| Self::encode(segment))
        } else {
            path
        };

        format!(
            "{protocol}{domain}{}{}",
            encoded_path.unwrap_or_default(),
            params.unwrap_or_default()
        )
    }

    /// 对齐 Java: `URLUtil.completeUrl(String, String)`
    pub fn complete_url(base: &str, path: &str) -> String {
        let base = base.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        format!("{base}/{path}")
    }

    /// 对齐 Java: `URLUtil.encodeBlank(String)` — 空白转 `%20`。
    pub fn encode_blank(value: &str) -> String {
        value.replace(' ', "%20")
    }

    /// 对齐 Java: `URLUtil.buildQuery(Map)` — `k=v&` 拼接（值已编码）。
    pub fn build_query(params: &[(&str, &str)]) -> String {
        params
            .iter()
            .map(|(k, v)| format!("{k}={}", Self::encode_query(v)))
            .collect::<Vec<_>>()
            .join("&")
    }

    /// 对齐 Java: `URLUtil.isFileURL(URL)` — `file:` scheme。
    pub fn is_file_url(url: &str) -> bool {
        url.trim().to_ascii_lowercase().starts_with("file:")
    }

    /// 对齐 Java: `URLUtil.isJarURL(URL)`
    pub fn is_jar_url(url: &str) -> bool {
        let lower = url.trim().to_ascii_lowercase();
        lower.starts_with("jar:") || lower.contains(".jar!")
    }

    /// 对齐 Java: `URLUtil.isJarFileURL(URL)`
    pub fn is_jar_file_url(url: &str) -> bool {
        Self::is_file_url(url) && url.to_ascii_lowercase().contains(".jar")
    }

    /// 对齐 Java: `URLUtil.getDecodedPath(URL)`
    pub fn get_decoded_path(url: &str) -> String {
        Self::decode(&Self::get_path(url))
    }

    /// 对齐 Java: `URLUtil.getDataUri` / `getDataUriBase64`
    pub fn get_data_uri(mime: &str, base64: &str) -> String {
        format!("data:{mime};base64,{base64}")
    }

    /// 对齐 Java: `URLUtil.getStringURI(String)` — 规范化字符串 URI。
    pub fn get_string_uri(location: &str) -> String {
        Self::to_uri(location)
            .map(|u| u.as_str().to_string())
            .unwrap_or_else(|_| trim(location).to_string())
    }

    /// 对齐 Java: `URLUtil.url(String)` / `toUrlForHttp` — HTTP(S) 规范化。
    pub fn url(location: &str) -> String {
        let trimmed = trim(location);
        if Self::is_url(trimmed) {
            Self::normalize(trimmed)
        } else if trimmed.starts_with("//") {
            format!("http:{}", trimmed)
        } else {
            Self::normalize(trimmed)
        }
    }

    /// 对齐 Java: `URLUtil.toUrlForHttp(String)`
    pub fn to_url_for_http(location: &str) -> String {
        Self::url(location)
    }
}

/// 校验 URI 字符串语法,对齐 Java `new URI(...)` 抛出的 `URISyntaxException`。
fn validate_uri(location: &str) -> Result<()> {
    if location.is_empty() {
        return Ok(());
    }
    if location.contains(char::is_whitespace) {
        return Err(CoreError::Codec(format!("invalid URI: {location}")));
    }
    Ok(())
}

/// 拆分 scheme 与 body,无 scheme 时默认 `http://`。
fn split_protocol(url: &str) -> (String, String) {
    if let Some(sep_index) = url.find("://") {
        if sep_index > 0 {
            let protocol = url[..sep_index + 3].to_string();
            let body = url[sep_index + 3..].to_string();
            return (protocol, body);
        }
    }
    ("http://".to_string(), url.to_string())
}

/// 拆分 query 参数段。
fn split_query(body: &str) -> (Option<String>, String) {
    if let Some(index) = body.find('?') {
        if index > 0 {
            let params = body[index..].to_string();
            let without_params = body[..index].to_string();
            return (Some(params), without_params);
        }
    }
    (None, body.to_string())
}

/// 去除 body 开头连续的 `\` 或 `/`。
fn trim_leading_slashes(body: &str) -> &str {
    body.trim_start_matches(['\\', '/'])
}

/// 将连续 `/` 折叠为单个 `/`。
fn collapse_slashes(body: &str) -> String {
    let mut output = String::with_capacity(body.len());
    let mut previous_slash = false;
    for ch in body.chars() {
        if ch == '/' {
            if !previous_slash {
                output.push(ch);
            }
            previous_slash = true;
        } else {
            previous_slash = false;
            output.push(ch);
        }
    }
    output
}

/// 拆分域名与 path,`pathSepIndex > 0` 与 Hutool 一致。
fn split_domain_and_path(body: &str) -> (String, Option<String>) {
    if let Some(index) = body.find('/') {
        if index > 0 {
            let domain = body[..index].to_string();
            let path = body[index..].to_string();
            return (domain, Some(path));
        }
    }
    (body.to_string(), None)
}

/// 从 URI 字符串提取 path 部分。
fn extract_path(raw: &str) -> Option<&str> {
    let trimmed = trim(raw);
    let path_start = if let Some(index) = trimmed.find("://") {
        let rest = &trimmed[index + 3..];
        rest.find('/').map(|offset| index + 3 + offset)
    } else if trimmed.starts_with('/') {
        Some(0)
    } else {
        None
    }?;

    let suffix = &trimmed[path_start..];
    let end = suffix
        .find(['?', '#'])
        .map_or(suffix.len(), |offset| offset);
    Some(&suffix[..end])
}

/// 对齐 Java `URI.resolve` 的 path 语义(仅覆盖 parity 所需场景)。
fn resolve_path(base_path: &str, other: &str) -> String {
    if other == "." {
        let mut path = base_path.to_string();
        if let Some(index) = path.rfind('/') {
            path.truncate(index + 1);
        } else {
            path.clear();
        }
        return path;
    }
    if other.starts_with('/') {
        return other.to_string();
    }
    let mut path = base_path.to_string();
    if !path.ends_with('/') {
        if let Some(index) = path.rfind('/') {
            path.truncate(index + 1);
        }
    }
    path.push_str(other);
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_matches_hutool() {
        assert_eq!(
            UrlUtil::normalize("http://www.hutool.cn//aaa/bbb"),
            "http://www.hutool.cn//aaa/bbb"
        );
        assert_eq!(
            UrlUtil::normalize("www.hutool.cn//aaa/bbb"),
            "http://www.hutool.cn//aaa/bbb"
        );
        assert_eq!(
            UrlUtil::normalize("http://www.hutool.cn//aaa/\\bbb?a=1&b=2"),
            "http://www.hutool.cn//aaa//bbb?a=1&b=2"
        );
    }

    #[test]
    fn encode_path_uses_percent_twenty() {
        assert_eq!(
            UrlUtil::encode("366466 - 副本.jpg"),
            "366466%20-%20%E5%89%AF%E6%9C%AC.jpg"
        );
    }

    #[test]
    fn get_path_via_to_uri() {
        assert_eq!(
            UrlUtil::get_path(" http://www.aaa.bbb/search?scope=ccc&q=ddd"),
            "/search"
        );
    }
}
