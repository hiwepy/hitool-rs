//! Hutool-aligned provider and model constants.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Supported AI vendors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModelName {
    /// Hutool hosted gateway.
    Hutool,
    /// `DeepSeek`.
    DeepSeek,
    /// `OpenAI`.
    OpenAi,
    /// Volcano Engine Doubao.
    Doubao,
    /// xAI Grok.
    Grok,
    /// Local Ollama.
    Ollama,
    /// Google Gemini.
    Gemini,
}

impl ModelName {
    /// Hutool-compatible provider name.
    #[must_use]
    pub const fn value(self) -> &'static str {
        match self {
            Self::Hutool => "hutool",
            Self::DeepSeek => "deepSeek",
            Self::OpenAi => "openai",
            Self::Doubao => "doubao",
            Self::Grok => "grok",
            Self::Ollama => "ollama",
            Self::Gemini => "gemini",
        }
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "hutool" => Some(Self::Hutool),
            "deepseek" => Some(Self::DeepSeek),
            "openai" => Some(Self::OpenAi),
            "doubao" => Some(Self::Doubao),
            "grok" => Some(Self::Grok),
            "ollama" => Some(Self::Ollama),
            "gemini" => Some(Self::Gemini),
            _ => None,
        }
    }

    /// Default API root and model used by Hutool.
    #[must_use]
    pub const fn defaults(self) -> (&'static str, &'static str) {
        match self {
            Self::Hutool => ("https://api.hutool.cn/ai/api", "hutool"),
            Self::DeepSeek => ("https://api.deepseek.com", "deepseek-chat"),
            Self::OpenAi => ("https://api.openai.com/v1", "gpt-4o"),
            Self::Doubao => (
                "https://ark.cn-beijing.volces.com/api/v3",
                "doubao-1.5-lite-32k-250115",
            ),
            Self::Grok => ("https://api.x.ai/v1", "grok-2-1212"),
            Self::Ollama => ("http://localhost:11434", "qwen3:32b"),
            Self::Gemini => (
                "https://generativelanguage.googleapis.com/v1beta",
                "gemini-2.5-flash",
            ),
        }
    }
}

macro_rules! string_enum {
    ($name:ident { $($variant:ident => $value:literal),+ $(,)? }) => {
        #[doc = concat!("Hutool-aligned `", stringify!($name), "` values.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum $name { $($variant),+ }
        impl $name {
            /// Wire value expected by the provider.
            #[must_use]
            pub const fn value(self) -> &'static str {
                match self { $(Self::$variant => $value),+ }
            }
        }
    };
}
