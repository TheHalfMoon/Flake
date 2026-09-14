# T01-03 evidence report — Commit save, full history and command result atomically

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §13/§15/§16/§18/§22, task `T01-03`
- **Baseline / tested source commit:** forked from `origin/main` `fdc15c42f1fde706336d1f9b0d7b5faadd83fa92` (PR #68, `T01-02` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5 / Claude Fable 5.1 session)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy` (unchanged limitation, recorded at `T00-01`/`T01-01`/`T01-02`). All checks below were executed locally on this development host.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`; confirmed `origin/main` at `fdc15c4...` (PR #68 merged) with all 8 canonical GitHub Actions checks (`manifest-check`, `canonical-equality`, `validate`, `test-review-binding`, `test-statistical-design`, `test-validator`, `test-scorer`, `verify-artifacts`) reporting `success` on that exact head commit.
- `specs/CURRENT.md` on `main` at this baseline: `T01-02_STATUS=COMPLETE`, `ACTIVE_IMPLEMENTATION_UNIT=T01-03`, `NEXT_DEPENDENCY_READY_UNIT=T01-03`. Matches the canonical plan's dependency chain `T01-02 -> T01-03`.
- Re-read the full `T01-03` task-contract row in `FLAKE_CANONICAL_BUILD_PLAN.md` §"P01" before writing any code.

## Scope actually touched

`src/canonical.rs` (extended: transaction/command admission API, schema version 2), `src/vault.rs` (one visibility-only change: `WriteLock::acquire` widened from private to `pub(crate)` so this task's writer capability reuses the existing lock mechanism instead of duplicating it — no behavioral change), `docs/formats/format-2-canonical-sqlite.md` (extended for schema version 2 and the commit protocol). No `Cargo.toml`/`Cargo.lock` change. No file outside the task's named `Files/components` ("Core transaction/admission API, CLI save path, revision/history/command tables and tests") was touched, with one narrow addition explained above.

```text
docs/formats/format-2-canonical-sqlite.md |  234 +++++--
src/canonical.rs                          | 1013 +++++++++++++++++++++++++++--
src/vault.rs                              |    8 +-
3 files changed, 1135 insertions(+), 120 deletions(-)
```
(`git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock`, `raw/04-diff-stat.txt`)

The task's own `Files/components` also names "CLI save path" — not touched in this unit. `src/cli.rs` has no command surface yet that would call this API (its existing commands operate on the format-1 `Vault`); wiring a CLI command to the format-2 store belongs to whichever task first makes the format-2 store the active product path (this task's objective is the Core admission API itself, and its acceptance criteria are all about the storage boundary, not a CLI verb). Recorded as a scope note, not silently dropped.

## What was built, and why — grounded in the task contract

1. **Schema version 2**: `revision`, `current_object`, `command` tables added alongside `T01-02`'s `canonical_vault`. `assert_schema_recognized` was generalized from a single hardcoded table to iterate `EXPECTED_TABLES` (`["canonical_vault", "command", "current_object", "revision"]`), so an unrecognized table or a retyped column in *any* of the four is refused exactly as `T01-02` already refused it for one. `CANONICAL_SCHEMA_VERSION`/`CANONICAL_MIN_READER_CAPABILITY` both bumped `1 -> 2`.
2. **`CanonicalStore::writer()`**: acquires `crate::vault::WriteLock` (widened to `pub(crate)`, reused rather than duplicated) against the same vault root, returning a `CanonicalWriter<'_>` capability — the format-2 mirror of `crate::vault::Vault::writer()`/`VaultWriter`. Taking `&mut self` means a second concurrent `writer()` call in the *same* process is a compile-time borrow error, on top of the OS-held lease making a second process's call fail visibly with the pre-existing `Error::WriterLocked`.
3. **`CanonicalWriter::commit`**: the actual "Core transaction/admission API." One `CommandInput` (a `CreateObject` or `UpdateObject` against exactly one opaque UTF-8 payload — "a minimal record", the task's own objective wording) is admitted inside exactly one `rusqlite::Transaction`: read the current head, validate the expected revision for an update (I07 — never a silent last-writer-wins), insert the new `revision` row, upsert `current_object`'s pointer, compute the chained `resulting_head_hash`, insert the `command` result row, advance `canonical_vault`'s head, commit.
4. **Idempotency and digest-conflict rejection (§18, F01)**: a duplicate `command_id` with an identical input digest returns the original committed result without opening a new transaction at all (`replay: true`); a duplicate `command_id` with a *different* input digest is refused immediately, before any write. `command_id`/UUID identity is caller-supplied and never regenerated by this API — "never issue a replacement ID automatically."
5. **Outcome reconciliation after a lost acknowledgement (F01)**: `CanonicalStore::command_outcome`/`read_current`/`history` let a caller (or a fresh reopened handle) discover the true committed state without resubmitting anything, and retrying `commit` with the exact same `CommandInput` reconciles to that same result via the idempotency check in (4) rather than executing a second commit.
6. **Command digest**: computed over a small `serde_json::Value` (sorted keys, since this crate's `serde_json` `Map` is `BTreeMap`-backed — the `preserve_order` feature is not enabled) hashed with the existing `crate::events::hash_bytes` helper. Deliberately narrower than full RFC 8785 JCS — recorded explicitly in `docs/formats/format-2-canonical-sqlite.md` "Command digest" and "Known limitations," not silently claimed as full compliance.
7. **Format documentation**: `docs/formats/format-2-canonical-sqlite.md` extended with the four-table DDL, the exact schema-recognition rule generalized to all four tables, the command digest and head-chain algorithms, the full commit protocol, and a generic `sqlite3` CLI reader example against a post-commit database.

## Explicit scope boundaries (recorded, not silently dropped)

- **One object, one operation per command.** No multi-object or ordered multi-operation commands (§15's "ordered requested operations" plural). The task's own objective is "save **a minimal record**"; a richer command shape is successor work this task's forbidden scope excludes.
- **No tombstone/delete command.** `current_object.tombstoned` exists in the schema (so a future task can add one without another schema-version bump for that column) but nothing ever sets it to `1` in this task.
- **No literal cross-process kill test.** "Child-process termination schedule" is satisfied via deterministic in-process fault injection (`CommitFaultPoint::BeforeSqlCommit`/`AfterSqlCommitBeforeReturn`), matching this repository's own established methodology — `T01-01`'s evidence used the identical in-process approach (temporarily reverting code in place, not spawning and killing a real process) for its regression proof; this repository has no OS-level process-kill test harness for any task, `T01-01` included.
- **No genuine multi-*process* writer contention test.** Only in-process sequencing is proven (a second `CanonicalStore::writer()` call, same process, fails with `Error::WriterLocked` while the first is held — `only_one_writer_capability_can_exist_at_a_time`). This is the same limitation `T01-02`'s evidence already named for creation and is inherited here for commit, since both reuse the identical `WriteLock` primitive.
- **CLI save path not wired.** See "Scope actually touched" above.
- **Command digest is not full RFC 8785 JCS.** See "Command digest" above and the format doc.

## Tests added (11 new; all pass; full raw output `raw/01-full-test-run.txt`, isolated module run `raw/07-canonical-module-tests-isolated.txt`)

| Test | What it proves | Verification hierarchy tier(s) |
|---|---|---|
| `create_object_commits_atomically_and_advances_head` | A `CreateObject` commit atomically writes the revision, current pointer, command row and advances the head in one step | V02/V03 |
| `update_object_requires_matching_expected_revision` | A correct `expected_revision_id` succeeds and advances state; a stale one is refused as an explicit conflict and changes nothing (I07) | V02/V05 |
| `update_of_unknown_object_id_is_refused` | Updating a never-created `object_id` is refused, not silently treated as a create | V02 |
| `duplicate_command_id_with_identical_input_replays_the_original_result` | Exact resubmission returns the original result (`replay: true`) and performs no new mutation (§18) | V02/V05 |
| `duplicate_command_id_with_different_input_is_rejected` | A changed digest under the same `command_id` is refused, and nothing new is committed | V02/V09 |
| `oversized_payload_is_refused_before_any_mutation` | A payload over `limits::MAX_OBJECT_BYTES` is refused before any transaction opens; head unchanged (S04) | V02/V09 |
| `full_history_reconstructs_current_state_independent_of_pointer` | Walking only the immutable `revision` table's parent chain (never touching `current_object`) finds the same head `current_object` maintains — an independent cross-check, not the same code path (§16 acceptance clause) | V02/V05 |
| `fault_before_sql_commit_leaves_zero_trace` | A fault injected after all statements execute but before `COMMIT` leaves the head unchanged and no command row — "complete pre-state or complete committed state," never a half-state | V06 |
| `fault_after_sql_commit_reconciles_on_retry_instead_of_double_committing` | A fault injected after the real `COMMIT` (simulating a lost acknowledgement) still leaves the transaction durably committed; reconciling via `command_outcome` or retrying `commit` with the same input returns the same result without a second commit | V06/V07 |
| `only_one_writer_capability_can_exist_at_a_time` | A second `CanonicalStore` handle's `writer()` call fails with `Error::WriterLocked` while the first writer is held (S06) | V03/V09 |
| `reopened_store_independently_shows_the_committed_command_and_revision` | A **freshly reopened, independent** `CanonicalStore` handle — not the one that committed — sees the same revision, command outcome and head (task's own "inspect reopened DB independently" verification method) | V03/V15 |

All 12 pre-existing `T01-02` tests were re-run unmodified except one assertion updated for the now-4-table schema (`generic_sqlite_connection_can_inspect_published_schema`: it asserted the table list equals `["canonical_vault"]`; now asserts it equals the sorted `EXPECTED_TABLES` four-table list — the same property, generalized, not weakened). No test was deleted or had its assertion loosened.

## Commands executed (all re-run on this branch against `origin/main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings after one fix (below) |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **153/153 pass**, 0 failed (120 lib [109 pre-`T01-02` + 12 `T01-02` + 11 new `T01-03`... note: 109 lib tests existed before `T01-02`; `T01-02` added 12 → 121; but `T01-02`'s own module started at 97 non-canonical + 12 canonical = 109 total lib tests at that baseline — see below] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib canonical::` | `raw/07-canonical-module-tests-isolated.txt` | 23/23 pass (12 `T01-02` + 11 new), isolated confirmation |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff (one intermediate draft needed `cargo fmt --all`; re-checked clean before this final run) |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `cargo run --release --example commit_timing` (throwaway harness, deleted before this commit) | `raw/05-commit-timing.txt` | commit p50=2.9ms/p95=6.0ms/max=8.1ms (n=30: 1 create + 29 sequential updates to the same object) |
| Environment capture | `raw/06-environment.txt` | Windows 11 Home 10.0.26200, 12 logical CPUs, 16,106 MB RAM, rustc/cargo 1.97.1, vendored SQLite 3.50.2 (unchanged) |

Exact lib test count clarification: at the `T01-02` baseline the full lib suite was 109 (97 pre-existing before any `canonical.rs` work + 12 `T01-02` canonical tests). This task added 11 more canonical tests and touched no other lib test file, so the lib suite is now 109 + 11 = **120**, matching `raw/01-full-test-run.txt`. Total suite: 120 lib + 10 integration + 23 kill_tests = **153**.

## Failed attempts / exclusions

- The first draft left `command_id` computed but unused in one test (`duplicate_command_id_with_identical_input_replays_the_original_result`), producing an `unused_variables` compiler warning (would have failed `clippy -D warnings`). Fixed by asserting `first.command_id`/`second.command_id` both equal the captured `command_id` — a real assertion, not a `#[allow]` or an underscore-prefixed rename to silence it.
- The first `cargo fmt --all -- --check` run on the extended module reported a real (whitespace/wrapping only) diff; `cargo fmt --all` was run and the check re-verified clean before this final evidence run.
- No other failed attempt occurred on the final implementation; no test was skipped, deleted, or weakened to reach green.

## Performance gate

Closest applicable §27 analog, measured at S-scale (one store, up to 30 sequential commits against one object, `raw/05-commit-timing.txt`):

| Operation | Closest §27 analog | Target / Maximum | Observed (S-scale) |
|---|---|---|---|
| `commit` (validate + one SQL transaction + advance head) | "Save/write acknowledgement ≤64 KiB": p95 150ms / max 750ms | p95 6.0ms, max 8.1ms — well inside |

No `O(history)` append: each `commit` writes exactly one new `revision` row and one `UPDATE` to `current_object`'s single pointer row, regardless of how many prior revisions the object already has — reading the object's full history (`history()`) is a separate, caller-invoked operation, never part of the write path. No dataset-M/L measurement (typed records don't exist before `P02`), consistent with the task's own cross-platform gate note.

## Durability gate

D1/D2 "all commit/ack boundaries, zero false Saved outcomes": `fault_before_sql_commit_leaves_zero_trace` proves a failure before `COMMIT` leaves the head and command table completely unchanged (no half-state — the underlying `rusqlite::Transaction`'s `Drop` rolls back every statement executed against it when `.commit()` is never called). `fault_after_sql_commit_reconciles_on_retry_instead_of_double_committing` proves the inverse: once `COMMIT` has actually run, the result is real and durable even if the caller never received it, and is reconcilable — never re-executed, never silently reported as failed when it actually succeeded ("zero false Saved outcomes" cuts both ways: this also proves the store never *loses* a real commit just because the acknowledgement was lost).

## Cross-platform gate

Native development profile only (Windows 11, MSYS/MinGW toolchain, NTFS), matching this task's own "Development native gate now; exact same matrix on all profiles T05-02" clause and the identical limitation already recorded at `T01-01`/`T01-02`.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Every storage boundary yields either complete pre-state or complete committed state | Satisfied — `fault_before_sql_commit_leaves_zero_trace` (pre-state), `create_object_commits_atomically_and_advances_head` (committed state) |
| Repeated request changes state exactly once | Satisfied — `duplicate_command_id_with_identical_input_replays_the_original_result`, `fault_after_sql_commit_reconciles_on_retry_instead_of_double_committing` |
| Full history reconstructs current state and head | Satisfied — `full_history_reconstructs_current_state_independent_of_pointer`, `reopened_store_independently_shows_the_committed_command_and_revision` |

## Completion condition

Every acceptance clause above is satisfied or explicitly, honestly scoped with recorded justification (multi-object/multi-operation commands, tombstone/delete, CLI wiring, full JCS, and literal multi-process kill testing all deferred, matching this task's own forbidden-scope boundary against successor work). Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (153/153). No sealed evidence altered; no force-push; no historical evidence file touched. `T01-03` is complete.

## Next frontier

`T01-04` — Preserve forensic bytes and recover to a verified new root. Depends on `T01-03` (this task). Acquires recovery ownership, preserves the complete guard/DB/journal before any engine recovery, and publishes a recovered working copy only after independent verification — for the format-2 store this task's `commit` now writes real state into.
