//! Convenient hexadecimal operations aligned with Hutool's `HexUtil` family.

use std::num::ParseIntError;

use encoding_rs::{Encoding, UTF_8, UTF_16BE, UTF_16LE};
use num_bigint::BigInt;

use crate::{Base16Codec, CoreError};

/// Errors produced by [`HexUtil`] conversions.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum HexUtilError {
    /// Base16 decoding failed.
    #[error(transparent)]
    Decode(#[from] CoreError),

    /// A fixed-width integer could not be parsed.
    #[error("invalid hexadecimal integer: {0}")]
    Integer(#[from] ParseIntError),

    /// An arbitrary-precision hexadecimal integer could not be parsed.
    #[error("invalid hexadecimal big integer: {0}")]
    BigInteger(String),

    /// A Java-style color literal was malformed or out of range.
    #[error("invalid color literal: {0}")]
    Color(String),
}

/// An RGB color whose channels are always in the valid `0..=255` range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbColor {
    /// Red channel.
    pub red: u8,
    /// Green channel.
    pub green: u8,
    /// Blue channel.
    pub blue: u8,
}

impl RgbColor {
    /// Creates a color from its red, green, and blue channels.
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

/// Zero-sized facade for Hutool-style hexadecimal operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct HexUtil;

impl HexUtil {
    /// Returns whether `value` is a non-negative hexadecimal number.
    #[must_use]
    pub fn is_hex_number(value: &str) -> bool {
        Self::is_hex_number_option(Some(value))
    }

    /// Nullable variant matching Hutool's `null -> false` behavior.
    #[must_use]
    pub fn is_hex_number_option(value: Option<&str>) -> bool {
        let Some(value) = value.filter(|value| !value.is_empty()) else {
            return false;
        };
        if value.starts_with('-') {
            return false;
        }
        let value = value
            .strip_prefix("0x")
            .or_else(|| value.strip_prefix("0X"))
            .or_else(|| value.strip_prefix('#'))
            .unwrap_or(value);
        !value.is_empty() && BigInt::parse_bytes(value.as_bytes(), 16).is_some()
    }

    /// Encodes bytes as lowercase hexadecimal text.
    #[must_use]
    pub fn encode_hex(data: impl AsRef<[u8]>) -> String {
        Base16Codec::LOWER.encode_bytes(data.as_ref())
    }

    /// Encodes bytes using the requested letter case.
    #[must_use]
    pub fn encode_hex_case(data: impl AsRef<[u8]>, lower_case: bool) -> String {
        Base16Codec::new(lower_case).encode_bytes(data.as_ref())
    }

    /// Encodes text with `encoding`, then converts the resulting bytes to lowercase hexadecimal.
    #[must_use]
    pub fn encode_hex_text(data: &str, encoding: &'static Encoding) -> String {
        Self::encode_hex(encode_text_bytes(data, encoding))
    }

    /// UTF-8 convenience variant of [`Self::encode_hex_text`].
    #[must_use]
    pub fn encode_hex_utf8(data: &str) -> String {
        Self::encode_hex_text(data, UTF_8)
    }

    /// Decodes hexadecimal text using Hutool's whitespace and odd-nibble rules.
    pub fn decode_hex(data: &str) -> Result<Vec<u8>, HexUtilError> {
        Ok(Base16Codec::LOWER.decode_text(data)?)
    }

    /// Decodes a Rust character slice as hexadecimal text.
    pub fn decode_hex_chars(data: &[char]) -> Result<Vec<u8>, HexUtilError> {
        Self::decode_hex(&data.iter().collect::<String>())
    }

    /// Decodes a Rust character slice and converts the bytes with `encoding`.
    pub fn decode_hex_chars_to_text(
        data: &[char],
        encoding: &'static Encoding,
    ) -> Result<String, HexUtilError> {
        Ok(encoding
            .decode(&Self::decode_hex_chars(data)?)
            .0
            .into_owned())
    }

    /// Decodes hexadecimal bytes and converts them to UTF-8 text.
    pub fn decode_hex_text(data: &str) -> Result<String, HexUtilError> {
        Self::decode_hex_text_with_encoding(data, UTF_8)
    }

    /// Decodes hexadecimal bytes and converts them with `encoding`.
    pub fn decode_hex_text_with_encoding(
        data: &str,
        encoding: &'static Encoding,
    ) -> Result<String, HexUtilError> {
        if data.is_empty() {
            return Ok(String::new());
        }
        Ok(encoding.decode(&Self::decode_hex(data)?).0.into_owned())
    }

    /// Encodes a color as lowercase `#rrggbb`.
    #[must_use]
    pub fn encode_color(color: RgbColor) -> String {
        Self::encode_color_with_prefix(color, "#")
    }

    /// Encodes a color with a custom prefix such as `#` or `0x`.
    #[must_use]
    pub fn encode_color_with_prefix(color: RgbColor, prefix: &str) -> String {
        format!(
            "{prefix}{:02x}{:02x}{:02x}",
            color.red, color.green, color.blue
        )
    }

    /// Decodes the same signed decimal, octal, `#`, and `0x` forms as Java `Color.decode`.
    pub fn decode_color(value: &str) -> Result<RgbColor, HexUtilError> {
        let value = decode_java_integer(value)?;
        let bytes = value.to_be_bytes();
        Ok(RgbColor::new(bytes[1], bytes[2], bytes[3]))
    }

    /// Formats an `i32` as a Java-compatible Unicode escape, padded to at least four digits.
    #[must_use]
    pub fn to_unicode_hex_i32(value: i32) -> String {
        let hex = Self::to_hex_i32(value);
        format!("\\u{hex:0>4}")
    }

    /// Formats a Rust character as one or two Java UTF-16 escapes.
    #[must_use]
    pub fn to_unicode_hex(character: char) -> String {
        Base16Codec::LOWER.to_unicode_hex(character)
    }

    /// Formats an `i32` with Java's two's-complement `Integer.toHexString` semantics.
    #[must_use]
    pub fn to_hex_i32(value: i32) -> String {
        format!("{:x}", u32::from_ne_bytes(value.to_ne_bytes()))
    }

    /// Parses a hexadecimal `i32` after applying Hutool's prefix-removal behavior.
    pub fn hex_to_i32(value: &str) -> Result<i32, HexUtilError> {
        Ok(i32::from_str_radix(remove_hex_prefix(value), 16)?)
    }

    /// Formats an `i64` with Java's two's-complement `Long.toHexString` semantics.
    #[must_use]
    pub fn to_hex_i64(value: i64) -> String {
        format!("{:x}", u64::from_ne_bytes(value.to_ne_bytes()))
    }

    /// Parses a hexadecimal `i64` after applying Hutool's prefix-removal behavior.
    pub fn hex_to_i64(value: &str) -> Result<i64, HexUtilError> {
        Ok(i64::from_str_radix(remove_hex_prefix(value), 16)?)
    }

    /// Formats Java-compatible `Float.floatToIntBits` bits as hexadecimal.
    #[must_use]
    pub fn to_hex_f32(value: f32) -> String {
        format!("{:x}", java_f32_bits(value))
    }

    /// Reconstructs an `f32` from its unsigned hexadecimal bit pattern.
    pub fn hex_to_f32(value: &str) -> Result<f32, HexUtilError> {
        Ok(f32::from_bits(u32::from_str_radix(
            remove_hex_prefix(value),
            16,
        )?))
    }

    /// Formats Java-compatible `Double.doubleToLongBits` bits as hexadecimal.
    #[must_use]
    pub fn to_hex_f64(value: f64) -> String {
        format!("{:x}", java_f64_bits(value))
    }

    /// Reconstructs an `f64` from its unsigned hexadecimal bit pattern.
    pub fn hex_to_f64(value: &str) -> Result<f64, HexUtilError> {
        Ok(f64::from_bits(u64::from_str_radix(
            remove_hex_prefix(value),
            16,
        )?))
    }

    /// Appends one byte as exactly two hexadecimal digits.
    pub fn append_hex(output: &mut String, byte: u8, lower_case: bool) {
        Base16Codec::new(lower_case).append_hex(output, byte);
    }

    /// Parses an optional arbitrary-precision hexadecimal integer.
    pub fn to_big_integer(value: Option<&str>) -> Result<Option<BigInt>, HexUtilError> {
        let Some(value) = value else {
            return Ok(None);
        };
        let value = remove_hex_prefix(value);
        BigInt::parse_bytes(value.as_bytes(), 16)
            .map(Some)
            .ok_or_else(|| HexUtilError::BigInteger(value.to_owned()))
    }

    /// Inserts a space after every pair of hexadecimal characters.
    #[must_use]
    pub fn format(data: &str) -> String {
        Self::format_with_prefix(data, "")
    }

    /// Prefixes every pair and inserts one space between pairs.
    #[must_use]
    pub fn format_with_prefix(data: &str, prefix: &str) -> String {
        if data.is_empty() {
            return String::new();
        }
        let mut output = String::with_capacity(data.len() + data.len() / 2);
        for (index, character) in data.chars().enumerate() {
            if index % 2 == 0 {
                if index != 0 {
                    output.push(' ');
                }
                output.push_str(prefix);
            }
            output.push(character);
        }
        output
    }
}

fn remove_hex_prefix(value: &str) -> &str {
    let bytes = value.as_bytes();
    if bytes.len() <= 1 {
        return value;
    }
    match bytes[0] {
        b'0' if matches!(bytes[1], b'x' | b'X') => &value[2..],
        // Hutool's Java switch intentionally falls through from `0` to `#`.
        b'0' | b'#' => &value[1..],
        _ => value,
    }
}

fn java_f32_bits(value: f32) -> u32 {
    if value.is_nan() {
        0x7fc0_0000
    } else {
        value.to_bits()
    }
}

fn java_f64_bits(value: f64) -> u64 {
    if value.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        value.to_bits()
    }
}

fn encode_text_bytes(data: &str, encoding: &'static Encoding) -> Vec<u8> {
    if std::ptr::eq(encoding, UTF_16BE) {
        data.encode_utf16()
            .flat_map(u16::to_be_bytes)
            .collect::<Vec<_>>()
    } else if std::ptr::eq(encoding, UTF_16LE) {
        data.encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>()
    } else {
        encoding.encode(data).0.into_owned()
    }
}

fn decode_java_integer(value: &str) -> Result<i32, HexUtilError> {
    let (negative, unsigned) = match value.as_bytes().first() {
        Some(b'-') => (true, &value[1..]),
        Some(b'+') => (false, &value[1..]),
        _ => (false, value),
    };
    let (radix, digits) = if let Some(digits) = unsigned
        .strip_prefix("0x")
        .or_else(|| unsigned.strip_prefix("0X"))
        .or_else(|| unsigned.strip_prefix('#'))
    {
        (16, digits)
    } else if unsigned.len() > 1 && unsigned.starts_with('0') {
        (8, &unsigned[1..])
    } else {
        (10, unsigned)
    };
    if digits.is_empty() || matches!(digits.as_bytes().first(), Some(b'+' | b'-')) {
        return Err(HexUtilError::Color(value.to_owned()));
    }
    let magnitude =
        i64::from_str_radix(digits, radix).map_err(|_| HexUtilError::Color(value.to_owned()))?;
    let signed = if negative { -magnitude } else { magnitude };
    i32::try_from(signed).map_err(|_| HexUtilError::Color(value.to_owned()))
}

#[cfg(test)]
mod tests {
    use encoding_rs::{GBK, UTF_16BE, UTF_16LE};

    use super::*;

    #[test]
    fn recognition_preserves_hutool_prefix_sign_and_big_integer_rules() {
        for value in ["0", "002c", "0x3544534F444", "#ff", "+ff", "#-1"] {
            assert!(HexUtil::is_hex_number(value), "{value}");
        }
        for value in ["", "-1", "0x", "0x10T0"] {
            assert!(!HexUtil::is_hex_number(value), "{value}");
        }
        assert!(!HexUtil::is_hex_number_option(None));
        assert_eq!(HexUtil::to_big_integer(None).unwrap(), None);
        assert_eq!(
            HexUtil::to_big_integer(Some("0xFF")).unwrap(),
            Some(BigInt::from(255_u16))
        );
        assert!(HexUtil::to_big_integer(Some("#nope")).is_err());
    }

    #[test]
    fn byte_and_text_facades_delegate_to_base16_and_character_engines() {
        assert_eq!(HexUtil::encode_hex([0xab, 0xcd]), "abcd");
        assert_eq!(HexUtil::encode_hex_case([0xab, 0xcd], false), "ABCD");
        assert_eq!(
            HexUtil::encode_hex_utf8("我是一个字符串"),
            "e68891e698afe4b880e4b8aae5ad97e7aca6e4b8b2"
        );
        assert_eq!(HexUtil::encode_hex_text("你", GBK), "c4e3");
        assert_eq!(HexUtil::encode_hex_text("烟", UTF_16BE), "70df");
        assert_eq!(HexUtil::encode_hex_text("烟", UTF_16LE), "df70");
        assert_eq!(HexUtil::decode_hex(" A B C ").unwrap(), [0x0a, 0xbc]);
        assert_eq!(
            HexUtil::decode_hex_chars(&['4', '8', '6', '9']).unwrap(),
            b"Hi"
        );
        assert_eq!(
            HexUtil::decode_hex_chars_to_text(&['7', '0', 'd', 'f'], UTF_16BE).unwrap(),
            "烟"
        );
        assert!(HexUtil::decode_hex_chars_to_text(&['x'], UTF_8).is_err());
        assert_eq!(HexUtil::decode_hex_text("36").unwrap(), "6");
        assert_eq!(
            HexUtil::decode_hex_text_with_encoding("c4e3", GBK).unwrap(),
            "你"
        );
        assert_eq!(HexUtil::decode_hex_text("").unwrap(), "");
        assert!(HexUtil::decode_hex("xyz").is_err());
        assert!(HexUtil::decode_hex_text("xyz").is_err());
    }

    #[test]
    fn color_facade_matches_java_decode_and_padded_encoding() {
        let color = RgbColor::new(1, 10, 255);
        assert_eq!(HexUtil::encode_color(color), "#010aff");
        assert_eq!(HexUtil::encode_color_with_prefix(color, "0x"), "0x010aff");
        assert_eq!(HexUtil::decode_color("#010aff").unwrap(), color);
        assert_eq!(HexUtil::decode_color("0x010aff").unwrap(), color);
        assert_eq!(HexUtil::decode_color("68351").unwrap(), color);
        assert_eq!(HexUtil::decode_color("0205377").unwrap(), color);
        assert_eq!(
            HexUtil::decode_color("-1").unwrap(),
            RgbColor::new(255, 255, 255)
        );
        assert_eq!(
            HexUtil::decode_color("+255").unwrap(),
            RgbColor::new(0, 0, 255)
        );
        for invalid in ["", "0x", "0x-1", "nope", "0x100000000"] {
            assert!(HexUtil::decode_color(invalid).is_err(), "{invalid}");
        }
    }

    #[test]
    fn numeric_conversions_preserve_java_bit_patterns_and_prefix_fallthrough() {
        assert_eq!(HexUtil::to_unicode_hex('\u{2001}'), "\\u2001");
        assert_eq!(HexUtil::to_unicode_hex('🦀'), "\\ud83e\\udd80");
        assert_eq!(HexUtil::to_unicode_hex_i32(1), "\\u0001");
        assert_eq!(HexUtil::to_unicode_hex_i32(-1), "\\uffffffff");
        assert_eq!(HexUtil::to_hex_i32(-1), "ffffffff");
        assert_eq!(HexUtil::hex_to_i32("#FF").unwrap(), 255);
        assert_eq!(HexUtil::hex_to_i32("f").unwrap(), 15);
        assert_eq!(HexUtil::hex_to_i32("002c").unwrap(), 44);
        assert!(HexUtil::hex_to_i32("0xFFFFFFFF").is_err());
        assert_eq!(HexUtil::to_hex_i64(-1), "ffffffffffffffff");
        assert_eq!(HexUtil::hex_to_i64("0XFF").unwrap(), 255);
        assert!(HexUtil::hex_to_i64("#nope").is_err());

        for value in [1.5_f32, -1.5, 1.234_567_9e-5] {
            assert_eq!(
                HexUtil::hex_to_f32(&HexUtil::to_hex_f32(value))
                    .unwrap()
                    .to_bits(),
                value.to_bits()
            );
        }
        for value in [
            1.5_f64,
            -1.5,
            std::f64::consts::PI,
            1.234_567_890_123_45e-10,
        ] {
            assert_eq!(
                HexUtil::hex_to_f64(&HexUtil::to_hex_f64(value))
                    .unwrap()
                    .to_bits(),
                value.to_bits()
            );
        }
        assert_eq!(HexUtil::to_hex_f32(f32::from_bits(0x7fa0_0001)), "7fc00000");
        assert_eq!(
            HexUtil::to_hex_f64(f64::from_bits(0x7ff0_0000_0000_0001)),
            "7ff8000000000000"
        );
        assert!(HexUtil::hex_to_f32("xyz").is_err());
        assert!(HexUtil::hex_to_f64("xyz").is_err());
    }

    #[test]
    fn append_and_pair_formatting_cover_empty_odd_and_prefix_forms() {
        let mut output = String::from("byte=");
        HexUtil::append_hex(&mut output, 0xaf, false);
        HexUtil::append_hex(&mut output, 0x01, true);
        assert_eq!(output, "byte=AF01");

        assert_eq!(HexUtil::format(""), "");
        assert_eq!(HexUtil::format("1"), "1");
        assert_eq!(HexUtil::format("123"), "12 3");
        assert_eq!(HexUtil::format_with_prefix("1", "0x"), "0x1");
        assert_eq!(HexUtil::format_with_prefix("123", "0x"), "0x12 0x3");
        assert_eq!(HexUtil::format_with_prefix("abcd", ""), "ab cd");
    }
}
