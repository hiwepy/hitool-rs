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
use super::jwt_payload::JWTPayload;
use super::jwt_signer::JWTSigner;

/// JWT algorithm and registered-date validator.
pub struct JWTValidator {
    jwt: JWT,
}

impl JWTValidator {
    /// Creates a validator.
    #[must_use]
    pub fn new(jwt: &JWT) -> Self {
        Self { jwt: jwt.clone() }
    }

    /// Creates a validator from a compact token.
    pub fn of_token(token: &str) -> Result<Self, JWTException> {
        Ok(Self {
            jwt: JWT::of(token)?,
        })
    }

    /// Creates a validator from an owned JWT value.
    #[must_use]
    pub const fn of_jwt(jwt: JWT) -> Self {
        Self { jwt }
    }

    /// Validates the configured algorithm and signature.
    pub fn validate_algorithm(&self) -> Result<&Self, JWTException> {
        let signer = self
            .jwt
            .signer()
            .ok_or_else(|| JWTException::new("no signer provided"))?;
        self.validate_algorithm_with(signer)
    }

    /// Validates the header algorithm and signature using an explicit signer.
    pub fn validate_algorithm_with(&self, signer: &dyn JWTSigner) -> Result<&Self, JWTException> {
        if self.jwt.verify_with(signer)? {
            Ok(self)
        } else {
            Err(JWTException::new("signature verification failed"))
        }
    }

    /// Validates nbf, exp, and iat at explicit epoch seconds with leeway.
    pub fn validate_date_at(&self, now: u64, leeway: u64) -> Result<&Self, JWTException> {
        let claims = self.jwt.payload.claims();
        let number = |name| {
            claims
                .get_claim(name)
                .map(Value::as_u64)
                .transpose_value(name)
        };
        if let Some(nbf) = number(JWTPayload::NOT_BEFORE)? {
            if nbf > now.saturating_add(leeway) {
                return Err(JWTException::new("token is not active yet"));
            }
        }
        if let Some(exp) = number(JWTPayload::EXPIRES_AT)? {
            if exp.saturating_add(leeway) < now {
                return Err(JWTException::new("token has expired"));
            }
        }
        if let Some(iat) = number(JWTPayload::ISSUED_AT)? {
            if iat > now.saturating_add(leeway) {
                return Err(JWTException::new("token was issued in the future"));
            }
        }
        Ok(self)
    }

    /// Validates dates against current UTC epoch seconds.
    pub fn validate_date(&self) -> Result<&Self, JWTException> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.validate_date_at(now, 0)
    }
}

impl TransposeValue for Option<Option<u64>> {
    fn transpose_value(self, name: &str) -> Result<Option<u64>, JWTException> {
        match self {
            None => Ok(None),
            Some(Some(value)) => Ok(Some(value)),
            Some(None) => Err(JWTException::formatted(
                "registered claim {} must be an unsigned integer",
                &[&name],
            )),
        }
    }
}
