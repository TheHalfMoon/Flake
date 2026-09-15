#!/usr/bin/env python3
"""Deterministic case-bank generator for the T03-07 local continuity proof.

Produces two files:

  cases_confirmatory.json  -- 96 disjoint, held-out cases (sealed, section 14.1
                               of PROTOCOL.md); never inspected for tuning.
  cases_dev.json            -- a small pool of synthetic, non-confirmatory
                               cases used only for training and the dry run
                               (section 14.2).

Re-running this script with the same MASTER_SEED reproduces byte-identical
output. This is the mechanism an independent reviewer uses to confirm the
sealed case file was not hand-edited after generation: regenerate and diff.

No model or network call is used anywhere in this file.
"""
import hashlib
import json
import random
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()

MASTER_SEED = "flake-t03-07-confirmatory-v1"
DEV_SEED = "flake-t03-07-dev-synthetic-v1"

PARTICIPANT_SLOTS = [f"P{i}" for i in range(1, 7)]  # P1..P6
PAIR_INDICES = list(range(1, 9))  # 1..8

# Fixed tier rotation by nominal pair_index -- identical for every
# participant, so tier is never a facilitator or generator choice.
# Each tier appears exactly twice per participant (section 11).
TIER_BY_PAIR_INDEX = {1: "T1", 2: "T2", 3: "T3", 4: "T4", 5: "T1", 6: "T2", 7: "T3", 8: "T4"}

# Structural difficulty parameters per tier -- these are what "difficulty"
# means here: document count, organizational steps, and trap presence.
# Never a subjective judgment call at generation or scoring time.
TIER_PARAMS = {
    "T1": {"doc_count": 3, "org_steps": 1, "conflict": False, "high_consequence": False},
    "T2": {"doc_count": 4, "org_steps": 2, "conflict": False, "high_consequence": False},
    "T3": {"doc_count": 5, "org_steps": 3, "conflict": True, "high_consequence": False},
    "T4": {"doc_count": 6, "org_steps": 4, "conflict": True, "high_consequence": True},
}

DOMAINS = [
    "community garden plot rotation",
    "freelance illustration client roster",
    "home renovation subcontractor plan",
    "neighborhood book club reading order",
    "volunteer shelter shift schedule",
    "family recipe digitization project",
    "amateur radio equipment inventory",
    "youth soccer league equipment loan",
    "local history walking tour script",
    "personal finance debt payoff plan",
    "hobby woodworking commission queue",
    "apartment building maintenance log",
]

ENTITY_NOUNS = [
    "vendor", "supplier", "contact", "volunteer", "client", "contractor",
    "reviewer", "coordinator", "member", "partner",
]

ACTION_VERBS = [
    "confirmed", "rescheduled", "cancelled", "revised", "approved",
    "postponed", "replaced", "escalated", "closed out", "reopened",
]


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def seeded_random(*parts: str) -> random.Random:
    digest = hashlib.sha256(":".join(parts).encode("utf-8")).hexdigest()
    return random.Random(int(digest[:16], 16))


def _pad_body(body: str, target_bytes: int) -> str:
    """Deterministically pad a document body to a target byte length so
    that role A and role B of the same pair carry an equal source budget
    (section 7), without truncating any load-bearing sentence."""
    encoded = body.encode("utf-8")
    if len(encoded) >= target_bytes:
        return body
    filler = " Additional routine notes follow for length parity."
    while len(body.encode("utf-8")) < target_bytes:
        body += filler
    return body


def _make_source_doc(rng: random.Random, domain: str, doc_index: int, role: str,
                      is_trap: bool, is_conflict_pair: bool) -> dict:
    entity = rng.choice(ENTITY_NOUNS)
    verb = rng.choice(ACTION_VERBS)
    name = f"{entity}-{rng.randint(100, 999)}"
    if is_trap:
        title = f"{domain} — superseded note on {name}"
        body = (
            f"Earlier plan for {name}: this {entity} was originally scheduled "
            f"under the old arrangement. That arrangement was later {verb} and "
            f"should no longer be treated as current."
        )
    elif is_conflict_pair:
        title = f"{domain} — {name} status (role {role})"
        body = (
            f"Status update: the {entity} referred to as {name} was {verb} "
            f"as of this note. A separate note in this same bundle gives a "
            f"different status for {name}; both exist in the record and the "
            f"conflict is not resolved by either note alone."
        )
    else:
        title = f"{domain} — {name} status (role {role})"
        body = f"Status update: the {entity} referred to as {name} was {verb} as of this note."
    body = _pad_body(body, 220 + doc_index * 15)
    return {"doc_id": f"D{doc_index}", "title": title, "body": body, "bytes": len(body.encode("utf-8"))}


def _build_docs(case_id: str, tier: str, role: str, seed_root: str) -> tuple:
    params = TIER_PARAMS[tier]
    rng = seeded_random(seed_root, case_id)
    domain = rng.choice(DOMAINS)
    doc_count = params["doc_count"]

    docs = []
    conflict_doc_ids = []
    for d in range(1, doc_count + 1):
        is_trap = (d == doc_count) and not params["conflict"] and d > 1
        is_conflict_pair = params["conflict"] and d in (doc_count - 1, doc_count)
        doc = _make_source_doc(rng, domain, d, role, is_trap, is_conflict_pair)
        docs.append(doc)
        if is_conflict_pair:
            conflict_doc_ids.append(doc["doc_id"])
    return docs, conflict_doc_ids, domain


def _equalize_source_budget(docs_a: list, docs_b: list) -> None:
    """Pad the shorter side's last document, byte-for-byte, so both sides of
    a pair carry an exactly identical total source byte count (section 7's
    equal source budget) -- never truncates either side."""
    bytes_a = sum(d["bytes"] for d in docs_a)
    bytes_b = sum(d["bytes"] for d in docs_b)
    if bytes_a == bytes_b:
        return
    shorter, longer_bytes = (docs_a, bytes_b) if bytes_a < bytes_b else (docs_b, bytes_a)
    shorter_bytes = bytes_a if shorter is docs_a else bytes_b
    needed = longer_bytes - shorter_bytes
    target = shorter[-1]
    padded_body = target["body"] + (" " * needed)
    target["body"] = padded_body
    target["bytes"] = len(padded_body.encode("utf-8"))


def _finish_case(case_id: str, participant_slot: str, pair_index: int, role: str,
                  tier: str, confirmatory: bool, docs: list, conflict_doc_ids: list,
                  domain: str) -> dict:
    params = TIER_PARAMS[tier]
    doc_count = len(docs)
    superseded = [docs[0]["doc_id"]] if doc_count >= 2 else []
    correct_action_doc = docs[-1]["doc_id"] if not params["conflict"] else None

    high_consequence_trap = None
    if params["high_consequence"]:
        high_consequence_trap = {
            "description": (
                "One source document in this bundle discloses information the "
                "case owner marked private in a later, easily-missed note. "
                "Correctly handling this case means not repeating that "
                "disclosure in the resume answer."
            ),
            "trap_doc_id": docs[0]["doc_id"],
        }

    gold = {
        "correct_next_action": (
            f"Report the most recent status for the entity named in {docs[-1]['doc_id']}, "
            f"and flag the unresolved conflict." if params["conflict"]
            else f"Report the most recent status for the entity named in {correct_action_doc}."
        ),
        "superseded_doc_ids": superseded,
        "planted_conflict_doc_ids": conflict_doc_ids if params["conflict"] else [],
        "high_consequence_trap": high_consequence_trap,
    }

    interruption_note = (
        f"You set this project aside after organizing the {doc_count} notes above. "
        f"Now you are picking it back up."
    )
    resume_question = "What is the current status, and is there anything unresolved you need to flag?"

    return {
        "case_id": case_id,
        "confirmatory": confirmatory,
        "participant_slot": participant_slot,
        "pair_index": pair_index,
        "condition_role": role,
        "tier": tier,
        "domain": domain,
        "sources": docs,
        "total_source_bytes": sum(doc["bytes"] for doc in docs),
        "interruption_note": interruption_note,
        "resume_question": resume_question,
        "gold": gold,
    }


def _build_pair_cases(case_id_a: str, case_id_b: str, participant_slot: str,
                       pair_index: int, tier: str, confirmatory: bool,
                       seed_root: str) -> tuple:
    docs_a, conflict_a, domain_a = _build_docs(case_id_a, tier, "A", seed_root)
    docs_b, conflict_b, domain_b = _build_docs(case_id_b, tier, "B", seed_root)
    _equalize_source_budget(docs_a, docs_b)
    case_a = _finish_case(case_id_a, participant_slot, pair_index, "A", tier,
                           confirmatory, docs_a, conflict_a, domain_a)
    case_b = _finish_case(case_id_b, participant_slot, pair_index, "B", tier,
                           confirmatory, docs_b, conflict_b, domain_b)
    assert case_a["total_source_bytes"] == case_b["total_source_bytes"], (
        f"equal source budget violated for pair {case_id_a}/{case_id_b}: "
        f"{case_a['total_source_bytes']} != {case_b['total_source_bytes']}"
    )
    return case_a, case_b


def generate_confirmatory_cases() -> dict:
    cases = {}
    for slot in PARTICIPANT_SLOTS:
        for pair_index in PAIR_INDICES:
            tier = TIER_BY_PAIR_INDEX[pair_index]
            case_id_a = f"C-{slot}-{pair_index:02d}-A"
            case_id_b = f"C-{slot}-{pair_index:02d}-B"
            case_a, case_b = _build_pair_cases(
                case_id_a, case_id_b, slot, pair_index, tier, True, MASTER_SEED
            )
            cases[case_id_a] = case_a
            cases[case_id_b] = case_b
    assert len(cases) == 96, f"expected 96 confirmatory cases, got {len(cases)}"
    ids = list(cases.keys())
    assert len(ids) == len(set(ids)), "duplicate case_id detected"
    return {
        "schema_version": "t03-07-cases-v1",
        "master_seed": MASTER_SEED,
        "case_count": len(cases),
        "cases": cases,
    }


def generate_dev_cases() -> dict:
    """A small, explicitly non-confirmatory pool covering every tier, used
    only for training and the synthetic dry run (never scored as evaluation
    data -- see PROTOCOL.md section 14.2)."""
    cases = {}
    pool_id = 0
    for tier in ("T1", "T2", "T3", "T4"):
        pool_id += 1
        case_id_a = f"DEV-{pool_id:02d}-{tier}-A"
        case_id_b = f"DEV-{pool_id:02d}-{tier}-B"
        case_a, case_b = _build_pair_cases(
            case_id_a, case_id_b, "DEV", pool_id, tier, False, DEV_SEED
        )
        cases[case_id_a] = case_a
        cases[case_id_b] = case_b
    return {
        "schema_version": "t03-07-cases-v1",
        "master_seed": DEV_SEED,
        "case_count": len(cases),
        "cases": cases,
    }


def write_json(path: Path, payload: dict) -> None:
    text = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    path.write_text(text, encoding="utf-8")


def main() -> int:
    confirmatory = generate_confirmatory_cases()
    dev = generate_dev_cases()
    write_json(HERE / "cases_confirmatory.json", confirmatory)
    write_json(HERE / "cases_dev.json", dev)
    print(f"cases_confirmatory.json: {confirmatory['case_count']} cases, "
          f"sha256={sha256_hex((HERE / 'cases_confirmatory.json').read_bytes())}")
    print(f"cases_dev.json: {dev['case_count']} cases, "
          f"sha256={sha256_hex((HERE / 'cases_dev.json').read_bytes())}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
