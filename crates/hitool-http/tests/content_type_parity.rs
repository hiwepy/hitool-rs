//! ContentType / Rest content-type / HTTP status metadata parity
//! 对齐: `cn.hutool.http.ContentTypeTest`

use hitool_http::{ContentType, Header, Method};

/// 对齐 Java: `ContentTypeTest.testBuild()`
#[test]
fn test_build() {
    let result = ContentType::build_type(ContentType::Json, "UTF-8");
    assert_eq!(result, "application/json;charset=UTF-8");
}

/// 对齐 Java: `ContentTypeTest.testGetWithLeadingSpace()`
#[test]
fn test_get_with_leading_space() {
    let json = " {\n     \"name\": \"hutool\"\n }";
    assert_eq!(ContentType::detect(json), Some(ContentType::Json));
}

/// 对齐 Java: `RestTest.contentTypeTest()`
#[test]
fn rest_content_type_test() {
    // Hutool HttpRequest.post(...).body(json) sets Content-Type via ContentType.JSON + charset
    let ct = ContentType::build_type(ContentType::Json, "UTF-8");
    assert_eq!(ct, "application/json;charset=UTF-8");
    assert_eq!(Header::ContentType.value(), "Content-Type");
}

/// 对齐 Java: `HttpGlobalConfigTest.allowPatchTest()`
#[test]
fn allow_patch_test() {
    // Hutool enables PATCH on the global client; hitool exposes Method::PATCH
    assert_eq!(Method::PATCH.as_str(), "PATCH");
}
