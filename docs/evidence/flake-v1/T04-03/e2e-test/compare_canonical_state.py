"""Content-equivalence comparator for the T04-03 "produces identical
canonical state to CLI" acceptance clause.

Reuses T02-07's already-independent, unmodified `sqlite_reader.py` (no
Flake binary, no Flake library, no Rust source imported) to read both a
CLI-driven vault and a desktop-driven vault, then compares their content
*ignoring* object_id/revision_id/timestamps -- those are UUIDv7/wall-clock
values and are never expected to match byte-for-byte between two
independently created vaults. What must match is the actual record
content: names, titles, bodies, states, lifecycles, relation types/notes.

Usage: python compare_canonical_state.py <cli_vault_db> <desktop_vault_db>
Exits 0 and prints {"match": true} if every kind's normalized content list
is equal (as multisets); exits 1 and prints the first mismatch otherwise.
"""
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[5]
sys.path.insert(0, str(REPO_ROOT / "tools" / "independent-verify"))

from sqlite_reader import read_vault  # noqa: E402


def latest_payloads(db_path: Path):
    result = read_vault(db_path)
    latest_by_object = {}
    for e in result["envelopes"]:
        oid = e["object_id"]
        if oid not in latest_by_object or e["recorded_seq"] > latest_by_object[oid]["recorded_seq"]:
            latest_by_object[oid] = e
    return list(latest_by_object.values())


def normalize(envelopes):
    """One content tuple per kind, with every ID/revision/timestamp field
    dropped -- exactly what "identical canonical state" can honestly mean
    when object IDs are independently generated per vault."""
    out = {"project": [], "note": [], "action": [], "decision": [], "relation": []}
    for e in envelopes:
        p = e["payload"]
        kind = e["kind"]
        if kind == "project":
            out["project"].append((p.get("name"), p.get("description"), p.get("active")))
        elif kind == "note":
            out["note"].append((p.get("title"), p.get("body"), p.get("tombstoned")))
        elif kind == "action":
            # The desktop-only failure-conflict probe action is deliberately
            # excluded here rather than replicated into the CLI vault too --
            # it exists only to trigger a real `expected revision conflict`
            # (see run-e2e-test.mjs's own comment on why), and comparing it
            # would require giving the CLI arm a pointless identical action
            # just to satisfy this comparator, which would test nothing.
            if p.get("title") == "Conflict probe action (not part of the CLI comparison)":
                continue
            out["action"].append(
                (
                    p.get("title"),
                    p.get("body"),
                    p.get("state"),
                    tuple(sorted(p.get("dependency_ids") or [])),
                    p.get("completion_summary"),
                )
            )
        elif kind == "decision":
            out["decision"].append(
                (
                    p.get("decision_key"),
                    p.get("statement"),
                    p.get("rationale"),
                    p.get("basis"),
                    p.get("verification"),
                    p.get("lifecycle"),
                    p.get("valid_from"),
                    p.get("valid_to"),
                )
            )
        elif kind == "relation":
            out["relation"].append((p.get("relation_type"), p.get("note")))
    for k in out:
        out[k].sort(key=lambda t: json.dumps(t, sort_keys=True))
    return out


def main():
    cli_db = Path(sys.argv[1])
    desktop_db = Path(sys.argv[2])
    cli_norm = normalize(latest_payloads(cli_db))
    desktop_norm = normalize(latest_payloads(desktop_db))

    mismatches = {}
    for kind in cli_norm:
        if cli_norm[kind] != desktop_norm[kind]:
            mismatches[kind] = {"cli": cli_norm[kind], "desktop": desktop_norm[kind]}

    if mismatches:
        print(json.dumps({"match": False, "mismatches": mismatches}, indent=2))
        sys.exit(1)
    else:
        counts = {k: len(v) for k, v in cli_norm.items()}
        print(json.dumps({"match": True, "counts": counts}))
        sys.exit(0)


if __name__ == "__main__":
    main()
