#!/usr/bin/env python3
"""Tests for the preregistered analysis/decision routing (V05/V15)."""
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()
sys.path.insert(0, str(HERE))

from harness import PHASE_FIELDS
import analysis


def _record(participant, pair, condition, case_id, outcome="RESUME_CORRECT",
            exclusion=None, high_consequence_miss=False, total_seconds=100.0,
            case_source="confirmatory"):
    per_field = total_seconds / len(PHASE_FIELDS)
    return {
        "participant_slot": participant,
        "pair_index": pair,
        "condition": condition,
        "condition_order": "first",
        "case_id": case_id,
        "case_source": case_source,
        "outcome": outcome,
        "exclusion": exclusion,
        "high_consequence_miss": high_consequence_miss,
        "phase_durations": {f: per_field for f in PHASE_FIELDS},
    }


def _all_correct_pairs(n_pairs, tier="T1", flake_seconds=60.0, baseline_seconds=100.0):
    records = []
    case_tiers = {}
    for i in range(1, n_pairs + 1):
        flake_case = f"C-P{i}-{i:02d}-A"
        baseline_case = f"C-P{i}-{i:02d}-B"
        case_tiers[flake_case] = tier
        case_tiers[baseline_case] = tier
        records.append(_record(f"P{i}", i, "flake", flake_case, total_seconds=flake_seconds))
        records.append(_record(f"P{i}", i, "baseline", baseline_case, total_seconds=baseline_seconds))
    return records, case_tiers


def test_pass_route_on_strong_flake_result():
    records, tiers = _all_correct_pairs(48, flake_seconds=60.0, baseline_seconds=100.0)
    result = analysis.analyze(records, expected_pairs=48, min_valid_pairs=44, case_tiers=tiers)
    assert result["route"] == "PASS", result
    print("PASS: test_pass_route_on_strong_flake_result")


def test_inconclusive_on_insufficient_valid_pairs():
    records, tiers = _all_correct_pairs(48)
    # Exclude enough pairs to drop below the 44/48 threshold: mark 10 as excluded.
    for r in records:
        if r["pair_index"] <= 10:
            r["exclusion"] = "TOOLING_CRASH_UNRELATED"
    result = analysis.analyze(records, expected_pairs=48, min_valid_pairs=44, case_tiers=tiers)
    assert result["route"] == "INCONCLUSIVE: INSUFFICIENT_VALID_PAIRS", result
    print("PASS: test_inconclusive_on_insufficient_valid_pairs")


def test_fail_on_high_consequence_miss():
    records, tiers = _all_correct_pairs(48)
    records[0]["high_consequence_miss"] = True
    result = analysis.analyze(records, expected_pairs=48, min_valid_pairs=44, case_tiers=tiers)
    assert result["route"] == "FAIL: HIGH_CONSEQUENCE_MISS", result
    print("PASS: test_fail_on_high_consequence_miss")


def test_fail_on_class_loss():
    records, tiers = _all_correct_pairs(48)
    # Force every flake attempt in one participant's case to a different,
    # dedicated tier with zero correct outcomes anywhere for that tier.
    tiers["C-P1-01-A"] = "T9"
    for r in records:
        if r["case_id"] == "C-P1-01-A":
            r["outcome"] = "RESUME_FAILED"
    result = analysis.analyze(records, expected_pairs=48, min_valid_pairs=44, case_tiers=tiers)
    assert result["route"].startswith("FAIL: CLASS_LOSS"), result
    print("PASS: test_fail_on_class_loss")


def test_fail_on_resume_success_below_90pct():
    records, tiers = _all_correct_pairs(48)
    # Fail ~15% of flake attempts (>10%, pushing success below 90%).
    failed_count = 0
    for r in records:
        if r["condition"] == "flake" and failed_count < 8:
            r["outcome"] = "RESUME_FAILED"
            failed_count += 1
    result = analysis.analyze(records, expected_pairs=48, min_valid_pairs=44, case_tiers=tiers)
    assert result["route"] == "FAIL: RESUME_SUCCESS_BELOW_90PCT", result
    print("PASS: test_fail_on_resume_success_below_90pct")


def test_fail_on_deficit_exceeds_5pp():
    # Construction: 40 pairs, flake fails exactly 4 of 40 (90.0%, right at
    # the floor so this test isolates the deficit gate from the success
    # gate), baseline stays at 100% -> deficit = 10pp > 5pp threshold.
    records, tiers = _all_correct_pairs(40, flake_seconds=60.0, baseline_seconds=100.0)
    flake_records = [r for r in records if r["condition"] == "flake"]
    for r in flake_records[:4]:
        r["outcome"] = "RESUME_FAILED"
    result = analysis.analyze(records, expected_pairs=40, min_valid_pairs=37, case_tiers=tiers)
    assert result["flake_success_rate"] >= 0.90, result
    assert result["route"] == "FAIL: DEFICIT_EXCEEDS_5PP", result
    print("PASS: test_fail_on_deficit_exceeds_5pp")


def test_fail_on_time_reduction_below_20pct():
    records, tiers = _all_correct_pairs(48, flake_seconds=95.0, baseline_seconds=100.0)
    result = analysis.analyze(records, expected_pairs=48, min_valid_pairs=44, case_tiers=tiers)
    assert result["route"] == "FAIL: TIME_REDUCTION_BELOW_20PCT", result
    print("PASS: test_fail_on_time_reduction_below_20pct")


def test_dev_synthetic_forbidden_in_confirmatory_route():
    records, tiers = _all_correct_pairs(1)
    records[0]["case_source"] = "dev_synthetic"
    try:
        analysis.analyze(records, expected_pairs=1, min_valid_pairs=1, case_tiers=tiers,
                          forbid_case_source="dev_synthetic")
        raise AssertionError("expected AnalysisError")
    except analysis.AnalysisError:
        pass
    print("PASS: test_dev_synthetic_forbidden_in_confirmatory_route")


def test_decision_routing_order_safety_before_time():
    # A case with both a high-consequence miss AND a time-gate failure
    # must route on the safety gate first (section 20's fixed step order).
    records, tiers = _all_correct_pairs(48, flake_seconds=95.0, baseline_seconds=100.0)
    records[0]["high_consequence_miss"] = True
    result = analysis.analyze(records, expected_pairs=48, min_valid_pairs=44, case_tiers=tiers)
    assert result["route"] == "FAIL: HIGH_CONSEQUENCE_MISS", result
    print("PASS: test_decision_routing_order_safety_before_time")


def run_all_tests():
    tests = [
        test_pass_route_on_strong_flake_result,
        test_inconclusive_on_insufficient_valid_pairs,
        test_fail_on_high_consequence_miss,
        test_fail_on_class_loss,
        test_fail_on_resume_success_below_90pct,
        test_fail_on_deficit_exceeds_5pp,
        test_fail_on_time_reduction_below_20pct,
        test_dev_synthetic_forbidden_in_confirmatory_route,
        test_decision_routing_order_safety_before_time,
    ]
    passed = 0
    failed = 0
    errors = []
    for test in tests:
        try:
            test()
            passed += 1
        except AssertionError as e:
            failed += 1
            errors.append(f"FAIL: {test.__name__}: {e}")
        except Exception as e:
            failed += 1
            errors.append(f"ERROR: {test.__name__}: {e}")
    print(f"\n{'=' * 60}\nResults: {passed} passed, {failed} failed\n{'=' * 60}")
    if errors:
        for e in errors:
            print(e)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(run_all_tests())
