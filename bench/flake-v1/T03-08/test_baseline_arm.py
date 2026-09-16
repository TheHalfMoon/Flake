"""Tests for baseline_arm.py: the independent maintained-Markdown arm.
Pure-Python, no Flake CLI/binary involved."""
import tempfile
import unittest
from pathlib import Path

import baseline_arm

NONCONFLICT_CASE = {
    "case_id": "C-T1-01",
    "tier": "T1",
    "sources": [
        {"doc_id": "D1", "title": "a", "body": "vendor-101 status: confirmed"},
        {"doc_id": "D2", "title": "b", "body": "vendor-101 status: rescheduled"},
        {"doc_id": "D3", "title": "c", "body": "vendor-101 status: cancelled -- superseded note"},
    ],
}

CONFLICT_CASE = {
    "case_id": "C-T3-01",
    "tier": "T3",
    "sources": [
        {"doc_id": "D1", "title": "a", "body": "b1"},
        {"doc_id": "D2", "title": "b", "body": "b2"},
        {"doc_id": "D3", "title": "c", "body": "b3"},
        {"doc_id": "D4", "title": "d", "body": "b4 conflicting status one"},
        {"doc_id": "D5", "title": "e", "body": "b5 conflicting status two"},
    ],
}


class TestBaselineArm(unittest.TestCase):
    def setUp(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.tmp = Path(self.tmpdir.name)

    def tearDown(self):
        self.tmpdir.cleanup()

    def test_nonconflict_last_doc_is_current(self):
        case_file, elapsed = baseline_arm.setup_case(self.tmp, NONCONFLICT_CASE)
        self.assertGreaterEqual(elapsed, 0.0)
        facts = baseline_arm.resume_case(case_file)
        self.assertEqual(facts["current_doc_ids"], ["D3"])
        self.assertFalse(facts["conflict_flagged"])

    def test_conflict_pair_both_current_and_flagged(self):
        case_file, _ = baseline_arm.setup_case(self.tmp, CONFLICT_CASE)
        facts = baseline_arm.resume_case(case_file)
        self.assertEqual(facts["current_doc_ids"], ["D4", "D5"])
        self.assertTrue(facts["conflict_flagged"])

    def test_resume_is_a_fresh_read_not_shared_state(self):
        """Resume must re-derive its answer purely from the file on disk --
        confirmed by mutating the file after setup and observing the change
        is picked up (proves no cached in-memory state is used)."""
        case_file, _ = baseline_arm.setup_case(self.tmp, NONCONFLICT_CASE)
        text = case_file.read_text(encoding="utf-8")
        mutated = text.replace("[CURRENT] D3", "[SUPERSEDED] D3").replace(
            "[SUPERSEDED] D2", "[CURRENT] D2"
        )
        case_file.write_text(mutated, encoding="utf-8")
        facts = baseline_arm.resume_case(case_file)
        self.assertEqual(facts["current_doc_ids"], ["D2"])

    def test_run_case_produces_complete_record(self):
        record = baseline_arm.run_case(self.tmp, NONCONFLICT_CASE, "harness-v-test")
        self.assertEqual(record["arm"], "baseline")
        self.assertEqual(record["case_id"], "C-T1-01")
        self.assertIsNone(record["error"])
        self.assertEqual(record["current_doc_ids"], ["D3"])

    def test_run_case_records_error_without_raising(self):
        bogus_root = self.tmp / "does" / "not" / "exist_but_parent_missing_permission_free"
        # A missing directory is created by setup_case itself via write_text's
        # implicit parent requirement -- force a real failure by pointing at a
        # path that collides with an existing file instead.
        collider = self.tmp / "collider"
        collider.write_text("not a directory", encoding="utf-8")
        record = baseline_arm.run_case(collider, NONCONFLICT_CASE, "harness-v-test")
        self.assertIsNotNone(record["error"])


if __name__ == "__main__":
    unittest.main()
