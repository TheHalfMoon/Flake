"""Client A -- reads one disclosure package and proposes a note edit.

Independence: written entirely from `docs/formats/agent-disclosure-protocol-v1.md`.
Does not import, call, or share any code with `src/disclosure.rs`,
`src/proposal.rs`, or `client_b.py`. Offline: no network, no model, no
account -- this file touches only the package file path it is given and
Python's standard library.

Usage: client_a.py <package.jsonl> <receipt_id> <out_proposal.json>
"""
import json
import sys


def read_package_lines(package_path):
    """Split the wire bytes into (header_dict, [item_dict, ...]) -- one
    JSON object per non-empty line, header first, per the protocol
    document's "Wire shape" section."""
    with open(package_path, "r", encoding="utf-8") as f:
        lines = [line for line in f.read().split("\n") if line.strip()]
    if not lines:
        raise ValueError("empty package: no header line")
    header = json.loads(lines[0])
    items = [json.loads(line) for line in lines[1:]]
    return header, items


def find_first_note(items):
    for item in items:
        if item.get("kind") == "note":
            return item
    raise ValueError("no disclosed item of kind 'note' in this package")


def build_note_edit_proposal(receipt_id, note_item, new_body, declared_agent):
    """Per the protocol document's "Part 2" -- a single JSON document, one
    operation, referencing the disclosed note's own object_id/revision_id
    exactly as read from the package (never guessed or re-derived)."""
    return {
        "receipt_id": receipt_id,
        "declared_agent": declared_agent,
        "declared_model": "none",
        "declared_tool": "interchange-client-a-fixture",
        "operations": [
            {
                "kind": "note_edit",
                "note_id": note_item["object_id"],
                "expected_revision_id": note_item["revision_id"],
                "body": new_body,
            }
        ],
    }


def main(argv):
    if len(argv) != 4:
        print(
            "usage: client_a.py <package.jsonl> <receipt_id> <out_proposal.json>",
            file=sys.stderr,
        )
        return 2
    package_path, receipt_id, out_path = argv[1], argv[2], argv[3]

    header, items = read_package_lines(package_path)
    note_item = find_first_note(items)
    new_body = "Edited by interchange client A, citing project_id=" + header["project_id"]
    proposal = build_note_edit_proposal(receipt_id, note_item, new_body, "interchange-client-a")

    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(proposal, f)

    print(
        f"client_a: proposed note_edit for {note_item['object_id']} "
        f"(from revision {note_item['revision_id']}) -> {out_path}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
