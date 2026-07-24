use std::{fmt, ops::Index};

use serde::Serialize;
use serde_json::{Map, Number, Value};

use crate::{JsonError, Result};

use super::json_config::JSONConfig;
use super::json_container::JsonContainer;
use super::path_error::PathError;

/// Mutable, configured JSON array compatible with Hutool's `JSONArray`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JSONArray {
    values: Vec<Value>,
    config: JSONConfig,
}

impl Default for JSONArray {
    fn default() -> Self {
        Self::new()
    }
}

impl JSONArray {
    pub(crate) fn from_values(values: Vec<Value>, config: JSONConfig) -> Self {
        Self { values, config }
    }
    /// Creates an empty array.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(JSONConfig::default())
    }

    /// Creates an empty array with explicit configuration.
    #[must_use]
    pub fn with_config(config: JSONConfig) -> Self {
        Self {
            values: Vec::new(),
            config,
        }
    }

    /// Builds an array from a dynamic value.
    pub fn from_value(value: Value, config: JSONConfig) -> Result<Self> {
        match value {
            Value::Array(values) => Ok(Self { values, config }),
            value => Err(JsonError::UnexpectedType {
                expected: "array",
                actual: value_type(&value),
            }),
        }
    }

    /// Parses a JSON array.
    pub fn parse(input: &str) -> Result<Self> {
        Self::from_value(crate::parse(input)?, JSONConfig::default())
    }

    /// Returns the number of elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether the array is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Borrows one element.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.values.get(index)
    }

    /// Appends a value unless configured to ignore nulls.
    pub fn push(&mut self, value: Value) -> &mut Self {
        if !(self.config.is_ignore_null_value() && value.is_null()) {
            self.values.push(normalize_value(value, &self.config));
        }
        self
    }

    /// Sets an index, extending missing positions with nulls.
    pub fn set(&mut self, index: usize, value: Value) -> &mut Self {
        if self.values.len() <= index {
            self.values.resize(index + 1, Value::Null);
        }
        self.values[index] = normalize_value(value, &self.config);
        self
    }

    /// Removes and returns an element.
    pub fn remove(&mut self, index: usize) -> Option<Value> {
        if index < self.values.len() {
            Some(self.values.remove(index))
        } else {
            None
        }
    }

    /// Joins display values without JSON string quoting.
    #[must_use]
    pub fn join(&self, separator: &str) -> String {
        self.values
            .iter()
            .map(display_value)
            .collect::<Vec<_>>()
            .join(separator)
    }

    /// Borrows a nested value by path.
    #[must_use]
    pub fn get_by_path(&self, path: &str) -> Option<&Value> {
        let tokens = parse_path(path).ok()?;
        let mut current: Option<&Value> = None;
        for token in tokens {
            current = Some(match (current, token) {
                (None, PathToken::Index(index)) => self.values.get(index)?,
                (Some(value), PathToken::Index(index)) => value.get(index)?,
                (Some(value), PathToken::Key(key)) => value.get(key)?,
                (None, PathToken::Key(_)) => return None,
            });
        }
        current
    }

    /// Writes a nested value by path.
    pub fn put_by_path(&mut self, path: &str, value: Value) -> Result<()> {
        let mut root = self.to_value();
        put_by_path(&mut root, path, value)?;
        self.values = root.as_array().cloned().ok_or(JsonError::UnexpectedType {
            expected: "array",
            actual: value_type(&root),
        })?;
        Ok(())
    }

    /// Iterates over array elements.
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.values.iter()
    }
}

impl JsonContainer for JSONArray {
    fn config(&self) -> &JSONConfig {
        &self.config
    }
    fn to_value(&self) -> Value {
        Value::Array(self.values.clone())
    }
}

impl fmt::Display for JSONArray {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.to_value())
    }
}

impl Index<usize> for JSONArray {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

fn parse_path(path: &str) -> std::result::Result<Vec<PathToken>, PathError> {
    let path = path.strip_prefix('$').unwrap_or(path);
    let bytes = path.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'.' {
            index += 1;
            continue;
        }
        if bytes[index] == b'[' {
            let end = path[index + 1..]
                .find(']')
                .map(|offset| index + 1 + offset)
                .ok_or_else(|| PathError::Invalid(path.to_owned()))?;
            let value = path[index + 1..end]
                .parse::<usize>()
                .map_err(|_| PathError::Invalid(path.to_owned()))?;
            tokens.push(PathToken::Index(value));
            index = end + 1;
            continue;
        }
        let end = path[index..]
            .find(['.', '['])
            .map_or(bytes.len(), |offset| index + offset);
        tokens.push(PathToken::Key(path[index..end].to_owned()));
        index = end;
    }
    Ok(tokens)
}

enum PathToken {
    Key(String),
    Index(usize),
}

pub(crate) fn put_by_path(value: &mut Value, path: &str, replacement: Value) -> Result<()> {
    let tokens = parse_path(path)?;
    if tokens.is_empty() {
        *value = replacement;
        return Ok(());
    }
    let mut current = value;
    for (position, token) in tokens.iter().enumerate() {
        let last = position + 1 == tokens.len();
        match token {
            PathToken::Key(key) => {
                if current.is_null() {
                    *current = Value::Object(Map::new());
                }
                let Value::Object(object) = current else {
                    return Err(PathError::Type(key.clone()).into());
                };
                if last {
                    object.insert(key.clone(), replacement.clone());
                    break;
                }
                current = object.entry(key.clone()).or_insert(Value::Null);
            }
            PathToken::Index(index) => {
                if current.is_null() {
                    *current = Value::Array(Vec::new());
                }
                let Value::Array(array) = current else {
                    return Err(PathError::Type(index.to_string()).into());
                };
                if array.len() <= *index {
                    array.resize(*index + 1, Value::Null);
                }
                if last {
                    array[*index] = replacement.clone();
                    break;
                }
                current = &mut array[*index];
            }
        }
    }
    Ok(())
}

fn normalize_value(value: Value, config: &JSONConfig) -> Value {
    if config.is_write_long_as_string() {
        if let Value::Number(number) = &value {
            if number.as_i64().is_some() || number.as_u64().is_some() {
                return Value::String(number.to_string());
            }
        }
    }
    value
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

pub(crate) fn get_by_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let tokens = parse_path(path).ok()?;
    let mut current = value;
    for token in tokens {
        current = match token {
            PathToken::Key(key) => current.get(key)?,
            PathToken::Index(index) => current.get(index)?,
        };
    }
    Some(current)
}

fn display_value(value: &Value) -> String {
    value
        .as_str()
        .map_or_else(|| value.to_string(), str::to_owned)
}
