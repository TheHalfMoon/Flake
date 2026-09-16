"""Real Flake CLI/Core execution arm for T03-08's automated continuity
qualification (replaces the T03-07 human confirmatory trial per the founder
decision `docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md`).

Drives the compiled `fehrest` binary directly by subprocess through the
format-2 CLI surface -- never touches `canonical.sqlite` directly, never
imports any `src/` Rust code, never scripts around product behavior.

Independence-from-gold discipline: this module derives its setup structure
(which documents chain-supersede which, and which pair of documents is left
as a genuinely competing pair) only from `case["tier"]` and `case["sources"]`
-- both openly public case-CONSTRUCTION metadata already present in the
sealed `bench/flake-v1/T03-07/generate_cases.py` (`TIER_PARAMS`'s
`conflict`/`high_consequence` flags are a pure function of tier name). It
never reads `case["gold"]`. Grading against the gold key happens later,
exclusively in `analysis.py` / `verify_independent.py`, using only the raw
facts this module records here. This mirrors how a competent case reader
would organize the same material without being handed the answer key.
"""
from __future__ import annotations

import datetime
import re
import subprocess
import time
from pathlib import Path
from typing import Any

CONFLICT_TIERS = {"T3", "T4"}
DECISION_KEY = "status"
ORG_STEPS_BY_TIER = {"T1": 1, "T2": 2, "T3": 3, "T4": 4}


class CliError(RuntimeError):
    pass


def _now_iso() -> str:
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def run_cli(binary: Path, vault: Path, args: list[str], timeout: float = 30.0) -> str:
    cmd = [str(binary), *args, "--vault", str(vault)]
    proc = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    if proc.returncode != 0:
        raise CliError(
            f"cli failed rc={proc.returncode} args={args} "
            f"stdout={proc.stdout.strip()!r} stderr={proc.stderr.strip()!r}"
        )
    return proc.stdout


def project_create(binary: Path, vault: Path, name: str) -> str:
    out = run_cli(binary, vault, ["project-create", "--name", name])
    return out.strip().split()[0]


def decision_create(binary: Path, vault: Path, project_id: str, key: str, statement: str) -> tuple[str, str]:
    out = run_cli(
        binary, vault,
        ["decision-create", "--project", project_id, "--key", key,
         "--statement", statement, "--basis", "evidence"],
    )
    obj_id, rev_id = out.strip().split()
    return obj_id, rev_id


def decision_accept(binary: Path, vault: Path, decision_id: str, expect: str) -> tuple[str, str]:
    out = run_cli(binary, vault, ["decision-accept", "--id", decision_id, "--expect", expect])
    parts = out.strip().split()
    return parts[0], parts[1]


def decision_supersede(binary: Path, vault: Path, new_id: str, old_id: str,
                        expect_old_rev: str, reason: str) -> None:
    run_cli(
        binary, vault,
        ["decision-supersede", "--new", new_id, "--old", old_id,
         "--expect", expect_old_rev, "--reason", reason],
    )


def note_create(binary: Path, vault: Path, project_id: str, title: str, body: str) -> str:
    out = run_cli(binary, vault, ["note-create", "--project", project_id, "--title", title, "--body", body])
    return out.strip().split()[0]


def decision_state(binary: Path, vault: Path, project_id: str, key: str) -> str:
    return run_cli(binary, vault, ["decision-state", "--project", project_id, "--key", key])


def resume(binary: Path, vault: Path, project_id: str) -> str:
    return run_cli(binary, vault, ["resume", "--project", project_id])


def record_show(binary: Path, vault: Path, record_id: str) -> str:
    return run_cli(binary, vault, ["record-show", "--id", record_id])


def _doc_marker(doc_id: str) -> str:
    return f"[[DOC:{doc_id}]]"


def setup_case(binary: Path, vault: Path, case: dict[str, Any]) -> tuple[str, float]:
    """Capture a case's sources into a fresh Flake project. Returns
    (project_id, elapsed_seconds). Never reads case["gold"]."""
    t0 = time.monotonic()
    docs = case["sources"]
    tier = case["tier"]
    conflict = tier in CONFLICT_TIERS

    project_id = project_create(binary, vault, f"T03-08 {case['case_id']}")

    if conflict:
        chain_docs, branch_docs = docs[:-2], docs[-2:]
    else:
        chain_docs, branch_docs = docs[:-1], docs[-1:]

    prev_id: str | None = None
    prev_rev: str | None = None
    for d in chain_docs:
        statement = f"{_doc_marker(d['doc_id'])} {d['body']}"
        did, rev = decision_create(binary, vault, project_id, DECISION_KEY, statement)
        did, rev = decision_accept(binary, vault, did, rev)
        if prev_id is not None:
            decision_supersede(binary, vault, did, prev_id, prev_rev, f"superseded by {d['doc_id']}")
        prev_id, prev_rev = did, rev

    for i, d in enumerate(branch_docs):
        statement = f"{_doc_marker(d['doc_id'])} {d['body']}"
        did, rev = decision_create(binary, vault, project_id, DECISION_KEY, statement)
        did, rev = decision_accept(binary, vault, did, rev)
        if prev_id is not None and (not conflict or i == 0):
            # Non-conflict: the single branch doc always supersedes the
            # chain end. Conflict: only the FIRST branch doc (the
            # doc_count-1'th overall) supersedes the chain end; the second
            # branch doc is deliberately left un-superseded so it competes
            # for the same key -- the planted, genuinely surfaced conflict.
            decision_supersede(binary, vault, did, prev_id, prev_rev, f"superseded by {d['doc_id']}")
            prev_id, prev_rev = did, rev
        elif not conflict:
            prev_id, prev_rev = did, rev
        # conflict branch's second doc: intentionally left un-chained.

    for i in range(ORG_STEPS_BY_TIER[tier]):
        note_create(
            binary, vault, project_id, f"organizing note {i + 1}",
            f"Reviewed and organized {case['case_id']} materials (step {i + 1}).",
        )

    return project_id, time.monotonic() - t0


_ADMITTED_RE = re.compile(r"^\s*(\S+)\s+admitted=(true|false)")
_OUTCOME_RE = re.compile(r"outcome=(\w+)")
_MARKER_RE = re.compile(r"\[\[DOC:([^\]]+)\]\]")
_CONFLICTS_RE = re.compile(r"conflicts: (\d+)")


def resume_case(binary: Path, vault: Path, project_id: str) -> dict[str, Any]:
    """Exercise the real `resume` and `decision-state` CLI paths and extract
    which doc_id(s) currently resolve as admitted for DECISION_KEY. Never
    reads case["gold"]."""
    t0 = time.monotonic()
    resume_out = resume(binary, vault, project_id)
    conflicts_declared = int(_CONFLICTS_RE.search(resume_out).group(1))

    state_out = decision_state(binary, vault, project_id, DECISION_KEY)
    lines = state_out.splitlines()
    outcome = _OUTCOME_RE.search(lines[0]).group(1)

    admitted_ids = [
        m.group(1)
        for line in lines[1:]
        if (m := _ADMITTED_RE.match(line)) and m.group(2) == "true"
    ]

    current_doc_ids = []
    for did in admitted_ids:
        shown = record_show(binary, vault, did)
        m = _MARKER_RE.search(shown)
        if m:
            current_doc_ids.append(m.group(1))

    return {
        "resume_conflicts_declared": conflicts_declared,
        "decision_state_outcome": outcome,
        "current_doc_ids": sorted(current_doc_ids),
        "elapsed_seconds": time.monotonic() - t0,
    }


def run_case(binary: Path, vault: Path, case: dict[str, Any],
             harness_version: str, product_version: str) -> dict[str, Any]:
    """Run one confirmatory case end-to-end through the real Flake CLI.
    Returns a raw manifest record. Never consults case["gold"]."""
    start = _now_iso()
    error: str | None = None
    setup_seconds = 0.0
    facts = {
        "resume_conflicts_declared": None,
        "decision_state_outcome": None,
        "current_doc_ids": [],
        "elapsed_seconds": 0.0,
    }
    try:
        project_id, setup_seconds = setup_case(binary, vault, case)
        facts = resume_case(binary, vault, project_id)
    except Exception as exc:  # noqa: BLE001 -- recorded as raw evidence, never hidden
        error = f"{type(exc).__name__}: {exc}"
    end = _now_iso()

    conflict_flagged = facts["decision_state_outcome"] == "NeedsReview"

    return {
        "schema_version": "t03-08-run-v1",
        "case_id": case["case_id"],
        "tier": case["tier"],
        "arm": "flake",
        "harness_version": harness_version,
        "product_version": product_version,
        "start_time": start,
        "end_time": end,
        "setup_seconds": setup_seconds,
        "navigation_seconds": facts["elapsed_seconds"],
        "current_doc_ids": facts["current_doc_ids"],
        "conflict_flagged": conflict_flagged,
        "resume_conflicts_declared": facts["resume_conflicts_declared"],
        "decision_state_outcome": facts["decision_state_outcome"],
        "error": error,
    }
