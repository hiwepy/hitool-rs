//! PBKDF2 helpers aligned with Hutool `PBKDF2Test`.

use crate::CryptoError;
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;

const PBKDF2_OUTPUT_BYTES: usize = 64;

/// Derives a 512-bit key and returns 128-char lowercase hex (Hutool `SecureUtil.pbkdf2`).
pub fn pbkdf2_hex(password: &[u8], salt: &[u8]) -> Result<String, CryptoError> {
    let mut out = [0u8; PBKDF2_OUTPUT_BYTES];
    pbkdf2_hmac::<Sha1>(password, salt, 1000, &mut out);
    Ok(hex::encode(out))
}
