/// Supported default masking strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DesensitizedType {
    /// Replace a user identifier with zero.
    UserId,
    /// Preserve only the first character of a name.
    ChineseName,
    /// Preserve one leading and two trailing identity-card characters.
    IdCard,
    /// Preserve four leading and two trailing landline characters.
    FixedPhone,
    /// Preserve three leading and four trailing mobile characters.
    MobilePhone,
    /// Preserve the address prefix and mask eight trailing characters.
    Address,
    /// Preserve the first mailbox character and domain.
    Email,
    /// Mask every password character.
    Password,
    /// Mask the middle of a Chinese vehicle plate.
    CarLicense,
    /// Preserve the first four and final group of a bank card.
    BankCard,
    /// Preserve the first IPv4 component.
    Ipv4,
    /// Preserve the first IPv6 component.
    Ipv6,
    /// Preserve two leading and two trailing passport characters.
    Passport,
    /// Preserve four leading and four trailing credit-code characters.
    CreditCode,
    /// Preserve only the first character.
    FirstMask,
    /// Return no value.
    ClearToNull,
    /// Return an empty value.
    ClearToEmpty,
}

/// Unicode-safe Hutool-compatible masking facade.
pub struct DesensitizedUtil;

impl DesensitizedUtil {
    /// Applies a predefined masking strategy.
    #[must_use]
    pub fn desensitized(value: Option<&str>, kind: DesensitizedType) -> Option<String> {
        let Some(value) = nonblank(value) else {
            return Some(String::new());
        };
        match kind {
            DesensitizedType::UserId => Some(Self::user_id().to_string()),
            DesensitizedType::ChineseName => Some(Self::chinese_name(Some(value))),
            DesensitizedType::IdCard => Some(Self::id_card_num(Some(value), 1, 2)),
            DesensitizedType::FixedPhone => Some(Self::fixed_phone(Some(value))),
            DesensitizedType::MobilePhone => Some(Self::mobile_phone(Some(value))),
            DesensitizedType::Address => Some(Self::address(Some(value), 8)),
            DesensitizedType::Email => Some(Self::email(Some(value))),
            DesensitizedType::Password => Some(Self::password(Some(value))),
            DesensitizedType::CarLicense => Some(Self::car_license(Some(value))),
            DesensitizedType::BankCard => Self::bank_card(Some(value)),
            DesensitizedType::Ipv4 => Some(Self::ipv4(value)),
            DesensitizedType::Ipv6 => Some(Self::ipv6(value)),
            DesensitizedType::Passport => Self::passport(Some(value)),
            DesensitizedType::CreditCode => Self::credit_code(Some(value)),
            DesensitizedType::FirstMask => Some(Self::first_mask(Some(value))),
            DesensitizedType::ClearToNull => Self::clear_to_null(),
            DesensitizedType::ClearToEmpty => Some(Self::clear()),
        }
    }

    /// Returns an empty string.
    #[must_use]
    pub const fn clear() -> String {
        String::new()
    }

    /// Returns no string value.
    #[must_use]
    pub const fn clear_to_null() -> Option<String> {
        None
    }

    /// Returns Hutool's fixed masked user identifier.
    #[must_use]
    pub const fn user_id() -> i64 {
        0
    }

    /// Preserves the first character and masks the rest.
    #[must_use]
    pub fn first_mask(value: Option<&str>) -> String {
        mask_blank_or(value, |value| hide(value, 1, char_len(value)))
    }

    /// Preserves the first Chinese-name character and masks the rest.
    #[must_use]
    pub fn chinese_name(value: Option<&str>) -> String {
        Self::first_mask(value)
    }

    /// Preserves `front` leading and `end` trailing identity-card characters.
    #[must_use]
    pub fn id_card_num(value: Option<&str>, front: i32, end: i32) -> String {
        let Some(value) = nonblank(value) else {
            return String::new();
        };
        let length = char_len(value);
        let (Ok(front), Ok(end)) = (usize::try_from(front), usize::try_from(end)) else {
            return String::new();
        };
        if front.saturating_add(end) > length {
            return String::new();
        }
        hide(value, front, length - end)
    }

    /// Preserves four leading and two trailing fixed-phone characters.
    #[must_use]
    pub fn fixed_phone(value: Option<&str>) -> String {
        mask_blank_or(value, |value| {
            let length = char_len(value);
            hide(value, 4, length.saturating_sub(2))
        })
    }

    /// Preserves three leading and four trailing mobile-phone characters.
    #[must_use]
    pub fn mobile_phone(value: Option<&str>) -> String {
        mask_blank_or(value, |value| {
            let length = char_len(value);
            hide(value, 3, length.saturating_sub(4))
        })
    }

    /// Masks `sensitive_size` trailing address characters.
    #[must_use]
    pub fn address(value: Option<&str>, sensitive_size: i32) -> String {
        mask_blank_or(value, |value| {
            let length = char_len(value);
            let size = usize::try_from(sensitive_size).unwrap_or(0).min(length);
            hide(value, length - size, length)
        })
    }

    /// Preserves the first mailbox character and the complete domain.
    #[must_use]
    pub fn email(value: Option<&str>) -> String {
        mask_blank_or(value, |value| {
            let Some(index) = value.chars().position(|ch| ch == '@') else {
                return value.to_owned();
            };
            if index <= 1 {
                value.to_owned()
            } else {
                hide(value, 1, index)
            }
        })
    }

    /// Masks every password character.
    #[must_use]
    pub fn password(value: Option<&str>) -> String {
        mask_blank_or(value, |value| "*".repeat(char_len(value)))
    }

    /// Masks the middle of valid-length ordinary and new-energy plates.
    #[must_use]
    pub fn car_license(value: Option<&str>) -> String {
        mask_blank_or(value, |value| match char_len(value) {
            7 => hide(value, 3, 6),
            8 => hide(value, 3, 7),
            _ => value.to_owned(),
        })
    }

    /// Formats and masks a blank-insensitive bank-card number.
    #[must_use]
    pub fn bank_card(value: Option<&str>) -> Option<String> {
        let value = value?;
        if nonblank(Some(value)).is_none() {
            return Some(value.to_owned());
        }
        let compact: String = value
            .chars()
            .filter(|ch| !crate::CharUtil::is_blank_char(*ch))
            .collect();
        let length = char_len(&compact);
        if length < 9 {
            return Some(compact);
        }
        let end_length = match length % 4 {
            0 => 4,
            remainder => remainder,
        };
        let middle = length - 4 - end_length;
        let chars: Vec<char> = compact.chars().collect();
        let mut output: String = chars[..4].iter().collect();
        for index in 0..middle {
            if index % 4 == 0 {
                output.push(' ');
            }
            output.push('*');
        }
        output.push(' ');
        output.extend(chars[length - end_length..].iter());
        Some(output)
    }

    /// Preserves the first IPv4 component.
    #[must_use]
    pub fn ipv4(value: &str) -> String {
        format!("{}.*.*.*", value.split('.').next().unwrap_or(value))
    }

    /// Preserves the first IPv6 component.
    #[must_use]
    pub fn ipv6(value: &str) -> String {
        format!("{}:*:*:*:*:*:*:*", value.split(':').next().unwrap_or(value))
    }

    /// Preserves two leading and two trailing passport characters.
    #[must_use]
    pub fn passport(value: Option<&str>) -> Option<String> {
        let value = value?;
        if nonblank(Some(value)).is_none() {
            return Some(value.to_owned());
        }
        let length = char_len(value);
        Some(if length <= 2 {
            hide(value, 0, length)
        } else {
            hide(value, 2, length - 2)
        })
    }

    /// Preserves four leading and four trailing unified-credit-code characters.
    #[must_use]
    pub fn credit_code(value: Option<&str>) -> Option<String> {
        let value = value?;
        if nonblank(Some(value)).is_none() {
            return Some(value.to_owned());
        }
        let length = char_len(value);
        Some(if length <= 4 {
            hide(value, 0, length)
        } else {
            hide(value, 4, length.saturating_sub(4))
        })
    }
}

fn nonblank(value: Option<&str>) -> Option<&str> {
    value.filter(|value| !value.chars().all(crate::CharUtil::is_blank_char))
}

fn mask_blank_or(value: Option<&str>, mask: impl FnOnce(&str) -> String) -> String {
    nonblank(value).map_or_else(String::new, mask)
}

fn char_len(value: &str) -> usize {
    value.chars().count()
}

fn hide(value: &str, start: usize, end: usize) -> String {
    value
        .chars()
        .enumerate()
        .map(|(index, ch)| {
            if (start..end).contains(&index) {
                '*'
            } else {
                ch
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatcher_covers_every_strategy_and_blank_short_circuit() {
        let cases = [
            (DesensitizedType::UserId, "100", Some("0")),
            (DesensitizedType::ChineseName, "段正淳", Some("段**")),
            (
                DesensitizedType::IdCard,
                "51343620000320711X",
                Some("5***************1X"),
            ),
            (
                DesensitizedType::FixedPhone,
                "09157518479",
                Some("0915*****79"),
            ),
            (
                DesensitizedType::MobilePhone,
                "18049531999",
                Some("180****1999"),
            ),
            (
                DesensitizedType::Address,
                "北京市海淀区马连洼街道289号",
                Some("北京市海淀区马********"),
            ),
            (
                DesensitizedType::Email,
                "duandazhi-jack@gmail.com.cn",
                Some("d*************@gmail.com.cn"),
            ),
            (DesensitizedType::Password, "1234567890", Some("**********")),
            (DesensitizedType::CarLicense, "苏D40000", Some("苏D4***0")),
            (
                DesensitizedType::BankCard,
                "11011111222233333256",
                Some("1101 **** **** **** 3256"),
            ),
            (DesensitizedType::Ipv4, "192.168.1.1", Some("192.*.*.*")),
            (
                DesensitizedType::Ipv6,
                "2001:0db8:86a3:08d3:1319:8a2e:0370:7344",
                Some("2001:*:*:*:*:*:*:*"),
            ),
            (DesensitizedType::Passport, "EM1234567", Some("EM*****67")),
            (
                DesensitizedType::CreditCode,
                "91110108MA01ABCDE7",
                Some("9111**********CDE7"),
            ),
            (DesensitizedType::FirstMask, "123", Some("1**")),
            (DesensitizedType::ClearToEmpty, "100", Some("")),
            (DesensitizedType::ClearToNull, "100", None),
        ];
        for (kind, input, expected) in cases {
            assert_eq!(
                DesensitizedUtil::desensitized(Some(input), kind).as_deref(),
                expected
            );
        }
        assert_eq!(
            DesensitizedUtil::desensitized(None, DesensitizedType::ClearToNull),
            Some(String::new())
        );
    }

    #[test]
    fn individual_masking_functions_cover_invalid_short_and_unicode_inputs() {
        assert_eq!(DesensitizedUtil::clear(), "");
        assert_eq!(DesensitizedUtil::clear_to_null(), None);
        assert_eq!(DesensitizedUtil::user_id(), 0);
        assert_eq!(DesensitizedUtil::first_mask(None), "");
        assert_eq!(DesensitizedUtil::chinese_name(Some("李雷")), "李*");
        assert_eq!(DesensitizedUtil::id_card_num(Some("123"), 2, 2), "");
        assert_eq!(DesensitizedUtil::id_card_num(Some("123"), -1, 1), "");
        assert_eq!(DesensitizedUtil::id_card_num(None, 1, 1), "");
        assert_eq!(DesensitizedUtil::fixed_phone(Some("12345")), "12345");
        assert_eq!(DesensitizedUtil::mobile_phone(Some("123456")), "123456");
        assert_eq!(
            DesensitizedUtil::address(Some("北京市海淀区马连洼街道289号"), 5),
            "北京市海淀区马连洼街*****"
        );
        assert_eq!(DesensitizedUtil::address(Some("地址"), 50), "**");
        assert_eq!(DesensitizedUtil::address(Some("地址"), -1), "地址");
        assert_eq!(DesensitizedUtil::email(Some("a@b.cn")), "a@b.cn");
        assert_eq!(DesensitizedUtil::email(Some("plain")), "plain");
        assert_eq!(DesensitizedUtil::password(Some("密碼🔒")), "***");
        assert_eq!(DesensitizedUtil::car_license(Some("京A123")), "京A123");
        assert_eq!(
            DesensitizedUtil::car_license(Some("陕A12345D")),
            "陕A1****D"
        );
    }

    #[test]
    fn bank_network_passport_and_credit_code_paths_are_complete() {
        assert_eq!(DesensitizedUtil::bank_card(None), None);
        assert_eq!(DesensitizedUtil::bank_card(Some("")), Some(String::new()));
        assert_eq!(
            DesensitizedUtil::bank_card(Some("\u{200c}")),
            Some("\u{200c}".to_owned())
        );
        assert_eq!(
            DesensitizedUtil::bank_card(Some("1234\u{200c}56789")).as_deref(),
            Some("1234 **** 9")
        );
        assert_eq!(
            DesensitizedUtil::bank_card(Some("12345678")).as_deref(),
            Some("12345678")
        );
        assert_eq!(
            DesensitizedUtil::bank_card(Some("1234 2222 3333 4444 6789 9")).as_deref(),
            Some("1234 **** **** **** **** 9")
        );
        assert_eq!(
            DesensitizedUtil::bank_card(Some("1234 2222 3333 4444 6789")).as_deref(),
            Some("1234 **** **** **** 6789")
        );
        assert_eq!(DesensitizedUtil::ipv4("localhost"), "localhost.*.*.*");
        assert_eq!(
            DesensitizedUtil::ipv6("localhost"),
            "localhost:*:*:*:*:*:*:*"
        );
        assert_eq!(DesensitizedUtil::passport(None), None);
        assert_eq!(DesensitizedUtil::passport(Some("")), Some(String::new()));
        assert_eq!(
            DesensitizedUtil::passport(Some("\u{3164}")),
            Some("\u{3164}".to_owned())
        );
        assert_eq!(DesensitizedUtil::passport(Some("3")).as_deref(), Some("*"));
        assert_eq!(DesensitizedUtil::credit_code(None), None);
        assert_eq!(DesensitizedUtil::credit_code(Some("")), Some(String::new()));
        assert_eq!(
            DesensitizedUtil::credit_code(Some("\u{2800}")),
            Some("\u{2800}".to_owned())
        );
        assert_eq!(
            DesensitizedUtil::credit_code(Some("1234")).as_deref(),
            Some("****")
        );
        assert_eq!(
            DesensitizedUtil::credit_code(Some("12345678")).as_deref(),
            Some("12345678")
        );
    }
}
