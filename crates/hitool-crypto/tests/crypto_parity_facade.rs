//! Facade parity tests for newly wired Hutool crypto surfaces.
//! BCUtil / ASN1Util / SpecUtil / ECIES / DigestUtil / HMac / AES / HOTP / TOTP

use hitool_crypto as hc;

/// Hutool `ASN1Util.encodeDer` / `decode` / `getDumpStr`.
#[test]
fn asn1_util_encode_decode() {
    let der = hc::Asn1Util::encode_der(&[b"hi", b"tool"]).expect("encode");
    assert_eq!(der[0], 0x30);
    let round = hc::Asn1Util::decode(&der).expect("decode");
    assert_eq!(round, der);
    let dump = hc::Asn1Util::get_dump_str(&der);
    assert_eq!(dump.len(), der.len() * 2);
    assert!(dump.chars().all(|c| c.is_ascii_hexdigit()));
    let mut out = Vec::new();
    hc::Asn1Util::encode_to("DER", &mut out, &[b"a"]).expect("encode_to");
    assert!(!out.is_empty());
}

/// Hutool `SpecUtil.createKeySpec` / PBE / XML CRT.
#[test]
fn spec_util_key_and_xml() {
    let spec = hc::SpecUtil::create_key_spec("AES", Some(b"0123456789abcdef"));
    assert_eq!(spec.algorithm, "AES");
    assert_eq!(spec.key.len(), 16);

    let des = hc::SpecUtil::create_key_spec("DES", None);
    assert_eq!(des.key.len(), 8);

    let pbe = hc::SpecUtil::create_pbe_key_spec(Some(b"secret"));
    assert_eq!(pbe.password, b"secret");

    let params = hc::SpecUtil::create_pbe_parameter_spec(b"salt", 1000);
    assert_eq!(params.iteration_count, 1000);

    // Minimal C#-style RSA XML with Base64 "AQAB" (65537) reused for all fields.
    let xml = r#"
    <RSAKeyValue>
      <Modulus>AQAB</Modulus>
      <Exponent>AQAB</Exponent>
      <P>AQAB</P>
      <Q>AQAB</Q>
      <DP>AQAB</DP>
      <DQ>AQAB</DQ>
      <InverseQ>AQAB</InverseQ>
      <D>AQAB</D>
    </RSAKeyValue>
    "#;
    let crt = hc::SpecUtil::xml_to_rsa_private_crt_key_spec(xml).expect("xml");
    assert!(crt.public_exponent > 0u32.into());
}

/// Hutool `BCUtil` EC encode/decode + SM2 params + PKCS#1.
#[test]
fn bc_util_ec_and_sm2_params() {
    let (secret, public) = hc::generate_ec_keypair().expect("ec");
    let d = hc::BcUtil::encode_ec_private_key(&secret);
    let restored = hc::BcUtil::decode_ec_private_key(&d, "secp256r1").expect("decode d");
    assert_eq!(restored.to_bytes(), secret.to_bytes());

    let q = hc::BcUtil::encode_ec_public_key_ex(&public, false);
    assert_eq!(q[0], 0x04);
    let pub2 = hc::BcUtil::decode_ec_point(&q, "P-256").expect("decode q");
    assert_eq!(pub2, public);

    let domain = hc::BcUtil::to_domain_params("sm2p256v1").expect("domain");
    assert_eq!(domain, hc::EcDomainParams::Sm2);

    let sm2_pub = hc::BcUtil::to_sm2_params_xy(
        "706AD9DAA3E5CEAC3DA59F583429E8043BAFC576BE10092C4EA4D8E19846CA62",
        "F7E938B02EED7280277493B8556E5B01CB436E018A562DFDC53342BF41FDF728",
    )
    .expect("sm2 pub");
    assert!(sm2_pub.x_valid && sm2_pub.y_valid);

    let sm2_priv = hc::BcUtil::to_sm2_params_d(
        "5F6CA5BB044C40ED2355F0372BF72A5B3AE6943712F9FDB7C1FFBAECC06F3829",
    )
    .expect("sm2 priv");
    assert!(sm2_priv.valid);

    let pair = hc::generate_rsa_keypair().expect("rsa");
    let pkcs1 = hc::BcUtil::to_pkcs1_private(&pair.private_key).expect("pkcs1");
    assert!(!pkcs1.is_empty());
    let pkcs1_pub = hc::BcUtil::to_pkcs1_public(&pair.public_key).expect("pkcs1 pub");
    assert!(!pkcs1_pub.is_empty());
}

/// Hutool `ECIES` encrypt/decrypt round-trip.
#[test]
fn ecies_encrypt_decrypt_roundtrip() {
    let alice = hc::Ecies::new().expect("alice");
    let bob = hc::Ecies::from_keys(None, alice.public_key().cloned());
    let ct = bob.encrypt(b"hello hutool").expect("enc");
    let pt = alice.decrypt(&ct).expect("dec");
    assert_eq!(pt, b"hello hutool");
}

/// Hutool-named digest / MAC / AES / OTP facades.
#[test]
fn hutool_named_facades_delegate() {
    assert_eq!(
        hc::DigestUtil::sha256_hex("abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    let digester = hc::Digester::md5().set_salt(b"salt").set_digest_count(2);
    assert_eq!(digester.digest_hex("data").len(), 32);
    assert_eq!(hc::Md5Util::digest_hex("x").len(), 32);
    assert_eq!(hc::Sm3Util::digest_hex("x").len(), 64);

    let mac = hc::HMac::new(b"key");
    assert_eq!(mac.digest_hex(b"msg").expect("hmac").len(), 64);

    let key = [7u8; 32];
    let ct = hc::Aes::gcm_encrypt(&key, b"secret").expect("aes");
    assert_eq!(hc::Aes::gcm_decrypt(&key, &ct).expect("dec"), b"secret");

    let hotp = hc::Hotp::new(b"12345678901234567890", 6);
    assert_eq!(hotp.generate(0).expect("hotp").to_string().len() <= 6, true);

    let totp = hc::Totp::new(b"12345678901234567890");
    let code = totp.generate(59).expect("totp");
    assert!(totp.validate(59, 0, code).expect("validate"));
}
