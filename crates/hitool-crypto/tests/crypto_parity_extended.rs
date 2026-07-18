//! `cn.hutool.crypto` 扩展对比验证测试 —— 对齐 Hutool 146 个 @Test
//! 来源: hutool-crypto/src/test/java/cn/hutool/crypto/
//!
//! hitool-crypto 当前 7 个函数: sha256_hex, hmac_sha256, verify_hmac_sha256,
//! hash_password, verify_password, aes256_gcm_encrypt, aes256_gcm_decrypt
//!
//! 本文件按 Java 测试文件分组,已实现函数用真实断言,
//! 未实现函数标记 #[ignore] 待后续实现后启用。

use hitool_crypto as hc;

// ===== SecureUtilTest 已实现方法 =====

/// 对齐 Java: SecureUtilTest.sha256Test()
#[test]
fn secure_util_sha256_test() {
    let hash = hc::sha256_hex("test中文".as_bytes());
    assert_eq!(hash.len(), 64, "SHA-256 输出 64 位十六进制");
    // 验证幂等性
    let hash2 = hc::sha256_hex("test中文".as_bytes());
    assert_eq!(hash, hash2, "SHA-256 幂等性");
}

/// 对齐 Java: SecureUtilTest.hmacSha256Test()
#[test]
fn secure_util_hmac_sha256_test() {
    let key = b"password";
    let msg = "test中文".as_bytes();
    let mac = hc::hmac_sha256(key, msg).unwrap();
    assert_eq!(mac.len(), 32, "HMAC-SHA256 输出 32 字节");
    assert!(hc::verify_hmac_sha256(key, msg, &mac).unwrap(), "验证成功");
}

// ===== DigestTest 已实现方法 =====

/// 对齐 Java: DigestTest.hash256Test()
#[test]
fn digest_hash256_test() {
    let hash = hc::sha256_hex("test中文".as_bytes());
    assert_eq!(hash.len(), 64);
}

// ===== Argon2Test 已实现方法 =====

/// 对齐 Java: Argon2Test.argon2Test()
#[test]
fn argon2_test() {
    let pw = secrecy::SecretString::from("123456");
    let encoded = hc::hash_password(&pw).unwrap();
    assert!(encoded.starts_with("$argon2"), "Argon2 输出应以 $argon2 开头");
    assert!(hc::verify_password(&pw, &encoded).unwrap());
}

/// 对齐 Java: Argon2Test.argon2WithSaltTest()
#[test]
fn argon2_with_salt_test() {
    let pw = secrecy::SecretString::from("123456");
    let encoded = hc::hash_password(&pw).unwrap();
    // Argon2 内部自动加盐,验证每次生成不同
    let encoded2 = hc::hash_password(&pw).unwrap();
    assert_ne!(encoded, encoded2, "两次 hash 应不同(不同盐)");
}

// ===== AESTest 已实现方法 =====

/// 对齐 Java: AESTest.gcmTest()
#[test]
fn aes_gcm_round_trip() {
    let key = [0u8; 32];
    let plaintext = "test中文".as_bytes();
    let ct = hc::aes256_gcm_encrypt(&key, plaintext).unwrap();
    assert!(ct.len() > plaintext.len());
    let pt = hc::aes256_gcm_decrypt(&key, &ct).unwrap();
    assert_eq!(pt, plaintext);
}

/// 对齐 Java: AESTest.gcmTest() — 错误密钥
#[test]
fn aes_gcm_wrong_key() {
    let k1 = [0u8; 32]; let k2 = [1u8; 32];
    let ct = hc::aes256_gcm_encrypt(&k1, "secret".as_bytes()).unwrap();
    assert!(hc::aes256_gcm_decrypt(&k2, &ct).is_err());
}

/// 对齐 Java: AESTest.gcmTest() — 空明文
#[test]
fn aes_gcm_empty_plaintext() {
    let key = [0u8; 32];
    let ct = hc::aes256_gcm_encrypt(&key, b"").unwrap();
    let pt = hc::aes256_gcm_decrypt(&key, &ct).unwrap();
    assert!(pt.is_empty());
}

// ===== BCryptTest 已实现方法 =====

/// 对齐 Java: BCryptTest.checkpwTest()
#[test]
fn bcrypt_checkpw_test() {
    // Hutool BCryptTest: 错误密码应验证失败
    // Rust argon2 verify_password 使用标准 Argon2id,格式不同于 Java BCrypt
    // 直接测试 Rust argon2 的密码验证逻辑
    let pw = secrecy::SecretString::from("correct_password");
    let encoded = hc::hash_password(&pw).unwrap();
    let wrong_pw = secrecy::SecretString::from("wrong_password");
    assert!(!hc::verify_password(&wrong_pw, &encoded).unwrap(),
        "错误密码应验证失败 (对齐 Java BCryptTest)");
}

// ===== 未实现方法 (全部 #[ignore]) =====
// 待 hitool-crypto 扩展后取消 ignore

/// 对齐 Java: DigestTest.md5Test()
#[ignore = "MD5 已废弃，等待 hitool-crypto 扩展(或安全策略拒绝)"]
#[test]
fn digest_md5_test() {}

/// 对齐 Java: DigestTest.sha1Test()
#[ignore = "SHA1 已废弃，等待 hitool-crypto 扩展"]
#[test]
fn digest_sha1_test() {}

/// 对齐 Java: HmacTest.hmacTest() — HmacMD5
#[ignore = "等待 hitool-crypto 扩展 hmac_md5"]
#[test]
fn hmac_md5_test() {}

/// 对齐 Java: HmacTest.hmacSha1Test()
#[ignore = "等待 hitool-crypto 扩展 hmac_sha1"]
#[test]
fn hmac_sha1_test() {}

/// 对齐 Java: AESTest.encryptCBCTest()
#[ignore = "等待 hitool-crypto 扩展 AES-CBC/PKCS7"]
#[test]
fn aes_cbc_test() {}

/// 对齐 Java: AESTest.encryptCTSTest()
#[ignore = "等待 hitool-crypto 扩展 AES-CTS"]
#[test]
fn aes_cts_test() {}

/// 对齐 Java: RSATest.rsaTest()
#[ignore = "等待 hitool-crypto 扩展 RSA"]
#[test]
fn rsa_test() {}

/// 对齐 Java: SymmetricTest.desTest()
#[ignore = "DES 已废弃，等待 hitool-crypto 扩展(或安全策略拒绝)"]
#[test]
fn symmetric_des_test() {}

/// 对齐 Java: SM2Test.sm2Test()
#[ignore = "等待 hitool-crypto 扩展 SM2/SM3/SM4 国密"]
#[test]
fn sm2_test() {}

/// 对齐 Java: PemUtilTest.readPrivateKeyTest()
#[ignore = "等待 hitool-crypto 扩展 PEM 密钥读取"]
#[test]
fn pem_read_private_key_test() {}

/// 对齐 Java: KeyUtilTest.generateECIESKeyTest()
#[ignore = "等待 hitool-crypto 扩展密钥生成"]
#[test]
fn key_util_generate_ecies_key_test() {}

/// 对齐 Java: SignTest.signAndVerifyTest()
#[ignore = "等待 hitool-crypto 扩展签名/验签"]
#[test]
fn sign_and_verify_test() {}

/// 对齐 Java: OTPTest.genKeyTest()
#[ignore = "等待 hitool-crypto 扩展 OTP/TOTP"]
#[test]
fn otp_gen_key_test() {}

/// 对齐 Java: RC4Test.testCryptMessage()
#[ignore = "RC4 已废弃，等待 hitool-crypto 扩展(或安全策略拒绝)"]
#[test]
fn rc4_crypt_test() {}

/// 对齐 Java: TEATest.teaTest()
#[ignore = "等待 hitool-crypto 扩展 TEA/XTEA/XXTEA"]
#[test]
fn tea_test() {}

/// 对齐 Java: SmTest.sm3Test()
#[ignore = "等待 hitool-crypto 扩展国密 SM3/SM4"]
#[test]
fn sm3_test() {}

/// 对齐 Java: ChaCha20Test.encryptAndDecryptTest()
#[ignore = "等待 hitool-crypto 扩展 ChaCha20"]
#[test]
fn chacha20_encrypt_decrypt_test() {}

/// 对齐 Java: PBKDF2Test.encryptTest()
#[ignore = "等待 hitool-crypto 扩展 PBKDF2"]
#[test]
fn pbkdf2_encrypt_test() {}

/// 对齐 Java: FPETest.ff1Test()
#[ignore = "等待 hitool-crypto 扩展格式保留加密 FPE"]
#[test]
fn fpe_ff1_test() {}
