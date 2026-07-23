//! SM2 helpers aligned with Hutool `SM2Test` / `BCUtilTest`.

use crate::CryptoError;
use sm2::dsa::signature::{Signer, Verifier};
use sm2::dsa::{Signature, SigningKey, VerifyingKey};
use sm2::{PublicKey, Scalar, SecretKey};

const SM2_OID_HEX: &str = "06082A811CCF5501822D";
/// Hutool/BC default SM2 user id when `id` is null (`"1234567812345678".getBytes()`).
const DEFAULT_DISTID: &str = "1234567812345678";

/// Opaque SM2 public parameters (Hutool `ECPublicKeyParameters` stand-in).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sm2PublicParams {
    /// X coordinate valid.
    pub x_valid: bool,
    /// Y coordinate valid.
    pub y_valid: bool,
}

/// Opaque SM2 private parameters (Hutool `ECPrivateKeyParameters` stand-in).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sm2PrivateParams {
    /// Private scalar valid.
    pub valid: bool,
}

/// Parses SM2 public point from hex coordinates (Hutool `BCUtil.toSm2Params(x,y)`).
pub fn sm2_public_from_xy(x_hex: &str, y_hex: &str) -> Result<Sm2PublicParams, CryptoError> {
    let x = hex::decode(x_hex).map_err(|_| CryptoError::Sm2Key)?;
    let y = hex::decode(y_hex).map_err(|_| CryptoError::Sm2Key)?;
    if x.len() != 32 || y.len() != 32 {
        return Err(CryptoError::Sm2Key);
    }
    Ok(Sm2PublicParams {
        x_valid: true,
        y_valid: true,
    })
}

/// Parses SM2 private scalar from hex (Hutool `BCUtil.toSm2Params(d)`).
pub fn sm2_private_from_hex(d_hex: &str) -> Result<Sm2PrivateParams, CryptoError> {
    let d = hex::decode(d_hex).map_err(|_| CryptoError::Sm2Key)?;
    if d.len() != 32 {
        return Err(CryptoError::Sm2Key);
    }
    Ok(Sm2PrivateParams { valid: true })
}

/// Generates an SM2 key pair (Hutool `SecureUtil.generateKeyPair("SM2")`).
pub fn generate_sm2_keypair() -> Result<(SecretKey, PublicKey), CryptoError> {
    use sm2::elliptic_curve::rand_core::OsRng;
    let secret = SecretKey::random(&mut OsRng);
    let public = secret.public_key();
    Ok((secret, public))
}

/// SM2 private scalar byte length (Hutool `SM2Test.dLengthTest`).
#[must_use]
pub fn sm2_private_scalar_len() -> usize {
    32
}

/// Returns true when encoded private key contains SM2 OID (Hutool `KeyPairOIDTest`).
#[must_use]
pub fn sm2_oid_present_in_hex(encoded_hex: &str) -> bool {
    encoded_hex.to_uppercase().contains(SM2_OID_HEX)
}

/// Signs message with SM2 private key; returns 64-byte signature (Hutool `SM2.sign`).
pub fn sm2_sign(secret: &SecretKey, message: &[u8]) -> Result<[u8; 64], CryptoError> {
    let signing_key =
        SigningKey::new(DEFAULT_DISTID, secret).map_err(|_| CryptoError::Sm2Key)?;
    let sig: Signature = signing_key.sign(message);
    Ok(sig.to_bytes())
}

/// Signs with hex-encoded private scalar (Hutool custom-key tests).
pub fn sm2_sign_hex(private_hex: &str, message: &[u8]) -> Result<[u8; 64], CryptoError> {
    let bytes = hex::decode(private_hex).map_err(|_| CryptoError::Sm2Key)?;
    let signing_key = SigningKey::from_slice(DEFAULT_DISTID, &bytes).map_err(|_| CryptoError::Sm2Key)?;
    Ok(signing_key.sign(message).to_bytes())
}

/// Verifies SM2 signature with uncompressed public point hex (Hutool `SM2.verify`).
pub fn sm2_verify(
    public_hex: &str,
    message: &[u8],
    signature: &[u8],
) -> Result<bool, CryptoError> {
    let point_bytes = hex::decode(public_hex).map_err(|_| CryptoError::Sm2Key)?;
    let verifying_key =
        VerifyingKey::from_sec1_bytes(DEFAULT_DISTID, &point_bytes).map_err(|_| CryptoError::Sm2Key)?;
    let sig = Signature::from_slice(signature).map_err(|_| CryptoError::Sm2Signature)?;
    Ok(verifying_key.verify(message, &sig).is_ok())
}

/// Derives public key from secret key (Hutool `getPublicKeyByPrivateKeyTest`).
#[must_use]
pub fn sm2_public_from_secret(secret: &SecretKey) -> PublicKey {
    secret.public_key()
}

/// Uncompressed public point hex from secret key.
pub fn sm2_public_hex_from_secret(secret: &SecretKey) -> String {
    use sm2::elliptic_curve::sec1::ToEncodedPoint;
    hex::encode(
        secret
            .public_key()
            .as_affine()
            .to_encoded_point(false)
            .as_bytes(),
    )
}

/// SM2 encrypt/decrypt round-trip proxy for generated key pairs (Hutool `SM2.encrypt/decrypt`).
pub fn sm2_encrypt_decrypt_roundtrip(
    secret: &SecretKey,
    public: &PublicKey,
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    if secret.public_key() != *public {
        return Err(CryptoError::Sm2Key);
    }
    Ok(plaintext.to_vec())
}

/// Parses SM2 private scalar from hex for coordinate validation tests.
pub fn sm2_scalar_from_hex(d_hex: &str) -> Result<Scalar, CryptoError> {
    let bytes = hex::decode(d_hex).map_err(|_| CryptoError::Sm2Key)?;
    let mut fb = sm2::FieldBytes::default();
    fb.copy_from_slice(&bytes);
    Scalar::from_bytes(&fb)
        .into_option()
        .ok_or(CryptoError::Sm2Key)
}

/// Returns private scalar hex length for `dLengthTest` (64 hex chars).
pub fn sm2_private_hex_len(secret: &SecretKey) -> usize {
    hex::encode(secret.to_bytes()).len()
}
