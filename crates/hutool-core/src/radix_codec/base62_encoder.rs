//! Configurable radix codecs and Rust-native stream/file overloads.

use std::{
    io::{Read, Write},
    path::Path,
};

use encoding_rs::{Encoding, GBK};

use crate::{
    CoreError, Decoder, Encoder, Result,
    advanced_codec::{convert_base, translate_digits},
    base32_decode, base32_encode, base32_hex_decode, base32_hex_encode, base62_decode,
    base62_encode, base62_inverted_decode, base62_inverted_encode,
};

/// Base62 encoder with a custom validated byte alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base62Encoder {
    alphabet: [u8; 62],
}

impl Base62Encoder {
    /// GMP-style alphabet encoder.
    pub const fn gmp() -> Self {
        Self {
            alphabet: BASE62_GMP_BYTES,
        }
    }

    /// Case-inverted alphabet encoder.
    pub const fn inverted() -> Self {
        Self {
            alphabet: BASE62_INVERTED_BYTES,
        }
    }

    /// Creates an encoder for a 62-character unique ASCII alphabet.
    pub fn new(alphabet: &str) -> Result<Self> {
        Ok(Self {
            alphabet: validate_alphabet::<62>(alphabet, "Base62")?,
        })
    }

    /// Encodes bytes to ASCII Base62 bytes.
    #[must_use]
    pub fn encode_bytes(&self, input: &[u8]) -> Vec<u8> {
        convert_base(input, 256, 62)
            .iter()
            .map(|digit| self.alphabet[usize::from(*digit)])
            .collect()
    }
}

impl Encoder<[u8], Vec<u8>> for Base62Encoder {
    fn encode(&self, input: &[u8]) -> Result<Vec<u8>> {
        Ok(self.encode_bytes(input))
    }
}

const BASE62_GMP_BYTES: [u8; 62] =

const BASE62_INVERTED_BYTES: [u8; 62] =

fn validate_alphabet<const N: usize>(alphabet: &str, name: &str) -> Result<[u8; N]> {
    let bytes: [u8; N] = alphabet.as_bytes().try_into().map_err(|_| {
        CoreError::Codec(format!(
            "{name} alphabet must contain exactly {N} ASCII bytes"
        ))
    })?;
    if !bytes.is_ascii() {
        return Err(CoreError::Codec(format!("{name} alphabet must be ASCII")));
    }
    let mut sorted = bytes;
    sorted.sort_unstable();
    if sorted.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(CoreError::Codec(format!(
            "{name} alphabet characters must be unique"
        )));
    }
    Ok(bytes)
}
