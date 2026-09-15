#!/usr/bin/env python3
"""Writes allocation_confirmatory.json from allocation.py's pure functions.

Re-running this script reproduces byte-identical output -- the same
reconstructability check as generate_cases.py.
"""
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()
sys.path.insert(0, str(HERE))

from allocation import full_allocation_table, counterbalance_summary
from generate_cases import TIER_BY_PAIR_INDEX


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def main() -> int:
    rows = full_allocation_table(TIER_BY_PAIR_INDEX)
    summary = counterbalance_summary(rows)

    payload = {
        "schema_version": "t03-07-allocation-v1",
        "tier_by_pair_index": TIER_BY_PAIR_INDEX,
        "rows": rows,
        "counterbalance_summary": summary,
    }
    out_path = HERE / "allocation_confirmatory.json"
    text = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    out_path.write_text(text, encoding="utf-8")
    print(f"allocation_confirmatory.json: {len(rows)} pair-rows, "
          f"sha256={sha256_hex(out_path.read_bytes())}")
    print(json.dumps(summary, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
