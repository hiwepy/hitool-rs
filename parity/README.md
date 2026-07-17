# Hutool parity evidence

The source baseline is Hutool `v5.8.46`, commit
`a0bd223dc0d036f55cfe4d8e2f5737ddc31f2b12`.

`hutool-v5.8.46-api.csv` is a generated inventory of public production types
and methods. Each record needs one row in `decisions.csv` before parity can be
claimed. A decision counts only when it names an implemented/native/idiomatic
HiTool symbol and concrete test evidence.

Regenerate and verify with:

```shell
./scripts/generate-hutool-inventory.sh ../hutool
./scripts/record-codec-parity.py
./scripts/record-collection-types-parity.py
./scripts/record-getter-parity.py
./scripts/record-clone-parity.py
./scripts/record-compiler-parity.py
./scripts/record-stream-parity.py
./scripts/record-compress-parity.py
./scripts/record-aop-parity.py
./scripts/record-dfa-parity.py
./scripts/record-bloom-filter-parity.py
./scripts/record-script-parity.py
./scripts/record-captcha-parity.py
./scripts/record-socket-parity.py
./scripts/record-jwt-parity.py
./scripts/record-cache-parity.py
./scripts/record-all-parity.py
./scripts/record-system-parity.py
./scripts/record-cron-parity.py
./scripts/verify-parity.py
./scripts/verify-parity.py --require-complete
```

The strict command intentionally fails while any API is unaccounted for.
