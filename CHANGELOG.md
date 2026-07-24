# Changelog

All notable changes are documented here. The project follows Semantic
Versioning once public releases begin.

## [Unreleased]

### Added

- Hutool-aligned workspace with the `hutool` facade and 21 capability crates.
- Additive feature model with minimal `core` + `json` defaults.
- Initial production-oriented implementations for the capabilities exposed by the `hutool` facade; POI is excluded.
- Provenance, security, feature-cost and Hutool-parity documentation.
- Explicit idempotent HTTP retries and bounded OpenAI-compatible SSE streams.
- Safe image transforms; injectable Rustls SMTP/MIME mail; randomized SVG/PNG
  and injectable audio CAPTCHA rendering.
- Property tests, compile-fail documentation, and parser fuzz targets.
- Fallible cron jobs with independent bounded exponential retry, execution
  timeout, cancellation, non-overlap, and tracing outcomes.
- Real PostgreSQL, MySQL, and SQLite pool/transaction integration tests.
- Independent `hutool-observability` crate with default tracing, Prometheus
  metrics, and health reporting. CPU profiling, Tokio Console, and DHAT heap
  profiling require both a non-default feature and runtime authorization.
- Bounded single-buffer XML event reading, early-stop visitors, streaming
  transforms, iterative DOM construction and `quick_xml::Writer` output.
- Optional direct XML Serde, encoding and Tokio adapters through the
  `xml-serde`, `xml-encoding` and `xml-async` features.

### Security

- Rustls HTTP defaults with explicit network and response limits.
- Argon2id password hashing and AES-256-GCM authenticated encryption.
- Scoped JWT validation and bounded script/archive processing.
- Image, mail and CAPTCHA media limits; SSRF policy hooks;
  idempotent-only retries; and bounded SSE event framing.
- XML input, depth, node, per-element attribute and cumulative text limits;
  DTD and unknown named general references are rejected by default.
