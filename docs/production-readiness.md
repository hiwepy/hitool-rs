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
| Complete Hutool functional parity | in progress | pinned v5.8.46 inventory contains 13,871 public production API records; 378 records have implementation and executable evidence: all 175 `core.codec` records plus 203 reviewed `core.collection` records |

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
| Secure crypto defaults | complete | AES-256-GCM, HMAC-SHA-256, Argon2id, scoped HS256 JWT |
| Legacy crypto isolated | complete | explicit empty migration feature; insecure algorithms are not exported |
| Supply-chain policy | complete | `cargo-deny` advisories, licenses, bans, and sources checks |
| Default/no-default/all-feature tests | complete | CI quality job and local release gate |
| Per-feature compilation | complete | `cargo hack --each-feature --locked` |
| Stable, MSRV, Nightly, GNU/MUSL, macOS, Windows | complete | CI matrix; MSRV is Rust 1.85 |
| Property, compile-fail, fuzz and integration testing | complete for current parsers | codec properties; compile-fail docs; structured parser fuzz targets; real HTTP/database tests |
| 100% test coverage | in progress | current all-feature baseline: lines 86.23%, regions 86.83%, functions 84.76%; new collection type, partition, iterator, and utility code is 100% across all three metrics and CI requires 100% workspace-wide |
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
