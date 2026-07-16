//! Provider-neutral chat model APIs and an OpenAI-compatible implementation.

#![forbid(unsafe_code)]

use async_trait::async_trait;
use futures_core::Stream;
use hitool_http::{HttpClient, Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::{fmt, pin::Pin, sync::Arc};
use thiserror::Error;

/// A chat message role.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System-level instructions.
    System,
    /// End-user content.
    User,
    /// Model-generated content.
    Assistant,
    /// Tool output supplied to the model.
    Tool,
}

/// One chat message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    /// Message role.
    pub role: Role,
    /// Text content.
    pub content: String,
}

impl Message {
    /// Creates a system message.
    #[must_use]
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    /// Creates a user message.
    #[must_use]
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    /// Creates an assistant message.
    #[must_use]
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

/// A provider-neutral chat request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    /// Provider model identifier. `None` selects the provider default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Ordered conversation messages.
    pub messages: Vec<Message>,
    /// Optional sampling temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Optional output-token ceiling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

impl ChatRequest {
    /// Creates a request with one user message and provider defaults.
    #[must_use]
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            model: None,
            messages: vec![Message::user(content)],
            temperature: None,
            max_tokens: None,
        }
    }
}

/// Token accounting reported by a provider.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Usage {
    /// Input/prompt token count.
    #[serde(default)]
    pub prompt_tokens: u64,
    /// Generated token count.
    #[serde(default)]
    pub completion_tokens: u64,
    /// Total token count.
    #[serde(default)]
    pub total_tokens: u64,
}

/// A normalized non-streaming chat response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    /// Provider request identifier.
    pub id: String,
    /// Model that produced the response.
    pub model: String,
    /// First returned assistant message.
    pub message: Message,
    /// Optional finish reason.
    pub finish_reason: Option<String>,
    /// Provider token accounting.
    pub usage: Usage,
}

/// One incremental streaming delta.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatChunk {
    /// Provider request identifier.
    pub id: String,
    /// Incremental text, if any.
    pub content: Option<String>,
    /// Finish reason on the terminal chunk.
    pub finish_reason: Option<String>,
}

/// A boxed provider stream.
pub type ChatStream =
    Pin<Box<dyn Stream<Item = Result<ChatChunk, ProviderError>> + Send + 'static>>;

/// AI provider failures.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// HTTP transport or response decoding failed.
    #[error(transparent)]
    Http(#[from] hitool_http::HttpError),
    /// The base URL is invalid.
    #[error(transparent)]
    Url(#[from] url::ParseError),
    /// Provider payload was not valid JSON.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// Provider returned no choices.
    #[error("provider returned no chat choices")]
    EmptyChoices,
    /// Streaming is not implemented by this provider.
    #[error("provider does not support streaming")]
    StreamingUnsupported,
    /// One server-sent event exceeded the defensive parser limit.
    #[error("provider stream event exceeds {limit} bytes")]
    StreamEventTooLarge {
        /// Maximum accepted bytes per event.
        limit: usize,
    },
}

/// A provider-neutral asynchronous chat interface.
#[async_trait]
pub trait ChatProvider: Send + Sync {
    /// Completes one chat request.
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError>;

    /// Starts a streaming completion when supported.
    async fn stream(&self, _request: ChatRequest) -> Result<ChatStream, ProviderError> {
        Err(ProviderError::StreamingUnsupported)
    }
}

/// OpenAI-compatible `/chat/completions` provider.
pub struct OpenAiCompatibleProvider {
    client: HttpClient,
    base_url: Url,
    api_key: Arc<SecretString>,
    default_model: String,
}

impl fmt::Debug for OpenAiCompatibleProvider {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenAiCompatibleProvider")
            .field("base_url", &self.base_url)
            .field("api_key", &"[REDACTED]")
            .field("default_model", &self.default_model)
            .finish_non_exhaustive()
    }
}

impl OpenAiCompatibleProvider {
    /// Creates an OpenAI-compatible provider.
    pub fn new(
        client: HttpClient,
        base_url: impl AsRef<str>,
        api_key: impl Into<String>,
        default_model: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let mut base_url = Url::parse(base_url.as_ref())?;
        if !base_url.path().ends_with('/') {
            let path = format!("{}/", base_url.path());
            base_url.set_path(&path);
        }
        Ok(Self {
            client,
            base_url,
            api_key: Arc::new(SecretString::from(api_key.into())),
            default_model: default_model.into(),
        })
    }
}

#[derive(Debug, Serialize)]
struct OpenAiRequest<'a> {
    model: &'a str,
    messages: &'a [Message],
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponse {
    id: String,
    model: String,
    choices: Vec<OpenAiChoice>,
    #[serde(default)]
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: Message,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiStreamResponse {
    id: String,
    choices: Vec<OpenAiStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiStreamChoice {
    #[serde(default)]
    delta: OpenAiDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct OpenAiDelta {
    content: Option<String>,
}

const MAX_SSE_EVENT_BYTES: usize = 256 * 1024;

#[derive(Debug)]
struct SseDecoder {
    pending: Vec<u8>,
    data: Vec<u8>,
    max_event_bytes: usize,
}

impl SseDecoder {
    fn new(max_event_bytes: usize) -> Self {
        Self {
            pending: Vec::new(),
            data: Vec::new(),
            max_event_bytes,
        }
    }

    fn push(&mut self, chunk: &[u8]) -> Result<Vec<Vec<u8>>, ProviderError> {
        self.pending.extend_from_slice(chunk);
        let mut events = Vec::new();
        while let Some(newline) = self.pending.iter().position(|byte| *byte == b'\n') {
            let mut line: Vec<u8> = self.pending.drain(..=newline).collect();
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            if line.is_empty() {
                if !self.data.is_empty() {
                    self.data.pop();
                    events.push(std::mem::take(&mut self.data));
                }
                continue;
            }
            if let Some(value) = line.strip_prefix(b"data:") {
                let value = value.strip_prefix(b" ").unwrap_or(value);
                let next_len = self.data.len().saturating_add(value.len() + 1);
                if next_len > self.max_event_bytes {
                    return Err(ProviderError::StreamEventTooLarge {
                        limit: self.max_event_bytes,
                    });
                }
                self.data.extend_from_slice(value);
                self.data.push(b'\n');
            }
        }
        if self.pending.len() > self.max_event_bytes {
            return Err(ProviderError::StreamEventTooLarge {
                limit: self.max_event_bytes,
            });
        }
        Ok(events)
    }
}

#[async_trait]
impl ChatProvider for OpenAiCompatibleProvider {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError> {
        let endpoint = self.base_url.join("chat/completions")?;
        let model = request.model.as_deref().unwrap_or(&self.default_model);
        let payload = OpenAiRequest {
            model,
            messages: &request.messages,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: false,
        };
        let response: OpenAiResponse = self
            .client
            .send_json(
                self.client
                    .request(Method::POST, endpoint)
                    .bearer_auth(self.api_key.expose_secret())
                    .json(&payload),
            )
            .await?;
        let choice = response
            .choices
            .into_iter()
            .next()
            .ok_or(ProviderError::EmptyChoices)?;
        Ok(ChatResponse {
            id: response.id,
            model: response.model,
            message: choice.message,
            finish_reason: choice.finish_reason,
            usage: response.usage,
        })
    }

    async fn stream(&self, request: ChatRequest) -> Result<ChatStream, ProviderError> {
        let endpoint = self.base_url.join("chat/completions")?;
        let model = request.model.as_deref().unwrap_or(&self.default_model);
        let payload = OpenAiRequest {
            model,
            messages: &request.messages,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: true,
        };
        let mut response = self
            .client
            .send(
                self.client
                    .request(Method::POST, endpoint)
                    .bearer_auth(self.api_key.expose_secret())
                    .header("accept", "text/event-stream")
                    .json(&payload),
            )
            .await?;

        Ok(Box::pin(async_stream::try_stream! {
            let mut decoder = SseDecoder::new(MAX_SSE_EVENT_BYTES);
            'stream: while let Some(chunk) = response.chunk().await.map_err(hitool_http::HttpError::from)? {
                for event in decoder.push(&chunk)? {
                    if event == b"[DONE]" {
                        break 'stream;
                    }
                    let response: OpenAiStreamResponse = serde_json::from_slice(&event)?;
                    for choice in response.choices {
                        yield ChatChunk {
                            id: response.id.clone(),
                            content: choice.delta.content,
                            finish_reason: choice.finish_reason,
                        };
                    }
                }
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hitool_http::HttpConfig;

    #[test]
    fn provider_debug_redacts_api_key_and_normalizes_base_url() {
        let client = HttpClient::new(&HttpConfig::default()).unwrap();
        let provider =
            OpenAiCompatibleProvider::new(client, "https://example.com/v1", "top-secret", "model")
                .unwrap();
        let debug = format!("{provider:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains("top-secret"));
        assert_eq!(provider.base_url.as_str(), "https://example.com/v1/");
    }

    #[test]
    fn request_constructor_is_provider_neutral() {
        let request = ChatRequest::user("hello");
        assert_eq!(request.messages, [Message::user("hello")]);
        assert!(request.model.is_none());
    }

    #[test]
    fn sse_decoder_handles_chunk_boundaries_and_crlf() {
        let mut decoder = SseDecoder::new(1024);
        assert!(decoder.push(b"data: {\"id\":\"1").unwrap().is_empty());
        let events = decoder
            .push(b"\",\"choices\":[]}\r\n\r\ndata: [DONE]\n\n")
            .unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], br#"{"id":"1","choices":[]}"#);
        assert_eq!(events[1], b"[DONE]");
    }

    #[test]
    fn sse_decoder_bounds_unterminated_input() {
        let mut decoder = SseDecoder::new(4);
        assert!(matches!(
            decoder.push(b"12345"),
            Err(ProviderError::StreamEventTooLarge { limit: 4 })
        ));
    }
}
