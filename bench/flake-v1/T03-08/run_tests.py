#!/usr/bin/env python3
"""Aggregate test runner for the T03-08 package, mirroring
`bench/flake-v1/T03-07/run_tests.py`'s own convention."""
import sys
import unittest
from pathlib import Path

HERE = Path(__file__).parent.resolve()
sys.path.insert(0, str(HERE))

SUITES = [
    "test_flake_arm",
    "test_baseline_arm",
    "test_harness",
    "test_analysis",
    "test_verify_independent",
]


def main() -> int:
    loader = unittest.TestLoader()
    suite = unittest.TestSuite()
    for name in SUITES:
        suite.addTests(loader.loadTestsFromName(name))
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    sys.exit(main())
