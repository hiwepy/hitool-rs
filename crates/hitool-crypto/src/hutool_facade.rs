//! Hutool-named type facades that delegate to idiomatic helpers.
//!
//! These zero-sized / thin types mirror Hutool class names so callers can find
//! `DigestUtil.md5_hex`, `HMac`, `AES`, `RSA`, `HOTP`, `TOTP`, etc. without
//! changing the underlying RustCrypto implementations.

use crate::{
    aes128_cbc_decrypt, aes128_cbc_encrypt, aes128_ecb_decrypt, aes128_ecb_encrypt,
    aes256_gcm_decrypt, aes256_gcm_encrypt, hotp, hmac_md5_hex, hmac_sha1_hex, hmac_sha256,
    hmac_sha256_hex, hmac_sm3_hex, md5_hex, md5_hex16, md5_hex_repeat, md5_hex_salt,
    md5_hex_salt_repeat, md5_hex_with_salt, sha1_hex, sha256_hex, sha512_hex, sm3_hex,
    sm4_ecb_decrypt, sm4_ecb_encrypt, totp, totp_validate, CryptoError, OtpAlgorithm,
};
use secrecy::SecretString;

/// Hutool `DigestUtil` static facade.
#[derive(Debug, Clone, Copy, Default)]
pub struct DigestUtil;

impl DigestUtil {
    /// Hutool `DigestUtil.md5Hex`.
    #[must_use]
    pub fn md5_hex(input: impl AsRef<[u8]>) -> String {
        md5_hex(input)
    }

    /// Hutool `DigestUtil.md5Hex` 16-char form.
    #[must_use]
    pub fn md5_hex16(input: impl AsRef<[u8]>) -> String {
        md5_hex16(input)
    }

    /// Hutool `DigestUtil.sha1Hex`.
    #[must_use]
    pub fn sha1_hex(input: impl AsRef<[u8]>) -> String {
        sha1_hex(input)
    }

    /// Hutool `DigestUtil.sha256Hex`.
    #[must_use]
    pub fn sha256_hex(input: impl AsRef<[u8]>) -> String {
        sha256_hex(input)
    }

    /// Hutool `DigestUtil.sha512Hex`.
    #[must_use]
    pub fn sha512_hex(input: impl AsRef<[u8]>) -> String {
        sha512_hex(input)
    }

    /// Hutool `DigestUtil.bcrypt` → Argon2id PHC string.
    pub fn bcrypt(password: &str) -> Result<String, CryptoError> {
        crate::hash_password(&SecretString::from(password.to_owned()))
    }

    /// Hutool `DigestUtil.bcryptCheck`.
    pub fn bcrypt_check(password: &str, hashed: &str) -> Result<bool, CryptoError> {
        crate::verify_password(&SecretString::from(password.to_owned()), hashed)
    }
}

/// Hutool `Digester` with salt + repeat count.
#[derive(Debug, Clone)]
pub struct Digester {
    salt: Vec<u8>,
    digest_count: usize,
}

impl Digester {
    /// Creates an MD5 digester (Hutool `new Digester(DigestAlgorithm.MD5)`).
    #[must_use]
    pub fn md5() -> Self {
        Self {
            salt: Vec::new(),
            digest_count: 1,
        }
    }

    /// Sets salt bytes (Hutool `setSalt`).
    #[must_use]
    pub fn set_salt(mut self, salt: impl Into<Vec<u8>>) -> Self {
        self.salt = salt.into();
        self
    }

    /// Sets digest repeat count (Hutool `setDigestCount`).
    #[must_use]
    pub fn set_digest_count(mut self, count: usize) -> Self {
        self.digest_count = count.max(1);
        self
    }

    /// Digests to lowercase hex (Hutool `digestHex`).
    #[must_use]
    pub fn digest_hex(&self, input: impl AsRef<[u8]>) -> String {
        if self.salt.is_empty() {
            md5_hex_repeat(input, self.digest_count)
        } else if self.digest_count <= 1 {
            md5_hex_with_salt(input, &self.salt)
        } else {
            md5_hex_salt_repeat(&self.salt, input, self.digest_count)
        }
    }

    /// Salt-prefixed MD5 without repeat (Hutool saltPosition ≤ 0 path).
    #[must_use]
    pub fn digest_hex_salt_prefix(&self, input: impl AsRef<[u8]>) -> String {
        md5_hex_salt(&self.salt, input)
    }
}

/// Hutool `MD5` convenience type.
#[derive(Debug, Clone, Copy, Default)]
pub struct Md5Util;

impl Md5Util {
    /// Hutool `MD5.create().digestHex`.
    #[must_use]
    pub fn digest_hex(input: impl AsRef<[u8]>) -> String {
        md5_hex(input)
    }
}

/// Hutool `SM3` / `SmUtil.sm3` convenience type.
#[derive(Debug, Clone, Copy, Default)]
pub struct Sm3Util;

impl Sm3Util {
    /// Hutool `SM3.create().digestHex` / `SmUtil.sm3`.
    #[must_use]
    pub fn digest_hex(input: impl AsRef<[u8]>) -> String {
        sm3_hex(input)
    }
}

/// Hutool `HMac` facade over typed HMAC helpers.
#[derive(Debug, Clone)]
pub struct HMac {
    key: Vec<u8>,
}

impl HMac {
    /// Creates HMAC with raw key bytes (Hutool `new HMac(algorithm, key)`).
    #[must_use]
    pub fn new(key: impl Into<Vec<u8>>) -> Self {
        Self { key: key.into() }
    }

    /// HMAC-SHA256 hex (default Hutool HmacSHA256 path).
    pub fn digest_hex(&self, message: &[u8]) -> Result<String, CryptoError> {
        hmac_sha256_hex(&self.key, message)
    }

    /// HMAC-SHA256 bytes.
    pub fn digest(&self, message: &[u8]) -> Result<[u8; 32], CryptoError> {
        hmac_sha256(&self.key, message)
    }

    /// HMAC-MD5 hex.
    pub fn digest_md5_hex(&self, message: &[u8]) -> Result<String, CryptoError> {
        hmac_md5_hex(&self.key, message)
    }

    /// HMAC-SHA1 hex.
    pub fn digest_sha1_hex(&self, message: &[u8]) -> Result<String, CryptoError> {
        hmac_sha1_hex(&self.key, message)
    }

    /// HMAC-SM3 hex (`SmUtil.hmacSm3`).
    pub fn digest_sm3_hex(&self, message: &[u8]) -> Result<String, CryptoError> {
        hmac_sm3_hex(&self.key, message)
    }
}

/// Hutool `AES` / `SymmetricCrypto` AES mode facade.
#[derive(Debug, Clone, Copy, Default)]
pub struct Aes;

impl Aes {
    /// AES-256-GCM encrypt (secure default).
    pub fn gcm_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        aes256_gcm_encrypt(key, plaintext)
    }

    /// AES-256-GCM decrypt.
    pub fn gcm_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        aes256_gcm_decrypt(key, ciphertext)
    }

    /// AES-128-CBC encrypt (Hutool mode overload shape).
    pub fn cbc_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        aes128_cbc_encrypt(key, iv, plaintext)
    }

    /// AES-128-CBC decrypt.
    pub fn cbc_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        aes128_cbc_decrypt(key, iv, ciphertext)
    }

    /// AES-128-ECB encrypt.
    pub fn ecb_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        aes128_ecb_encrypt(key, plaintext)
    }

    /// AES-128-ECB decrypt.
    pub fn ecb_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        aes128_ecb_decrypt(key, ciphertext)
    }
}

/// Hutool `SM4` facade.
#[derive(Debug, Clone, Copy, Default)]
pub struct Sm4;

impl Sm4 {
    /// SM4-ECB encrypt.
    pub fn ecb_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        sm4_ecb_encrypt(key, plaintext)
    }

    /// SM4-ECB decrypt.
    pub fn ecb_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        sm4_ecb_decrypt(key, ciphertext)
    }
}

/// Hutool `RSA` facade over [`crate::RsaKeyPair`] helpers.
#[derive(Debug, Clone, Copy, Default)]
pub struct Rsa;

impl Rsa {
    /// Generates a 2048-bit key pair.
    pub fn generate_keypair() -> Result<crate::RsaKeyPair, CryptoError> {
        crate::generate_rsa_keypair()
    }

    /// PKCS#1 v1.5 encrypt.
    pub fn encrypt(
        public_key: &rsa::RsaPublicKey,
        plaintext: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        crate::rsa_encrypt_pkcs1v15(public_key, plaintext)
    }

    /// PKCS#1 v1.5 decrypt.
    pub fn decrypt(
        private_key: &rsa::RsaPrivateKey,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        crate::rsa_decrypt_pkcs1v15(private_key, ciphertext)
    }
}

/// Hutool `Sign` / `SignUtil` SM2-oriented facade (RSA-PSS variants stay on typed helpers).
#[derive(Debug, Clone, Copy, Default)]
pub struct SignUtil;

impl SignUtil {
    /// SM2 sign → 64-byte signature.
    pub fn sm2_sign(secret: &sm2::SecretKey, message: &[u8]) -> Result<[u8; 64], CryptoError> {
        crate::sm2_sign(secret, message)
    }

    /// SM2 verify with uncompressed public hex.
    pub fn sm2_verify(
        public_hex: &str,
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, CryptoError> {
        crate::sm2_verify(public_hex, message, signature)
    }
}

/// Hutool `HOTP` facade.
#[derive(Debug, Clone)]
pub struct Hotp {
    key: Vec<u8>,
    digits: u32,
}

impl Hotp {
    /// Creates HOTP with key and digit count (Hutool `new HOTP(key, digits)`).
    #[must_use]
    pub fn new(key: impl Into<Vec<u8>>, digits: u32) -> Self {
        Self {
            key: key.into(),
            digits,
        }
    }

    /// Generates HOTP for counter (Hutool `generate`).
    pub fn generate(&self, counter: u64) -> Result<u32, CryptoError> {
        hotp(&self.key, counter, self.digits)
    }
}

/// Hutool `TOTP` facade.
#[derive(Debug, Clone)]
pub struct Totp {
    key: Vec<u8>,
    digits: u32,
    step_secs: u64,
    algorithm: OtpAlgorithm,
}

impl Totp {
    /// Creates TOTP with defaults (6 digits, 30s, HMAC-SHA1).
    #[must_use]
    pub fn new(key: impl Into<Vec<u8>>) -> Self {
        Self {
            key: key.into(),
            digits: 6,
            step_secs: 30,
            algorithm: OtpAlgorithm::HmacSha1,
        }
    }

    /// Sets digit count.
    #[must_use]
    pub fn digits(mut self, digits: u32) -> Self {
        self.digits = digits;
        self
    }

    /// Sets time step seconds.
    #[must_use]
    pub fn step_secs(mut self, step_secs: u64) -> Self {
        self.step_secs = step_secs;
        self
    }

    /// Sets HMAC algorithm.
    #[must_use]
    pub fn algorithm(mut self, algorithm: OtpAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Generates TOTP for epoch seconds (Hutool `generate`).
    pub fn generate(&self, epoch_secs: u64) -> Result<u32, CryptoError> {
        totp(
            &self.key,
            epoch_secs,
            self.step_secs,
            self.digits,
            self.algorithm,
        )
    }

    /// Validates within offset windows (Hutool `validate`).
    pub fn validate(
        &self,
        epoch_secs: u64,
        offset_size: u64,
        expected: u32,
    ) -> Result<bool, CryptoError> {
        totp_validate(
            &self.key,
            epoch_secs,
            self.step_secs,
            offset_size,
            expected,
            self.digits,
            self.algorithm,
        )
    }
}
