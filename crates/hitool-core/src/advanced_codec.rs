//! Hutool-aligned binary and text codecs with Rust-native error handling.

use data_encoding::{BASE32, BASE32HEX};
use idna::punycode;
use sha2::{Digest as _, Sha256};

use crate::{CoreError, Result};

const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BASE62_GMP: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const BASE62_INVERTED: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CAESAR_TABLE: &[u8] = b"AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz";
const HASHIDS_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const HASHIDS_SEPARATORS: &str = "cfhistuCFHISTU";

/// Encodes bytes as padded RFC 4648 Base32.
#[must_use]
pub fn base32_encode(input: impl AsRef<[u8]>) -> String {
    BASE32.encode(input.as_ref())
}

/// Decodes case-insensitive padded RFC 4648 Base32.
pub fn base32_decode(input: &str) -> Result<Vec<u8>> {
    BASE32
        .decode(input.to_ascii_uppercase().as_bytes())
        .map_err(|error| CoreError::Codec(error.to_string()))
}

/// Encodes bytes with the RFC 4648 Extended Hex Base32 alphabet.
#[must_use]
pub fn base32_hex_encode(input: impl AsRef<[u8]>) -> String {
    BASE32HEX.encode(input.as_ref())
}

/// Decodes case-insensitive RFC 4648 Extended Hex Base32.
pub fn base32_hex_decode(input: &str) -> Result<Vec<u8>> {
    BASE32HEX
        .decode(input.to_ascii_uppercase().as_bytes())
        .map_err(|error| CoreError::Codec(error.to_string()))
}

/// Encodes bytes with the Bitcoin Base58 alphabet.
#[must_use]
pub fn base58_encode(input: impl AsRef<[u8]>) -> String {
    translate_digits(&convert_base(input.as_ref(), 256, 58), BASE58_ALPHABET)
}

/// Decodes Bitcoin-alphabet Base58 while preserving leading zero bytes.
pub fn base58_decode(input: &str) -> Result<Vec<u8>> {
    decode_radix(input, BASE58_ALPHABET, 58)
}

/// Encodes a `Base58Check` payload using Hutool's optional one-byte version.
#[must_use]
pub fn base58_encode_checked(version: Option<u8>, payload: &[u8]) -> String {
    let mut bytes = Vec::with_capacity(payload.len() + 5);
    if let Some(version) = version {
        bytes.push(version);
    }
    bytes.extend_from_slice(payload);
    bytes.extend_from_slice(&double_sha256(payload)[..4]);
    base58_encode(bytes)
}

/// Decodes `Base58Check` and removes an optional one-byte version.
pub fn base58_decode_checked(input: &str, with_version: bool) -> Result<Vec<u8>> {
    let decoded = base58_decode(input)?;
    let minimum = if with_version { 5 } else { 4 };
    if decoded.len() < minimum {
        return Err(CoreError::Codec("Base58Check payload is too short".into()));
    }
    let payload_start = usize::from(with_version);
    let checksum_start = decoded.len() - 4;
    let payload = &decoded[payload_start..checksum_start];
    if decoded[checksum_start..] != double_sha256(payload)[..4] {
        return Err(CoreError::Codec("Base58 checksum is invalid".into()));
    }
    Ok(payload.to_vec())
}

/// Decodes `Base58Check`, accepting either versioned or unversioned form.
pub fn base58_decode_checked_auto(input: &str) -> Result<Vec<u8>> {
    base58_decode_checked(input, true).or_else(|_| base58_decode_checked(input, false))
}

/// Encodes arbitrary bytes using Hutool's GMP-style Base62 alphabet.
#[must_use]
pub fn base62_encode(input: impl AsRef<[u8]>) -> String {
    base62_encode_with_alphabet(input.as_ref(), BASE62_GMP)
}

/// Decodes Hutool's GMP-style Base62.
pub fn base62_decode(input: &str) -> Result<Vec<u8>> {
    decode_radix(input, BASE62_GMP, 62)
}

/// Encodes arbitrary bytes using Hutool's case-inverted Base62 alphabet.
#[must_use]
pub fn base62_inverted_encode(input: impl AsRef<[u8]>) -> String {
    base62_encode_with_alphabet(input.as_ref(), BASE62_INVERTED)
}

/// Decodes Hutool's case-inverted Base62.
pub fn base62_inverted_decode(input: &str) -> Result<Vec<u8>> {
    decode_radix(input, BASE62_INVERTED, 62)
}

/// Applies an ASCII ROT-N transform to letters and optionally digits.
#[must_use]
pub fn rot_encode(input: &str, offset: i32, rotate_digits: bool) -> String {
    input
        .chars()
        .map(|character| rotate_ascii(character, offset, rotate_digits))
        .collect()
}

/// Reverses [`rot_encode`].
#[must_use]
pub fn rot_decode(input: &str, offset: i32, rotate_digits: bool) -> String {
    rot_encode(input, -offset, rotate_digits)
}

/// Applies Hutool's interleaved-case Caesar wheel.
#[must_use]
pub fn caesar_encode(input: &str, offset: i32) -> String {
    caesar(input, offset)
}

/// Reverses [`caesar_encode`].
#[must_use]
pub fn caesar_decode(input: &str, offset: i32) -> String {
    caesar(input, -offset)
}

/// Packs hexadecimal ASCII digits into binary-coded nibbles.
pub fn bcd_encode(input: &str) -> Result<Vec<u8>> {
    let padded;
    let input = if input.len() % 2 == 0 {
        input
    } else {
        padded = format!("0{input}");
        &padded
    };
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = hex_nibble(pair[0])?;
            let low = hex_nibble(pair[1])?;
            Ok((high << 4) | low)
        })
        .collect()
}

/// Expands packed BCD/hexadecimal nibbles as uppercase ASCII.
#[must_use]
pub fn bcd_decode(input: &[u8]) -> String {
    const HEX: &[u8] = b"0123456789ABCDEF";
    let mut output = String::with_capacity(input.len() * 2);
    for byte in input {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

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

/// Encodes one Unicode label with RFC 3492 Punycode.
pub fn punycode_encode(input: &str) -> Result<String> {
    punycode_encode_prefixed(input, false)
}

/// Encodes one Unicode label and optionally prepends Hutool's `xn--` marker.
pub fn punycode_encode_prefixed(input: &str, with_prefix: bool) -> Result<String> {
    if input.is_ascii() {
        return Ok(input.to_owned());
    }
    let encoded = punycode::encode_str(input)
        .ok_or_else(|| CoreError::Codec("Punycode encode failed".into()))?;
    Ok(if with_prefix {
        format!("xn--{encoded}")
    } else {
        encoded
    })
}

/// Decodes one RFC 3492 Punycode label.
pub fn punycode_decode(input: &str) -> Result<String> {
    let input = input
        .get(..4)
        .filter(|prefix| prefix.eq_ignore_ascii_case("xn--"))
        .map_or(input, |_| &input[4..]);
    punycode::decode_to_string(input)
        .ok_or_else(|| CoreError::Codec("Punycode decode failed".into()))
}

/// Converts a complete internationalized domain name to ASCII.
pub fn idna_encode_domain(input: &str) -> Result<String> {
    input
        .split('.')
        .map(|label| punycode_encode_prefixed(label, true))
        .collect::<Result<Vec<_>>>()
        .map(|labels| labels.join("."))
}

/// Converts an ASCII-compatible domain name to Unicode.
pub fn idna_decode_domain(input: &str) -> Result<String> {
    input
        .split('.')
        .map(|label| {
            if label
                .get(..4)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("xn--"))
            {
                punycode_decode(label)
            } else {
                Ok(label.to_owned())
            }
        })
        .collect::<Result<Vec<_>>>()
        .map(|labels| labels.join("."))
}

/// Hashids-compatible reversible identifier codec.
#[derive(Debug, Clone)]
pub struct HashIds {
    alphabet: Vec<char>,
    separators: Vec<char>,
    salt: Vec<char>,
    guards: Vec<char>,
    minimum_length: usize,
}

impl HashIds {
    /// Builds a salted codec with an optional minimum output length.
    pub fn new(salt: impl AsRef<str>, minimum_length: usize) -> Result<Self> {
        Self::with_alphabet(salt, HASHIDS_ALPHABET, minimum_length)
    }

    /// Builds a salted codec with a custom alphabet.
    pub fn with_alphabet(
        salt: impl AsRef<str>,
        alphabet: impl AsRef<str>,
        minimum_length: usize,
    ) -> Result<Self> {
        let salt: Vec<char> = salt.as_ref().chars().collect();
        let original_alphabet: Vec<char> = alphabet.as_ref().chars().collect();
        if original_alphabet.contains(&' ') {
            return Err(CoreError::Codec(
                "Hashids alphabet must not contain spaces".into(),
            ));
        }

        let mut separators: Vec<char> = HASHIDS_SEPARATORS
            .chars()
            .filter(|separator| original_alphabet.contains(separator))
            .collect();
        hashids_shuffle(&mut separators, &salt);

        let mut alphabet = Vec::new();
        for character in original_alphabet.iter().copied() {
            if !separators.contains(&character) && !alphabet.contains(&character) {
                alphabet.push(character);
            }
        }
        if alphabet.len() < 16 {
            return Err(CoreError::Codec(
                "Hashids alphabet must contain at least 16 unique characters".into(),
            ));
        }

        if separators.is_empty() || alphabet.len() / separators.len() > 3 {
            let minimum_separators = alphabet.len().saturating_mul(2).div_ceil(7);
            if minimum_separators > separators.len() {
                let missing = minimum_separators - separators.len();
                separators.extend_from_slice(&alphabet[..missing]);
                alphabet.drain(..missing);
            }
        }
        hashids_shuffle(&mut alphabet, &salt);

        let guard_count = alphabet.len().div_ceil(12);
        let guards = alphabet.drain(..guard_count).collect();
        Ok(Self {
            alphabet,
            separators,
            salt,
            guards,
            minimum_length,
        })
    }

    /// Encodes one or more unsigned identifiers.
    #[must_use]
    pub fn encode(&self, values: &[u64]) -> String {
        self.encode_numbers(values)
    }

    /// Decodes identifiers and rejects invalid or non-canonical input.
    pub fn decode(&self, input: &str) -> Result<Vec<u64>> {
        self.decode_numbers(input)
    }

    /// Encodes an arbitrarily long hexadecimal value.
    pub fn encode_hex(&self, input: &str) -> Result<String> {
        let input = input
            .strip_prefix("0x")
            .or_else(|| input.strip_prefix("0X"))
            .unwrap_or(input);
        if !input.chars().all(|character| character.is_ascii_hexdigit()) {
            return Err(CoreError::Codec("Hashids hex input is invalid".into()));
        }
        let values = input
            .as_bytes()
            .chunks(12)
            .map(|chunk| {
                let chunk = std::str::from_utf8(chunk)
                    .map_err(|error| CoreError::Codec(error.to_string()))?;
                u64::from_str_radix(&format!("1{chunk}"), 16)
                    .map_err(|error| CoreError::Codec(error.to_string()))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(self.encode_numbers(&values))
    }

    /// Decodes a Hashids value back to lowercase hexadecimal.
    pub fn decode_hex(&self, input: &str) -> Result<String> {
        self.decode_numbers(input).map(|values| {
            values
                .iter()
                .map(|value| format!("{value:x}").chars().skip(1).collect::<String>())
                .collect()
        })
    }

    fn encode_numbers(&self, values: &[u64]) -> String {
        if values.is_empty() {
            return String::new();
        }
        let lottery_id = values
            .iter()
            .enumerate()
            .fold(0_u64, |state, (index, value)| {
                state.wrapping_add(value % (u64::try_from(index).expect("index fits u64") + 100))
            });
        let lottery = self.alphabet
            [usize::try_from(lottery_id % self.alphabet.len() as u64).expect("index fits usize")];
        let mut alphabet = self.alphabet.clone();
        let mut output = String::new();

        for (index, value) in values.iter().copied().enumerate() {
            hashids_derive_alphabet(&mut alphabet, &self.salt, lottery);
            let encoded = hashids_translate(value, &alphabet);
            let first_encoded = encoded.chars().next().expect("translation is non-empty");
            if index == 0 {
                output.push(lottery);
            }
            output.push_str(&encoded);
            if index + 1 < values.len() {
                let divisor =
                    u64::from(u32::from(if index == 0 { lottery } else { first_encoded })) + 1;
                let separator = self.separators[usize::try_from(
                    (value % divisor) % self.separators.len() as u64,
                )
                .expect("separator index fits usize")];
                output.push(separator);
            }
        }

        if self.minimum_length > output.chars().count() {
            let guard_index = usize::try_from(
                (lottery_id + u64::from(u32::from(lottery))) % self.guards.len() as u64,
            )
            .expect("guard index fits usize");
            output.insert(0, self.guards[guard_index]);
            if self.minimum_length > output.chars().count() {
                let third = output
                    .chars()
                    .nth(2)
                    .expect("guarded hash has three characters");
                let guard_index = usize::try_from(
                    (lottery_id + u64::from(u32::from(third))) % self.guards.len() as u64,
                )
                .expect("guard index fits usize");
                output.push(self.guards[guard_index]);
            }
        }

        while self.minimum_length > output.chars().count() {
            let salt = alphabet.clone();
            hashids_shuffle(&mut alphabet, &salt);
            let half = alphabet.len() / 2;
            let output_length = output.chars().count();
            let padding_left = self.minimum_length - output_length;
            let old: Vec<char> = output.chars().collect();
            if padding_left > alphabet.len() {
                let offset = half + usize::from(alphabet.len() % 2 != 0);
                output = alphabet[half..half + offset]
                    .iter()
                    .chain(old.iter())
                    .chain(alphabet[..half].iter())
                    .collect();
            } else {
                let excess = alphabet.len() + output_length - self.minimum_length;
                let second_start = half + excess / 2;
                let second_length = alphabet.len() - second_start;
                let first_length = padding_left - second_length;
                output = alphabet[second_start..]
                    .iter()
                    .chain(old.iter())
                    .chain(alphabet[..first_length].iter())
                    .collect();
            }
        }
        output
    }

    fn decode_numbers(&self, input: &str) -> Result<Vec<u64>> {
        if input.is_empty() {
            return Ok(Vec::new());
        }
        let input: Vec<char> = input.chars().collect();
        let guard_positions: Vec<usize> = input
            .iter()
            .enumerate()
            .filter_map(|(index, character)| self.guards.contains(character).then_some(index))
            .collect();
        let (start, end) = guard_positions.first().map_or((0, input.len()), |first| {
            (
                first + 1,
                guard_positions.get(1).copied().unwrap_or(input.len()),
            )
        });
        if start >= end {
            return Err(CoreError::Codec("invalid Hashids value".into()));
        }
        let lottery = input[start];
        let mut alphabet = self.alphabet.clone();
        let mut values = Vec::new();
        let mut block = Vec::new();
        for character in input[start + 1..end].iter().copied() {
            if self.separators.contains(&character) {
                if !block.is_empty() {
                    hashids_derive_alphabet(&mut alphabet, &self.salt, lottery);
                    values.push(hashids_untranslate(&block, &alphabet)?);
                    block.clear();
                }
            } else {
                block.push(character);
            }
        }
        if !block.is_empty() {
            hashids_derive_alphabet(&mut alphabet, &self.salt, lottery);
            values.push(hashids_untranslate(&block, &alphabet)?);
        }
        if self.encode_numbers(&values) != input.iter().collect::<String>() {
            return Err(CoreError::Codec("invalid Hashids value".into()));
        }
        Ok(values)
    }
}

fn hashids_shuffle(alphabet: &mut [char], salt: &[char]) {
    if salt.is_empty() {
        return;
    }
    let mut value = 0_usize;
    let mut sum = 0_u64;
    for index in (1..alphabet.len()).rev() {
        value %= salt.len();
        let code_point = u64::from(u32::from(salt[value]));
        sum = sum.wrapping_add(code_point);
        let target = usize::try_from(
            (code_point + value as u64 + sum) % u64::try_from(index).expect("index fits u64"),
        )
        .expect("shuffle index fits usize");
        alphabet.swap(index, target);
        value += 1;
    }
}

fn hashids_derive_alphabet(alphabet: &mut [char], salt: &[char], lottery: char) {
    let mut derived = Vec::with_capacity(alphabet.len());
    derived.push(lottery);
    derived.extend(salt.iter().copied().take(alphabet.len().saturating_sub(1)));
    derived.extend(
        alphabet
            .iter()
            .copied()
            .take(alphabet.len().saturating_sub(derived.len())),
    );
    hashids_shuffle(alphabet, &derived);
}

fn hashids_translate(mut value: u64, alphabet: &[char]) -> String {
    let radix = u64::try_from(alphabet.len()).expect("alphabet length fits u64");
    let mut encoded = Vec::new();
    loop {
        encoded.push(alphabet[usize::try_from(value % radix).expect("alphabet index fits usize")]);
        value /= radix;
        if value == 0 {
            break;
        }
    }
    encoded.into_iter().rev().collect()
}

fn hashids_untranslate(value: &[char], alphabet: &[char]) -> Result<u64> {
    let radix = u64::try_from(alphabet.len()).expect("alphabet length fits u64");
    value.iter().try_fold(0_u64, |number, character| {
        let digit = alphabet
            .iter()
            .position(|candidate| candidate == character)
            .ok_or_else(|| CoreError::Codec("invalid Hashids alphabet character".into()))?;
        number
            .checked_mul(radix)
            .and_then(|number| number.checked_add(digit as u64))
            .ok_or_else(|| CoreError::Codec("Hashids value overflows u64".into()))
    })
}

fn base62_encode_with_alphabet(input: &[u8], alphabet: &[u8]) -> String {
    translate_digits(&convert_base(input, 256, 62), alphabet)
}

pub(crate) fn decode_radix(input: &str, alphabet: &[u8], radix: u32) -> Result<Vec<u8>> {
    let digits = input
        .bytes()
        .enumerate()
        .map(|(index, byte)| {
            alphabet
                .iter()
                .position(|candidate| *candidate == byte)
                .and_then(|value| u8::try_from(value).ok())
                .ok_or_else(|| CoreError::Codec(format!("invalid radix character at byte {index}")))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(convert_base(&digits, radix, 256))
}

pub(crate) fn convert_base(message: &[u8], source_base: u32, target_base: u32) -> Vec<u8> {
    if message.is_empty() {
        return Vec::new();
    }
    let mut source = message.to_vec();
    let mut reversed = Vec::new();
    while !source.is_empty() {
        let mut quotient = Vec::with_capacity(source.len());
        let mut remainder = 0_u32;
        for byte in source {
            let accumulator = u32::from(byte) + remainder * source_base;
            let digit = accumulator / target_base;
            remainder = accumulator % target_base;
            if !quotient.is_empty() || digit > 0 {
                quotient.push(u8::try_from(digit).expect("base conversion digit is at most 255"));
            }
        }
        reversed.push(u8::try_from(remainder).expect("base conversion remainder is at most 255"));
        source = quotient;
    }
    reversed.extend(
        message
            .iter()
            .take(message.len().saturating_sub(1))
            .take_while(|byte| **byte == 0)
            .map(|_| 0),
    );
    reversed.reverse();
    reversed
}

pub(crate) fn translate_digits(digits: &[u8], alphabet: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(alphabet[usize::from(*digit)]))
        .collect()
}

fn double_sha256(input: &[u8]) -> [u8; 32] {
    let first = Sha256::digest(input);
    Sha256::digest(first).into()
}

fn rotate_ascii(character: char, offset: i32, rotate_digits: bool) -> char {
    match character {
        'A'..='Z' => char::from(
            b'A' + u8::try_from(
                (i32::try_from(u32::from(character) - u32::from('A'))
                    .expect("ASCII offset fits i32")
                    + offset)
                    .rem_euclid(26),
            )
            .expect("ROT letter is bounded"),
        ),
        'a'..='z' => char::from(
            b'a' + u8::try_from(
                (i32::try_from(u32::from(character) - u32::from('a'))
                    .expect("ASCII offset fits i32")
                    + offset)
                    .rem_euclid(26),
            )
            .expect("ROT letter is bounded"),
        ),
        '0'..='9' if rotate_digits => char::from(
            b'0' + u8::try_from(
                (i32::try_from(u32::from(character) - u32::from('0'))
                    .expect("ASCII offset fits i32")
                    + offset)
                    .rem_euclid(10),
            )
            .expect("ROT digit is bounded"),
        ),
        _ => character,
    }
}

fn caesar(input: &str, offset: i32) -> String {
    input
        .chars()
        .map(|character| {
            u8::try_from(character)
                .ok()
                .and_then(|byte| CAESAR_TABLE.iter().position(|candidate| *candidate == byte))
                .map_or(character, |position| {
                    let position = (i32::try_from(position).expect("Caesar table is small")
                        + offset)
                        .rem_euclid(52);
                    char::from(
                        CAESAR_TABLE
                            [usize::try_from(position).expect("Caesar position is positive")],
                    )
                })
        })
        .collect()
}

fn hex_nibble(byte: u8) -> Result<u8> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(CoreError::Codec(
            "BCD input must contain hexadecimal ASCII".into(),
        )),
    }
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

fn morse_character(bits: &str) -> Option<char> {
    const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,?'!/()&:;=+-_\"$@";
    CHARACTERS
        .chars()
        .find(|character| morse_bits(*character) == Some(bits))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base32_standard_and_hex_match_rfc_vectors() {
        assert_eq!(base32_encode(b"foobar"), "MZXW6YTBOI======");
        assert_eq!(base32_decode("mzxw6ytboi======").unwrap(), b"foobar");
        let encoded = base32_hex_encode(b"foobar");
        assert_eq!(base32_hex_decode(&encoded).unwrap(), b"foobar");
        assert!(base32_decode("***").is_err());
    }

    #[test]
    fn base58_and_check_preserve_leading_zeroes() {
        let input = [0, 0, 1, 2, 3, 255];
        assert_eq!(base58_decode(&base58_encode(input)).unwrap(), input);
        assert_eq!(base58_encode(b"hello world"), "StV1DL6CwTryKyV");
        let checked = base58_encode_checked(Some(0), b"hitool");
        assert_eq!(base58_decode_checked(&checked, true).unwrap(), b"hitool");
        assert_eq!(base58_decode_checked_auto(&checked).unwrap(), b"hitool");
        assert_eq!(
            base58_encode_checked(Some(0), b"hello world"),
            "13vQB7B6MrGQZaxCuFg4oh"
        );
        assert_eq!(
            base58_encode_checked(None, b"hello world"),
            "3vQB7B6MrGQZaxCuFg4oh"
        );
        assert!(base58_decode_checked("1", true).is_err());
        assert!(base58_decode_checked("3vQB7B6MrGQZaxCuFg4oi", false).is_err());
        assert!(base58_decode("0OIl").is_err());
        assert_eq!(base58_decode(&base58_encode([])).unwrap(), b"");
    }

    #[test]
    fn base62_both_alphabets_round_trip_binary() {
        let input = [0, 0, 1, 10, 100, 255];
        assert_eq!(base62_decode(&base62_encode(input)).unwrap(), input);
        assert_eq!(
            base62_inverted_decode(&base62_inverted_encode(input)).unwrap(),
            input
        );
        let hutool = "伦家是一个非常长的字符串66";
        assert_eq!(
            base62_encode(hutool),
            "17vKU8W4JMG8dQF8lk9VNnkdMOeWn4rJMva6F0XsLrrT53iKBnqo"
        );
        assert_eq!(
            base62_inverted_encode(hutool),
            "17Vku8w4jmg8Dqf8LK9vnNKDmoEwN4RjmVA6f0xSlRRt53IkbNQO"
        );
        assert!(base62_decode("+").is_err());
    }

    #[test]
    fn classical_codecs_are_reversible() {
        let value = "HiTool-工具-2026🦀";
        assert_eq!(rot_decode(&rot_encode(value, 13, true), 13, true), value);
        assert_eq!(caesar_decode(&caesar_encode(value, 7), 7), value);
        assert_eq!(rot_encode("Az-工具-09", 1, true), "Ba-工具-10");
        assert_eq!(
            rot_encode("1f2e9df6131b480b9fdddc633cf24996", 13, true),
            "4s5r2qs9464o713o2sqqqp966ps57229"
        );
        assert_eq!(caesar_encode("Zz-工具", 1), "zA-工具");
        assert_eq!(bcd_decode(&bcd_encode("123ABC").unwrap()), "123ABC");
        assert_eq!(bcd_decode(&bcd_encode("abcdef").unwrap()), "ABCDEF");
        assert_eq!(bcd_decode(&bcd_encode("123").unwrap()), "0123");
        assert!(bcd_encode("12Z3").is_err());
    }

    #[test]
    fn morse_handles_dictionary_custom_markers_and_unicode_fallback() {
        let codec = MorseCodec::default();
        let encoded = codec.encode("SOS你🦀");
        assert_eq!(codec.decode(&encoded).unwrap(), "SOS你🦀");
        let custom = MorseCodec::new('·', '—', '|').unwrap();
        assert_eq!(custom.decode(&custom.encode("A1")).unwrap(), "A1");
        assert!(MorseCodec::new('.', '.', '/').is_err());
        assert!(codec.decode("invalid").is_err());
        assert_eq!(
            codec.encode("Hello World!"),
            "...././.-../.-../---/-...../.--/---/.-./.-../-../-.-.--/"
        );
    }

    #[test]
    fn punycode_and_idna_round_trip() {
        let encoded = punycode_encode("你好").unwrap();
        assert_eq!(punycode_decode(&encoded).unwrap(), "你好");
        let domain = idna_encode_domain("你好.example").unwrap();
        assert_eq!(idna_decode_domain(&domain).unwrap(), "你好.example");
        assert_eq!(
            punycode_encode("Hutool编码器").unwrap(),
            "Hutool-ux9js33tgln"
        );
        assert_eq!(
            punycode_decode("xn--Hutool-ux9js33tgln").unwrap(),
            "Hutool编码器"
        );
        assert_eq!(punycode_encode("Hutool").unwrap(), "Hutool");
        assert_eq!(
            idna_encode_domain("赵新虎.com").unwrap(),
            "xn--efvz93e52e.com"
        );
    }

    #[test]
    fn hashids_support_numbers_hex_and_custom_alphabet() {
        let codec = HashIds::new("hitool", 12).unwrap();
        let encoded = codec.encode(&[1, 2, 3]);
        assert_eq!(codec.decode(&encoded).unwrap(), [1, 2, 3]);
        let hex = codec.encode_hex("deadbeef").unwrap();
        assert_eq!(codec.decode_hex(&hex).unwrap(), "deadbeef");
        let hutool = HashIds::new("my awesome salt", 0).unwrap();
        let expected = "R2qnd2vkOJTXm7XV7yq4";
        assert_eq!(
            hutool.encode_hex("507f1f77bcf86cd799439011").unwrap(),
            expected
        );
        assert_eq!(
            hutool.encode_hex("0X507f1f77bcf86cd799439011").unwrap(),
            expected
        );
        assert_eq!(
            hutool.decode_hex(expected).unwrap(),
            "507f1f77bcf86cd799439011"
        );
        assert!(hutool.encode_hex("not-hex").is_err());
        assert!(hutool.decode("invalid").is_err());
        assert_eq!(hutool.encode(&[]), "");
        assert_eq!(hutool.decode("").unwrap(), Vec::<u64>::new());

        let padded = HashIds::new("", 100).unwrap();
        let encoded = padded.encode(&[0, 1, 2, 3]);
        assert_eq!(encoded.chars().count(), 100);
        assert_eq!(padded.decode(&encoded).unwrap(), [0, 1, 2, 3]);

        let no_default_separators =
            HashIds::with_alphabet("salt", "abdegjklmnopqrvwxyz0123456789", 8).unwrap();
        let encoded = no_default_separators.encode(&[42, 9]);
        assert_eq!(no_default_separators.decode(&encoded).unwrap(), [42, 9]);
        assert!(HashIds::with_alphabet("salt", "abcd efghijklmnop", 0).is_err());
        assert!(HashIds::with_alphabet("salt", "short", 0).is_err());

        let guard_only: String = std::iter::once(hutool.guards[0]).collect();
        assert!(hutool.decode(&guard_only).is_err());
        assert!(hashids_untranslate(&['?'], &hutool.alphabet).is_err());
    }
}
