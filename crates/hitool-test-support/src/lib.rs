//! Fixtures and support utilities shared by `HiTool` tests.

#![forbid(unsafe_code)]

pub use tempfile::{NamedTempFile, TempDir, tempdir};

/// Parses JSON and compares semantic values, ignoring object key order and
/// insignificant whitespace.
///
/// # Panics
///
/// Panics with the relevant parser error when either input is invalid JSON,
/// or with both normalized values when they differ.
pub fn assert_json_eq(actual: &str, expected: &str) {
    let actual: serde_json::Value =
        serde_json::from_str(actual).expect("actual value must be valid JSON");
    let expected: serde_json::Value =
        serde_json::from_str(expected).expect("expected value must be valid JSON");
    assert_eq!(actual, expected, "JSON values differ");
}

/// Returns a temporary directory that is deleted when dropped.
///
/// # Panics
///
/// Panics when the operating system cannot create the directory.
#[must_use]
pub fn temp_workspace() -> TempDir {
    tempfile::Builder::new()
        .prefix("hitool-test-")
        .tempdir()
        .expect("temporary test workspace must be creatable")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers_compare_json_and_create_isolated_workspace() {
        assert_json_eq(r#"{"a":1,"b":2}"#, r#"{ "b": 2, "a": 1 }"#);
        assert!(temp_workspace().path().exists());
    }
}
