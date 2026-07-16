# Feature matrix

`hitool` features are additive. The default set is only `core` and `json`;
`full` is provided for convenience but is never enabled implicitly.

| Feature | Adds | Compile cost | Runtime/platform notes | Security impact |
|---|---|---:|---|---|
| `core` | `hitool-core` | low | portable | parsers enforce typed errors |
| `json` | `hitool-json` | low | portable | full-input parsing; callers bound input transport |
| `aop` | interceptor chains | low | synchronous | advice can observe context by design |
| `bloom-filter` | probabilistic lookup | low | portable | false positives are expected |
| `cache` | Moka cache | medium | threads | applications must choose capacity/TTL |
| `cron` | cron parser and Tokio jobs | medium | Tokio runtime supplied by caller | cancellation, timeout, tracing, non-overlap and independent bounded retry policy |
| `crypto` | AES-GCM, HMAC, SHA-256, Argon2id | medium | OS randomness | secure defaults only |
| `crypto-legacy` | reserved legacy namespace | none today | explicit opt-in | never use for passwords/authentication |
| `db` | shared pagination/pool policy | medium | no driver selected | no global pool or hidden transaction |
| `db-postgres` | SQLx PostgreSQL | high | network/database; real CI transaction test | explicit driver and URL |
| `db-mysql` | SQLx MySQL | high | network/database; real CI transaction test | explicit driver and URL |
| `db-sqlite` | SQLx SQLite | high | bundled SQLite; real transaction test | file permissions remain caller-owned |
| `dfa` | Aho-Corasick matching | low | portable | pattern/input size affects memory |
| `extra` | QR SVG and ZIP | medium | filesystem for extraction | path traversal, symlinks and expansion are bounded |
| `extra-image` | decode, dimensions, resize, crop, encode | high | pure Rust codecs | encoded bytes, dimensions, pixels and output bytes are bounded |
| `extra-mail` | SMTP and MIME messages | high | Tokio, network, Rustls | credentials are redacted; plaintext SMTP is explicit; recipients/body/attachments are bounded |
| `http` | async Reqwest/Rustls | high | Tokio runtime supplied by caller | limits and URL policy; retries require an explicit policy and idempotent method |
| `http-blocking` | blocking Reqwest client | high | blocks current thread | explicit opt-in |
| `log` | tracing setup/redaction | medium | portable | sensitive values require `Redacted` wrapper |
| `script` | constrained Rhai | medium | in-process | operation/depth/container limits; dynamic eval disabled |
| `setting` | layered config | medium | filesystem/environment | do not log secret config values |
| `system` | host metrics | medium | OS-specific backends | may expose host metadata |
| `poi` | bounded XLSX read/write and CSV read/write | high | pure Rust | archive, expanded XML, row, column and materialized-cell ceilings |
| `poi-docx` | bounded DOCX generation | medium | pure Rust | paragraphs, runs, text, cells and output bytes are bounded; no macros or external relationships |
| `captcha` | code generation, verification and SVG rendering | low | OS randomness | storage/rate limits remain caller-owned; CAPTCHA alone is not a high-assurance bot defense |
| `captcha-raster` | randomized bitmap glyph PNG rendering | high | pure Rust image codec | dimensions, pixels and code length are bounded |
| `captcha-audio` | injected speech PCM to noisy WAV | low | caller supplies speech engine | sample rate, duration and sample count are bounded; synthesized content remains caller-owned |
| `socket` | bounded async TCP/UDP helpers | medium | Tokio, OS sockets | connect timeout and frame limits |
| `jwt` | scoped HS256 JWT | medium | portable | issuer/audience/leeway/expiry are mandatory policy |
| `ai` | provider abstraction/OpenAI-compatible JSON and SSE client | high | HTTP/Tokio | API key redacted; HTTP and SSE event sizes are bounded |
| `hutool-compat` | `StrUtil`/`JsonUtil` migration API | low | portable | isolated from idiomatic APIs |

MSRV is Rust 1.85. Feature additions must not change semantics of already
enabled functions. Database drivers and alternative blocking behavior remain
explicit opt-ins.
