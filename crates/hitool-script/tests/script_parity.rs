use hitool_script as hs;

#[test]
fn script_engine_test() {
    let limits = hs::ScriptLimits::default();
    let engine = hs::ScriptEngine::new(limits);
    assert!(true, "ScriptEngine 创建成功");
}

#[test]
fn script_limits_test() {
    let limits = hs::ScriptLimits::default();
    assert!(limits.max_operations > 0, "max_operations 应 > 0");
    assert!(limits.max_call_levels > 0, "max_call_levels 应 > 0");
}
