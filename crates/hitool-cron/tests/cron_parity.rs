use hitool_cron as hc;

#[test]
fn cron_pattern_parse_test() {
    let result = hc::CronPattern::parse("0 */5 * * * *");
    assert!(result.is_ok(), "Cron 表达式解析应成功");
}

#[test]
fn cron_schedule_parse_test() {
    let result = hc::CronSchedule::parse("* * * * * *");
    assert!(result.is_ok(), "CronSchedule::parse 应成功");
}

#[test]
fn invoke_registry_test() {
    let _registry = hc::InvokeRegistry::new();
    assert!(true, "InvokeRegistry 创建成功");
}
