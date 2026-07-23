use std::collections::HashSet;

use thiserror::Error;

const MAX_RADIX: usize = 256;

/// Errors returned by custom-radix conversion.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RadixError {
    /// At least two alphabet characters are required.
    #[error("a radix alphabet must contain at least two characters")]
    AlphabetTooShort,
    /// Repeated characters make decoding ambiguous.
    #[error("duplicate character `{0}` in radix alphabet")]
    DuplicateCharacter(char),
    /// Bounded alphabets prevent pathological allocation and arithmetic.
    #[error("radix alphabet exceeds the maximum of 256 characters")]
    AlphabetTooLarge,
    /// Hutool's long overload does not accept negative numbers.
    #[error("negative i64 values are not supported")]
    NegativeI64,
    /// An encoded value must contain at least one character.
    #[error("encoded value must not be empty")]
    EmptyInput,
    /// The value contains a character outside the selected alphabet.
    #[error("character `{0}` does not belong to the radix alphabet")]
    InvalidCharacter(char),
    /// The decoded value cannot be represented as an `i64`.
    #[error("decoded value exceeds i64 range")]
    Overflow,
}

/// Arbitrary-alphabet integer conversion compatible with Hutool's `RadixUtil`.
#[derive(Debug, Clone, Copy, Default)]
pub struct RadixUtil;

impl RadixUtil {
    /// Ordered base-34 alphabet without `I` or `O`.
    pub const RADIXS_34: &'static str = "0123456789ABCDEFGHJKLMNPQRSTUVWXYZ";
    /// Hutool's shuffled base-34 alphabet.
    pub const RADIXS_SHUFFLE_34: &'static str = "H3UM16TDFPSBZJ90CW28QYRE45AXKNGV7L";
    /// Ordered base-59 alphabet without `I`, `O`, or lowercase `l`.
    pub const RADIXS_59: &'static str =
        "0123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
    /// Hutool's shuffled base-59 alphabet.
    pub const RADIXS_SHUFFLE_59: &'static str =
        "vh9wGkfK8YmqbsoENP3764SeCX0dVzrgy1HRtpnTaLjJW2xQiZAcBMUFDu5";

    /// Encodes an `i32`; negative values use their unsigned 32-bit representation.
    pub fn encode_i32(alphabet: &str, value: i32) -> Result<String, RadixError> {
        let unsigned = u32::from_ne_bytes(value.to_ne_bytes());
        encode_unsigned(alphabet, u64::from(unsigned))
    }

    /// Encodes a non-negative `i64`.
    pub fn encode_i64(alphabet: &str, value: i64) -> Result<String, RadixError> {
        if value < 0 {
            return Err(RadixError::NegativeI64);
        }
        encode_unsigned(alphabet, value.unsigned_abs())
    }

    /// Decodes into an `i64` with checked arithmetic.
    pub fn decode(alphabet: &str, encoded: &str) -> Result<i64, RadixError> {
        let alphabet = validate_alphabet(alphabet)?;
        if encoded.is_empty() {
            return Err(RadixError::EmptyInput);
        }
        #[allow(clippy::cast_possible_wrap)]
        let radix = alphabet.len() as i64;
        encoded.chars().try_fold(0_i64, |result, character| {
            let digit = alphabet
                .iter()
                .position(|candidate| *candidate == character)
                .ok_or(RadixError::InvalidCharacter(character))?;
            #[allow(clippy::cast_possible_wrap)]
            let digit = digit as i64;
            result
                .checked_mul(radix)
                .and_then(|value| value.checked_add(digit))
                .ok_or(RadixError::Overflow)
        })
    }

    /// Decodes and applies Java's narrowing `long`-to-`int` conversion.
    pub fn decode_to_i32(alphabet: &str, encoded: &str) -> Result<i32, RadixError> {
        Self::decode(alphabet, encoded).map(|value| {
            let bytes = value.to_le_bytes();
            i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        })
    }
}

fn encode_unsigned(alphabet: &str, mut value: u64) -> Result<String, RadixError> {
    let alphabet = validate_alphabet(alphabet)?;
    #[allow(clippy::cast_possible_truncation)]
    let radix = alphabet.len() as u64;
    let mut encoded = Vec::new();
    loop {
        #[allow(clippy::cast_possible_truncation)]
        let index = (value % radix) as usize;
        encoded.push(alphabet[index]);
        value /= radix;
        if value == 0 {
            break;
        }
    }
    Ok(encoded.into_iter().rev().collect())
}

fn validate_alphabet(alphabet: &str) -> Result<Vec<char>, RadixError> {
    let characters: Vec<char> = alphabet.chars().collect();
    if characters.len() < 2 {
        return Err(RadixError::AlphabetTooShort);
    }
    if characters.len() > MAX_RADIX {
        return Err(RadixError::AlphabetTooLarge);
    }
    let mut unique = HashSet::with_capacity(characters.len());
    if let Some(duplicate) = characters
        .iter()
        .copied()
        .find(|character| !unique.insert(*character))
    {
        return Err(RadixError::DuplicateCharacter(duplicate));
    }
    Ok(characters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_overloads_round_trip_zero_positive_negative_and_unicode_alphabets() {
        assert_eq!(RadixUtil::encode_i32("AB", 10).unwrap(), "BABA");
        assert_eq!(RadixUtil::encode_i32("VIP", 21).unwrap(), "PIV");
        assert_eq!(RadixUtil::encode_i64(RadixUtil::RADIXS_34, 0).unwrap(), "0");
        let encoded = RadixUtil::encode_i64(RadixUtil::RADIXS_SHUFFLE_59, i64::MAX).unwrap();
        assert_eq!(
            RadixUtil::decode(RadixUtil::RADIXS_SHUFFLE_59, &encoded).unwrap(),
            i64::MAX
        );
        let negative = RadixUtil::encode_i32("01", -1).unwrap();
        assert_eq!(negative, "11111111111111111111111111111111");
        assert_eq!(RadixUtil::decode_to_i32("01", &negative).unwrap(), -1);
        let unicode = RadixUtil::encode_i64("零一二", 21).unwrap();
        assert_eq!(unicode, "二一零");
        assert_eq!(RadixUtil::decode("零一二", &unicode).unwrap(), 21);
        assert_ne!(RadixUtil::RADIXS_59, RadixUtil::RADIXS_SHUFFLE_59);
    }

    #[test]
    fn validation_and_checked_decoding_report_every_invalid_shape() {
        assert_eq!(
            RadixUtil::encode_i32("", 1),
            Err(RadixError::AlphabetTooShort)
        );
        assert_eq!(
            RadixUtil::encode_i32("001", 1),
            Err(RadixError::DuplicateCharacter('0'))
        );
        let oversized: String = (0..=MAX_RADIX)
            .map(|offset| char::from_u32(0x1000 + u32::try_from(offset).unwrap()).unwrap())
            .collect();
        assert_eq!(
            RadixUtil::encode_i32(&oversized, 1),
            Err(RadixError::AlphabetTooLarge)
        );
        assert_eq!(
            RadixUtil::encode_i64("01", -1),
            Err(RadixError::NegativeI64)
        );
        assert_eq!(RadixUtil::decode("01", ""), Err(RadixError::EmptyInput));
        assert_eq!(
            RadixUtil::decode("", "0"),
            Err(RadixError::AlphabetTooShort)
        );
        assert_eq!(
            RadixUtil::decode("0123456789ABC", "1X3"),
            Err(RadixError::InvalidCharacter('X'))
        );
        assert_eq!(
            RadixUtil::decode(
                "01",
                "1111111111111111111111111111111111111111111111111111111111111111"
            ),
            Err(RadixError::Overflow)
        );
    }
}
