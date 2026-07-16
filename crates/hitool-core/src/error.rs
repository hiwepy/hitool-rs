//! Error types for core utilities.

/// Result type returned by fallible core utilities.
pub type Result<T> = std::result::Result<T, CoreError>;

/// Errors produced by `hitool-core`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CoreError {
    /// Base64 input could not be decoded.
    #[error("invalid base64 input: {0}")]
    Base64(#[from] base64::DecodeError),

    /// Hexadecimal input could not be decoded.
    #[error("invalid hexadecimal input: {0}")]
    Hex(#[from] hex::FromHexError),

    /// Decoded bytes were not valid UTF-8.
    #[error("decoded text is not valid UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    /// A non-Base64/hex codec rejected malformed input or configuration.
    #[error("codec error: {0}")]
    Codec(String),

    /// A codec-backed reader, writer, or file operation failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// A date or date-time string did not match the requested format.
    #[error("invalid date or date-time: {0}")]
    DateParse(#[from] chrono::ParseError),

    /// An argument violated a documented precondition.
    #[error("invalid argument `{name}`: {reason}")]
    InvalidArgument {
        /// Argument name.
        name: &'static str,
        /// Human-readable reason.
        reason: &'static str,
    },

    /// A checked date operation overflowed its supported range.
    #[error("date operation overflowed the supported range")]
    DateOverflow,
}
