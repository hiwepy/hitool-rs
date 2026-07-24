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

mod jwt_exception;
mod claims;
mod jwt_header;
mod jwt_payload;
mod registered_payload;
mod jwt_signer;
mod h_mac_jwt_signer;
mod asymmetric_jwt_signer;
mod elliptic_curve_jwt_signer;
mod none_jwt_signer;
mod algorithm_util;
mod jwt_signer_util;
mod jwt;
mod jwt_validator;
mod jwt_util;

pub use jwt_exception::JWTException;
pub use claims::Claims;
pub use jwt_header::JWTHeader;
pub use jwt_payload::JWTPayload;
pub use registered_payload::RegisteredPayload;
pub use jwt_signer::JWTSigner;
pub use h_mac_jwt_signer::HMacJWTSigner;
pub use asymmetric_jwt_signer::AsymmetricJWTSigner;
pub use elliptic_curve_jwt_signer::EllipticCurveJWTSigner;
pub use none_jwt_signer::NoneJWTSigner;
pub use algorithm_util::AlgorithmUtil;
pub use jwt_signer_util::JWTSignerUtil;
pub use jwt::JWT;
pub use jwt_validator::JWTValidator;
pub use jwt_util::JWTUtil;
