//! Uses the default `core` and `json` facade features together.

use hutool::core::{base64_encode, upper_first};
use hutool::json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value = json!({
        "project": upper_first("hutool"),
        "payload": base64_encode("production-minded utilities"),
    });
    println!("{}", hutool::json::to_string_pretty(&value)?);
    Ok(())
}
