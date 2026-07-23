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

string_enum!(HutoolModel { Hutool => "hutool" });
string_enum!(DeepSeekModel {
    Chat => "deepseek-chat",
    Reasoner => "deepseek-reasoner"
});
string_enum!(OllamaModel { Qwen3_32b => "qwen3:32b" });
string_enum!(OpenAiModel {
    Gpt4o => "gpt-4o",
    Gpt4oMini => "gpt-4o-mini",
    O1 => "o1",
    O3Mini => "o3-mini",
    DallE3 => "dall-e-3",
    DallE2 => "dall-e-2",
    Tts1 => "tts-1",
    Whisper1 => "whisper-1",
    TextEmbedding3Large => "text-embedding-3-large",
    OmniModerationLatest => "omni-moderation-latest"
});
string_enum!(DoubaoModel {
    Pro32k => "doubao-1.5-pro-32k-250115",
    Pro256k => "doubao-1.5-pro-256k-250115",
    Lite32k => "doubao-1.5-lite-32k-250115",
    VisionPro32k => "doubao-1.5-vision-pro-32k-250115",
    EmbeddingLarge => "doubao-embedding-large-text-240915",
    EmbeddingVision => "doubao-embedding-vision-241215",
    Seedream3 => "doubao-seedream-3-0-t2i-250415",
    Seedance1Lite => "doubao-seedance-1-0-lite-t2v-250428"
});
string_enum!(GrokModel {
    Grok3 => "grok-3-beta",
    Grok3Mini => "grok-3-mini-beta",
    Grok2 => "grok-2-1212",
    Grok2Image => "grok-2-image-1212",
    Grok2Vision => "grok-2-vision-1212"
});
string_enum!(GeminiModel {
    Gemini25Pro => "gemini-2.5-pro",
    Gemini25Flash => "gemini-2.5-flash",
    Gemini25FlashImage => "gemini-2.5-flash-image",
    Imagen4 => "imagen-4.0-generate-001",
    Veo3 => "veo-3.0-generate-001",
    Gemini25FlashTts => "gemini-2.5-flash-preview-tts"
});

string_enum!(VisionDetail { Auto => "auto", Low => "low", High => "high" });
/// Hutool, Doubao, Grok, and `OpenAI` expose the same vision-detail vocabulary.
pub type HutoolVision = VisionDetail;
/// Doubao vision detail.
pub type DoubaoVision = VisionDetail;
/// Grok vision detail.
pub type GrokVision = VisionDetail;
/// `OpenAI` vision detail.
pub type OpenAiVision = VisionDetail;

string_enum!(SpeechVoice {
    Alloy => "alloy", Ash => "ash", Coral => "coral", Echo => "echo",
    Fable => "fable", Onyx => "onyx", Nova => "nova", Sage => "sage",
    Shimmer => "shimmer"
});
/// Hutool speech voices.
pub type HutoolSpeech = SpeechVoice;
/// `OpenAI` speech voices.
pub type OpenAiSpeech = SpeechVoice;

string_enum!(ReasoningEffort { Low => "low", Medium => "medium", High => "high" });
string_enum!(DoubaoContext { Session => "session", CommonPrefix => "common_prefix" });
string_enum!(OllamaFormat { Json => "json", None => "" });
string_enum!(GeminiImageSize { Size1k => "1K", Size2k => "2K" });
string_enum!(GeminiAspectRatio {
    Square => "1:1", Portrait3x4 => "3:4", Landscape4x3 => "4:3",
    Portrait9x16 => "9:16", Landscape16x9 => "16:9"
});
string_enum!(GeminiPersonGeneration {
    DontAllow => "dont_allow", AllowAdult => "allow_adult", AllowAll => "allow_all"
});
string_enum!(GeminiVoice {
    Aoede => "Aoede", Charon => "Charon", Kore => "Kore", Fenrir => "Fenrir", Puck => "Puck"
});

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_names_defaults_and_parsing_are_complete() {
        let providers = [
            ModelName::Hutool,
            ModelName::DeepSeek,
            ModelName::OpenAi,
            ModelName::Doubao,
            ModelName::Grok,
            ModelName::Ollama,
            ModelName::Gemini,
        ];
        for provider in providers {
            let (url, model) = provider.defaults();
            assert!(url.starts_with("http"));
            assert!(!model.is_empty());
            assert_eq!(ModelName::parse(provider.value()), Some(provider));
        }
        assert_eq!(ModelName::parse("DEEPSEEK"), Some(ModelName::DeepSeek));
        assert_eq!(ModelName::parse("unknown"), None);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn model_and_common_wire_values_match_hutool() {
        assert_eq!(HutoolModel::Hutool.value(), "hutool");
        for (actual, expected) in [
            (DeepSeekModel::Chat.value(), "deepseek-chat"),
            (DeepSeekModel::Reasoner.value(), "deepseek-reasoner"),
            (OllamaModel::Qwen3_32b.value(), "qwen3:32b"),
            (OpenAiModel::Gpt4o.value(), "gpt-4o"),
            (OpenAiModel::Gpt4oMini.value(), "gpt-4o-mini"),
            (OpenAiModel::O1.value(), "o1"),
            (OpenAiModel::O3Mini.value(), "o3-mini"),
            (OpenAiModel::DallE3.value(), "dall-e-3"),
            (OpenAiModel::DallE2.value(), "dall-e-2"),
            (OpenAiModel::Tts1.value(), "tts-1"),
            (OpenAiModel::Whisper1.value(), "whisper-1"),
            (
                OpenAiModel::TextEmbedding3Large.value(),
                "text-embedding-3-large",
            ),
            (
                OpenAiModel::OmniModerationLatest.value(),
                "omni-moderation-latest",
            ),
            (DoubaoModel::Pro32k.value(), "doubao-1.5-pro-32k-250115"),
            (DoubaoModel::Pro256k.value(), "doubao-1.5-pro-256k-250115"),
            (DoubaoModel::Lite32k.value(), "doubao-1.5-lite-32k-250115"),
            (
                DoubaoModel::VisionPro32k.value(),
                "doubao-1.5-vision-pro-32k-250115",
            ),
            (
                DoubaoModel::EmbeddingLarge.value(),
                "doubao-embedding-large-text-240915",
            ),
            (
                DoubaoModel::EmbeddingVision.value(),
                "doubao-embedding-vision-241215",
            ),
            (
                DoubaoModel::Seedream3.value(),
                "doubao-seedream-3-0-t2i-250415",
            ),
            (
                DoubaoModel::Seedance1Lite.value(),
                "doubao-seedance-1-0-lite-t2v-250428",
            ),
            (GrokModel::Grok3.value(), "grok-3-beta"),
            (GrokModel::Grok3Mini.value(), "grok-3-mini-beta"),
            (GrokModel::Grok2.value(), "grok-2-1212"),
            (GrokModel::Grok2Image.value(), "grok-2-image-1212"),
            (GrokModel::Grok2Vision.value(), "grok-2-vision-1212"),
            (GeminiModel::Gemini25Pro.value(), "gemini-2.5-pro"),
            (GeminiModel::Gemini25Flash.value(), "gemini-2.5-flash"),
            (
                GeminiModel::Gemini25FlashImage.value(),
                "gemini-2.5-flash-image",
            ),
            (GeminiModel::Imagen4.value(), "imagen-4.0-generate-001"),
            (GeminiModel::Veo3.value(), "veo-3.0-generate-001"),
            (
                GeminiModel::Gemini25FlashTts.value(),
                "gemini-2.5-flash-preview-tts",
            ),
        ] {
            assert_eq!(actual, expected);
        }

        for (actual, expected) in [
            (VisionDetail::Auto.value(), "auto"),
            (VisionDetail::Low.value(), "low"),
            (VisionDetail::High.value(), "high"),
            (SpeechVoice::Alloy.value(), "alloy"),
            (SpeechVoice::Ash.value(), "ash"),
            (SpeechVoice::Coral.value(), "coral"),
            (SpeechVoice::Echo.value(), "echo"),
            (SpeechVoice::Fable.value(), "fable"),
            (SpeechVoice::Onyx.value(), "onyx"),
            (SpeechVoice::Nova.value(), "nova"),
            (SpeechVoice::Sage.value(), "sage"),
            (SpeechVoice::Shimmer.value(), "shimmer"),
            (ReasoningEffort::Low.value(), "low"),
            (ReasoningEffort::Medium.value(), "medium"),
            (ReasoningEffort::High.value(), "high"),
            (DoubaoContext::Session.value(), "session"),
            (DoubaoContext::CommonPrefix.value(), "common_prefix"),
            (OllamaFormat::Json.value(), "json"),
            (OllamaFormat::None.value(), ""),
            (GeminiImageSize::Size1k.value(), "1K"),
            (GeminiImageSize::Size2k.value(), "2K"),
            (GeminiAspectRatio::Square.value(), "1:1"),
            (GeminiAspectRatio::Portrait3x4.value(), "3:4"),
            (GeminiAspectRatio::Landscape4x3.value(), "4:3"),
            (GeminiAspectRatio::Portrait9x16.value(), "9:16"),
            (GeminiAspectRatio::Landscape16x9.value(), "16:9"),
            (GeminiPersonGeneration::DontAllow.value(), "dont_allow"),
            (GeminiPersonGeneration::AllowAdult.value(), "allow_adult"),
            (GeminiPersonGeneration::AllowAll.value(), "allow_all"),
            (GeminiVoice::Aoede.value(), "Aoede"),
            (GeminiVoice::Charon.value(), "Charon"),
            (GeminiVoice::Kore.value(), "Kore"),
            (GeminiVoice::Fenrir.value(), "Fenrir"),
            (GeminiVoice::Puck.value(), "Puck"),
        ] {
            assert_eq!(actual, expected);
        }
        assert_eq!(
            [
                GeminiImageCount::One.count(),
                GeminiImageCount::Two.count(),
                GeminiImageCount::Three.count(),
                GeminiImageCount::Four.count()
            ],
            [1, 2, 3, 4]
        );
        assert_eq!(
            [
                GeminiDurationSeconds::Four.value(),
                GeminiDurationSeconds::Six.value(),
                GeminiDurationSeconds::Eight.value()
            ],
            [4, 6, 8]
        );
        assert_eq!(
            [
                OllamaOptions::TEMPERATURE,
                OllamaOptions::TOP_P,
                OllamaOptions::TOP_K,
                OllamaOptions::NUM_PREDICT,
                OllamaOptions::SEED
            ],
            ["temperature", "top_p", "top_k", "num_predict", "seed"]
        );
    }
}
