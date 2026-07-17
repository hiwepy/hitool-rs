//! Provider-neutral chat model APIs and an OpenAI-compatible implementation.

#![forbid(unsafe_code)]
#![allow(async_fn_in_trait)]

mod compat;
mod models;
mod operations;

pub use compat::{
    AIConfig, AIConfigBuilder, AIService, AIServiceFactory, AIServiceProvider, AIUtil, BaseConfig,
    ProviderService,
};
pub use models::*;
pub use operations::{AIResponse, Operation, StreamCallback, VideoParameter};

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
    pub fn system(content: &str) -> Self {
        Self {
            role: Role::System,
            content: content.to_owned(),
        }
    }

    /// Creates a user message.
    #[must_use]
    pub fn user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: content.to_owned(),
        }
    }

    /// Creates an assistant message.
    #[must_use]
    pub fn assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: content.to_owned(),
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
    pub fn user(content: &str) -> Self {
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
    /// The requested Hutool provider name is unknown.
    #[error("unsupported AI provider: {0}")]
    UnsupportedProvider(String),
    /// A bounded binary or JSON response exceeded the safety limit.
    #[error("provider response exceeds {limit} bytes")]
    ResponseTooLarge {
        /// Maximum accepted response size.
        limit: usize,
    },
}

/// A provider-neutral asynchronous chat interface.
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

impl ChatProvider for OpenAiCompatibleProvider {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError> {
        let endpoint = self
            .base_url
            .join("chat/completions")
            .expect("the fixed chat endpoint is a valid relative URL");
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
        let endpoint = self
            .base_url
            .join("chat/completions")
            .expect("the fixed chat endpoint is a valid relative URL");
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
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    async fn server(
        responses: Vec<(&'static str, Vec<u8>)>,
    ) -> (String, tokio::task::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let task = tokio::spawn(async move {
            for (content_type, body) in responses {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut request = vec![0; 8192];
                let _ = socket.read(&mut request).await.unwrap();
                let header = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: {content_type}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                    body.len()
                );
                socket.write_all(header.as_bytes()).await.unwrap();
                socket.write_all(&body).await.unwrap();
            }
        });
        (format!("http://{address}/v1/"), task)
    }

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
        let client = HttpClient::new(&HttpConfig::default()).unwrap();
        let provider =
            OpenAiCompatibleProvider::new(client, "https://example.com/v1/", "secret", "model")
                .unwrap();
        assert_eq!(provider.base_url.as_str(), "https://example.com/v1/");
        let client = HttpClient::new(&HttpConfig::default()).unwrap();
        assert!(OpenAiCompatibleProvider::new(client, "bad url", "x", "m").is_err());
    }

    #[test]
    fn request_constructor_is_provider_neutral() {
        let request = ChatRequest::user("hello");
        assert_eq!(request.messages, [Message::user("hello")]);
        assert!(request.model.is_none());
        assert_eq!(Message::system("rules").role, Role::System);
        assert_eq!(Message::assistant("answer").role, Role::Assistant);
        let tool = Message {
            role: Role::Tool,
            content: "result".into(),
        };
        assert_eq!(
            serde_json::from_str::<Message>(&serde_json::to_string(&tool).unwrap()).unwrap(),
            tool
        );
        for role in [Role::System, Role::User, Role::Assistant, Role::Tool] {
            let encoded = serde_json::to_string(&role).unwrap();
            assert_eq!(serde_json::from_str::<Role>(&encoded).unwrap(), role);
        }
        let mut complete = ChatRequest::user("all fields");
        complete.model = Some("model".into());
        complete.temperature = Some(0.1);
        complete.max_tokens = Some(2);
        let encoded = serde_json::to_string(&complete).unwrap();
        let decoded: ChatRequest = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.model.as_deref(), Some("model"));
        let minimal = serde_json::to_string(&ChatRequest::user("minimal")).unwrap();
        assert!(!minimal.contains("temperature"));
    }

    #[test]
    fn response_types_and_errors_round_trip() {
        let response = ChatResponse {
            id: "id".into(),
            model: "model".into(),
            message: Message::assistant("ok"),
            finish_reason: Some("stop".into()),
            usage: Usage {
                prompt_tokens: 1,
                completion_tokens: 2,
                total_tokens: 3,
            },
        };
        let encoded = serde_json::to_string(&response).unwrap();
        let decoded: ChatResponse = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.id, "id");
        let chunk = ChatChunk {
            id: "id".into(),
            content: None,
            finish_reason: Some("stop".into()),
        };
        assert_eq!(
            serde_json::from_str::<ChatChunk>(&serde_json::to_string(&chunk).unwrap()).unwrap(),
            chunk
        );
        assert_eq!(
            Usage::default(),
            Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0
            }
        );
        for error in [
            ProviderError::EmptyChoices,
            ProviderError::StreamingUnsupported,
            ProviderError::StreamEventTooLarge { limit: 1 },
            ProviderError::UnsupportedProvider("x".into()),
            ProviderError::ResponseTooLarge { limit: 2 },
            ProviderError::Url(Url::parse("bad url").unwrap_err()),
            ProviderError::Json(serde_json::from_str::<serde_json::Value>("{").unwrap_err()),
        ] {
            assert!(!error.to_string().is_empty());
        }
    }

    #[derive(Debug)]
    struct NonStreaming;
    impl ChatProvider for NonStreaming {
        async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError> {
            Ok(ChatResponse {
                id: "local".into(),
                model: "local".into(),
                message: request.messages[0].clone(),
                finish_reason: None,
                usage: Usage::default(),
            })
        }
    }

    #[tokio::test]
    async fn default_provider_stream_is_explicitly_unsupported() {
        assert_eq!(
            NonStreaming.chat(ChatRequest::user("x")).await.unwrap().id,
            "local"
        );
        assert_eq!(
            NonStreaming
                .stream(ChatRequest::user("x"))
                .await
                .err()
                .map(|error| error.to_string()),
            Some("provider does not support streaming".into())
        );
    }

    #[tokio::test]
    async fn openai_provider_executes_chat_empty_choices_and_streams() {
        let successful = br#"{"id":"one","model":"chosen","choices":[{"message":{"role":"assistant","content":"ok"},"finish_reason":"stop"}],"usage":{"prompt_tokens":1,"completion_tokens":2,"total_tokens":3}}"#.to_vec();
        let empty = br#"{"id":"two","model":"default","choices":[]}"#.to_vec();
        let sse = b"data:{\"id\":\"stream\",\"choices\":[{\"delta\":{\"content\":\"hi\"},\"finish_reason\":null},{\"delta\":{},\"finish_reason\":\"stop\"}]}\n\ndata: [DONE]\n\n".to_vec();
        let (url, task) = server(vec![
            ("application/json", successful),
            ("application/json", empty),
            ("text/event-stream", sse),
        ])
        .await;
        let client = HttpClient::new(&HttpConfig::default()).unwrap();
        let provider = OpenAiCompatibleProvider::new(client, &url, "key", "default").unwrap();
        let mut request = ChatRequest::user("hello");
        request.model = Some("chosen".into());
        request.temperature = Some(0.2);
        request.max_tokens = Some(4);
        let response = provider.chat(request).await.unwrap();
        assert_eq!(response.message, Message::assistant("ok"));
        assert_eq!(
            provider
                .chat(ChatRequest::user("empty"))
                .await
                .err()
                .map(|error| error.to_string()),
            Some("provider returned no chat choices".into())
        );
        let mut stream = provider.stream(ChatRequest::user("stream")).await.unwrap();
        let first = std::future::poll_fn(|cx| stream.as_mut().poll_next(cx))
            .await
            .unwrap()
            .unwrap();
        let second = std::future::poll_fn(|cx| stream.as_mut().poll_next(cx))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(first.content.as_deref(), Some("hi"));
        assert_eq!(second.finish_reason.as_deref(), Some("stop"));
        assert!(
            std::future::poll_fn(|cx| stream.as_mut().poll_next(cx))
                .await
                .is_none()
        );
        task.await.unwrap();
    }

    #[tokio::test]
    async fn openai_provider_propagates_transport_errors() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/v1/", listener.local_addr().unwrap());
        drop(listener);
        let client = HttpClient::new(&HttpConfig::default()).unwrap();
        let provider = OpenAiCompatibleProvider::new(client, url, "key", "model").unwrap();
        assert!(provider.chat(ChatRequest::user("x")).await.is_err());
        assert!(provider.stream(ChatRequest::user("x")).await.is_err());
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
        assert_eq!(
            decoder.push(b"12345").unwrap_err().to_string(),
            "provider stream event exceeds 4 bytes"
        );
        let mut decoder = SseDecoder::new(4);
        assert_eq!(
            decoder.push(b"data: 12345\n").unwrap_err().to_string(),
            "provider stream event exceeds 4 bytes"
        );
        let mut decoder = SseDecoder::new(20);
        assert!(decoder.push(b"event: ping\n\n").unwrap().is_empty());
    }
}
