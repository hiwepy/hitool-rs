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

const BASE32_STANDARD_BYTES: [u8; 32] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const BASE32_HEX_BYTES: [u8; 32] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
const BASE58_BITCOIN_BYTES: [u8; 58] =
    *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BASE62_GMP_BYTES: [u8; 62] =
    *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const BASE62_INVERTED_BYTES: [u8; 62] =
    *b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Base32 encoder with a validated custom ASCII alphabet and optional pad.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base32Encoder {
    alphabet: [u8; 32],
    pad: Option<char>,
}

impl Base32Encoder {
    /// Standard RFC 4648 encoder.
    pub const fn standard() -> Self {
        Self {
            alphabet: BASE32_STANDARD_BYTES,
            pad: Some('='),
        }
    }

    /// Extended Hex RFC 4648 encoder.
    pub const fn extended_hex() -> Self {
        Self {
            alphabet: BASE32_HEX_BYTES,
            pad: Some('='),
        }
    }

    /// Creates an encoder after validating length, uniqueness, ASCII, and pad ambiguity.
    pub fn new(alphabet: &str, pad: Option<char>) -> Result<Self> {
        let alphabet = validate_alphabet::<32>(alphabet, "Base32")?;
        if pad.is_some_and(|pad| !pad.is_ascii() || alphabet.contains(&(pad as u8))) {
            return Err(CoreError::Codec(
                "Base32 pad must be ASCII and outside the alphabet".into(),
            ));
        }
        Ok(Self { alphabet, pad })
    }

    /// Encodes arbitrary bytes.
    #[must_use]
    pub fn encode_bytes(&self, input: &[u8]) -> String {
        let mut output = String::with_capacity(input.len().div_ceil(5) * 8);
        let mut buffer = 0_u16;
        let mut bits = 0_u8;
        for byte in input {
            buffer = (buffer << 8) | u16::from(*byte);
            bits += 8;
            while bits >= 5 {
                bits -= 5;
                let digit = usize::from((buffer >> bits) & 0x1f);
                output.push(char::from(self.alphabet[digit]));
            }
            buffer &= (1_u16 << bits) - 1;
        }
        if bits > 0 {
            let digit = usize::from((buffer << (5 - bits)) & 0x1f);
            output.push(char::from(self.alphabet[digit]));
        }
        if let Some(pad) = self.pad {
            while output.len() % 8 != 0 {
                output.push(pad);
            }
        }
        output
    }
}

impl Encoder<[u8], String> for Base32Encoder {
    fn encode(&self, input: &[u8]) -> Result<String> {
        Ok(self.encode_bytes(input))
    }
}

/// Base32 decoder with a validated custom ASCII alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base32Decoder {
    lookup: [i8; 128],
}

impl Base32Decoder {
    /// Standard RFC 4648 decoder.
    pub fn standard() -> Self {
        Self::from_valid_alphabet(BASE32_STANDARD_BYTES)
    }

    /// Extended Hex RFC 4648 decoder.
    pub fn extended_hex() -> Self {
        Self::from_valid_alphabet(BASE32_HEX_BYTES)
    }

    /// Creates a case-tolerant decoder for an ASCII alphabet.
    pub fn new(alphabet: &str) -> Result<Self> {
        let alphabet = validate_alphabet::<32>(alphabet, "Base32")?;
        Ok(Self::from_valid_alphabet(alphabet))
    }

    fn from_valid_alphabet(alphabet: [u8; 32]) -> Self {
        let mut lookup = [-1; 128];
        for (digit, byte) in (0_i8..).zip(alphabet.iter().copied()) {
            lookup[usize::from(byte)] = digit;
            // Hutool adds lowercase aliases only for uppercase alphabet entries.
            if byte.is_ascii_uppercase() {
                lookup[usize::from(byte.to_ascii_lowercase())] = digit;
            }
        }
        Self { lookup }
    }

    /// Decodes while ignoring padding and characters outside the configured alphabet.
    #[must_use]
    pub fn decode_text(&self, input: &str) -> Vec<u8> {
        let mut output = Vec::with_capacity(input.len().saturating_mul(5) / 8);
        let mut buffer = 0_u16;
        let mut bits = 0_u8;
        for byte in input.bytes() {
            let digit = self
                .lookup
                .get(usize::from(byte))
                .copied()
                .filter(|digit| *digit >= 0);
            let Some(digit) = digit else {
                continue;
            };
            buffer = (buffer << 5) | u16::from(u8::try_from(digit).unwrap_or_default());
            bits += 5;
            if bits >= 8 {
                bits -= 8;
                output.push(u8::try_from((buffer >> bits) & 0xff).unwrap_or_default());
                buffer &= (1_u16 << bits) - 1;
            }
        }
        output
    }
}

impl Decoder<str, Vec<u8>> for Base32Decoder {
    fn decode(&self, input: &str) -> Result<Vec<u8>> {
        Ok(self.decode_text(input))
    }
}

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

/// Base58 decoder with a custom validated alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base58Decoder {
    alphabet: [u8; 58],
}

impl Base58Decoder {
    /// Bitcoin alphabet decoder used by Hutool.
    pub const fn bitcoin() -> Self {
        Self {
            alphabet: BASE58_BITCOIN_BYTES,
        }
    }

    /// Creates a decoder for a 58-character unique ASCII alphabet.
    pub fn new(alphabet: &str) -> Result<Self> {
        Ok(Self {
            alphabet: validate_alphabet::<58>(alphabet, "Base58")?,
        })
    }

    /// Decodes custom-alphabet Base58.
    pub fn decode_text(&self, input: &str) -> Result<Vec<u8>> {
        decode_alphabet(input.as_bytes(), &self.alphabet, 58)
    }
}

impl Decoder<str, Vec<u8>> for Base58Decoder {
    fn decode(&self, input: &str) -> Result<Vec<u8>> {
        self.decode_text(input)
    }
}

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

/// Base62 decoder with a custom validated byte alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base62Decoder {
    alphabet: [u8; 62],
}

impl Base62Decoder {
    /// GMP-style alphabet decoder.
    pub const fn gmp() -> Self {
        Self {
            alphabet: BASE62_GMP_BYTES,
        }
    }

    /// Case-inverted alphabet decoder.
    pub const fn inverted() -> Self {
        Self {
            alphabet: BASE62_INVERTED_BYTES,
        }
    }

    /// Creates a decoder for a 62-character unique ASCII alphabet.
    pub fn new(alphabet: &str) -> Result<Self> {
        Ok(Self {
            alphabet: validate_alphabet::<62>(alphabet, "Base62")?,
        })
    }

    /// Decodes ASCII Base62 bytes.
    pub fn decode_bytes(&self, input: &[u8]) -> Result<Vec<u8>> {
        decode_alphabet(input, &self.alphabet, 62)
    }
}

impl Decoder<[u8], Vec<u8>> for Base62Decoder {
    fn decode(&self, input: &[u8]) -> Result<Vec<u8>> {
        self.decode_bytes(input)
    }
}

/// Encodes text to Base32 after converting it with `encoding`.
#[must_use]
pub fn base32_encode_text(input: &str, encoding: &'static Encoding, use_hex: bool) -> String {
    let (bytes, _, _) = encoding.encode(input);
    if use_hex {
        base32_hex_encode(bytes)
    } else {
        base32_encode(bytes)
    }
}

/// Decodes Base32 and converts its bytes with `encoding`.
pub fn base32_decode_text(
    input: &str,
    encoding: &'static Encoding,
    use_hex: bool,
) -> Result<String> {
    let bytes = if use_hex {
        base32_hex_decode(input)?
    } else {
        base32_decode(input)?
    };
    Ok(encoding.decode(&bytes).0.into_owned())
}

/// Reads and encodes an entire stream as Base32.
pub fn base32_encode_reader(mut reader: impl Read, use_hex: bool) -> Result<String> {
    let mut input = Vec::new();
    reader.read_to_end(&mut input)?;
    Ok(if use_hex {
        base32_hex_encode(input)
    } else {
        base32_encode(input)
    })
}

/// Reads and encodes an entire file as Base32.
pub fn base32_encode_file(path: impl AsRef<Path>, use_hex: bool) -> Result<String> {
    base32_encode_reader(std::fs::File::open(path)?, use_hex)
}

/// Decodes Base32 into a writer and returns the byte count.
pub fn base32_decode_to_writer(
    input: &str,
    mut writer: impl Write,
    use_hex: bool,
) -> Result<usize> {
    let decoded = if use_hex {
        base32_hex_decode(input)?
    } else {
        base32_decode(input)?
    };
    writer.write_all(&decoded)?;
    Ok(decoded.len())
}

/// Decodes Base32 into a file and returns the byte count.
pub fn base32_decode_to_file(input: &str, path: impl AsRef<Path>, use_hex: bool) -> Result<usize> {
    base32_decode_to_writer(input, std::fs::File::create(path)?, use_hex)
}

/// Encodes text to Base62 after converting it with `encoding`.
#[must_use]
pub fn base62_encode_text(input: &str, encoding: &'static Encoding, inverted: bool) -> String {
    let (bytes, _, _) = encoding.encode(input);
    if inverted {
        base62_inverted_encode(bytes)
    } else {
        base62_encode(bytes)
    }
}

/// Decodes Base62 and converts its bytes with `encoding`.
pub fn base62_decode_text(
    input: &str,
    encoding: &'static Encoding,
    inverted: bool,
) -> Result<String> {
    let bytes = if inverted {
        base62_inverted_decode(input)?
    } else {
        base62_decode(input)?
    };
    Ok(encoding.decode(&bytes).0.into_owned())
}

/// Decodes GMP Base62 as GBK, matching Hutool's dedicated convenience overload.
pub fn base62_decode_text_gbk(input: &str) -> Result<String> {
    base62_decode_text(input, GBK, false)
}

/// Reads and encodes an entire stream as Base62.
pub fn base62_encode_reader(mut reader: impl Read, inverted: bool) -> Result<String> {
    let mut input = Vec::new();
    reader.read_to_end(&mut input)?;
    Ok(if inverted {
        base62_inverted_encode(input)
    } else {
        base62_encode(input)
    })
}

/// Reads and encodes an entire file as Base62.
pub fn base62_encode_file(path: impl AsRef<Path>, inverted: bool) -> Result<String> {
    base62_encode_reader(std::fs::File::open(path)?, inverted)
}

/// Decodes Base62 into a writer and returns the byte count.
pub fn base62_decode_to_writer(
    input: &str,
    mut writer: impl Write,
    inverted: bool,
) -> Result<usize> {
    let decoded = if inverted {
        base62_inverted_decode(input)?
    } else {
        base62_decode(input)?
    };
    writer.write_all(&decoded)?;
    Ok(decoded.len())
}

/// Decodes Base62 into a file and returns the byte count.
pub fn base62_decode_to_file(input: &str, path: impl AsRef<Path>, inverted: bool) -> Result<usize> {
    base62_decode_to_writer(input, std::fs::File::create(path)?, inverted)
}

/// Encodes the requested ASCII prefix as BCD, padding an odd final low nibble with zero.
pub fn bcd_encode_ascii_prefix(input: &[u8], length: usize) -> Result<Vec<u8>> {
    if length > input.len() {
        return Err(CoreError::Codec("BCD prefix exceeds input length".into()));
    }
    input[..length]
        .chunks(2)
        .map(|pair| {
            let high = bcd_nibble(pair[0])?;
            let low = pair.get(1).copied().map_or(Ok(0), bcd_nibble)?;
            Ok((high << 4) | low)
        })
        .collect()
}

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

fn decode_alphabet(input: &[u8], alphabet: &[u8], radix: u32) -> Result<Vec<u8>> {
    let digits = input
        .iter()
        .enumerate()
        .map(|(index, byte)| {
            alphabet
                .iter()
                .position(|candidate| candidate == byte)
                .and_then(|digit| u8::try_from(digit).ok())
                .ok_or_else(|| CoreError::Codec(format!("invalid radix byte at index {index}")))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(convert_base(&digits, radix, 256))
}

fn bcd_nibble(byte: u8) -> Result<u8> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(CoreError::Codec("invalid BCD hexadecimal digit".into())),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use encoding_rs::UTF_8;

    use super::*;

    #[test]
    fn custom_radix_alphabets_round_trip_and_validate() {
        let base32_alphabet = "ZYXWVUTSRQPONMLKJIHGFEDCBA765432";
        let base32_encoder = Base32Encoder::new(base32_alphabet, None).unwrap();
        let base32_decoder = Base32Decoder::new(base32_alphabet).unwrap();
        let encoded = base32_encoder.encode_bytes(b"hitool");
        assert_eq!(
            base32_decoder.decode_text(&encoded.to_lowercase()),
            b"hitool"
        );
        assert_eq!(Base32Encoder::standard().encode_bytes(b"foo"), "MZXW6===");
        assert_eq!(
            Base32Decoder::extended_hex().decode_text("CPNMU==="),
            b"foo"
        );
        let lowercase_alphabet = "abcdefghijklmnopqrstuvwxyz234567";
        let lowercase_decoder = Base32Decoder::new(lowercase_alphabet).unwrap();
        assert_eq!(lowercase_decoder.decode_text("mzxw6==="), b"foo");
        assert_ne!(lowercase_decoder.decode_text("MZXW6==="), b"foo");
        assert!(Base32Encoder::new("short", Some('=')).is_err());
        assert!(Base32Encoder::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", Some('A')).is_err());

        let reversed58: String = BASE58_BITCOIN_BYTES
            .iter()
            .rev()
            .map(|byte| char::from(*byte))
            .collect();
        let encoder58 = Base58Encoder::new(&reversed58).unwrap();
        let decoder58 = Base58Decoder::new(&reversed58).unwrap();
        let encoded = encoder58.encode_bytes(&[0, 1, 2, 3, 255]);
        assert_eq!(decoder58.decode_text(&encoded).unwrap(), [0, 1, 2, 3, 255]);
        assert!(decoder58.decode_text("invalid0").is_err());

        let reversed62: String = BASE62_GMP_BYTES
            .iter()
            .rev()
            .map(|byte| char::from(*byte))
            .collect();
        let encoder62 = Base62Encoder::new(&reversed62).unwrap();
        let decoder62 = Base62Decoder::new(&reversed62).unwrap();
        let encoded = encoder62.encode_bytes(&[0, 9, 42, 255]);
        assert_eq!(decoder62.decode_bytes(&encoded).unwrap(), [0, 9, 42, 255]);
        assert!(
            Base62Encoder::new("00000000000000000000000000000000000000000000000000000000000000")
                .is_err()
        );
    }

    #[test]
    fn text_stream_file_and_bcd_overloads_are_bounded_and_reversible() {
        let text = "HiTool工具";
        let base32 = base32_encode_text(text, UTF_8, false);
        assert_eq!(base32_decode_text(&base32, UTF_8, false).unwrap(), text);
        assert_eq!(
            base32_encode_reader(Cursor::new(text.as_bytes()), true).unwrap(),
            base32_encode_text(text, UTF_8, true)
        );
        let base62 = base62_encode_text(text, UTF_8, true);
        assert_eq!(base62_decode_text(&base62, UTF_8, true).unwrap(), text);
        assert_eq!(
            base62_encode_reader(Cursor::new(text.as_bytes()), false).unwrap(),
            base62_encode_text(text, UTF_8, false)
        );

        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.bin");
        let base32_target = directory.path().join("base32.bin");
        let base62_target = directory.path().join("base62.bin");
        std::fs::write(&source, b"file-data").unwrap();
        let encoded32 = base32_encode_file(&source, false).unwrap();
        assert_eq!(
            base32_decode_to_file(&encoded32, &base32_target, false).unwrap(),
            9
        );
        let encoded62 = base62_encode_file(&source, false).unwrap();
        assert_eq!(
            base62_decode_to_file(&encoded62, &base62_target, false).unwrap(),
            9
        );
        assert_eq!(std::fs::read(base32_target).unwrap(), b"file-data");
        assert_eq!(std::fs::read(base62_target).unwrap(), b"file-data");

        let mut output = Vec::new();
        assert_eq!(
            base32_decode_to_writer(&encoded32, &mut output, false).unwrap(),
            9
        );
        output.clear();
        assert_eq!(
            base62_decode_to_writer(&encoded62, &mut output, false).unwrap(),
            9
        );
        assert_eq!(
            bcd_encode_ascii_prefix(b"123ABC", 5).unwrap(),
            [0x12, 0x3a, 0xb0]
        );
        assert!(bcd_encode_ascii_prefix(b"12", 3).is_err());
        assert!(bcd_encode_ascii_prefix(b"1Z", 2).is_err());
    }
}
