//! Hutool-compatible configuration, factory, and service facade.

#![allow(clippy::missing_panics_doc)]

use crate::{AIResponse, Message, ModelName, Operation, ProviderError, StreamCallback};
use async_trait::async_trait;
use hutool_http::{HttpClient, Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde_json::{Map, Value};
use std::{fmt, sync::Arc, time::Duration};

use super::base_config::BaseConfig;

/// Chainable configuration builder corresponding to Hutool's builder.
#[derive(Debug, Clone)]
pub struct AIConfigBuilder {
    config: BaseConfig,
}

impl AIConfigBuilder {
    /// Creates a builder from a case-insensitive Hutool provider name.
    pub fn new(model_name: &str) -> Result<Self, ProviderError> {
        let provider = ModelName::parse(model_name)
            .ok_or_else(|| ProviderError::UnsupportedProvider(model_name.into()))?;
        Ok(Self {
            config: BaseConfig::new(provider).expect("built-in provider defaults are valid"),
        })
    }
    /// Sets the API key.
    #[must_use]
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.config.set_api_key(value);
        self
    }
    /// Sets the API URL.
    pub fn api_url(mut self, value: impl AsRef<str>) -> Result<Self, ProviderError> {
        self.config.set_api_url(value)?;
        Ok(self)
    }
    /// Sets the model.
    #[must_use]
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.config.set_model(value);
        self
    }
    /// Adds a dynamic request field.
    #[must_use]
    pub fn additional(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.config.put_additional(key, value);
        self
    }
    /// Sets the request timeout.
    #[must_use]
    pub fn timeout(mut self, value: Duration) -> Self {
        self.config.set_timeout(value);
        self
    }
    /// Sets the read timeout.
    #[must_use]
    pub fn read_timeout(mut self, value: Duration) -> Self {
        self.config.set_read_timeout(value);
        self
    }
    /// Sets an HTTP proxy.
    pub fn proxy(mut self, value: impl AsRef<str>) -> Result<Self, ProviderError> {
        self.config.set_proxy(value)?;
        Ok(self)
    }
    /// Returns the validated owned configuration.
    #[must_use]
    pub fn build(self) -> BaseConfig {
        self.config
    }
}
