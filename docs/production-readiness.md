# Production readiness audit

This ledger checks the implementation against the original HiTool architecture
requirements. It describes the current `0.1` line and does not claim
method-for-method Hutool compatibility.

## Architecture and API

| Requirement | Status | Evidence or boundary |
|---|---|---|
| Hutool-aligned public crates | complete | 20 capability crates plus facade, compatibility, macros, and test support |
| Minimal facade defaults | complete | `hitool` defaults to `core` and `json`; `full` is explicit |
| Additive feature model | complete | every facade and component feature is checked independently in CI |
| No reverse/cyclic component dependency | complete | facade-only aggregation; components never depend on `hitool` |
| Idiomatic and compatibility APIs separated | complete | `hitool-compat-hutool` delegates to core/JSON implementations |
| No hidden global client, pool, config, or runtime | complete | stateful resources are constructed and injected explicitly |
| Complete Hutool functional parity | in progress | pinned v5.8.46 inventory contains 13,871 public production API records; 3,109 records have implementation and executable evidence: all 3 `hutool-all`, 175 `core.codec`, 422 `core.collection`, all 96 `core.builder`, 9 `core.getter`, 10 `core.clone`, 33 `core.compiler`, 36 `core.stream`, 45 `core.compress`, 37 `hutool-aop`, 43 `hutool-dfa`, 135 `hutool-http` base, metadata, User-Agent, semantically connected `HttpConfig`, and bounded `HttpResponse` records, 72 `hutool-bloomFilter`, 79 `hutool-script`, 87 `hutool-captcha`, 102 `hutool-socket`, 121 `hutool-jwt`, 124 `hutool-cache`, all 189 `hutool-system`, all 208 `hutool-cron`, all 225 `hutool-setting`, all 281 `hutool-ai`, all 283 `hutool-log`, and all 294 `hutool-json` records |

## Runtime boundaries

| Requirement | Status | Evidence or boundary |
|---|---|---|
| HTTP timeouts, redirects, body limits, streaming, SSRF hook | complete | `hitool-http` bounded Rustls client and `UrlPolicy` |
| Idempotent-only HTTP retry | complete | explicit capped jitter/backoff and `Retry-After` handling |
| Explicit SQLx pools and transactions | complete | driver features plus real PostgreSQL/MySQL/SQLite transaction tests |
| Cron external runtime, shutdown, timeout, non-overlap, tracing | complete | `spawn_on`, `JobHandle`, `JobPolicy`, per-run spans |
| Cron retry independent from job implementation | complete | `RetryPolicy` and fallible spawn APIs with bounded exponential delay |
| Bounded network/parser/media inputs | complete for current APIs | HTTP, SSE, sockets, ZIP, XLSX, DOCX, image, mail, CAPTCHA, Rhai |

## Security and quality

| Requirement | Status | Evidence or boundary |
|---|---|---|
| No workspace unsafe code | complete | every crate forbids unsafe; CI builds all features |
| Secure crypto defaults | complete | AES-256-GCM, HMAC-SHA-256, Argon2id, scoped HS256 JWT, and RustCrypto-backed HS/RS/ES JOSE signers |
| Legacy crypto isolated | complete | Hutool-named MD5/SHA1/DSA/SM4-CMAC and unavailable ES512 JWT entry points return typed rejection errors instead of enabling unsafe compatibility |
| Supply-chain policy | complete | `cargo-deny` advisories, licenses, bans, and sources checks |
| Default/no-default/all-feature tests | complete | CI quality job and local release gate |
| Per-feature compilation | complete | `cargo hack --each-feature --locked` |
| Stable, MSRV, Nightly, GNU/MUSL, macOS, Windows | complete | CI matrix; MSRV is Rust 1.85 |
| Property, compile-fail, fuzz and integration testing | complete for current parsers | codec properties; compile-fail docs; structured parser fuzz targets; real HTTP/database tests |
| 100% test coverage | in progress | current all-feature baseline: lines 98.51% (25,333/25,715), regions 98.49% (41,433/42,070), functions 97.99% (3,465/3,536); the facade registry, JWT, cache, system, cron, setting, AI, log, JSON, and HTTP crates are each exactly 100%, and `hitool-core/src/builder.rs` is exactly 100% across 723 lines, 1,073 regions, and 118 functions; CI still requires 100% workspace-wide |
| SemVer regression check | ready after first release | tag workflow runs `cargo-semver-checks`; no published baseline exists yet |
| Performance baseline | deferred with evidence required | no hotspot is claimed in `0.1`; add Criterion baselines only after representative profiles identify one |

## Gates before a public 1.0

- Reserve and publish the chosen crates.io names in dependency order.
- Complete an independent security and license/provenance review.
- Define the exact 1.0 Hutool capability scope and freeze those public APIs.
- Establish a released SemVer baseline and supported-version policy.
- Run long-duration fuzz campaigns and record their corpus, duration, and
  toolchain; CI currently verifies that every fuzz target builds.
- Add performance baselines for hotspots demonstrated by production-shaped
  benchmarks rather than for every utility function.
