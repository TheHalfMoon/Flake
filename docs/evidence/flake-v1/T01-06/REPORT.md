# T01-06 evidence report — Import legacy vaults without rewriting accepted history

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §14/§15/§16/§20/§22, task `T01-06`
- **Baseline / tested source commit:** forked from `origin/main` `3e1e30cba341e3a1301e0af646e37f771debb2c7` (PR #71, `T01-05` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5 / Claude Fable 5.1 session)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy` (unchanged limitation, recorded at every prior `flake-v1` task). All checks below were executed locally on this development host.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`; confirmed `origin/main` at `3e1e30c...` (PR #71 merged) with all 8 canonical GitHub Actions checks reporting `success` on that exact head commit.
- `specs/CURRENT.md` on `main` at this baseline: `T01-05_STATUS=COMPLETE`, `ACTIVE_IMPLEMENTATION_UNIT=T01-06`, `NEXT_DEPENDENCY_READY_UNIT=T01-06`.
- Re-read the full `T01-06` task-contract row before writing any code.

## Scope actually touched

`src/migration.rs` (new module), `src/canonical.rs` (one new `CommandTarget::ImportObject` variant, plus its two match-site handlers and digest arm — see "A deliberate, scoped exception" below), `src/lib.rs` (module registration, one new `Error::Migration` variant), `docs/formats/legacy-migration.md` (new format doc for the migration manifest and admission rules), `tests/fixtures/migration/` (two new gold fixtures). No `Cargo.toml`/`Cargo.lock` change — this task needed no new dependency or feature. No `src/vault.rs` change: format-1's own `Vault::open_read`/`scan` were already exactly what this task needed (nonmutating, and `scan()` already surfaces conflicts/skipped/malformed separately, from `T01-01`/earlier Phase T work).

```text
docs/formats/legacy-migration.md                  |  98 ++++
src/canonical.rs                                  |  34 ++
src/lib.rs                                        |   7 +
src/migration.rs                                  | 604 ++++++++++++++++++++++
tests/fixtures/migration/legacy-crlf-unicode.md   |   7 +
tests/fixtures/migration/legacy-unknown-fields.md |   7 +
6 files changed, 757 insertions(+)
```
(`git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock`, `raw/04-diff-stat.txt`)

The task's own `Files/components` also names "CLI migration preview" — not built in this unit, consistent with every prior `flake-v1` task's recorded scope boundary (no CLI wiring exists for the format-2 store at all yet).

## A deliberate, scoped exception to I03

`crate::canonical::CommandTarget::ImportObject` is the one command variant that accepts a **caller-supplied** `object_id` rather than allocating a fresh UUIDv7 — every other command (`CreateObject`, `UpdateObject`) either allocates or references an identity the store itself already minted. This is not a loophole: it is refused if the supplied `object_id` is not a valid UUID, and refused if an object with that identity already exists in the target vault ("no overwrite of live data or import-minted authority", S05). It exists specifically because plan §14's own data invariant for this task says "original IDs retained only when unambiguous" — a legacy format-1 object's UUIDv7 genuinely is that object's identity, and manufacturing a new one during migration would sever that continuity for no reason. `preview_migration`'s own ambiguity check (below) is what makes "only when unambiguous" real rather than aspirational.

## What was built, and why — grounded in the task contract

1. **`preview_migration`**: a nonmutating dry-run (`Vault::open_read` + `Vault::scan()`, both pre-existing and already proven nonmutating by `T01-01`'s own tests) that classifies every discovered legacy path as admitted or omitted-with-a-reason. An object whose UUIDv7 was observed at more than one path (`scan()`'s own pre-existing conflict detection, from earlier Phase T work) is omitted entirely as `ambiguous identity` — never guessed at, never arbitrarily assigned to one of the colliding paths. Malformed and skipped (reserved-directory/unsupported-extension) paths are surfaced with their own distinct reasons. The source vault's own event-log chain status is recorded as informational text only, never authenticated or treated as proof of anything (this task's own "do not authenticate legacy log claims" instruction).
2. **Exact-byte payloads**: every admitted record's payload is the file's raw bytes, read directly from disk — never round-tripped through `identity::parse`/`serialize`. CRLF line endings, Unicode content, trailing whitespace, and unrecognized frontmatter lines all survive untouched because the parser used to *discover* an object's ID is never used to *reconstruct* its content. Proven directly by `gold_fixtures_with_crlf_unicode_and_unknown_fields_migrate_byte_identically` against two checked-in fixture files.
3. **`import_to_new_root` / `ImportSelection`**: `AllAdmittedOnly` refuses immediately, before creating anything, if the preview shows any omission at all — "ambiguous complete migration refuses" as a hard precondition. `Selected(ids)` always reports `complete: false`, even when the selection happens to cover every admittable record, and refuses outright (not silently) if an unadmittable `id` is requested.
4. **Migration manifest**: `.fehrest/migration-manifest.json`, written into the **new** vault whether the import finished or was interrupted partway — `imported`/`omitted`/`complete`/`interrupted`/`failure_reason`, matching the incident-manifest/backup-manifest pattern already established at `T01-04`/`T01-05`.
5. **Interruption handling**: each admitted record is imported as its own real, independently committed transaction (via `CanonicalWriter::commit`, unchanged from `T01-03`). A failure partway (a real error, or this module's own `ImportFaultPoint::AfterFirstRecord` test fault) still leaves the new vault in a real, inspectable state — exactly the records that committed before the failure — and the manifest says so explicitly (`interrupted: true`), so a caller cannot mistake a partial result for a finished one.
6. **Format documentation**: `docs/formats/legacy-migration.md` covers the admission rules, the manifest shape, and an explicit "what is not migrated" section (the source event log's own events are never replayed as format-2 history).

## Explicit scope boundaries (recorded, not silently dropped)

- **No CLI wiring.** Consistent with every prior `flake-v1` task's recorded scope boundary — no CLI surface exists for the format-2 store at all yet.
- **No event-log content migration.** Only the source's file-based records are imported; the event log's own events are inspected for chain status only, never replayed as commands — deliberate, per this task's own "do not authenticate legacy log claims" and "must not manufacture lost revisions" rationale.
- **No streaming import.** `Vault::scan()` itself is not streaming (a pre-existing property of the format-1 reader this task does not change); a legacy vault too large to scan into memory at once is a limitation inherited from the reader, not newly introduced here.

## Tests added (7 new; all pass; full raw output `raw/01-full-test-run.txt`, isolated run `raw/07`)

| Test | What it proves | Verification hierarchy tier(s) |
|---|---|---|
| `complete_migration_of_a_clean_vault_preserves_exact_bytes_and_identity` | A clean vault migrates completely; the imported payload is byte-identical to the legacy file (CRLF/Unicode included, via an inline case); the legacy UUID becomes the format-2 `object_id`; the source remains openable and unchanged | V03/V05 |
| `duplicate_identity_is_omitted_and_blocks_a_complete_migration` | Two files sharing one UUID are **both** omitted (never arbitrarily pick one), and a complete-migration request refuses | V02/V09 |
| `malformed_file_blocks_complete_but_selected_import_still_works` | One malformed file blocks `AllAdmittedOnly`; an explicit `Selected` import of just the good record still succeeds, correctly labeled `complete: false` | V02/V09 |
| `requesting_an_unadmittable_id_is_refused_not_silently_skipped` | Requesting a nonexistent/unadmittable `legacy_object_id` in `Selected` is refused outright | V02 |
| `interrupted_migration_writes_a_manifest_and_leaves_source_untouched` | A fault after the first of two records leaves the new vault with exactly one committed record, a manifest marked `interrupted: true`, and the source's own bytes byte-for-byte unchanged | V06/V09 |
| `new_root_no_clobber` | A second import attempt at an already-published destination is refused | V02/V09 |
| `gold_fixtures_with_crlf_unicode_and_unknown_fields_migrate_byte_identically` | Two checked-in gold fixtures (`tests/fixtures/migration/`) — one with CRLF line endings and non-ASCII Unicode content, one with unrecognized frontmatter fields — both migrate byte-for-byte | V03/V05 |

No pre-existing test was modified.

## Commands executed (all re-run on this branch against `origin/main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **174/174 pass**, 0 failed (141 lib [134 pre-existing + 7 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib migration::` | `raw/07-migration-module-tests-isolated.txt` | 7/7 pass |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `cargo run --release --example migration_timing` (throwaway harness, deleted before this commit) | `raw/05-migration-timing.txt` | preview p50=53.5ms/p95=126.2ms/max=126.2ms; import p50=166.7ms/p95=178.3ms/max=178.3ms (n=15 each, 20-record legacy vaults) |
| Environment capture | `raw/06-environment.txt` | Windows 11 Home 10.0.26200, 12 logical CPUs, 16,106 MB RAM, rustc/cargo 1.97.1, vendored SQLite 3.50.2 (unchanged) |

## Failed attempts / exclusions

No failed attempt occurred on the final implementation this time: the module built and every test passed on the first real compilation attempt after two small, expected fixes (a private-import path correction for `ObjectId`, moved from `crate::vault::ObjectId` — a re-export — to its actual home `crate::identity::ObjectId`; and adding a missing `Serialize` derive on `LegacyRecord` needed by the manifest struct that embeds it). No test was skipped, deleted, or weakened to reach green.

## Performance gate

Closest applicable §27 analog, measured at S-scale (one 20-record legacy vault, `raw/05-migration-timing.txt`):

| Operation | Closest §27 analog | Target / Maximum | Observed (S-scale) |
|---|---|---|---|
| `preview_migration` (20 records) | "Full export/import M": target 60s / max 180s | p50 53.5ms, p95 126.2ms, max 126.2ms — well inside |
| `import_to_new_root` (20 records, 20 real transactions) | "Full export/import M": target 60s / max 180s | p50 166.7ms, p95 178.3ms, max 178.3ms — well inside |

`import_to_new_root` costs more than `preview_migration` alone because it also creates a fresh format-2 vault and commits one real, independently-durable transaction per record — a legitimate cost of this task's own inherited "one acknowledged command = one committed transaction" invariant (`T01-03`), not an inefficiency specific to migration. No dataset-M/L measurement: `Vault::scan()` is not streaming, so an M/L-scale legacy vault is out of this task's own performance-gate scope as recorded above.

## Durability gate

D5 "migration interruption and original preservation": `interrupted_migration_writes_a_manifest_and_leaves_source_untouched` proves both directions at once — the source's exact bytes are unchanged after an interrupted attempt (this module never opens the source for anything but `Vault::open_read`, which takes no write lock and mutates nothing), and the new vault's own manifest honestly reports the interruption rather than silently presenting a partial result as finished.

## Cross-platform gate

Native development profile only (Windows 11, MSYS/MinGW toolchain, NTFS), matching this task's own "Development profile; cross-platform path/line-ending fixtures and later native run" clause. The checked-in gold fixtures already exercise CRLF line endings (a Windows/cross-platform-relevant case) even though only one OS was used to run them; full cross-platform (Linux/ext4, macOS/APFS) execution is deferred to `T05-02` like every prior `flake-v1` task.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Valid gold fixtures preserve exact payloads and identity mappings | Satisfied — `gold_fixtures_with_crlf_unicode_and_unknown_fields_migrate_byte_identically` |
| Corrupt/duplicate cases cannot report complete migration | Satisfied — `duplicate_identity_is_omitted_and_blocks_a_complete_migration`, `malformed_file_blocks_complete_but_selected_import_still_works` |
| Originals hash-identical after success/failure | Satisfied — `complete_migration_of_a_clean_vault_preserves_exact_bytes_and_identity` (success path), `interrupted_migration_writes_a_manifest_and_leaves_source_untouched` (failure path, explicit byte comparison) |

## Completion condition

Every acceptance clause above is satisfied. CLI wiring, event-log content migration, and streaming import for oversized legacy vaults are explicitly deferred, matching this task's own forbidden-scope boundary against successor work and inherited limitations from the pre-existing format-1 reader. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (174/174). No sealed evidence altered; no force-push; no historical evidence file touched. `T01-06` is complete.

## Next frontier

`T01-07` — Close the corrective save/recovery gate on a native host. Depends on `T01-06` (this task) and is `P01`'s final task: a native-host closeout of the whole corrective save/recovery slice (`T01-01` through `T01-06`) this sub-plan has built.
