"""T05-02 D6: independently verify one post-kill VM disk snapshot.

Reads a `canonical.sqlite` copy pulled off the guest disk image
*after* a forced VM kill, using the same independent reader every
other flake-v1 task already relies on
(`tools/independent-verify/sqlite_reader.py`) -- no Flake binary or
library is imported here. Prints one JSON object to stdout; never
raises past its own `try` -- a corrupt/missing database after a kill
is an expected, recordable outcome, not a script crash.
"""

import json
import sqlite3
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[5]
sys.path.insert(0, str(REPO_ROOT / "tools" / "independent-verify"))

from sqlite_reader import read_vault, SqliteFormatError  # noqa: E402


def roll_forward_hot_journal_if_present(db_path: Path) -> None:
    """A kill mid-transaction can leave a `<db>-journal` file next to the
    copied database (Flake uses `journal_mode=DELETE` -- a rollback
    journal, not WAL). `sqlite_reader.py`'s own reader deliberately opens
    strictly read-only (`?mode=ro`) so it can never touch what it reads --
    correct for reading a live vault, but SQLite cannot roll back a hot
    journal on a connection opened with that flag (observed directly:
    `OperationalError: attempt to write a readonly database` on some
    SQLite builds when a journal is pending). This is never called on a
    live vault -- only on this script's own disposable copy, pulled off
    the guest disk purely for inspection -- so a normal read-write open
    here is safe: it lets SQLite perform its own standard crash recovery
    once, leaving a clean database for the read-only reader to open
    afterward, exactly as a real Flake process's own open would."""
    journal_path = db_path.with_name(db_path.name + "-journal")
    if not journal_path.exists():
        return
    conn = sqlite3.connect(str(db_path))
    try:
        conn.execute("PRAGMA quick_check")
    finally:
        conn.close()


def main() -> int:
    if len(sys.argv) != 2:
        print(json.dumps({"ok": False, "error": "usage: verify_fault_cycle.py <db-path>"}))
        return 1
    db_path = Path(sys.argv[1])
    result = {"ok": False, "db_exists": db_path.exists()}
    if not result["db_exists"]:
        result["error"] = "no canonical.sqlite present after this cycle's kill"
        print(json.dumps(result))
        return 0
    try:
        roll_forward_hot_journal_if_present(db_path)
        vault = read_vault(db_path)
    except SqliteFormatError as e:
        result["error"] = f"SqliteFormatError: {e}"
        print(json.dumps(result))
        return 0
    except Exception as e:  # noqa: BLE001 -- record, never crash the harness
        result["error"] = f"{type(e).__name__}: {e}"
        print(json.dumps(result))
        return 0

    result["ok"] = True
    result["head_hash_chain_verified"] = vault["head_hash_chain_verified"]
    result["command_count"] = vault["command_count"]
    result["current_object_count"] = len(vault["current_object"])
    result["vault_row"] = vault["vault_row"]
    print(json.dumps(result))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
