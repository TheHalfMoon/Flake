#!/usr/bin/env python3
"""Tests for the deterministic allocation mechanism (V01/V15)."""
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()
sys.path.insert(0, str(HERE))

import allocation
from generate_cases import TIER_BY_PAIR_INDEX


def test_case_role_map_is_deterministic_and_balanced():
    rows = allocation.full_allocation_table(TIER_BY_PAIR_INDEX)
    summary = allocation.counterbalance_summary(rows)
    assert summary["total_pairs"] == 48
    assert summary["flake_is_role_a"] == 24
    assert summary["flake_is_role_b"] == 24
    assert summary["flake_attempted_first"] == 24
    assert summary["baseline_attempted_first"] == 24
    print("PASS: test_case_role_map_is_deterministic_and_balanced")


def test_tier_balance_two_per_participant():
    rows = allocation.full_allocation_table(TIER_BY_PAIR_INDEX)
    summary = allocation.counterbalance_summary(rows)
    for slot, tiers in summary["tier_counts_per_participant"].items():
        assert tiers == {"T1": 2, "T2": 2, "T3": 2, "T4": 2}, (slot, tiers)
    print("PASS: test_tier_balance_two_per_participant")


def test_presentation_order_is_a_permutation_of_1_to_8():
    for slot in allocation.PARTICIPANT_SLOTS:
        order = allocation.pair_presentation_order(slot)
        assert sorted(order) == list(range(1, 9)), (slot, order)
    print("PASS: test_presentation_order_is_a_permutation_of_1_to_8")


def test_presentation_order_differs_across_participants():
    orders = {slot: tuple(allocation.pair_presentation_order(slot)) for slot in allocation.PARTICIPANT_SLOTS}
    # Not a strict requirement of the protocol, but a sanity check that the
    # per-participant seed actually varies the order rather than collapsing
    # to one fixed sequence for everyone.
    assert len(set(orders.values())) > 1, "all participants got the identical order"
    print("PASS: test_presentation_order_differs_across_participants")


def test_reconstructable_from_two_integers_alone():
    # An independent reviewer must be able to reconstruct the exact
    # allocation for (participant, pair) from those two values alone,
    # with no other state.
    a1 = allocation.allocate_pair("P3", 5, TIER_BY_PAIR_INDEX)
    a2 = allocation.allocate_pair("P3", 5, TIER_BY_PAIR_INDEX)
    assert a1 == a2
    print("PASS: test_reconstructable_from_two_integers_alone")


def test_full_table_is_deterministic_across_calls():
    rows1 = allocation.full_allocation_table(TIER_BY_PAIR_INDEX)
    rows2 = allocation.full_allocation_table(TIER_BY_PAIR_INDEX)
    assert rows1 == rows2
    print("PASS: test_full_table_is_deterministic_across_calls")


def test_invalid_participant_slot_rejected():
    try:
        allocation.case_to_condition_map("P7", 1)
        raise AssertionError("expected ValueError for out-of-range slot")
    except ValueError:
        pass
    try:
        allocation.case_to_condition_map("X1", 1)
        raise AssertionError("expected ValueError for malformed slot")
    except ValueError:
        pass
    print("PASS: test_invalid_participant_slot_rejected")


def run_all_tests():
    tests = [
        test_case_role_map_is_deterministic_and_balanced,
        test_tier_balance_two_per_participant,
        test_presentation_order_is_a_permutation_of_1_to_8,
        test_presentation_order_differs_across_participants,
        test_reconstructable_from_two_integers_alone,
        test_full_table_is_deterministic_across_calls,
        test_invalid_participant_slot_rejected,
    ]
    passed = 0
    failed = 0
    errors = []
    for test in tests:
        try:
            test()
            passed += 1
        except AssertionError as e:
            failed += 1
            errors.append(f"FAIL: {test.__name__}: {e}")
        except Exception as e:
            failed += 1
            errors.append(f"ERROR: {test.__name__}: {e}")
    print(f"\n{'=' * 60}\nResults: {passed} passed, {failed} failed\n{'=' * 60}")
    if errors:
        for e in errors:
            print(e)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(run_all_tests())
