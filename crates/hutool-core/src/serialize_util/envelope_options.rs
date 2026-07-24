//! Binary serialization helpers aligned with Hutool's `SerializeUtil`.
//!
//! HiTool keeps the public API stable while optional engines provide the actual
//! encoding. Müsli formats are intentionally explicit: wire is for independently
//! upgraded peers, storage is for forward-evolving persisted data, packed is for
//! synchronized models, and descriptive is for inspection and migration.

use std::{fmt, ops::RangeInclusive};

use super::serialize_error::SerializeError;
use super::serialize_result::SerializeResult;

/// Options controlling envelope validation and resource usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvelopeOptions {
    schema_id: u32,
    schema_version: u16,
    minimum_compatible_version: u16,
    maximum_compatible_version: u16,
    max_payload_len: usize,
    checksum: bool,
    require_checksum: bool,
}

impl EnvelopeOptions {
    /// Creates options for one exact schema version.
    #[must_use]
    pub const fn new(schema_id: u32, schema_version: u16) -> Self {
        Self {
            schema_id,
            schema_version,
            minimum_compatible_version: schema_version,
            maximum_compatible_version: schema_version,
            max_payload_len: DEFAULT_MAX_PAYLOAD_LEN,
            checksum: true,
            require_checksum: true,
        }
    }

    /// Accepts frames whose schema versions fall in the inclusive range.
    #[must_use]
    pub fn with_compatible_versions(mut self, versions: RangeInclusive<u16>) -> Self {
        self.minimum_compatible_version = *versions.start();
        self.maximum_compatible_version = *versions.end();
        self
    }

    /// Sets the maximum accepted or emitted payload length.
    #[must_use]
    pub const fn with_max_payload_len(mut self, max_payload_len: usize) -> Self {
        self.max_payload_len = max_payload_len;
        self
    }

    /// Controls whether newly encoded frames contain a CRC32 checksum.
    #[must_use]
    pub const fn with_checksum(mut self, checksum: bool) -> Self {
        self.checksum = checksum;
        self
    }

    /// Controls whether decoding rejects frames without a checksum.
    #[must_use]
    pub const fn require_checksum(mut self, require_checksum: bool) -> Self {
        self.require_checksum = require_checksum;
        self
    }

    /// Returns the stable schema identifier.
    #[must_use]
    pub const fn schema_id(self) -> u32 {
        self.schema_id
    }

    /// Returns the schema version written when encoding.
    #[must_use]
    pub const fn schema_version(self) -> u16 {
        self.schema_version
    }

    /// Returns the maximum payload length.
    #[must_use]
    pub const fn max_payload_len(self) -> usize {
        self.max_payload_len
    }

    fn validate(self) -> SerializeResult<()> {
        if self.minimum_compatible_version > self.maximum_compatible_version {
            return Err(SerializeError::InvalidOptions(
                "minimum compatible schema version exceeds maximum",
            ));
        }

        if !self.compatible_versions().contains(&self.schema_version) {
            return Err(SerializeError::InvalidOptions(
                "encoded schema version is outside the compatible range",
            ));
        }

        Ok(())
    }

    fn compatible_versions(self) -> RangeInclusive<u16> {
        self.minimum_compatible_version..=self.maximum_compatible_version
    }
}

impl Default for EnvelopeOptions {
    fn default() -> Self {
        Self::new(0, 1)
    }
}

pub(crate) const DEFAULT_MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;
