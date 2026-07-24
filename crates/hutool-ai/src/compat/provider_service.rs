//! Hutool-compatible configuration, factory, and service facade.

#![allow(clippy::missing_panics_doc)]

use crate::{AIResponse, Message, ModelName, Operation, ProviderError, StreamCallback};
use async_trait::async_trait;
use hutool_http::{HttpClient, Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde_json::{Map, Value};
use std::{fmt, sync::Arc, time::Duration};

use super::ai_service::AIService;
use super::base_config::BaseConfig;

/// Unified production HTTP implementation for Hutool provider capabilities.
#[derive(Debug, Clone)]
pub struct ProviderService {
    config: BaseConfig,
    client: HttpClient,
    max_response_bytes: usize,
}

impl ProviderService {
    /// Builds a pooled, Rustls-backed provider client.
    pub fn new(config: BaseConfig) -> Result<Self, ProviderError> {
        let mut builder = HttpClient::builder()
            .timeout(config.timeout())
            .max_response_size(MAX_MEDIA_BYTES);
        if let Some(proxy) = config.proxy() {
            builder = builder
                .proxy(proxy.as_str())
                .expect("validated proxy URLs are accepted by reqwest");
        }
        let client = builder
            .build()
            .expect("fixed Rustls HTTP client configuration is valid");
        Ok(Self {
            config,
            client,
            max_response_bytes: MAX_MEDIA_BYTES,
        })
    }

    /// Builds a service from an application-managed client and response limit.
    pub fn with_client(
        config: BaseConfig,
        client: HttpClient,
        max_response_bytes: usize,
    ) -> Result<Self, ProviderError> {
        if max_response_bytes == 0 {
            return Err(ProviderError::ResponseTooLarge { limit: 0 });
        }
        Ok(Self {
            config,
            client,
            max_response_bytes,
        })
    }

    fn request(&self, operation: &Operation, stream: bool) -> reqwest::RequestBuilder {
        let endpoint = operation.endpoint(self.config.model_name(), self.config.model());
        let mut url = self.config.api_url().clone();
        let root = url.path().trim_end_matches('/');
        url.set_path(&format!("{root}{endpoint}"));
        if self.config.model_name() == ModelName::Gemini {
            url.query_pairs_mut()
                .append_pair("key", self.config.api_key().expose_secret());
        }
        let method = match operation {
            Operation::ListModels
            | Operation::ListLanguageModels
            | Operation::Balance
            | Operation::GetModel { .. }
            | Operation::GetLanguageModel { .. }
            | Operation::GetVideo { .. }
            | Operation::DeferredCompletion { .. } => Method::GET,
            Operation::DeleteModel { .. } => Method::DELETE,
            _ => Method::POST,
        };
        let payload = operation.payload(self.config.model(), self.config.additional(), stream);
        let mut request = self
            .client
            .request(method, url)
            .header("accept", "application/json");
        if !matches!(
            self.config.model_name(),
            ModelName::Gemini | ModelName::Ollama
        ) {
            request = request.bearer_auth(self.config.api_key().expose_secret());
        }
        request.json(&payload)
    }
}

impl AIService for ProviderService {
    async fn execute(&self, operation: Operation) -> Result<AIResponse, ProviderError> {
        let binary = matches!(operation, Operation::TextToSpeech { .. });
        let response = self.client.send(self.request(&operation, false)).await?;
        let bytes = response
            .bytes()
            .await
            .map_err(hutool_http::HttpError::from)?;
        if bytes.len() > self.max_response_bytes {
            return Err(ProviderError::ResponseTooLarge {
                limit: self.max_response_bytes,
            });
        }
        if binary {
            Ok(AIResponse::Bytes(bytes.to_vec()))
        } else {
            Ok(AIResponse::Json(serde_json::from_slice(&bytes)?))
        }
    }

    async fn execute_stream(
        &self,
        operation: Operation,
        callback: StreamCallback,
    ) -> Result<(), ProviderError> {
        let mut response = self.client.send(self.request(&operation, true)).await?;
        let mut decoder = crate::SseDecoder::new(crate::MAX_SSE_EVENT_BYTES);
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(hutool_http::HttpError::from)?
        {
            for event in decoder.push(&chunk)? {
                if event == b"[DONE]" {
                    return Ok(());
                }
                callback(String::from_utf8_lossy(&event).into_owned());
            }
        }
        Ok(())
    }
}

const MAX_MEDIA_BYTES: usize = 64 * 1024 * 1024;
