# Flake canonical format 2 — `canonical.sqlite`

**Status:** schema version 2, built by `T01-02` (create-only) and `T01-03`
(transaction/command admission). This document describes exactly what those
two tasks publish today. It is not a forward-looking schema for the
finished product: the typed `Project`/`Note`/`Action`/`Decision`/`Source`/...
records in plan §15 do not exist yet — `T01-03` admits one opaque UTF-8
payload per object ("a minimal record", its own objective wording), not a
typed model. Typed records are `P02`'s job once the command API exists to
populate them, each landing through its own reviewed schema-version bump,
not by silently widening what this document calls "recognized."

## On-disk layout

```text
<vault root>/.fehrest/vault.json         guard file, format_version = 2
<vault root>/.fehrest/canonical.sqlite   the canonical database
```

Both files live inside the same `.fehrest` control directory format 1 uses
(`crate::vault::CONTROL_DIR`). A vault root has **either** a format-1 guard
(`format_version = 1`, markdown-tree canonical state) **or** a format-2
guard (`format_version = 2`, this format) — never both; the format-2
publish protocol below refuses to run if `.fehrest` already exists in any
form.

## Guard file (`vault.json`)

Identical JSON shape to the format-1 guard (`VaultMeta` in `src/vault.rs`):

```json
{
  "vault_id": "<lowercase UUIDv7>",
  "format_version": 2,
  "created_by_version": "<producing binary's Cargo package version>",
  "created_at": "<placeholder RFC3339-shaped timestamp — see Known limitations>"
}
```

An old format-1 binary's existing reader (`crate::vault::read_vault_meta`)
already refuses any `format_version` greater than its own
`SUPPORTED_FORMAT_VERSION` (`1`) with an explicit
`"unsupported vault format_version 2, newest supported is 1"` error. This
format intentionally reuses that exact mechanism rather than introducing a
second one: publishing a format-2 guard is what makes an old binary refuse
it, with no code change required in the old binary.

## Database (`canonical.sqlite`)

### Required engine configuration

Set on every connection that opens this database (not persisted by SQLite
across connections, except `page_size` which is fixed at creation):

| Setting | Value | Why |
|---|---|---|
| `PRAGMA page_size` | `4096` | §13 tie-break; fixed at creation, cannot change without `VACUUM` |
| `PRAGMA journal_mode` | `DELETE` | §13: rollback journal, not WAL (WAL requires a new ADR) |
| `PRAGMA synchronous` | `EXTRA` (`3`) | §13: rollback journal with EXTRA synchronization |
| `PRAGMA foreign_keys` | `ON` (`1`) | referential integrity for later tables |
| `PRAGMA trusted_schema` | `OFF` (`0`) | S04: schema-level defense against malicious/unexpected SQL constructs |
| `PRAGMA cache_size` | `-8192` (8192 KiB = 8 MiB) | §13 tie-break |

No SQLite extension loading is possible: the `rusqlite` dependency is
compiled with `default-features = false, features = ["bundled"]`
(`Cargo.toml`), so the `load_extension` C API is not even linked in. This
build never calls `enable_load_extension`.

### Tables (schema version 2)

Four tables, exactly. `canonical_vault` (`T01-02`) plus three added by
`T01-03`: `revision`, `current_object`, `command`.

```sql
CREATE TABLE canonical_vault (
    singleton               INTEGER PRIMARY KEY CHECK (singleton = 1),
    vault_id                TEXT NOT NULL,
    schema_version          INTEGER NOT NULL,
    min_reader_capability   INTEGER NOT NULL,
    created_by_version      TEXT NOT NULL,
    created_at              TEXT NOT NULL,
    transaction_head_seq    INTEGER NOT NULL,
    transaction_head_hash   TEXT
);

CREATE TABLE revision (
    revision_id             TEXT PRIMARY KEY,
    object_id               TEXT NOT NULL,
    parent_revision_id      TEXT,
    recorded_seq            INTEGER NOT NULL,
    recorded_at             TEXT NOT NULL,
    actor                   TEXT NOT NULL,
    origin                  TEXT NOT NULL,
    payload                 TEXT NOT NULL,
    payload_sha256          TEXT NOT NULL,
    FOREIGN KEY (parent_revision_id) REFERENCES revision(revision_id)
);

CREATE TABLE current_object (
    object_id               TEXT PRIMARY KEY,
    current_revision_id     TEXT NOT NULL,
    tombstoned              INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (current_revision_id) REFERENCES revision(revision_id)
);

CREATE TABLE command (
    command_id              TEXT PRIMARY KEY,
    input_digest            TEXT NOT NULL,
    actor                   TEXT NOT NULL,
    recorded_seq            INTEGER NOT NULL,
    recorded_at             TEXT NOT NULL,
    previous_head_seq       INTEGER NOT NULL,
    previous_head_hash      TEXT,
    object_id               TEXT NOT NULL,
    revision_id             TEXT NOT NULL,
    resulting_head_seq      INTEGER NOT NULL,
    resulting_head_hash     TEXT NOT NULL,
    FOREIGN KEY (revision_id) REFERENCES revision(revision_id)
);
```

| Table | Column | Meaning |
|---|---|---|
| `canonical_vault` | `singleton` | Always `1`. Enforces exactly one identity row via `PRIMARY KEY CHECK`. |
| | `vault_id` | Lowercase UUIDv7, **must equal** the guard's `vault_id` (§15 "guard/DB identity agrees"). |
| | `schema_version` | `2` as of `T01-03` (adds `revision`/`current_object`/`command`). |
| | `min_reader_capability` | `2` as of `T01-03`: a `T01-02`-only build does not understand these tables and must refuse to write through them, not guess. |
| | `created_by_version` | Producing binary's `Cargo.toml` package version. |
| | `created_at` | Placeholder RFC3339-shaped creation timestamp (see Known limitations). |
| | `transaction_head_seq` | Sequence of the last committed command. `0` until the first commit. |
| | `transaction_head_hash` | Hash of the last committed command. `NULL` until the first commit. |
| `revision` | `revision_id` | UUIDv7, immutable once written. One row per accepted payload version — never overwritten or deleted (I05). |
| | `object_id` | Groups every revision of one record. Allocated by `CreateObject`, never caller-supplied (I03). |
| | `parent_revision_id` | The revision this one supersedes, or `NULL` for an object's first revision. Forms the full history chain. |
| | `recorded_seq` | Transaction-monotonic sequence at which this revision was accepted (matches the command's `resulting_head_seq`). |
| | `recorded_at` | Placeholder observation timestamp (see Known limitations). |
| | `actor` | Caller-declared principal (§15 "actor"), not extracted from `payload`. |
| | `origin` | One of `user`/`import`/`agent-proposal`/`migration`/`system`, caller-declared. |
| | `payload` | Exact UTF-8 payload bytes (Rust `String`, so UTF-8 validity holds by construction — never raw arbitrary bytes). |
| | `payload_sha256` | Lowercase hex SHA-256 of `payload`'s UTF-8 bytes. |
| `current_object` | `object_id` | One row per live object. |
| | `current_revision_id` | The object's current revision. Only Core (the transaction path below) writes this; a discrepancy with `revision` blocks mutation (§16). |
| | `tombstoned` | Reserved for a future deletion task; always `0` as of `T01-03` (no tombstone/delete command exists yet). |
| `command` | `command_id` | Caller-supplied UUID: the idempotency key. Immutable once committed. |
| | `input_digest` | Digest of the normalized command input (see "Command digest" below). Detects a changed request replayed under the same `command_id`. |
| | `actor` | Same value as the resulting revision's `actor`. |
| | `recorded_seq` / `recorded_at` | Same values as the resulting revision's. |
| | `previous_head_seq` / `previous_head_hash` | The transaction head immediately before this command, for independent chain verification. |
| | `object_id` / `revision_id` | The object and revision this command produced. |
| | `resulting_head_seq` / `resulting_head_hash` | The transaction head immediately after this command (mirrors `canonical_vault`'s head at the moment of commit). |

### Command digest

`input_digest` is computed over a JSON object with keys `version`,
`command_id`, `actor`, `origin`, `target_kind` (`create_object` or
`update_object`), `object_id` (`null` for a create), `expected_revision_id`
(`null` for a create), and `payload_sha256` — **not** the raw payload bytes,
which are separately hashed and stored. This is serialized through
`serde_json::Value` (whose `Map` is `BTreeMap`-backed in this crate, since
the `preserve_order` feature is not enabled), which yields object keys in
sorted order — the core "canonical key order" property of RFC 8785 JCS.
This is **deliberately narrower than full JCS**: no Unicode NFC
normalization, and no cross-language golden vectors (§15) exist yet, because
there is no second implementation to vector against — this digest is used
only for this store's own in-process idempotency check, not yet an interop
wire format. The non-finite-number rejection rule JCS also requires cannot
be violated here by construction: every field is a string or `null`, never
a float. `sha256(canonical_json_bytes)`, lowercase hex, using the same
`hash_bytes` helper `src/events.rs` already uses for the event log.

### `resulting_head_hash` chain

`sha256("flake-canonical-tx-v1|" + previous_head_hash_or_empty + "|" +
revision_id + "|" + input_digest)`. Each command's result depends on the
previous head, so the chain is tamper-evident in the same limited sense
`src/events.rs`'s event chain already documents (S05: detects inconsistent
local edits relative to an expected head; a same-user attacker with direct
file access can still rewrite the whole file — this is not a signature or
non-repudiation claim).

### Schema recognition (what "refuses writes" means here)

Every open (`CanonicalStore::open`, and the independent verification step
inside `CanonicalStore::create` before publication) runs the same check:

1. `SELECT name FROM sqlite_master WHERE type='table'` must return **exactly**
   `["canonical_vault", "command", "current_object", "revision"]`, sorted —
   no fewer, no more, no differently-named table.
2. `PRAGMA table_info(<table>)` must return exactly the documented
   `(name, declared_type)` pairs, in that order, for **every** one of the
   four tables above.
3. `SELECT vault_id, min_reader_capability FROM canonical_vault WHERE singleton = 1`
   must return exactly one row; its `vault_id` must equal the guard's; its
   `min_reader_capability` must not exceed what the opening build
   implements.
4. `PRAGMA page_size` must read back `4096`.

Any deviation — an extra table (however innocuous-looking), a renamed or
retyped column in any of the four tables, more or fewer identity rows, a
mismatched `vault_id`, a `min_reader_capability` this build does not
implement, or a wrong `page_size` — is refused with a specific
`Error::Canonical` message rather than silently opened, partially trusted,
or "migrated" on the spot.

## Publication protocol

1. Refuse immediately if `<root>/.fehrest` already exists (no-clobber).
2. Build the guard and database inside a uniquely named sibling staging
   directory, `<root>/.fehrest.staging-<uuid7>/`.
3. Set `page_size`, then the runtime pragmas, then create `canonical_vault`
   and insert its one row — all inside the staging database.
4. Reopen the staged database **independently** (a fresh, read-only
   connection distinct from the one that wrote it) and run the exact schema
   recognition check above against it.
5. Only after that independent verification succeeds, publish by renaming
   the staging directory onto `.fehrest`. Both POSIX `rename(2)` and Win32
   `MoveFileExW` refuse to replace an existing directory, so this rename is
   the actual no-clobber enforcement point, not the upfront existence
   check in step 1 (which only gives a faster, friendlier error).
6. Any failure at any stage removes the staging directory and returns an
   error; `.fehrest` is never touched, so a retry always sees a clean root.

## Command commit protocol (`T01-03`)

Mutation requires a `CanonicalWriter`, obtained only via
`CanonicalStore::writer()`, which acquires the same OS-held `writer.lock`
lease format-1's `Vault::open_write` uses (reused, not duplicated — both
formats share the `.fehrest` control-directory layout, and a root publishes
at most one format at a time). One `CanonicalWriter` can exist per root at
a time, both across processes (the OS lease) and within one process
(`writer()` takes `&mut self`).

`CanonicalWriter::commit` runs entirely inside one `rusqlite` transaction
(I04 — one acknowledged command, one committed transaction):

1. Compute `payload_sha256` and `input_digest` (above).
2. If `command_id` already has a stored `command` row: same `input_digest`
   → return that row's result again (`replay: true`, no new mutation, no
   transaction opened at all); different `input_digest` → refuse
   immediately ("changed digest under the same command_id").
3. Begin a transaction. Read the current `transaction_head_seq`/`_hash`.
4. `CreateObject`: allocate a fresh UUIDv7 `object_id`, no parent revision.
   `UpdateObject`: look up `current_object`'s pointer for the given
   `object_id`; missing → refuse ("unknown object_id"); present but not
   equal to the caller's `expected_revision_id` → refuse ("expected
   revision conflict", never a silent last-writer-wins).
5. Insert the new `revision` row, upsert `current_object`'s pointer,
   compute `resulting_head_hash`, insert the `command` row, and advance
   `canonical_vault`'s head — four statements, one transaction.
6. Commit. A failure at any point before this step rolls back every
   statement above (nothing partial is ever visible); a failure reported
   to the caller *after* this step means the command is durably committed
   even though the caller did not receive confirmation — retrying with the
   exact same `command_id` reconciles to the real result via step 2, rather
   than re-executing or minting a second command for one logical request.

## Generic-reader example

No Flake binary, library, or network access is required to inspect a
published format-2 store — any SQLite 3 tool works directly against
`canonical.sqlite`:

```sh
sqlite3 <vault-root>/.fehrest/canonical.sqlite <<'SQL'
.headers on
PRAGMA page_size;
PRAGMA journal_mode;
SELECT * FROM canonical_vault;
SELECT revision_id, object_id, parent_revision_id, recorded_seq, payload FROM revision ORDER BY recorded_seq;
SELECT object_id, current_revision_id FROM current_object;
SELECT command_id, object_id, revision_id, resulting_head_seq FROM command ORDER BY recorded_seq;
SQL
```

Expected output shape (after one `CreateObject` commit):

```text
page_size
4096
journal_mode
delete
singleton|vault_id|schema_version|min_reader_capability|created_by_version|created_at|transaction_head_seq|transaction_head_hash
1|<uuid7>|2|2|0.0.1-phase-t|<timestamp>|1|<hex64>
revision_id|object_id|parent_revision_id|recorded_seq|payload
<uuid7>|<uuid7>||1|hello
object_id|current_revision_id
<uuid7>|<uuid7>
command_id|object_id|revision_id|resulting_head_seq
<uuid7>|<uuid7>|<uuid7>|1
```

## Access coordination and recovery (`T01-04`)

A third file, `.fehrest/access.lock`, coordinates normal opens against
recovery — plan §14's second lock, alongside `writer.lock`:

| Lock | File | Held by | Mode |
|---|---|---|---|
| Writer ownership | `writer.lock` | one writer/recovery process at a time | exclusive, `create_new`/O_EXCL marker file (unchanged from `T01-01`) |
| Access | `access.lock` | every normal open connection | shared, for its entire open lifetime |
| Access | `access.lock` | `crate::recovery` | exclusive, only while recovering |

`access.lock` is created empty, unlocked, as part of `CanonicalStore::create`'s
staged publication (never lazily by a read — `CanonicalStore::open` requires
it to already exist, exactly like it requires the guard to exist). This
uses `std::fs::File`'s stable advisory-lock API (`lock_shared`/`try_lock`/
`unlock`, stable since Rust 1.89; this crate's `rust-version` is `1.97`) —
no new dependency. This coordinates cooperating Flake processes, not
adversarial same-user filesystem access (§14, §20), matching every other
lock in this crate.

Recovery (`crate::recovery::recover_to_new_root`) acquires writer ownership
*then* exclusive access, in that fixed order — the same order a normal
writer's `CanonicalStore::writer()` effectively participates in by sharing
the identical `writer.lock` primitive — so every code path in the crate
acquires these two locks in one consistent global order. If any normal
connection currently holds shared access, recovery's exclusive attempt
returns `Busy`-shaped `Error::Recovery` immediately rather than blocking; a
new normal `open` attempted while recovery holds exclusive access blocks
until recovery releases it.

Recovery never touches the live original: it preserves the exact guard/
database/(-journal, if present) bytes to a `.fehrest/recovery-preserved-
<uuid7>/` directory first, builds a disposable working copy from *those*
preserved bytes, and verifies the working copy independently and far more
thoroughly than an ordinary `open` — recomputing the entire `command` chain
from scratch (§16 "verify... history/head and reconstructed current
state"), not merely re-reading the stored head. Only a fully-verified
working copy is published, staged-and-renamed onto a caller-chosen new
root exactly like `CanonicalStore::create`'s own no-clobber discipline. A
verification failure removes only the disposable working copy; the
original is completely untouched and the preserved bytes remain on disk.

Every recovery attempt — success or refusal — writes
`incident-manifest.json` into its preservation directory:

```json
{
  "schema": "flake-recovery-incident-v1",
  "original_root": "<path>",
  "attempted_at": "<placeholder timestamp>",
  "outcome": "verified_and_published" | "refused",
  "recovered_root": "<path> | null",
  "vault_id": "<uuid7> | null",
  "verified_transaction_head_seq": "<int> | null",
  "verified_object_count": "<int> | null",
  "refusal_reason": "<string> | null"
}
```

## Known limitations (recorded, not hidden)

- `created_at` is a placeholder timestamp (seconds-of-day since a fixed
  base date, RFC3339-shaped but not calendar-correct), matching the
  identical, pre-existing limitation in the format-1 guard
  (`vault.rs::chrono_like_now_iso8601`). A real `chrono`/`time` dependency
  was not admitted for this minimal task; this is a named limitation for a
  future task to resolve, not silently worked around here.
- Genuine concurrent-**multi-process** contention over `CanonicalWriter` is
  not tested here — only in-process sequencing (`&mut self` makes a second
  concurrent `writer()` call in the *same* process a compile error, and a
  second `CanonicalStore` handle's `writer()` call is proven to return
  `Error::WriterLocked`). A real second-process kill/contention harness
  matches `T01-01`'s own established methodology of deterministic in-process
  fault injection rather than literal cross-process signaling; see
  `docs/evidence/flake-v1/T01-03/REPORT.md`.
- `T01-03` admits exactly one opaque payload per command against exactly
  one object. Multi-object commands, ordered multi-operation commands, and
  a tombstone/delete command (the `tombstoned` column exists but nothing
  ever sets it to `1`) are not implemented — all explicitly out of this
  task's "minimal record" scope.
- The command digest is a deliberately narrower canonicalization than full
  RFC 8785 JCS (sorted keys only, no Unicode NFC, no cross-language golden
  vectors) — see "Command digest" above.
- No performance measurement against plan §27's M/L datasets: those
  datasets require typed record shapes (`Project`/`Note`/`Action`/...) that
  do not exist before `P02`. Evidence reports record bounded, S-scale
  timing only, consistent with these tasks' own "Native development profile
  mandatory now; all remaining profiles retained for T05-02" cross-platform
  gate.
- `T01-04` recovery is all-or-nothing (a single chain break, dangling
  reference, or head mismatch refuses complete publication); it does not
  build a labeled *partial* salvage that admits a truncated-but-valid
  history prefix. It does not have a CLI `recover`/`verify` command (this
  store has no CLI wiring at all yet). It does not restore from a separate
  backup artifact — only from the live root's own current bytes (`T01-05`'s
  objective). Recovery's `Busy` response to contention is immediate, not a
  bounded wait — `crate::vault::Vault`'s own readers/writers use a blocking
  acquire for the analogous case, but this module chose immediate refusal
  instead; a bounded-wait variant is left to a future task if needed.
- Format-1's own torn-tail repair is completely untouched by `T01-04`: the
  §14 access-lock model is, per `T01-01`'s own evidence, described "in
  terms of the future SQLite canonical store" — this format — not the
  file-based format-1 store.
