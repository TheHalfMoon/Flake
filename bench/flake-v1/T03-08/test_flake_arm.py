"""Integration tests for flake_arm.py against the real compiled `fehrest`
binary. Requires a build under target/{release,debug}/fehrest[.exe] --
build it first with `cargo build --release --bin fehrest` (or debug)."""
import shutil
import tempfile
import unittest
from pathlib import Path

import flake_arm

REPO_ROOT = Path(__file__).resolve().parents[3]


def _find_binary():
    for candidate in ("target/release/fehrest.exe", "target/release/fehrest",
                       "target/debug/fehrest.exe", "target/debug/fehrest"):
        p = REPO_ROOT / candidate
        if p.exists():
            return p
    return None


BINARY = _find_binary()

NONCONFLICT_CASE = {
    "case_id": "C-T1-01",
    "tier": "T1",
    "sources": [
        {"doc_id": "D1", "title": "a", "body": "vendor-101 status: confirmed"},
        {"doc_id": "D2", "title": "b", "body": "vendor-101 status: rescheduled"},
        {"doc_id": "D3", "title": "c", "body": "vendor-101 status: cancelled"},
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


@unittest.skipIf(BINARY is None, "no built fehrest binary found under target/{release,debug}")
class TestFlakeArm(unittest.TestCase):
    def setUp(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.vault = Path(self.tmpdir.name) / "vault"
        flake_arm.run_cli(BINARY, self.vault, ["canonical-init"])

    def tearDown(self):
        self.tmpdir.cleanup()

    def test_nonconflict_case_resolves_to_last_doc(self):
        project_id, elapsed = flake_arm.setup_case(BINARY, self.vault, NONCONFLICT_CASE)
        self.assertGreater(elapsed, 0.0)
        facts = flake_arm.resume_case(BINARY, self.vault, project_id)
        self.assertEqual(facts["decision_state_outcome"], "CurrentSet")
        self.assertEqual(facts["current_doc_ids"], ["D3"])
        self.assertEqual(facts["resume_conflicts_declared"], 0)

    def test_conflict_case_surfaces_needs_review(self):
        project_id, _ = flake_arm.setup_case(BINARY, self.vault, CONFLICT_CASE)
        facts = flake_arm.resume_case(BINARY, self.vault, project_id)
        self.assertEqual(facts["decision_state_outcome"], "NeedsReview")
        self.assertEqual(facts["current_doc_ids"], ["D4", "D5"])
        self.assertGreaterEqual(facts["resume_conflicts_declared"], 1)

    def test_run_case_end_to_end(self):
        record = flake_arm.run_case(BINARY, self.vault, NONCONFLICT_CASE, "hv", "pv")
        self.assertEqual(record["arm"], "flake")
        self.assertIsNone(record["error"])
        self.assertEqual(record["current_doc_ids"], ["D3"])
        self.assertFalse(record["conflict_flagged"])

    def test_run_case_records_cli_error_without_raising(self):
        bogus_case = {"case_id": "BAD", "tier": "T1", "sources": [{"doc_id": "D1", "title": "a", "body": "x"}]}
        # A single-source T1 case: chain_docs = docs[:-1] = [] and
        # branch_docs = [D1], so setup itself succeeds trivially; force a
        # real CLI failure instead by pointing at an unwritable vault path.
        unwritable = Path(self.tmpdir.name) / "not-a-dir-parent" / "vault"
        (Path(self.tmpdir.name) / "not-a-dir-parent").write_text("collide", encoding="utf-8")
        record = flake_arm.run_case(BINARY, unwritable, bogus_case, "hv", "pv")
        self.assertIsNotNone(record["error"])


if __name__ == "__main__":
    unittest.main()
