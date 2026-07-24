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

/// JWT payload claims.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JWTPayload(Claims);

impl JWTPayload {
    /// Issuer claim.
    pub const ISSUER: &'static str = "iss";
    /// Subject claim.
    pub const SUBJECT: &'static str = "sub";
    /// Audience claim.
    pub const AUDIENCE: &'static str = "aud";
    /// Expiration claim.
    pub const EXPIRES_AT: &'static str = "exp";
    /// Not-before claim.
    pub const NOT_BEFORE: &'static str = "nbf";
    /// Issued-at claim.
    pub const ISSUED_AT: &'static str = "iat";
    /// JWT ID claim.
    pub const JWT_ID: &'static str = "jti";

    /// Sets a payload claim.
    pub fn set_payload(&mut self, name: impl Into<String>, value: Value) -> &mut Self {
        self.0.set_claim(name, value);
        self
    }

    /// Adds payload claims.
    pub fn add_payloads(&mut self, values: Map<String, Value>) -> &mut Self {
        self.0.put_all(values);
        self
    }

    /// Returns the underlying claims.
    #[must_use]
    pub const fn claims(&self) -> &Claims {
        &self.0
    }
}
