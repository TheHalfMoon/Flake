#!/usr/bin/env python3
"""Structural checks that PROTOCOL.md and CONSENT.md actually contain every
element the T03-07 task contract requires (V01 static check). This does not
judge scientific quality; it catches an accidentally-omitted section."""
import sys
from pathlib import Path

HERE = Path(__file__).parent.resolve()

REQUIRED_PROTOCOL_TOPICS = [
    "research question", "target participant", "eligibility",
    "recruitment", "consent", "withdrawal", "privacy", "data minimization",
    "training", "condition order", "counterbalanc", "case assignment",
    "budget", "maintenance", "success", "failure behavior", "timeout",
    "exclusion", "missing-data", "missing data", "protocol-deviation",
    "protocol drift", "class-loss", "repair repeat", "analysis plan",
    "decision rout",
]

REQUIRED_CONSENT_TOPICS = [
    "optional", "voluntary", "informed", "withdraw", "data minimization",
    "retention", "what we record", "we do not record", "public",
]


def _read(name: str) -> str:
    return (HERE / name).read_text(encoding="utf-8").lower()


def test_protocol_covers_every_required_topic():
    text = _read("PROTOCOL.md")
    missing = [topic for topic in REQUIRED_PROTOCOL_TOPICS if topic.lower() not in text]
    assert not missing, f"PROTOCOL.md is missing required topics: {missing}"
    print("PASS: test_protocol_covers_every_required_topic")


def test_protocol_declares_exact_design_numbers():
    text = _read("PROTOCOL.md")
    for token in ["6 participants", "eight matched", "96 total attempts", "8 pairs",
                  "8-minute", "10-minute"]:
        assert token.lower() in text, f"PROTOCOL.md missing design number: {token}"
    print("PASS: test_protocol_declares_exact_design_numbers")


def test_protocol_states_mechanical_routing():
    text = _read("PROTOCOL.md")
    for token in ["pass", "fail", "inconclusive"]:
        assert token in text
    print("PASS: test_protocol_states_mechanical_routing")


def test_protocol_forbids_model_and_paid_api():
    text = _read("PROTOCOL.md")
    assert "no model" in text or "no model/api" in text or "no model or paid api" in text
    print("PASS: test_protocol_forbids_model_and_paid_api")


def test_consent_covers_every_required_topic():
    text = _read("CONSENT.md")
    missing = [topic for topic in REQUIRED_CONSENT_TOPICS if topic.lower() not in text]
    assert not missing, f"CONSENT.md is missing required topics: {missing}"
    print("PASS: test_consent_covers_every_required_topic")


def test_consent_never_requires_sensitive_content():
    text = _read("CONSENT.md")
    assert "never required to use your own real project or any sensitive material" in text \
        or "not required to use your own real project" in text
    print("PASS: test_consent_never_requires_sensitive_content")


def run_all_tests():
    tests = [
        test_protocol_covers_every_required_topic,
        test_protocol_declares_exact_design_numbers,
        test_protocol_states_mechanical_routing,
        test_protocol_forbids_model_and_paid_api,
        test_consent_covers_every_required_topic,
        test_consent_never_requires_sensitive_content,
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
