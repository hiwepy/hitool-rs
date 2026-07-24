use std::cmp::Ordering;

use thiserror::Error;

/// Errors returned by Hutool-compatible version expression matching.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum VersionError {
    /// The expression separator is blank, a range marker, or a comparison operator.
    #[error("invalid version delimiter: {0:?}")]
    InvalidDelimiter(String),
}
