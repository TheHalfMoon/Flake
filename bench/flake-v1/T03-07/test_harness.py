#!/usr/bin/env python3
"""Tests for the append-only, resumable raw run manifest (V03/V07)."""
import shutil
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).parent.resolve()
sys.path.insert(0, str(HERE))

from harness import RunManifest, DuplicateCellError, ManifestError, validate_record, PHASE_FIELDS


def _sample_record(**overrides):
    record = {
        "schema_version": "t03-07-run-v1",
        "participant_slot": "P1",
        "pair_index": 1,
        "condition": "flake",
        "condition_order": "first",
        "case_id": "C-P1-01-A",
        "case_source": "confirmatory",
        "attempt_index": 1,
        "harness_version": "test-fixture",
        "product_version": "test-fixture",
        "previous_state": "state-hash-abc",
        "start_time": "2026-01-01T00:00:00Z",
        "end_time": "2026-01-01T00:05:00Z",
        "phase_durations": {f: 1.0 for f in PHASE_FIELDS},
        "outcome": "RESUME_CORRECT",
        "high_consequence_miss": False,
        "exclusion": None,
        "protocol_deviation": None,
    }
    record.update(overrides)
    return record


def test_valid_record_has_no_errors():
    errors = validate_record(_sample_record())
    assert errors == [], errors
    print("PASS: test_valid_record_has_no_errors")


def test_missing_field_detected():
    record = _sample_record()
    del record["outcome"]
    errors = validate_record(record)
    assert any("outcome" in e for e in errors), errors
    print("PASS: test_missing_field_detected")


def test_invalid_condition_detected():
    errors = validate_record(_sample_record(condition="both"))
    assert any("condition" in e for e in errors), errors
    print("PASS: test_invalid_condition_detected")


def test_invalid_exclusion_code_detected():
    errors = validate_record(_sample_record(exclusion="MADE_UP_REASON"))
    assert any("exclusion" in e for e in errors), errors
    print("PASS: test_invalid_exclusion_code_detected")


def test_negative_duration_detected():
    durations = {f: 1.0 for f in PHASE_FIELDS}
    durations["setup_seconds"] = -1.0
    errors = validate_record(_sample_record(phase_durations=durations))
    assert any("setup_seconds" in e for e in errors), errors
    print("PASS: test_negative_duration_detected")


def test_append_and_read_back():
    tmpdir = Path(tempfile.mkdtemp())
    try:
        manifest = RunManifest(tmpdir / "records.jsonl")
        manifest.append(_sample_record())
        records = manifest.all_records()
        assert len(records) == 1
        assert records[0]["case_id"] == "C-P1-01-A"
        print("PASS: test_append_and_read_back")
    finally:
        shutil.rmtree(tmpdir)


def test_duplicate_cell_refused():
    tmpdir = Path(tempfile.mkdtemp())
    try:
        manifest = RunManifest(tmpdir / "records.jsonl")
        manifest.append(_sample_record())
        try:
            manifest.append(_sample_record(attempt_index=2))
            raise AssertionError("expected DuplicateCellError")
        except DuplicateCellError:
            pass
        print("PASS: test_duplicate_cell_refused")
    finally:
        shutil.rmtree(tmpdir)


def test_invalid_record_refused_before_write():
    tmpdir = Path(tempfile.mkdtemp())
    try:
        manifest = RunManifest(tmpdir / "records.jsonl")
        bad = _sample_record()
        del bad["outcome"]
        try:
            manifest.append(bad)
            raise AssertionError("expected ManifestError")
        except ManifestError:
            pass
        assert not (tmpdir / "records.jsonl").exists() or manifest.all_records() == []
        print("PASS: test_invalid_record_refused_before_write")
    finally:
        shutil.rmtree(tmpdir)


def test_resume_after_simulated_restart():
    tmpdir = Path(tempfile.mkdtemp())
    try:
        path = tmpdir / "records.jsonl"
        manifest_a = RunManifest(path)
        manifest_a.append(_sample_record(pair_index=1, condition="flake"))

        # Simulated process restart: brand-new object, no shared memory.
        manifest_b = RunManifest(path)
        expected = {("P1", 1, "flake"), ("P1", 1, "baseline")}
        missing = manifest_b.missing_cells(expected)
        assert missing == {("P1", 1, "baseline")}, missing

        manifest_b.append(_sample_record(pair_index=1, condition="baseline", attempt_index=2))
        assert manifest_b.is_complete(expected)
        print("PASS: test_resume_after_simulated_restart")
    finally:
        shutil.rmtree(tmpdir)


def test_corrupt_line_raises_manifest_error():
    tmpdir = Path(tempfile.mkdtemp())
    try:
        path = tmpdir / "records.jsonl"
        path.write_text("{not valid json\n", encoding="utf-8")
        manifest = RunManifest(path)
        try:
            manifest.all_records()
            raise AssertionError("expected ManifestError for corrupt line")
        except ManifestError:
            pass
        print("PASS: test_corrupt_line_raises_manifest_error")
    finally:
        shutil.rmtree(tmpdir)


def run_all_tests():
    tests = [
        test_valid_record_has_no_errors,
        test_missing_field_detected,
        test_invalid_condition_detected,
        test_invalid_exclusion_code_detected,
        test_negative_duration_detected,
        test_append_and_read_back,
        test_duplicate_cell_refused,
        test_invalid_record_refused_before_write,
        test_resume_after_simulated_restart,
        test_corrupt_line_raises_manifest_error,
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
