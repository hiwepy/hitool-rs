//! Hutool-aligned binary and text codecs with Rust-native error handling.

use data_encoding::{BASE32, BASE32HEX};
use idna::punycode;
use sha2::{Digest as _, Sha256};

use crate::{CoreError, Result};

/// Configurable Morse encoder compatible with Hutool's binary dictionary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorseCodec {
    dit: char,
    dah: char,
    separator: char,
}

impl MorseCodec {
    /// Creates a codec with custom dot, dash, and symbol separator characters.
    pub fn new(dit: char, dah: char, separator: char) -> Result<Self> {
        if dit == dah || dit == separator || dah == separator {
            return Err(CoreError::Codec(
                "Morse markers and separator must be distinct".into(),
            ));
        }
        Ok(Self {
            dit,
            dah,
            separator,
        })
    }

    /// Encodes Unicode text; unknown symbols use their binary code point.
    #[must_use]
    pub fn encode(self, input: &str) -> String {
        let mut output = String::new();
        for code_unit in input.to_uppercase().encode_utf16() {
            let bits = char::from_u32(u32::from(code_unit))
                .and_then(morse_bits)
                .map_or_else(|| format!("{code_unit:b}"), str::to_owned);
            for bit in bits.bytes() {
                output.push(if bit == b'0' { self.dit } else { self.dah });
            }
            output.push(self.separator);
        }
        output
    }

    /// Decodes Morse text and validates every input character.
    pub fn decode(self, input: &str) -> Result<String> {
        if input
            .chars()
            .any(|value| value != self.dit && value != self.dah && value != self.separator)
        {
            return Err(CoreError::Codec("incorrect Morse input".into()));
        }
        let mut output = Vec::new();
        for word in input.split(self.separator).filter(|word| !word.is_empty()) {
            let bits: String = word
                .chars()
                .map(|value| if value == self.dit { '0' } else { '1' })
                .collect();
            let code_unit = if let Some(character) = morse_character(&bits) {
                u16::try_from(u32::from(character))
                    .map_err(|_| CoreError::Codec("invalid Morse code point".into()))?
            } else {
                u16::from_str_radix(&bits, 2)
                    .map_err(|_| CoreError::Codec("invalid Morse code point".into()))?
            };
            output.push(code_unit);
        }
        String::from_utf16(&output).map_err(|error| CoreError::Codec(error.to_string()))
    }
}

impl Default for MorseCodec {
    fn default() -> Self {
        Self {
            dit: '.',
            dah: '-',
            separator: '/',
        }
    }
}

fn morse_character(bits: &str) -> Option<char> {
    const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,?'!/()&:;=+-_\"$@";
    CHARACTERS
        .chars()
        .find(|character| morse_bits(*character) == Some(bits))
}

fn morse_bits(character: char) -> Option<&'static str> {
    Some(match character {
        'A' => "01",
        'B' => "1000",
        'C' => "1010",
        'D' => "100",
        'E' => "0",
        'F' => "0010",
        'G' => "110",
        'H' => "0000",
        'I' => "00",
        'J' => "0111",
        'K' => "101",
        'L' => "0100",
        'M' => "11",
        'N' => "10",
        'O' => "111",
        'P' => "0110",
        'Q' => "1101",
        'R' => "010",
        'S' => "000",
        'T' => "1",
        'U' => "001",
        'V' => "0001",
        'W' => "011",
        'X' => "1001",
        'Y' => "1011",
        'Z' => "1100",
        '0' => "11111",
        '1' => "01111",
        '2' => "00111",
        '3' => "00011",
        '4' => "00001",
        '5' => "00000",
        '6' => "10000",
        '7' => "11000",
        '8' => "11100",
        '9' => "11110",
        '.' => "010101",
        ',' => "110011",
        '?' => "001100",
        '\'' => "011110",
        '!' => "101011",
        '/' => "10010",
        '(' => "10110",
        ')' => "101101",
        '&' => "01000",
        ':' => "111000",
        ';' => "101010",
        '=' => "10001",
        '+' => "01010",
        '-' => "100001",
        '_' => "001101",
        '"' => "010010",
        '$' => "0001001",
        '@' => "011010",
        _ => return None,
    })
}
