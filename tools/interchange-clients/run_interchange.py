"""`T03-06` interchange conformance orchestrator.

Drives the exact scenario this task's own acceptance criterion names:
client A reads a package and proposes a note/action update; the owner
reviews and accepts it through Flake's real CLI; client B reads the
*resulting* package/accepted history and continues with a compatible,
different proposal; the owner reviews and accepts that too. Then runs the
three adversarial cases (`negative_cases.py`) and independently verifies
final canonical state by reading `canonical.sqlite` directly (reusing
`tools/independent-verify/sqlite_reader.py`'s already-independent
low-level reader -- never trusting the CLI's own stdout as the sole
oracle for content correctness, only for wiring calls together).

This file is the orchestrator, not a framework: one linear script, no
class hierarchy, no plugin system. Offline throughout -- every subprocess
call is either the `fehrest` binary or a `python3` interpreter running one
of this directory's own fixture scripts.
"""
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO_ROOT = HERE.parent.parent
sys.path.insert(0, str(REPO_ROOT / "tools" / "independent-verify"))
import sqlite_reader  # noqa: E402  (independent reader, imported after sys.path setup)

import negative_cases  # noqa: E402


class ConformanceFailure(Exception):
    pass


def find_binary(explicit):
    if explicit:
        return explicit
    for profile in ("debug", "release"):
        candidate = REPO_ROOT / "target" / profile / "fehrest.exe"
        if candidate.exists():
            return str(candidate)
        candidate = REPO_ROOT / "target" / profile / "fehrest"
        if candidate.exists():
            return str(candidate)
    raise ConformanceFailure(
        "no fehrest binary found under target/debug or target/release; "
        "run `cargo build --locked --bin fehrest` first"
    )


def run_cli(binary, args, expect_success=True):
    proc = subprocess.run([binary, *args], capture_output=True, text=True, timeout=60)
    if expect_success and proc.returncode != 0:
        raise ConformanceFailure(
            f"command failed: {args}\nstdout={proc.stdout}\nstderr={proc.stderr}"
        )
    return proc.returncode, proc.stdout.strip(), proc.stderr.strip()


def first_token(line):
    return line.split()[0]


def second_token(line):
    return line.split()[1]


def extract_field(line, name):
    for tok in line.split():
        if tok.startswith(name + "="):
            return tok[len(name) + 1 :]
    raise ConformanceFailure(f"field {name!r} not found in line: {line!r}")


def run_client(script_name, args):
    script = HERE / script_name
    proc = subprocess.run(
        [sys.executable, str(script), *args], capture_output=True, text=True, timeout=30
    )
    if proc.returncode != 0:
        raise ConformanceFailure(f"{script_name} failed: {proc.stdout}\n{proc.stderr}")
    return proc.stdout.strip()


def read_final_vault(vault_root):
    db_path = Path(vault_root) / ".fehrest" / "canonical.sqlite"
    return sqlite_reader.read_vault(db_path)


def main():
    report_lines = []

    def log(line):
        print(line)
        report_lines.append(line)

    binary = find_binary(None)
    log(f"fehrest binary: {binary}")

    work_dir = Path(tempfile.mkdtemp(prefix="fehrest-interchange-"))
    vault = str(work_dir / "vault")
    log(f"scratch vault: {vault}")

    try:
        run_cli(binary, ["canonical-init", "--vault", vault])
        _, out, _ = run_cli(binary, ["project-create", "--vault", vault, "--name", "Interop"])
        project_id = first_token(out)
        log(f"project_id={project_id}")

        _, out, _ = run_cli(
            binary,
            ["note-create", "--vault", vault, "--project", project_id, "--body", "Original note body"],
        )
        note_id = first_token(out)
        log(f"note_id={note_id}")

        _, out, _ = run_cli(binary, ["grant-issue", "--vault", vault, "--project", project_id])
        grant_id = first_token(out)
        log(f"grant_id={grant_id}")

        # ---- Client A: reads a package, proposes a note edit ----------
        package_a = str(work_dir / "package_a.jsonl")
        _, out, _ = run_cli(
            binary,
            [
                "package-export", "--vault", vault, "--grant", grant_id,
                "--request-id", "req-a", "--out", package_a,
            ],
        )
        receipt_a = extract_field(out, "receipt_id")
        log(f"receipt_a={receipt_a}")

        proposal_a = str(work_dir / "proposal_a.json")
        log(run_client("client_a.py", [package_a, receipt_a, proposal_a]))

        _, out, _ = run_cli(
            binary,
            ["propose-import", "--vault", vault, "--project", project_id, "--file", proposal_a],
        )
        proposal_a_id, proposal_a_rev = first_token(out), second_token(out)
        log(f"proposal_a_id={proposal_a_id} status_line={out}")

        _, out, _ = run_cli(
            binary,
            [
                "propose-accept", "--vault", vault, "--id", proposal_a_id,
                "--expect", proposal_a_rev, "--select", "0",
            ],
        )
        log(f"accept_a: {out}")

        # Independent check #1: the note's canonical body actually changed,
        # verified by reading canonical.sqlite directly, not by trusting
        # the CLI's own printed status line.
        vault_state = read_final_vault(vault)
        note_envelope = [
            e for e in vault_state["envelopes"]
            if e["object_id"] == note_id and e["kind"] == "note"
        ][-1]
        if "client A" not in note_envelope["payload"]["body"] and "Edited by interchange client A" not in note_envelope["payload"]["body"]:
            raise ConformanceFailure(
                f"note body after client A's accepted proposal does not reflect the edit: {note_envelope['payload']['body']!r}"
            )
        log("independent check: note body reflects client A's accepted edit (verified via canonical.sqlite)")

        # ---- Client B: reads the resulting package, proposes a draft decision ----
        package_b = str(work_dir / "package_b.jsonl")
        _, out, _ = run_cli(
            binary,
            [
                "package-export", "--vault", vault, "--grant", grant_id,
                "--request-id", "req-b", "--out", package_b,
            ],
        )
        receipt_b = extract_field(out, "receipt_id")
        log(f"receipt_b={receipt_b}")

        proposal_b = str(work_dir / "proposal_b.json")
        log(run_client("client_b.py", [package_b, receipt_b, proposal_b]))

        _, out, _ = run_cli(
            binary,
            ["propose-import", "--vault", vault, "--project", project_id, "--file", proposal_b],
        )
        proposal_b_id, proposal_b_rev = first_token(out), second_token(out)
        log(f"proposal_b_id={proposal_b_id} status_line={out}")

        _, out, _ = run_cli(
            binary,
            [
                "propose-accept", "--vault", vault, "--id", proposal_b_id,
                "--expect", proposal_b_rev, "--select", "0",
            ],
        )
        log(f"accept_b: {out}")

        # Independent check #2: a Draft decision with the expected key and
        # basis=agent_proposal now exists in canonical state.
        vault_state = read_final_vault(vault)
        decision_envelopes = [
            e for e in vault_state["envelopes"] if e["kind"] == "decision"
            and e["payload"].get("decision_key") == "client-b-observation"
        ]
        if not decision_envelopes:
            raise ConformanceFailure("no decision with key 'client-b-observation' found after client B's accepted proposal")
        decision_payload = decision_envelopes[-1]["payload"]
        if decision_payload.get("basis") != "agent_proposal":
            raise ConformanceFailure(f"expected basis=agent_proposal, got {decision_payload.get('basis')!r}")
        if decision_payload.get("lifecycle") != "draft":
            raise ConformanceFailure(f"expected lifecycle=draft (agents never self-accept), got {decision_payload.get('lifecycle')!r}")
        log("independent check: draft decision from client B exists with basis=agent_proposal, lifecycle=draft (verified via canonical.sqlite)")

        # ---- Negative / adversarial cases ------------------------------
        # Fresh note+revision for these, so they never collide with the
        # already-accepted proposals above.
        _, out, _ = run_cli(
            binary,
            ["note-create", "--vault", vault, "--project", project_id, "--body", "adversarial fixture note"],
        )
        adv_note_id, adv_note_rev = first_token(out), second_token(out)
        _, out, _ = run_cli(
            binary,
            [
                "package-export", "--vault", vault, "--grant", grant_id,
                "--request-id", "req-adv", "--out", str(work_dir / "package_adv.jsonl"),
            ],
        )
        receipt_adv = extract_field(out, "receipt_id")

        results = []
        results.append(
            negative_cases.unknown_declared_identity(
                binary, vault, project_id, receipt_adv, adv_note_id, adv_note_rev,
                str(work_dir / "proposal_unknown.json"),
            )
        )
        results.append(
            negative_cases.unsupported_operation_kind(
                binary, vault, project_id, receipt_adv, str(work_dir / "proposal_bad_kind.json"),
            )
        )
        results.append(
            negative_cases.duplicate_delivery(
                binary, vault, project_id, receipt_adv, adv_note_id, adv_note_rev,
                str(work_dir / "proposal_dup.json"),
            )
        )

        for r in results:
            status = "PASS" if r["passed"] else "FAIL"
            log(f"negative case [{status}] {r['case']}: expected {r['expected']}, got exit_code={r['exit_code']}")
            if not r["passed"]:
                raise ConformanceFailure(f"negative case {r['case']} did not behave as the protocol document requires: {r}")

        # Independent check #2.5: the refused unsupported-kind proposal
        # never became a committed record at all -- not merely that the
        # CLI exited nonzero, but that no `agent_proposal` object anywhere
        # in canonical state contains a "delete_everything" operation.
        vault_state = read_final_vault(vault)
        bad_kind_found = any(
            "delete_everything" in json.dumps(e["payload"])
            for e in vault_state["envelopes"]
            if e["kind"] == "agent_proposal"
        )
        if bad_kind_found:
            raise ConformanceFailure("the unsupported-kind proposal was committed despite being refused")
        log("independent check: the refused unsupported-kind proposal was never committed as any record (verified via canonical.sqlite)")

        # Independent check #3: the unknown-identity proposal's declared_*
        # fields are genuinely absent (null), not defaulted to any guess.
        vault_state = read_final_vault(vault)
        proposal_envelopes = [e for e in vault_state["envelopes"] if e["kind"] == "agent_proposal"]
        unknown_id_proposals = [
            e for e in proposal_envelopes
            if e["payload"].get("declared_agent") is None
            and any(op.get("note_id") == adv_note_id for op in e["payload"].get("operations", []))
        ]
        if not unknown_id_proposals:
            raise ConformanceFailure("the unknown-declared-identity proposal was not found with declared_agent=null")
        log("independent check: unknown-declared-identity proposal has declared_agent=null, never a guessed default (verified via canonical.sqlite)")

        # Independent check #4: duplicate delivery produced two distinct
        # proposal object_ids, both admitted.
        dup_body_proposals = [
            e for e in proposal_envelopes
            if any(
                op.get("body") == "duplicate delivery candidate body"
                for op in e["payload"].get("operations", [])
            )
        ]
        distinct_ids = {e["object_id"] for e in dup_body_proposals}
        if len(distinct_ids) != 2:
            raise ConformanceFailure(f"expected exactly 2 distinct proposal object_ids for duplicate delivery, found {len(distinct_ids)}")
        log("independent check: duplicate delivery produced exactly two distinct Pending proposals (verified via canonical.sqlite)")

        log(f"head_hash_chain_verified={vault_state['head_hash_chain_verified']} command_count={vault_state['command_count']}")
        log("ALL CHECKS PASSED")
        return 0, report_lines
    finally:
        shutil.rmtree(work_dir, ignore_errors=True)


if __name__ == "__main__":
    try:
        code, _ = main()
    except ConformanceFailure as e:
        print(f"CONFORMANCE FAILURE: {e}", file=sys.stderr)
        code = 1
    sys.exit(code)
