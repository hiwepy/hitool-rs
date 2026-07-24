//! Hutool-compatible codec facades whose configuration exceeds the small core helpers.

use std::{
    collections::BTreeSet,
    io::{Read, Write},
    path::Path,
};

use base64::Engine as _;
use encoding_rs::Encoding;

use crate::{CoreError, Result};

/// Rust-native equivalent of Hutool's generic decoder contract.
pub trait Decoder<Input: ?Sized, Output> {
    /// Decodes `input` into the configured output representation.
    fn decode(&self, input: &Input) -> Result<Output>;
}

pub(crate) const fn is_base64_code(byte: u8) -> bool {
    matches!(
        byte,
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'-' | b'/' | b'_' | b'='
    )
}
