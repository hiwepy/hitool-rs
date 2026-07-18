//! charset_util module parity tests
//! 对齐: hutool-core CharsetUtilTest

use hitool_core::{Charset, CharsetUtil};

// ── Charset constants ──

#[test]
fn charset_utf8() {
    let cs = Charset::UTF_8;
    assert_eq!(cs.name(), "UTF-8");
}

#[test]
fn charset_gbk() {
    let cs = Charset::GBK;
    assert_eq!(cs.name(), "GBK");
}

#[test]
fn charset_iso_8859_1() {
    let cs = Charset::ISO_8859_1;
    assert_eq!(cs.name(), "ISO-8859-1");
}

#[test]
fn charset_utf16() {
    let cs = Charset::UTF_16;
    assert_eq!(cs.name(), "UTF-16");
}

// ── CharsetUtil::charset ──

#[test]
fn charset_util_parse_utf8() {
    let cs = CharsetUtil::charset(Some("UTF-8")).unwrap();
    assert_eq!(cs.name(), "UTF-8");
}

#[test]
fn charset_util_parse_gbk() {
    let cs = CharsetUtil::charset(Some("GBK")).unwrap();
    assert_eq!(cs.name(), "GBK");
}

#[test]
fn charset_util_parse_none() {
    let cs = CharsetUtil::charset(None).unwrap();
    assert_eq!(cs.name(), "UTF-8");
}

// ── CharsetUtil::parse ──

#[test]
fn charset_util_parse_with_default() {
    let cs = CharsetUtil::parse(Some("UTF-8"), Charset::GBK);
    assert_eq!(cs.name(), "UTF-8");
}

#[test]
fn charset_util_parse_with_default_fallback() {
    let cs = CharsetUtil::parse(None, Charset::GBK);
    assert_eq!(cs.name(), "GBK");
}

// ── CharsetUtil::parse_default ──

#[test]
fn charset_util_parse_default_utf8() {
    let cs = CharsetUtil::parse_default(Some("utf-8"));
    assert_eq!(cs.name(), "UTF-8");
}

#[test]
fn charset_util_parse_default_unknown() {
    let cs = CharsetUtil::parse_default(Some("unknown"));
    assert_eq!(cs.name(), "UTF-8");
}

#[test]
fn charset_util_parse_default_none() {
    let cs = CharsetUtil::parse_default(None);
    assert_eq!(cs.name(), "UTF-8");
}
