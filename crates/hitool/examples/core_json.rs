//! Uses the default `core` and `json` facade features together.

use hitool::core::{base64_encode, upper_first};
use hitool::json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value = json!({
        "project": upper_first("hitool"),
        "payload": base64_encode("production-minded utilities"),
    });
    println!("{}", hitool::json::to_string_pretty(&value)?);
    Ok(())
}
