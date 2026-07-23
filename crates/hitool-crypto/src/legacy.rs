//! Explicitly rejected legacy algorithms (DES / RC4).

use crate::CryptoError;

/// DES encrypt — rejected by security policy (Hutool `DesTest` proxy).
pub fn des_encrypt(_key: &[u8], _plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Err(CryptoError::LegacyRejected(
        "DES is deprecated and rejected by hitool-crypto security policy",
    ))
}

/// DES decrypt — rejected by security policy.
pub fn des_decrypt(_key: &[u8], _ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Err(CryptoError::LegacyRejected(
        "DES is deprecated and rejected by hitool-crypto security policy",
    ))
}

/// RC4 encrypt — rejected by security policy (Hutool `RC4Test` proxy).
pub fn rc4_crypt(_key: &[u8], _message: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Err(CryptoError::LegacyRejected(
        "RC4 is deprecated and rejected by hitool-crypto security policy",
    ))
}
