//! Hutool-aligned provider and model constants.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Number of Gemini images to generate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeminiImageCount {
    One,
    Two,
    Three,
    Four,
}

impl GeminiImageCount {
    /// Numeric image count.
    #[must_use]
    pub const fn count(self) -> u8 {
        self as u8 + 1
    }
}
