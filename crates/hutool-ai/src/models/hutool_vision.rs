//! Hutool-aligned provider and model constants.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Hutool, Doubao, Grok, and `OpenAI` expose the same vision-detail vocabulary.
pub type HutoolVision = VisionDetail;
