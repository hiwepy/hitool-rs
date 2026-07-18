use hitool_socket as hs;

#[test]
fn socket_config_test() {
    let config = hs::SocketConfig::new();
    assert!(config.read_timeout().as_secs() >= 0, "read_timeout 应存在");
}
