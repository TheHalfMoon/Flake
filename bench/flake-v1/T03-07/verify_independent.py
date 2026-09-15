#!/usr/bin/env python3
"""Independent arithmetic verification for the T03-07 local continuity proof.

This is a separately written "oracle" for `analysis.py`'s "producer" role
(PROTOCOL.md section 20 / plan section 28's "avoid using the same
implementation as both producer and oracle"). It deliberately:

  - reads the raw .jsonl manifest itself, line by line, rather than
    importing harness.RunManifest;
  - never imports analysis.py or any of its functions or constants;
  - computes the median by sorting and indexing by hand, not via the
    statistics module;
  - groups pairs with a plain dict keyed by a string, not a tuple, and
    walks the routing steps in an independently structured control flow.

If this file and analysis.py ever disagree on a route for the same raw
manifest, that disagreement is itself the finding -- the run is not
reportable until it is root-caused (a defect in one of the two
implementations) and fixed, per the repair-repeat rule in PROTOCOL.md
section 15 if sealed code must change.
"""
import json
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()

CONFIRMATORY_EXPECTED_PAIRS = 48
CONFIRMATORY_MIN_VALID_PAIRS = 44
FLAKE_SUCCESS_MIN = 0.90
DEFICIT_MAX_PP = 5.0
TIME_REDUCTION_MIN = 0.20


class IndependentVerificationError(Exception):
    pass


def read_jsonl(path: Path) -> list:
    records = []
    if not path.exists():
        return records
    with path.open("r", encoding="utf-8") as handle:
        for raw_line in handle:
            stripped = raw_line.strip()
            if stripped:
                records.append(json.loads(stripped))
    return records


def load_case_tiers(cases_path: Path) -> dict:
    with cases_path.open("r", encoding="utf-8") as handle:
        raw = json.load(handle)
    tiers = {}
    for case_id, case in raw["cases"].items():
        tiers[case_id] = case["tier"]
    return tiers


def phase_sum(record: dict) -> float:
    d = record["phase_durations"]
    total = 0.0
    for key in ("setup_seconds", "maintenance_seconds", "navigation_seconds",
                "recovery_seconds", "answer_seconds"):
        total += d[key]
    return total


def median_of(values: list) -> float:
    ordered = sorted(values)
    n = len(ordered)
    if n == 0:
        raise IndependentVerificationError("median of empty list")
    mid = n // 2
    if n % 2 == 1:
        return ordered[mid]
    return (ordered[mid - 1] + ordered[mid]) / 2.0


def verify(records: list, expected_pairs: int, min_valid_pairs: int,
           case_tiers: dict, forbid_case_source: str = None) -> dict:
    if forbid_case_source is not None:
        for r in records:
            if r.get("case_source") == forbid_case_source:
                raise IndependentVerificationError(
                    f"record for case {r.get('case_id')} carries forbidden "
                    f"case_source {forbid_case_source!r}"
                )

    pair_index_by_key = {}
    for r in records:
        key = f"{r['participant_slot']}::{r['pair_index']}"
        slot = pair_index_by_key.setdefault(key, {})
        slot[r["condition"]] = r

    valid_pair_keys = []
    for key, slot in pair_index_by_key.items():
        if "flake" in slot and "baseline" in slot:
            if slot["flake"]["exclusion"] is None and slot["baseline"]["exclusion"] is None:
                valid_pair_keys.append(key)

    out = {
        "expected_pairs": expected_pairs,
        "observed_pair_cells": len(pair_index_by_key),
        "valid_pairs": len(valid_pair_keys),
        "min_valid_pairs_required": min_valid_pairs,
    }

    if len(valid_pair_keys) < min_valid_pairs:
        out["route"] = "INCONCLUSIVE: INSUFFICIENT_VALID_PAIRS"
        return out

    valid_flake = []
    valid_baseline = []
    high_consequence_count = 0
    for key in valid_pair_keys:
        slot = pair_index_by_key[key]
        for cond in ("flake", "baseline"):
            rec = slot[cond]
            if rec["high_consequence_miss"]:
                high_consequence_count += 1
            if cond == "flake":
                valid_flake.append(rec)
            else:
                valid_baseline.append(rec)

    out["high_consequence_miss_count"] = high_consequence_count
    if high_consequence_count > 0:
        out["route"] = "FAIL: HIGH_CONSEQUENCE_MISS"
        return out

    tier_totals = {}
    tier_correct = {}
    for rec in valid_flake:
        tier = case_tiers[rec["case_id"]]
        tier_totals[tier] = tier_totals.get(tier, 0) + 1
        if rec["outcome"] == "RESUME_CORRECT":
            tier_correct[tier] = tier_correct.get(tier, 0) + 1

    lost = []
    for tier in tier_totals:
        if tier_correct.get(tier, 0) == 0:
            lost.append(tier)
    out["tier_breakdown"] = {
        t: {"total": tier_totals[t], "correct": tier_correct.get(t, 0)} for t in tier_totals
    }
    if lost:
        out["route"] = f"FAIL: CLASS_LOSS ({sorted(lost)})"
        return out

    flake_correct = sum(1 for rec in valid_flake if rec["outcome"] == "RESUME_CORRECT")
    baseline_correct = sum(1 for rec in valid_baseline if rec["outcome"] == "RESUME_CORRECT")
    flake_rate = flake_correct / len(valid_flake)
    baseline_rate = baseline_correct / len(valid_baseline)
    deficit_pp = (baseline_rate - flake_rate) * 100.0

    out["flake_success_rate"] = flake_rate
    out["baseline_success_rate"] = baseline_rate
    out["deficit_pp"] = deficit_pp

    if flake_rate < FLAKE_SUCCESS_MIN:
        out["route"] = "FAIL: RESUME_SUCCESS_BELOW_90PCT"
        return out
    if deficit_pp > DEFICIT_MAX_PP:
        out["route"] = "FAIL: DEFICIT_EXCEEDS_5PP"
        return out

    flake_pair_totals = []
    baseline_pair_totals = []
    for key in valid_pair_keys:
        slot = pair_index_by_key[key]
        flake_pair_totals.append(phase_sum(slot["flake"]))
        baseline_pair_totals.append(phase_sum(slot["baseline"]))

    med_flake = median_of(flake_pair_totals)
    med_baseline = median_of(baseline_pair_totals)
    out["median_flake_total_seconds"] = med_flake
    out["median_baseline_total_seconds"] = med_baseline
    threshold = med_baseline * (1.0 - TIME_REDUCTION_MIN)
    out["time_threshold_seconds"] = threshold

    if med_flake > threshold:
        out["route"] = "FAIL: TIME_REDUCTION_BELOW_20PCT"
        return out

    out["route"] = "PASS"
    return out


def verify_confirmatory(manifest_path: Path, cases_path: Path) -> dict:
    records = read_jsonl(manifest_path)
    case_tiers = load_case_tiers(cases_path)
    return verify(
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
        print(f"no confirmatory manifest at {manifest_path} -- nothing to verify")
        return 0
    result = verify_confirmatory(manifest_path, cases_path)
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
