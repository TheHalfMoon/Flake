#!/usr/bin/env python3
"""Synthetic, non-evaluation dry run for the T03-07 local continuity proof
(PROTOCOL.md section 19).

Exercises collection, resume-after-interruption, failure recording,
timeout recording, exclusion routing, counterbalancing, analysis,
independent arithmetic checks, and manifest recovery -- using only
`cases_dev.json`, never treated as human evidence.
"""
import hashlib
import json
import shutil
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()
sys.path.insert(0, str(HERE))

from allocation import full_allocation_table
from generate_cases import TIER_BY_PAIR_INDEX
import analysis
import verify_independent
from harness import RunManifest, DuplicateCellError, expected_confirmatory_cells

DRY_RUN_DIR = HERE / "runs" / "dry-run"
DRY_RUN_MANIFEST = DRY_RUN_DIR / "records.jsonl"

# Deterministic tier -> dev case pool id, matching generate_cases.py's
# generate_dev_cases() numbering (T1->1, T2->2, T3->3, T4->4).
DEV_POOL_ID_BY_TIER = {"T1": 1, "T2": 2, "T3": 3, "T4": 4}


def _dev_case_id(tier: str, role: str) -> str:
    pool_id = DEV_POOL_ID_BY_TIER[tier]
    return f"DEV-{pool_id:02d}-{tier}-{role}"


def _deterministic_bool(*parts: str) -> bool:
    digest = hashlib.sha256(":".join(parts).encode("utf-8")).hexdigest()
    return int(digest[:2], 16) % 5 == 0  # ~20% true, deterministic


def _build_record(row: dict, condition: str, attempt_seq: int,
                   force_outcome: str = None, force_exclusion: str = None,
                   force_deviation: str = None) -> dict:
    role = row["case_role_for_condition"][condition]
    case_id = _dev_case_id(row["tier"], role)
    condition_order = "first" if row["condition_attempted_first"] == condition else "second"

    is_failure = _deterministic_bool("fail", case_id, condition) and condition == "baseline"
    outcome = force_outcome or ("RESUME_FAILED" if is_failure else "RESUME_CORRECT")

    # Flake is deterministically faster than baseline in this synthetic
    # shape -- it is a fixture for exercising the harness/analysis
    # machinery, not a claim about real product performance.
    base = 60 + (attempt_seq % 7) * 5
    if condition == "flake":
        durations = {
            "setup_seconds": base * 0.6, "maintenance_seconds": 5.0,
            "navigation_seconds": base * 0.15, "recovery_seconds": 2.0,
            "answer_seconds": base * 0.2,
        }
    else:
        durations = {
            "setup_seconds": base * 0.9, "maintenance_seconds": 8.0,
            "navigation_seconds": base * 0.4, "recovery_seconds": 6.0,
            "answer_seconds": base * 0.3,
        }

    return {
        "schema_version": "t03-07-run-v1",
        "participant_slot": row["participant_slot"],
        "pair_index": row["pair_index"],
        "condition": condition,
        "condition_order": condition_order,
        "case_id": case_id,
        "case_source": "dev_synthetic",
        "attempt_index": attempt_seq,
        "harness_version": "dry-run-fixture",
        "product_version": "n/a" if condition == "baseline" else "dry-run-fixture",
        "previous_state": f"synthetic-setup-state-{case_id}",
        "start_time": f"2026-01-01T00:{attempt_seq % 60:02d}:00Z",
        "end_time": f"2026-01-01T00:{attempt_seq % 60:02d}:{int(sum(durations.values())) % 60:02d}Z",
        "phase_durations": durations,
        "outcome": outcome,
        "high_consequence_miss": False,
        "exclusion": force_exclusion,
        "protocol_deviation": force_deviation,
    }


def generate_dry_run_records() -> list:
    rows = full_allocation_table(TIER_BY_PAIR_INDEX)
    records = []
    attempt_seq = 0
    injected_timeout = False
    injected_exclusion = False
    injected_deviation = False

    for row in rows:
        for condition in ("flake", "baseline"):
            attempt_seq += 1
            force_outcome = None
            force_exclusion = None
            force_deviation = None

            # Inject exactly one of each required scenario, on fixed,
            # easy-to-locate cells rather than randomly, so the dry-run
            # report can point at them directly.
            if not injected_timeout and row["participant_slot"] == "P1" and row["pair_index"] == 1 and condition == "baseline":
                force_outcome = "RESUME_TIMEOUT"
                injected_timeout = True
            elif not injected_exclusion and row["participant_slot"] == "P2" and row["pair_index"] == 3 and condition == "flake":
                force_outcome = "RESUME_FAILED"
                force_exclusion = "FACILITATOR_PROTOCOL_ERROR"
                injected_exclusion = True
            elif not injected_deviation and row["participant_slot"] == "P3" and row["pair_index"] == 5 and condition == "baseline":
                force_deviation = "Facilitator restarted the timer after a fire-drill interruption unrelated to the case."
                injected_deviation = True

            records.append(_build_record(row, condition, attempt_seq,
                                          force_outcome, force_exclusion, force_deviation))

    assert injected_timeout and injected_exclusion and injected_deviation, (
        "dry run must inject exactly one timeout, one exclusion, and one protocol deviation"
    )
    return records


def run_collection_with_simulated_interruption(records: list) -> None:
    """Writes the first half of records, then simulates the harness
    process being killed and restarted by constructing a brand-new
    RunManifest and continuing from missing_cells() -- proving no
    duplicate and no silently lost attempt."""
    if DRY_RUN_DIR.exists():
        shutil.rmtree(DRY_RUN_DIR)

    manifest = RunManifest(DRY_RUN_MANIFEST)
    midpoint = len(records) // 2
    for record in records[:midpoint]:
        manifest.append(record)

    # "process restart": a fresh RunManifest instance, no in-memory state
    # carried over, reading only what is on disk.
    resumed_manifest = RunManifest(DRY_RUN_MANIFEST)
    expected_cells = {(r["participant_slot"], r["pair_index"], r["condition"]) for r in records}
    missing_before = resumed_manifest.missing_cells(expected_cells)
    assert len(missing_before) == len(records) - midpoint, (
        f"expected {len(records) - midpoint} missing cells after simulated restart, "
        f"found {len(missing_before)}"
    )

    for record in records[midpoint:]:
        resumed_manifest.append(record)

    assert resumed_manifest.is_complete(expected_cells), "manifest incomplete after full replay"

    # Duplicate-append must be refused, not silently accepted.
    try:
        resumed_manifest.append(records[0])
        raise AssertionError("expected DuplicateCellError, none was raised")
    except DuplicateCellError:
        pass


def confirm_dev_guard_raises() -> None:
    """Section 14.2's guard: a dev_synthetic attempt must never be
    accepted into a confirmatory-style analysis."""
    fake_record = {
        "case_source": "dev_synthetic", "case_id": "DEV-01-T1-A",
        "participant_slot": "P1", "pair_index": 1, "condition": "flake",
    }
    try:
        analysis.analyze(
            [fake_record], expected_pairs=1, min_valid_pairs=1,
            case_tiers={"DEV-01-T1-A": "T1"}, forbid_case_source="dev_synthetic",
        )
        raise AssertionError("expected AnalysisError, none was raised")
    except analysis.AnalysisError:
        pass


def main() -> int:
    records = generate_dry_run_records()
    run_collection_with_simulated_interruption(records)
    confirm_dev_guard_raises()

    from generate_cases import DEV_SEED  # noqa: F401 (documents provenance in output)
    cases_path = HERE / "cases_dev.json"
    case_tiers = json.loads(cases_path.read_text(encoding="utf-8"))["cases"]
    case_tiers = {cid: c["tier"] for cid, c in case_tiers.items()}

    analysis_result = analysis.analyze(
        records, expected_pairs=48, min_valid_pairs=44, case_tiers=case_tiers,
    )
    verification_result = verify_independent.verify(
        records, expected_pairs=48, min_valid_pairs=44, case_tiers=case_tiers,
    )

    agreement = analysis_result["route"] == verification_result["route"]

    report = {
        "dry_run_attempt_count": len(records),
        "injected_scenarios": ["RESUME_TIMEOUT", "FACILITATOR_PROTOCOL_ERROR exclusion",
                                "protocol_deviation (non-excluding)",
                                "simulated harness interruption/resume",
                                "duplicate-append refusal",
                                "dev_synthetic confirmatory-guard rejection"],
        "analysis_route": analysis_result["route"],
        "verify_independent_route": verification_result["route"],
        "producer_and_oracle_agree": agreement,
        "analysis_full": analysis_result,
    }
    print(json.dumps(report, indent=2, sort_keys=True))

    if not agreement:
        print("DRY RUN FAILED: analysis.py and verify_independent.py disagree", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
