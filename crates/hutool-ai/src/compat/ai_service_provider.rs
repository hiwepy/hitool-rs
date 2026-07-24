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

/// Provider plug-in contract used by custom registries.
pub trait AIServiceProvider: fmt::Debug + Send + Sync {
    /// Provider identifier.
    fn service_name(&self) -> ModelName;
    /// Creates a service from explicit configuration.
    fn create(&self, config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError>;
}
