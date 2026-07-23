# Changelog

All notable changes are documented here. The project follows Semantic
Versioning once public releases begin.

## [Unreleased]

### Added

- Hutool-aligned workspace with the `hutool` facade and 20 capability crates.
- Additive feature model with minimal `core` + `json` defaults.
- Initial production-oriented implementations for every public capability.
- Provenance, security, feature-cost and Hutool-parity documentation.
- Explicit idempotent HTTP retries and bounded OpenAI-compatible SSE streams.
- Bounded XLSX reading and minimal DOCX generation; safe image transforms;
  injectable Rustls SMTP/MIME mail; randomized SVG/PNG and injectable audio
  CAPTCHA rendering.
- Property tests, compile-fail documentation, and parser fuzz targets.
- Fallible cron jobs with independent bounded exponential retry, execution
  timeout, cancellation, non-overlap, and tracing outcomes.
- Real PostgreSQL, MySQL, and SQLite pool/transaction integration tests.

### Security

- Rustls HTTP defaults with explicit network and response limits.
- Argon2id password hashing and AES-256-GCM authenticated encryption.
- Scoped JWT validation and bounded script/archive processing.
- XLSX/DOCX, image, mail and CAPTCHA media limits; SSRF policy hooks;
  idempotent-only retries; and bounded SSE event framing.
