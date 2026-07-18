//! id module parity tests
//! 对齐: hutool-core IdUtilTest

use hitool_core::IdUtil;

// ── IdUtil ──

#[test]
fn id_util_simple_uuid() {
    let id = IdUtil::simple_uuid();
    assert!(!id.is_empty());
    assert_eq!(id.len(), 32);
}

#[test]
fn id_util_uuid() {
    let id = IdUtil::uuid();
    assert!(!id.is_empty());
    // UUID format: 8-4-4-4-12
    assert_eq!(id.len(), 36);
    assert_eq!(id.chars().filter(|c| *c == '-').count(), 4);
}

#[test]
fn id_util_is_valid() {
    assert!(IdUtil::is_valid("550e8400-e29b-41d4-a716-446655440000"));
    assert!(!IdUtil::is_valid("not-a-uuid"));
    assert!(!IdUtil::is_valid(""));
}

#[test]
fn id_util_uuid_uniqueness() {
    let id1 = IdUtil::uuid();
    let id2 = IdUtil::uuid();
    assert_ne!(id1, id2);
}

#[test]
fn id_util_simple_uuid_uniqueness() {
    let id1 = IdUtil::simple_uuid();
    let id2 = IdUtil::simple_uuid();
    assert_ne!(id1, id2);
}
