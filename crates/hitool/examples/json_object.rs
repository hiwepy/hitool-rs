//! Hutool 能力对照：`hutool-json` → `hitool-json`（feature `json`）。
//!
//! 在 `core_json` 之外演示 parse / pretty / minify 与动态对象字段读取。

use hitool::json::{minify, parse_object, pretty, to_string_pretty};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw = r#"{"name":"hitool","version":1,"tags":["core","json"]}"#;
    let object = parse_object(raw)?;
    println!("keys = {:?}", object.keys().collect::<Vec<_>>());
    println!(
        "name = {}",
        object
            .get("name")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
    );

    println!("pretty =\n{}", pretty(raw)?);
    println!("minified = {}", minify(raw)?);
    println!("object pretty =\n{}", to_string_pretty(&object)?);
    Ok(())
}
