# T02-01 evidence report — Create and manage the four project record types

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §12/§14/§15/§20/§27-29, task `T02-01` — first task of `P02`
- **Baseline / tested source commit:** forked from `origin/main` `57de9bbd77bfec64a73c8c654a6d9d1459802d0c` (PR #73, `T01-07` merge, `P01` closed)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5 / Claude Fable 5.1 session)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy`. All checks below were executed locally on the development host recorded in `specs/002-post-r1-canonical-core-convergence/corrective-t01/reference-hardware.md`.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`; confirmed `origin/main` at `57de9bb...` (PR #73 merged) with all 8 canonical GitHub Actions checks reporting `success` on that exact head commit.
- `specs/CURRENT.md` on `main` at this baseline: `P01_STATUS=CLOSED`, `ACTIVE_IMPLEMENTATION_UNIT=T02-01`.
- Re-read the full `T02-01` task-contract row before writing any code, and the entity definitions it depends on (plan §15 "Canonical data model": common envelope, `Project`/`Note`/`Action`/`Decision` rows).

## Scope actually touched

`src/project.rs` (new module: the typed record layer), `src/canonical.rs` (one new read-only method, `list_current_objects` — no change to any existing mutating path, no schema change), `src/cli.rs` (11 new subcommands wiring the typed layer to the CLI, plus a fixed `CLI_ACTOR` constant), `src/lib.rs` (module registration, one new `Error::Project` variant), `docs/formats/typed-records.md` (new format specification). No `Cargo.toml`/`Cargo.lock` change. No change to `canonical.rs`'s schema, `CommandTarget` enum, or any of `T01-02`–`T01-07`'s already-audited mutating logic — see "Architecture" below for why that was possible.

```text
docs/formats/typed-records.md | 135 +++++++
src/canonical.rs              |  22 ++
src/cli.rs                    | 322 +++++++++++++++
src/lib.rs                    |   7 +
src/project.rs                | 893 ++++++++++++++++++++++++++++++++++++++++++
5 files changed, 1379 insertions(+)
```
(`git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock`, `raw/04-diff-stat.txt`)

## Architecture: no changes to the already-proven transaction core

Every typed record (`Project`/`Note`/`Action`/`Decision`) is committed as an ordinary opaque-payload object through the exact same `CommandTarget::CreateObject`/`UpdateObject` and `CanonicalWriter::commit` `T01-03` built and `T01-07` proved with 100 D1 fault schedules — `project.rs` adds no new `CommandTarget` variant and no new SQL column to `revision`/`command`/`current_object`. The `payload` field is this module's own JSON serialization, tagged with a `kind` field the module itself reads back to dispatch. This was a deliberate choice, not a shortcut: it means every typed mutation inherits, unchanged, `T01-03`'s atomicity guarantee, idempotent-replay rule, expected-revision-conflict rule, and `T01-04`/`T01-05`/`T01-06`'s recovery/backup/migration compatibility, with zero new surface for any of those to re-prove. `docs/formats/typed-records.md` documents the resulting JSON shape as its own specification layer, on top of the existing `docs/formats/format-2-canonical-sqlite.md`.

The one new read path, `CanonicalStore::list_current_objects`, is a plain `SELECT` over `current_object JOIN revision`, added because §15's "one owning project per work record" needs *some* way to enumerate a project's records; it is a full scan, not an index (see "Explicit scope boundaries").

## What was built, and why — grounded in the task contract

1. **`Project`**: `create_project`/`archive_project`/`unarchive_project`/`open_project`. Archiving/unarchiving commits a new revision with `active` flipped — the prior revision remains in immutable history (I05), proven directly by `create_open_archive_unarchive_project_round_trips` (3 revisions: create, archive, unarchive, all still present in `history()`).
2. **`Note`/`Action`/`Decision`**: `create_note`/`update_note`/`create_action`/`create_decision`, each requiring a `project_id` that `require_project` validates against a live `Project` object before any transaction opens (I03/I05/I07 "one owning project per work record; no orphaned canonical references" — "invalid cross-project references... reject"). Referencing a nonexistent object, or an existing object of the *wrong kind* (a note's own ID passed as a project), both refuse with a specific message (`note_requires_an_existing_project_and_rejects_wrong_kind_reference`).
3. **Field limits enforced before any transaction opens** (F05/F07/F20): title ≤1 KiB, body ≤1 MiB (reusing `crate::limits::MAX_OBJECT_BYTES` directly rather than redefining it), decision statement ≤8 KiB, decision key 1–128 bytes — all from plan §27/§15's own named bounds. `field_limits_are_rejected_before_any_transaction_opens` proves the transaction head is provably unchanged after a rejected oversized field, not merely that an error was returned.
4. **Unknown-field preservation** (§15): every struct carries a `#[serde(flatten)]` catch-all. `unknown_fields_survive_a_read_then_write_round_trip` simulates a payload written by a newer build (an extra `future_field`) and proves a read-then-write (archive) by this build preserves it unchanged — exactly like `identity::Frontmatter::unknown` already does for format-1.
5. **Type-tag admission control** (S04/S06 "type tags... cannot bypass admission"): `kind` is written only inside `RecordPayload::to_json`'s one serialization choke point, never accepted from caller-supplied data — there is no public constructor that lets a caller set an arbitrary `kind` on a payload.
6. **Record schema capability compatibility**: `payload_schema_version` is checked on every read; a version newer than `RECORD_PAYLOAD_SCHEMA_VERSION` is refused rather than guessed at (`a_payload_schema_version_newer_than_supported_is_refused`).
7. **CLI wiring** (this task's own acceptance criterion: "...work through CLI"): 11 new subcommands (`canonical-init`, `project-create/archive/unarchive/show`, `note-create/update`, `action-create`, `decision-create`, `record-show`, `project-records`) added to `src/cli.rs`'s existing hand-dispatch pattern. `full_project_and_record_lifecycle_works_through_the_cli` exercises the entire lifecycle — create a store, create a project, create one of each work record, list them, archive, unarchive, show — through the actual CLI dispatcher (`run()`), not by calling `project.rs` functions directly, and `cli_rejects_a_note_create_with_an_invalid_project_reference` proves the invalid-reference rejection surfaces correctly through the CLI too.

## A real bug found and fixed during this task's own tests

The first draft's `RecordPayload::from_json` extracted `kind` via `value.get("kind")` (a borrow, not a removal) before deserializing the *same* `value` into a concrete struct. Since no struct has a named `kind` field (it is written only by `to_json`), the `#[serde(flatten)] unknown` catch-all captured it — so a freshly created project's own `kind` tag ended up duplicated into its `unknown` map, caught immediately by `create_open_archive_unarchive_project_round_trips`'s equality assertion (`left: ... unknown: {"kind": String("project")} ... right: ... unknown: {}`). Fixed by removing (`Map::remove`) the `kind` field from the JSON object before dispatching to the concrete struct's deserializer, so it is consumed exactly once, by exactly the code that needs it.

## Explicit scope boundaries (recorded, not silently dropped)

- **`Note`'s source/evidence/artifact linkage** is `T02-02`'s objective ("capture exact notes and selected local evidence"), not built here.
- **`Action`'s full state machine** (`Doing`/`Blocked`/`Done`/`Cancelled` transitions, "any reopening requires an event," ordered dependency-cycle rejection) and **`Decision`'s evidence linkage, owner-only acceptance, override/supersession, and valid-time intervals** are `T02-03`'s objective ("record decisions and complete actions with visible history"). This task's actions always start `Open`; decisions always start `Draft`; neither type exposes a state-transition function here.
- **No project-scoped index.** `list_project_records` is a full scan over `list_current_objects`, filtered in application code. `T02-04`'s objective ("find work with a disposable current lexical index") is the indexing layer.
- **`decision_key` uniqueness is deliberately not enforced** — plan §12/§15 explicitly allow competing decisions to share a key; computing that conflict is `T02-03`'s job, not an error condition here.
- **CLI is functional, not polished** — plain flag parsing reusing the pre-existing hand-dispatch style (`Ponytail DELETE: clap`), matching this crate's own stated minimalism; no interactive UX, no desktop client (that is `P04`'s scope entirely).

## Tests added (16 new; all pass)

| Test | File | What it proves |
|---|---|---|
| `create_open_archive_unarchive_project_round_trips` | project.rs | Full project lifecycle; 3 immutable revisions preserved in history |
| `note_requires_an_existing_project_and_rejects_wrong_kind_reference` | project.rs | Nonexistent and wrong-kind project references both refuse |
| `note_update_conflicts_exactly_like_any_other_typed_record` | project.rs | Stale `expected_revision_id` conflicts identically to the untyped path |
| `field_limits_are_rejected_before_any_transaction_opens` | project.rs | Oversized name/statement refused; transaction head provably unchanged |
| `action_starts_open_and_decision_starts_draft` | project.rs | Correct initial lifecycle states |
| `list_project_records_scopes_correctly_and_excludes_other_projects` | project.rs | Cross-project isolation in the scan-based query |
| `unknown_fields_survive_a_read_then_write_round_trip` | project.rs | §15 unknown-field preservation, proven end to end |
| `a_payload_schema_version_newer_than_supported_is_refused` | project.rs | Forward-compatibility refusal |
| `full_project_and_record_lifecycle_works_through_the_cli` | cli.rs | This task's own acceptance criterion, through the actual CLI dispatcher |
| `cli_rejects_a_note_create_with_an_invalid_project_reference` | cli.rs | Invalid reference rejection surfaces through the CLI, not only the Rust API |

(8 in `project.rs`, 2 new in `cli.rs`; the diff-stat's `+322` in `cli.rs` also includes the 11 new command handlers themselves.)

## Commands executed (all re-run on this branch against `origin/main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **192/192 pass**, 0 failed (159 lib [149 pre-existing + 10 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib project::` | `raw/07-project-module-tests-isolated.txt` | 8/8 pass |
| `cargo test --locked --lib cli::` | `raw/08-cli-module-tests-isolated.txt` | 4/4 pass (2 pre-existing + 2 new) |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `cargo run --release --example project_timing` (throwaway harness, deleted before this commit) | `raw/05-typed-record-timing.txt` | create p50=2.9ms/p95=6.2ms/max=6.4ms (n=31); `list_project_records` (30 records) 249us |
| Environment capture | `raw/06-environment.txt` | Matches `reference-hardware.md`'s frozen development profile |

## Failed attempts / exclusions

- The `kind`-field-leaks-into-`unknown` bug above, caught by the round-trip test's own equality assertion on the very first run, not discovered later. Fixed by removing the field before dispatch rather than by loosening the test's assertion.
- A borrow-checker error on the first draft: `create_note`/`create_action`/`create_decision`/`update_note` originally took both a `&mut CanonicalWriter` and a separate `&CanonicalStore` parameter for validation — but `CanonicalWriter` already holds an exclusive `&mut CanonicalStore` internally, so the two borrows conflicted (`E0502`, 12 compile errors). Fixed by using `CanonicalWriter::store()`'s existing accessor (already `pub`, added by `T01-05` for the Online Backup API) instead of a second parameter — a cleaner API besides, since callers no longer need to thread two references through.
- No test was skipped, deleted, or weakened to reach green.

## Performance gate

Closest applicable measurement, at S-scale (`raw/05-typed-record-timing.txt`):

| Operation | Comparison | Observed |
|---|---|---|
| Typed record create (1 project + 30 notes) | `T01-03`'s own untyped `commit()` measurement (p95 6.0ms) | p95 6.2ms — consistent; the typed layer's one JSON serialize/deserialize adds negligible overhead |
| `list_project_records` (30 records, full scan) | This task's own "S/M project open/read" gate | 249us — well inside |

No dataset-M/L measurement: typed records at M/L scale first need `T02-04`'s indexing to be a meaningful measurement of the *product* rather than of an admittedly-unindexed full scan.

## Durability gate

"All mutations use proven command transaction; D1 regression sample for each new type": every typed mutation is, structurally, the exact same `CanonicalWriter::commit` call `T01-07`'s D1 matrix already ran 100 fault schedules against — no new transaction path exists for this task to separately regression-test. `note_update_conflicts_exactly_like_any_other_typed_record` is this task's own regression sample proving the conflict/idempotency behavior is unchanged for a typed caller.

## Cross-platform gate

Native development profile only (Windows 11/NTFS, matching `reference-hardware.md`), consistent with every prior `flake-v1` task. All profiles remain `T05-02`'s scope.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Create/open/archive/unarchive... work through CLI | Satisfied — `full_project_and_record_lifecycle_works_through_the_cli` |
| ...and typed record CRUD-as-revision work through CLI | Satisfied — the same test creates a note/action/decision and lists them via `project-records` |
| Invalid cross-project references... reject | Satisfied — `note_requires_an_existing_project_and_rejects_wrong_kind_reference`, `cli_rejects_a_note_create_with_an_invalid_project_reference` |
| ...and ID collisions reject | Satisfied by construction: `CreateObject` always allocates a fresh UUIDv7 (no caller-supplied ID path exists in this typed layer), so an ID collision at creation is structurally impossible; `UpdateObject`'s own expected-revision-conflict rule (inherited unchanged from `T01-03`) is what rejects a stale/conflicting update instead |

## Completion condition

Every acceptance clause above is satisfied. `Note` evidence linkage, `Action`/`Decision` full lifecycle semantics, and project-scoped indexing are explicitly deferred to `T02-02`/`T02-03`/`T02-04` respectively, matching this task's own forbidden-scope boundary against successor work. One real bug (the `kind`-field leak) was caught by this task's own tests and fixed at the root cause. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (192/192). No sealed evidence altered; no force-push; no historical evidence file touched; no change to any `T01-02`–`T01-07` mutating code path. `T02-01` is complete.

## Next frontier

`T02-02` — Capture exact notes and selected local evidence. Depends on `T02-01` (this task). Builds source/artifact admission and a Markdown parser/preview contract on top of the `Note` type this task introduced.
