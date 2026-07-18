//! escape_util parity tests
//! 对齐: hutool-core EscapeUtilTest

use hitool_core::EscapeUtil;

// ── HTML 转义 ──

#[test]
fn escape_html_basic() {
    assert_eq!(EscapeUtil::escape_html("<div>hello</div>"), "&lt;div&gt;hello&lt;/div&gt;");
}

#[test]
fn escape_html_special_chars() {
    assert_eq!(EscapeUtil::escape_html("a & b"), "a &amp; b");
    assert_eq!(EscapeUtil::escape_html("\"hello\""), "&quot;hello&quot;");
    assert_eq!(EscapeUtil::escape_html("it's"), "it&#39;s");
}

#[test]
fn escape_html_no_special() {
    assert_eq!(EscapeUtil::escape_html("hello world"), "hello world");
}

#[test]
fn unescape_html_basic() {
    assert_eq!(EscapeUtil::unescape_html("&lt;div&gt;hello&lt;/div&gt;"), "<div>hello</div>");
}

#[test]
fn unescape_html_special_chars() {
    assert_eq!(EscapeUtil::unescape_html("a &amp; b"), "a & b");
    assert_eq!(EscapeUtil::unescape_html("&quot;hello&quot;"), "\"hello\"");
    assert_eq!(EscapeUtil::unescape_html("it&#39;s"), "it's");
}

#[test]
fn escape_unescape_html_roundtrip() {
    let input = "<div class=\"test\">Hello & World</div>";
    let escaped = EscapeUtil::escape_html(input);
    let unescaped = EscapeUtil::unescape_html(&escaped);
    assert_eq!(unescaped, input);
}

// ── XML 转义 ──

#[test]
fn escape_xml_basic() {
    assert_eq!(EscapeUtil::escape_xml("<tag>value</tag>"), "&lt;tag&gt;value&lt;/tag&gt;");
}

#[test]
fn unescape_xml_basic() {
    assert_eq!(EscapeUtil::unescape_xml("&lt;tag&gt;value&lt;/tag&gt;"), "<tag>value</tag>");
}

// ── Java/JavaScript 转义 ──

#[test]
fn escape_java_newline() {
    assert_eq!(EscapeUtil::escape_java("line1\nline2"), "line1\\nline2");
}

#[test]
fn escape_java_tab() {
    assert_eq!(EscapeUtil::escape_java("col1\tcol2"), "col1\\tcol2");
}

#[test]
fn escape_java_quotes() {
    assert_eq!(EscapeUtil::escape_java("say \"hello\""), "say \\\"hello\\\"");
}

#[test]
fn escape_java_backslash() {
    assert_eq!(EscapeUtil::escape_java("path\\to\\file"), "path\\\\to\\\\file");
}

#[test]
fn unescape_java_newline() {
    assert_eq!(EscapeUtil::unescape_java("line1\\nline2"), "line1\nline2");
}

#[test]
fn unescape_java_tab() {
    assert_eq!(EscapeUtil::unescape_java("col1\\tcol2"), "col1\tcol2");
}

#[test]
fn unescape_java_quotes() {
    assert_eq!(EscapeUtil::unescape_java("say \\\"hello\\\""), "say \"hello\"");
}

#[test]
fn escape_unescape_java_roundtrip() {
    let input = "line1\nline2\t\"hello\"\tpath\\to";
    let escaped = EscapeUtil::escape_java(input);
    let unescaped = EscapeUtil::unescape_java(&escaped);
    assert_eq!(unescaped, input);
}

// ── SQL 转义 ──

#[test]
fn escape_sql_basic() {
    assert_eq!(EscapeUtil::escape_sql("it's"), "it''s");
}

#[test]
fn escape_sql_no_quotes() {
    assert_eq!(EscapeUtil::escape_sql("hello"), "hello");
}

#[test]
fn escape_sql_multiple_quotes() {
    assert_eq!(EscapeUtil::escape_sql("it's a \"test\""), "it''s a \"test\"");
}

// ── 通用转义 ──

#[test]
fn escape_basic() {
    assert_eq!(EscapeUtil::escape("<script>alert('xss')</script>"), "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");
}

#[test]
fn unescape_basic() {
    assert_eq!(EscapeUtil::unescape("&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"), "<script>alert('xss')</script>");
}
