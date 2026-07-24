//! Emoji helpers aligned with Hutool `cn.hutool.extra.emoji.EmojiUtil`.
//!
//! Backed by the [`emojis`] crate (GitHub gemoji shortcodes) rather than
//! emoji-java, with the same facade shapes Hutool callers expect.

use std::collections::BTreeSet;

use super::emoji::Emoji;
use super::fitzpatrick_action::FitzpatrickAction;

/// Hutool `EmojiUtil` facade.
///
/// Java: `cn.hutool.extra.emoji.EmojiUtil`
pub struct EmojiUtil;

impl EmojiUtil {
    /// Returns whether `str` is exactly one emoji (including ZWJ sequences).
    ///
    /// Java: `EmojiUtil.isEmoji(String)`
    #[must_use]
    pub fn is_emoji(s: &str) -> bool {
        let trimmed = s.trim();
        !trimmed.is_empty() && emojis::get(trimmed).is_some()
    }

    /// Returns whether `str` contains at least one emoji.
    ///
    /// Java: `EmojiUtil.containsEmoji(String)`
    #[must_use]
    pub fn contains_emoji(s: &str) -> bool {
        !Self::extract_emojis(s).is_empty()
    }

    /// Finds emojis whose name or shortcode contains `tag` (case-insensitive).
    ///
    /// Java: `EmojiUtil.getByTag(String)` — gemoji has no emoji-java tag set;
    /// name/shortcode substring match is the idiomatic stand-in.
    #[must_use]
    pub fn get_by_tag(tag: &str) -> BTreeSet<String> {
        let needle = tag.to_ascii_lowercase();
        let mut out = BTreeSet::new();
        for emoji in emojis::iter() {
            let name_hit = emoji.name().to_ascii_lowercase().contains(&needle);
            let code_hit = emoji
                .shortcodes()
                .any(|c| c.to_ascii_lowercase().contains(&needle));
            if name_hit || code_hit {
                out.insert(emoji.as_str().to_owned());
            }
        }
        out
    }

    /// Looks up an emoji by GitHub shortcode alias (with or without colons).
    ///
    /// Java: `EmojiUtil.get(String alias)`
    #[must_use]
    pub fn get(alias: &str) -> Option<Emoji> {
        let key = alias.trim().trim_matches(':');
        emojis::get_by_shortcode(key).map(from_static)
    }

    /// Replaces `:alias:` shortcodes and HTML decimal/hex entities with unicode.
    ///
    /// Java: `EmojiUtil.toUnicode(String)`
    #[must_use]
    pub fn to_unicode(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            // `:shortcode:` or `:shortcode|type_N:`
            if chars[i] == ':' {
                if let Some(end) = chars[i + 1..].iter().position(|&c| c == ':') {
                    let inner: String = chars[i + 1..i + 1 + end].iter().collect();
                    let alias = inner.split('|').next().unwrap_or(&inner);
                    if let Some(emoji) = emojis::get_by_shortcode(alias) {
                        out.push_str(emoji.as_str());
                        i += end + 2;
                        continue;
                    }
                }
            }
            // &#123; or &#x1f600;
            if chars[i] == '&' && i + 2 < chars.len() && chars[i + 1] == '#' {
                if let Some(semi) = chars[i + 2..].iter().position(|&c| c == ';') {
                    let body: String = chars[i + 2..i + 2 + semi].iter().collect();
                    if let Some(cp) = parse_html_codepoint(&body) {
                        if let Some(ch) = char::from_u32(cp) {
                            out.push(ch);
                            i += semi + 3;
                            continue;
                        }
                    }
                }
            }
            out.push(chars[i]);
            i += 1;
        }
        out
    }

    /// Converts unicode emoji to `:shortcode:` aliases.
    ///
    /// Java: `EmojiUtil.toAlias(String)`
    #[must_use]
    pub fn to_alias(s: &str) -> String {
        Self::to_alias_with(s, FitzpatrickAction::Parse)
    }

    /// Converts unicode emoji to aliases with Fitzpatrick handling.
    ///
    /// Java: `EmojiUtil.toAlias(String, FitzpatrickAction)`
    #[must_use]
    pub fn to_alias_with(s: &str, action: FitzpatrickAction) -> String {
        replace_emojis(s, |emoji, rest_tone| {
            let code = emoji.shortcode().unwrap_or(emoji.name());
            match action {
                FitzpatrickAction::Parse => {
                    if let Some(tone) = skin_tone_type(rest_tone) {
                        format!(":{code}|type_{tone}:")
                    } else {
                        format!(":{code}:")
                    }
                }
                FitzpatrickAction::Remove => format!(":{code}:"),
                FitzpatrickAction::Ignore => {
                    if rest_tone.is_empty() {
                        format!(":{code}:")
                    } else {
                        format!(":{code}:{rest_tone}")
                    }
                }
            }
        })
    }

    /// Converts emoji to HTML hexadecimal entities.
    ///
    /// Java: `EmojiUtil.toHtmlHex(String)`
    #[must_use]
    pub fn to_html_hex(s: &str) -> String {
        Self::to_html(s, true)
    }

    /// Converts emoji to HTML decimal entities (default) or hex when `is_hex`.
    ///
    /// Java: `EmojiUtil.toHtml(String)` / `toHtml(String, boolean)`
    #[must_use]
    pub fn to_html(s: &str, is_hex: bool) -> String {
        replace_emojis(s, |emoji, _| {
            emoji
                .as_str()
                .chars()
                .map(|c| {
                    if is_hex {
                        format!("&#x{:x};", c as u32)
                    } else {
                        format!("&#{};", c as u32)
                    }
                })
                .collect::<String>()
        })
    }

    /// Removes all emoji sequences from `s`.
    ///
    /// Java: `EmojiUtil.removeAllEmojis(String)`
    #[must_use]
    pub fn remove_all_emojis(s: &str) -> String {
        replace_emojis(s, |_, _| String::new())
    }

    /// Extracts every emoji sequence from `s` (left-to-right, longest match).
    ///
    /// Java: `EmojiUtil.extractEmojis(String)`
    #[must_use]
    pub fn extract_emojis(s: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut rest = s;
        while !rest.is_empty() {
            if let Some((emoji, consumed)) = match_emoji_prefix(rest) {
                out.push(emoji.as_str().to_owned());
                rest = &rest[consumed..];
            } else {
                let mut chars = rest.chars();
                chars.next();
                rest = chars.as_str();
            }
        }
        out
    }
}

fn match_emoji_prefix(s: &str) -> Option<(&'static emojis::Emoji, usize)> {
    let mut best: Option<(&'static emojis::Emoji, usize)> = None;
    for (idx, _) in s.char_indices() {
        let end = idx + s[idx..].chars().next()?.len_utf8();
        let candidate = &s[..end];
        if let Some(emoji) = emojis::get(candidate) {
            best = Some((emoji, end));
        }
    }
    // Also try full string when it is an exact multi-codepoint emoji.
    if let Some(emoji) = emojis::get(s) {
        return Some((emoji, s.len()));
    }
    best
}

fn parse_html_codepoint(body: &str) -> Option<u32> {
    if let Some(hex) = body
        .strip_prefix('x')
        .or_else(|| body.strip_prefix('X'))
    {
        u32::from_str_radix(hex, 16).ok()
    } else {
        body.parse().ok()
    }
}

fn from_static(emoji: &'static emojis::Emoji) -> Emoji {
    Emoji {
        unicode: emoji.as_str().to_owned(),
        shortcode: emoji.shortcode().map(str::to_owned),
        name: emoji.name().to_owned(),
    }
}

fn skin_tone_type(tone: &str) -> Option<u8> {
    match tone {
        "\u{1F3FB}" => Some(1),
        "\u{1F3FC}" => Some(2),
        "\u{1F3FD}" => Some(3),
        "\u{1F3FE}" => Some(4),
        "\u{1F3FF}" => Some(5),
        // emoji-java type_6 is the darkest; gemoji uses the same fifth modifier
        // as type_5 — keep type_5 for the last modifier to stay within 1..5.
        _ => None,
    }
}

fn replace_emojis(s: &str, mut map: impl FnMut(&'static emojis::Emoji, &str) -> String) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while !rest.is_empty() {
        if let Some((emoji, consumed)) = match_emoji_prefix(rest) {
            let after = &rest[consumed..];
            // Optional single Fitzpatrick modifier after the base emoji.
            let (tone, tone_len) = match after.chars().next() {
                Some(c) if (0x1F3FB..=0x1F3FF).contains(&(c as u32)) => {
                    let len = c.len_utf8();
                    (&after[..len], len)
                }
                _ => ("", 0),
            };
            out.push_str(&map(emoji, tone));
            rest = &rest[consumed + tone_len..];
        } else {
            let mut chars = rest.chars();
            if let Some(c) = chars.next() {
                out.push(c);
            }
            rest = chars.as_str();
        }
    }
    out
}
