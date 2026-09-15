#!/usr/bin/env python3
"""Append-only, resumable raw run manifest for the T03-07 local continuity
proof (PROTOCOL.md section 17).

Records are one JSON object per line in a `.jsonl` file. The file is never
rewritten in place -- only appended to. A facilitator can stop and restart
a study session at any point; `RunManifest.recorded_cells()` reconstructs
exactly which (participant_slot, pair_index, condition) cells already have
a record, so `append()` refuses a duplicate and `missing_cells()` tells the
facilitator exactly what remains.
"""
import json
from pathlib import Path

REQUIRED_FIELDS = [
    "schema_version", "participant_slot", "pair_index", "condition",
    "condition_order", "case_id", "case_source", "attempt_index",
    "harness_version", "product_version", "previous_state",
    "start_time", "end_time", "phase_durations", "outcome",
    "high_consequence_miss", "exclusion", "protocol_deviation",
]

VALID_CONDITIONS = {"flake", "baseline"}
VALID_OUTCOMES = {"RESUME_CORRECT", "RESUME_FAILED", "RESUME_TIMEOUT"}
VALID_EXCLUSIONS = {
    None,
    "FACILITATOR_PROTOCOL_ERROR",
    "TOOLING_CRASH_UNRELATED",
    "PARTICIPANT_WITHDREW_MID_ATTEMPT",
    "CASE_MATERIAL_DEFECT",
    "MISSING_REQUIRED_FIELD",
}
PHASE_FIELDS = [
    "setup_seconds", "maintenance_seconds", "navigation_seconds",
    "recovery_seconds", "answer_seconds",
]


class ManifestError(Exception):
    pass


class DuplicateCellError(ManifestError):
    pass


def validate_record(record: dict) -> list:
    """Returns a list of validation error strings; empty means valid."""
    errors = []
    for field in REQUIRED_FIELDS:
        if field not in record:
            errors.append(f"missing required field: {field}")
    if errors:
        return errors  # cannot check field values without the fields present

    if record["condition"] not in VALID_CONDITIONS:
        errors.append(f"invalid condition: {record['condition']!r}")
    if record["condition_order"] not in ("first", "second"):
        errors.append(f"invalid condition_order: {record['condition_order']!r}")
    if record["outcome"] not in VALID_OUTCOMES:
        errors.append(f"invalid outcome: {record['outcome']!r}")
    if record["exclusion"] not in VALID_EXCLUSIONS:
        errors.append(f"invalid exclusion code: {record['exclusion']!r}")
    if record["case_source"] not in ("confirmatory", "dev_synthetic"):
        errors.append(f"invalid case_source: {record['case_source']!r}")
    if not isinstance(record["high_consequence_miss"], bool):
        errors.append("high_consequence_miss must be boolean")

    durations = record["phase_durations"]
    if not isinstance(durations, dict):
        errors.append("phase_durations must be an object")
    else:
        for phase in PHASE_FIELDS:
            if phase not in durations:
                errors.append(f"phase_durations missing: {phase}")
            elif not isinstance(durations[phase], (int, float)) or durations[phase] < 0:
                errors.append(f"phase_durations.{phase} must be a non-negative number")
    return errors


class RunManifest:
    def __init__(self, path: Path):
        self.path = Path(path)

    def _read_all(self) -> list:
        if not self.path.exists():
            return []
        records = []
        with self.path.open("r", encoding="utf-8") as f:
            for line_number, line in enumerate(f, start=1):
                line = line.strip()
                if not line:
                    continue
                try:
                    records.append(json.loads(line))
                except json.JSONDecodeError as e:
                    raise ManifestError(
                        f"{self.path}:{line_number}: corrupt JSON line: {e}"
                    ) from e
        return records

    def all_records(self) -> list:
        return self._read_all()

    def recorded_cells(self) -> set:
        """Set of (participant_slot, pair_index, condition) tuples that
        already have at least one record -- the resume-without-duplication
        state (PROTOCOL.md section 17.2)."""
        cells = set()
        for r in self._read_all():
            cells.add((r.get("participant_slot"), r.get("pair_index"), r.get("condition")))
        return cells

    def missing_cells(self, expected_cells: set) -> set:
        return expected_cells - self.recorded_cells()

    def append(self, record: dict) -> None:
        errors = validate_record(record)
        if errors:
            raise ManifestError(f"refusing to append invalid record: {errors}")

        cell = (record["participant_slot"], record["pair_index"], record["condition"])
        if cell in self.recorded_cells():
            raise DuplicateCellError(
                f"a record already exists for {cell}; refusing to append a duplicate"
            )

        self.path.parent.mkdir(parents=True, exist_ok=True)
        with self.path.open("a", encoding="utf-8") as f:
            f.write(json.dumps(record, sort_keys=True))
            f.write("\n")

    def is_complete(self, expected_cells: set) -> bool:
        return len(self.missing_cells(expected_cells)) == 0


def expected_confirmatory_cells() -> set:
    from allocation import PARTICIPANT_SLOTS, PAIR_INDICES
    cells = set()
    for slot in PARTICIPANT_SLOTS:
        for pair_index in PAIR_INDICES:
            cells.add((slot, pair_index, "flake"))
            cells.add((slot, pair_index, "baseline"))
    return cells
