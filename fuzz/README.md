# Hutool-Rust fuzz targets

Install `cargo-fuzz`, then run a target from the repository root:

```text
cargo fuzz run codec_round_trip
cargo fuzz run json_parse
cargo fuzz run structured_parsers
cargo fuzz run text_search
```

`structured_parsers` covers bounded XLSX/ZIP/XML, URL, Cron, and JWT inputs;
the dedicated targets cover JSON, codecs, and multi-pattern UTF-8 search.

The fuzz package is intentionally excluded from the release workspace because
`libfuzzer-sys` is a test-only native runtime and must never enter published
crate dependency graphs.
