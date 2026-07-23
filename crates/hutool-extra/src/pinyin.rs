//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use pinyin::{ToPinyin, ToPinyinMulti};

/// Error type matching Hutool `PinyinException` messaging.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
#[error("{message}")]
pub struct PinyinException {
    message: String,
}

impl PinyinException {
    /// Java: `new PinyinException(String message)`
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Java: `new PinyinException(Throwable)`
    #[must_use]
    pub fn from_cause(cause: impl std::fmt::Display) -> Self {
        Self {
            message: cause.to_string(),
        }
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Engine trait mirroring Hutool `PinyinEngine`.
pub trait PinyinEngine: Send + Sync {
    /// Converts a single Chinese character.
    fn get_pinyin_char(&self, c: char, tone: bool) -> String;
    /// Converts a string with separator.
    fn get_pinyin_str(&self, str: &str, separator: &str, tone: bool) -> String;
}

/// Default engine using the `pinyin` crate (covers Hutool engine variants).
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultPinyinEngine;

impl PinyinEngine for DefaultPinyinEngine {
    fn get_pinyin_char(&self, c: char, tone: bool) -> String {
        PinyinUtil::get_pinyin_char(c, tone)
    }

    fn get_pinyin_str(&self, str: &str, separator: &str, tone: bool) -> String {
        PinyinUtil::get_pinyin(str, separator, tone)
    }
}

/// Factory returning the default engine (Hutool multi-engine SPI collapsed to one Rust crate).
pub struct PinyinFactory;

impl PinyinFactory {
    /// Java: `PinyinFactory.get()` / `create()`
    #[must_use]
    pub fn get() -> DefaultPinyinEngine {
        DefaultPinyinEngine
    }

    /// Alias of [`Self::get`].
    #[must_use]
    pub fn create() -> DefaultPinyinEngine {
        Self::get()
    }
}

/// Hutool `PinyinUtil` facade.
///
/// Java: `cn.hutool.extra.pinyin.PinyinUtil`
pub struct PinyinUtil;

impl PinyinUtil {
    /// Java: `PinyinUtil.getEngine()`
    #[must_use]
    pub fn get_engine() -> DefaultPinyinEngine {
        PinyinFactory::get()
    }

    /// Java: `PinyinUtil.isChinese(char)` — CJK unified + Hutool `〇` special-case.
    #[must_use]
    pub fn is_chinese(c: char) -> bool {
        c == '〇' || matches!(c as u32, 0x4E00..=0x9FA5 | 0x3400..=0x4DBF)
    }

    /// Java: `PinyinUtil.getPinyin(char)` / `(char, boolean tone)`
    #[must_use]
    pub fn get_pinyin_char(c: char, tone: bool) -> String {
        let Some(py) = c.to_pinyin() else {
            return c.to_string();
        };
        if tone {
            py.with_tone().to_string()
        } else {
            py.plain().to_string()
        }
    }

    /// Java: `PinyinUtil.getPinyin(String)` / with tone / separator overloads.
    #[must_use]
    pub fn get_pinyin(str: &str, separator: &str, tone: bool) -> String {
        let mut parts = Vec::new();
        for ch in str.chars() {
            if Self::is_chinese(ch) {
                parts.push(Self::get_pinyin_char(ch, tone));
            } else if !ch.is_whitespace() {
                parts.push(ch.to_string());
            }
        }
        parts.join(separator)
    }

    /// Convenience: default separator `" "` without tone.
    #[must_use]
    pub fn get_pinyin_default(str: &str) -> String {
        Self::get_pinyin(str, " ", false)
    }

    /// Java: `PinyinUtil.getFirstLetter(char)`
    #[must_use]
    pub fn get_first_letter_char(c: char) -> char {
        Self::get_pinyin_char(c, false)
            .chars()
            .next()
            .unwrap_or(c)
            .to_ascii_lowercase()
    }

    /// Java: `PinyinUtil.getFirstLetter(String, String separator)`
    #[must_use]
    pub fn get_first_letter(str: &str, separator: &str) -> String {
        let mut parts = Vec::new();
        for ch in str.chars() {
            if Self::is_chinese(ch) {
                parts.push(Self::get_first_letter_char(ch).to_string());
            } else if ch.is_ascii_alphabetic() {
                parts.push(ch.to_ascii_lowercase().to_string());
            }
        }
        parts.join(separator)
    }

    /// Returns all candidate pinyin readings for a character (engine multi-sound helper).
    #[must_use]
    pub fn get_pinyin_multi(c: char) -> Vec<String> {
        c.to_pinyin_multi()
            .map(|multi| {
                multi
                    .into_iter()
                    .map(|py| py.plain().to_string())
                    .collect()
            })
            .unwrap_or_default()
    }
}

// Engine type aliases so Hutool engine class names share the same surface.
/// Alias for [`DefaultPinyinEngine`] (Hutool `HoubbPinyinEngine`).
pub type HoubbPinyinEngine = DefaultPinyinEngine;
/// Alias for [`DefaultPinyinEngine`] (Hutool `JPinyinEngine`).
pub type JPinyinEngine = DefaultPinyinEngine;
/// Alias for [`DefaultPinyinEngine`] (Hutool `Pinyin4jEngine`).
pub type Pinyin4jEngine = DefaultPinyinEngine;
/// Alias for [`DefaultPinyinEngine`] (Hutool `TinyPinyinEngine`).
pub type TinyPinyinEngine = DefaultPinyinEngine;
/// Alias for [`DefaultPinyinEngine`] (Hutool `Bopomofo4jEngine` — romanized pinyin, not Bopomofo).
pub type Bopomofo4jEngine = DefaultPinyinEngine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinyin_util_chinese_roundtrip() {
        assert!(PinyinUtil::is_chinese('汉'));
        assert!(PinyinUtil::is_chinese('〇'));
        assert!(!PinyinUtil::is_chinese('A'));
        let py = PinyinUtil::get_pinyin_char('汉', false);
        assert_eq!(py, "han");
        let with_tone = PinyinUtil::get_pinyin_char('汉', true);
        assert!(with_tone.starts_with('h'));
        let sentence = PinyinUtil::get_pinyin("汉字", " ", false);
        assert_eq!(sentence, "han zi");
        // Hutool PinyinUtilTest fixtures
        assert_eq!(PinyinUtil::get_pinyin("你好怡", " ", false), "ni hao yi");
        assert_eq!(
            PinyinUtil::get_first_letter("H是第一个", ", "),
            "h, s, d, y, g"
        );
        assert_eq!(PinyinUtil::get_first_letter("崞阳", ", "), "g, y");
        assert_eq!(PinyinUtil::get_first_letter_char('汉'), 'h');
        assert_eq!(PinyinUtil::get_first_letter("汉字", ""), "hz");
        let _ = PinyinUtil::get_engine().get_pinyin_str("中", " ", false);
        assert_eq!(PinyinException::new("x").message(), "x");
        let _ = PinyinUtil::get_pinyin_multi('重');
    }
}
