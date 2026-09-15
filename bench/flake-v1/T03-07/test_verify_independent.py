#!/usr/bin/env python3
"""Cross-checks that verify_independent.py (the independently written
oracle) agrees with analysis.py (the producer) on every routing branch,
and unit-tests the oracle's own from-scratch primitives (V15)."""
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()
sys.path.insert(0, str(HERE))

import analysis
import verify_independent
from test_analysis import _all_correct_pairs  # reuse fixture builder only, not analysis logic


def test_median_of_matches_known_values():
    assert verify_independent.median_of([1, 2, 3]) == 2
    assert verify_independent.median_of([1, 2, 3, 4]) == 2.5
    assert verify_independent.median_of([5]) == 5
    print("PASS: test_median_of_matches_known_values")


def _agree(records, expected_pairs, min_valid_pairs, tiers, forbid=None):
    a = analysis.analyze(records, expected_pairs, min_valid_pairs, tiers, forbid_case_source=forbid)
    b = verify_independent.verify(records, expected_pairs, min_valid_pairs, tiers, forbid_case_source=forbid)
    assert a["route"] == b["route"], f"disagreement: analysis={a['route']!r} verify_independent={b['route']!r}"
    return a, b


def test_agreement_on_pass():
    records, tiers = _all_correct_pairs(48, flake_seconds=60.0, baseline_seconds=100.0)
    a, b = _agree(records, 48, 44, tiers)
    assert a["route"] == "PASS"
    print("PASS: test_agreement_on_pass")


def test_agreement_on_inconclusive():
    records, tiers = _all_correct_pairs(48)
    for r in records:
        if r["pair_index"] <= 10:
            r["exclusion"] = "TOOLING_CRASH_UNRELATED"
    a, b = _agree(records, 48, 44, tiers)
    assert a["route"] == "INCONCLUSIVE: INSUFFICIENT_VALID_PAIRS"
    print("PASS: test_agreement_on_inconclusive")


def test_agreement_on_high_consequence_fail():
    records, tiers = _all_correct_pairs(48)
    records[0]["high_consequence_miss"] = True
    a, b = _agree(records, 48, 44, tiers)
    assert a["route"] == "FAIL: HIGH_CONSEQUENCE_MISS"
    print("PASS: test_agreement_on_high_consequence_fail")


def test_agreement_on_time_gate():
    records, tiers = _all_correct_pairs(48, flake_seconds=95.0, baseline_seconds=100.0)
    a, b = _agree(records, 48, 44, tiers)
    assert a["route"] == "FAIL: TIME_REDUCTION_BELOW_20PCT"
    assert abs(a["median_flake_total_seconds"] - b["median_flake_total_seconds"]) < 1e-9
    assert abs(a["median_baseline_total_seconds"] - b["median_baseline_total_seconds"]) < 1e-9
    print("PASS: test_agreement_on_time_gate")


def test_agreement_numeric_fields_match_exactly():
    records, tiers = _all_correct_pairs(48, flake_seconds=60.0, baseline_seconds=100.0)
    a, b = _agree(records, 48, 44, tiers)
    for field in ("flake_success_rate", "baseline_success_rate", "deficit_pp",
                  "median_flake_total_seconds", "median_baseline_total_seconds"):
        assert abs(a[field] - b[field]) < 1e-9, (field, a[field], b[field])
    print("PASS: test_agreement_numeric_fields_match_exactly")


def test_oracle_rejects_forbidden_case_source_independently():
    records, tiers = _all_correct_pairs(1)
    records[0]["case_source"] = "dev_synthetic"
    try:
        verify_independent.verify(records, 1, 1, tiers, forbid_case_source="dev_synthetic")
        raise AssertionError("expected IndependentVerificationError")
    except verify_independent.IndependentVerificationError:
        pass
    print("PASS: test_oracle_rejects_forbidden_case_source_independently")


def run_all_tests():
    tests = [
        test_median_of_matches_known_values,
        test_agreement_on_pass,
        test_agreement_on_inconclusive,
        test_agreement_on_high_consequence_fail,
        test_agreement_on_time_gate,
        test_agreement_numeric_fields_match_exactly,
        test_oracle_rejects_forbidden_case_source_independently,
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
