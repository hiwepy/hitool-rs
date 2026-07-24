# Performance baselines

Hutool-Rust adds benchmarks only for measured hot paths with a stable input corpus.
This directory records that policy so release work does not accumulate
decorative microbenchmarks. Candidate baselines for the API-freeze phase are
DFA scanning, codec throughput, cache behavior, and bounded HTTP framing.
XLSX is excluded because `hutool-poi` has no implementation.
