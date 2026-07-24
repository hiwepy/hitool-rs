//! Hutool-compatible configuration, factory, and service facade.

#![allow(clippy::missing_panics_doc)]

use crate::{AIResponse, Message, ModelName, Operation, ProviderError, StreamCallback};
use async_trait::async_trait;
use hutool_http::{HttpClient, Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde_json::{Map, Value};
use std::{fmt, sync::Arc, time::Duration};

use super::ai_config::AIConfig;

/// Owned provider configuration. Secrets are redacted from debug output.
#[derive(Clone)]
pub struct BaseConfig {
    provider: ModelName,
    api_key: Arc<SecretString>,
    api_url: Url,
    model: String,
    additional: Map<String, Value>,
    timeout: Duration,
    read_timeout: Duration,
    proxy: Option<Url>,
}

impl fmt::Debug for BaseConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BaseConfig")
            .field("provider", &self.provider)
            .field("api_key", &"[REDACTED]")
            .field("api_url", &self.api_url)
            .field("model", &self.model)
            .field("additional", &self.additional)
            .field("timeout", &self.timeout)
            .field("read_timeout", &self.read_timeout)
            .field("proxy", &self.proxy)
            .finish()
    }
}

impl BaseConfig {
    /// Creates the provider's Hutool-compatible defaults.
    pub fn new(provider: ModelName) -> Result<Self, ProviderError> {
        let (url, model) = provider.defaults();
        Ok(Self {
            provider,
            api_key: Arc::new(SecretString::from(String::new())),
            api_url: Url::parse(url).expect("built-in provider URL constants are valid"),
            model: model.into(),
            additional: Map::new(),
            timeout: DEFAULT_TIMEOUT,
            read_timeout: DEFAULT_READ_TIMEOUT,
            proxy: None,
        })
    }

    /// Creates defaults with a provider credential.
    pub fn with_api_key(
        provider: ModelName,
        api_key: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let mut config = Self::new(provider).expect("built-in provider defaults are valid");
        config.set_api_key(api_key);
        Ok(config)
    }

    /// Replaces the credential.
    pub fn set_api_key(&mut self, api_key: impl Into<String>) -> &mut Self {
        self.api_key = Arc::new(SecretString::from(api_key.into()));
        self
    }
    /// Replaces the API root.
    pub fn set_api_url(&mut self, api_url: impl AsRef<str>) -> Result<&mut Self, ProviderError> {
        self.api_url = Url::parse(api_url.as_ref())?;
        Ok(self)
    }
    /// Replaces the concrete model.
    pub fn set_model(&mut self, model: impl Into<String>) -> &mut Self {
        self.model = model.into();
        self
    }
    /// Adds a provider-specific request field.
    pub fn put_additional(&mut self, key: impl Into<String>, value: impl Into<Value>) -> &mut Self {
        self.additional.insert(key.into(), value.into());
        self
    }
    /// Returns one provider-specific request field.
    #[must_use]
    pub fn get_additional(&self, key: &str) -> Option<&Value> {
        self.additional.get(key)
    }
    /// Replaces the request timeout when non-zero.
    pub fn set_timeout(&mut self, timeout: Duration) -> &mut Self {
        if !timeout.is_zero() {
            self.timeout = timeout;
        }
        self
    }
    /// Replaces the stream read timeout when non-zero.
    pub fn set_read_timeout(&mut self, timeout: Duration) -> &mut Self {
        if !timeout.is_zero() {
            self.read_timeout = timeout;
        }
        self
    }
    /// Configures an HTTP proxy URL.
    pub fn set_proxy(&mut self, proxy: impl AsRef<str>) -> Result<&mut Self, ProviderError> {
        self.proxy = Some(Url::parse(proxy.as_ref())?);
        Ok(self)
    }
    /// Removes the HTTP proxy.
    pub fn clear_proxy(&mut self) -> &mut Self {
        self.proxy = None;
        self
    }
}

impl AIConfig for BaseConfig {
    fn model_name(&self) -> ModelName {
        self.provider
    }
    fn api_key(&self) -> &SecretString {
        &self.api_key
    }
    fn api_url(&self) -> &Url {
        &self.api_url
    }
    fn model(&self) -> &str {
        &self.model
    }
    fn additional(&self) -> &Map<String, Value> {
        &self.additional
    }
    fn timeout(&self) -> Duration {
        self.timeout
    }
    fn read_timeout(&self) -> Duration {
        self.read_timeout
    }
    fn proxy(&self) -> Option<&Url> {
        self.proxy.as_ref()
    }
}

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(180);

const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(300);
