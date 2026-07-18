use hitool_jwt as hj;

#[test]
fn hmac_signer_test() {
    let signer = hj::HMacJWTSigner::new(hj::Algorithm::HS256, b"mysecret123");
    assert!(signer.is_ok(), "HMacJWTSigner 创建应成功");
}

#[test]
fn claims_parse_test() {
    let claims = hj::Claims::parse(r#"{"sub":"alice","iss":"hitool"}"#).unwrap();
    assert_eq!(claims.get_claim("sub").map(|v| v.as_str().unwrap_or_default()), Some("alice"));
}

#[test]
fn claims_set_test() {
    let mut claims = hj::Claims::parse("{}").unwrap();
    claims.set_claim("sub", serde_json::json!("bob"));
    assert_eq!(claims.get_claim("sub").map(|v| v.as_str().unwrap_or_default()), Some("bob"));
}
