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
use super::jwt_signer::JWTSigner;

/// Convenience functions corresponding to Hutool's `JWTUtil`.
pub struct JWTUtil;

impl JWTUtil {
    /// Creates an HS256 token.
    pub fn create_token(payload: Map<String, Value>, key: &[u8]) -> Result<String, JWTException> {
        Self::create_token_with_headers(Map::new(), payload, key)
    }

    /// Creates an HS256 token with protected headers.
    pub fn create_token_with_headers(
        headers: Map<String, Value>,
        payload: Map<String, Value>,
        key: &[u8],
    ) -> Result<String, JWTException> {
        let mut jwt = JWT::create();
        jwt.add_headers(headers).add_payloads(payload);
        jwt.set_key(key)?;
        jwt.sign()
    }

    /// Creates a token with an explicit signer.
    pub fn create_token_with_signer(
        payload: Map<String, Value>,
        signer: Arc<dyn JWTSigner>,
    ) -> Result<String, JWTException> {
        Self::create_token_with_headers_and_signer(Map::new(), payload, signer)
    }

    /// Creates a token with protected headers and an explicit signer.
    pub fn create_token_with_headers_and_signer(
        headers: Map<String, Value>,
        payload: Map<String, Value>,
        signer: Arc<dyn JWTSigner>,
    ) -> Result<String, JWTException> {
        let mut jwt = JWT::create();
        jwt.add_headers(headers).add_payloads(payload);
        jwt.sign_with(signer)
    }

    /// Parses a token.
    pub fn parse_token(token: &str) -> Result<JWT, JWTException> {
        JWT::of(token)
    }

    /// Verifies an HS256 token.
    pub fn verify(token: &str, key: &[u8]) -> Result<bool, JWTException> {
        let mut jwt = JWT::of(token)?;
        jwt.set_key(key)?;
        jwt.verify()
    }

    /// Verifies a token with an explicit signer.
    pub fn verify_with_signer(token: &str, signer: &dyn JWTSigner) -> Result<bool, JWTException> {
        JWT::of(token)?.verify_with(signer)
    }
}
