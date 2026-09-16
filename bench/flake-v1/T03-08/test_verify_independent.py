"""Cross-checks that verify_independent.py (the oracle) and analysis.py (the
producer) agree exactly on every route, using the same fixture builder as
test_analysis.py (duplicated deliberately -- these two test files must stay
independently readable, matching T03-07's own precedent)."""
import json
import tempfile
import unittest
from pathlib import Path

import analysis
import verify_independent


def _case(case_id, tier, sources, superseded, conflict, trap_doc_id=None):
    return {
        "case_id": case_id, "tier": tier, "sources": sources,
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


def _fixture(mutate=None, trap=False):
    cases, records = {}, []
    for tier in ("T1", "T2", "T3", "T4"):
        cid = f"{tier}-0"
        conflict = tier in ("T3", "T4")
        trap_doc_id = "D1" if (trap and tier == "T4") else None
        if conflict:
            case = _case(cid, tier, CONFLICT_SOURCES, ["D1"], ["D2", "D3"], trap_doc_id)
            current, flagged = ["D2", "D3"], True
        else:
            case = _case(cid, tier, NONCONFLICT_SOURCES, ["D1"], [])
            current, flagged = ["D2"], False
        cases[cid] = case
        records.append(_rec(cid, "flake", tier, current, flagged))
        records.append(_rec(cid, "baseline", tier, current, flagged))
    if mutate:
        mutate(cases, records)
    return cases, records


class TestAgreement(unittest.TestCase):
    def setUp(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.tmp = Path(self.tmpdir.name)

    def tearDown(self):
        self.tmpdir.cleanup()

    def _assert_agree(self, cases, records):
        cases_path = _write_cases(self.tmp, cases)
        manifest_path = _write_manifest(self.tmp, records)
        a = analysis.analyze(manifest_path, cases_path)
        b = verify_independent.verify(manifest_path, cases_path)
        self.assertEqual(a["route"], b["route"])
        return a, b

    def test_agree_on_pass(self):
        cases, records = _fixture()
        a, b = self._assert_agree(cases, records)
        self.assertEqual(a["route"], "PASS")
        self.assertAlmostEqual(a["flake_success_rate"], b["flake_success_rate"])
        self.assertAlmostEqual(a["baseline_success_rate"], b["baseline_success_rate"])
        self.assertAlmostEqual(a["deficit_pp"], b["deficit_pp"])

    def test_agree_on_missing(self):
        def mutate(cases, records):
            records.pop()
        cases, records = _fixture(mutate)
        self._assert_agree(cases, records)

    def test_agree_on_high_consequence_miss(self):
        def mutate(cases, records):
            for r in records:
                if r["case_id"] == "T4-0" and r["arm"] == "flake":
                    r["current_doc_ids"] = ["D1", "D2", "D3"]
        cases, records = _fixture(mutate, trap=True)
        a, b = self._assert_agree(cases, records)
        self.assertEqual(a["route"], "FAIL: HIGH_CONSEQUENCE_MISS")

    def test_agree_on_class_loss(self):
        def mutate(cases, records):
            for r in records:
                if r["tier"] == "T3" and r["arm"] == "flake":
                    r["current_doc_ids"] = ["D1"]
        cases, records = _fixture(mutate)
        a, b = self._assert_agree(cases, records)
        self.assertEqual(a["route"], "FAIL: CLASS_LOSS")

    def test_agree_on_forbidden_case_source(self):
        cases, records = _fixture()
        records[0]["case_source"] = "dev_synthetic"
        cases_path = _write_cases(self.tmp, cases)
        manifest_path = _write_manifest(self.tmp, records)
        with self.assertRaises(analysis.AnalysisError):
            analysis.analyze(manifest_path, cases_path, forbid_case_source="dev_synthetic")
        with self.assertRaises(verify_independent.VerificationError):
            verify_independent.verify(manifest_path, cases_path, forbid_case_source="dev_synthetic")


if __name__ == "__main__":
    unittest.main()
