# Source provenance

## Rules

1. Mature Rust crates are preferred as execution engines.
2. Hutool Java source defines capability coverage and compatibility semantics;
   Java implementations are not mechanically translated.
3. Code adapted from yimi-rutool must be reviewed, attributed, and substantially
   revised for Hutool-Rust's crate boundaries, error model, and security defaults.
4. Security-sensitive code is implemented through audited upstream crates and is
   never copied merely for API parity.

## Initial sources

| Source | Version | License | Use |
|---|---:|---|---|
| yimi-rutool | 0.2.5 | Apache-2.0 as distributed | Core/JSON behavior and test ideas |
| Hutool | local checkout | Mulan PSL v2 repository license | Capability and behavior reference |

