# Flake canonical format 2 — `canonical.sqlite`

**Status:** `T01-02` (canonical build plan §13/§15/§16) — **create-only**.
This document describes exactly what `T01-02` publishes today. It is not a
forward-looking schema for the finished product; the `Project`/`Note`/
`Action`/`Decision`/`Source`/`Transaction`/... tables in plan §15 do not
exist yet. They are added by later `P01`/`P02` tasks as the command/
transaction admission API (`T01-03` onward) lands, each through its own
reviewed schema-version bump, not by silently widening what this document
calls "recognized."

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

### Table `canonical_vault`

The only table `T01-02` publishes. One singleton row.

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
```

| Column | Meaning |
|---|---|
| `singleton` | Always `1`. Enforces exactly one identity row via `PRIMARY KEY CHECK`. |
| `vault_id` | Lowercase UUIDv7, **must equal** the guard's `vault_id` (§15 "guard/DB identity agrees"). |
| `schema_version` | Logical schema version. `1` as of `T01-02` (identity/head-summary table only). May advance for a compatible, in-place addition. |
| `min_reader_capability` | Minimum reader capability a build must implement to open this database safely. `1` as of `T01-02`; a reader that implements less must refuse, never guess. |
| `created_by_version` | Producing binary's `Cargo.toml` package version (currently `fehrest`'s `0.0.1-phase-t`). |
| `created_at` | Placeholder RFC3339-shaped creation timestamp (see Known limitations). |
| `transaction_head_seq` | Transaction-monotonic sequence of the last committed transaction. `0` until `T01-03` commits the first one. |
| `transaction_head_hash` | Hash of the last committed transaction. `NULL` until `T01-03`. |

### Schema recognition (what "refuses writes" means here)

Every open (`CanonicalStore::open`, and the independent verification step
inside `CanonicalStore::create` before publication) runs the same check:

1. `SELECT name FROM sqlite_master WHERE type='table'` must return **exactly**
   `["canonical_vault"]` — no fewer, no more, no differently-named table.
2. `PRAGMA table_info(canonical_vault)` must return exactly the eight
   `(name, declared_type)` pairs above, in that order.
3. `SELECT vault_id, min_reader_capability FROM canonical_vault WHERE singleton = 1`
   must return exactly one row; its `vault_id` must equal the guard's; its
   `min_reader_capability` must not exceed what the opening build
   implements.
4. `PRAGMA page_size` must read back `4096`.

Any deviation — an extra table (however innocuous-looking), a renamed or
retyped column, more or fewer identity rows, a mismatched `vault_id`, a
`min_reader_capability` this build does not implement, or a wrong
`page_size` — is refused with a specific `Error::Canonical` message rather
than silently opened, partially trusted, or "migrated" on the spot.

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

## Generic-reader example

No Flake binary, library, or network access is required to inspect a
published format-2 store — any SQLite 3 tool works directly against
`canonical.sqlite`:

```sh
sqlite3 <vault-root>/.fehrest/canonical.sqlite <<'SQL'
.headers on
PRAGMA page_size;
PRAGMA journal_mode;
.schema canonical_vault
SELECT * FROM canonical_vault;
SQL
```

Expected output shape:

```text
page_size
4096
journal_mode
delete
CREATE TABLE canonical_vault (
    singleton               INTEGER PRIMARY KEY CHECK (singleton = 1),
    vault_id                TEXT NOT NULL,
    ...
);
singleton|vault_id|schema_version|min_reader_capability|created_by_version|created_at|transaction_head_seq|transaction_head_hash
1|<uuid7>|1|1|0.0.1-phase-t|<timestamp>|0|
```

## Known limitations (recorded, not hidden)

- `created_at` is a placeholder timestamp (seconds-of-day since a fixed
  base date, RFC3339-shaped but not calendar-correct), matching the
  identical, pre-existing limitation in the format-1 guard
  (`vault.rs::chrono_like_now_iso8601`). A real `chrono`/`time` dependency
  was not admitted for this minimal task; this is a named limitation for a
  future task to resolve, not silently worked around here.
- This task does not integrate the OS-held single-writer lease
  (`crate::vault::WriteLock`) with the format-2 store. `create`/`open` in
  `src/canonical.rs` do not take that lease. Binding mutation of this store
  to the writer lease is `T01-03`'s "bind private mutators to owning vault
  writer" clause, not this one's.
- Genuine concurrent-multi-process creation racing on a brand-new root is
  not tested here (only a single interrupted creator, and a second
  *sequential* creation attempt against an already-published root). This
  task's own acceptance clause is "interrupted creation leaves old paths
  intact," not a multi-process stress proof; that pattern exists for the
  format-1 writer lease (`T01-01`) and will apply here once `T01-03` wires
  the lease in.
- No performance measurement against plan §27's M/L datasets: those
  datasets require typed record shapes (`Project`/`Note`/`Action`/...) that
  do not exist before `P02`. `docs/evidence/flake-v1/T01-02/REPORT.md`
  records a bounded, S-scale (single empty store) creation/reopen timing
  only, consistent with this task's own "Native development profile
  mandatory now; all remaining profiles retained for T05-02" cross-platform
  gate.
