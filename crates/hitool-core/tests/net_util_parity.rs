//! NetUtil / Ipv4Util / URL encode helpers parity
//!
//! 对齐: `cn.hutool.core.net.NetUtil` / `Ipv4Util` / URLEncoder 族

use hitool_core::{
    Ipv4Util, LocalPortGenerater, NetUtil, UrlBuilder, UrlDecoder, UrlEncodeUtil, UrlEncoder,
};

/// 对齐 Java: `NetUtil` 端口 / IPv4 / 内网 / 代理 IP
#[test]
fn net_util_ip_port_helpers() {
    assert!(NetUtil::is_valid_port(8080));
    assert!(!NetUtil::is_valid_port(70_000));
    assert_eq!(NetUtil::long_to_ipv4(0x7F00_0001), "127.0.0.1");
    assert_eq!(NetUtil::ipv4_to_long("127.0.0.1"), Some(0x7F00_0001));
    assert!(NetUtil::is_inner_ip("10.0.0.1"));
    assert!(!NetUtil::is_inner_ip("8.8.8.8"));
    assert_eq!(NetUtil::hide_ip_part("1.2.3.4"), "1.2.*.*");
    assert!(NetUtil::is_in_range("192.168.1.10", "192.168.1.0/24"));
    assert!(NetUtil::is_unknown(" unknown "));
    assert_eq!(
        NetUtil::get_multistage_reverse_proxy_ip("unknown, 1.2.3.4"),
        "1.2.3.4"
    );
    assert_eq!(
        NetUtil::to_absolute_url("https://a.com/x/", "/y"),
        "https://a.com/x/y"
    );
    let port = NetUtil::get_usable_local_port().expect("usable port");
    assert!(NetUtil::is_usable_local_port(port as i32));
    assert!(!NetUtil::local_ipv4s().is_empty());
    assert!(NetUtil::is_valid_ipv6("::1"));
}

/// 对齐 Java: `Ipv4Util` 掩码 / 区间
#[test]
fn ipv4_util_mask_and_range() {
    assert_eq!(Ipv4Util::long_to_ipv4(0xC0A8_0001), "192.168.0.1");
    assert!(Ipv4Util::is_mask_bit_valid(24));
    assert_eq!(
        Ipv4Util::get_mask_by_mask_bit(24).as_deref(),
        Some("255.255.255.0")
    );
    assert_eq!(Ipv4Util::get_mask_bit_by_mask("255.255.255.0"), Some(24));
    assert_eq!(
        Ipv4Util::get_begin_ip_str("192.168.1.100", 24).as_deref(),
        Some("192.168.1.0")
    );
    assert_eq!(
        Ipv4Util::get_end_ip_str("192.168.1.100", 24).as_deref(),
        Some("192.168.1.255")
    );
    assert_eq!(Ipv4Util::count_by_mask_bit(30, true), Some(4));
    assert!(Ipv4Util::matches("192.168.*.*", "192.168.1.2"));
    assert!(Ipv4Util::is_inner_ip("172.16.0.1"));
    let list = Ipv4Util::list_by_mask("10.0.0.1", 30, true).unwrap();
    assert_eq!(list.len(), 4);
}

/// 对齐 Java: `LocalPortGenerater`
#[test]
fn local_port_generater() {
    let generator = LocalPortGenerater::new(10_000);
    let port = generator.generate().expect("port");
    assert!(port >= 10_000);
}

/// 对齐 Java: URLEncoder / URLDecoder / URLEncodeUtil
#[test]
fn url_encode_decode_helpers() {
    let encoded = UrlEncodeUtil::encode("a b");
    assert!(encoded.contains('%') || encoded.contains('+') || encoded == "a%20b");
    assert_eq!(UrlDecoder::decode("%20"), " ");
    assert!(!UrlEncoder::encode_query("x=y").is_empty());
}

/// 对齐 Java: `UrlBuilder`
#[test]
fn url_builder_path() {
    let mut builder = UrlBuilder::of();
    builder.add_path("api").add_path("v1");
    assert!(builder.build_path().contains("api"));
}
