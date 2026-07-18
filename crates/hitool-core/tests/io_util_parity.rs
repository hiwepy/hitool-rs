//! io_util parity tests
//! 对齐: hutool-core IoUtilTest

use hitool_core::IoUtil;
use std::io::Cursor;

// ── 流操作 ──

#[test]
fn copy_basic() {
    let data = b"Hello, World!";
    let mut reader = Cursor::new(data);
    let mut writer = Cursor::new(Vec::new());
    let copied = IoUtil::copy(&mut reader, &mut writer).unwrap();
    assert_eq!(copied, 13);
    assert_eq!(writer.into_inner(), data);
}

#[test]
fn copy_empty() {
    let mut reader = Cursor::new(b"");
    let mut writer = Cursor::new(Vec::new());
    let copied = IoUtil::copy(&mut reader, &mut writer).unwrap();
    assert_eq!(copied, 0);
}

#[test]
fn copy_with_buffer() {
    let data = b"Hello, World!";
    let mut reader = Cursor::new(data);
    let mut writer = Cursor::new(Vec::new());
    let copied = IoUtil::copy_with_buffer(&mut reader, &mut writer, 5).unwrap();
    assert_eq!(copied, 13);
    assert_eq!(writer.into_inner(), data);
}

// ── 读取操作 ──

#[test]
fn read_all_basic() {
    let data = b"Hello, World!";
    let mut reader = Cursor::new(data);
    let result = IoUtil::read_all(&mut reader).unwrap();
    assert_eq!(result, data);
}

#[test]
fn read_to_string_basic() {
    let data = "Hello, World!";
    let mut reader = Cursor::new(data);
    let result = IoUtil::read_to_string(&mut reader).unwrap();
    assert_eq!(result, data);
}

#[test]
fn read_lines_basic() {
    let data = "line1\nline2\nline3";
    let reader = Cursor::new(data);
    let lines = IoUtil::read_lines(reader).unwrap();
    assert_eq!(lines, vec!["line1", "line2", "line3"]);
}

#[test]
fn read_lines_empty() {
    let reader = Cursor::new(b"");
    let lines = IoUtil::read_lines(reader).unwrap();
    assert!(lines.is_empty());
}

// ── 写入操作 ──

#[test]
fn write_all_basic() {
    let mut writer = Cursor::new(Vec::new());
    IoUtil::write_all(&mut writer, b"Hello").unwrap();
    assert_eq!(writer.into_inner(), b"Hello");
}

#[test]
fn write_string_basic() {
    let mut writer = Cursor::new(Vec::new());
    IoUtil::write_string(&mut writer, "Hello").unwrap();
    assert_eq!(writer.into_inner(), b"Hello");
}

// ── 工具方法 ──

#[test]
fn bytes_to_hex_basic() {
    assert_eq!(IoUtil::bytes_to_hex(&[0xCA, 0xFE, 0xBA, 0xBE]), "cafebabe");
    assert_eq!(IoUtil::bytes_to_hex(&[]), "");
}

#[test]
fn hex_to_bytes_basic() {
    let result = IoUtil::hex_to_bytes("cafebabe").unwrap();
    assert_eq!(result, vec![0xCA, 0xFE, 0xBA, 0xBE]);
}

#[test]
fn hex_to_bytes_empty() {
    let result = IoUtil::hex_to_bytes("").unwrap();
    assert!(result.is_empty());
}

#[test]
fn hex_to_bytes_invalid() {
    assert!(IoUtil::hex_to_bytes("xyz").is_err());
    assert!(IoUtil::hex_to_bytes("123").is_err());
}

#[test]
fn hex_roundtrip() {
    let original = vec![0, 1, 2, 127, 128, 255];
    let hex = IoUtil::bytes_to_hex(&original);
    let decoded = IoUtil::hex_to_bytes(&hex).unwrap();
    assert_eq!(decoded, original);
}

// ── 流转换 ──

#[test]
fn read_u8_basic() {
    let data = b"AB";
    let mut reader = Cursor::new(data);
    assert_eq!(IoUtil::read_u8(&mut reader).unwrap(), Some(b'A'));
    assert_eq!(IoUtil::read_u8(&mut reader).unwrap(), Some(b'B'));
    assert_eq!(IoUtil::read_u8(&mut reader).unwrap(), None);
}

#[test]
fn read_exact_basic() {
    let data = b"Hello";
    let mut reader = Cursor::new(data);
    let result = IoUtil::read_exact(&mut reader, 3).unwrap();
    assert_eq!(result, b"Hel");
    let result = IoUtil::read_exact(&mut reader, 2).unwrap();
    assert_eq!(result, b"lo");
}
