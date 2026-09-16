"""Independent (non-Flake-binary) reader for one export package directory,
for the T04-05 E2E test's "destination manifest/byte verification"
requirement. Reuses T02-07's already-independent, unmodified
`export_reader.py` unchanged -- no Flake binary, no Flake library, no Rust
source imported.

Usage: python independent_read_export.py <dest_root>
`dest_root` is the directory passed to `vault_export`/`export-run`
itself (not the `.fehrest-export` subdirectory it publishes into).
Prints one JSON object: {"record_count", "revision_count", "dangling_references"}.
"""
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[5]
sys.path.insert(0, str(REPO_ROOT / "tools" / "independent-verify"))

from export_reader import read_export  # noqa: E402


def main():
    dest_root = Path(sys.argv[1])
    export_dir = dest_root / ".fehrest-export"
    result = read_export(export_dir)
    print(
        json.dumps(
            {
                "record_count": len(result["envelopes"]),
                "dangling_references": result["dangling_references"],
                "manifest_kind": result["manifest"]["kind"],
                "manifest_integrity_root": result["manifest"]["integrity_root"],
            }
        )
    )


if __name__ == "__main__":
    main()
