//! Binary serialization helpers aligned with Hutool's `SerializeUtil`.
//!
//! HiTool keeps the public API stable while optional engines provide the actual
//! encoding. Müsli formats are intentionally explicit: wire is for independently
//! upgraded peers, storage is for forward-evolving persisted data, packed is for
//! synchronized models, and descriptive is for inspection and migration.

use std::{fmt, ops::RangeInclusive};

use super::serialization_codec::SerializationCodec;

/// Metadata decoded from a validated HiTool envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameMetadata {
    /// Encoding used for the payload.
    pub codec: SerializationCodec,
    /// Application-defined schema identifier.
    pub schema_id: u32,
    /// Application-defined schema version.
    pub schema_version: u16,
    /// Encoded payload length.
    pub payload_len: usize,
    /// Whether the frame carries a CRC32 checksum.
    pub checksummed: bool,
}
