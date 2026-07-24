//! Hutool-aligned provider and model constants.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Ollama option keys.
pub struct OllamaOptions;

impl OllamaOptions {
    /// Temperature option.
    pub const TEMPERATURE: &'static str = "temperature";
    /// Nucleus-sampling option.
    pub const TOP_P: &'static str = "top_p";
    /// Top-k option.
    pub const TOP_K: &'static str = "top_k";
    /// Maximum predicted tokens.
    pub const NUM_PREDICT: &'static str = "num_predict";
    /// Random seed.
    pub const SEED: &'static str = "seed";
}
