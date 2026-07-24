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
