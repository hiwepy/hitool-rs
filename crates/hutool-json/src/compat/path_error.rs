use std::{fmt, ops::Index};

use serde::Serialize;
use serde_json::{Map, Number, Value};

use crate::{JsonError, Result};

/// Error raised by JSON path parsing or traversal.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PathError {
    /// The path has invalid syntax.
    #[error("invalid JSON path: {0}")]
    Invalid(String),
    /// Traversal encountered a value with an incompatible shape.
    #[error("cannot traverse JSON path segment: {0}")]
    Type(String),
}
