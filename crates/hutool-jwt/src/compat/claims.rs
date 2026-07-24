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
use super::jwt_exception::JWTException;

/// Mutable dynamic JSON claims.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Claims(Map<String, Value>);

impl Claims {
    /// Parses a JSON object.
    pub fn parse(input: &str) -> Result<Self, JWTException> {
        let value: Value = serde_json::from_str(input)?;
        value
            .as_object()
            .cloned()
            .map(Self)
            .ok_or_else(|| JWTException::new("JWT claims must be a JSON object"))
    }

    /// Returns a claim.
    #[must_use]
    pub fn get_claim(&self, name: &str) -> Option<&Value> {
        self.0.get(name)
    }

    /// Returns all claims.
    #[must_use]
    pub const fn claims_json(&self) -> &Map<String, Value> {
        &self.0
    }

    /// Sets a claim; JSON null removes it, matching Hutool.
    pub fn set_claim(&mut self, name: impl Into<String>, value: Value) -> &mut Self {
        let name = name.into();
        if value.is_null() {
            self.0.remove(&name);
        } else {
            self.0.insert(name, value);
        }
        self
    }

    /// Adds claims.
    pub fn put_all(&mut self, values: impl IntoIterator<Item = (String, Value)>) -> &mut Self {
        for (name, value) in values {
            self.set_claim(name, value);
        }
        self
    }

    /// Returns a claim as `i64`, matching Hutool `JSONObject#getLong`.
    #[must_use]
    pub fn get_long(&self, name: &str) -> Option<i64> {
        self.0.get(name).and_then(|value| {
            value
                .as_i64()
                .or_else(|| value.as_u64().and_then(|n| i64::try_from(n).ok()))
                .or_else(|| value.as_f64().map(|n| n as i64))
        })
    }

    fn encode(&self) -> String {
        URL_SAFE_NO_PAD
            .encode(serde_json::to_vec(&self.0).expect("serde_json::Value maps always serialize"))
    }
}

impl fmt::Display for Claims {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(&self.0)
            .expect("serde_json::Value maps always serialize to valid UTF-8");
        formatter.write_str(&json)
    }
}
