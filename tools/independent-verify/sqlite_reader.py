"""Raw reader #1 for the T02-07 independent verifier: reads
`canonical.sqlite` directly with Python's built-in `sqlite3` module.

Independence: no Flake binary, no Flake library, no Rust source is
imported or executed by this file. Every check here is re-derived from
`docs/formats/format-2-canonical-sqlite.md`'s own published text -- the
schema-recognition rules, the pragma values, the `payload_sha256`
definition and the `resulting_head_hash` chain formula are all
transcribed from that document, not from reading `src/canonical.rs`'s
implementation.
"""
import hashlib
import json
import sqlite3
from pathlib import Path

EXPECTED_TABLES = ["canonical_vault", "command", "current_object", "revision"]


class SqliteFormatError(Exception):
    pass


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _connect_readonly(db_path: Path) -> sqlite3.Connection:
    uri = f"file:{db_path.as_posix()}?mode=ro"
    conn = sqlite3.connect(uri, uri=True)
    conn.row_factory = sqlite3.Row
    return conn


def check_schema_recognition(conn: sqlite3.Connection) -> None:
    """Independently re-run the exact schema-recognition rules the format
    document specifies -- not "does it open", but "is this exactly the
    documented schema"."""
    tables = sorted(
        r[0] for r in conn.execute(
            "SELECT name FROM sqlite_master WHERE type='table'"
        ).fetchall()
    )
    if tables != EXPECTED_TABLES:
        raise SqliteFormatError(
            f"unexpected table set: {tables}, expected exactly {EXPECTED_TABLES}"
        )
    page_size = conn.execute("PRAGMA page_size").fetchone()[0]
    if page_size != 4096:
        raise SqliteFormatError(f"unexpected page_size {page_size}, expected 4096")


def read_canonical_vault(conn: sqlite3.Connection) -> dict:
    rows = conn.execute(
        "SELECT singleton, vault_id, schema_version, min_reader_capability, "
        "created_by_version, created_at, transaction_head_seq, transaction_head_hash "
        "FROM canonical_vault"
    ).fetchall()
    if len(rows) != 1 or rows[0]["singleton"] != 1:
        raise SqliteFormatError(
            f"canonical_vault must have exactly one row with singleton=1, got {len(rows)}"
        )
    return dict(rows[0])


def read_revision_envelopes(conn: sqlite3.Connection) -> list[dict]:
    """Every revision row, oldest first, independently re-hashing
    `payload_sha256` from the stored payload bytes and refusing on
    mismatch (a defect this reader must catch, not paper over)."""
    rows = conn.execute(
        "SELECT revision_id, object_id, parent_revision_id, recorded_seq, "
        "recorded_at, actor, origin, payload, payload_sha256 "
        "FROM revision ORDER BY recorded_seq ASC"
    ).fetchall()
    envelopes = []
    for row in rows:
        payload_text = row["payload"]
        recomputed = sha256_hex(payload_text.encode("utf-8"))
        if recomputed != row["payload_sha256"]:
            raise SqliteFormatError(
                f"payload_sha256 mismatch for revision {row['revision_id']}: "
                f"stored={row['payload_sha256']} recomputed={recomputed}"
            )
        try:
            payload = json.loads(payload_text)
        except json.JSONDecodeError as e:
            raise SqliteFormatError(
                f"revision {row['revision_id']} payload is not valid JSON: {e}"
            ) from e
        kind = payload.get("kind")
        if not kind:
            raise SqliteFormatError(
                f"revision {row['revision_id']} payload has no 'kind' field"
            )
        envelopes.append({
            "object_id": row["object_id"],
            "revision_id": row["revision_id"],
            "parent_revision_id": row["parent_revision_id"],
            "recorded_seq": row["recorded_seq"],
            "recorded_at": row["recorded_at"],
            "actor": row["actor"],
            "origin": row["origin"],
            "kind": kind,
            "payload": payload,
        })
    return envelopes


def read_current_object(conn: sqlite3.Connection) -> dict:
    rows = conn.execute(
        "SELECT object_id, current_revision_id, tombstoned FROM current_object"
    ).fetchall()
    return {r["object_id"]: {"current_revision_id": r["current_revision_id"], "tombstoned": r["tombstoned"]} for r in rows}


def verify_current_object_pointers(envelopes: list[dict], current_object: dict) -> None:
    """current_object's pointer for every object must equal the
    highest-recorded_seq revision this reader independently derived --
    proves `current_object` is not silently stale or wrong."""
    latest_by_object: dict[str, dict] = {}
    for env in envelopes:
        prior = latest_by_object.get(env["object_id"])
        if prior is None or env["recorded_seq"] > prior["recorded_seq"]:
            latest_by_object[env["object_id"]] = env
    if set(latest_by_object) != set(current_object):
        raise SqliteFormatError(
            "current_object object set does not match revision table's object set: "
            f"{set(latest_by_object) ^ set(current_object)}"
        )
    for object_id, latest in latest_by_object.items():
        pointer = current_object[object_id]["current_revision_id"]
        if pointer != latest["revision_id"]:
            raise SqliteFormatError(
                f"current_object[{object_id}] points at {pointer} but the highest "
                f"recorded_seq revision independently derived is {latest['revision_id']}"
            )


def read_command_chain(conn: sqlite3.Connection) -> list[dict]:
    rows = conn.execute(
        "SELECT command_id, input_digest, actor, recorded_seq, recorded_at, "
        "previous_head_seq, previous_head_hash, object_id, revision_id, "
        "resulting_head_seq, resulting_head_hash FROM command ORDER BY recorded_seq ASC"
    ).fetchall()
    return [dict(r) for r in rows]


def verify_head_hash_chain(commands: list[dict], vault_head_seq: int, vault_head_hash) -> None:
    """Independently recompute the documented
    `sha256("flake-canonical-tx-v1|" + previous_head_hash_or_empty + "|" +
    revision_id + "|" + input_digest)` chain and confirm it links seq-to-seq
    and matches the stored `canonical_vault` head. `input_digest` itself is
    trusted as stored here (the command table does not persist
    `target_kind`/`expected_revision_id`, so an outside reader cannot
    independently recompute `input_digest` from the documented algorithm
    without that additional undocumented data -- this is a real, explicitly
    disclosed limitation, not glossed over). What IS independently proven:
    the hash-chain-of-input-digests algorithm itself is reproducible by an
    outside tool, and the stored chain is internally consistent end to end.
    """
    prev_hash = ""
    prev_seq = 0
    for cmd in commands:
        if cmd["previous_head_seq"] != prev_seq:
            raise SqliteFormatError(
                f"command {cmd['command_id']} previous_head_seq={cmd['previous_head_seq']} "
                f"but chain so far is at seq {prev_seq}"
            )
        expected_prev_hash = prev_hash if prev_hash else None
        stored_prev_hash = cmd["previous_head_hash"]
        if (stored_prev_hash or None) != expected_prev_hash:
            raise SqliteFormatError(
                f"command {cmd['command_id']} previous_head_hash={stored_prev_hash!r} "
                f"but chain so far computed {expected_prev_hash!r}"
            )
        material = f"flake-canonical-tx-v1|{prev_hash}|{cmd['revision_id']}|{cmd['input_digest']}"
        recomputed_hash = sha256_hex(material.encode("utf-8"))
        if recomputed_hash != cmd["resulting_head_hash"]:
            raise SqliteFormatError(
                f"command {cmd['command_id']} resulting_head_hash mismatch: "
                f"stored={cmd['resulting_head_hash']} recomputed={recomputed_hash}"
            )
        prev_hash = cmd["resulting_head_hash"]
        prev_seq = cmd["resulting_head_seq"]
    if commands:
        if prev_seq != vault_head_seq or prev_hash != (vault_head_hash or ""):
            raise SqliteFormatError(
                f"recomputed final head (seq={prev_seq}, hash={prev_hash}) does not match "
                f"canonical_vault (seq={vault_head_seq}, hash={vault_head_hash})"
            )


def read_vault(db_path: Path) -> dict:
    """Top-level entry point: open canonical.sqlite read-only, run every
    independent check, and return a dict with the vault row, the semantic
    -model-ready revision envelope list, and the head-hash-chain proof
    result."""
    conn = _connect_readonly(db_path)
    try:
        check_schema_recognition(conn)
        vault_row = read_canonical_vault(conn)
        envelopes = read_revision_envelopes(conn)
        current_object = read_current_object(conn)
        verify_current_object_pointers(envelopes, current_object)
        commands = read_command_chain(conn)
        verify_head_hash_chain(
            commands, vault_row["transaction_head_seq"], vault_row["transaction_head_hash"]
        )
        return {
            "vault_row": vault_row,
            "envelopes": envelopes,
            "current_object": current_object,
            "command_count": len(commands),
            "head_hash_chain_verified": True,
        }
    finally:
        conn.close()
