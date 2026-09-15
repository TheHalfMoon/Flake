"""Adversarial proposal cases for `T03-06`'s own acceptance criterion:
"Verify unknown declared identity, replay and unsupported protocol
behavior."

Each function submits one adversarial proposal directly through Flake's
real `propose-import`/`propose-accept` CLI (never Flake's own library
code from Python) and returns a small result dict. Called by
`run_interchange.py`, which owns the running vault/project/receipt these
functions operate against -- kept here as plain functions, not a class or
framework, per "keep executable fixtures minimal."
"""
import json
import subprocess


def _run(binary, args):
    proc = subprocess.run(
        [binary, *args], capture_output=True, text=True, timeout=30
    )
    return proc.returncode, proc.stdout.strip(), proc.stderr.strip()


def unknown_declared_identity(binary, vault, project_id, receipt_id, note_id, note_rev, proposal_path):
    """A well-formed proposal with no declared_agent/model/tool at all.
    Per the protocol document, admission must succeed and the identity
    fields must stay genuinely absent (never defaulted to a guessed
    value) -- verified by the caller reading the committed record back,
    not by this function's own stdout parsing.
    """
    proposal = {
        "receipt_id": receipt_id,
        "operations": [
            {
                "kind": "note_edit",
                "note_id": note_id,
                "expected_revision_id": note_rev,
                "body": "edited by an agent of unknown identity",
            }
        ],
    }
    with open(proposal_path, "w", encoding="utf-8") as f:
        json.dump(proposal, f)
    code, out, err = _run(
        binary,
        ["propose-import", "--vault", vault, "--project", project_id, "--file", proposal_path],
    )
    return {
        "case": "unknown_declared_identity",
        "expected": "admission succeeds (exit 0)",
        "exit_code": code,
        "passed": code == 0,
        "stdout": out,
        "stderr": err,
    }


def unsupported_operation_kind(binary, vault, project_id, receipt_id, proposal_path):
    """An operation `kind` not in the protocol's closed four-variant set.
    Must be refused at admission -- the whole proposal, not just that one
    operation -- and must never become a Pending record.
    """
    proposal = {
        "receipt_id": receipt_id,
        "operations": [{"kind": "delete_everything", "target": "the whole vault"}],
    }
    with open(proposal_path, "w", encoding="utf-8") as f:
        json.dump(proposal, f)
    code, out, err = _run(
        binary,
        ["propose-import", "--vault", vault, "--project", project_id, "--file", proposal_path],
    )
    return {
        "case": "unsupported_operation_kind",
        "expected": "admission refused (nonzero exit, no Pending record created)",
        "exit_code": code,
        "passed": code != 0,
        "stdout": out,
        "stderr": err,
    }


def duplicate_delivery(binary, vault, project_id, receipt_id, note_id, note_rev, proposal_path):
    """The exact same well-formed proposal bytes, submitted twice. Per the
    protocol document, this produces two independent Pending proposals
    (never deduplicated, never an error) -- the caller checks object-id
    distinctness independently against `canonical.sqlite`.
    """
    proposal = {
        "receipt_id": receipt_id,
        "declared_agent": "duplicate-delivery-fixture",
        "operations": [
            {
                "kind": "note_edit",
                "note_id": note_id,
                "expected_revision_id": note_rev,
                "body": "duplicate delivery candidate body",
            }
        ],
    }
    with open(proposal_path, "w", encoding="utf-8") as f:
        json.dump(proposal, f)
    code1, out1, err1 = _run(
        binary,
        ["propose-import", "--vault", vault, "--project", project_id, "--file", proposal_path],
    )
    code2, out2, err2 = _run(
        binary,
        ["propose-import", "--vault", vault, "--project", project_id, "--file", proposal_path],
    )
    return {
        "case": "duplicate_delivery",
        "expected": "both admissions succeed independently (exit 0 twice)",
        "exit_code": (code1, code2),
        "passed": code1 == 0 and code2 == 0,
        "stdout": (out1, out2),
        "stderr": (err1, err2),
    }
