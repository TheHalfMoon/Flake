#!/usr/bin/env python3
"""T02-07 independent-verifier driver.

Orchestrates (see README.md for the independence boundary):
  1. builds a disposable fixture via the real `fehrest` CLI (build_fixture.py)
  2. reads canonical.sqlite directly (sqlite_reader.py)
  3. reads the full-store and project-scoped exports (export_reader.py)
  4. builds independent semantic reports from both (semantic_model.py)
  5. cross-checks them against each other (crosscheck.py)
  6. proves the CLI loop survives derived-index removal
  7. runs the adversarial/negative suite (adversarial.py)
  8. proves cross-project isolation at the raw-byte level

Prints a JSON report to stdout and exits non-zero if anything failed.
"""
import argparse
import json
import sys
from pathlib import Path

import build_fixture
import export_reader
import sqlite_reader
import semantic_model
import crosscheck
import adversarial


def scan_for_marker(export_dir: Path, marker: str) -> list[str]:
    hits = []
    for path in export_dir.rglob("*"):
        if path.is_file():
            try:
                text = path.read_text(encoding="utf-8", errors="strict")
            except (UnicodeDecodeError, PermissionError):
                text = path.read_bytes().decode("latin-1")
            if marker in text:
                hits.append(str(path.relative_to(export_dir)))
    return hits


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", required=True, help="path to the fehrest CLI binary")
    parser.add_argument("--workdir", required=True, help="disposable scratch directory")
    args = parser.parse_args()

    binary = Path(args.binary).resolve()
    workdir = Path(args.workdir).resolve()

    report: dict = {"sections": {}}
    failures: list[str] = []

    # 1. Fixture via the real CLI.
    fixture = build_fixture.build(binary, workdir)
    fixture["derived_index_removal_proof"] = build_fixture.prove_derived_index_removal(
        binary, Path(fixture["vault"]), fixture["project_a"], workdir
    )
    report["sections"]["fixture"] = fixture
    if not fixture["derived_index_removal_proof"]["index_existed_before_removal"]:
        failures.append("derived index was never built, so its removal proves nothing")
    if fixture["derived_index_removal_proof"]["index_exists_after_removal"]:
        failures.append("derived index still exists after deletion attempt")

    # 2. Raw SQLite read.
    db_path = Path(fixture["vault"]) / ".fehrest" / "canonical.sqlite"
    sqlite_result = sqlite_reader.read_vault(db_path)
    sqlite_semantic = semantic_model.build_report(sqlite_result["envelopes"])
    report["sections"]["sqlite"] = {
        "vault_row": sqlite_result["vault_row"],
        "command_count": sqlite_result["command_count"],
        "head_hash_chain_verified": sqlite_result["head_hash_chain_verified"],
        "semantic_record_count_by_kind": sqlite_semantic["record_count_by_kind"],
        "semantic_revision_count": sqlite_semantic["revision_count"],
    }

    # 3 + 4. Raw export reads + semantic reports (full, project-scoped, and the
    # re-export taken after the derived index was deleted).
    export_full_dir = Path(fixture["export_full_dir"])
    export_project_dir = Path(fixture["export_project_dir"])
    export_after_removal_dir = Path(fixture["derived_index_removal_proof"]["export_after_index_removed_dir"])

    export_full_raw = export_reader.read_export(export_full_dir)
    export_full_semantic = semantic_model.build_report(export_full_raw["envelopes"])
    report["sections"]["export_full"] = {
        "manifest_kind": export_full_raw["manifest"]["kind"],
        "record_count": export_full_raw["manifest"]["record_count"],
        "revision_count": export_full_raw["manifest"]["revision_count"],
        "integrity_root": export_full_raw["manifest"]["integrity_root"],
        "dangling_references": export_full_raw["dangling_references"],
    }
    if export_full_raw["dangling_references"]:
        failures.append(f"full export has dangling references: {export_full_raw['dangling_references']}")

    export_project_raw = export_reader.read_export(export_project_dir)
    export_project_semantic = semantic_model.build_report(export_project_raw["envelopes"])
    report["sections"]["export_project"] = {
        "manifest_kind": export_project_raw["manifest"]["kind"],
        "record_count": export_project_raw["manifest"]["record_count"],
        "revision_count": export_project_raw["manifest"]["revision_count"],
        "integrity_root": export_project_raw["manifest"]["integrity_root"],
        "dangling_references": export_project_raw["dangling_references"],
    }
    if export_project_raw["dangling_references"]:
        failures.append(f"project export has dangling references: {export_project_raw['dangling_references']}")

    export_after_removal_raw = export_reader.read_export(export_after_removal_dir)
    export_after_removal_semantic = semantic_model.build_report(export_after_removal_raw["envelopes"])
    report["sections"]["export_after_index_removed"] = {
        "record_count": export_after_removal_raw["manifest"]["record_count"],
        "revision_count": export_after_removal_raw["manifest"]["revision_count"],
    }

    # 5. Cross-checks: two independent artifacts of the same state must agree.
    full_problems = crosscheck.compare_full_export(sqlite_semantic, export_full_semantic)
    report["sections"]["crosscheck_full"] = {"problems": full_problems}
    failures.extend(f"crosscheck(full): {p}" for p in full_problems)

    forbidden = set(fixture["project_b_object_ids"])
    project_problems = crosscheck.compare_project_export(
        sqlite_semantic, export_project_semantic, fixture["project_a"], forbidden
    )
    report["sections"]["crosscheck_project"] = {"problems": project_problems}
    failures.extend(f"crosscheck(project): {p}" for p in project_problems)

    # The re-export taken after the derived index was deleted must describe
    # the identical project-A state as the original project export -- proof
    # that canonical export does not depend on the derived FTS index.
    reexport_problems = crosscheck.compare_reports(
        "reexport-after-index-removed vs original project export",
        export_project_semantic, export_after_removal_semantic,
    )
    report["sections"]["crosscheck_reexport_after_index_removed"] = {"problems": reexport_problems}
    failures.extend(f"crosscheck(reexport): {p}" for p in reexport_problems)

    # Raw-byte cross-project leak scan (stricter than semantic comparison:
    # catches a leak even if it were hiding in an unparsed/unknown field).
    marker_hits = scan_for_marker(export_project_dir, fixture["project_b_secret_marker"])
    report["sections"]["cross_project_byte_scan"] = {"marker_hits": marker_hits}
    if marker_hits:
        failures.append(f"project B secret marker leaked into project A export: {marker_hits}")
    for forbidden_id in fixture["project_b_object_ids"]:
        id_hits = scan_for_marker(export_project_dir, forbidden_id)
        if id_hits:
            failures.append(f"project B object id {forbidden_id} leaked into project A export: {id_hits}")

    # 6. FTS search sanity (already exercised inside build_fixture; recorded here).
    report["sections"]["find_loop"] = {
        "fts_search_hits": fixture["fts_search_hit_count_before_index_removed"],
    }
    if fixture["fts_search_hit_count_before_index_removed"] != "hits: 1":
        failures.append(
            f"expected fts-search to find exactly 1 hit for the updated note, got "
            f"{fixture['fts_search_hit_count_before_index_removed']!r}"
        )

    # 7. Adversarial suite, run against the full export (broadest kind coverage).
    adversarial_scratch = workdir / "adversarial"
    adversarial_scratch.mkdir(exist_ok=True)
    adversarial_results = adversarial.run_all_cases(export_full_dir, adversarial_scratch)
    report["sections"]["adversarial"] = adversarial_results
    for result in adversarial_results:
        if not result["passed"]:
            failures.append(f"adversarial case {result['name']} failed: {result['message']}")

    report["failures"] = failures
    report["passed"] = not failures
    print(json.dumps(report, indent=2, default=str))
    return 0 if not failures else 1


if __name__ == "__main__":
    raise SystemExit(main())
