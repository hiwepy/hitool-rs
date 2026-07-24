//! Binary serialization helpers aligned with Hutool's `SerializeUtil`.
//!
//! HiTool keeps the public API stable while optional engines provide the actual
//! encoding. Müsli formats are intentionally explicit: wire is for independently
//! upgraded peers, storage is for forward-evolving persisted data, packed is for
//! synchronized models, and descriptive is for inspection and migration.

use std::{fmt, ops::RangeInclusive};

mod serialize_result;
mod serialization_codec;
mod envelope_options;
mod frame_metadata;
mod frame;
mod serialize_error;
mod musli_wire;
mod musli_storage;
mod musli_packed;
mod musli_descriptive;
mod serialize_util;

pub use serialize_result::SerializeResult;
pub use serialization_codec::SerializationCodec;
pub use envelope_options::EnvelopeOptions;
pub use frame_metadata::FrameMetadata;
pub use frame::Frame;
pub use serialize_error::SerializeError;
pub use musli_wire::MusliWire;
pub use musli_storage::MusliStorage;
pub use musli_packed::MusliPacked;
pub use musli_descriptive::MusliDescriptive;
pub use serialize_util::SerializeUtil;
pub use serialize_result::ENVELOPE_VERSION;
pub use serialize_result::ENVELOPE_HEADER_LEN;
pub use envelope_options::DEFAULT_MAX_PAYLOAD_LEN;
pub use serialize_result::inspect_frame;
