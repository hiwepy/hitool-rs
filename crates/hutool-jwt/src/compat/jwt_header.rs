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

use super::claims::Claims;
use super::jwt::JWT;

/// JWT protected header.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JWTHeader(Claims);

impl JWTHeader {
    /// `alg` header name.
    pub const ALGORITHM: &'static str = "alg";
    /// `typ` header name.
    pub const TYPE: &'static str = "typ";
    /// `cty` header name.
    pub const CONTENT_TYPE: &'static str = "cty";
    /// `kid` header name.
    pub const KEY_ID: &'static str = "kid";

    /// Sets `alg`.
    pub fn set_algorithm(&mut self, value: impl Into<String>) -> &mut Self {
        self.0
            .set_claim(Self::ALGORITHM, Value::String(value.into()));
        self
    }

    /// Sets `typ`.
    pub fn set_type(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.set_claim(Self::TYPE, Value::String(value.into()));
        self
    }

    /// Sets `cty`.
    pub fn set_content_type(&mut self, value: impl Into<String>) -> &mut Self {
        self.0
            .set_claim(Self::CONTENT_TYPE, Value::String(value.into()));
        self
    }

    /// Sets `kid`.
    pub fn set_key_id(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.set_claim(Self::KEY_ID, Value::String(value.into()));
        self
    }

    /// Adds protected headers.
    pub fn add_headers(&mut self, values: Map<String, Value>) -> &mut Self {
        self.0.put_all(values);
        self
    }

    /// Returns the underlying claims.
    #[must_use]
    pub const fn claims(&self) -> &Claims {
        &self.0
    }
}
