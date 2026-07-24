//! Hutool-aligned binary and text codecs with Rust-native error handling.

use data_encoding::{BASE32, BASE32HEX};
use idna::punycode;
use sha2::{Digest as _, Sha256};

use crate::{CoreError, Result};

mod morse_codec;
mod hash_ids;

pub use morse_codec::MorseCodec;
pub use hash_ids::HashIds;
pub use morse_codec::base32_encode;
pub use morse_codec::base32_decode;
pub use morse_codec::base32_hex_encode;
pub use morse_codec::base32_hex_decode;
pub use morse_codec::base58_encode;
pub use morse_codec::base58_decode;
pub use morse_codec::base58_encode_checked;
pub use morse_codec::base58_decode_checked;
pub use morse_codec::base58_decode_checked_auto;
pub use morse_codec::base62_encode;
pub use morse_codec::base62_decode;
pub use morse_codec::base62_inverted_encode;
pub use morse_codec::base62_inverted_decode;
pub use morse_codec::rot_encode;
pub use morse_codec::rot_decode;
pub use morse_codec::caesar_encode;
pub use morse_codec::caesar_decode;
pub use morse_codec::bcd_encode;
pub use morse_codec::bcd_decode;
pub use morse_codec::punycode_encode;
pub use morse_codec::punycode_encode_prefixed;
pub use morse_codec::punycode_decode;
pub use morse_codec::idna_encode_domain;
pub use morse_codec::idna_decode_domain;
