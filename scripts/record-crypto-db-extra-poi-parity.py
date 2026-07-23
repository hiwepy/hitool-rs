#!/usr/bin/env python3
"""Run crypto/db/extra/poi decision ledger recorders in order."""

from __future__ import annotations

import runpy
from pathlib import Path

SCRIPTS = [
    "scripts/record-crypto-parity.py",
    "scripts/record-db-parity.py",
    "scripts/record-extra-parity.py",
    "scripts/record-poi-parity.py",
]


def main() -> None:
    root = Path(__file__).resolve().parents[1]
    for rel in SCRIPTS:
        print(f"==> {rel}")
        runpy.run_path(str(root / rel), run_name="__main__")


if __name__ == "__main__":
    main()
