//! Hutool-style migration facade.
//!
//! New Rust code should prefer `hitool-core` extension traits and the free
//! functions in `hitool-json`. These utility structs exist to make incremental
//! migration from Java/Hutool terminology straightforward.

#![forbid(unsafe_code)]

use std::fmt::Display;

use serde::Serialize;
use serde::de::DeserializeOwned;

/// Hutool-style string utility facade.
pub struct StrUtil;

impl StrUtil {
    /// Returns `true` when `value` is empty or only Unicode whitespace.
    #[inline]
    #[must_use]
    pub fn is_blank(value: &str) -> bool {
        hitool_core::is_blank(value)
    }

    /// Returns `true` when `value` contains a non-whitespace character.
    #[inline]
    #[must_use]
    pub fn is_not_blank(value: &str) -> bool {
        !Self::is_blank(value)
    }

    /// Returns `true` when `value` has zero bytes.
    #[inline]
    #[must_use]
    pub const fn is_empty(value: &str) -> bool {
        value.is_empty()
    }

    /// Removes all occurrences of `needle`.
    #[must_use]
    pub fn remove_all(value: &str, needle: &str) -> String {
        hitool_core::remove_all(value, needle)
    }

    /// Formats sequential `{}` placeholders.
    #[must_use]
    pub fn format(template: &str, values: &[&dyn Display]) -> String {
        hitool_core::format_template(template, values)
    }

    /// Uppercases the first Unicode character.
    #[must_use]
    pub fn upper_first(value: &str) -> String {
        hitool_core::upper_first(value)
    }

    /// Lowercases the first Unicode character.
    #[must_use]
    pub fn lower_first(value: &str) -> String {
        hitool_core::lower_first(value)
    }
}

/// Hutool-style JSON utility facade.
pub struct JsonUtil;

impl JsonUtil {
    /// Serializes a value to compact JSON.
    ///
    /// # Errors
    ///
    /// Returns an error when serialization fails.
    pub fn to_string<T: Serialize + ?Sized>(value: &T) -> hitool_json::Result<String> {
        hitool_json::to_string(value)
    }

    /// Deserializes JSON into a requested type.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed JSON or incompatible target types.
    pub fn to_bean<T: DeserializeOwned>(input: &str) -> hitool_json::Result<T> {
        hitool_json::from_str(input)
    }

    /// Parses JSON into a dynamic value.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed JSON.
    pub fn parse(input: &str) -> hitool_json::Result<serde_json::Value> {
        hitool_json::parse(input)
    }

    /// Returns `true` when the complete input is valid JSON.
    #[must_use]
    pub fn is_valid(input: &str) -> bool {
        hitool_json::is_valid(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatibility_facades_delegate_to_native_crates() {
        assert!(StrUtil::is_blank(" \t"));
        assert_eq!(StrUtil::format("{} {}", &[&"Hi", &"Tool"]), "Hi Tool");

        let value = JsonUtil::parse(r#"{"ok":true}"#).unwrap();
        assert_eq!(value["ok"], true);
    }
}
