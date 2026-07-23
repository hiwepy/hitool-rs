//! 对齐: `cn.hutool.core.net.RFC3986`
//! 来源: hutool-core/src/main/java/cn/hutool/core/net/RFC3986.java

use encoding_rs::UTF_8;

use crate::hutool_codec::PercentCodec;

/// 对齐 Java 类: `cn.hutool.core.net.RFC3986`
#[derive(Debug, Clone, Copy, Default)]
pub struct Rfc3986;

impl Rfc3986 {
    /// 对齐 Java: `RFC3986.UNRESERVED.encode(...)` / `URLEncodeUtil.encodeAll(...)`
    pub fn encode_all(input: &str) -> String {
        unreserved_codec().encode(input, UTF_8, &[])
    }

    /// 对齐 Java: `RFC3986.PATH.encode(...)` / `URLEncodeUtil.encode(...)`
    pub fn encode_path(input: &str) -> String {
        path_codec().encode(input, UTF_8, &[])
    }

    /// 对齐 Java: `RFC3986.QUERY.encode(...)` / `URLEncodeUtil.encodeQuery(...)`
    pub fn encode_query(input: &str) -> String {
        query_codec().encode(input, UTF_8, &[])
    }

    /// 对齐 Java: `RFC3986.SEGMENT.encode(...)` / `URLEncodeUtil.encodePathSegment(...)`
    pub fn encode_path_segment(input: &str) -> String {
        if input.is_empty() {
            return input.to_string();
        }
        pchar_codec().encode(input, UTF_8, &[])
    }

    /// 对齐 Java: `RFC3986.FRAGMENT.encode(...)` / `URLEncodeUtil.encodeFragment(...)`
    pub fn encode_fragment(input: &str) -> String {
        if input.is_empty() {
            return input.to_string();
        }
        fragment_codec().encode(input, UTF_8, &[])
    }

    /// 对齐 Java: `RFC3986.QUERY_PARAM_VALUE.encode(...)`
    pub fn encode_query_param_value(input: &str) -> String {
        query_param_value_codec().encode(input, UTF_8, &[])
    }

    /// 对齐 Java: `RFC3986.QUERY_PARAM_NAME.encode(...)`
    pub fn encode_query_param_name(input: &str) -> String {
        query_param_name_codec().encode(input, UTF_8, &[])
    }
}

fn unreserved_codec() -> PercentCodec {
    let mut codec = PercentCodec::new();
    for ch in 'A'..='Z' {
        codec.add_safe(ch);
    }
    for ch in 'a'..='z' {
        codec.add_safe(ch);
    }
    for ch in '0'..='9' {
        codec.add_safe(ch);
    }
    for ch in ['_', '.', '-', '~'] {
        codec.add_safe(ch);
    }
    codec
}

fn sub_delims_codec() -> PercentCodec {
    PercentCodec::with_safe("!$&'()*+,;=".chars())
}

fn pchar_codec() -> PercentCodec {
    unreserved_codec()
        .union_new(&sub_delims_codec())
        .union_new(&PercentCodec::with_safe(":@".chars()))
}

fn path_codec() -> PercentCodec {
    pchar_codec().union_new(&PercentCodec::with_safe("/".chars()))
}

fn query_codec() -> PercentCodec {
    pchar_codec().union_new(&PercentCodec::with_safe("/?+".chars()))
}

fn fragment_codec() -> PercentCodec {
    query_codec()
}

fn query_param_value_codec() -> PercentCodec {
    let mut codec = query_codec();
    codec.remove_safe('&');
    codec
}

fn query_param_name_codec() -> PercentCodec {
    let mut codec = query_param_value_codec();
    codec.remove_safe('=');
    codec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_param_value_keeps_colon() {
        assert_eq!(
            Rfc3986::encode_query_param_value("2022-03-31 00:00:00"),
            "2022-03-31%2000:00:00"
        );
    }
}
