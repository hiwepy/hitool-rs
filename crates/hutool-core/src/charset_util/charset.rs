use std::{
    fmt, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use encoding_rs::{Encoding, GBK, UTF_8};
use thiserror::Error;

/// A Java-compatible character set, including encodings intentionally absent from WHATWG.
#[derive(Clone, Copy)]
pub enum Charset {
    /// Exact ISO-8859-1 rather than the WHATWG Windows-1252 alias.
    Iso88591,
    /// Strict seven-bit US-ASCII.
    Ascii,
    /// BOM-aware UTF-16 with big-endian fallback.
    Utf16,
    /// A mature WHATWG encoding supplied by `encoding_rs`.
    Encoding(&'static Encoding),
}

impl Charset {
    /// ISO-8859-1 with its exact one-byte mapping.
    pub const ISO_8859_1: Self = Self::Iso88591;
    /// UTF-8, Rust's native string encoding.
    pub const UTF_8: Self = Self::Encoding(UTF_8);
    /// GBK backed by `encoding_rs`.
    pub const GBK: Self = Self::Encoding(GBK);
    /// BOM-aware UTF-16, defaulting to big endian when no BOM is present.
    pub const UTF_16: Self = Self::Utf16;

    /// Returns the canonical Java-style charset name.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Iso88591 => "ISO-8859-1",
            Self::Ascii => "US-ASCII",
            Self::Utf16 => "UTF-16",
            Self::Encoding(encoding) => encoding.name(),
        }
    }

    fn for_label(label: &str) -> Option<Self> {
        if is_iso_8859_1_label(label) {
            Some(Self::Iso88591)
        } else if is_ascii_label(label) {
            Some(Self::Ascii)
        } else if label.eq_ignore_ascii_case("UTF-16") || label.eq_ignore_ascii_case("UNICODE") {
            Some(Self::Utf16)
        } else {
            Encoding::for_label(label.as_bytes()).map(Self::Encoding)
        }
    }

    fn encode(self, source: &str) -> Vec<u8> {
        match self {
            Self::Iso88591 => source
                .chars()
                .map(|character| u8::try_from(u32::from(character)).unwrap_or(b'?'))
                .collect(),
            Self::Ascii => source
                .chars()
                .map(|character| {
                    u8::try_from(u32::from(character))
                        .ok()
                        .filter(u8::is_ascii)
                        .unwrap_or(b'?')
                })
                .collect(),
            Self::Utf16 => std::iter::once(0xFE)
                .chain(std::iter::once(0xFF))
                .chain(source.encode_utf16().flat_map(u16::to_be_bytes))
                .collect(),
            Self::Encoding(encoding) => encoding.encode(source).0.into_owned(),
        }
    }

    fn decode(self, bytes: &[u8]) -> String {
        match self {
            Self::Iso88591 => bytes.iter().map(|byte| char::from(*byte)).collect(),
            Self::Ascii => bytes
                .iter()
                .map(|byte| {
                    if byte.is_ascii() {
                        char::from(*byte)
                    } else {
                        char::REPLACEMENT_CHARACTER
                    }
                })
                .collect(),
            Self::Utf16 => decode_utf16(bytes),
            Self::Encoding(encoding) => encoding.decode(bytes).0.into_owned(),
        }
    }

    fn identifies(self, bytes: &[u8]) -> bool {
        match self {
            Self::Iso88591 => true,
            Self::Ascii => bytes.iter().all(u8::is_ascii),
            Self::Utf16 => identify_utf16(bytes),
            Self::Encoding(encoding) => encoding
                .decode_without_bom_handling_and_without_replacement(bytes)
                .is_some(),
        }
    }
}

impl fmt::Debug for Charset {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Charset")
            .field(&self.name())
            .finish()
    }
}

impl PartialEq for Charset {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::Iso88591, Self::Iso88591)
            | (Self::Ascii, Self::Ascii)
            | (Self::Utf16, Self::Utf16) => true,
            (Self::Encoding(left), Self::Encoding(right)) => std::ptr::eq(left, right),
            _ => false,
        }
    }
}

impl Eq for Charset {}

fn is_iso_8859_1_label(label: &str) -> bool {
    [
        "ISO-8859-1",
        "ISO_8859-1",
        "LATIN1",
        "LATIN-1",
        "L1",
        "IBM819",
        "CP819",
        "CSISOLATIN1",
    ]
    .iter()
    .any(|candidate| label.eq_ignore_ascii_case(candidate))
}

fn is_ascii_label(label: &str) -> bool {
    ["US-ASCII", "ASCII", "ISO646-US", "ANSI_X3.4-1968"]
        .iter()
        .any(|candidate| label.eq_ignore_ascii_case(candidate))
}

fn decode_utf16(bytes: &[u8]) -> String {
    let (bytes, little_endian) = if bytes.starts_with(&[0xFE, 0xFF]) {
        (&bytes[2..], false)
    } else if bytes.starts_with(&[0xFF, 0xFE]) {
        (&bytes[2..], true)
    } else {
        (bytes, false)
    };
    let mut chunks = bytes.chunks_exact(2);
    let units: Vec<u16> = chunks
        .by_ref()
        .map(|chunk| {
            let pair = [chunk[0], chunk[1]];
            if little_endian {
                u16::from_le_bytes(pair)
            } else {
                u16::from_be_bytes(pair)
            }
        })
        .collect();
    let mut decoded = String::from_utf16_lossy(&units);
    if !chunks.remainder().is_empty() {
        decoded.push(char::REPLACEMENT_CHARACTER);
    }
    decoded
}

fn identify_utf16(bytes: &[u8]) -> bool {
    if bytes.len() % 2 != 0 {
        return false;
    }
    let (bytes, little_endian) = if bytes.starts_with(&[0xFE, 0xFF]) {
        (&bytes[2..], false)
    } else if bytes.starts_with(&[0xFF, 0xFE]) {
        (&bytes[2..], true)
    } else {
        (bytes, false)
    };
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| {
            let pair = [chunk[0], chunk[1]];
            if little_endian {
                u16::from_le_bytes(pair)
            } else {
                u16::from_be_bytes(pair)
            }
        })
        .collect();
    String::from_utf16(&units).is_ok()
}
