"""Producer analysis for T03-08's automated continuity qualification.

Computes, from the raw manifest only, in the fixed order specified by
`docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md`'s
"Automated P03 pass criteria":

  1. completeness (all 96 cases accounted for in both arms)
  2. safety (zero high-consequence misses on the Flake arm)
  3. class-loss (every tier has >=1 successful Flake case)
  4. resume-success gate (Flake success rate >= 90%)
  5. relative-deficit gate (baseline - Flake <= 5 percentage points)
  6. otherwise PASS

The former time-reduction gate is intentionally absent: the founder decision
removes it because automated execution establishes no human-effort claim.

`verify_independent.py` is a separately written oracle that never imports
this module and must reach the same route from the same raw manifest.
"""
from __future__ import annotations

import json
from pathlib import Path
from typing import Any

TIERS = ("T1", "T2", "T3", "T4")


class AnalysisError(RuntimeError):
    pass


def load_manifest(path: Path) -> list[dict[str, Any]]:
    records = []
    with path.open("r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if line:
                records.append(json.loads(line))
    return records


def load_cases(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))["cases"]


def grade_record(record: dict[str, Any], case: dict[str, Any]) -> tuple[str, bool]:
    """Return (outcome, high_consequence_miss) for one raw record, graded
    against its case's sealed gold key. This is the only function in this
    package that reads `case["gold"]`."""
    if record["error"]:
        return "RESUME_FAILED", False

    gold = case["gold"]
    superseded = set(gold["superseded_doc_ids"])
    trap = gold["high_consequence_trap"]
    trap_id = trap["trap_doc_id"] if trap else None
    current = set(record["current_doc_ids"])
    high_consequence_miss = trap_id is not None and trap_id in current

    planted_conflict = set(gold["planted_conflict_doc_ids"])
    if planted_conflict:
        correct = (
            bool(record["conflict_flagged"])
            and current == planted_conflict
            and not (superseded & current)
        )
    else:
        expected_current = {case["sources"][-1]["doc_id"]}
        correct = (
            not record["conflict_flagged"]
            and current == expected_current
            and not (superseded & current)
        )

    outcome = "RESUME_CORRECT" if (correct and not high_consequence_miss) else "RESUME_FAILED"
    return outcome, high_consequence_miss


def analyze(manifest_path: Path, cases_path: Path, forbid_case_source: str | None = None) -> dict[str, Any]:
    records = load_manifest(manifest_path)
    cases = load_cases(cases_path)
    case_count = len(cases)

    if forbid_case_source is not None:
        for r in records:
            if r.get("case_source") == forbid_case_source:
                raise AnalysisError(
                    f"record for case {r['case_id']} carries forbidden case_source "
                    f"{forbid_case_source!r} -- dev/synthetic data must never be analyzed "
                    "as confirmatory"
                )

    by_cell = {(r["case_id"], r["arm"]): r for r in records}
    missing = [
        (cid, arm) for cid in cases for arm in ("flake", "baseline")
        if (cid, arm) not in by_cell
    ]

    if missing:
        return {
            "route": "INCONCLUSIVE: MISSING_EXECUTIONS",
            "case_count": case_count,
            "missing_cells": missing,
        }

    graded = {}
    for (cid, arm), r in by_cell.items():
        outcome, hc_miss = grade_record(r, cases[cid])
        graded[(cid, arm)] = {"outcome": outcome, "high_consequence_miss": hc_miss, "tier": r["tier"]}

    flake_hc_misses = [cid for (cid, arm), g in graded.items() if arm == "flake" and g["high_consequence_miss"]]
    if flake_hc_misses:
        return {
            "route": "FAIL: HIGH_CONSEQUENCE_MISS",
            "case_count": case_count,
            "high_consequence_miss_cases": sorted(flake_hc_misses),
        }

    tier_flake_correct = {t: 0 for t in TIERS}
    for (cid, arm), g in graded.items():
        if arm == "flake" and g["outcome"] == "RESUME_CORRECT":
            tier_flake_correct[g["tier"]] += 1
    class_loss_tiers = [t for t in TIERS if tier_flake_correct[t] == 0]
    if class_loss_tiers:
        return {
            "route": "FAIL: CLASS_LOSS",
            "case_count": case_count,
            "class_loss_tiers": class_loss_tiers,
        }

    flake_correct = sum(1 for (cid, arm), g in graded.items() if arm == "flake" and g["outcome"] == "RESUME_CORRECT")
    baseline_correct = sum(1 for (cid, arm), g in graded.items() if arm == "baseline" and g["outcome"] == "RESUME_CORRECT")
    flake_success_rate = flake_correct / case_count
    baseline_success_rate = baseline_correct / case_count

    if flake_success_rate < 0.90:
        return {
            "route": "FAIL: RESUME_SUCCESS_BELOW_90PCT",
            "case_count": case_count,
            "flake_success_rate": flake_success_rate,
        }

    deficit_pp = (baseline_success_rate - flake_success_rate) * 100
    if deficit_pp > 5.0:
        return {
            "route": "FAIL: DEFICIT_EXCEEDS_5PP",
            "case_count": case_count,
            "deficit_pp": deficit_pp,
        }

    tier_breakdown = {
        t: {"flake_correct": tier_flake_correct[t]} for t in TIERS
    }

    return {
        "route": "PASS",
        "case_count": case_count,
        "flake_correct": flake_correct,
        "baseline_correct": baseline_correct,
        "flake_success_rate": flake_success_rate,
        "baseline_success_rate": baseline_success_rate,
        "deficit_pp": deficit_pp,
        "high_consequence_misses": 0,
        "tier_breakdown": tier_breakdown,
    }


def main() -> int:
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--cases", required=True)
    parser.add_argument("--forbid-case-source", default=None)
    args = parser.parse_args()
    result = analyze(Path(args.manifest), Path(args.cases), args.forbid_case_source)
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    import sys
    sys.exit(main())
