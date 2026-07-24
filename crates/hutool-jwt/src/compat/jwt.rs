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
use super::jwt_exception::JWTException;
use super::jwt_header::JWTHeader;
use super::jwt_payload::JWTPayload;
use super::jwt_signer::JWTSigner;
use super::jwt_signer_util::JWTSignerUtil;
use super::jwt_validator::JWTValidator;
use super::none_jwt_signer::NoneJWTSigner;
use super::registered_payload::RegisteredPayload;

/// Dynamic JWT builder/parser.
#[derive(Clone)]
pub struct JWT {
    header: JWTHeader,
    payload: JWTPayload,
    signer: Option<Arc<dyn JWTSigner>>,
    tokens: Option<[String; 3]>,
}

impl Default for JWT {
    fn default() -> Self {
        Self::create()
    }
}

impl JWT {
    /// Creates an empty token.
    #[must_use]
    pub fn create() -> Self {
        Self {
            header: JWTHeader::default(),
            payload: JWTPayload::default(),
            signer: None,
            tokens: None,
        }
    }

    /// Parses a compact JWT.
    pub fn of(token: &str) -> Result<Self, JWTException> {
        Self::create().parse(token)
    }

    /// Replaces this object with parsed token content.
    pub fn parse(mut self, token: &str) -> Result<Self, JWTException> {
        if token.trim().is_empty() {
            return Err(JWTException::new("Token String must be not blank!"));
        }
        let parts: Vec<_> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(JWTException::formatted(
                "the token was expected 3 parts, but got {}",
                &[&parts.len()],
            ));
        }
        let header = String::from_utf8(decode_jwt_part(parts[0])?)
            .map_err(|error| JWTException::new(error.to_string()))?;
        let payload = String::from_utf8(decode_jwt_part(parts[1])?)
            .map_err(|error| JWTException::new(error.to_string()))?;
        self.header = JWTHeader(Claims::parse(&header)?);
        self.payload = JWTPayload(Claims::parse(&payload)?);
        self.tokens = Some([
            parts[0].to_owned(),
            parts[1].to_owned(),
            parts[2].to_owned(),
        ]);
        Ok(self)
    }

    /// Configures an HMAC signer from the shared key.
    ///
    /// Aligns with Hutool `JWT#setKey(byte[])`: uses the existing header `alg`
    /// when present (so a pre-set `HS384` header yields an HS384 signer), otherwise
    /// defaults to HS256. When `alg` is `none`/empty, returns an error (Hutool throws).
    pub fn set_key(&mut self, key: &[u8]) -> Result<&mut Self, JWTException> {
        let algorithm_id = self.algorithm().unwrap_or("HS256");
        if NoneJWTSigner::is_none(Some(algorithm_id)) {
            return Err(JWTException::new(
                "When key is not null, algorithmId must not be none.",
            ));
        }
        let signer = JWTSignerUtil::create_signer(algorithm_id, key)?;
        self.set_signer(Arc::new(signer));
        Ok(self)
    }

    /// Sets the signer and its header algorithm when absent.
    pub fn set_signer(&mut self, signer: Arc<dyn JWTSigner>) -> &mut Self {
        if self.algorithm().is_none() {
            self.header.set_algorithm(signer.algorithm_id());
        }
        self.signer = Some(signer);
        self
    }

    /// Returns the signer.
    #[must_use]
    pub fn signer(&self) -> Option<&dyn JWTSigner> {
        self.signer.as_deref()
    }

    /// Returns protected headers.
    #[must_use]
    pub const fn header(&self) -> &JWTHeader {
        &self.header
    }

    /// Returns mutable protected headers.
    pub const fn header_mut(&mut self) -> &mut JWTHeader {
        &mut self.header
    }

    /// Returns payload claims.
    #[must_use]
    pub const fn payload(&self) -> &JWTPayload {
        &self.payload
    }

    /// Returns mutable payload claims.
    pub const fn payload_mut(&mut self) -> &mut JWTPayload {
        &mut self.payload
    }

    /// Returns the header algorithm.
    #[must_use]
    pub fn algorithm(&self) -> Option<&str> {
        self.header
            .claims()
            .get_claim(JWTHeader::ALGORITHM)
            .and_then(Value::as_str)
    }

    /// Returns a header claim by name (Hutool `JWT#getHeader(String)`).
    #[must_use]
    pub fn get_header(&self, name: &str) -> Option<&Value> {
        self.header.claims().get_claim(name)
    }

    /// Returns a payload claim by name (Hutool `JWT#getPayload(String)`).
    #[must_use]
    pub fn get_payload(&self, name: &str) -> Option<&Value> {
        self.payload.claims().get_claim(name)
    }

    /// Returns all payload claims (Hutool `JWT#getPayloads()`).
    #[must_use]
    pub const fn get_payloads(&self) -> &Claims {
        self.payload.claims()
    }

    /// Sets a header.
    pub fn set_header(&mut self, name: impl Into<String>, value: Value) -> &mut Self {
        self.header.0.set_claim(name, value);
        self
    }

    /// Sets a payload.
    pub fn set_payload(&mut self, name: impl Into<String>, value: Value) -> &mut Self {
        self.payload.set_payload(name, value);
        self
    }

    /// Adds protected headers.
    pub fn add_headers(&mut self, headers: Map<String, Value>) -> &mut Self {
        self.header.add_headers(headers);
        self
    }

    /// Adds payload claims.
    pub fn add_payloads(&mut self, payloads: Map<String, Value>) -> &mut Self {
        self.payload.add_payloads(payloads);
        self
    }

    /// Signs using the configured signer.
    pub fn sign(&mut self) -> Result<String, JWTException> {
        self.sign_with_type(true)
    }

    /// Signs using the configured signer and controls automatic `typ=JWT` insertion.
    pub fn sign_with_type(&mut self, add_type_if_missing: bool) -> Result<String, JWTException> {
        let signer = self
            .signer
            .as_ref()
            .ok_or_else(|| JWTException::new("no signer provided"))?;
        if add_type_if_missing && self.header.0.get_claim(JWTHeader::TYPE).is_none() {
            self.header.set_type("JWT");
        }
        let header = self.header.0.encode();
        let payload = self.payload.0.encode();
        let signature = signer.sign(&header, &payload)?;
        Ok(format!("{header}.{payload}.{signature}"))
    }

    /// Installs an explicit signer and signs in one operation.
    pub fn sign_with(&mut self, signer: Arc<dyn JWTSigner>) -> Result<String, JWTException> {
        self.set_signer(signer);
        self.sign()
    }

    /// Verifies using the configured signer.
    ///
    /// Aligns with Hutool `JWT#verify()`: a missing signer defaults to `none`.
    pub fn verify(&self) -> Result<bool, JWTException> {
        match self.signer.as_ref() {
            Some(signer) => self.verify_with(signer.as_ref()),
            None => self.verify_with(&NoneJWTSigner),
        }
    }

    /// Verifies with an explicit signer.
    ///
    /// Aligns with Hutool `JWT#verify(JWTSigner)` including `alg=none` guards.
    pub fn verify_with(&self, signer: &dyn JWTSigner) -> Result<bool, JWTException> {
        let parts = self
            .tokens
            .as_ref()
            .ok_or_else(|| JWTException::new("no token to verify"))?;
        let none_alg = NoneJWTSigner::is_none(self.algorithm());
        let none_signer = NoneJWTSigner::is_none(Some(signer.algorithm_id()));
        if none_alg && !none_signer {
            return Err(JWTException::formatted(
                "Alg is 'none' but use: {} !",
                &[&signer.algorithm_id()],
            ));
        }
        if none_signer && !none_alg {
            return Err(JWTException::new(
                "Alg is not 'none' but use NoneJWTSigner!",
            ));
        }
        if !none_alg && self.algorithm() != Some(signer.algorithm_id()) {
            return Err(JWTException::new(
                "header and signer algorithms do not match",
            ));
        }
        signer.verify(&parts[0], &parts[1], &parts[2])
    }

    /// Creates a validator.
    #[must_use]
    pub fn validate(&self) -> JWTValidator {
        JWTValidator::new(self)
    }

    /// Verifies signature and registered dates with the supplied leeway.
    ///
    /// Aligns with Hutool `JWT#validate(long)`.
    pub fn validate_leeway(&self, leeway: u64) -> Result<bool, JWTException> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.is_valid_at(now, leeway)
    }

    /// Verifies signature and registered dates with the supplied leeway.
    pub fn is_valid_at(&self, now: u64, leeway: u64) -> Result<bool, JWTException> {
        if !self.verify()? {
            return Ok(false);
        }
        Ok(self.validate().validate_date_at(now, leeway).is_ok())
    }
}

impl RegisteredPayload for JWT {
    fn set_registered(&mut self, name: &'static str, value: Value) -> &mut Self {
        self.set_payload(name, value)
    }
}

fn decode_jwt_part(part: &str) -> Result<Vec<u8>, JWTException> {
    let pad = |input: &str| -> String {
        let rem = input.len() % 4;
        if rem == 0 {
            input.to_owned()
        } else {
            format!("{input}{}", "=".repeat(4 - rem))
        }
    };
    let padded = pad(part);
    if let Ok(bytes) = URL_SAFE.decode(padded.as_bytes()) {
        return Ok(bytes);
    }
    if let Ok(bytes) = URL_SAFE_NO_PAD.decode(part.as_bytes()) {
        return Ok(bytes);
    }
    let standard = padded.replace('-', "+").replace('_', "/");
    STANDARD
        .decode(standard.as_bytes())
        .map_err(JWTException::from)
}
