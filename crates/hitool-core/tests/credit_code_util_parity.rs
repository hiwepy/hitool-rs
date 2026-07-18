//! credit_code_util module parity tests
//! 对齐: hutool-core CreditCodeUtilTest

use hitool_core::CreditCodeUtil;

// ── is_credit_code_simple ──

#[test]
fn is_credit_code_simple_valid() {
    assert!(CreditCodeUtil::is_credit_code_simple("91350100M000100Y43"));
}

#[test]
fn is_credit_code_simple_invalid_length() {
    assert!(!CreditCodeUtil::is_credit_code_simple("short"));
}

#[test]
fn is_credit_code_simple_invalid_chars() {
    assert!(!CreditCodeUtil::is_credit_code_simple("91350100M000100Y4!"));
}

// ── is_credit_code ──

#[test]
fn is_credit_code_valid() {
    assert!(CreditCodeUtil::is_credit_code("91350100M000100Y43"));
}

#[test]
fn is_credit_code_invalid() {
    assert!(!CreditCodeUtil::is_credit_code("invalid"));
}

// ── random_credit_code ──

#[test]
fn random_credit_code_length() {
    let code = CreditCodeUtil::random_credit_code();
    assert_eq!(code.len(), 18);
}

#[test]
fn random_credit_code_is_valid() {
    let code = CreditCodeUtil::random_credit_code();
    assert!(CreditCodeUtil::is_credit_code_simple(&code));
}

#[test]
fn random_credit_code_uniqueness() {
    let code1 = CreditCodeUtil::random_credit_code();
    let code2 = CreditCodeUtil::random_credit_code();
    assert_ne!(code1, code2);
}
