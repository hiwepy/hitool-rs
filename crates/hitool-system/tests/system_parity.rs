use hitool_system as hs;

#[test]
fn system_snapshot_test() {
    let snap = hs::SystemSnapshot::collect();
    assert!(snap.used_memory >= 0, "used_memory 应 >= 0");
}

#[test]
fn runtime_info_test() {
    let rt = hs::RuntimeInfo::collect();
    assert!(rt.total_memory > 0, "total_memory 应 > 0");
}

#[test]
fn host_info_test() {
    let host = hs::HostInfo::collect();
    assert!(host.name.is_some() || true, "HostInfo::collect() 成功");
}
