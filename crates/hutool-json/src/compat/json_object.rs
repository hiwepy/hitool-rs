use std::{fmt, ops::Index};

use serde::Serialize;
use serde_json::{Map, Number, Value};

use crate::{JsonError, Result};

use super::json_config::JSONConfig;
use super::json_container::JsonContainer;
use super::path_error::PathError;

/// Mutable, configured JSON object compatible with Hutool's `JSONObject`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JSONObject {
    entries: Map<String, Value>,
    config: JSONConfig,
}

impl Default for JSONObject {
    fn default() -> Self {
        Self::new()
    }
}

impl JSONObject {
    pub(crate) fn from_entries(entries: Map<String, Value>, config: JSONConfig) -> Self {
        Self { entries, config }
    }
    /// Creates an empty object.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(JSONConfig::default())
    }

    /// Creates an empty object with explicit configuration.
    #[must_use]
    pub fn with_config(config: JSONConfig) -> Self {
        Self {
            entries: Map::new(),
            config,
        }
    }

    /// Converts a serializable Rust value into an object.
    pub fn from_serializable<T: Serialize + ?Sized>(value: &T) -> Result<Self> {
        let value = serde_json::to_value(value)?;
        Self::from_value(value, JSONConfig::default())
    }

    /// Builds an object from a dynamic value.
    pub fn from_value(value: Value, config: JSONConfig) -> Result<Self> {
        match value {
            Value::Object(entries) => Ok(Self { entries, config }),
            value => Err(JsonError::UnexpectedType {
                expected: "object",
                actual: value_type(&value),
            }),
        }
    }

    /// Parses a JSON object.
    pub fn parse(input: &str) -> Result<Self> {
        Self::from_value(crate::parse(input)?, JSONConfig::default())
    }

    /// Returns the number of fields.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the object has no fields.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Gets one field, applying configured ASCII case-insensitive lookup.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.get(key).or_else(|| {
            self.config.is_ignore_case().then(|| {
                self.entries
                    .iter()
                    .find(|(candidate, _)| candidate.eq_ignore_ascii_case(key))
                    .map(|(_, value)| value)
            })?
        })
    }

    /// Gets one field or a caller-provided default.
    #[must_use]
    pub fn get_or<'a>(&'a self, key: &str, default: &'a Value) -> &'a Value {
        self.get(key).unwrap_or(default)
    }

    /// Inserts or replaces a field according to the configuration.
    pub fn set(&mut self, key: &str, value: Value) -> Result<&mut Self> {
        if self.config.is_ignore_null_value() && value.is_null() {
            self.entries.remove(key);
            return Ok(self);
        }
        if self.config.is_check_duplicate() && self.entries.contains_key(key) {
            return Err(PathError::Invalid(format!("duplicate key: {key}")).into());
        }
        self.entries
            .insert(key.to_owned(), normalize_value(value, &self.config));
        Ok(self)
    }

    /// Inserts only a non-null field.
    pub fn put_opt(&mut self, key: &str, value: Value) -> Result<&mut Self> {
        if !key.is_empty() && !value.is_null() {
            self.set(key, value)?;
        }
        Ok(self)
    }

    /// Inserts a field only when it does not already exist.
    pub fn put_once(&mut self, key: &str, value: Value) -> Result<&mut Self> {
        if self.entries.contains_key(key) {
            return Err(PathError::Invalid(format!("duplicate key: {key}")).into());
        }
        self.set(key, value)
    }

    /// Accumulates repeated values into an array.
    pub fn accumulate(&mut self, key: &str, value: Value) -> Result<&mut Self> {
        match self.entries.remove(key) {
            None => self.set(key, value),
            Some(Value::Array(mut values)) => {
                values.push(value);
                self.set(key, Value::Array(values))
            }
            Some(previous) => self.set(key, Value::Array(vec![previous, value])),
        }
    }

    /// Appends to an existing array field.
    pub fn append(&mut self, key: &str, value: Value) -> Result<&mut Self> {
        let values = self
            .entries
            .get_mut(key)
            .and_then(Value::as_array_mut)
            .ok_or_else(|| PathError::Type(key.to_owned()))?;
        values.push(value);
        Ok(self)
    }

    /// Increments a numeric field, treating a missing field as zero.
    pub fn increment(&mut self, key: &str) -> Result<&mut Self> {
        let next = match self.entries.get(key) {
            None => 1,
            Some(Value::Number(number)) => number
                .as_i64()
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| PathError::Type(key.to_owned()))?,
            Some(_) => return Err(PathError::Type(key.to_owned()).into()),
        };
        self.entries
            .insert(key.to_owned(), Value::Number(Number::from(next)));
        Ok(self)
    }

    /// Borrows a nested value by path.
    #[must_use]
    pub fn get_by_path(&self, path: &str) -> Option<&Value> {
        get_object_path(self, path)
    }

    /// Writes a nested value by path.
    pub fn put_by_path(&mut self, path: &str, value: Value) -> Result<()> {
        let mut root = self.to_value();
        put_by_path(&mut root, path, value)?;
        self.entries = root.as_object().cloned().ok_or(JsonError::UnexpectedType {
            expected: "object",
            actual: value_type(&root),
        })?;
        Ok(())
    }

    /// Removes and returns a field.
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.entries.remove(key)
    }

    /// Iterates over fields in deterministic map order.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.entries.iter()
    }
}

impl JsonContainer for JSONObject {
    fn config(&self) -> &JSONConfig {
        &self.config
    }
    fn to_value(&self) -> Value {
        Value::Object(self.entries.clone())
    }
}

impl fmt::Display for JSONObject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.to_value())
    }
}

impl Index<&str> for JSONObject {
    type Output = Value;
    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap_or(&Value::Null)
    }
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

fn get_object_path<'a>(object: &'a JSONObject, path: &str) -> Option<&'a Value> {
    let tokens = parse_path(path).ok()?;
    let mut tokens = tokens.into_iter();
    let first = tokens.next()?;
    let mut current = match first {
        PathToken::Key(key) => object.get(&key)?,
        PathToken::Index(_) => return None,
    };
    for token in tokens {
        current = match token {
            PathToken::Key(key) => current.get(key)?,
            PathToken::Index(index) => current.get(index)?,
        };
    }
    Some(current)
}
