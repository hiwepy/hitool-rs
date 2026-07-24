//! Hutool-aligned provider and model constants.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Supported Gemini video durations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeminiDurationSeconds {
    Four,
    Six,
    Eight,
}

impl GeminiDurationSeconds {
    /// Duration in seconds.
    #[must_use]
    pub const fn value(self) -> u8 {
        match self {
            Self::Four => 4,
            Self::Six => 6,
            Self::Eight => 8,
        }
    }
}
