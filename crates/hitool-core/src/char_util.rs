use std::any::{Any, TypeId};

use unicode_general_category::{GeneralCategory, get_general_category};

/// Errors returned by checked character conversions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum CharError {
    /// Enclosed decimal numbers are defined only for 1 through 20.
    #[error("number must be in the inclusive range 1..=20")]
    InvalidEnclosedNumber,
}

/// Hutool-compatible ASCII, Unicode-category, and enclosed-character helpers.
pub struct CharUtil;

impl CharUtil {
    /// Returns whether `ch` is in the seven-bit ASCII range.
    #[must_use]
    pub const fn is_ascii(ch: char) -> bool {
        (ch as u32) < 128
    }

    /// Returns whether `ch` is a printable ASCII character.
    #[must_use]
    pub const fn is_ascii_printable(ch: char) -> bool {
        matches!(ch as u32, 32..=126)
    }

    /// Returns whether `ch` is an ASCII control character.
    #[must_use]
    pub const fn is_ascii_control(ch: char) -> bool {
        matches!(ch as u32, 0..=31 | 127)
    }

    /// Returns whether `ch` is an ASCII letter.
    #[must_use]
    pub const fn is_letter(ch: char) -> bool {
        Self::is_letter_upper(ch) || Self::is_letter_lower(ch)
    }

    /// Returns whether `ch` is an uppercase ASCII letter.
    #[must_use]
    pub const fn is_letter_upper(ch: char) -> bool {
        ch.is_ascii_uppercase()
    }

    /// Returns whether `ch` is a lowercase ASCII letter.
    #[must_use]
    pub const fn is_letter_lower(ch: char) -> bool {
        ch.is_ascii_lowercase()
    }

    /// Returns whether `ch` is an ASCII decimal digit.
    #[must_use]
    pub const fn is_number(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    /// Returns whether `ch` is an ASCII hexadecimal digit.
    #[must_use]
    pub const fn is_hex_char(ch: char) -> bool {
        Self::is_number(ch) || matches!(ch, 'a'..='f' | 'A'..='F')
    }

    /// Returns whether `ch` is an ASCII letter or digit.
    #[must_use]
    pub const fn is_letter_or_number(ch: char) -> bool {
        Self::is_letter(ch) || Self::is_number(ch)
    }

    /// Converts one Unicode scalar to an owned string.
    #[must_use]
    pub fn to_string(ch: char) -> String {
        ch.to_string()
    }

    /// Returns whether `T` is Rust's character type.
    #[must_use]
    pub fn is_char_class<T: 'static>() -> bool {
        TypeId::of::<T>() == TypeId::of::<char>()
    }

    /// Returns whether a dynamically typed value is a Rust character.
    #[must_use]
    pub fn is_char(value: &dyn Any) -> bool {
        value.is::<char>()
    }

    /// Returns Hutool's extended blank-character classification.
    #[must_use]
    pub fn is_blank_char(ch: char) -> bool {
        Self::is_blank_code(ch as u32)
    }

    /// Integer-code-point overload of [`Self::is_blank_char`].
    #[must_use]
    pub fn is_blank_code(code: u32) -> bool {
        matches!(
            code,
            0x0000 | 0xfeff | 0x202a | 0x3164 | 0x2800 | 0x200c | 0x180e
        ) || char::from_u32(code).is_some_and(|ch| ch != '\u{0085}' && ch.is_whitespace())
    }

    /// Applies Hutool's broad UTF-16-era emoji heuristic to a Rust scalar.
    #[must_use]
    pub const fn is_emoji(ch: char) -> bool {
        let code = ch as u32;
        !matches!(
            code,
            0x0000 | 0x0009 | 0x000a | 0x000d | 0x0020..=0xd7ff | 0xe000..=0xfffd | 0x0010_0000..=0x0010_ffff
        )
    }

    /// Returns whether `ch` is a Unix or Windows path separator.
    #[must_use]
    pub const fn is_file_separator(ch: char) -> bool {
        matches!(ch, '/' | '\\')
    }

    /// Compares two characters, optionally using one-to-one lowercase mapping.
    #[must_use]
    pub fn equals(left: char, right: char, case_insensitive: bool) -> bool {
        if case_insensitive {
            lower_char(left) == lower_char(right)
        } else {
            left == right
        }
    }

    /// Returns the numeric constant used by Java `Character.getType`.
    #[must_use]
    pub fn get_type(code: u32) -> i32 {
        if (0xd800..=0xdfff).contains(&code) {
            return 19;
        }
        let Some(ch) = char::from_u32(code) else {
            return 0;
        };
        java_category(get_general_category(ch))
    }

    /// Returns the hexadecimal value of a Unicode code point, or `-1`.
    #[must_use]
    pub fn digit16(code: u32) -> i32 {
        let Some(ch) = char::from_u32(code) else {
            return -1;
        };
        if get_general_category(ch) == GeneralCategory::DecimalNumber {
            let mut start = code;
            while start > 0
                && char::from_u32(start - 1).is_some_and(|previous| {
                    get_general_category(previous) == GeneralCategory::DecimalNumber
                })
            {
                start -= 1;
            }
            return i32::try_from((code - start) % 10).unwrap_or(-1);
        }
        match ch {
            'A'..='F' => 10 + i32::from(ch as u8 - b'A'),
            'a'..='f' => 10 + i32::from(ch as u8 - b'a'),
            'Ａ'..='Ｆ' => 10 + (ch as i32 - 'Ａ' as i32),
            'ａ'..='ｆ' => 10 + (ch as i32 - 'ａ' as i32),
            _ => -1,
        }
    }

    /// Converts ASCII letters and digits 1 through 9 to enclosed characters.
    #[must_use]
    pub fn to_close_char(ch: char) -> char {
        let code = match ch {
            '1'..='9' => '①' as u32 + ch as u32 - '1' as u32,
            'A'..='Z' => 'Ⓐ' as u32 + ch as u32 - 'A' as u32,
            'a'..='z' => 'ⓐ' as u32 + ch as u32 - 'a' as u32,
            _ => return ch,
        };
        char::from_u32(code).unwrap_or(ch)
    }

    /// Converts a number in `1..=20` to its enclosed decimal form.
    #[allow(clippy::cast_sign_loss)]
    pub fn to_close_by_number(number: i32) -> Result<char, CharError> {
        if !(1..=20).contains(&number) {
            return Err(CharError::InvalidEnclosedNumber);
        }
        // The range guard proves this conversion cannot lose the sign.
        let number = number as u32;
        char::from_u32('①' as u32 + number - 1).ok_or(CharError::InvalidEnclosedNumber)
    }
}

fn lower_char(ch: char) -> char {
    ch.to_lowercase().next().unwrap_or(ch)
}

fn java_category(category: GeneralCategory) -> i32 {
    java_category_abbreviation(category.abbreviation())
}

#[allow(clippy::match_same_arms)]
fn java_category_abbreviation(category: &str) -> i32 {
    match category {
        "Cn" => 0,
        "Lu" => 1,
        "Ll" => 2,
        "Lt" => 3,
        "Lm" => 4,
        "Lo" => 5,
        "Mn" => 6,
        "Me" => 7,
        "Mc" => 8,
        "Nd" => 9,
        "Nl" => 10,
        "No" => 11,
        "Zs" => 12,
        "Zl" => 13,
        "Zp" => 14,
        "Cc" => 15,
        "Cf" => 16,
        "Co" => 18,
        "Cs" => 19,
        "Pd" => 20,
        "Ps" => 21,
        "Pe" => 22,
        "Pc" => 23,
        "Po" => 24,
        "Sm" => 25,
        "Sc" => 26,
        "Sk" => 27,
        "So" => 28,
        "Pi" => 29,
        "Pf" => 30,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_classification_and_dynamic_character_checks_match_hutool() {
        assert!(CharUtil::is_ascii('a'));
        assert!(!CharUtil::is_ascii('©'));
        assert!(CharUtil::is_ascii_printable('~'));
        assert!(!CharUtil::is_ascii_printable('\n'));
        assert!(CharUtil::is_ascii_control('\n'));
        assert!(CharUtil::is_ascii_control('\u{7f}'));
        assert!(!CharUtil::is_ascii_control('a'));
        assert!(CharUtil::is_letter('Z'));
        assert!(CharUtil::is_letter_upper('A'));
        assert!(CharUtil::is_letter_lower('z'));
        assert!(!CharUtil::is_letter('中'));
        assert!(CharUtil::is_number('3'));
        assert!(CharUtil::is_hex_char('F'));
        assert!(!CharUtil::is_hex_char('g'));
        assert!(CharUtil::is_letter_or_number('7'));
        assert!(!CharUtil::is_letter_or_number('-'));
        assert_eq!(CharUtil::to_string('🌹'), "🌹");
        assert!(CharUtil::is_char_class::<char>());
        assert!(!CharUtil::is_char_class::<u8>());
        assert!(CharUtil::is_char(&'x'));
        assert!(!CharUtil::is_char(&"x"));
    }

    #[test]
    fn unicode_blank_emoji_category_and_digit_paths_are_explicit() {
        for blank in [
            '\u{00a0}', '\u{3000}', '\0', '\u{202a}', '\u{3164}', '\u{2800}', '\u{200c}',
            '\u{180e}', '\u{feff}',
        ] {
            assert!(CharUtil::is_blank_char(blank));
        }
        assert!(!CharUtil::is_blank_code(0x85));
        assert!(!CharUtil::is_blank_code(0x11_0000));
        assert!(CharUtil::is_emoji('🌹'));
        assert!(!CharUtil::is_emoji('莉'));
        assert!(!CharUtil::is_emoji('\0'));
        assert!(!CharUtil::is_emoji('\u{100000}'));
        assert!(CharUtil::is_file_separator('/'));
        assert!(CharUtil::is_file_separator('\\'));
        assert!(!CharUtil::is_file_separator(':'));
        assert!(CharUtil::equals('A', 'a', true));
        assert!(!CharUtil::equals('A', 'a', false));
        assert_eq!(CharUtil::get_type('A' as u32), 1);
        assert_eq!(CharUtil::get_type('a' as u32), 2);
        assert_eq!(CharUtil::get_type(' ' as u32), 12);
        assert_eq!(CharUtil::get_type(0xd800), 19);
        assert_eq!(CharUtil::get_type(0x11_0000), 0);
        assert_eq!(CharUtil::digit16('f' as u32), 15);
        assert_eq!(CharUtil::digit16('A' as u32), 10);
        assert_eq!(CharUtil::digit16('Ｆ' as u32), 15);
        assert_eq!(CharUtil::digit16('ａ' as u32), 10);
        assert_eq!(CharUtil::digit16('١' as u32), 1);
        assert_eq!(CharUtil::digit16('𝟠' as u32), 8);
        assert_eq!(CharUtil::digit16('z' as u32), -1);
        assert_eq!(CharUtil::digit16(0x11_0000), -1);
    }

    #[test]
    fn enclosed_conversions_cover_supported_tables_and_errors() {
        assert_eq!(CharUtil::to_close_char('2'), '②');
        assert_eq!(CharUtil::to_close_char('M'), 'Ⓜ');
        assert_eq!(CharUtil::to_close_char('r'), 'ⓡ');
        assert_eq!(CharUtil::to_close_char('0'), '0');
        assert_eq!(CharUtil::to_close_by_number(1), Ok('①'));
        assert_eq!(CharUtil::to_close_by_number(12), Ok('⑫'));
        assert_eq!(CharUtil::to_close_by_number(20), Ok('⑳'));
        assert_eq!(
            CharUtil::to_close_by_number(0),
            Err(CharError::InvalidEnclosedNumber)
        );
    }

    #[test]
    fn every_java_general_category_constant_is_mapped() {
        let categories = [
            ("Cn", 0),
            ("Lu", 1),
            ("Ll", 2),
            ("Lt", 3),
            ("Lm", 4),
            ("Lo", 5),
            ("Mn", 6),
            ("Me", 7),
            ("Mc", 8),
            ("Nd", 9),
            ("Nl", 10),
            ("No", 11),
            ("Zs", 12),
            ("Zl", 13),
            ("Zp", 14),
            ("Cc", 15),
            ("Cf", 16),
            ("Co", 18),
            ("Cs", 19),
            ("Pd", 20),
            ("Ps", 21),
            ("Pe", 22),
            ("Pc", 23),
            ("Po", 24),
            ("Sm", 25),
            ("Sc", 26),
            ("Sk", 27),
            ("So", 28),
            ("Pi", 29),
            ("Pf", 30),
            ("future-category", 0),
        ];
        for (category, expected) in categories {
            assert_eq!(java_category_abbreviation(category), expected);
        }
    }
}
