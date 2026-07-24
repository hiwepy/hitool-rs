//! Binary serialization helpers aligned with Hutool's `SerializeUtil`.
//!
//! HiTool keeps the public API stable while optional engines provide the actual
//! encoding. Müsli formats are intentionally explicit: wire is for independently
//! upgraded peers, storage is for forward-evolving persisted data, packed is for
//! synchronized models, and descriptive is for inspection and migration.

use std::{fmt, ops::RangeInclusive};

use super::musli_descriptive::MusliDescriptive;
use super::musli_packed::MusliPacked;
use super::musli_storage::MusliStorage;
use super::musli_wire::MusliWire;
use super::serialize_error::SerializeError;
use super::serialize_result::SerializeResult;

/// Stable identifiers written into HiTool serialization envelopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SerializationCodec {
    /// Fully upgrade-stable Müsli wire encoding.
    MusliWire = 1,
    /// Partially upgrade-stable Müsli storage encoding.
    MusliStorage = 2,
    /// Compact Müsli packed encoding for synchronized models.
    MusliPacked = 3,
    /// Self-descriptive Müsli encoding.
    MusliDescriptive = 4,
}

impl TryFrom<u8> for SerializationCodec {
    type Error = SerializeError;

    fn try_from(value: u8) -> SerializeResult<Self> {
        match value {
            1 => Ok(Self::MusliWire),
            2 => Ok(Self::MusliStorage),
            3 => Ok(Self::MusliPacked),
            4 => Ok(Self::MusliDescriptive),
            codec => Err(SerializeError::UnsupportedCodec { codec }),
        }
    }
}

impl fmt::Display for SerializationCodec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::MusliWire => "musli-wire",
            Self::MusliStorage => "musli-storage",
            Self::MusliPacked => "musli-packed",
            Self::MusliDescriptive => "musli-descriptive",
        })
    }
}
