//! hutool-extra parity tests
use hitool_extra as he;

#[test]
fn create_zip_test() {
    let entries = vec![("test.txt", b"hello world" as &[u8])];
    let zip_bytes = he::archive::create_zip(&entries).unwrap();
    assert!(zip_bytes.len() > 0, "ZIP 应非空");
}

#[test]
fn qrcode_svg_test() {
    let svg = he::qrcode::to_svg("hello", 100).unwrap();
    assert!(svg.contains("<svg"), "QR SVG 应含 <svg");
}
