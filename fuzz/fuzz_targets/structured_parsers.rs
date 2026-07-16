#![no_main]

use hitool_jwt::{JwtHs256, JwtValidationPolicy};
use hitool_poi::XlsxReadLimits;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let limits = XlsxReadLimits {
        max_archive_bytes: 64 * 1024,
        max_entry_bytes: 64 * 1024,
        max_cells: 1_000,
        max_rows: 1_000,
        max_columns: 256,
    };
    let _ = hitool_poi::read_first_sheet(data, limits);

    if let Ok(text) = std::str::from_utf8(data) {
        let _ = url::Url::parse(text);
        let _ = hitool_cron::CronSchedule::parse(text);
        let policy = JwtValidationPolicy::new("fuzz", "fuzz", 0, true);
        let codec = JwtHs256::new(b"fixed fuzzing secret", &policy);
        let _ = codec.decode::<serde_json::Value>(text);
    }
});
