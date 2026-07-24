//! Binary serialization helpers aligned with Hutool's `SerializeUtil`.
//!
//! HiTool keeps the public API stable while optional engines provide the actual
//! encoding. Müsli formats are intentionally explicit: wire is for independently
//! upgraded peers, storage is for forward-evolving persisted data, packed is for
//! synchronized models, and descriptive is for inspection and migration.

use std::{fmt, ops::RangeInclusive};

use super::envelope_options::EnvelopeOptions;
use super::musli_wire::MusliWire;
use super::serialize_result::SerializeResult;

/// Hutool-aligned convenience facade.
///
/// Its compatibility methods deliberately use Müsli wire frames, so the
/// serialized bytes are self-identifying and tolerate independently upgraded
/// Rust models. New code may use [`MusliWire`] directly to make the format
/// choice visible at the call site.
#[derive(Debug, Clone, Copy, Default)]
pub struct SerializeUtil;

impl SerializeUtil {
    /// Deep-clones an owned value through a wire round trip.
    pub fn clone<T>(value: &T) -> SerializeResult<T>
    where
        T: musli::Encode<musli::mode::Binary>,
        T: for<'de> musli::Decode<'de, musli::mode::Binary, musli::alloc::Global>,
    {
        let bytes = MusliWire::encode(value)?;
        MusliWire::decode(&bytes)
    }

    /// Serializes using a checksummed default wire envelope.
    pub fn serialize<T>(value: &T) -> SerializeResult<Vec<u8>>
    where
        T: ?Sized + musli::Encode<musli::mode::Binary>,
    {
        MusliWire::encode_frame(value, &EnvelopeOptions::default())
    }

    /// Deserializes a checksummed default wire envelope.
    pub fn deserialize<'de, T>(bytes: &'de [u8]) -> SerializeResult<T>
    where
        T: musli::Decode<'de, musli::mode::Binary, musli::alloc::Global>,
    {
        MusliWire::decode_frame(bytes, &EnvelopeOptions::default())
    }

    /// Serializes using caller-defined schema and resource policies.
    pub fn serialize_with_options<T>(
        value: &T,
        options: &EnvelopeOptions,
    ) -> SerializeResult<Vec<u8>>
    where
        T: ?Sized + musli::Encode<musli::mode::Binary>,
    {
        MusliWire::encode_frame(value, options)
    }

    /// Deserializes using caller-defined schema and resource policies.
    pub fn deserialize_with_options<'de, T>(
        bytes: &'de [u8],
        options: &EnvelopeOptions,
    ) -> SerializeResult<T>
    where
        T: musli::Decode<'de, musli::mode::Binary, musli::alloc::Global>,
    {
        MusliWire::decode_frame(bytes, options)
    }
}
