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

/// Base58 encoder with a custom validated alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base58Encoder {
    alphabet: [u8; 58],
}

impl Base58Encoder {
    /// Bitcoin alphabet encoder used by Hutool.
    pub const fn bitcoin() -> Self {
        Self {
            alphabet: BASE58_BITCOIN_BYTES,
        }
    }

    /// Creates an encoder for a 58-character unique ASCII alphabet.
    pub fn new(alphabet: &str) -> Result<Self> {
        Ok(Self {
            alphabet: validate_alphabet::<58>(alphabet, "Base58")?,
        })
    }

    /// Encodes arbitrary bytes.
    #[must_use]
    pub fn encode_bytes(&self, input: &[u8]) -> String {
        translate_digits(&convert_base(input, 256, 58), &self.alphabet)
    }
}

impl Encoder<[u8], String> for Base58Encoder {
    fn encode(&self, input: &[u8]) -> Result<String> {
        Ok(self.encode_bytes(input))
    }
}

const BASE58_BITCOIN_BYTES: [u8; 58] =

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
