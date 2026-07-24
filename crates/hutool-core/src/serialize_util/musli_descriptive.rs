//! Binary serialization helpers aligned with Hutool's `SerializeUtil`.
//!
//! HiTool keeps the public API stable while optional engines provide the actual
//! encoding. Müsli formats are intentionally explicit: wire is for independently
//! upgraded peers, storage is for forward-evolving persisted data, packed is for
//! synchronized models, and descriptive is for inspection and migration.

use std::{fmt, ops::RangeInclusive};

/// Müsli descriptive facade for inspection and migration.
#[derive(Debug, Clone, Copy, Default)]
pub struct MusliDescriptive;

macro_rules! define_musli_codec {
    ($type:ident, $feature:literal, $module:ident, $codec:expr) => {
        #[cfg(feature = $feature)]
        impl $type {
            /// Encodes a raw payload without a HiTool envelope.
            pub fn encode<T>(value: &T) -> SerializeResult<Vec<u8>>
            where
                T: ?Sized + musli::Encode<musli::mode::Binary>,
            {
                musli::$module::to_vec(value).map_err(|error| SerializeError::Encode {
                    codec: $codec,
                    message: error.to_string(),
                })
            }

            /// Encodes into a reusable vector, clearing it first.
            ///
            /// The vector is empty if encoding fails.
            pub fn encode_into<T>(output: &mut Vec<u8>, value: &T) -> SerializeResult<()>
            where
                T: ?Sized + musli::Encode<musli::mode::Binary>,
            {
                output.clear();
                if let Err(error) = musli::$module::Encoding::new().encode(&mut *output, value) {
                    output.clear();
                    return Err(SerializeError::Encode {
                        codec: $codec,
                        message: error.to_string(),
                    });
                }
                Ok(())
            }

            /// Encodes into caller-owned fixed storage.
            pub fn encode_to_slice<T>(output: &mut [u8], value: &T) -> SerializeResult<usize>
            where
                T: ?Sized + musli::Encode<musli::mode::Binary>,
            {
                musli::$module::to_slice(output, value).map_err(|error| {
                    let message = error.to_string();
                    let normalized = message.to_ascii_lowercase();
                    if normalized.contains("overflow")
                        || normalized.contains("full")
                        || normalized.contains("space")
                    {
                        SerializeError::BufferTooSmall { codec: $codec }
                    } else {
                        SerializeError::Encode {
                            codec: $codec,
                            message,
                        }
                    }
                })
            }

            /// Decodes one exact raw payload and rejects trailing bytes.
            pub fn decode<'de, T>(bytes: &'de [u8]) -> SerializeResult<T>
            where
                T: musli::Decode<'de, musli::mode::Binary, musli::alloc::Global>,
            {
                decode_exact::<T, _>(bytes, $codec, |reader| {
                    musli::$module::decode(reader).map_err(|error| SerializeError::Decode {
                        codec: $codec,
                        message: error.to_string(),
                    })
                })
            }

            /// Encodes a checksummed, schema-aware HiTool frame.
            pub fn encode_frame<T>(value: &T, options: &EnvelopeOptions) -> SerializeResult<Vec<u8>>
            where
                T: ?Sized + musli::Encode<musli::mode::Binary>,
            {
                let mut output = Vec::new();
                Self::encode_frame_into(&mut output, value, options)?;
                Ok(output)
            }

            /// Encodes a frame into a reusable vector.
            ///
            /// The vector is empty if encoding or envelope validation fails.
            pub fn encode_frame_into<T>(
                output: &mut Vec<u8>,
                value: &T,
                options: &EnvelopeOptions,
            ) -> SerializeResult<()>
            where
                T: ?Sized + musli::Encode<musli::mode::Binary>,
            {
                encode_frame_with(output, $codec, options, |output| {
                    musli::$module::Encoding::new()
                        .encode(output, value)
                        .map(|_| ())
                        .map_err(|error| SerializeError::Encode {
                            codec: $codec,
                            message: error.to_string(),
                        })
                })
            }

            /// Validates an envelope and decodes its exact payload.
            pub fn decode_frame<'de, T>(
                bytes: &'de [u8],
                options: &EnvelopeOptions,
            ) -> SerializeResult<T>
            where
                T: musli::Decode<'de, musli::mode::Binary, musli::alloc::Global>,
            {
                let frame = parse_frame(bytes, Some($codec), options)?;
                Self::decode(frame.payload)
            }
        }
    };
}
