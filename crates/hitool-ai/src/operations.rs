//! Provider capability model used by the thin Hutool-compatible facade.

#![allow(missing_docs, clippy::enum_glob_use, clippy::match_same_arms)]

use crate::Message;
use serde_json::{Map, Value, json};
use std::{path::PathBuf, sync::Arc};

/// Thread-safe callback receiving one provider stream event.
pub type StreamCallback = Arc<dyn Fn(String) + Send + Sync + 'static>;

/// A video generation command-line style parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct VideoParameter {
    /// Provider option name, such as `--rt`.
    pub kind: String,
    /// Provider option value.
    pub value: Value,
}

impl VideoParameter {
    /// Creates a typed video option.
    #[must_use]
    pub fn new(kind: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            kind: kind.into(),
            value: value.into(),
        }
    }
}

/// All capabilities exposed by Hutool AI providers.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Operation {
    /// Chat completion.
    Chat { messages: Vec<Message> },
    /// Chat with image URLs or data URLs.
    Vision {
        prompt: String,
        images: Vec<String>,
        detail: String,
    },
    /// Provider-specific beta/reasoning prompt.
    Beta { prompt: String },
    /// Grok message completion with a token ceiling.
    Message {
        messages: Vec<Message>,
        max_tokens: u32,
    },
    /// Provider model catalogue.
    ListModels,
    /// Fetch one model.
    GetModel { id: String },
    /// Grok language-model catalogue.
    ListLanguageModels,
    /// Fetch one Grok language model.
    GetLanguageModel { id: String },
    /// `DeepSeek` account balance.
    Balance,
    /// Text embedding.
    EmbedText { inputs: Vec<String> },
    /// Multimodal embedding.
    EmbedVision { text: String, image: String },
    /// Tokenization.
    Tokenize { texts: Vec<String> },
    /// Image generation.
    GenerateImage { prompt: String },
    /// `OpenAI` image editing.
    EditImage {
        prompt: String,
        image: PathBuf,
        mask: Option<PathBuf>,
    },
    /// `OpenAI` image variation.
    VaryImage { image: PathBuf },
    /// Text-to-speech.
    TextToSpeech { input: String, voice: String },
    /// Speech-to-text.
    SpeechToText { file: PathBuf },
    /// Content moderation.
    Moderate {
        text: String,
        image_url: Option<String>,
    },
    /// Video generation task.
    CreateVideo {
        text: String,
        image: Option<String>,
        parameters: Vec<VideoParameter>,
    },
    /// Poll video task or Gemini operation.
    GetVideo { id: String },
    /// Doubao bot chat.
    BotChat { messages: Vec<Message> },
    /// Doubao batch chat.
    BatchChat { messages: Vec<Message> },
    /// Doubao context creation.
    CreateContext {
        messages: Vec<Message>,
        mode: String,
    },
    /// Doubao context chat.
    ContextChat {
        messages: Vec<Message>,
        context_id: String,
    },
    /// Gemini multimodal generation.
    Multimodal { prompt: String, media: Vec<String> },
    /// Gemini JSON-mode chat.
    JsonChat { messages: Vec<Message> },
    /// Gemini file upload.
    UploadFile { file: PathBuf },
    /// Ollama generation.
    Generate {
        prompt: String,
        format: Option<String>,
    },
    /// Ollama embeddings.
    Embeddings { prompt: String },
    /// Ollama model inspection.
    ShowModel { name: String },
    /// Ollama model pull.
    PullModel { name: String },
    /// Ollama model deletion.
    DeleteModel { name: String },
    /// Ollama model copy.
    CopyModel { source: String, destination: String },
    /// Fetch an already-deferred Grok completion.
    DeferredCompletion { request_id: String },
}

impl Operation {
    pub(crate) fn endpoint(&self, provider: crate::ModelName, model: &str) -> String {
        use Operation::*;
        if provider == crate::ModelName::Gemini {
            return match self {
                Chat { .. } | Multimodal { .. } | JsonChat { .. } => {
                    format!("/models/{model}:generateContent")
                }
                GenerateImage { .. } => format!("/models/{model}:predict"),
                CreateVideo { .. } => format!("/models/{model}:predictLongRunning"),
                GetVideo { id } => format!("/{id}"),
                TextToSpeech { .. } => format!("/models/{model}:generateContent"),
                UploadFile { .. } => "/upload/v1beta/files".into(),
                _ => "/models".into(),
            };
        }
        if provider == crate::ModelName::Ollama {
            return match self {
                Chat { .. } => "/api/chat",
                Generate { .. } => "/api/generate",
                Embeddings { .. } | EmbedText { .. } => "/api/embeddings",
                ListModels => "/api/tags",
                ShowModel { .. } => "/api/show",
                PullModel { .. } => "/api/pull",
                DeleteModel { .. } => "/api/delete",
                CopyModel { .. } => "/api/copy",
                _ => "/api/generate",
            }
            .into();
        }
        match self {
            Chat { .. } | Vision { .. } | Message { .. } | BotChat { .. } | ContextChat { .. } => {
                "/chat/completions"
            }
            Beta { .. } => "/beta/completions",
            ListModels => "/models",
            GetModel { id } => return format!("/models/{id}"),
            ListLanguageModels => "/language-models",
            GetLanguageModel { id } => return format!("/language-models/{id}"),
            Balance => "/user/balance",
            EmbedText { .. } | Embeddings { .. } => "/embeddings",
            EmbedVision { .. } => "/embeddings/multimodal",
            Tokenize { .. } => "/tokenization",
            GenerateImage { .. } => "/images/generations",
            EditImage { .. } => "/images/edits",
            VaryImage { .. } => "/images/variations",
            TextToSpeech { .. } => "/audio/speech",
            SpeechToText { .. } => "/audio/transcriptions",
            Moderate { .. } => "/moderations",
            CreateVideo { .. } => "/contents/generations/tasks",
            GetVideo { id } => return format!("/contents/generations/tasks/{id}"),
            BatchChat { .. } => "/batch/chat/completions",
            CreateContext { .. } => "/context/create",
            Multimodal { .. } | JsonChat { .. } => "/chat/completions",
            UploadFile { .. } => "/files",
            Generate { .. } => "/completions",
            ShowModel { .. } | PullModel { .. } | DeleteModel { .. } | CopyModel { .. } => {
                "/models"
            }
            DeferredCompletion { request_id } => {
                return format!("/chat/deferred-completion/{request_id}");
            }
        }
        .into()
    }

    pub(crate) fn payload(
        &self,
        model: &str,
        additional: &Map<String, Value>,
        stream: bool,
    ) -> Value {
        use Operation::*;
        let mut value = match self {
            Chat { messages } | BotChat { messages } | BatchChat { messages } => {
                json!({"model": model, "messages": messages, "stream": stream})
            }
            Vision {
                prompt,
                images,
                detail,
            } => {
                let mut content = vec![json!({"type":"text", "text":prompt})];
                content.extend(images.iter().map(
                    |image| json!({"type":"image_url", "image_url":{"url":image,"detail":detail}}),
                ));
                json!({"model":model,"messages":[{"role":"user","content":content}],"stream":stream})
            }
            Beta { prompt } | Generate { prompt, .. } => {
                json!({"model":model,"prompt":prompt,"stream":stream})
            }
            Message {
                messages,
                max_tokens,
            } => json!({"model":model,"messages":messages,"max_tokens":max_tokens,"stream":stream}),
            EmbedText { inputs } => json!({"model":model,"input":inputs}),
            EmbedVision { text, image } => {
                json!({"model":model,"input":[{"type":"text","text":text},{"type":"image_url","image_url":image}]})
            }
            Tokenize { texts } => json!({"model":model,"text":texts}),
            GenerateImage { prompt } => json!({"model":model,"prompt":prompt}),
            EditImage {
                prompt,
                image,
                mask,
            } => json!({"model":model,"prompt":prompt,"image":image,"mask":mask}),
            VaryImage { image } => json!({"model":model,"image":image}),
            TextToSpeech { input, voice } => json!({"model":model,"input":input,"voice":voice}),
            SpeechToText { file } | UploadFile { file } => json!({"model":model,"file":file}),
            Moderate { text, image_url } => {
                json!({"model":model,"input":{"text":text,"image_url":image_url}})
            }
            CreateVideo {
                text,
                image,
                parameters,
            } => {
                json!({"model":model,"text":text,"image":image,"parameters":parameters.iter().map(|p| json!({"type":p.kind,"value":p.value})).collect::<Vec<_>>() })
            }
            GetVideo { id } | GetModel { id } | GetLanguageModel { id } => json!({"id":id}),
            CreateContext { messages, mode } => {
                json!({"model":model,"messages":messages,"mode":mode})
            }
            ContextChat {
                messages,
                context_id,
            } => json!({"model":model,"messages":messages,"context_id":context_id,"stream":stream}),
            Multimodal { prompt, media } => {
                json!({"contents":[{"parts":[{"text":prompt},{"media":media}]}]})
            }
            JsonChat { messages } => {
                json!({"model":model,"messages":messages,"response_format":{"type":"json_object"}})
            }
            Embeddings { prompt } => json!({"model":model,"prompt":prompt}),
            ShowModel { name } | PullModel { name } | DeleteModel { name } => json!({"name":name}),
            CopyModel {
                source,
                destination,
            } => json!({"source":source,"destination":destination}),
            DeferredCompletion { request_id } => json!({"request_id":request_id}),
            ListModels | ListLanguageModels | Balance => Value::Object(Map::new()),
        };
        let object = value
            .as_object_mut()
            .expect("all operation payloads are JSON objects");
        for (key, item) in additional {
            object.entry(key.clone()).or_insert_with(|| item.clone());
        }
        if let Generate {
            format: Some(format),
            ..
        } = self
        {
            object.insert("format".into(), Value::String(format.clone()));
        }
        value
    }
}

/// Normalized raw provider response.
#[derive(Debug, Clone, PartialEq)]
pub enum AIResponse {
    /// JSON payload.
    Json(Value),
    /// Binary media payload.
    Bytes(Vec<u8>),
}

impl AIResponse {
    /// Serializes JSON or returns a lossy textual representation of bytes.
    #[must_use]
    pub fn into_text(self) -> String {
        match self {
            Self::Json(value) => value.to_string(),
            Self::Bytes(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        }
    }

    /// Extracts binary media, serializing JSON when necessary.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Json(value) => value.to_string().into_bytes(),
            Self::Bytes(bytes) => bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ModelName, Role};
    use std::path::PathBuf;

    fn messages() -> Vec<Message> {
        vec![Message::user("hello")]
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn every_operation_builds_a_bounded_explicit_request() {
        let parameter = VideoParameter::new("--dur", 5);
        assert_eq!(parameter.kind, "--dur");
        let operations = vec![
            Operation::Chat {
                messages: messages(),
            },
            Operation::Vision {
                prompt: "p".into(),
                images: vec!["i".into()],
                detail: "high".into(),
            },
            Operation::Beta { prompt: "p".into() },
            Operation::Message {
                messages: messages(),
                max_tokens: 9,
            },
            Operation::ListModels,
            Operation::GetModel { id: "m".into() },
            Operation::ListLanguageModels,
            Operation::GetLanguageModel { id: "m".into() },
            Operation::Balance,
            Operation::EmbedText {
                inputs: vec!["a".into()],
            },
            Operation::EmbedVision {
                text: "a".into(),
                image: "i".into(),
            },
            Operation::Tokenize {
                texts: vec!["a".into()],
            },
            Operation::GenerateImage { prompt: "p".into() },
            Operation::EditImage {
                prompt: "p".into(),
                image: PathBuf::from("i"),
                mask: Some(PathBuf::from("m")),
            },
            Operation::VaryImage {
                image: PathBuf::from("i"),
            },
            Operation::TextToSpeech {
                input: "p".into(),
                voice: "alloy".into(),
            },
            Operation::SpeechToText {
                file: PathBuf::from("a.wav"),
            },
            Operation::Moderate {
                text: "t".into(),
                image_url: Some("i".into()),
            },
            Operation::CreateVideo {
                text: "t".into(),
                image: Some("i".into()),
                parameters: vec![parameter],
            },
            Operation::GetVideo { id: "task".into() },
            Operation::BotChat {
                messages: messages(),
            },
            Operation::BatchChat {
                messages: messages(),
            },
            Operation::CreateContext {
                messages: messages(),
                mode: "session".into(),
            },
            Operation::ContextChat {
                messages: messages(),
                context_id: "c".into(),
            },
            Operation::Multimodal {
                prompt: "p".into(),
                media: vec!["m".into()],
            },
            Operation::JsonChat {
                messages: messages(),
            },
            Operation::UploadFile {
                file: PathBuf::from("f"),
            },
            Operation::Generate {
                prompt: "p".into(),
                format: Some("json".into()),
            },
            Operation::Embeddings { prompt: "p".into() },
            Operation::ShowModel { name: "m".into() },
            Operation::PullModel { name: "m".into() },
            Operation::DeleteModel { name: "m".into() },
            Operation::CopyModel {
                source: "a".into(),
                destination: "b".into(),
            },
            Operation::DeferredCompletion {
                request_id: "r".into(),
            },
        ];
        let mut additional = Map::new();
        additional.insert("temperature".into(), json!(0.2));
        for operation in operations {
            assert!(
                operation
                    .endpoint(ModelName::OpenAi, "model")
                    .starts_with('/')
            );
            assert!(operation.payload("model", &additional, true).is_object());
        }
        let empty_mask = Operation::EditImage {
            prompt: "p".into(),
            image: "i".into(),
            mask: None,
        };
        assert!(empty_mask.payload("m", &Map::new(), false)["mask"].is_null());
        let no_format = Operation::Generate {
            prompt: "p".into(),
            format: None,
        };
        assert!(
            no_format
                .payload("m", &Map::new(), false)
                .get("format")
                .is_none()
        );
        let no_image = Operation::Moderate {
            text: "t".into(),
            image_url: None,
        };
        assert!(no_image.payload("m", &Map::new(), false)["input"]["image_url"].is_null());
    }

    #[test]
    fn gemini_and_ollama_routes_cover_provider_specific_shapes() {
        let gemini = [
            Operation::Chat {
                messages: messages(),
            },
            Operation::Multimodal {
                prompt: "p".into(),
                media: vec![],
            },
            Operation::JsonChat {
                messages: messages(),
            },
            Operation::GenerateImage { prompt: "p".into() },
            Operation::CreateVideo {
                text: "t".into(),
                image: None,
                parameters: vec![],
            },
            Operation::GetVideo {
                id: "operations/x".into(),
            },
            Operation::TextToSpeech {
                input: "t".into(),
                voice: "Kore".into(),
            },
            Operation::UploadFile { file: "f".into() },
            Operation::Balance,
        ];
        for operation in gemini {
            assert!(!operation.endpoint(ModelName::Gemini, "gemini").is_empty());
        }
        let ollama = [
            Operation::Chat {
                messages: messages(),
            },
            Operation::Generate {
                prompt: "p".into(),
                format: None,
            },
            Operation::Embeddings { prompt: "p".into() },
            Operation::ListModels,
            Operation::ShowModel { name: "m".into() },
            Operation::PullModel { name: "m".into() },
            Operation::DeleteModel { name: "m".into() },
            Operation::CopyModel {
                source: "a".into(),
                destination: "b".into(),
            },
            Operation::Balance,
        ];
        for operation in ollama {
            assert!(
                operation
                    .endpoint(ModelName::Ollama, "m")
                    .starts_with("/api/")
            );
        }
    }

    #[test]
    fn normalized_responses_convert_in_both_directions() {
        assert_eq!(
            AIResponse::Json(json!({"ok":true})).into_text(),
            "{\"ok\":true}"
        );
        assert_eq!(AIResponse::Bytes(b"ok".to_vec()).into_text(), "ok");
        assert_eq!(AIResponse::Bytes(vec![1, 2]).into_bytes(), vec![1, 2]);
        assert_eq!(AIResponse::Json(json!(1)).into_bytes(), b"1");
        assert_eq!(messages()[0].role, Role::User);
    }
}
