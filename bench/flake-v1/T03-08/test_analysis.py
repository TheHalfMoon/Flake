"""Tests for analysis.py's producer routing, using only synthetic
non-confirmatory fixtures (never cases_confirmatory.json)."""
import json
import tempfile
import unittest
from pathlib import Path

import analysis


def _case(case_id, tier, sources, superseded, conflict, trap_doc_id=None):
    return {
        "case_id": case_id,
        "tier": tier,
        "sources": sources,
        "gold": {
            "correct_next_action": "n/a",
            "superseded_doc_ids": superseded,
            "planted_conflict_doc_ids": conflict,
            "high_consequence_trap": {"trap_doc_id": trap_doc_id, "description": "x"} if trap_doc_id else None,
        },
    }


def _write_cases(tmp: Path, cases: dict) -> Path:
    p = tmp / "cases.json"
    p.write_text(json.dumps({"schema_version": "t", "master_seed": "s", "case_count": len(cases), "cases": cases}), encoding="utf-8")
    return p


def _write_manifest(tmp: Path, records: list) -> Path:
    p = tmp / "records.jsonl"
    with p.open("w", encoding="utf-8") as f:
        for r in records:
            f.write(json.dumps(r) + "\n")
    return p


def _rec(case_id, arm, tier, current, conflict_flagged, error=None, case_source="confirmatory"):
    return {
        "schema_version": "t03-08-run-v1", "case_id": case_id, "tier": tier, "arm": arm,
        "harness_version": "x", "product_version": "x", "start_time": "t", "end_time": "t",
        "setup_seconds": 0.1, "navigation_seconds": 0.01,
        "current_doc_ids": current, "conflict_flagged": conflict_flagged,
        "resume_conflicts_declared": 1 if conflict_flagged else 0,
        "decision_state_outcome": "NeedsReview" if conflict_flagged else "CurrentSet",
        "error": error, "case_source": case_source,
    }


NONCONFLICT_SOURCES = [{"doc_id": "D1", "title": "t", "body": "b1"}, {"doc_id": "D2", "title": "t", "body": "b2"}]
CONFLICT_SOURCES = [{"doc_id": "D1", "title": "t", "body": "b1"}, {"doc_id": "D2", "title": "t", "body": "b2"}, {"doc_id": "D3", "title": "t", "body": "b3"}]


class TestGradeRecord(unittest.TestCase):
    def test_correct_nonconflict(self):
        case = _case("C1", "T1", NONCONFLICT_SOURCES, ["D1"], [])
        rec = _rec("C1", "flake", "T1", ["D2"], False)
        outcome, hc = analysis.grade_record(rec, case)
        self.assertEqual(outcome, "RESUME_CORRECT")
        self.assertFalse(hc)

    def test_wrong_doc_reported(self):
        case = _case("C1", "T1", NONCONFLICT_SOURCES, ["D1"], [])
        rec = _rec("C1", "flake", "T1", ["D1"], False)
        outcome, hc = analysis.grade_record(rec, case)
        self.assertEqual(outcome, "RESUME_FAILED")

    def test_correct_conflict(self):
        case = _case("C2", "T3", CONFLICT_SOURCES, ["D1"], ["D2", "D3"])
        rec = _rec("C2", "flake", "T3", ["D2", "D3"], True)
        outcome, hc = analysis.grade_record(rec, case)
        self.assertEqual(outcome, "RESUME_CORRECT")

    def test_missed_conflict_not_flagged(self):
        case = _case("C2", "T3", CONFLICT_SOURCES, ["D1"], ["D2", "D3"])
        rec = _rec("C2", "flake", "T3", ["D3"], False)
        outcome, hc = analysis.grade_record(rec, case)
        self.assertEqual(outcome, "RESUME_FAILED")

    def test_high_consequence_miss_overrides_otherwise_correct(self):
        case = _case("C3", "T4", CONFLICT_SOURCES, ["D1"], ["D2", "D3"], trap_doc_id="D1")
        rec = _rec("C3", "flake", "T4", ["D1", "D2", "D3"], True)
        outcome, hc = analysis.grade_record(rec, case)
        self.assertTrue(hc)
        self.assertEqual(outcome, "RESUME_FAILED")

    def test_error_is_always_failed(self):
        case = _case("C1", "T1", NONCONFLICT_SOURCES, ["D1"], [])
        rec = _rec("C1", "flake", "T1", [], False, error="CliError: boom")
        outcome, hc = analysis.grade_record(rec, case)
        self.assertEqual(outcome, "RESUME_FAILED")
        self.assertFalse(hc)


class TestAnalyzeRouting(unittest.TestCase):
    def setUp(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.tmp = Path(self.tmpdir.name)

    def tearDown(self):
        self.tmpdir.cleanup()

    def _all_correct_fixture(self, n_per_tier=1):
        cases = {}
        records = []
        for tier in ("T1", "T2", "T3", "T4"):
            for i in range(n_per_tier):
                cid = f"{tier}-{i}"
                conflict = tier in ("T3", "T4")
                if conflict:
                    case = _case(cid, tier, CONFLICT_SOURCES, ["D1"], ["D2", "D3"])
                    current = ["D2", "D3"]
                    flagged = True
                else:
                    case = _case(cid, tier, NONCONFLICT_SOURCES, ["D1"], [])
                    current = ["D2"]
                    flagged = False
                cases[cid] = case
                records.append(_rec(cid, "flake", tier, current, flagged))
                records.append(_rec(cid, "baseline", tier, current, flagged))
        return cases, records

    def test_pass_route(self):
        cases, records = self._all_correct_fixture()
        cases_path = _write_cases(self.tmp, cases)
        manifest_path = _write_manifest(self.tmp, records)
        result = analysis.analyze(manifest_path, cases_path)
        self.assertEqual(result["route"], "PASS")
        self.assertEqual(result["flake_success_rate"], 1.0)

    def test_missing_execution_is_inconclusive(self):
        cases, records = self._all_correct_fixture()
        records.pop()  # drop one baseline record
        cases_path = _write_cases(self.tmp, cases)
        manifest_path = _write_manifest(self.tmp, records)
        result = analysis.analyze(manifest_path, cases_path)
        self.assertEqual(result["route"], "INCONCLUSIVE: MISSING_EXECUTIONS")

    def test_high_consequence_miss_fails_regardless_of_rate(self):
        cases, records = self._all_correct_fixture()
        trap_case_id = "T4-0"
        cases[trap_case_id]["gold"]["high_consequence_trap"] = {"trap_doc_id": "D1", "description": "x"}
        for r in records:
            if r["case_id"] == trap_case_id and r["arm"] == "flake":
                r["current_doc_ids"] = ["D1", "D2", "D3"]
        cases_path = _write_cases(self.tmp, cases)
        manifest_path = _write_manifest(self.tmp, records)
        result = analysis.analyze(manifest_path, cases_path)
        self.assertEqual(result["route"], "FAIL: HIGH_CONSEQUENCE_MISS")

    def test_class_loss_fails(self):
        cases, records = self._all_correct_fixture()
        for r in records:
            if r["tier"] == "T2" and r["arm"] == "flake":
                r["current_doc_ids"] = ["D1"]  # wrong -- forces T2 flake failure
        cases_path = _write_cases(self.tmp, cases)
        manifest_path = _write_manifest(self.tmp, records)
        result = analysis.analyze(manifest_path, cases_path)
        self.assertEqual(result["route"], "FAIL: CLASS_LOSS")
        self.assertIn("T2", result["class_loss_tiers"])

    def test_resume_success_below_90pct(self):
        cases, records = self._all_correct_fixture(n_per_tier=10)
        # Fail more than 10% of flake T1 attempts (keep every tier >=1 correct).
        failed = 0
        for r in records:
            if r["tier"] == "T1" and r["arm"] == "flake" and failed < 5:
                r["current_doc_ids"] = ["D1"]
                failed += 1
        cases_path = _write_cases(self.tmp, cases)
        manifest_path = _write_manifest(self.tmp, records)
        result = analysis.analyze(manifest_path, cases_path)
        self.assertEqual(result["route"], "FAIL: RESUME_SUCCESS_BELOW_90PCT")

    def test_deficit_exceeds_5pp(self):
        cases, records = self._all_correct_fixture(n_per_tier=10)
        # Flake stays >=90% but baseline is made to fail nowhere -- instead
        # push flake down to exactly the boundary and baseline stays 100%.
        failed = 0
        for r in records:
            if r["tier"] == "T1" and r["arm"] == "flake" and failed < 4:
                r["current_doc_ids"] = ["D1"]
                failed += 1
        cases_path = _write_cases(self.tmp, cases)
        manifest_path = _write_manifest(self.tmp, records)
        result = analysis.analyze(manifest_path, cases_path)
        self.assertEqual(result["route"], "FAIL: DEFICIT_EXCEEDS_5PP")

    def test_forbid_case_source_guard_raises(self):
        cases, records = self._all_correct_fixture()
        records[0]["case_source"] = "dev_synthetic"
        cases_path = _write_cases(self.tmp, cases)
        manifest_path = _write_manifest(self.tmp, records)
        with self.assertRaises(analysis.AnalysisError):
            analysis.analyze(manifest_path, cases_path, forbid_case_source="dev_synthetic")


if __name__ == "__main__":
    unittest.main()
