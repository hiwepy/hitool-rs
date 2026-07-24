use std::{fmt, ops::Index};

use serde::Serialize;
use serde_json::{Map, Number, Value};

use crate::{JsonError, Result};

/// Configuration shared by Hutool-compatible JSON containers.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct JSONConfig {
    ignore_error: bool,
    ignore_case: bool,
    date_format: Option<String>,
    ignore_null_value: bool,
    transient_support: bool,
    strip_trailing_zeros: bool,
    check_duplicate: bool,
    write_long_as_string: bool,
    natural_key_order: bool,
}

impl Default for JSONConfig {
    fn default() -> Self {
        Self {
            ignore_error: false,
            ignore_case: false,
            date_format: None,
            ignore_null_value: false,
            transient_support: true,
            strip_trailing_zeros: true,
            check_duplicate: false,
            write_long_as_string: false,
            natural_key_order: false,
        }
    }
}

impl JSONConfig {
    /// Creates Hutool-compatible defaults.
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// JSON objects are deterministically ordered in Rust.
    #[must_use]
    pub const fn is_order(&self) -> bool {
        true
    }

    /// Retains the deprecated Hutool option as a no-op.
    pub const fn set_order(&mut self, _order: bool) -> &mut Self {
        self
    }

    /// Uses lexicographic key order when serializing an object.
    pub const fn set_nature_key_comparator(&mut self) -> &mut Self {
        self.natural_key_order = true;
        self
    }

    /// Returns whether lexicographic key ordering was requested.
    #[must_use]
    pub const fn has_nature_key_comparator(&self) -> bool {
        self.natural_key_order
    }

    /// Returns whether conversion failures may be ignored.
    #[must_use]
    pub const fn is_ignore_error(&self) -> bool {
        self.ignore_error
    }

    /// Configures conversion error handling.
    pub const fn set_ignore_error(&mut self, value: bool) -> &mut Self {
        self.ignore_error = value;
        self
    }

    /// Returns whether object lookup is ASCII case-insensitive.
    #[must_use]
    pub const fn is_ignore_case(&self) -> bool {
        self.ignore_case
    }

    /// Configures ASCII case-insensitive object lookup.
    pub const fn set_ignore_case(&mut self, value: bool) -> &mut Self {
        self.ignore_case = value;
        self
    }

    /// Returns the configured date format.
    #[must_use]
    pub fn date_format(&self) -> Option<&str> {
        self.date_format.as_deref()
    }

    /// Replaces the date format; an empty value restores timestamp mode.
    pub fn set_date_format(&mut self, value: &str) -> &mut Self {
        self.date_format = (!value.is_empty()).then(|| value.to_owned());
        self
    }

    /// Returns whether null object fields and array entries are omitted.
    #[must_use]
    pub const fn is_ignore_null_value(&self) -> bool {
        self.ignore_null_value
    }

    /// Configures null omission.
    pub const fn set_ignore_null_value(&mut self, value: bool) -> &mut Self {
        self.ignore_null_value = value;
        self
    }

    /// Returns whether transient Java-style fields are ignored during migration.
    #[must_use]
    pub const fn is_transient_support(&self) -> bool {
        self.transient_support
    }

    /// Retains the Java transient-field compatibility option.
    pub const fn set_transient_support(&mut self, value: bool) -> &mut Self {
        self.transient_support = value;
        self
    }

    /// Returns whether decimal trailing zeroes are stripped.
    #[must_use]
    pub const fn is_strip_trailing_zeros(&self) -> bool {
        self.strip_trailing_zeros
    }

    /// Configures decimal formatting.
    pub const fn set_strip_trailing_zeros(&mut self, value: bool) -> &mut Self {
        self.strip_trailing_zeros = value;
        self
    }

    /// Returns whether duplicate object keys are rejected.
    #[must_use]
    pub const fn is_check_duplicate(&self) -> bool {
        self.check_duplicate
    }

    /// Configures duplicate-key rejection for explicit mutations.
    pub const fn set_check_duplicate(&mut self, value: bool) -> &mut Self {
        self.check_duplicate = value;
        self
    }

    /// Returns whether 64-bit integers are serialized as strings.
    #[must_use]
    pub const fn is_write_long_as_string(&self) -> bool {
        self.write_long_as_string
    }

    /// Configures JavaScript-safe 64-bit integer serialization.
    pub const fn set_write_long_as_string(&mut self, value: bool) -> &mut Self {
        self.write_long_as_string = value;
        self
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
