"""Independent maintained-Markdown + index/task-list baseline execution arm
for T03-08 (the "genuinely maintained workflow" condition, PROTOCOL.md
section 10, sealed by `bench/flake-v1/T03-07/SEALS.json`).

Deliberately written from scratch, in a different structural style than
`flake_arm.py`, and imports nothing from it, from `src/`, or from the
compiled `fehrest` binary -- this is the founder decision's explicit
"must not import Flake Core, Flake parsers, Flake analysis code" boundary.

Same independence-from-gold discipline as `flake_arm.py`: setup derives its
structure only from `case["tier"]` and `case["sources"]`, never from
`case["gold"]`. The resume phase re-derives its answer purely by re-reading
the maintained file from disk (a fresh read, not the in-memory state setup
produced) -- the file-based equivalent of `dry_run.py`'s "brand-new object,
no shared in-memory state" resumability discipline.
"""
from __future__ import annotations

import datetime
import re
import time
from pathlib import Path
from typing import Any

CONFLICT_TIERS = {"T3", "T4"}
ORG_STEPS_BY_TIER = {"T1": 1, "T2": 2, "T3": 3, "T4": 4}

_ENTRY_RE = re.compile(r"^- \[(CURRENT(?: \(conflicting\))?|SUPERSEDED)\] (\S+):")


def _now_iso() -> str:
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def _entry_line(doc: dict[str, Any], marker: str) -> str:
    return f"- [{marker}] {doc['doc_id']}: {doc['body']}"


def _render(case: dict[str, Any], log_lines: list[str], org_lines: list[str]) -> str:
    parts = [
        f"# {case['case_id']} -- maintained project file",
        "",
        "## Index",
        "",
        "(see Status Log below; the most recent [CURRENT] entry/entries are",
        "this project's authoritative status)",
        "",
        "## Status Log",
        "",
        *log_lines,
        "",
        "## Organizing Notes",
        "",
        *org_lines,
        "",
    ]
    return "\n".join(parts)


def setup_case(root_dir: Path, case: dict[str, Any]) -> tuple[Path, float]:
    """Write a genuinely maintained Markdown file for one case. Returns
    (case_file_path, elapsed_seconds). Never reads case["gold"]."""
    t0 = time.monotonic()
    docs = case["sources"]
    tier = case["tier"]
    conflict = tier in CONFLICT_TIERS
    case_file = root_dir / f"{case['case_id']}.md"

    if conflict:
        chain_docs, branch_docs = docs[:-2], docs[-2:]
    else:
        chain_docs, branch_docs = docs[:-1], docs[-1:]

    log_lines: list[str] = []
    for d in chain_docs:
        log_lines.append(_entry_line(d, "SUPERSEDED"))

    if conflict:
        log_lines.append(_entry_line(branch_docs[0], "CURRENT (conflicting)"))
        log_lines.append(_entry_line(branch_docs[1], "CURRENT (conflicting)"))
    else:
        log_lines.append(_entry_line(branch_docs[0], "CURRENT"))

    org_lines = [
        f"- organizing note {i + 1}: reviewed and organized {case['case_id']} materials (step {i + 1})"
        for i in range(ORG_STEPS_BY_TIER[tier])
    ]

    case_file.write_text(_render(case, log_lines, org_lines), encoding="utf-8")
    return case_file, time.monotonic() - t0


def resume_case(case_file: Path) -> dict[str, Any]:
    """Re-read the maintained file from disk (fresh read, no shared setup
    state) and re-derive the current status / conflict flag purely by
    parsing it. Never reads case["gold"]."""
    t0 = time.monotonic()
    text = case_file.read_text(encoding="utf-8")

    current_doc_ids: list[str] = []
    conflict_flagged = False
    for line in text.splitlines():
        m = _ENTRY_RE.match(line)
        if not m:
            continue
        marker, doc_id = m.group(1), m.group(2)
        if marker.startswith("CURRENT"):
            current_doc_ids.append(doc_id)
        if "conflicting" in marker:
            conflict_flagged = True

    return {
        "current_doc_ids": sorted(current_doc_ids),
        "conflict_flagged": conflict_flagged,
        "elapsed_seconds": time.monotonic() - t0,
    }


def run_case(root_dir: Path, case: dict[str, Any], harness_version: str) -> dict[str, Any]:
    """Run one confirmatory case end-to-end through the maintained-Markdown
    baseline. Returns a raw manifest record. Never consults case["gold"]."""
    start = _now_iso()
    error: str | None = None
    setup_seconds = 0.0
    facts = {"current_doc_ids": [], "conflict_flagged": False, "elapsed_seconds": 0.0}
    try:
        case_file, setup_seconds = setup_case(root_dir, case)
        facts = resume_case(case_file)
    except Exception as exc:  # noqa: BLE001 -- recorded as raw evidence, never hidden
        error = f"{type(exc).__name__}: {exc}"
    end = _now_iso()

    return {
        "schema_version": "t03-08-run-v1",
        "case_id": case["case_id"],
        "tier": case["tier"],
        "arm": "baseline",
        "harness_version": harness_version,
        "product_version": "n/a",
        "start_time": start,
        "end_time": end,
        "setup_seconds": setup_seconds,
        "navigation_seconds": facts["elapsed_seconds"],
        "current_doc_ids": facts["current_doc_ids"],
        "conflict_flagged": facts["conflict_flagged"],
        "resume_conflicts_declared": None,
        "decision_state_outcome": None,
        "error": error,
    }
