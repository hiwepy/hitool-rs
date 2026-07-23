# Workspace examples

Runnable Cargo examples are colocated with the crate that owns their required
features. The facade examples live under `crates/hutool/examples`.

| Example | Features | Command |
|---|---|---|
| `core_json` | `core`, `json` (default) | `cargo run -p hutool --example core_json` |
| `json_object` | `json` | `cargo run -p hutool --example json_object --features json` |
| `http_client` | `http` | `cargo run -p hutool --example http_client --features http` |
| `crypto_hash` | `crypto` | `cargo run -p hutool --example crypto_hash --features crypto` |
| `cache_demo` | `cache` | `cargo run -p hutool --example cache_demo --features cache` |

`http_client` builds a production-shaped client with timeouts and
`DenyLocalTargets`; it does not open network connections.
