//! Core utilities shared by the `HiTool` workspace.
//!
//! The crate intentionally avoids async runtimes, HTTP clients, and database
//! drivers. It provides small, deterministic building blocks with explicit
//! errors and allocation behavior.

#![forbid(unsafe_code)]

mod advanced_codec;
mod codec;
mod collection;
mod date;
mod error;
mod id;
mod string;

pub use advanced_codec::{
    HashIds, MorseCodec, base32_decode, base32_encode, base32_hex_decode, base32_hex_encode,
    base58_decode, base58_decode_checked, base58_decode_checked_auto, base58_encode,
    base58_encode_checked, base62_decode, base62_encode, base62_inverted_decode,
    base62_inverted_encode, bcd_decode, bcd_encode, caesar_decode, caesar_encode,
    idna_decode_domain, idna_encode_domain, punycode_decode, punycode_encode,
    punycode_encode_prefixed, rot_decode, rot_encode,
};
pub use codec::{
    base64_decode, base64_encode, base64_url_decode, base64_url_encode, hex_decode, hex_encode,
    percent_decode, percent_encode_component,
};
pub use collection::{distinct, group_by, partition};
pub use date::DateUtil;
pub use error::{CoreError, Result};
pub use id::IdUtil;
pub use string::{
    StrExt, format_template, is_blank, lower_first, remove_all, remove_chars, split, upper_first,
};

/// Common imports for applications using `hitool-core`.
pub mod prelude {
    pub use crate::{DateUtil, IdUtil, StrExt};
}
