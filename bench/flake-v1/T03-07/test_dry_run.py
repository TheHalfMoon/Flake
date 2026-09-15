#!/usr/bin/env python3
"""Tests that the synthetic dry run actually exercises every required
scenario end to end (V05/V09/V15, PROTOCOL.md section 19)."""
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()
sys.path.insert(0, str(HERE))

import dry_run
import analysis
import verify_independent
from harness import RunManifest, DuplicateCellError


def test_generated_records_have_expected_shape():
    records = dry_run.generate_dry_run_records()
    assert len(records) == 96
    conditions = {(r["participant_slot"], r["pair_index"], r["condition"]) for r in records}
    assert len(conditions) == 96, "duplicate cell in generated dry-run records"
    print("PASS: test_generated_records_have_expected_shape")


def test_all_records_are_dev_synthetic():
    records = dry_run.generate_dry_run_records()
    assert all(r["case_source"] == "dev_synthetic" for r in records)
    print("PASS: test_all_records_are_dev_synthetic")


def test_injected_scenarios_present():
    records = dry_run.generate_dry_run_records()
    outcomes = {r["outcome"] for r in records}
    assert "RESUME_TIMEOUT" in outcomes
    exclusions = {r["exclusion"] for r in records if r["exclusion"]}
    assert "FACILITATOR_PROTOCOL_ERROR" in exclusions
    deviations = [r["protocol_deviation"] for r in records if r["protocol_deviation"]]
    assert len(deviations) == 1
    print("PASS: test_injected_scenarios_present")


def test_simulated_interruption_resumes_without_duplication():
    records = dry_run.generate_dry_run_records()
    dry_run.run_collection_with_simulated_interruption(records)
    manifest = RunManifest(dry_run.DRY_RUN_MANIFEST)
    on_disk = manifest.all_records()
    assert len(on_disk) == 96
    cells = {(r["participant_slot"], r["pair_index"], r["condition"]) for r in on_disk}
    assert len(cells) == 96, "duplicate or lost cell after simulated interruption"
    print("PASS: test_simulated_interruption_resumes_without_duplication")


def test_confirmatory_guard_rejects_dev_data():
    dry_run.confirm_dev_guard_raises()  # raises AssertionError internally on failure
    print("PASS: test_confirmatory_guard_rejects_dev_data")


def test_full_dry_run_main_reports_agreement():
    exit_code = dry_run.main()
    assert exit_code == 0, "dry_run.main() reported disagreement between producer and oracle"
    print("PASS: test_full_dry_run_main_reports_agreement")


def run_all_tests():
    tests = [
        test_generated_records_have_expected_shape,
        test_all_records_are_dev_synthetic,
        test_injected_scenarios_present,
        test_simulated_interruption_resumes_without_duplication,
        test_confirmatory_guard_rejects_dev_data,
        test_full_dry_run_main_reports_agreement,
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
