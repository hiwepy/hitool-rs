#![no_main]

use hitool_jwt::{JwtHs256, JwtValidationPolicy};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        let _ = url::Url::parse(text);
        let _ = hitool_cron::CronSchedule::parse(text);
        let policy = JwtValidationPolicy::new("fuzz", "fuzz", 0, true);
        let codec = JwtHs256::new(b"fixed fuzzing secret", &policy);
        let _ = codec.decode::<serde_json::Value>(text);
    }
});
