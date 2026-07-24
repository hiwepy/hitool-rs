//! Hutool-compatible codec facades whose configuration exceeds the small core helpers.

use std::{
    collections::BTreeSet,
    io::{Read, Write},
    path::Path,
};

use base64::Engine as _;
use encoding_rs::Encoding;

use crate::{CoreError, Result};

mod encoder;
mod decoder;
mod base16_codec;
mod percent_codec;

pub use encoder::Encoder;
pub use decoder::Decoder;
pub use base16_codec::Base16Codec;
pub use percent_codec::PercentCodec;
pub use encoder::base64_encode_config;
pub use encoder::base64_encode_without_padding;
pub use encoder::base64_decode_tolerant;
pub use encoder::base64_decode_range_tolerant;
pub use encoder::fn;
pub use encoder::is_base64;
pub use encoder::encoding_for_label;
pub use encoder::base64_encode_text;
pub use encoder::base64_decode_text;
pub use encoder::base64_encode_reader;
pub use encoder::base64_encode_file;
pub use encoder::base64_decode_to_writer;
pub use encoder::base64_decode_to_file;
