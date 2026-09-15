#!/usr/bin/env python3
"""Runs every test_*.py suite in this directory and reports a combined
result. Mirrors bench/R1's convention of standalone `python3 test_x.py`
invocation; this is a convenience aggregator, not a new test framework."""
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()

TEST_FILES = [
    "test_cases.py",
    "test_allocation.py",
    "test_harness.py",
    "test_analysis.py",
    "test_verify_independent.py",
    "test_dry_run.py",
    "test_protocol.py",
]


def main() -> int:
    overall = 0
    for name in TEST_FILES:
        print(f"\n{'#' * 70}\n# {name}\n{'#' * 70}")
        result = subprocess.run([sys.executable, str(HERE / name)], cwd=HERE)
        if result.returncode != 0:
            overall = 1
    return overall


if __name__ == "__main__":
    sys.exit(main())
