# Hutool parity ledger

HiTool aligns with Hutool's capability model, not Java's reflection/runtime
model. Status values are `native`, `idiomatic`, `compatible`,
`unsafe-to-copy`, and `planned`.

| Hutool module | HiTool crate | Status | Current mapping |
|---|---|---|---|
| `hutool-all` | `hitool` | idiomatic | Cargo feature-gated facade plus deterministic compile-time capability registry |
| `hutool-bom` | workspace | native | lockstep versions and workspace dependencies |
| `hutool-core` | `hitool-core` | idiomatic | strings, collections, codecs, dates, IDs, closure/Serde-backed builders, ownership-aware mutable wrappers, Hutool boolean vocabulary/aggregations, checked endian-aware `ByteUtil`, Unicode-category character inspection and enclosed conversions, `encoding_rs`-backed charset conversion/detection, GB 32100 credit codes, standard-library WGS84/GCJ-02/BD-09/Mercator coordinates, Unicode-safe data masking, `chrono`-backed mainland/Taiwan/Macao/Hong Kong identity-card validation and information extraction, owned/range-based pagination, `regex`-backed regional phone validation, bounded custom radix conversion, loose version-expression matching, Java UTF-16 classic hashes, and a `hex`/`num-bigint`-backed `HexUtil` facade |
| `hutool-json` | `hitool-json` | complete | Serde typed/dynamic JSON plus Hutool-aligned configured objects/arrays, bounded tokenizer/parser, XML conversion, stateful writer and owned serializer registry |
| `hutool-http` | `hitool-http` | idiomatic | Reqwest/Rustls client with limits, URL policy, streaming and explicit idempotent retry; Hutool-aligned methods, headers, content types, status codes, shared metadata and a bounded `HttpResponse` facade for status/header/body inspection and output; `HttpConfig` drives real async/blocking proxy, TLS, cache and interceptor behavior; User-Agent parsing backed by Woothee |
| `hutool-crypto` | `hitool-crypto` | idiomatic | RustCrypto AEAD/HMAC/SHA-256/Argon2id |
| `hutool-jwt` | `hitool-jwt` | idiomatic | dynamic claims, HS/RS/ES/none signers, PEM factories, typed validation, and explicit rejection of obsolete/non-JOSE algorithms |
| `hutool-cache` | `hitool-cache` | idiomatic | Moka native cache plus deterministic FIFO/LFU/LRU/timed/weak/no-op compatibility caches, per-cache prune workers, listeners, statistics, and bounded file caching |
| `hutool-bloomFilter` | `hitool-bloom-filter` | idiomatic | typed probabilistic membership |
| `hutool-dfa` | `hitool-dfa` | idiomatic | Aho-Corasick leftmost-longest matching |
| `hutool-setting` | `hitool-setting` | complete | ordered grouped settings and sets, variable expansion, Java properties and typed conversion, owned/injectable auto-reload, profiles, layered files/environment, and YAML |
| `hutool-cron` | `hitool-cron` | complete | Hutool-aligned 5/6/7-part patterns, parsers and matchers; task tables, listeners and executors; explicitly owned Tokio schedulers; timing wheel/system timer; cancellation, timeout, tracing, non-overlap and bounded retries |
| `hutool-log` | `hitool-log` | complete | Hutool-aligned levels, records, factories, static facade and backend aliases over an injectable tracing sink; logger caches are owned and the compatibility global is replaceable/resettable |
| `hutool-system` | `hitool-system` | idiomatic | sysinfo-backed CPU/process/memory/disk/network/sensor snapshots plus OS, host, user, runtime and explicit optional Java/JVM properties |
| `hutool-aop` | `hitool-aop` | idiomatic | explicit interceptor chain instead of runtime proxying |
| `hutool-script` | `hitool-script` | idiomatic | bounded Rhai engine; no JSR-223 globals |
| `hutool-socket` | `hitool-socket` | idiomatic | Tokio TCP/UDP, bounded AIO/NIO sessions, protocol traits, timeouts, and managed shutdown |
| `hutool-extra` | `hitool-extra` | idiomatic | QR SVG, safe ZIP, bounded image transforms, injectable Rustls SMTP/MIME mail |
| `hutool-poi` | `hitool-poi` | planned | current bounded XLSX/CSV/DOCX bootstrap remains available, but full parity is deferred until `easyexcel-rs`, `easydoc-rs`, `easyofd-rs`, and `easypdf-rs` can serve as dedicated engines behind a thin Hutool-compatible facade |
| `hutool-captcha` | `hitool-captcha` | idiomatic | generator/challenge verification, randomized SVG/PNG, and injectable speech-to-WAV audio rendering |
| `hutool-db` | `hitool-db` | idiomatic | SQLx pools, explicit transactions and pagination, not a custom ORM; PostgreSQL/MySQL/SQLite integration-tested |
| `hutool-ai` | `hitool-ai` | complete | provider-neutral core plus Hutool-aligned configs, models, factory and exhaustive operations for seven providers; bounded JSON/media, proxy, redacted secrets and SSE |
| `StrUtil`/`JSONUtil` | `hitool-compat-hutool` | compatible | focused migration facade |
| Bean reflection copy | none | unsafe-to-copy | use Serde, `From`/`TryFrom`, or derive macros |
| global HTTP/config/DB singletons | none | unsafe-to-copy | clients and pools are explicitly injected |

The ledger records capability direction, not a claim of method-for-method API
completion. New compatibility APIs must delegate to the idiomatic crate and
must not introduce a second implementation.

## Unportable matrix

HiTool forbids `unsafe` and does not embed a JVM. The following surfaces are
**registered** as `planned` or `unsafe-to-copy` and are excluded from the
`feasible_covered` KPI (`python3 scripts/verify-parity.py --feasible`).

| Tag | Typical Hutool area | HiTool disposition |
|---|---|---|
| `awt_swing` | `core.swing`, desktop UI | planned — no AWT |
| `javax_servlet` | `extra.servlet` | planned — no Servlet container |
| `jndi` | JNDI factories | planned — explicit injection instead |
| `reflection` | ReflectUtil, ClassUtil, BeanDesc, proxies | unsafe-to-copy — Serde / `From` / macros |
| `javax_sql_spi` | JDBC Statement/Connection wrappers, globals | planned — SQLx pools only |
| `bouncycastle_only` | ZUC, Cipher SPI, pure-BC param types | planned — RustCrypto subset |
| `soap_server` | SOAP clients, SimpleServer, HttpConnection | planned — reqwest client model |
| `jvm_only` | SSH/FTP/Spring/CGLIB/template/tokenizer, poi engines | planned — or deferred to `easy*` for poi |
| `portable` | everything else | idiomatic when implemented; planned only as a temporary Wave ledger row |

Classifier: `python3 scripts/classify-unportable.py --summary`
Tags CSV: `python3 scripts/classify-unportable.py --write-csv parity/unportable-tags.csv`

### Migration DoD (ledger)

1. **registered = 100%** of pinned v5.8.46 APIs
2. Maximize **idiomatic** for `portable` tags
3. Keep TEST registration 100%; behavioral planned only for declared unportable tests
4. Never delete existing idiomatic implementations; facades must delegate
5. `hutool-poi` stays planned stubs until `easyexcel-rs` / `easydoc-rs` / `easyofd-rs` / `easypdf-rs`
