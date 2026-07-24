//! Hutool-compatible configuration, factory, and service facade.

#![allow(clippy::missing_panics_doc)]

use crate::{AIResponse, Message, ModelName, Operation, ProviderError, StreamCallback};
use async_trait::async_trait;
use hutool_http::{HttpClient, Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde_json::{Map, Value};
use std::{fmt, sync::Arc, time::Duration};

use super::ai_service::AIService;
use super::ai_service_factory::AIServiceFactory;
use super::base_config::BaseConfig;

/// Static convenience facade matching Hutool's `AIUtil` role.
pub struct AIUtil;

impl AIUtil {
    /// Creates a built-in service.
    pub fn get_ai_service(config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError> {
        AIServiceFactory::get_ai_service(config)
    }
    /// Sends one prompt as a user message.
    pub async fn chat(
        config: BaseConfig,
        prompt: impl Into<String>,
    ) -> Result<String, ProviderError> {
        let prompt = prompt.into();
        Self::chat_messages(config, vec![Message::user(&prompt)]).await
    }
    /// Sends an ordered conversation.
    pub async fn chat_messages(
        config: BaseConfig,
        messages: Vec<Message>,
    ) -> Result<String, ProviderError> {
        Self::get_ai_service(config)
            .expect("validated built-in provider configuration creates a service")
            .chat(messages)
            .await
    }
}
