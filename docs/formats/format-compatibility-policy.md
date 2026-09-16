# Format compatibility policy (`T05-01`)

**Status:** freezes, as a single cross-referenced policy document, the
compatibility rules plan §21 ("Format evolution") requires and that
`T01-02`/`T01-03`/`T02-01`/`T02-02`/`T01-06` already built and tested,
each in its own module doc. This document adds **no new mechanism** —
every rule below cites the exact already-shipped code that enforces it.
Its own job is narrower and, for a compatibility promise, more important
than any one mechanism: state plainly, in one place, what an owner can
rely on across every future Flake release, so that promise is reviewable
independent of wherever the code that keeps it happens to live.

## The epoch model

A **format epoch** is a whole-database structural generation: the fixed
table/column shape `docs/formats/format-2-canonical-sqlite.md` documents,
guarded by `.fehrest/vault.json`'s own `format_version` field
(`crate::vault::VaultMeta`, `crate::canonical::CANONICAL_FORMAT_VERSION`).
Two epochs exist as of this document: **format 1** (legacy markdown-tree
canonical state, pre-`flake-v1`) and **format 2** (the current and, as of
this task, frozen release epoch — `CANONICAL_FORMAT_VERSION = 2`). No
format 3 exists. This document does not invent one; §21's own instruction
("no invented future production format") is a hard boundary, not a
suggestion this task works around by sketching one anyway.

Within one epoch, `min_reader_capability` (`canonical_vault` table,
`CANONICAL_MIN_READER_CAPABILITY` in `src/canonical.rs`) allows a
**compatible, in-place schema addition** without moving the whole epoch —
already implemented and tested
(`min_reader_capability_higher_than_supported_is_refused`,
`src/canonical.rs`). A build whose own `CANONICAL_MIN_READER_CAPABILITY`
is lower than a database's declared `min_reader_capability` refuses to
open it for writing rather than guessing compatibility.

Independently, each **typed record payload** (`Project`/`Note`/`Action`/
`Decision`/`Source`/... — `docs/formats/typed-records.md`) carries its own
`payload_schema_version`, one level below the whole-database epoch. A
payload declaring a version newer than this build's
`RECORD_PAYLOAD_SCHEMA_VERSION` is refused on read
(`a_payload_schema_version_newer_than_supported_is_refused`,
`src/project.rs`) — a record-shape capability check, independent of and
finer-grained than the whole-database `min_reader_capability` check above.

## The policy, stated once, with its exact enforcement point

| Rule (§21 wording) | Enforced by | Proof |
|---|---|---|
| "current major epoch and its immediately previous epoch may be supported by the live reader" | `flake-migrate` (this task) reads format-1 nonmutating (`crate::vault::Vault::open_read`/`scan`) purely to migrate it into a fresh format-2 root; no live format-2 write path ever opens a format-1 root or vice versa — the two epochs are never silently blended in one live vault | `docs/formats/legacy-migration.md`; this task's own evidence |
| "Unknown required field/capability or newer major refuses writes and states the required reader" | `CANONICAL_MIN_READER_CAPABILITY` guard (whole database); `RECORD_PAYLOAD_SCHEMA_VERSION` guard (each typed payload) | `src/canonical.rs::min_reader_capability_higher_than_supported_is_refused`; `src/project.rs::a_payload_schema_version_newer_than_supported_is_refused`; this task's own golden-fixture reproduction, `tests/fixtures/format-compat/` |
| "Unknown optional payload bytes survive read/export/import unchanged" | `#[serde(flatten)] unknown: JsonMap<...>` on every typed record struct (`src/project.rs`, `src/capture.rs`, `src/checkpoint.rs`, `src/disclosure.rs`, `src/grant.rs`, `src/proposal.rs`, `src/relation.rs`, `src/source_check.rs`); `identity::Frontmatter::unknown` for format-1 frontmatter | `src/project.rs::unknown_fields_survive_a_read_then_write_round_trip`; this task's own golden-fixture reproduction |
| "Major migration is always copy-to-new-root with a verified backup and explicit user confirmation" | `migration::import_to_new_root` always creates a **fresh** root, refuses if it already exists (`new_root_no_clobber`), never mutates the source; the desktop/CLI backup-before-migrate obligation is `T01-05`'s own backup command, unchanged by this task | `src/migration.rs` module docs, "no-clobber" test |
| "Publish permanent format specifications, golden fixtures and versioned standalone offline migration tools" | This document plus `docs/formats/format-2-canonical-sqlite.md`/`docs/formats/typed-records.md`/`docs/formats/legacy-migration.md`; `tests/fixtures/migration/` and `tests/fixtures/format-compat/`; `flake-migrate` (`src/bin/flake-migrate.rs`) | This task |
| "old owned data must remain readable without keeping an obsolete main app" | `flake-migrate` links only the `fehrest` library crate, builds and runs standalone, offline, independent of the desktop app's own release cadence (see its own module docs for the exact reasoning) | This task |
| "No perpetual unbounded upcaster stack in Core" | No format-3 (or later) exists; nothing to stack. When a future epoch is authorized, this policy requires it stay a bounded current+immediately-previous pair, not an accreting chain — a constraint on that future task, not code this one adds | This document (forward-looking constraint, not a present mechanism) |
| "Downgrade opens a retained old backup, not a lossy reverse mutation" | No in-place downgrade path exists anywhere in `src/`; the only way back to an older epoch is opening a retained pre-migration backup (`T01-05`) | Absence of any downgrade code path (verified: `grep -rn "downgrade" src/` finds no implementation, only this policy's own wording) |

## The standalone offline migration tool

`flake-migrate` (`src/bin/flake-migrate.rs`, this task) is a thin CLI/JSON
shell over `migration::preview_migration`/`migration::import_to_new_root`
(`T01-06`, unchanged) — it adds no migration policy of its own. It links
only the `fehrest` library crate: no desktop/Tauri dependency, no network
code path anywhere in the binary or the crate it links. Build once,
offline, and it keeps working regardless of what desktop app version (if
any) is installed:

```sh
cargo build --release --bin flake-migrate
./target/release/flake-migrate preview <format-1-vault-root>
./target/release/flake-migrate import  <format-1-vault-root> <new-format-2-root>
```

See its own `--help` output and module docs for exact command shapes,
`--select` partial-import semantics, and exit codes.

## Golden fixtures (`tests/fixtures/`)

- `tests/fixtures/migration/` (`T01-06`): two format-1 legacy records
  (CRLF/Unicode preservation, unknown-frontmatter-field preservation) —
  the supported-epoch migration path.
- `tests/fixtures/format-compat/` (this task): synthetic fixtures for the
  *refusal* and *unknown-optional-preservation* rules above — see that
  directory's own `README.md` for exactly what each fixture proves and
  how to reproduce it independently.

## Known limitation

Only one format-2 epoch has ever existed. Every "current epoch reads its
immediately previous epoch" claim above is proven for the format-1→2
boundary specifically; it cannot yet be proven for a format-2→3 boundary
because format 3 does not exist. This is recorded honestly as a structural
limitation of "first release," not silently implied to be already
exercised across two format-2-or-later epochs.
