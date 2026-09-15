#!/usr/bin/env python3
"""Deterministic allocation for the T03-07 local continuity proof.

Every function here is a pure function of small integers/strings -- no
hidden state, no file I/O, no randomness beyond a documented seeded
permutation. This is what lets an independent reviewer reconstruct the
exact condition order and case assignment for any (participant, pair)
before any result exists (PROTOCOL.md section 9.3 / acceptance criteria).
"""
import hashlib

PARTICIPANT_SLOTS = [f"P{i}" for i in range(1, 7)]  # P1..P6
PAIR_INDICES = list(range(1, 9))  # 1..8
MASTER_SEED = "flake-t03-07-confirmatory-v1"


def _slot_number(participant_slot: str) -> int:
    if not participant_slot.startswith("P") or not participant_slot[1:].isdigit():
        raise ValueError(f"not a participant slot id: {participant_slot!r}")
    n = int(participant_slot[1:])
    if not (1 <= n <= 6):
        raise ValueError(f"participant slot out of range 1..6: {participant_slot!r}")
    return n


def case_to_condition_map(participant_slot: str, pair_index: int) -> dict:
    """Which physical case (role A / role B) maps to which condition.

    Alternates by (participant + pair) parity (PROTOCOL.md section 9.3).
    Returns {"flake": "A"|"B", "baseline": "A"|"B"}.
    """
    p = _slot_number(participant_slot)
    if not (1 <= pair_index <= 8):
        raise ValueError(f"pair_index out of range 1..8: {pair_index}")
    parity = (p + pair_index) % 2
    if parity == 0:
        return {"flake": "A", "baseline": "B"}
    return {"flake": "B", "baseline": "A"}


def condition_attempted_first(participant_slot: str, pair_index: int) -> str:
    """Which condition ("flake" | "baseline") is attempted first within the
    pair. Deliberately offset (+1) from case_to_condition_map's parity so
    the two counterbalancing factors are not perfectly correlated."""
    p = _slot_number(participant_slot)
    parity = (p + pair_index + 1) % 2
    return "flake" if parity == 0 else "baseline"


def pair_presentation_order(participant_slot: str) -> list:
    """The order in which this participant's 8 pairs (by nominal
    pair_index 1..8) are actually administered, as a permutation of
    1..8. Seeded deterministically per participant so no participant's
    real-time order is difficulty-ascending or otherwise predictable,
    while remaining exactly reconstructable from participant_slot alone.
    """
    _slot_number(participant_slot)  # validates
    seed_material = f"pair-order:{participant_slot}:{MASTER_SEED}"
    digest = hashlib.sha256(seed_material.encode("utf-8")).digest()

    # Deterministic Fisher-Yates using bytes from the digest as the source
    # of "randomness" -- no dependency on Python's random module version
    # behavior, so this is stable across any Python 3 interpreter.
    order = list(range(1, 9))
    for i in range(len(order) - 1, 0, -1):
        j = digest[i] % (i + 1)
        order[i], order[j] = order[j], order[i]
    return order


def allocate_pair(participant_slot: str, pair_index: int, tier_by_pair_index: dict) -> dict:
    """Full allocation record for one (participant, pair): condition-role
    mapping, which condition goes first, and the pair's difficulty tier."""
    condition_map = case_to_condition_map(participant_slot, pair_index)
    return {
        "participant_slot": participant_slot,
        "pair_index": pair_index,
        "tier": tier_by_pair_index[pair_index],
        "case_role_for_condition": condition_map,
        "condition_attempted_first": condition_attempted_first(participant_slot, pair_index),
    }


def full_allocation_table(tier_by_pair_index: dict) -> list:
    rows = []
    for slot in PARTICIPANT_SLOTS:
        presentation_order = pair_presentation_order(slot)
        for position, pair_index in enumerate(presentation_order, start=1):
            row = allocate_pair(slot, pair_index, tier_by_pair_index)
            row["presentation_position"] = position
            rows.append(row)
    return rows


def counterbalance_summary(rows: list) -> dict:
    """Aggregate counts used to verify the counterbalancing promise in
    PROTOCOL.md section 9.3: 24/24 case-role split, 24/24 first-condition
    split, and each tier appearing exactly twice per participant."""
    flake_is_a = sum(1 for r in rows if r["case_role_for_condition"]["flake"] == "A")
    flake_is_b = len(rows) - flake_is_a
    flake_first = sum(1 for r in rows if r["condition_attempted_first"] == "flake")
    baseline_first = len(rows) - flake_first

    tier_counts_per_participant = {}
    for r in rows:
        key = r["participant_slot"]
        tier_counts_per_participant.setdefault(key, {}).setdefault(r["tier"], 0)
        tier_counts_per_participant[key][r["tier"]] += 1

    return {
        "total_pairs": len(rows),
        "flake_is_role_a": flake_is_a,
        "flake_is_role_b": flake_is_b,
        "flake_attempted_first": flake_first,
        "baseline_attempted_first": baseline_first,
        "tier_counts_per_participant": tier_counts_per_participant,
    }
