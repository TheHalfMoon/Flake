"""Independent oracle for T03-08's automated continuity qualification.

Deliberately never imports `analysis.py`, `flake_arm.py`, or `baseline_arm.py`
(the producer path). Reads the raw `.jsonl` manifest itself, groups records
with string-keyed dicts (not the tuple-keyed dicts `analysis.py` uses), and
recomputes the same seven-branch route with an independently written
grading pass. `test_verify_independent.py` cross-checks that this module and
`analysis.py` agree exactly on every route their test fixtures exercise.
"""
from __future__ import annotations

import json
from pathlib import Path
from typing import Any

REQUIRED_TIERS = ["T1", "T2", "T3", "T4"]


class VerificationError(RuntimeError):
    pass


def _cell_key(case_id: str, arm: str) -> str:
    return f"{case_id}|{arm}"


def read_raw_records(manifest_path: Path) -> list[dict[str, Any]]:
    raw = manifest_path.read_text(encoding="utf-8")
    out = []
    for line in raw.split("\n"):
        stripped = line.strip()
        if stripped:
            out.append(json.loads(stripped))
    return out


def read_gold_by_case(cases_path: Path) -> dict[str, Any]:
    payload = json.loads(cases_path.read_text(encoding="utf-8"))
    return {cid: c["gold"] for cid, c in payload["cases"].items()}


def read_sources_by_case(cases_path: Path) -> dict[str, Any]:
    payload = json.loads(cases_path.read_text(encoding="utf-8"))
    return {cid: c["sources"] for cid, c in payload["cases"].items()}


def _grade_one(record: dict[str, Any], gold: dict[str, Any], sources: list[dict[str, Any]]) -> tuple[bool, bool]:
    """Returns (is_correct, is_high_consequence_miss), independently derived."""
    if record.get("error"):
        return False, False

    superseded_ids = gold["superseded_doc_ids"]
    trap = gold["high_consequence_trap"]
    current_ids = record["current_doc_ids"]

    hc_miss = False
    if trap is not None:
        for cid in current_ids:
            if cid == trap["trap_doc_id"]:
                hc_miss = True

    leaked_superseded = False
    for cid in current_ids:
        for sid in superseded_ids:
            if cid == sid:
                leaked_superseded = True

    conflict_ids = gold["planted_conflict_doc_ids"]
    if len(conflict_ids) > 0:
        matches_conflict = sorted(current_ids) == sorted(conflict_ids)
        is_correct = bool(record.get("conflict_flagged")) and matches_conflict and not leaked_superseded
    else:
        last_doc_id = sources[-1]["doc_id"]
        matches_single = sorted(current_ids) == [last_doc_id]
        is_correct = (not record.get("conflict_flagged")) and matches_single and not leaked_superseded

    if hc_miss:
        is_correct = False

    return is_correct, hc_miss


def verify(manifest_path: Path, cases_path: Path, forbid_case_source: str | None = None) -> dict[str, Any]:
    records = read_raw_records(manifest_path)
    gold_by_case = read_gold_by_case(cases_path)
    sources_by_case = read_sources_by_case(cases_path)
    case_count = len(gold_by_case)

    if forbid_case_source is not None:
        for r in records:
            if r.get("case_source") == forbid_case_source:
                raise VerificationError(
                    f"case_source {forbid_case_source!r} must never reach independent verification"
                )

    index: dict[str, dict[str, Any]] = {}
    for r in records:
        index[_cell_key(r["case_id"], r["arm"])] = r

    missing_list = []
    for case_id in gold_by_case:
        for arm in ("flake", "baseline"):
            if _cell_key(case_id, arm) not in index:
                missing_list.append([case_id, arm])
    if len(missing_list) > 0:
        return {
            "route": "INCONCLUSIVE: MISSING_EXECUTIONS",
            "case_count": case_count,
            "missing_cells": missing_list,
        }

    hc_miss_cases = []
    correctness: dict[str, bool] = {}
    for case_id, gold in gold_by_case.items():
        for arm in ("flake", "baseline"):
            record = index[_cell_key(case_id, arm)]
            is_correct, hc_miss = _grade_one(record, gold, sources_by_case[case_id])
            correctness[_cell_key(case_id, arm)] = is_correct
            if arm == "flake" and hc_miss:
                hc_miss_cases.append(case_id)

    if len(hc_miss_cases) > 0:
        return {
            "route": "FAIL: HIGH_CONSEQUENCE_MISS",
            "case_count": case_count,
            "high_consequence_miss_cases": sorted(hc_miss_cases),
        }

    tier_correct_count = {t: 0 for t in REQUIRED_TIERS}
    for r in records:
        if r["arm"] == "flake" and correctness[_cell_key(r["case_id"], "flake")]:
            tier_correct_count[r["tier"]] = tier_correct_count.get(r["tier"], 0) + 1

    lost_tiers = [t for t in REQUIRED_TIERS if tier_correct_count.get(t, 0) == 0]
    if len(lost_tiers) > 0:
        return {
            "route": "FAIL: CLASS_LOSS",
            "case_count": case_count,
            "class_loss_tiers": lost_tiers,
        }

    flake_correct_total = 0
    baseline_correct_total = 0
    for key, is_correct in correctness.items():
        if not is_correct:
            continue
        if key.endswith("|flake"):
            flake_correct_total += 1
        elif key.endswith("|baseline"):
            baseline_correct_total += 1

    flake_rate = flake_correct_total / case_count
    baseline_rate = baseline_correct_total / case_count

    if flake_rate < 0.90:
        return {
            "route": "FAIL: RESUME_SUCCESS_BELOW_90PCT",
            "case_count": case_count,
            "flake_success_rate": flake_rate,
        }

    deficit_points = (baseline_rate - flake_rate) * 100.0
    if deficit_points > 5.0:
        return {
            "route": "FAIL: DEFICIT_EXCEEDS_5PP",
            "case_count": case_count,
            "deficit_pp": deficit_points,
        }

    return {
        "route": "PASS",
        "case_count": case_count,
        "flake_correct": flake_correct_total,
        "baseline_correct": baseline_correct_total,
        "flake_success_rate": flake_rate,
        "baseline_success_rate": baseline_rate,
        "deficit_pp": deficit_points,
        "high_consequence_misses": 0,
        "tier_breakdown": {t: {"flake_correct": tier_correct_count[t]} for t in REQUIRED_TIERS},
    }


def main() -> int:
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--cases", required=True)
    parser.add_argument("--forbid-case-source", default=None)
    args = parser.parse_args()
    result = verify(Path(args.manifest), Path(args.cases), args.forbid_case_source)
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    import sys
    sys.exit(main())
