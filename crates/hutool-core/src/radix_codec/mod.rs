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

mod base32_encoder;
mod base32_decoder;
mod base58_encoder;
mod base58_decoder;
mod base62_encoder;
mod base62_decoder;

pub use base32_encoder::Base32Encoder;
pub use base32_decoder::Base32Decoder;
pub use base58_encoder::Base58Encoder;
pub use base58_decoder::Base58Decoder;
pub use base62_encoder::Base62Encoder;
pub use base62_decoder::Base62Decoder;
pub use base32_encoder::base32_encode_text;
pub use base32_encoder::base32_decode_text;
pub use base32_encoder::base32_encode_reader;
pub use base32_encoder::base32_encode_file;
pub use base32_encoder::base32_decode_to_writer;
pub use base32_encoder::base32_decode_to_file;
pub use base32_encoder::base62_encode_text;
pub use base32_encoder::base62_decode_text;
pub use base32_encoder::base62_decode_text_gbk;
pub use base32_encoder::base62_encode_reader;
pub use base32_encoder::base62_encode_file;
pub use base32_encoder::base62_decode_to_writer;
pub use base32_encoder::base62_decode_to_file;
pub use base32_encoder::bcd_encode_ascii_prefix;
