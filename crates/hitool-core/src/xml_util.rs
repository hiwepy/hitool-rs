//! 对齐: `cn.hutool.core.util.XmlUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/XmlUtil.java
//!
//! Rust 版本提供 XML 操作的 idiomatic 实现。

/// 对齐 Java: `cn.hutool.core.util.XmlUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct XmlUtil;

impl XmlUtil {
    // ── XML 验证 ──

    /// 对齐 Java: `XmlUtil.isXml(String)`
    pub fn is_xml(value: &str) -> bool {
        let trimmed = value.trim();
        trimmed.starts_with("<?xml") || trimmed.starts_with("<")
    }

    // ── XML 转义 ──

    /// 对齐 Java: `XmlUtil.escape(char)`
    pub fn escape_char(c: char) -> &'static str {
        match c {
            '&' => "&amp;",
            '<' => "&lt;",
            '>' => "&gt;",
            '"' => "&quot;",
            '\'' => "&apos;",
            _ => "",
        }
    }

    /// 对齐 Java: `XmlUtil.escape(String)`
    pub fn escape(value: &str) -> String {
        let mut result = String::with_capacity(value.len());
        for c in value.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                '\'' => result.push_str("&apos;"),
                _ => result.push(c),
            }
        }
        result
    }

    /// 对齐 Java: `XmlUtil.unescape(String)`
    pub fn unescape(value: &str) -> String {
        value
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&apos;", "'")
    }

    // ── XML 构建 ──

    /// 构建简单的 XML 元素
    pub fn element(tag: &str, content: &str) -> String {
        format!("<{}>{}</{}>", tag, Self::escape(content), tag)
    }

    /// 构建带属性的 XML 元素
    pub fn element_with_attrs(tag: &str, attrs: &[(&str, &str)], content: &str) -> String {
        let attr_str: String = attrs
            .iter()
            .map(|(k, v)| format!(" {}=\"{}\"", k, Self::escape(v)))
            .collect();
        format!("<{}{}>{}</{}>", tag, attr_str, Self::escape(content), tag)
    }

    /// 构建自闭合 XML 元素
    pub fn self_closing_element(tag: &str) -> String {
        format!("<{} />", tag)
    }

    /// 构建带属性的自闭合 XML 元素
    pub fn self_closing_element_with_attrs(tag: &str, attrs: &[(&str, &str)]) -> String {
        let attr_str: String = attrs
            .iter()
            .map(|(k, v)| format!(" {}=\"{}\"", k, Self::escape(v)))
            .collect();
        format!("<{}{} />", tag, attr_str)
    }

    // ── XML 声明 ──

    /// 对齐 Java: `XmlUtil.xmlHeader(String)`
    pub fn xml_header(encoding: &str) -> String {
        format!("<?xml version=\"1.0\" encoding=\"{}\"?>", encoding)
    }

    /// 默认 UTF-8 编码的 XML 声明
    pub fn xml_header_utf8() -> String {
        Self::xml_header("UTF-8")
    }

    // ── CDATA ──

    /// 对齐 Java: `XmlUtil.wrapCDATA(String)`
    pub fn wrap_cdata(content: &str) -> String {
        format!("<![CDATA[{}]]>", content)
    }

    // ── XML 解析辅助 ──

    /// 提取 XML 标签内容
    pub fn get_tag_content(xml: &str, tag: &str) -> Option<String> {
        let start_tag = format!("<{}>", tag);
        let end_tag = format!("</{}>", tag);
        if let Some(start_pos) = xml.find(&start_tag) {
            let content_start = start_pos + start_tag.len();
            if let Some(end_pos) = xml[content_start..].find(&end_tag) {
                return Some(xml[content_start..content_start + end_pos].to_string());
            }
        }
        None
    }

    /// 提取 XML 属性值
    pub fn get_attribute(xml: &str, attr_name: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr_name);
        if let Some(start_pos) = xml.find(&pattern) {
            let value_start = start_pos + pattern.len();
            if let Some(end_pos) = xml[value_start..].find('"') {
                return Some(xml[value_start..value_start + end_pos].to_string());
            }
        }
        None
    }
}
