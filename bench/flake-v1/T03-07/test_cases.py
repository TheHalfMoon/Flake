#!/usr/bin/env python3
"""Structural/schema tests for the T03-07 case bank (V01 static checks)."""
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()
sys.path.insert(0, str(HERE))

import generate_cases


def _load(path):
    return json.loads((HERE / path).read_text(encoding="utf-8"))


def test_confirmatory_case_count_and_uniqueness():
    data = _load("cases_confirmatory.json")
    assert data["case_count"] == 96, data["case_count"]
    assert len(data["cases"]) == 96
    print("PASS: test_confirmatory_case_count_and_uniqueness")


def test_confirmatory_covers_every_participant_pair_role():
    data = _load("cases_confirmatory.json")
    seen = set()
    for case_id, case in data["cases"].items():
        seen.add((case["participant_slot"], case["pair_index"], case["condition_role"]))
    expected = set()
    for slot in generate_cases.PARTICIPANT_SLOTS:
        for pair_index in generate_cases.PAIR_INDICES:
            for role in ("A", "B"):
                expected.add((slot, pair_index, role))
    assert seen == expected, f"missing or extra cells: {expected.symmetric_difference(seen)}"
    print("PASS: test_confirmatory_covers_every_participant_pair_role")


def test_equal_source_budget_within_each_pair():
    data = _load("cases_confirmatory.json")
    by_pair = {}
    for case in data["cases"].values():
        key = (case["participant_slot"], case["pair_index"])
        by_pair.setdefault(key, {})[case["condition_role"]] = case["total_source_bytes"]
    for key, roles in by_pair.items():
        assert roles["A"] == roles["B"], f"unequal source budget for {key}: {roles}"
    print("PASS: test_equal_source_budget_within_each_pair")


def test_tier_balance_per_participant():
    data = _load("cases_confirmatory.json")
    counts = {}
    for case in data["cases"].values():
        key = (case["participant_slot"], case["tier"])
        counts[key] = counts.get(key, 0) + 1
    for slot in generate_cases.PARTICIPANT_SLOTS:
        for tier in ("T1", "T2", "T3", "T4"):
            # each tier has 2 pairs, each pair has 2 roles -> 4 cases per (slot, tier)
            assert counts.get((slot, tier)) == 4, (slot, tier, counts.get((slot, tier)))
    print("PASS: test_tier_balance_per_participant")


def test_no_duplicate_content_anywhere():
    confirmatory = _load("cases_confirmatory.json")["cases"]
    dev = _load("cases_dev.json")["cases"]
    seen_bodies = {}
    for case_id, case in {**confirmatory, **dev}.items():
        key = tuple(doc["body"] for doc in case["sources"])
        assert key not in seen_bodies, f"duplicate case content: {case_id} and {seen_bodies[key]}"
        seen_bodies[key] = case_id
    print("PASS: test_no_duplicate_content_anywhere")


def test_dev_cases_marked_non_confirmatory():
    dev = _load("cases_dev.json")["cases"]
    assert len(dev) == 8
    for case in dev.values():
        assert case["confirmatory"] is False
    confirmatory_ids = set(_load("cases_confirmatory.json")["cases"].keys())
    dev_ids = set(dev.keys())
    assert confirmatory_ids.isdisjoint(dev_ids)
    print("PASS: test_dev_cases_marked_non_confirmatory")


def test_generation_is_deterministic():
    confirmatory = generate_cases.generate_confirmatory_cases()
    dev = generate_cases.generate_dev_cases()
    on_disk_confirmatory = _load("cases_confirmatory.json")
    on_disk_dev = _load("cases_dev.json")
    assert confirmatory == on_disk_confirmatory, "regenerated confirmatory cases differ from sealed file"
    assert dev == on_disk_dev, "regenerated dev cases differ from sealed file"
    print("PASS: test_generation_is_deterministic")


def test_gold_keys_present_for_conflict_tiers():
    data = _load("cases_confirmatory.json")
    for case in data["cases"].values():
        if case["tier"] in ("T3", "T4"):
            assert case["gold"]["planted_conflict_doc_ids"], case["case_id"]
        if case["tier"] == "T4":
            assert case["gold"]["high_consequence_trap"] is not None, case["case_id"]
        else:
            assert case["gold"]["high_consequence_trap"] is None, case["case_id"]
    print("PASS: test_gold_keys_present_for_conflict_tiers")


def run_all_tests():
    tests = [
        test_confirmatory_case_count_and_uniqueness,
        test_confirmatory_covers_every_participant_pair_role,
        test_equal_source_budget_within_each_pair,
        test_tier_balance_per_participant,
        test_no_duplicate_content_anywhere,
        test_dev_cases_marked_non_confirmatory,
        test_generation_is_deterministic,
        test_gold_keys_present_for_conflict_tiers,
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
