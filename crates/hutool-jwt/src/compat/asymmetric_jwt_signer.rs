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

/// RSA signer supporting RS256/384/512 with separate private/public PEM keys.
#[derive(Clone)]
pub struct AsymmetricJWTSigner {
    algorithm: Algorithm,
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl AsymmetricJWTSigner {
    /// Creates an RSA signer from PEM keys.
    pub fn from_rsa_pem(
        algorithm: Algorithm,
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<Self, JWTException> {
        if !matches!(
            algorithm,
            Algorithm::RS256
                | Algorithm::RS384
                | Algorithm::RS512
                | Algorithm::PS256
                | Algorithm::PS384
                | Algorithm::PS512
        ) {
            return Err(JWTException::new(
                "algorithm is not an RSA or RSA-PSS JWT algorithm",
            ));
        }
        let private_key = rsa_private_der(private_key)?;
        let public_key = rsa_public_der(public_key)?;
        Ok(Self {
            algorithm,
            encoding: EncodingKey::from_rsa_der(&private_key),
            decoding: DecodingKey::from_rsa_der(&public_key),
        })
    }
}

impl JWTSigner for AsymmetricJWTSigner {
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

fn rsa_public_der(pem: &[u8]) -> Result<Vec<u8>, JWTException> {
    let pem = pem_text(pem)?;
    let key = if pem.contains("BEGIN RSA PUBLIC KEY") {
        rsa::RsaPublicKey::from_pkcs1_pem(pem).map_err(key_error)?
    } else {
        rsa::RsaPublicKey::from_public_key_pem(pem).map_err(key_error)?
    };
    Ok(key
        .to_pkcs1_der()
        .expect("validated RSA public keys always encode as PKCS#1")
        .as_bytes()
        .to_vec())
}

fn rsa_private_der(pem: &[u8]) -> Result<Vec<u8>, JWTException> {
    let pem = pem_text(pem)?;
    let key = if pem.contains("BEGIN RSA PRIVATE KEY") {
        rsa::RsaPrivateKey::from_pkcs1_pem(pem).map_err(key_error)?
    } else {
        rsa::RsaPrivateKey::from_pkcs8_pem(pem).map_err(key_error)?
    };
    Ok(key
        .to_pkcs1_der()
        .expect("validated RSA private keys always encode as PKCS#1")
        .as_bytes()
        .to_vec())
}
