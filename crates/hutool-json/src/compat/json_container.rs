use std::{fmt, ops::Index};

use serde::Serialize;
use serde_json::{Map, Number, Value};

use crate::{JsonError, Result};

use super::json_config::JSONConfig;

/// Common behavior shared by JSON object and array wrappers.
pub trait JsonContainer: Clone + fmt::Display {
    /// Returns the container configuration.
    fn config(&self) -> &JSONConfig;
    /// Returns an owned dynamic JSON representation.
    fn to_value(&self) -> Value;
    /// Serializes the container with an optional indentation width.
    fn to_json_string(&self, indent: usize) -> Result<String> {
        if indent == 0 {
            crate::to_string(&self.to_value())
        } else {
            crate::to_string_pretty(&self.to_value())
        }
    }
}

const fn value_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
