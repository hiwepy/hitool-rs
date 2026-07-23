#!/usr/bin/env python3
"""Record Hutool crypto APIs against hitool-crypto idiomatic surfaces / planned gaps."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
MODULE = "hutool-crypto"
EXPECTED = 745


def family(qualified_name: str) -> str:
    return qualified_name.split("::", 1)[1].split("#", 1)[0].split("::", 1)[0]


def method_name(qualified_name: str) -> str | None:
    parts = qualified_name.split("::")
    if len(parts) < 3:
        return None
    return parts[-1]


def mapping(class_name: str, method: str | None, signature: str) -> tuple[str, str, str, str] | None:
    digest = {
        "DigestUtil", "Digester", "DigesterFactory", "DigestAlgorithm", "MD5", "SM3",
    }
    aes = {
        "AES", "SymmetricCrypto", "SymmetricEncryptor", "SymmetricDecryptor",
        "SymmetricAlgorithm", "Mode", "Padding", "CipherMode",
    }
    keys = {"KeyUtil", "ECKeyUtil", "OpensslKeyUtil", "GlobalBouncyCastleProvider"}
    rsa = {
        "RSA", "AsymmetricCrypto", "AsymmetricAlgorithm", "AsymmetricEncryptor",
        "AsymmetricDecryptor", "AbstractAsymmetricCrypto", "BaseAsymmetric",
        "KeyType", "Sign", "SignAlgorithm", "SignUtil",
    }
    sm = {"SM2", "SmUtil", "SM4"}
    mac = {
        "Mac", "HMac", "HmacAlgorithm", "MacEngine", "MacEngineFactory",
        "DefaultHMacEngine", "BCHMacEngine", "BCMacEngine", "CBCBlockCipherMacEngine",
        "SM4MacEngine",
    }
    otp = {"HOTP", "TOTP"}
    password = {"Argon2", "BCrypt"}
    legacy = {"DES", "DESede", "RC4", "FPE", "Vigenere", "XXTEA"}

    if class_name in digest:
        return (
            "idiomatic",
            "hitool_crypto::{DigestUtil,Digester,Md5Util,Sm3Util,md5_hex,sha256_hex,sm3_hex}",
            "crates/hitool-crypto/tests/crypto_parity_extended.rs::digest_hash256_test",
            "Hutool digest facades map to typed hex helpers; SecureUtil-style digests share the same vectors.",
        )
    if class_name in aes:
        return (
            "idiomatic",
            "hitool_crypto::{Aes,aes256_gcm_encrypt,aes128_cbc_encrypt,aes128_ecb_encrypt}",
            "crates/hitool-crypto/tests/crypto_util_parity.rs::aes256_gcm_encrypt_decrypt",
            "Authenticated AES-GCM is the default; CBC/ECB helpers preserve Hutool mode overload shapes without Java Cipher SPI.",
        )
    if class_name == "SecureUtil":
        return (
            "idiomatic",
            "hitool_crypto::{sha256_hex,hmac_sha256,aes256_gcm_encrypt,hash_password,pbkdf2_hex}",
            "crates/hitool-crypto/tests/crypto_parity_extended.rs::secure_util_sha256_test",
            "SecureUtil static facade is split into typed Rust helpers (digest/MAC/AEAD/password/PBKDF2).",
        )
    if class_name in keys:
        return (
            "idiomatic",
            "hitool_crypto::{generate_random_key_bytes,generate_rsa_keypair,generate_ec_keypair,generate_sm2_keypair}",
            "crates/hitool-crypto/tests/crypto_parity_gap.rs::key_util_generate_key_pair_test",
            "Key generation uses RustCrypto/OS RNG instead of BouncyCastle KeyPairGenerator.",
        )
    if class_name in rsa:
        return (
            "idiomatic",
            "hitool_crypto::{Rsa,SignUtil,generate_rsa_keypair,rsa_public_from_private_key,KeyType}",
            "crates/hitool-crypto/tests/crypto_parity_extended.rs::rsa_test",
            "RSA/asymmetric signing helpers are owned Rust types; Java Key/Certificate overloads collapse to byte/PEM inputs.",
        )
    if class_name in sm:
        return (
            "idiomatic",
            "hitool_crypto::{Sm4,sm2_sign,sm2_verify,sm4_ecb_encrypt,generate_sm2_keypair}",
            "crates/hitool-crypto/tests/crypto_parity_extended.rs::sm2_test",
            "SM2/SM4 use Rust SM crates; Hutool BC provider wiring is omitted.",
        )
    if class_name in mac:
        return (
            "idiomatic",
            "hitool_crypto::{HMac,hmac_sha256,hmac_sha256_hex,hmac_md5_hex,hmac_sha1_hex,hmac_sm3_hex}",
            "crates/hitool-crypto/tests/crypto_util_parity.rs::hmac_sha256_basic",
            "MAC engines collapse to typed HMAC helpers with explicit key/message bytes.",
        )
    if class_name in otp:
        return (
            "idiomatic",
            "hitool_crypto::{Hotp,Totp,hotp,totp,totp_validate,decode_base32_secret}",
            "crates/hitool-crypto/tests/crypto_parity_extended.rs::otp_gen_key_test",
            "HOTP/TOTP match Hutool counter/time-step semantics via Rust OTP helpers.",
        )
    if class_name == "PemUtil":
        return (
            "idiomatic",
            "hitool_crypto::{read_pem_key,read_pem_private_key,write_pkcs8_private_pem,PemKind}",
            "crates/hitool-crypto/tests/crypto_parity_extended.rs::pem_read_private_key_test",
            "PEM read/write is pure Rust without BouncyCastle PEMParser.",
        )
    if class_name in password:
        return (
            "idiomatic",
            "hitool_crypto::{hash_password,verify_password}",
            "crates/hitool-crypto/tests/crypto_util_parity.rs::bcrypt_hash_and_verify",
            "Password hashing uses Argon2id (SecretString) rather than Java BCrypt/Argon2 SPI beans.",
        )
    if class_name == "ChaCha20":
        return (
            "idiomatic",
            "hitool_crypto::{chacha20_encrypt,chacha20_decrypt}",
            "crates/hitool-crypto/tests/crypto_parity_extended.rs::chacha20_encrypt_decrypt_test",
            "ChaCha20 is a typed helper over RustCrypto stream cipher primitives.",
        )
    if class_name in legacy:
        return (
            "idiomatic",
            "hitool_crypto::{des_ecb_encrypt,Rc4,FpeFf1,vigenere_encrypt,tea_encrypt}",
            "crates/hitool-crypto/tests/crypto_parity_extended.rs::symmetric_des_test",
            "Legacy DES/RC4/FPE/Vigenere/TEA keep Hutool-shaped helpers with explicit Rust ownership.",
        )
    if class_name == "PBKDF2":
        return (
            "idiomatic",
            "hitool_crypto::{pbkdf2_hex,pbkdf2_sha1_hex}",
            "crates/hitool-crypto/tests/crypto_parity_extended.rs::pbkdf2_encrypt_test",
            "PBKDF2-HMAC-SHA1 matches Hutool SecureUtil.pbkdf2 hex width.",
        )
    if class_name == "CryptoException":
        return (
            "idiomatic",
            "hitool_crypto::CryptoError",
            "crates/hitool-crypto/tests/crypto_util_parity.rs::aes256_gcm_encrypt_decrypt",
            "Checked CryptoException maps to non-exhaustive CryptoError.",
        )

    # ── Newly wired planned → idiomatic surfaces ──────────────────────────
    if class_name == "ASN1Util":
        return (
            "idiomatic",
            "hitool_crypto::Asn1Util",
            "crates/hitool-crypto/tests/crypto_parity_facade.rs::asn1_util_encode_decode",
            "ASN1Util DER SEQUENCE helpers use the `der` crate; BC ASN1Encodable types collapse to octet payloads.",
        )
    if class_name == "SpecUtil":
        return (
            "idiomatic",
            "hitool_crypto::{SpecUtil,KeySpecBytes,PbeKeySpec,RsaPrivateCrtKeySpec}",
            "crates/hitool-crypto/tests/crypto_parity_facade.rs::spec_util_key_and_xml",
            "Java KeySpec/PBE*Spec/RSAPrivateCrtKeySpec map to owned Rust byte/BigUint containers.",
        )
    if class_name == "ECIES":
        return (
            "idiomatic",
            "hitool_crypto::{Ecies,encrypt_ecies,decrypt_ecies}",
            "crates/hitool-crypto/tests/crypto_parity_facade.rs::ecies_encrypt_decrypt_roundtrip",
            "ECIES is P-256 ECDH + SHA-256 + AES-256-GCM; BC ECIES Cipher SPI algorithm strings are accepted then ignored.",
        )
    if class_name == "BCUtil":
        return bc_util_mapping(method, signature)

    # ── Remain planned: Java-only BC engines / JCA SPI ────────────────────
    if class_name == "ZUC":
        return None  # handled as planned with note below
    if class_name in {"CipherWrapper", "ProviderFactory"}:
        return None

    return None


def bc_util_mapping(method: str | None, signature: str) -> tuple[str, str, str, str] | None:
    """Map BCUtil methods that have RustCrypto stand-ins; leave pure BC types planned."""
    idiomatic = (
        "idiomatic",
        "hitool_crypto::{BcUtil,EcDomainParams,EcPrivateParams,EcPublicParams}",
        "crates/hitool-crypto/tests/crypto_parity_facade.rs::bc_util_ec_and_sm2_params",
        "BCUtil EC/SM2/PEM/PKCS#1 helpers collapse BC parameter objects to named-curve tags and byte params.",
    )
    if method is None:
        return idiomatic

    # Pure BouncyCastle type bridges — cannot safely mirror.
    if method == "toDomainParams" and (
        "ECParameterSpec" in signature or "X9ECParameters" in signature
    ):
        return None
    if method == "toParams" and (
        signature.startswith("AsymmetricKeyParameter (Key")
        or signature.startswith("ECPublicKeyParameters (PublicKey")
        or signature.startswith("ECPrivateKeyParameters (PrivateKey")
    ):
        return None

    covered = {
        "encodeECPrivateKey",
        "decodeECPrivateKey",
        "encodeECPublicKey",
        "decodeECPoint",
        "toDomainParams",  # String overload only (filtered above)
        "toSm2Params",
        "toParams",  # scalar/coord overloads (Key overloads filtered above)
        "readPemPrivateKey",
        "readPemPublicKey",
        "toPkcs1",
    }
    if method in covered:
        return idiomatic
    return None


def planned_note(class_name: str, method: str | None, signature: str) -> str:
    if class_name == "ZUC":
        return (
            "ZUC-128/256 is a BouncyCastle-only stream cipher in Hutool; no RustCrypto engine is "
            "linked yet — kept planned (do not proxy via AES)."
        )
    if class_name == "CipherWrapper":
        return (
            "CipherWrapper wraps javax.crypto.Cipher SPI (AlgorithmParameterSpec/SecureRandom); "
            "no JCA Cipher mirror in hitool-crypto — kept planned."
        )
    if class_name == "ProviderFactory":
        return (
            "ProviderFactory.createBouncyCastleProvider is Java Security Provider wiring; "
            "Rust has no BC provider — kept planned."
        )
    if class_name == "BCUtil":
        return (
            f"BCUtil.{method or 'type'}{(' ' + signature) if signature else ''} returns "
            "BouncyCastle ECDomainParameters/AsymmetricKeyParameter/Key types that cannot be "
            "safely mirrored without a BC dependency — kept planned."
        )
    return (
        f"{class_name} not yet ported as a first-class hitool-crypto surface; "
        "deferred pending additional RustCrypto coverage."
    )


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = idiomatic = planned = 0
    for row in inventory:
        if row["module"] != MODULE:
            continue
        selected += 1
        class_name = family(row["qualified_name"])
        method = method_name(row["qualified_name"])
        signature = row.get("signature") or ""
        mapped = mapping(class_name, method, signature)
        if mapped is None:
            planned += 1
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "planned",
                "hitool_symbol": "",
                "test_evidence": "",
                "notes": planned_note(class_name, method, signature),
            }
        else:
            idiomatic += 1
            status, symbol, evidence, notes = mapped
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": status,
                "hitool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }

    if selected != EXPECTED:
        raise SystemExit(f"expected {EXPECTED} {MODULE} APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} {MODULE} APIs (idiomatic={idiomatic}, planned={planned})")


if __name__ == "__main__":
    main()
