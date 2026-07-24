//! Emoji helpers aligned with Hutool `cn.hutool.extra.emoji.EmojiUtil`.
//!
//! Backed by the [`emojis`] crate (GitHub gemoji shortcodes) rather than
//! emoji-java, with the same facade shapes Hutool callers expect.

use std::collections::BTreeSet;

/// Lightweight emoji metadata returned by lookup helpers.
///
/// Java: `com.vdurmont.emoji.Emoji` stand-in (unicode + shortcode only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emoji {
    unicode: String,
    shortcode: Option<String>,
    name: String,
}

impl Emoji {
    /// Returns the emoji unicode string.
    #[must_use]
    pub fn unicode(&self) -> &str {
        &self.unicode
    }

    /// Returns the primary GitHub shortcode when present.
    #[must_use]
    pub fn shortcode(&self) -> Option<&str> {
        self.shortcode.as_deref()
    }

    /// Returns the emoji CLDR / gemoji name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}
