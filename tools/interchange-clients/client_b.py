"""Client B -- reads a (second) disclosure package and continues with a
compatible, different proposal: a draft decision citing the note content
client A left behind.

Independence: written entirely from `docs/formats/agent-disclosure-protocol-v1.md`,
deliberately in a different (class-based) style from `client_a.py` -- no
code, helper function, or constant is shared between the two files, and
neither imports `src/disclosure.rs`/`src/proposal.rs`. Offline: no
network, no model, no account.

Usage: client_b.py <package.jsonl> <receipt_id> <out_proposal.json>
"""
import json
import sys


class DisclosurePackage:
    """A tiny, from-scratch line-oriented reader -- intentionally not the
    same shape as client_a.py's plain function pair, to keep the two
    clients' own parsing logic genuinely independent."""

    def __init__(self, path):
        with open(path, encoding="utf-8") as handle:
            raw_lines = [ln for ln in handle.read().splitlines() if ln.strip()]
        if not raw_lines:
            raise ValueError("package has no header line")
        parsed = [json.loads(ln) for ln in raw_lines]
        self.header = parsed[0]
        self.items = parsed[1:]

    def items_of_kind(self, kind):
        return [it for it in self.items if it.get("kind") == kind]


def make_draft_decision(receipt_id, decision_key, statement, rationale, declared_agent):
    return {
        "receipt_id": receipt_id,
        "declared_agent": declared_agent,
        # declared_model / declared_tool intentionally absent here -- the
        # protocol document requires absence to mean openly "Unknown", not
        # a guessed default; client B exercises that path deliberately.
        "operations": [
            {
                "kind": "draft_decision",
                "decision_key": decision_key,
                "statement": statement,
                "rationale": rationale,
            }
        ],
    }


def main(argv):
    if len(argv) != 4:
        sys.stderr.write(
            "usage: client_b.py <package.jsonl> <receipt_id> <out_proposal.json>\n"
        )
        return 2
    package_path, receipt_id, out_path = argv[1], argv[2], argv[3]

    package = DisclosurePackage(package_path)
    notes = package.items_of_kind("note")
    if not notes:
        raise ValueError("no disclosed item of kind 'note' in this package")
    note = notes[0]

    proposal = make_draft_decision(
        receipt_id=receipt_id,
        decision_key="client-b-observation",
        statement="Client B observed the current note content and proposes a decision.",
        rationale=f"Read from disclosed note {note['object_id']}: {note['content'][:80]!r}",
        declared_agent="interchange-client-b",
    )

    with open(out_path, "w", encoding="utf-8") as handle:
        json.dump(proposal, handle)

    print(f"client_b: proposed draft_decision citing note {note['object_id']} -> {out_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
