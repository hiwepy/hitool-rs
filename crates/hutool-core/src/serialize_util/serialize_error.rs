//! Binary serialization helpers aligned with Hutool's `SerializeUtil`.
//!
//! HiTool keeps the public API stable while optional engines provide the actual
//! encoding. Müsli formats are intentionally explicit: wire is for independently
//! upgraded peers, storage is for forward-evolving persisted data, packed is for
//! synchronized models, and descriptive is for inspection and migration.

use std::{fmt, ops::RangeInclusive};

use super::frame::Frame;
use super::serialization_codec::SerializationCodec;

/// Errors emitted by HiTool serialization and envelope processing.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum SerializeError {
    /// The selected engine failed while encoding.
    #[error("{codec} encoding failed: {message}")]
    Encode {
        /// Selected codec.
        codec: SerializationCodec,
        /// Engine diagnostic isolated from the public error type.
        message: String,
    },
    /// The selected engine failed while decoding.
    #[error("{codec} decoding failed: {message}")]
    Decode {
        /// Selected codec.
        codec: SerializationCodec,
        /// Engine diagnostic isolated from the public error type.
        message: String,
    },
    /// A fixed output buffer could not hold the encoded value.
    #[error("{codec} output buffer is too small")]
    BufferTooSmall {
        /// Selected codec.
        codec: SerializationCodec,
    },
    /// Payload length exceeded the configured resource limit.
    #[error("payload exceeds limit: {actual} > {limit}")]
    PayloadTooLarge {
        /// Observed or requested payload length.
        actual: usize,
        /// Configured limit.
        limit: usize,
    },
    /// Input did not contain a complete envelope header.
    #[error("truncated serialization envelope: {actual} < {required}")]
    TruncatedEnvelope {
        /// Available byte count.
        actual: usize,
        /// Required byte count.
        required: usize,
    },
    /// Envelope magic did not identify HiTool data.
    #[error("invalid HiTool serialization magic")]
    InvalidMagic,
    /// Envelope version is not supported by this release.
    #[error("unsupported serialization envelope version: {version}")]
    UnsupportedEnvelopeVersion {
        /// Version found in the frame.
        version: u8,
    },
    /// Codec identifier is not known.
    #[error("unsupported serialization codec: {codec}")]
    UnsupportedCodec {
        /// Raw codec identifier.
        codec: u8,
    },
    /// Frame used a different codec from the explicitly selected facade.
    #[error("unexpected codec: expected {expected}, got {actual}")]
    UnexpectedCodec {
        /// Codec required by the facade.
        expected: SerializationCodec,
        /// Codec found in the frame.
        actual: SerializationCodec,
    },
    /// Schema identifier did not match the configured application schema.
    #[error("schema mismatch: expected {expected}, got {actual}")]
    SchemaMismatch {
        /// Configured schema identifier.
        expected: u32,
        /// Identifier found in the frame.
        actual: u32,
    },
    /// Schema version fell outside the configured compatibility range.
    #[error("incompatible schema version: {actual}, expected {minimum}..={maximum}")]
    IncompatibleSchemaVersion {
        /// Version found in the frame.
        actual: u16,
        /// Minimum supported version.
        minimum: u16,
        /// Maximum supported version.
        maximum: u16,
    },
    /// Declared payload length did not consume the whole input.
    #[error("frame length mismatch: declared {declared}, actual {actual}")]
    LengthMismatch {
        /// Length declared by the envelope.
        declared: usize,
        /// Bytes actually available after the header.
        actual: usize,
    },
    /// CRC32 validation failed.
    #[error("serialization checksum mismatch")]
    ChecksumMismatch,
    /// Policy required checksums but the frame did not contain one.
    #[error("serialization frame is missing a required checksum")]
    ChecksumRequired,
    /// A frame used flags not understood by this release.
    #[error("unsupported serialization envelope flags: {flags:#06x}")]
    UnsupportedFlags {
        /// Unknown flag bits.
        flags: u16,
    },
    /// A decoder left unconsumed bytes inside the declared payload.
    #[error("{codec} payload contains {remaining} trailing bytes")]
    TrailingBytes {
        /// Selected codec.
        codec: SerializationCodec,
        /// Number of unconsumed bytes.
        remaining: usize,
    },
    /// Envelope options were internally inconsistent.
    #[error("invalid serialization options: {0}")]
    InvalidOptions(&'static str),
}
