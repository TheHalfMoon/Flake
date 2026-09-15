#!/usr/bin/env python3
"""Preregistered analysis and decision routing for the T03-07 local
continuity proof (PROTOCOL.md section 20). Implemented and sealed before
any confirmatory human observation exists.

This module is the "producer." `verify_independent.py` is a separately
written "oracle" that recomputes the same route from the same raw
manifest via a different code path -- see that file for the independence
discipline.
"""
import json
import statistics
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()
sys.path.insert(0, str(HERE))

from harness import RunManifest

CONFIRMATORY_EXPECTED_PAIRS = 48
CONFIRMATORY_MIN_VALID_PAIRS = 44  # 44/48 = 91.67%, section 20 step 1
FLAKE_SUCCESS_MIN = 0.90
DEFICIT_MAX_PP = 5.0
TIME_REDUCTION_MIN = 0.20  # median_flake_total must be <= 0.80 * median_baseline_total


class AnalysisError(Exception):
    pass


def _load_case_tiers(cases_path: Path) -> dict:
    data = json.loads(cases_path.read_text(encoding="utf-8"))
    return {case_id: case["tier"] for case_id, case in data["cases"].items()}


def _pair_key(record: dict):
    return (record["participant_slot"], record["pair_index"])


def _phase_total(record: dict) -> float:
    d = record["phase_durations"]
    return sum(d[field] for field in
               ("setup_seconds", "maintenance_seconds", "navigation_seconds",
                "recovery_seconds", "answer_seconds"))


def _group_pairs(records: list) -> dict:
    pairs = {}
    for r in records:
        key = _pair_key(r)
        pairs.setdefault(key, {})[r["condition"]] = r
    return pairs


def _pair_is_valid(pair: dict) -> bool:
    if "flake" not in pair or "baseline" not in pair:
        return False
    return pair["flake"]["exclusion"] is None and pair["baseline"]["exclusion"] is None


def analyze(records: list, expected_pairs: int, min_valid_pairs: int,
            case_tiers: dict, forbid_case_source: str = None) -> dict:
    """Generic analysis over a set of raw run records. Used directly by
    the confirmatory wrapper below, and reused (with a smaller shape) by
    dry_run.py so the same routing logic is exercised in both places.
    """
    if forbid_case_source is not None:
        for r in records:
            if r.get("case_source") == forbid_case_source:
                raise AnalysisError(
                    f"record for case {r.get('case_id')} has case_source="
                    f"{forbid_case_source!r}; this data source must never "
                    "contribute to a confirmatory Pass/Fail/Inconclusive route "
                    "(PROTOCOL.md section 14.2)"
                )

    pairs = _group_pairs(records)
    valid_pairs = {k: v for k, v in pairs.items() if _pair_is_valid(v)}

    result = {
        "expected_pairs": expected_pairs,
        "observed_pair_cells": len(pairs),
        "valid_pairs": len(valid_pairs),
        "min_valid_pairs_required": min_valid_pairs,
    }

    # Step 1: data-completeness check.
    if len(valid_pairs) < min_valid_pairs:
        result["route"] = "INCONCLUSIVE: INSUFFICIENT_VALID_PAIRS"
        return result

    valid_attempts = [r for pair in valid_pairs.values() for r in pair.values()]

    # Step 2: safety check.
    high_consequence_misses = [r for r in valid_attempts if r["high_consequence_miss"]]
    result["high_consequence_miss_count"] = len(high_consequence_misses)
    if high_consequence_misses:
        result["route"] = "FAIL: HIGH_CONSEQUENCE_MISS"
        return result

    # Step 3: class-loss check (per difficulty tier, Flake attempts only).
    flake_attempts = [r for r in valid_attempts if r["condition"] == "flake"]
    tiers_seen = {}
    for r in flake_attempts:
        tier = case_tiers[r["case_id"]]
        tiers_seen.setdefault(tier, {"total": 0, "correct": 0})
        tiers_seen[tier]["total"] += 1
        if r["outcome"] == "RESUME_CORRECT":
            tiers_seen[tier]["correct"] += 1
    lost_tiers = [t for t, c in tiers_seen.items() if c["correct"] == 0]
    result["tier_breakdown"] = tiers_seen
    if lost_tiers:
        result["route"] = f"FAIL: CLASS_LOSS ({sorted(lost_tiers)})"
        return result

    # Step 4/5: resume-success and relative-deficit gates.
    baseline_attempts = [r for r in valid_attempts if r["condition"] == "baseline"]
    flake_success_rate = (sum(1 for r in flake_attempts if r["outcome"] == "RESUME_CORRECT")
                           / len(flake_attempts))
    baseline_success_rate = (sum(1 for r in baseline_attempts if r["outcome"] == "RESUME_CORRECT")
                              / len(baseline_attempts))
    deficit_pp = (baseline_success_rate - flake_success_rate) * 100.0
    result["flake_success_rate"] = flake_success_rate
    result["baseline_success_rate"] = baseline_success_rate
    result["deficit_pp"] = deficit_pp

    if flake_success_rate < FLAKE_SUCCESS_MIN:
        result["route"] = "FAIL: RESUME_SUCCESS_BELOW_90PCT"
        return result
    if deficit_pp > DEFICIT_MAX_PP:
        result["route"] = "FAIL: DEFICIT_EXCEEDS_5PP"
        return result

    # Step 6: time gate, computed per-pair then medianed.
    flake_totals = []
    baseline_totals = []
    for pair in valid_pairs.values():
        flake_totals.append(_phase_total(pair["flake"]))
        baseline_totals.append(_phase_total(pair["baseline"]))
    median_flake_total = statistics.median(flake_totals)
    median_baseline_total = statistics.median(baseline_totals)
    result["median_flake_total_seconds"] = median_flake_total
    result["median_baseline_total_seconds"] = median_baseline_total
    threshold = (1.0 - TIME_REDUCTION_MIN) * median_baseline_total
    result["time_threshold_seconds"] = threshold

    if median_flake_total > threshold:
        result["route"] = "FAIL: TIME_REDUCTION_BELOW_20PCT"
        return result

    result["route"] = "PASS"
    return result


def analyze_confirmatory(manifest_path: Path, cases_path: Path) -> dict:
    manifest = RunManifest(manifest_path)
    records = manifest.all_records()
    case_tiers = _load_case_tiers(cases_path)
    return analyze(
        records,
        expected_pairs=CONFIRMATORY_EXPECTED_PAIRS,
        min_valid_pairs=CONFIRMATORY_MIN_VALID_PAIRS,
        case_tiers=case_tiers,
        forbid_case_source="dev_synthetic",
    )


def main() -> int:
    manifest_path = HERE / "runs" / "confirmatory" / "records.jsonl"
    cases_path = HERE / "cases_confirmatory.json"
    if not manifest_path.exists():
        print(f"no confirmatory manifest at {manifest_path} -- nothing to analyze "
              "(no confirmatory attempt has been recorded)")
        return 0
    result = analyze_confirmatory(manifest_path, cases_path)
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
