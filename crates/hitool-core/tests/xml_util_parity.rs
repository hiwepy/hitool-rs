//! xml_util parity tests
//! 对齐: hutool-core XmlUtilTest

use hitool_core::XmlUtil;

// ── XML 验证 ──

#[test]
fn is_xml_valid() {
    assert!(XmlUtil::is_xml("<?xml version=\"1.0\"?>"));
    assert!(XmlUtil::is_xml("<root>content</root>"));
    assert!(XmlUtil::is_xml("<tag/>"));
}

#[test]
fn is_xml_invalid() {
    assert!(!XmlUtil::is_xml("not xml"));
    assert!(!XmlUtil::is_xml(""));
}

// ── XML 转义 ──

#[test]
fn escape_char_basic() {
    assert_eq!(XmlUtil::escape_char('&'), "&amp;");
    assert_eq!(XmlUtil::escape_char('<'), "&lt;");
    assert_eq!(XmlUtil::escape_char('>'), "&gt;");
    assert_eq!(XmlUtil::escape_char('"'), "&quot;");
    assert_eq!(XmlUtil::escape_char('\''), "&apos;");
}

#[test]
fn escape_char_normal() {
    assert_eq!(XmlUtil::escape_char('a'), "");
    assert_eq!(XmlUtil::escape_char('1'), "");
}

#[test]
fn escape_basic() {
    assert_eq!(XmlUtil::escape("<tag>value</tag>"), "&lt;tag&gt;value&lt;/tag&gt;");
}

#[test]
fn escape_special_chars() {
    assert_eq!(XmlUtil::escape("a & b"), "a &amp; b");
    assert_eq!(XmlUtil::escape("\"hello\""), "&quot;hello&quot;");
    assert_eq!(XmlUtil::escape("it's"), "it&apos;s");
}

#[test]
fn unescape_basic() {
    assert_eq!(XmlUtil::unescape("&lt;tag&gt;value&lt;/tag&gt;"), "<tag>value</tag>");
}

#[test]
fn unescape_special_chars() {
    assert_eq!(XmlUtil::unescape("a &amp; b"), "a & b");
    assert_eq!(XmlUtil::unescape("&quot;hello&quot;"), "\"hello\"");
    assert_eq!(XmlUtil::unescape("it&apos;s"), "it's");
}

#[test]
fn escape_unescape_roundtrip() {
    let inputs = vec![
        "<tag>value</tag>",
        "a & b",
        "\"hello\"",
        "it's",
        "<script>alert('xss')</script>",
    ];
    for input in inputs {
        let escaped = XmlUtil::escape(input);
        let unescaped = XmlUtil::unescape(&escaped);
        assert_eq!(unescaped, input, "roundtrip failed for {:?}", input);
    }
}

// ── XML 构建 ──

#[test]
fn element_basic() {
    assert_eq!(XmlUtil::element("name", "Alice"), "<name>Alice</name>");
}

#[test]
fn element_with_special_chars() {
    assert_eq!(XmlUtil::element("tag", "<value>"), "<tag>&lt;value&gt;</tag>");
}

#[test]
fn element_with_attrs() {
    let result = XmlUtil::element_with_attrs("tag", &[("id", "1"), ("class", "test")], "value");
    assert_eq!(result, "<tag id=\"1\" class=\"test\">value</tag>");
}

#[test]
fn self_closing_element_basic() {
    assert_eq!(XmlUtil::self_closing_element("br"), "<br />");
}

#[test]
fn self_closing_element_with_attrs() {
    let result = XmlUtil::self_closing_element_with_attrs("img", &[("src", "test.png"), ("alt", "test")]);
    assert_eq!(result, "<img src=\"test.png\" alt=\"test\" />");
}

// ── XML 声明 ──

#[test]
fn xml_header_utf8() {
    assert_eq!(XmlUtil::xml_header_utf8(), "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
}

#[test]
fn xml_header_custom() {
    assert_eq!(XmlUtil::xml_header("ISO-8859-1"), "<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>");
}

// ── CDATA ──

#[test]
fn wrap_cdata_basic() {
    assert_eq!(XmlUtil::wrap_cdata("hello"), "<![CDATA[hello]]>");
}

#[test]
fn wrap_cdata_with_special_chars() {
    assert_eq!(XmlUtil::wrap_cdata("<tag>&amp;</tag>"), "<![CDATA[<tag>&amp;</tag>]]>");
}

// ── XML 解析辅助 ──

#[test]
fn get_tag_content_basic() {
    let xml = "<root><name>Alice</name><age>30</age></root>";
    assert_eq!(XmlUtil::get_tag_content(xml, "name"), Some("Alice".to_string()));
    assert_eq!(XmlUtil::get_tag_content(xml, "age"), Some("30".to_string()));
}

#[test]
fn get_tag_content_missing() {
    let xml = "<root><name>Alice</name></root>";
    assert_eq!(XmlUtil::get_tag_content(xml, "age"), None);
}

#[test]
fn get_attribute_basic() {
    let xml = "<tag id=\"1\" class=\"test\">value</tag>";
    assert_eq!(XmlUtil::get_attribute(xml, "id"), Some("1".to_string()));
    assert_eq!(XmlUtil::get_attribute(xml, "class"), Some("test".to_string()));
}

#[test]
fn get_attribute_missing() {
    let xml = "<tag id=\"1\">value</tag>";
    assert_eq!(XmlUtil::get_attribute(xml, "class"), None);
}
