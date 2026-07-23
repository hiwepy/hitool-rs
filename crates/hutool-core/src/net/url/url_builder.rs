//! 对齐: `cn.hutool.core.net.url.UrlBuilder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/net/url/UrlBuilder.java

use crate::net::rfc3986::Rfc3986;
use crate::net::url::url_path::UrlPath;

/// 对齐 Java 类: `cn.hutool.core.net.url.UrlBuilder`
#[derive(Debug, Clone, Default)]
pub struct UrlBuilder {
    path: UrlPath,
}

impl UrlBuilder {
    /// 对齐 Java: `UrlBuilder.of()` — 创建空构建器。
    pub fn of() -> Self {
        Self::default()
    }

    /// 对齐 Java: `UrlBuilder.addPath(CharSequence)` — 按段追加 path，空/`/`/`//` 不 panic。
    pub fn add_path(&mut self, path: &str) -> &mut Self {
        if path.is_empty() {
            return self;
        }
        if fix_path_only(path).is_empty() && path.contains('/') {
            self.path.set_with_end_tag(true);
        }
        let parsed = UrlPath::of(path);
        for segment in parsed.segments() {
            self.path.add_segment(segment);
        }
        self
    }

    /// 返回当前 path（对齐 `UrlPath.build()`）。
    pub fn build_path(&self) -> String {
        self.path.build()
    }

    /// 对齐 Java: `UrlQuery.build` 默认 query 值编码（RFC3986.QUERY_PARAM_VALUE，保留 `:`）。
    pub fn encode_query_param_value(value: &str) -> String {
        Rfc3986::encode_query_param_value(value)
    }

    /// 对齐 Java: `UrlQuery.build` 默认 query 名编码（RFC3986.QUERY_PARAM_NAME）。
    pub fn encode_query_param_name(name: &str) -> String {
        Rfc3986::encode_query_param_name(name)
    }
}

fn fix_path_only(path: &str) -> String {
    if path == "/" {
        return String::new();
    }
    path.trim().trim_matches('/').trim().to_string()
}
