//! Hutool-compatible configuration, factory, and service facade.

#![allow(clippy::missing_panics_doc)]

use crate::{AIResponse, Message, ModelName, Operation, ProviderError, StreamCallback};
use async_trait::async_trait;
use hutool_http::{HttpClient, Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde_json::{Map, Value};
use std::{fmt, sync::Arc, time::Duration};

/// Common asynchronous AI service contract.
#[async_trait]
pub trait AIService: fmt::Debug + Send + Sync {
    /// Executes one typed provider operation.
    async fn execute(&self, operation: Operation) -> Result<AIResponse, ProviderError>;
    /// Executes an operation and delivers stream events.
    async fn execute_stream(
        &self,
        operation: Operation,
        callback: StreamCallback,
    ) -> Result<(), ProviderError>;
    /// Convenience chat operation.
    async fn chat(&self, messages: Vec<Message>) -> Result<String, ProviderError> {
        Ok(self
            .execute(Operation::Chat { messages })
            .await?
            .into_text())
    }
}
