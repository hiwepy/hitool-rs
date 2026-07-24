//! CipherWrapper + ProviderFactory facade，对齐 hutool 的：
//! - `cn.hutool.crypto.CipherWrapper`
//! - `cn.hutool.crypto.ProviderFactory`
//!
//! **仅提供 trait 抽象 + 桩实现**。具体 JCE Cipher SPI / BouncyCastle Provider
//! 是 `bouncycastle_only` unsafe-to-copy，Rust 用 RustCrypto 替代。

use std::any::Any;

use crate::CryptoError;

/// Cipher 包装器，对齐 `cn.hutool.crypto.CipherWrapper`。
///
/// Java 包装 `javax.crypto.Cipher`；Rust 用 trait 提供形状。
/// 具体加密操作由 hutool-crypto 的其他模块（aes_modes/chacha_util 等）实现。
pub trait CipherWrapper: Send + Sync {
    /// 对齐 `CipherWrapper(String algorithm)`：构造器（按算法名）
    fn from_algorithm(algorithm: &str) -> Result<Self, CryptoError>
    where
        Self: Sized;

    /// 对齐 `CipherWrapper(Cipher cipher)`：构造器（包装已有 Cipher）
    fn from_cipher(cipher: Box<dyn Any>) -> Result<Self, CryptoError>
    where
        Self: Sized;

    /// 对齐 `CipherWrapper.getParams()`：获取算法参数
    fn get_params(&self) -> Option<Box<dyn Any>>;

    /// 对齐 `CipherWrapper.setParams(AlgorithmParameterSpec)`：设置算法参数
    fn set_params(&mut self, params: Box<dyn Any>) -> Result<(), CryptoError>;

    /// 对齐 `CipherWrapper.setRandom(SecureRandom)`：设置随机数生成器
    fn set_random(&mut self, random: Box<dyn Any + Send + Sync>) -> Result<(), CryptoError>;

    /// 对齐 `CipherWrapper.getCipher()`：获取底层 Cipher
    fn get_cipher(&self) -> Option<Box<dyn Any>>;

    /// 对齐 `CipherWrapper.initMode(int mode, Key key)`：初始化模式
    /// - mode: 1=ENCRYPT, 2=DECRYPT, 3=WRAP, 4=UNWRAP
    fn init_mode(&mut self, mode: i32, key: &[u8]) -> Result<(), CryptoError>;
}

/// CipherWrapper 桩实现：所有 JCE 操作返回 Err
pub struct StubCipherWrapper {
    algorithm: String,
    params: Option<Box<dyn Any + Send + Sync>>,
    random: Option<Box<dyn Any + Send + Sync>>,
    cipher_raw: Option<Box<dyn Any + Send + Sync>>,
}

impl CipherWrapper for StubCipherWrapper {
    fn from_algorithm(algorithm: &str) -> Result<Self, CryptoError> {
        Ok(Self {
            algorithm: algorithm.to_string(),
            params: None,
            random: None,
            cipher_raw: None,
        })
    }

    fn from_cipher(_cipher: Box<dyn Any>) -> Result<Self, CryptoError> {
        Err(CryptoError::LegacyRejected(
            "StubCipherWrapper::from_cipher requires javax.crypto.Cipher (Java-only); use RustCrypto in Rust",
        ))
    }

    fn get_params(&self) -> Option<Box<dyn Any>> {
        None
    }

    fn set_params(&mut self, _params: Box<dyn Any>) -> Result<(), CryptoError> {
        Err(CryptoError::LegacyRejected(
            "set_params requires javax.crypto.spec.AlgorithmParameterSpec (Java-only)",
        ))
    }

    fn set_random(&mut self, _random: Box<dyn Any + Send + Sync>) -> Result<(), CryptoError> {
        Err(CryptoError::LegacyRejected(
            "set_random requires java.security.SecureRandom (Java-only)",
        ))
    }

    fn get_cipher(&self) -> Option<Box<dyn Any>> {
        None
    }

    fn init_mode(&mut self, _mode: i32, _key: &[u8]) -> Result<(), CryptoError> {
        Err(CryptoError::LegacyRejected(
            "init_mode requires javax.crypto.Cipher (Java-only); use RustCrypto Cipher::new",
        ))
    }
}

impl StubCipherWrapper {
    /// 获取算法名
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }
}

