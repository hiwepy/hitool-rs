//! Hutool-aligned provider and model constants.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

mod model_name;
mod hutool_vision;
mod doubao_vision;
mod grok_vision;
mod open_ai_vision;
mod hutool_speech;
mod open_ai_speech;
mod gemini_image_count;
mod gemini_duration_seconds;
mod ollama_options;

pub use model_name::ModelName;
pub use hutool_vision::HutoolVision;
pub use doubao_vision::DoubaoVision;
pub use grok_vision::GrokVision;
pub use open_ai_vision::OpenAiVision;
pub use hutool_speech::HutoolSpeech;
pub use open_ai_speech::OpenAiSpeech;
pub use gemini_image_count::GeminiImageCount;
pub use gemini_duration_seconds::GeminiDurationSeconds;
pub use ollama_options::OllamaOptions;
