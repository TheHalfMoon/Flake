"""Tests for harness.py's resumability discipline: append-only manifest,
duplicate-cell refusal, and correct scan-then-skip behavior on resume."""
import json
import tempfile
import unittest
from pathlib import Path

import harness


class TestResumability(unittest.TestCase):
    def setUp(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.tmp = Path(self.tmpdir.name)

    def tearDown(self):
        self.tmpdir.cleanup()

    def test_existing_cells_empty_when_no_manifest(self):
        self.assertEqual(harness.existing_cells(self.tmp / "records.jsonl"), set())

    def test_append_record_then_scan_finds_it(self):
        manifest = self.tmp / "records.jsonl"
        seen = set()
        harness.append_record(manifest, {"case_id": "C1", "arm": "flake", "x": 1}, seen)
        rescanned = harness.existing_cells(manifest)
        self.assertEqual(rescanned, {("C1", "flake")})

    def test_duplicate_cell_refused(self):
        manifest = self.tmp / "records.jsonl"
        seen = set()
        harness.append_record(manifest, {"case_id": "C1", "arm": "flake", "x": 1}, seen)
        with self.assertRaises(harness.DuplicateCellError):
            harness.append_record(manifest, {"case_id": "C1", "arm": "flake", "x": 2}, seen)

    def test_simulated_interruption_resumes_without_duplication(self):
        """Writes half a collection, then a brand-new scan of the manifest
        (simulating a fresh process) correctly identifies only the
        remaining cells as missing -- no shared in-memory state is used."""
        manifest = self.tmp / "records.jsonl"
        seen = set()
        cells = [("C1", "flake"), ("C1", "baseline"), ("C2", "flake"), ("C2", "baseline")]
        for cid, arm in cells[:2]:
            harness.append_record(manifest, {"case_id": cid, "arm": arm}, seen)

        # Simulate a process restart: a fresh scan, no shared `seen` set.
        resumed_seen = harness.existing_cells(manifest)
        self.assertEqual(resumed_seen, {("C1", "flake"), ("C1", "baseline")})
        remaining = [c for c in cells if c not in resumed_seen]
        for cid, arm in remaining:
            harness.append_record(manifest, {"case_id": cid, "arm": arm}, resumed_seen)

        final = harness.existing_cells(manifest)
        self.assertEqual(final, set(cells))
        # Confirm no duplicate lines were written.
        with manifest.open(encoding="utf-8") as f:
            lines = [json.loads(l) for l in f if l.strip()]
        self.assertEqual(len(lines), 4)

    def test_run_collection_refuses_confirmatory_case_under_wrong_source(self):
        cases = {"cases": {"C1": {"case_id": "C1", "tier": "T1", "confirmatory": True, "sources": []}}}
        run_dir = self.tmp / "run"
        with self.assertRaises(RuntimeError):
            harness.run_collection(cases, run_dir, Path("unused-binary"), "hv", "pv", "dev_synthetic")


if __name__ == "__main__":
    unittest.main()
