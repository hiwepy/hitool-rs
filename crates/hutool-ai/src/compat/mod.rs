//! Hutool-compatible configuration, factory, and service facade.

#![allow(clippy::missing_panics_doc)]

use crate::{AIResponse, Message, ModelName, Operation, ProviderError, StreamCallback};
use async_trait::async_trait;
use hutool_http::{HttpClient, Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde_json::{Map, Value};
use std::{fmt, sync::Arc, time::Duration};

mod ai_config;
mod base_config;
mod ai_config_builder;
mod ai_service;
mod ai_service_provider;
mod provider_service;
mod ai_service_factory;
mod ai_util;

pub use ai_config::AIConfig;
pub use base_config::BaseConfig;
pub use ai_config_builder::AIConfigBuilder;
pub use ai_service::AIService;
pub use ai_service_provider::AIServiceProvider;
pub use provider_service::ProviderService;
pub use ai_service_factory::AIServiceFactory;
pub use ai_util::AIUtil;
