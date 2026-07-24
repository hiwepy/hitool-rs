//! Hutool-named dynamic JWT facade backed by `jsonwebtoken`.

use std::fmt;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use base64::engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD};
use jsonwebtoken::crypto;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use p256::elliptic_curve::sec1::ToEncodedPoint as _;
use rsa::pkcs1::{
    DecodeRsaPrivateKey as _, DecodeRsaPublicKey as _, EncodeRsaPrivateKey as _,
    EncodeRsaPublicKey as _,
};
use rsa::pkcs8::{DecodePrivateKey as _, DecodePublicKey as _, EncodePrivateKey as _};
use serde_json::{Map, Value};

use super::jwt::JWT;

/// Structured JWT facade error.
#[derive(Debug, thiserror::Error)]
pub enum JWTException {
    /// A token, claim, or algorithm is invalid.
    #[error("{0}")]
    Invalid(String),
    /// JSON serialization or parsing failed.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// Base64 decoding failed.
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
    /// The cryptographic engine rejected a key or operation.
    #[error(transparent)]
    Crypto(#[from] jsonwebtoken::errors::Error),
}

impl JWTException {
    /// Creates a message error.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self::Invalid(message.into())
    }

    /// Replaces sequential `{}` placeholders.
    #[must_use]
    pub fn formatted(template: &str, values: &[&dyn fmt::Display]) -> Self {
        let mut message = String::new();
        let mut remaining = template;
        for value in values {
            if let Some(index) = remaining.find("{}") {
                message.push_str(&remaining[..index]);
                message.push_str(&value.to_string());
                remaining = &remaining[index + 2..];
            } else {
                break;
            }
        }
        message.push_str(remaining);
        Self::new(message)
    }
}
