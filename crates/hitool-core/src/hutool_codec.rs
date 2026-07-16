//! Hutool-compatible codec facades whose configuration exceeds the small core helpers.

use std::{
    collections::BTreeSet,
    io::{Read, Write},
    path::Path,
};

use base64::Engine as _;
use encoding_rs::Encoding;

use crate::{CoreError, Result};

/// Rust-native equivalent of Hutool's generic encoder contract.
pub trait Encoder<Input: ?Sized, Output> {
    /// Encodes `input` into the configured output representation.
    fn encode(&self, input: &Input) -> Result<Output>;
}

/// Rust-native equivalent of Hutool's generic decoder contract.
pub trait Decoder<Input: ?Sized, Output> {
    /// Decodes `input` into the configured output representation.
    fn decode(&self, input: &Input) -> Result<Output>;
}

/// Configurable lower- or uppercase Base16 codec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Base16Codec {
    lower_case: bool,
}

impl Base16Codec {
    /// Lowercase hexadecimal codec.
    pub const LOWER: Self = Self::new(true);
    /// Uppercase hexadecimal codec.
    pub const UPPER: Self = Self::new(false);

    /// Creates a codec with the requested alphabet case.
    #[must_use]
    pub const fn new(lower_case: bool) -> Self {
        Self { lower_case }
    }

    /// Encodes bytes as Base16 text.
    #[must_use]
    pub fn encode_bytes(self, input: &[u8]) -> String {
        let encoded = hex::encode(input);
        if self.lower_case {
            encoded
        } else {
            encoded.to_ascii_uppercase()
        }
    }

    /// Decodes Base16 after removing Unicode whitespace and padding an odd nibble.
    pub fn decode_text(self, input: &str) -> Result<Vec<u8>> {
        let mut cleaned: String = input
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect();
        if cleaned.len() % 2 != 0 {
            cleaned.insert(0, '0');
        }
        hex::decode(cleaned).map_err(Into::into)
    }

    /// Formats a Rust character as one or two Java-compatible UTF-16 escapes.
    #[must_use]
    pub fn to_unicode_hex(self, character: char) -> String {
        character
            .encode_utf16(&mut [0; 2])
            .iter()
            .map(|code_unit| {
                if self.lower_case {
                    format!("\\u{code_unit:04x}")
                } else {
                    format!("\\u{code_unit:04X}")
                }
            })
            .collect()
    }

    /// Appends one byte as two hexadecimal characters.
    pub fn append_hex(self, output: &mut String, byte: u8) {
        output.push_str(&self.encode_bytes(&[byte]));
    }
}

impl Encoder<[u8], String> for Base16Codec {
    fn encode(&self, input: &[u8]) -> Result<String> {
        Ok(self.encode_bytes(input))
    }
}

impl Decoder<str, Vec<u8>> for Base16Codec {
    fn decode(&self, input: &str) -> Result<Vec<u8>> {
        self.decode_text(input)
    }
}

/// Encodes Base64 with Hutool's multiline and URL-safe switches.
#[must_use]
pub fn base64_encode_config(input: &[u8], multiline: bool, url_safe: bool) -> String {
    let encoded = if url_safe {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
    } else {
        base64::engine::general_purpose::STANDARD.encode(input)
    };
    if !multiline || encoded.len() <= 76 {
        return encoded;
    }
    let mut output = String::with_capacity(encoded.len() + encoded.len() / 76 * 2);
    for (index, character) in encoded.chars().enumerate() {
        if index > 0 && index % 76 == 0 {
            output.push_str("\r\n");
        }
        output.push(character);
    }
    output
}

/// Encodes standard Base64 without padding.
#[must_use]
pub fn base64_encode_without_padding(input: impl AsRef<[u8]>) -> String {
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(input)
}

/// Decodes both Base64 alphabets while ignoring padding and unrelated bytes like Hutool.
#[must_use]
pub fn base64_decode_tolerant(input: impl AsRef<[u8]>) -> Vec<u8> {
    let sextets: Vec<u8> = input
        .as_ref()
        .iter()
        .filter_map(|byte| base64_sextet(*byte))
        .collect();
    let mut output = Vec::with_capacity(sextets.len().saturating_mul(3) / 4);
    for chunk in sextets.chunks(4) {
        if chunk.len() >= 2 {
            output.push((chunk[0] << 2) | (chunk[1] >> 4));
        }
        if chunk.len() >= 3 {
            output.push((chunk[1] << 4) | (chunk[2] >> 2));
        }
        if chunk.len() == 4 {
            output.push((chunk[2] << 6) | chunk[3]);
        }
    }
    output
}

/// Decodes a checked byte range with Hutool's tolerant alphabet rules.
pub fn base64_decode_range_tolerant(
    input: &[u8],
    position: usize,
    length: usize,
) -> Result<Vec<u8>> {
    let end = position
        .checked_add(length)
        .filter(|end| *end <= input.len())
        .ok_or_else(|| CoreError::Codec("Base64 range is out of bounds".into()))?;
    Ok(base64_decode_tolerant(&input[position..end]))
}

/// Returns whether one byte is accepted by Hutool's mixed Base64 alphabet.
#[must_use]
pub const fn is_base64_code(byte: u8) -> bool {
    matches!(
        byte,
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'-' | b'/' | b'_' | b'='
    )
}

/// Performs Hutool's structural Base64 check, including its permissive whitespace rules.
#[must_use]
pub fn is_base64(input: impl AsRef<[u8]>) -> bool {
    let input = input.as_ref();
    if input.len() < 3 || !input.is_ascii() {
        return false;
    }
    let mut padding = false;
    for byte in input {
        if padding {
            if *byte != b'=' {
                return false;
            }
        } else if *byte == b'=' {
            padding = true;
        } else if !is_base64_code(*byte) && !matches!(*byte, b' ' | b'\n' | b'\r' | b'\t') {
            return false;
        }
    }
    true
}

/// Resolves a WHATWG/IANA-compatible character encoding label.
pub fn encoding_for_label(label: &str) -> Result<&'static Encoding> {
    Encoding::for_label(label.as_bytes())
        .ok_or_else(|| CoreError::Codec(format!("unknown character encoding: {label}")))
}

/// Encodes text to the selected character encoding before Base64 conversion.
#[must_use]
pub fn base64_encode_text(input: &str, encoding: &'static Encoding, url_safe: bool) -> String {
    let (bytes, _, _) = encoding.encode(input);
    base64_encode_config(&bytes, false, url_safe)
}

/// Decodes tolerant Base64 and then converts bytes with the selected character encoding.
#[must_use]
pub fn base64_decode_text(input: &str, encoding: &'static Encoding) -> String {
    let bytes = base64_decode_tolerant(input);
    encoding.decode(&bytes).0.into_owned()
}

/// Reads an entire stream and encodes it as Base64.
pub fn base64_encode_reader(
    mut reader: impl Read,
    multiline: bool,
    url_safe: bool,
) -> Result<String> {
    let mut input = Vec::new();
    reader.read_to_end(&mut input)?;
    Ok(base64_encode_config(&input, multiline, url_safe))
}

/// Reads a file and encodes it as Base64.
pub fn base64_encode_file(path: impl AsRef<Path>, url_safe: bool) -> Result<String> {
    base64_encode_reader(std::fs::File::open(path)?, false, url_safe)
}

/// Decodes tolerant Base64 into a writer and returns the byte count.
pub fn base64_decode_to_writer(input: &str, mut writer: impl Write) -> Result<usize> {
    let decoded = base64_decode_tolerant(input);
    writer.write_all(&decoded)?;
    Ok(decoded.len())
}

/// Decodes tolerant Base64 into a file and returns the byte count.
pub fn base64_decode_to_file(input: &str, path: impl AsRef<Path>) -> Result<usize> {
    base64_decode_to_writer(input, std::fs::File::create(path)?)
}

/// Configurable percent encoder matching Hutool's mutable safe-character model.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PercentCodec {
    safe_characters: BTreeSet<char>,
    encode_space_as_plus: bool,
}

impl PercentCodec {
    /// Creates a codec with no predefined safe characters, matching Hutool's constructor.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            safe_characters: BTreeSet::new(),
            encode_space_as_plus: false,
        }
    }

    /// Creates a codec whose initial safe set is `characters`.
    #[must_use]
    pub fn with_safe(characters: impl IntoIterator<Item = char>) -> Self {
        Self {
            safe_characters: characters.into_iter().collect(),
            encode_space_as_plus: false,
        }
    }

    /// Adds one safe character.
    pub fn add_safe(&mut self, character: char) -> &mut Self {
        self.safe_characters.insert(character);
        self
    }

    /// Removes one safe character.
    pub fn remove_safe(&mut self, character: char) -> &mut Self {
        self.safe_characters.remove(&character);
        self
    }

    /// Adds another codec's safe characters to this codec.
    pub fn union(&mut self, other: &Self) -> &mut Self {
        self.safe_characters
            .extend(other.safe_characters.iter().copied());
        self
    }

    /// Returns a cloned codec containing the union of both safe sets.
    #[must_use]
    pub fn union_new(&self, other: &Self) -> Self {
        let mut combined = self.clone();
        combined.union(other);
        combined
    }

    /// Selects `+` instead of `%20` for spaces.
    pub fn set_encode_space_as_plus(&mut self, enabled: bool) -> &mut Self {
        self.encode_space_as_plus = enabled;
        self
    }

    /// Percent-encodes text using a selected character encoding and per-call safe characters.
    #[must_use]
    pub fn encode(&self, input: &str, encoding: &'static Encoding, custom_safe: &[char]) -> String {
        let mut output = String::with_capacity(input.len());
        for character in input.chars() {
            if self.safe_characters.contains(&character) || custom_safe.contains(&character) {
                output.push(character);
            } else if self.encode_space_as_plus && character == ' ' {
                output.push('+');
            } else {
                let value = character.to_string();
                let (bytes, _, _) = encoding.encode(&value);
                for byte in bytes.as_ref() {
                    const HEX: &[u8; 16] = b"0123456789ABCDEF";
                    output.push('%');
                    output.push(char::from(HEX[usize::from(byte >> 4)]));
                    output.push(char::from(HEX[usize::from(byte & 0x0f)]));
                }
            }
        }
        output
    }
}

fn base64_sextet(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' | b'-' => Some(62),
        b'/' | b'_' => Some(63),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use encoding_rs::{GBK, UTF_8};

    use super::*;

    #[test]
    fn base16_matches_hutool_whitespace_odd_length_and_unicode_rules() {
        assert_eq!(Base16Codec::LOWER.encode_bytes(&[0xab, 0xcd]), "abcd");
        assert_eq!(Base16Codec::UPPER.encode_bytes(&[0xab, 0xcd]), "ABCD");
        assert_eq!(
            Base16Codec::LOWER.decode_text(" A B C ").unwrap(),
            [0x0a, 0xbc]
        );
        assert_eq!(Base16Codec::LOWER.to_unicode_hex('你'), "\\u4f60");
        assert_eq!(Base16Codec::UPPER.to_unicode_hex('🦀'), "\\uD83E\\uDD80");
        let mut output = String::new();
        Base16Codec::UPPER.append_hex(&mut output, 0xaf);
        assert_eq!(output, "AF");
        assert!(Base16Codec::LOWER.decode_text("xyz").is_err());
        assert_eq!(Encoder::encode(&Base16Codec::LOWER, b"Hi").unwrap(), "4869");
        assert_eq!(Decoder::decode(&Base16Codec::LOWER, "4869").unwrap(), b"Hi");
    }

    #[test]
    fn base64_supports_hutool_variants_and_tolerant_decoding() {
        assert_eq!(
            base64_encode_config(b"hello world", false, false),
            "aGVsbG8gd29ybGQ="
        );
        assert_eq!(
            base64_encode_without_padding(b"hello world"),
            "aGVsbG8gd29ybGQ"
        );
        assert_eq!(
            base64_decode_tolerant("aG!!V sbG8gd29ybGQ=="),
            b"hello world"
        );
        assert_eq!(
            base64_decode_range_tolerant(b"xxaGk=yy", 2, 4).unwrap(),
            b"hi"
        );
        assert!(base64_decode_range_tolerant(b"abc", 2, 2).is_err());

        let multiline = base64_encode_config(&[42; 60], true, false);
        assert_eq!(multiline.as_bytes()[76..78], *b"\r\n");
        let url_safe = base64_encode_config(&[0xfb, 0xff], false, true);
        assert_eq!(url_safe, "-_8");
        assert!(is_base64("aGk="));
        assert!(is_base64("aG\nk="));
        assert!(!is_base64("a=Gk"));
        assert!(!is_base64("短"));
        assert!(is_base64_code(b'_'));
        assert!(!is_base64_code(b'!'));

        let hutool = "伦家是一个非常长的字符串66";
        let expected = "5Lym5a625piv5LiA5Liq6Z2e5bi46ZW/55qE5a2X56ym5LiyNjY=";
        assert_eq!(base64_encode_text(hutool, UTF_8, false), expected);
        assert_eq!(
            base64_decode_text(expected.trim_end_matches('='), UTF_8),
            hutool
        );
    }

    #[test]
    fn base64_handles_character_encodings_readers_writers_and_files() {
        let gbk_text =
            "订购成功立即生效，30天内可观看专区中除单独计费影片外的所有内容，到期自动取消。";
        let gbk = base64_encode_text(gbk_text, GBK, false);
        assert_eq!(base64_decode_text(&gbk, GBK), gbk_text);
        assert_eq!(encoding_for_label("utf-8").unwrap(), UTF_8);
        assert!(encoding_for_label("not-an-encoding").is_err());
        assert_eq!(
            base64_encode_reader(Cursor::new(b"stream"), false, false).unwrap(),
            "c3RyZWFt"
        );

        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.bin");
        let target = directory.path().join("target.bin");
        std::fs::write(&source, b"file").unwrap();
        let encoded = base64_encode_file(&source, false).unwrap();
        assert_eq!(base64_decode_to_file(&encoded, &target).unwrap(), 4);
        assert_eq!(std::fs::read(target).unwrap(), b"file");
        let mut output = Vec::new();
        assert_eq!(base64_decode_to_writer("aGk=", &mut output).unwrap(), 2);
        assert_eq!(output, b"hi");
    }

    #[test]
    fn configurable_percent_codec_matches_hutool_safe_sets_and_plus_mode() {
        let mut alphanumeric = PercentCodec::with_safe('a'..='z');
        alphanumeric.add_safe('/').set_encode_space_as_plus(true);
        assert_eq!(alphanumeric.encode("abc/ 你", UTF_8, &[]), "abc/+%E4%BD%A0");
        alphanumeric.remove_safe('/');
        assert_eq!(alphanumeric.encode("a/b", UTF_8, &['/']), "a/b");

        let punctuation = PercentCodec::with_safe(['-', '_']);
        let combined = alphanumeric.union_new(&punctuation);
        assert_eq!(combined.encode("a-_", UTF_8, &[]), "a-_");
        assert_eq!(PercentCodec::new().encode("你", GBK, &[]), "%C4%E3");
    }
}
