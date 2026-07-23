# Hutool parity evidence

The source baseline is Hutool `v5.8.46`, commit
`a0bd223dc0d036f55cfe4d8e2f5737ddc31f2b12`.

## API surface parity

`hutool-v5.8.46-api.csv` is a generated inventory of public production types
and methods. Each record needs one row in `decisions.csv` before parity can be
claimed. A decision counts only when it names an implemented/native/idiomatic
HiTool symbol and concrete test evidence.

Regenerate and verify with:

```shell
./scripts/generate-hutool-inventory.sh ../hutool
# … record-*-parity.py helpers …
./scripts/verify-parity.py
./scripts/verify-parity.py --require-complete
```

The strict command intentionally fails while any API is unaccounted for.

## TEST method parity (two bars)

Goal: every Hutool `@Test` / `@ParameterizedTest` method has a hitool-rs
counterpart with **identical inputs, fixtures, assertions, and observable
outputs** (language/runtime differences aside).

| Bar | Meaning | Gate |
| --- | --- | --- |
| **Registration** | Every Java test id appears in the ledger (`covered` / `ignored` / `planned`) | `verify-test-parity.py --require-complete` |
| **Behavioral** | Every id is **runnable** `covered` — **no** `#[ignore]` stubs; same logic/fixtures/outputs as Hutool | `verify-test-parity.py --require-behavioral` |

`#[ignore]` alignment stubs **do not** satisfy behavioral parity. They only keep
the inventory id registered while APIs are still being ported.

`hitool-poi` may keep signature-only empty bodies until easyexcel-rs /
easydoc-rs / easyofd-rs / easypdf-rs land, but those tests must still become
real asserts once the engines exist — empty/`ignore` is temporary, not the
end state.

| Artifact | Role |
| --- | --- |
| `hutool-v5.8.46-tests.csv` | Inventory of Hutool test methods |
| `test-decisions.csv` | Coverage ledger (`covered` / `ignored` / `planned` / `unresolved`) |
| `scripts/generate-hutool-test-inventory.py` | Build the inventory from a Hutool checkout |
| `scripts/scan-hitool-test-parity.py` | Scan Rust `对齐 Java:` comments; marks `ignored` when `#[ignore]` |
| `scripts/verify-test-parity.py` | Report registration + behavioral coverage |

```shell
python3 scripts/generate-hutool-test-inventory.py ../hutool
python3 scripts/scan-hitool-test-parity.py
python3 scripts/verify-test-parity.py --by-module
python3 scripts/verify-test-parity.py --require-complete      # inventory registered
python3 scripts/verify-test-parity.py --require-behavioral    # true Hutool equivalence
```

CI (`.github/workflows/ci.yml`, `quality` job) runs the behavioral gate with
`continue-on-error: true` until the ledger is fully green; local runs should
treat a non-zero exit as the authoritative bar.

Rust parity tests must cite the Java method, for example:

```rust
/// 对齐 Java: `StrUtilTest.isBlankTest()`
#[test]
fn is_blank_test() { /* same inputs / fixtures / asserts as Hutool */ }
```

Do **not** mark behavioral-complete with:

```rust
/// 对齐 Java: `FooTest.bar()`
#[test]
#[ignore = "API not yet ported"]
fn bar() {}
```

Name mapping also accepts `snake_case` ↔ `camelCase` when the file header
contains `对齐: …XxxTest` or the filename is `*_parity.rs`.

