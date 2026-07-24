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

use super::jwt_exception::JWTException;

/// Pluggable JWS signer/verifier.
pub trait JWTSigner: Send + Sync {
    /// Signs encoded header and payload components.
    fn sign(&self, header: &str, payload: &str) -> Result<String, JWTException>;
    /// Verifies a JWS signature.
    fn verify(&self, header: &str, payload: &str, signature: &str) -> Result<bool, JWTException>;
    /// Returns the standard algorithm ID.
    fn algorithm_id(&self) -> &'static str;
}
