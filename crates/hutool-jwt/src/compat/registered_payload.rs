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
use super::jwt_payload::JWTPayload;

/// Fluent registered-claim setters.
pub trait RegisteredPayload {
    /// Sets a registered payload value.
    fn set_registered(&mut self, name: &'static str, value: Value) -> &mut Self;

    /// Sets issuer.
    fn set_issuer(&mut self, value: impl Into<String>) -> &mut Self {
        self.set_registered(JWTPayload::ISSUER, Value::String(value.into()))
    }
    /// Sets subject.
    fn set_subject(&mut self, value: impl Into<String>) -> &mut Self {
        self.set_registered(JWTPayload::SUBJECT, Value::String(value.into()))
    }
    /// Sets audience.
    fn set_audience(&mut self, value: impl Into<String>) -> &mut Self {
        self.set_registered(JWTPayload::AUDIENCE, Value::String(value.into()))
    }
    /// Sets expiration epoch seconds.
    fn set_expires_at(&mut self, value: u64) -> &mut Self {
        self.set_registered(JWTPayload::EXPIRES_AT, Value::from(value))
    }
    /// Sets not-before epoch seconds.
    fn set_not_before(&mut self, value: u64) -> &mut Self {
        self.set_registered(JWTPayload::NOT_BEFORE, Value::from(value))
    }
    /// Sets issued-at epoch seconds.
    fn set_issued_at(&mut self, value: u64) -> &mut Self {
        self.set_registered(JWTPayload::ISSUED_AT, Value::from(value))
    }
    /// Sets JWT ID.
    fn set_jwt_id(&mut self, value: impl Into<String>) -> &mut Self {
        self.set_registered(JWTPayload::JWT_ID, Value::String(value.into()))
    }
}

impl RegisteredPayload for JWTPayload {
    fn set_registered(&mut self, name: &'static str, value: Value) -> &mut Self {
        self.set_payload(name, value)
    }
}
