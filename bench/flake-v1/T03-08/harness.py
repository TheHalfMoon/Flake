"""Resumable, append-only harness for T03-08's automated continuity
qualification. Runs every case in a sealed T03-07 case bank through both the
real Flake CLI arm (`flake_arm.py`) and the independent maintained-Markdown
baseline arm (`baseline_arm.py`), appending one raw JSON record per
(case_id, arm) to `runs/<run_name>/records.jsonl`.

Mirrors `bench/flake-v1/T03-07/harness.py`'s own resumability discipline:
the manifest is scanned first, already-recorded (case_id, arm) cells are
skipped, and records are appended one line at a time so an interrupted run
can be resumed without risk of a duplicate or silently lost attempt.
"""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()
REPO_ROOT = HERE.parent.parent.parent

sys.path.insert(0, str(HERE))
import baseline_arm  # noqa: E402
import flake_arm  # noqa: E402


class DuplicateCellError(RuntimeError):
    pass


def default_binary() -> Path:
    for candidate in ("target/release/fehrest.exe", "target/release/fehrest",
                       "target/debug/fehrest.exe", "target/debug/fehrest"):
        p = REPO_ROOT / candidate
        if p.exists():
            return p
    raise FileNotFoundError("no built fehrest binary found under target/{release,debug}")


def git_head() -> str:
    out = subprocess.run(["git", "rev-parse", "HEAD"], cwd=REPO_ROOT,
                          capture_output=True, text=True, check=True)
    return out.stdout.strip()


def load_cases(cases_path: Path) -> dict:
    return json.loads(cases_path.read_text(encoding="utf-8"))


def existing_cells(manifest_path: Path) -> set[tuple[str, str]]:
    if not manifest_path.exists():
        return set()
    cells = set()
    with manifest_path.open("r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            rec = json.loads(line)
            cells.add((rec["case_id"], rec["arm"]))
    return cells


def append_record(manifest_path: Path, record: dict, seen: set[tuple[str, str]]) -> None:
    cell = (record["case_id"], record["arm"])
    if cell in seen:
        raise DuplicateCellError(f"cell {cell} already recorded")
    manifest_path.parent.mkdir(parents=True, exist_ok=True)
    with manifest_path.open("a", encoding="utf-8") as f:
        f.write(json.dumps(record, sort_keys=True) + "\n")
    seen.add(cell)


def run_collection(cases: dict, run_dir: Path, binary: Path,
                    harness_version: str, product_version: str,
                    case_source: str) -> None:
    for case_id, case in cases["cases"].items():
        if case.get("confirmatory") and case_source != "confirmatory":
            raise RuntimeError(
                f"refusing to run confirmatory case {case_id} under case_source={case_source!r}"
            )

    manifest_path = run_dir / "records.jsonl"
    baseline_dir = run_dir / "baseline_vault"
    baseline_dir.mkdir(parents=True, exist_ok=True)
    vault_dir = run_dir / "flake_vault"

    seen = existing_cells(manifest_path)

    if not vault_dir.exists():
        flake_arm.run_cli(binary, vault_dir, ["canonical-init"])

    for case_id in sorted(cases["cases"].keys()):
        case = cases["cases"][case_id]
        if (case_id, "flake") not in seen:
            record = flake_arm.run_case(binary, vault_dir, case, harness_version, product_version)
            record["case_source"] = case_source
            append_record(manifest_path, record, seen)
        if (case_id, "baseline") not in seen:
            record = baseline_arm.run_case(baseline_dir, case, harness_version)
            record["case_source"] = case_source
            append_record(manifest_path, record, seen)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cases", required=True, help="path to a cases_*.json file")
    parser.add_argument("--run-name", required=True, help="subdirectory under runs/")
    parser.add_argument("--case-source", required=True, choices=["confirmatory", "dev_synthetic"])
    parser.add_argument("--binary", default=None, help="path to the fehrest binary")
    args = parser.parse_args()

    cases = load_cases(Path(args.cases))
    binary = Path(args.binary) if args.binary else default_binary()
    run_dir = HERE / "runs" / args.run_name
    harness_version = git_head()
    product_version = f"{harness_version} ({binary.name})"

    run_collection(cases, run_dir, binary, harness_version, product_version, args.case_source)
    print(f"run complete: {run_dir / 'records.jsonl'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
