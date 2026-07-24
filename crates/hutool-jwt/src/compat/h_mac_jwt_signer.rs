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

use super::algorithm_util::AlgorithmUtil;
use super::jwt::JWT;
use super::jwt_exception::JWTException;
use super::jwt_signer::JWTSigner;

/// HMAC JWT signer supporting HS256/384/512.
#[derive(Clone)]
pub struct HMacJWTSigner {
    algorithm: Algorithm,
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl HMacJWTSigner {
    /// Creates a validated HMAC signer.
    pub fn new(algorithm: Algorithm, key: &[u8]) -> Result<Self, JWTException> {
        if !matches!(
            algorithm,
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512
        ) {
            return Err(JWTException::new("algorithm is not an HMAC JWT algorithm"));
        }
        Ok(Self {
            algorithm,
            encoding: EncodingKey::from_secret(key),
            decoding: DecodingKey::from_secret(key),
        })
    }
}

impl JWTSigner for HMacJWTSigner {
    fn sign(&self, header: &str, payload: &str) -> Result<String, JWTException> {
        signing_result(crypto::sign(
            &signing_input(header, payload),
            &self.encoding,
            self.algorithm,
        ))
    }

    fn verify(&self, header: &str, payload: &str, signature: &str) -> Result<bool, JWTException> {
        verification_result(crypto::verify(
            signature,
            &signing_input(header, payload),
            &self.decoding,
            self.algorithm,
        ))
    }

    fn algorithm_id(&self) -> &'static str {
        AlgorithmUtil::get_id(self.algorithm)
    }
}

fn signing_result(

fn signing_input(header: &str, payload: &str) -> Vec<u8> {
    format!("{header}.{payload}").into_bytes()
}

fn verification_result(
