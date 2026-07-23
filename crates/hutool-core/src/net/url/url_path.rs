//! 对齐: `cn.hutool.core.net.url.UrlPath`
//! 来源: hutool-core/src/main/java/cn/hutool/core/net/url/UrlPath.java

/// URL path 段封装 —— 对齐 Java `UrlPath`。
#[derive(Debug, Clone, Default)]
pub struct UrlPath {
    segments: Vec<String>,
    with_end_tag: bool,
}

impl UrlPath {
    /// 对齐 Java: `UrlPath.of(CharSequence, Charset)`
    pub fn of(path: &str) -> Self {
        let mut url_path = Self::default();
        url_path.parse(path);
        url_path
    }

    /// 对齐 Java: `parse(CharSequence, Charset)`
    pub fn parse(&mut self, path: &str) {
        if path.is_empty() {
            return;
        }
        if path.ends_with('/') {
            self.with_end_tag = true;
        }
        let fixed = fix_path(path);
        if fixed.is_empty() {
            return;
        }
        for segment in fixed.split('/') {
            if !segment.is_empty() {
                self.segments.push(segment.to_string());
            }
        }
    }

    /// 对齐 Java: `getSegments()`
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// 对齐 Java: `add(CharSequence)` / `addPathSegment`
    pub fn add_segment(&mut self, segment: &str) {
        let fixed = fix_path(segment);
        if fixed.is_empty() {
            return;
        }
        for part in fixed.split('/') {
            if !part.is_empty() {
                self.segments.push(part.to_string());
            }
        }
    }

    /// 标记 path 末尾 `/`（对齐 Java `withEngTag`）。
    pub fn set_with_end_tag(&mut self, with_end_tag: bool) {
        self.with_end_tag = with_end_tag;
    }

    /// 对齐 Java: `build(Charset)`
    pub fn build(&self) -> String {
        if self.segments.is_empty() {
            return if self.with_end_tag {
                "/".to_string()
            } else {
                String::new()
            };
        }
        let mut builder = String::new();
        for segment in &self.segments {
            builder.push('/');
            builder.push_str(segment);
        }
        if self.with_end_tag && !builder.ends_with('/') {
            builder.push('/');
        }
        builder
    }
}

/// 对齐 Java: `fixPath` — 去掉首尾 `/` 与空白。
fn fix_path(path: &str) -> String {
    if path == "/" {
        return String::new();
    }
    path.trim().trim_matches('/').trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_builds_segments() {
        let path = UrlPath::of("//a/b/");
        assert_eq!(path.segments(), &["a", "b"]);
        assert_eq!(path.build(), "/a/b/");
    }
}
