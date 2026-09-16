"""Independent (non-Flake-binary) reader for one note object's latest
committed payload, for the T04-02 E2E test's "exact before/after canonical
payload comparison" requirement.

Reuses T02-07's already-independent `tools/independent-verify/sqlite_reader.py`
unchanged -- this file adds only a thin CLI wrapper, no new parsing logic,
so the independence property (no Flake binary, no Flake library, no Rust
source imported or executed) is inherited, not re-claimed from scratch.

Usage: python independent_read_note.py <canonical.sqlite path> <object_id>
Prints one JSON object: {"revision_id", "recorded_seq", "payload"} for the
highest recorded_seq envelope matching that object_id, or {"error": ...}
if none exists.
"""
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[5]
sys.path.insert(0, str(REPO_ROOT / "tools" / "independent-verify"))

from sqlite_reader import read_vault  # noqa: E402


def main():
    db_path = Path(sys.argv[1])
    object_id = sys.argv[2]
    result = read_vault(db_path)
    matches = [e for e in result["envelopes"] if e["object_id"] == object_id]
    if not matches:
        print(json.dumps({"error": f"no revision found for object_id {object_id}"}))
        return
    latest = max(matches, key=lambda e: e["recorded_seq"])
    print(
        json.dumps(
            {
                "revision_id": latest["revision_id"],
                "recorded_seq": latest["recorded_seq"],
                "payload": latest["payload"],
            }
        )
    )


if __name__ == "__main__":
    main()
