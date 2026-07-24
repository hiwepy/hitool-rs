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
use super::provider_service::ProviderService;

/// Built-in provider factory.
pub struct AIServiceFactory;

impl AIServiceFactory {
    /// Creates the appropriate built-in service.
    pub fn get_ai_service(config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError> {
        ProviderService::new(config).map(|service| Arc::new(service) as Arc<dyn AIService>)
    }
}
