#!/usr/bin/env python3
"""Seals the T03-07 preregistration artifacts (PROTOCOL.md section 18).

Writes SEALS.json recording SHA-256 digests of every load-bearing file and
the exact product commit/tree this study is frozen against. Any later
change to a sealed file requires a new seal (and, if a confirmatory
attempt already exists against the old seal, invalidates it under the
repair-repeat rule).
"""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()
REPO_ROOT = HERE.parent.parent.parent  # bench/flake-v1/T03-07 -> repo root

SEALED_FILES = [
    "PROTOCOL.md",
    "CONSENT.md",
    "cases_confirmatory.json",
    "cases_dev.json",
    "allocation_confirmatory.json",
    "analysis.py",
    "verify_independent.py",
    "harness.py",
    "allocation.py",
    "generate_cases.py",
    "generate_allocation.py",
]


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*args: str) -> str:
    result = subprocess.run(["git", *args], cwd=REPO_ROOT, capture_output=True, text=True, check=True)
    return result.stdout.strip()


def main() -> int:
    digests = {}
    for name in SEALED_FILES:
        path = HERE / name
        if not path.exists():
            print(f"ERROR: sealed file missing: {name}", file=sys.stderr)
            return 1
        digests[name] = sha256_file(path)

    # The sealed product identity is the qualified baseline this branch
    # forked from (this task adds no src/ changes), not this task's own
    # evidence-adding commit -- so a later documentation-only commit on
    # this same branch does not silently move the sealed product target.
    product_commit = git("merge-base", "HEAD", "origin/main")
    product_src_tree = git("rev-parse", f"{product_commit}:src")

    seal = {
        "schema_version": "t03-07-seal-v1",
        "sealed_at_note": "sealed before any confirmatory human observation exists",
        "product_commit": product_commit,
        "product_src_tree": product_src_tree,
        "file_digests_sha256": digests,
    }

    out_path = HERE / "SEALS.json"
    out_path.write_text(json.dumps(seal, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(seal, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
