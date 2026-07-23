use std::fmt;

use chrono::{Datelike, Local, NaiveDate};
use thiserror::Error;

const CHINA_ID_MIN_LENGTH: usize = 15;
const CHINA_ID_MAX_LENGTH: usize = 18;
const POWER: [u32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
const CHECK_CODES: [char; 11] = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];

const CITY_CODES: [(&str, &str); 35] = [
    ("11", "北京"),
    ("12", "天津"),
    ("13", "河北"),
    ("14", "山西"),
    ("15", "内蒙古"),
    ("21", "辽宁"),
    ("22", "吉林"),
    ("23", "黑龙江"),
    ("31", "上海"),
    ("32", "江苏"),
    ("33", "浙江"),
    ("34", "安徽"),
    ("35", "福建"),
    ("36", "江西"),
    ("37", "山东"),
    ("41", "河南"),
    ("42", "湖北"),
    ("43", "湖南"),
    ("44", "广东"),
    ("45", "广西"),
    ("46", "海南"),
    ("50", "重庆"),
    ("51", "四川"),
    ("52", "贵州"),
    ("53", "云南"),
    ("54", "西藏"),
    ("61", "陕西"),
    ("62", "甘肃"),
    ("63", "青海"),
    ("64", "宁夏"),
    ("65", "新疆"),
    ("71", "台湾"),
    ("81", "香港"),
    ("82", "澳门"),
    ("83", "台湾"),
];

const TW_FIRST_CODES: [(char, u32); 26] = [
    ('A', 10),
    ('B', 11),
    ('C', 12),
    ('D', 13),
    ('E', 14),
    ('F', 15),
    ('G', 16),
    ('H', 17),
    ('J', 18),
    ('K', 19),
    ('L', 20),
    ('M', 21),
    ('N', 22),
    ('P', 23),
    ('Q', 24),
    ('R', 25),
    ('S', 26),
    ('T', 27),
    ('U', 28),
    ('V', 29),
    ('X', 30),
    ('Y', 31),
    ('W', 32),
    ('Z', 33),
    ('I', 34),
    ('O', 35),
];

/// Errors returned by identity-card accessors that Java Hutool exposes through exceptions.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdcardError {
    /// A required identity-card value was blank.
    #[error("identity card must not be blank")]
    Blank,
    /// A value cannot be interpreted as the requested identity-card representation.
    #[error("invalid identity card")]
    InvalidCard,
    /// The embedded birthday is not a real Gregorian date.
    #[error("invalid identity-card birthday: {0}")]
    InvalidBirthDate(String),
    /// Age cannot be calculated before the embedded birthday.
    #[error("comparison date precedes identity-card birthday")]
    BirthAfterComparison,
}

/// Parsed information returned for ten-character Taiwan, Macao, and Hong Kong cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card10Info {
    region: &'static str,
    gender: char,
    valid: bool,
}

impl Card10Info {
    /// Returns the card's region name.
    #[must_use]
    pub const fn region(self) -> &'static str {
        self.region
    }

    /// Returns `M`, `F`, or `N`, matching Hutool's information array.
    #[must_use]
    pub const fn gender(self) -> char {
        self.gender
    }

    /// Returns whether the regional checksum or syntax is valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.valid
    }
}

/// Hutool-compatible identity-card facade backed by checked Rust algorithms.
pub struct IdcardUtil;

impl IdcardUtil {
    /// Converts a numeric legacy 15-character mainland card into an 18-character card.
    #[must_use]
    pub fn convert_15_to_18(idcard: &str) -> Option<String> {
        let chars: Vec<char> = idcard.chars().collect();
        if chars.len() != CHINA_ID_MIN_LENGTH || !chars.iter().all(char::is_ascii_digit) {
            return None;
        }
        let year = parse_two_digits(&chars[6..8]);
        // Fifteen-character cards predate 2000. Hutool corrects a parsed 20xx
        // year by subtracting one century, while exactly `00` remains 2000.
        let full_year = if year == 0 { 2000 } else { 1900 + year };
        let mut code17 = chars[..6].iter().collect::<String>();
        code17.push_str(&full_year.to_string());
        code17.extend(chars[8..].iter());
        let check = weighted_check_code(code17.as_bytes());
        code17.push(check);
        Some(code17)
    }

    /// Converts a valid 18-character card to the legacy 15-character form.
    #[must_use]
    pub fn convert_18_to_15(idcard: &str) -> String {
        if !Self::is_valid_card_18(idcard) {
            return idcard.to_owned();
        }
        let chars: Vec<char> = idcard.chars().collect();
        chars[..6].iter().chain(chars[8..17].iter()).collect()
    }

    /// Validates mainland 15/18-character cards and regional 10-character cards.
    #[must_use]
    pub fn is_valid_card(idcard: &str) -> bool {
        if is_blank(idcard) {
            return false;
        }
        match idcard.chars().count() {
            CHINA_ID_MAX_LENGTH => Self::is_valid_card_18(idcard),
            CHINA_ID_MIN_LENGTH => Self::is_valid_card_15(idcard),
            10 => Self::is_valid_card_10(idcard).is_some_and(Card10Info::is_valid),
            _ => false,
        }
    }

    /// Validates an 18-character card while ignoring a checksum `X`'s case.
    #[must_use]
    pub fn is_valid_card_18(idcard: &str) -> bool {
        Self::is_valid_card_18_case(idcard, true)
    }

    /// Validates an 18-character card with explicit checksum case handling.
    #[must_use]
    pub fn is_valid_card_18_case(idcard: &str, ignore_case: bool) -> bool {
        if !idcard.is_ascii() || idcard.len() != CHINA_ID_MAX_LENGTH {
            return false;
        }
        let bytes = idcard.as_bytes();
        let province = if bytes[0] == b'9' {
            &idcard[1..3]
        } else {
            &idcard[..2]
        };
        if province_name(province).is_none() || parse_birth(&idcard[6..14]).is_none() {
            return false;
        }
        let Some(expected) = check_code_18(&bytes[..17]) else {
            return false;
        };
        let actual = char::from(bytes[17]);
        if ignore_case {
            expected.eq_ignore_ascii_case(&actual)
        } else {
            expected == actual
        }
    }

    /// Validates a numeric legacy 15-character mainland card.
    #[must_use]
    pub fn is_valid_card_15(idcard: &str) -> bool {
        if !idcard.is_ascii() || idcard.len() != CHINA_ID_MIN_LENGTH {
            return false;
        }
        idcard.bytes().all(|value| value.is_ascii_digit())
            && province_name(&idcard[..2]).is_some()
            && parse_birth(&format!("19{}", &idcard[6..12])).is_some()
    }

    /// Parses and validates Taiwan, Macao, or Hong Kong card information.
    #[must_use]
    pub fn is_valid_card_10(idcard: &str) -> Option<Card10Info> {
        if is_blank(idcard) {
            return None;
        }
        let normalized = idcard.replace('（', "(").replace('）', ")");
        let compact = compact_parenthesized_card(&normalized)?;
        let bytes = compact.as_bytes();
        if compact.len() == 10
            && bytes[0].is_ascii_alphabetic()
            && bytes[1..].iter().all(u8::is_ascii_digit)
        {
            let gender = match bytes[1] {
                b'1' => 'M',
                b'2' => 'F',
                _ => 'N',
            };
            let valid = gender != 'N' && Self::is_valid_tw_card(&compact);
            return Some(Card10Info {
                region: "台湾",
                gender,
                valid,
            });
        }
        if compact.len() == 8
            && matches!(bytes[0], b'1' | b'5' | b'7')
            && bytes[1..7].iter().all(u8::is_ascii_digit)
            && (bytes[7].is_ascii_digit() || bytes[7].is_ascii_uppercase())
        {
            return Some(Card10Info {
                region: "澳门",
                gender: 'N',
                valid: true,
            });
        }
        let letters = compact.len().saturating_sub(7);
        if matches!(letters, 1 | 2)
            && bytes[..letters].iter().all(u8::is_ascii_uppercase)
            && bytes[letters..letters + 6].iter().all(u8::is_ascii_digit)
            && matches!(bytes[letters + 6], b'0'..=b'9' | b'A')
        {
            return Some(Card10Info {
                region: "香港",
                gender: 'N',
                valid: Self::is_valid_hk_card(&normalized),
            });
        }
        None
    }

    /// Validates a Taiwan identity-card checksum.
    #[must_use]
    pub fn is_valid_tw_card(idcard: &str) -> bool {
        if !idcard.is_ascii() || idcard.len() != 10 {
            return false;
        }
        let bytes = idcard.as_bytes();
        let Some(first) = tw_first_code(char::from(bytes[0])) else {
            return false;
        };
        if !bytes[1..].iter().all(u8::is_ascii_digit) {
            return false;
        }
        let mut sum = first / 10 + (first % 10) * 9;
        for (value, weight) in bytes[1..9].iter().zip((1_u32..=8).rev()) {
            sum += u32::from(value - b'0') * weight;
        }
        let expected = if sum % 10 == 0 { 0 } else { 10 - sum % 10 };
        expected == u32::from(bytes[9] - b'0')
    }

    /// Validates a Hong Kong identity-card checksum.
    #[must_use]
    pub fn is_valid_hk_card(idcard: &str) -> bool {
        if !idcard.is_ascii() {
            return false;
        }
        let Some(compact) = compact_parenthesized_card(idcard) else {
            return false;
        };
        let bytes = compact.as_bytes();
        let letters = compact.len().saturating_sub(7);
        if !matches!(letters, 1 | 2)
            || !bytes[..letters].iter().all(u8::is_ascii_uppercase)
            || !bytes[letters..letters + 6].iter().all(u8::is_ascii_digit)
            || !matches!(bytes[letters + 6], b'0'..=b'9' | b'A')
        {
            return false;
        }
        let letter_value = |value: u8| u32::from(value - b'A' + 10);
        let mut sum = if letters == 2 {
            letter_value(bytes[0]) * 9 + letter_value(bytes[1]) * 8
        } else {
            522 + letter_value(bytes[0]) * 8
        };
        for (value, weight) in bytes[letters..letters + 6].iter().zip((2_u32..=7).rev()) {
            sum += u32::from(value - b'0') * weight;
        }
        sum += match bytes[letters + 6] {
            b'A' => 10,
            value => u32::from(value - b'0'),
        };
        sum % 11 == 0
    }

    /// Returns the embedded birthday as `yyyyMMdd`.
    pub fn get_birth_by_id_card(idcard: &str) -> Result<Option<String>, IdcardError> {
        Self::get_birth(idcard)
    }

    /// Returns the embedded birthday as `yyyyMMdd`.
    pub fn get_birth(idcard: &str) -> Result<Option<String>, IdcardError> {
        if is_blank(idcard) {
            return Err(IdcardError::Blank);
        }
        let chars: Vec<char> = idcard.chars().collect();
        if chars.len() < CHINA_ID_MIN_LENGTH {
            return Ok(None);
        }
        let expanded;
        let chars = if chars.len() == CHINA_ID_MIN_LENGTH {
            expanded = Self::convert_15_to_18(idcard).ok_or(IdcardError::InvalidCard)?;
            expanded.chars().collect::<Vec<_>>()
        } else {
            chars
        };
        Ok(Some(chars[6..14].iter().collect()))
    }

    /// Returns the embedded birthday as a checked Gregorian date.
    pub fn get_birth_date(idcard: &str) -> Result<Option<NaiveDate>, IdcardError> {
        let Some(birth) = Self::get_birth(idcard)? else {
            return Ok(None);
        };
        parse_birth(&birth)
            .map(Some)
            .ok_or(IdcardError::InvalidBirthDate(birth))
    }

    /// Calculates age at the machine's current local date.
    pub fn get_age_by_id_card(idcard: &str) -> Result<i32, IdcardError> {
        Self::get_age_by_id_card_at(idcard, Local::now().date_naive())
    }

    /// Calculates age at a caller-supplied comparison date.
    pub fn get_age_by_id_card_at(idcard: &str, comparison: NaiveDate) -> Result<i32, IdcardError> {
        let birth = Self::get_birth_date(idcard)?.ok_or(IdcardError::InvalidCard)?;
        if comparison < birth {
            return Err(IdcardError::BirthAfterComparison);
        }
        // 对齐 Hutool `CalendarUtil.age(long, long)`:
        // 1. 月份小于生日月份 → 年龄减 1
        // 2. 月份相等且日小于等于生日当天 → 年龄减 1
        //    注: Hutool issue#I6E6ZG,法定生日当天不算年龄,从第二天开始计算,
        //    即生日当天 dayOfMonth <= dayOfMonthBirth 仍减 1。
        let comp_md = (comparison.month(), comparison.day());
        let birth_md = (birth.month(), birth.day());
        let not_yet_birthday_this_year = match comp_md.0.cmp(&birth_md.0) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Equal => comp_md.1 <= birth_md.1,
            std::cmp::Ordering::Greater => false,
        };
        Ok(comparison.year() - birth.year() - i32::from(not_yet_birthday_this_year))
    }

    /// Returns the embedded four-digit birth year.
    pub fn get_year_by_id_card(idcard: &str) -> Result<Option<i16>, IdcardError> {
        parse_birth_component(idcard, 0..4)
    }

    /// Returns the embedded two-digit birth month.
    pub fn get_month_by_id_card(idcard: &str) -> Result<Option<i16>, IdcardError> {
        parse_birth_component(idcard, 4..6)
    }

    /// Returns the embedded two-digit birth day.
    pub fn get_day_by_id_card(idcard: &str) -> Result<Option<i16>, IdcardError> {
        parse_birth_component(idcard, 6..8)
    }

    /// Returns `1` for male and `0` for female using the sequence-code parity.
    pub fn get_gender_by_id_card(idcard: &str) -> Result<u8, IdcardError> {
        if is_blank(idcard) {
            return Err(IdcardError::Blank);
        }
        let mut chars: Vec<char> = idcard.chars().collect();
        if chars.len() == CHINA_ID_MIN_LENGTH {
            chars = Self::convert_15_to_18(idcard)
                .ok_or(IdcardError::InvalidCard)?
                .chars()
                .collect();
        } else if chars.len() != CHINA_ID_MAX_LENGTH {
            return Err(IdcardError::InvalidCard);
        }
        Ok(u8::from(u32::from(chars[16]) % 2 != 0))
    }

    /// Returns the two-character province code for 15/18-character values.
    #[must_use]
    pub fn get_province_code_by_id_card(idcard: &str) -> Option<String> {
        prefix_for_supported_length(idcard, 2)
    }

    /// Returns the province name for a known province code.
    #[must_use]
    pub fn get_province_by_id_card(idcard: &str) -> Option<&'static str> {
        let code = Self::get_province_code_by_id_card(idcard)?;
        province_name(&code)
    }

    /// Returns the four-character city-level code for 15/18-character values.
    #[must_use]
    pub fn get_city_code_by_id_card(idcard: &str) -> Option<String> {
        prefix_for_supported_length(idcard, 4)
    }

    /// Returns the six-character district-level code for 15/18-character values.
    #[must_use]
    pub fn get_district_code_by_id_card(idcard: &str) -> Option<String> {
        prefix_for_supported_length(idcard, 6)
    }

    /// Replaces the requested Unicode scalar-value interval with asterisks.
    #[must_use]
    pub fn hide(idcard: &str, start_include: i32, end_exclude: i32) -> String {
        let length = i64::try_from(idcard.chars().count()).unwrap_or(i64::MAX);
        let start_include = i64::from(start_include);
        let end_exclude = i64::from(end_exclude);
        if start_include > length || start_include > end_exclude {
            return idcard.to_owned();
        }
        let end_exclude = end_exclude.min(length);
        idcard
            .chars()
            .enumerate()
            .map(|(index, value)| {
                let index = i64::try_from(index).unwrap_or(i64::MAX);
                if index >= start_include && index < end_exclude {
                    '*'
                } else {
                    value
                }
            })
            .collect()
    }

    /// Builds the complete identity-card information value using the current local date.
    pub fn get_idcard_info(idcard: &str) -> Result<Idcard, IdcardError> {
        Idcard::new(idcard)
    }
}

/// Owned identity-card information corresponding to Hutool's nested `Idcard` value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Idcard {
    province_code: Option<String>,
    city_code: Option<String>,
    birth_date: NaiveDate,
    gender: u8,
    age: i32,
}

impl Idcard {
    /// Creates identity-card information using the current local date.
    pub fn new(idcard: &str) -> Result<Self, IdcardError> {
        Self::new_at(idcard, Local::now().date_naive())
    }

    /// Creates identity-card information at a deterministic comparison date.
    pub fn new_at(idcard: &str, comparison: NaiveDate) -> Result<Self, IdcardError> {
        Ok(Self {
            province_code: IdcardUtil::get_province_code_by_id_card(idcard),
            city_code: IdcardUtil::get_city_code_by_id_card(idcard),
            birth_date: IdcardUtil::get_birth_date(idcard)?.ok_or(IdcardError::InvalidCard)?,
            gender: IdcardUtil::get_gender_by_id_card(idcard)?,
            age: IdcardUtil::get_age_by_id_card_at(idcard, comparison)?,
        })
    }

    /// Returns the province code.
    #[must_use]
    pub fn province_code(&self) -> Option<&str> {
        self.province_code.as_deref()
    }

    /// Returns the province name.
    #[must_use]
    pub fn province(&self) -> Option<&'static str> {
        self.province_code.as_deref().and_then(province_name)
    }

    /// Returns the city-level code.
    #[must_use]
    pub fn city_code(&self) -> Option<&str> {
        self.city_code.as_deref()
    }

    /// Returns the birth date.
    #[must_use]
    pub const fn birth_date(&self) -> NaiveDate {
        self.birth_date
    }

    /// Returns `1` for male and `0` for female.
    #[must_use]
    pub const fn gender(&self) -> u8 {
        self.gender
    }

    /// Returns the age captured when this value was built.
    #[must_use]
    pub const fn age(&self) -> i32 {
        self.age
    }
}

impl fmt::Display for Idcard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Idcard{{provinceCode='{}', cityCode='{}', birthDate={} 00:00:00, gender={}, age={}}}",
            self.province_code.as_deref().unwrap_or("null"),
            self.city_code.as_deref().unwrap_or("null"),
            self.birth_date.format("%Y-%m-%d"),
            self.gender,
            self.age
        )
    }
}

fn parse_two_digits(value: &[char]) -> u32 {
    let tens = value[0].to_digit(10).unwrap_or(0);
    let ones = value[1].to_digit(10).unwrap_or(0);
    tens * 10 + ones
}

fn check_code_18(code17: &[u8]) -> Option<char> {
    if code17.len() != POWER.len() || !code17.iter().all(u8::is_ascii_digit) {
        return None;
    }
    Some(weighted_check_code(code17))
}

fn weighted_check_code(code17: &[u8]) -> char {
    let sum = code17
        .iter()
        .zip(POWER)
        .map(|(value, power)| u32::from(value - b'0') * power)
        .sum::<u32>();
    CHECK_CODES[(sum % 11) as usize]
}

fn province_name(code: &str) -> Option<&'static str> {
    CITY_CODES
        .iter()
        .find_map(|(candidate, name)| (*candidate == code).then_some(*name))
}

fn tw_first_code(value: char) -> Option<u32> {
    TW_FIRST_CODES
        .iter()
        .find_map(|(candidate, code)| (*candidate == value).then_some(*code))
}

fn parse_birth(value: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y%m%d").ok()
}

fn parse_birth_component(
    idcard: &str,
    range: std::ops::Range<usize>,
) -> Result<Option<i16>, IdcardError> {
    let Some(birth) = IdcardUtil::get_birth(idcard)? else {
        return Ok(None);
    };
    birth[range]
        .parse()
        .map(Some)
        .map_err(|_| IdcardError::InvalidCard)
}

fn prefix_for_supported_length(idcard: &str, length: usize) -> Option<String> {
    let chars: Vec<char> = idcard.chars().collect();
    matches!(chars.len(), CHINA_ID_MIN_LENGTH | CHINA_ID_MAX_LENGTH)
        .then(|| chars[..length].iter().collect())
}

fn compact_parenthesized_card(value: &str) -> Option<String> {
    if value.contains(['(', ')']) {
        let chars: Vec<char> = value.chars().collect();
        if chars.len() < 3
            || chars[chars.len() - 3] != '('
            || chars[chars.len() - 1] != ')'
            || chars[..chars.len() - 3]
                .iter()
                .any(|value| matches!(value, '(' | ')'))
        {
            return None;
        }
        Some(
            chars[..chars.len() - 3]
                .iter()
                .chain(std::iter::once(&chars[chars.len() - 2]))
                .collect(),
        )
    } else {
        Some(value.to_owned())
    }
}

fn is_blank(value: &str) -> bool {
    value.chars().all(crate::CharUtil::is_blank_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID_18: &str = "321083197812162119";
    const FOREIGN_ID_18: &str = "932682198501010017";
    const ID_15: &str = "150102880730303";

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn mainland_conversion_validation_and_checksums_match_hutool() {
        assert_eq!(
            IdcardUtil::convert_15_to_18(ID_15).as_deref(),
            Some("150102198807303035")
        );
        assert_eq!(
            IdcardUtil::convert_15_to_18("330102200403064").as_deref(),
            Some("33010219200403064X")
        );
        assert_eq!(
            IdcardUtil::convert_15_to_18("110101000101001").as_deref(),
            Some("110101200001010010")
        );
        assert_eq!(IdcardUtil::convert_15_to_18("short"), None);
        assert_eq!(IdcardUtil::convert_15_to_18("15010288073030A"), None);
        assert_eq!(IdcardUtil::convert_18_to_15("150102198807303035"), ID_15);
        assert_eq!(IdcardUtil::convert_18_to_15("invalid"), "invalid");

        for value in [
            ID_18,
            ID_15,
            FOREIGN_ID_18,
            "81000019980902013X",
            "820000200009100032",
            "830000200209060065",
            "111111111111111",
        ] {
            assert!(IdcardUtil::is_valid_card(value), "{value}");
        }
        for value in [
            "",
            "360198910283844",
            "201511221897205960",
            "815727834224151",
            "3301022011022000D6",
            "123",
        ] {
            assert!(!IdcardUtil::is_valid_card(value), "{value}");
        }
        assert!(!IdcardUtil::is_valid_card_18("éééééééééééééééééé"));
        assert!(!IdcardUtil::is_valid_card_18("123"));
        assert!(!IdcardUtil::is_valid_card_18("00110219200403064X"));
        assert!(!IdcardUtil::is_valid_card_18("33010219201303064X"));
        assert!(!IdcardUtil::is_valid_card_18("3301021920040306AX"));
        assert!(!IdcardUtil::is_valid_card_18_case(
            "33010219200403064x",
            false
        ));
        assert!(IdcardUtil::is_valid_card_18_case(
            "33010219200403064X",
            false
        ));
        assert!(IdcardUtil::is_valid_card_18("33010219200403064x"));
        assert!(!IdcardUtil::is_valid_card_15("ééééééééééééééé"));
        assert!(!IdcardUtil::is_valid_card_15("123"));
        assert!(!IdcardUtil::is_valid_card_15("001102880730303"));
        assert!(!IdcardUtil::is_valid_card_15("150102881330303"));
        assert!(!IdcardUtil::is_valid_card_15("15010288073030A"));
        assert_eq!(check_code_18(b"short"), None);
        assert_eq!(check_code_18(b"3301021920040306A"), None);
        assert_eq!(CHECK_CODES.len(), 11);
    }

    #[test]
    fn regional_card_rules_cover_taiwan_macao_and_hong_kong() {
        let tw = IdcardUtil::is_valid_card_10("B221690311").unwrap();
        assert_eq!(
            (tw.region(), tw.gender(), tw.is_valid()),
            ("台湾", 'F', true)
        );
        assert!(IdcardUtil::is_valid_card("B221690311"));
        let tw_male = IdcardUtil::is_valid_card_10("A123456789").unwrap();
        assert_eq!((tw_male.region(), tw_male.gender()), ("台湾", 'M'));
        let tw_unknown = IdcardUtil::is_valid_card_10("A323456789").unwrap();
        assert_eq!((tw_unknown.gender(), tw_unknown.is_valid()), ('N', false));
        assert!(
            !IdcardUtil::is_valid_card_10("M517086311")
                .unwrap()
                .is_valid()
        );
        assert!(IdcardUtil::is_valid_tw_card("B000000000"));
        assert_eq!(IdcardUtil::is_valid_card_10("B2216903112"), None);
        assert_eq!(IdcardUtil::is_valid_card_10(" "), None);

        for value in ["1608214(1)", "1608214（1）", "16082141"] {
            let info = IdcardUtil::is_valid_card_10(value).unwrap();
            assert_eq!(
                (info.region(), info.gender(), info.is_valid()),
                ("澳门", 'N', true)
            );
        }
        let hk = IdcardUtil::is_valid_card_10("P174468(6)").unwrap();
        assert_eq!(
            (hk.region(), hk.gender(), hk.is_valid()),
            ("香港", 'N', true)
        );
        assert!(IdcardUtil::is_valid_hk_card("AB987654(3)"));
        assert!(!IdcardUtil::is_valid_hk_card("H01487002"));
        assert!(!IdcardUtil::is_valid_hk_card("é174468(6)"));
        assert!(!IdcardUtil::is_valid_hk_card("P174468(B)"));
        assert!(!IdcardUtil::is_valid_hk_card("P17446A(6)"));
        assert!(!IdcardUtil::is_valid_hk_card("p174468(6)"));
        assert!(!IdcardUtil::is_valid_hk_card("P174468(6"));
        assert!(!IdcardUtil::is_valid_hk_card("A123456(A)"));
        assert!(!IdcardUtil::is_valid_hk_card("ABC1234567"));
        assert_eq!(IdcardUtil::is_valid_card_10("H01487002"), None);
        assert_eq!(
            IdcardUtil::is_valid_card_10("1608214(B)"),
            Some(Card10Info {
                region: "澳门",
                gender: 'N',
                valid: true
            })
        );
        assert_eq!(
            IdcardUtil::is_valid_card_10("P174468(5)"),
            Some(Card10Info {
                region: "香港",
                gender: 'N',
                valid: false
            })
        );
        assert_eq!(IdcardUtil::is_valid_card_10("P174468(6"), None);
        assert_eq!(IdcardUtil::is_valid_card_10("P17(4468(6)"), None);
        assert_eq!(IdcardUtil::is_valid_card_10("P174468(B)"), None);
        assert_eq!(IdcardUtil::is_valid_card_10("not-a-card"), None);
        assert_eq!(
            IdcardUtil::is_valid_card_10("A123456A"),
            Some(Card10Info {
                region: "香港",
                gender: 'N',
                valid: false
            })
        );

        assert!(IdcardUtil::is_valid_tw_card("B221690311"));
        assert!(!IdcardUtil::is_valid_tw_card("B2216903112"));
        assert!(!IdcardUtil::is_valid_tw_card("1221690311"));
        assert!(!IdcardUtil::is_valid_tw_card("B22169031A"));
        for (letter, code) in TW_FIRST_CODES {
            assert_eq!(tw_first_code(letter), Some(code));
        }
        assert_eq!(tw_first_code('?'), None);
    }

    #[test]
    fn birthday_and_age_are_checked() {
        assert_eq!(
            IdcardUtil::get_birth_by_id_card(ID_18).unwrap().as_deref(),
            Some("19781216")
        );
        assert_eq!(
            IdcardUtil::get_birth(ID_15).unwrap().as_deref(),
            Some("19880730")
        );
        assert_eq!(IdcardUtil::get_birth("short"), Ok(None));
        assert_eq!(IdcardUtil::get_birth(" "), Err(IdcardError::Blank));
        assert_eq!(
            IdcardUtil::get_birth("15010288073030A"),
            Err(IdcardError::InvalidCard)
        );
        assert_eq!(
            IdcardUtil::get_birth_date("321083197813162119"),
            Err(IdcardError::InvalidBirthDate("19781316".to_owned()))
        );
        assert_eq!(IdcardUtil::get_birth_date("short"), Ok(None));
        assert_eq!(IdcardUtil::get_birth_date(" "), Err(IdcardError::Blank));
        assert_eq!(
            IdcardUtil::get_age_by_id_card_at(ID_18, date(2017, 4, 10)),
            Ok(38)
        );
        assert_eq!(
            IdcardUtil::get_age_by_id_card_at(FOREIGN_ID_18, date(2017, 4, 10)),
            Ok(32)
        );
        assert_eq!(
            IdcardUtil::get_age_by_id_card_at(ID_15, date(2017, 8, 1)),
            Ok(29)
        );
        assert_eq!(
            IdcardUtil::get_age_by_id_card_at(ID_15, date(2017, 7, 1)),
            Ok(28)
        );
        assert_eq!(
            IdcardUtil::get_age_by_id_card_at(ID_18, date(1900, 1, 1)),
            Err(IdcardError::BirthAfterComparison)
        );
        assert_eq!(
            IdcardUtil::get_age_by_id_card_at("short", date(2020, 1, 1)),
            Err(IdcardError::InvalidCard)
        );
        assert_eq!(
            IdcardUtil::get_age_by_id_card_at("321083197813162119", date(2020, 1, 1)),
            Err(IdcardError::InvalidBirthDate("19781316".to_owned()))
        );
        assert!(IdcardUtil::get_age_by_id_card(ID_18).unwrap() > 0);
    }

    #[test]
    fn components_codes_and_masking_are_checked() {
        assert_eq!(IdcardUtil::get_year_by_id_card(ID_15), Ok(Some(1988)));
        assert_eq!(IdcardUtil::get_month_by_id_card(ID_18), Ok(Some(12)));
        assert_eq!(IdcardUtil::get_day_by_id_card(ID_18), Ok(Some(16)));
        assert_eq!(IdcardUtil::get_year_by_id_card("short"), Ok(None));
        assert_eq!(
            IdcardUtil::get_year_by_id_card(" "),
            Err(IdcardError::Blank)
        );
        assert_eq!(
            IdcardUtil::get_year_by_id_card("321083ABCD12162119"),
            Err(IdcardError::InvalidCard)
        );
        assert_eq!(IdcardUtil::get_gender_by_id_card(ID_18), Ok(1));
        assert_eq!(IdcardUtil::get_gender_by_id_card(ID_15), Ok(1));
        assert_eq!(
            IdcardUtil::get_gender_by_id_card(" "),
            Err(IdcardError::Blank)
        );
        assert_eq!(
            IdcardUtil::get_gender_by_id_card("short"),
            Err(IdcardError::InvalidCard)
        );
        assert_eq!(
            IdcardUtil::get_gender_by_id_card("15010288073030A"),
            Err(IdcardError::InvalidCard)
        );

        assert_eq!(
            IdcardUtil::get_province_code_by_id_card(ID_18).as_deref(),
            Some("32")
        );
        assert_eq!(IdcardUtil::get_province_by_id_card(ID_18), Some("江苏"));
        assert_eq!(IdcardUtil::get_province_by_id_card(ID_15), Some("内蒙古"));
        assert_eq!(
            IdcardUtil::get_city_code_by_id_card(ID_18).as_deref(),
            Some("3210")
        );
        assert_eq!(
            IdcardUtil::get_district_code_by_id_card(ID_18).as_deref(),
            Some("321083")
        );
        assert_eq!(IdcardUtil::get_province_code_by_id_card("short"), None);
        assert_eq!(
            IdcardUtil::get_province_by_id_card("001083197812162119"),
            None
        );
        assert_eq!(IdcardUtil::get_province_by_id_card("short"), None);
        assert_eq!(IdcardUtil::get_city_code_by_id_card("short"), None);
        assert_eq!(IdcardUtil::get_district_code_by_id_card("short"), None);
        for (code, name) in CITY_CODES {
            assert_eq!(province_name(code), Some(name));
        }
        assert_eq!(province_name("91"), None);

        assert_eq!(
            IdcardUtil::hide("51343620000320711X", 6, 14),
            "513436********711X"
        );
        assert_eq!(IdcardUtil::hide("身份证号码", -1, 2), "**证号码");
        assert_eq!(IdcardUtil::hide("123", 4, 5), "123");
        assert_eq!(IdcardUtil::hide("123", 2, 1), "123");
        assert_eq!(IdcardUtil::hide("123", 1, 99), "1**");
        assert_eq!(IdcardUtil::hide("123", 0, -1), "123");
    }

    #[test]
    fn owned_idcard_info_exposes_every_value_and_display() {
        let info = Idcard::new_at(ID_18, date(2017, 4, 10)).unwrap();
        assert_eq!(info.province_code(), Some("32"));
        assert_eq!(info.province(), Some("江苏"));
        assert_eq!(info.city_code(), Some("3210"));
        assert_eq!(info.birth_date(), date(1978, 12, 16));
        assert_eq!(info.gender(), 1);
        assert_eq!(info.age(), 38);
        assert_eq!(
            info.to_string(),
            "Idcard{provinceCode='32', cityCode='3210', birthDate=1978-12-16 00:00:00, gender=1, age=38}"
        );
        let current = IdcardUtil::get_idcard_info(ID_18).unwrap();
        assert!(current.age() > 0);
        assert!(Idcard::new(ID_18).unwrap().age() > 0);
        assert_eq!(
            Idcard::new_at("001083197812162119", date(2017, 4, 10))
                .unwrap()
                .province(),
            None
        );
        assert_eq!(
            Idcard::new_at("short", date(2017, 4, 10)),
            Err(IdcardError::InvalidCard)
        );
        assert_eq!(
            Idcard::new_at("321083197813162119", date(2017, 4, 10)),
            Err(IdcardError::InvalidBirthDate("19781316".to_owned()))
        );
        assert_eq!(
            Idcard::new_at("3210831978121621", date(2017, 4, 10)),
            Err(IdcardError::InvalidCard)
        );
        assert_eq!(
            Idcard::new_at(ID_18, date(1900, 1, 1)),
            Err(IdcardError::BirthAfterComparison)
        );
    }
}
