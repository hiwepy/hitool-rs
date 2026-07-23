//! Hutool 能力对照：`hutool-http` → `hitool-http`（feature `http`）。
//!
//! 演示生产向 `HttpClient` 构建：超时、响应上限、重定向限制与 `DenyLocalTargets`
//!（对齐 SSRF 防护思路）。本示例不发起外网请求，仅验证客户端可构建。

use std::time::Duration;

use hitool::http::{DenyLocalTargets, HttpClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::builder()
        .connect_timeout(Duration::from_secs(3))
        .timeout(Duration::from_secs(10))
        .max_response_size(8 * 1024 * 1024)
        .redirect_limit(5)
        .user_agent("hitool-example/0.1")
        .url_policy(DenyLocalTargets)
        .build()?;

    println!("HttpClient ready: {client:?}");
    println!("URL policy rejects private/link-local targets before send.");
    Ok(())
}
