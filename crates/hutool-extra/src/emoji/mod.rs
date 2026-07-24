//! Emoji helpers aligned with Hutool `cn.hutool.extra.emoji.EmojiUtil`.
//!
//! Backed by the [`emojis`] crate (GitHub gemoji shortcodes) rather than
//! emoji-java, with the same facade shapes Hutool callers expect.

use std::collections::BTreeSet;

mod fitzpatrick_action;
mod emoji;
mod emoji_util;

pub use fitzpatrick_action::FitzpatrickAction;
pub use emoji::Emoji;
pub use emoji_util::EmojiUtil;
