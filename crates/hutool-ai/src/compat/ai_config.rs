//! Hutool-compatible configuration, factory, and service facade.

#![allow(clippy::missing_panics_doc)]

use crate::{AIResponse, Message, ModelName, Operation, ProviderError, StreamCallback};
use async_trait::async_trait;
use hutool_http::{HttpClient, Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde_json::{Map, Value};
use std::{fmt, sync::Arc, time::Duration};

/// Shared configuration contract for all Hutool AI providers.
pub trait AIConfig: fmt::Debug + Send + Sync {
    /// Provider name.
    fn model_name(&self) -> ModelName;
    /// Secret provider credential.
    fn api_key(&self) -> &SecretString;
    /// Base API URL.
    fn api_url(&self) -> &Url;
    /// Concrete model identifier.
    fn model(&self) -> &str;
    /// Additional request fields.
    fn additional(&self) -> &Map<String, Value>;
    /// Connection/request timeout.
    fn timeout(&self) -> Duration;
    /// Streaming read timeout.
    fn read_timeout(&self) -> Duration;
    /// Optional HTTP proxy URL.
    fn proxy(&self) -> Option<&Url>;
}
