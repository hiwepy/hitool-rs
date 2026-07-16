//! Typed JSON serialization and value utilities.
//!
//! The initial operation set and tests were adapted from yimi-rutool 0.2.5
//! (Apache-2.0), then revised around `serde_json` types and a module-specific
//! error instead of a workspace-wide error enum.

#![forbid(unsafe_code)]

use serde::de::DeserializeOwned;

pub use serde::{Deserialize, Serialize};
pub use serde_json::{Map, Value, json};

/// Result type returned by JSON operations.
pub type Result<T> = std::result::Result<T, JsonError>;

/// Errors produced by `hitool-json`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum JsonError {
    /// Serialization or parsing failed in `serde_json`.
    #[error("JSON operation failed: {0}")]
    Serde(#[from] serde_json::Error),

    /// A JSON value had a different shape than the requested operation.
    #[error("expected a JSON {expected}, found {actual}")]
    UnexpectedType {
        /// Expected JSON type.
        expected: &'static str,
        /// Actual JSON type.
        actual: &'static str,
    },
}

/// Serializes a value to compact JSON.
///
/// # Errors
///
/// Returns an error when `value` cannot be represented as JSON.
pub fn to_string<T: Serialize + ?Sized>(value: &T) -> Result<String> {
    Ok(serde_json::to_string(value)?)
}

/// Serializes a value to indented JSON.
///
/// # Errors
///
/// Returns an error when `value` cannot be represented as JSON.
pub fn to_string_pretty<T: Serialize + ?Sized>(value: &T) -> Result<String> {
    Ok(serde_json::to_string_pretty(value)?)
}

/// Deserializes JSON text into a requested type.
///
/// # Errors
///
/// Returns an error for malformed JSON or incompatible target types.
pub fn from_str<T: DeserializeOwned>(input: &str) -> Result<T> {
    Ok(serde_json::from_str(input)?)
}

/// Parses arbitrary JSON into [`Value`].
///
/// # Errors
///
/// Returns an error for malformed JSON.
pub fn parse(input: &str) -> Result<Value> {
    from_str(input)
}

/// Parses a JSON object.
///
/// # Errors
///
/// Returns an error for malformed JSON or a non-object top-level value.
pub fn parse_object(input: &str) -> Result<Map<String, Value>> {
    let value = parse(input)?;
    let actual = type_name(&value);
    value.as_object().cloned().ok_or(JsonError::UnexpectedType {
        expected: "object",
        actual,
    })
}

/// Parses a JSON array.
///
/// # Errors
///
/// Returns an error for malformed JSON or a non-array top-level value.
pub fn parse_array(input: &str) -> Result<Vec<Value>> {
    let value = parse(input)?;
    let actual = type_name(&value);
    value.as_array().cloned().ok_or(JsonError::UnexpectedType {
        expected: "array",
        actual,
    })
}

/// Returns `true` when the complete input is valid JSON.
#[must_use]
pub fn is_valid(input: &str) -> bool {
    parse(input).is_ok()
}

/// Returns `true` when the complete input is a JSON object.
#[must_use]
pub fn is_json_object(input: &str) -> bool {
    parse(input).is_ok_and(|value| value.is_object())
}

/// Returns `true` when the complete input is a JSON array.
#[must_use]
pub fn is_json_array(input: &str) -> bool {
    parse(input).is_ok_and(|value| value.is_array())
}

/// Converts JSON text to its compact representation.
///
/// # Errors
///
/// Returns an error for malformed JSON.
pub fn minify(input: &str) -> Result<String> {
    to_string(&parse(input)?)
}

/// Converts JSON text to its indented representation.
///
/// # Errors
///
/// Returns an error for malformed JSON.
pub fn pretty(input: &str) -> Result<String> {
    to_string_pretty(&parse(input)?)
}

const fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// Common imports for applications using `hitool-json`.
pub mod prelude {
    pub use crate::{Deserialize, Serialize, Value, json};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Person {
        name: String,
        age: u8,
    }

    #[test]
    fn typed_round_trip() {
        let person = Person {
            name: "Alice".to_owned(),
            age: 30,
        };
        let encoded = to_string(&person).unwrap();
        assert_eq!(from_str::<Person>(&encoded).unwrap(), person);
    }

    #[test]
    fn validation_parses_the_complete_document() {
        assert!(is_valid(r#"{"ok":true}"#));
        assert!(!is_valid(r#"{"ok":true} trailing"#));
        assert!(!is_json_object("{not-json}"));
        assert!(is_json_array("[1, 2]"));
    }

    #[test]
    fn object_and_array_operations_reject_wrong_shapes() {
        assert!(parse_object("[]").is_err());
        assert!(parse_array("{}").is_err());
        assert_eq!(parse_object(r#"{"a":1}"#).unwrap()["a"], 1);
    }

    #[test]
    fn formatting_is_reversible() {
        let compact = minify("{ \"a\": [1, 2] }").unwrap();
        assert_eq!(compact, r#"{"a":[1,2]}"#);
        assert!(pretty(&compact).unwrap().contains('\n'));
    }
}
