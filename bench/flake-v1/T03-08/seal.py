#!/usr/bin/env python3
"""Seals the T03-08 automated continuity qualification harness/analysis
artifacts, before any confirmatory (case-bank-sealed) execution is run
against them, mirroring T03-07's own `seal.py`/PROTOCOL.md section 18
discipline. This is a code-freeze seal (this task is execution, not
preregistration), so any later change to a sealed file after confirmatory
records exist requires re-sealing and re-running, never a silent edit.
"""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()
REPO_ROOT = HERE.parent.parent.parent  # bench/flake-v1/T03-08 -> repo root

SEALED_FILES = [
    "PROTOCOL_ADDENDUM.md",
    "flake_arm.py",
    "baseline_arm.py",
    "harness.py",
    "analysis.py",
    "verify_independent.py",
]

# The sealed T03-07 case bank this run reuses as a fixed held-out
# engineering corpus (founder decision, "Use the sealed T03-07 confirmatory
# case corpus"). Recorded here so an independent reviewer can confirm this
# run's cases_confirmatory.json is byte-identical to the one T03-07 sealed.
REUSED_T03_07_FILES = [
    "../T03-07/cases_confirmatory.json",
]


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*args: str) -> str:
    result = subprocess.run(["git", *args], cwd=REPO_ROOT, capture_output=True, text=True, check=True)
    return result.stdout.strip()


def main() -> int:
    digests = {}
    for name in SEALED_FILES + REUSED_T03_07_FILES:
        path = HERE / name
        if not path.exists():
            print(f"ERROR: sealed file missing: {name}", file=sys.stderr)
            return 1
        digests[name] = sha256_file(path)

    t03_07_seal = json.loads((HERE / "../T03-07/SEALS.json").resolve().read_text(encoding="utf-8"))
    expected = t03_07_seal["file_digests_sha256"]["cases_confirmatory.json"]
    actual = digests["../T03-07/cases_confirmatory.json"]
    if expected != actual:
        print(
            f"ERROR: reused cases_confirmatory.json digest {actual} does not match "
            f"T03-07's own sealed digest {expected} -- the sealed case bank has drifted",
            file=sys.stderr,
        )
        return 1

    product_commit = git("rev-parse", "HEAD")
    product_src_tree = git("rev-parse", f"{product_commit}:src")

    seal = {
        "schema_version": "t03-08-seal-v1",
        "sealed_at_note": "harness/analysis code frozen before any confirmatory attempt was run",
        "founder_decision": "docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md",
        "t03_07_case_bank_seal_confirmed": True,
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
