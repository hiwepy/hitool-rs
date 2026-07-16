//! Cryptographic utilities with authenticated encryption by default.

#![forbid(unsafe_code)]

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng as AeadOsRng},
};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{Error as PasswordHashError, SaltString, rand_core::OsRng as PasswordOsRng},
};
use hmac::{Hmac, Mac};
use secrecy::{ExposeSecret, SecretString};
use sha2::{Digest, Sha256};
use thiserror::Error;

const AES_256_KEY_LENGTH: usize = 32;
const GCM_NONCE_LENGTH: usize = 12;

/// Errors returned by cryptographic operations.
#[derive(Debug, Error)]
pub enum CryptoError {
    /// AES-256 requires a 32-byte key.
    #[error("AES-256 key must contain exactly 32 bytes")]
    InvalidAesKey,
    /// Ciphertext does not contain the nonce and authentication tag.
    #[error("ciphertext is too short")]
    InvalidCiphertext,
    /// Authentication or encryption failed.
    #[error("authenticated encryption operation failed")]
    Aead,
    /// The supplied MAC key is invalid.
    #[error("invalid HMAC key")]
    InvalidMacKey,
    /// Password hashing or encoded-hash parsing failed.
    #[error(transparent)]
    PasswordHash(#[from] PasswordHashError),
}

/// Returns a lowercase SHA-256 digest.
#[must_use]
pub fn sha256_hex(input: impl AsRef<[u8]>) -> String {
    hex::encode(Sha256::digest(input.as_ref()))
}

/// Computes HMAC-SHA256.
pub fn hmac_sha256(key: &[u8], message: &[u8]) -> Result<[u8; 32], CryptoError> {
    let mut mac =
        <Hmac<Sha256> as Mac>::new_from_slice(key).map_err(|_| CryptoError::InvalidMacKey)?;
    mac.update(message);
    Ok(mac.finalize().into_bytes().into())
}

/// Verifies HMAC-SHA256 in constant time.
pub fn verify_hmac_sha256(
    key: &[u8],
    message: &[u8],
    expected: &[u8],
) -> Result<bool, CryptoError> {
    let mut mac =
        <Hmac<Sha256> as Mac>::new_from_slice(key).map_err(|_| CryptoError::InvalidMacKey)?;
    mac.update(message);
    Ok(mac.verify_slice(expected).is_ok())
}

/// Hashes a password with Argon2id and a fresh operating-system random salt.
pub fn hash_password(password: &SecretString) -> Result<String, CryptoError> {
    let salt = SaltString::generate(&mut PasswordOsRng);
    Ok(Argon2::default()
        .hash_password(password.expose_secret().as_bytes(), &salt)?
        .to_string())
}

/// Verifies a password against a PHC-formatted Argon2 hash.
pub fn verify_password(password: &SecretString, encoded_hash: &str) -> Result<bool, CryptoError> {
    let parsed = PasswordHash::new(encoded_hash)?;
    match Argon2::default().verify_password(password.expose_secret().as_bytes(), &parsed) {
        Ok(()) => Ok(true),
        Err(PasswordHashError::Password) => Ok(false),
        Err(error) => Err(CryptoError::PasswordHash(error)),
    }
}

/// Encrypts using AES-256-GCM and prefixes the random 96-bit nonce.
pub fn aes256_gcm_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != AES_256_KEY_LENGTH {
        return Err(CryptoError::InvalidAesKey);
    }
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::InvalidAesKey)?;
    let nonce = Aes256Gcm::generate_nonce(&mut AeadOsRng);
    let encrypted = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| CryptoError::Aead)?;
    let mut output = Vec::with_capacity(GCM_NONCE_LENGTH + encrypted.len());
    output.extend_from_slice(&nonce);
    output.extend_from_slice(&encrypted);
    Ok(output)
}

/// Decrypts bytes produced by [`aes256_gcm_encrypt`].
pub fn aes256_gcm_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != AES_256_KEY_LENGTH {
        return Err(CryptoError::InvalidAesKey);
    }
    if ciphertext.len() <= GCM_NONCE_LENGTH {
        return Err(CryptoError::InvalidCiphertext);
    }
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::InvalidAesKey)?;
    let (nonce, encrypted) = ciphertext.split_at(GCM_NONCE_LENGTH);
    cipher
        .decrypt(Nonce::from_slice(nonce), encrypted)
        .map_err(|_| CryptoError::Aead)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_and_macs_known_values() {
        assert_eq!(
            sha256_hex("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        let mac = hmac_sha256(b"key", b"message").unwrap();
        assert!(verify_hmac_sha256(b"key", b"message", &mac).unwrap());
        assert!(!verify_hmac_sha256(b"key", b"changed", &mac).unwrap());
    }

    #[test]
    fn authenticated_encryption_round_trips_and_rejects_tampering() {
        let key = [7_u8; 32];
        let mut ciphertext = aes256_gcm_encrypt(&key, b"secret").unwrap();
        assert_eq!(aes256_gcm_decrypt(&key, &ciphertext).unwrap(), b"secret");
        *ciphertext.last_mut().unwrap() ^= 1;
        assert!(matches!(
            aes256_gcm_decrypt(&key, &ciphertext),
            Err(CryptoError::Aead)
        ));
    }

    #[test]
    fn argon2_password_hashes_use_random_salts_and_verify() {
        let password = SecretString::from("correct horse battery staple".to_owned());
        let first = hash_password(&password).unwrap();
        let second = hash_password(&password).unwrap();
        assert_ne!(first, second);
        assert!(verify_password(&password, &first).unwrap());
        let wrong = SecretString::from("wrong".to_owned());
        assert!(!verify_password(&wrong, &first).unwrap());
    }
}
