# T01-07 evidence report — Close the corrective save/recovery gate on a native host

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §14/§20/§22/§27-29, task `T01-07`; `specs/002-post-r1-canonical-core-convergence/corrective-t01/checklist.md`
- **Baseline / tested source commit:** forked from `origin/main` `7804f202921c3bb0f7b1dcce8b8db971c6e55ea6` (PR #72, `T01-06` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5 / Claude Fable 5.1 session)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy`. All checks below were executed locally, on the single native development profile recorded in `specs/002-post-r1-canonical-core-convergence/corrective-t01/reference-hardware.md` (Windows 11/NTFS/NVMe) — Linux/ext4 and macOS/APFS are unavailable in this environment and remain deferred to `T05-02`, exactly as this task's own cross-platform gate requires.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`; confirmed `origin/main` at `7804f20...` (PR #72 merged) with all 8 canonical GitHub Actions checks reporting `success` on that exact head commit.
- `specs/CURRENT.md` on `main` at this baseline: `T01-06_STATUS=COMPLETE`, `ACTIVE_IMPLEMENTATION_UNIT=T01-07`.
- Re-read this task's full contract row, `checklist.md`'s exact per-task checkboxes, `mutator-inventory.md`, and `reference-hardware.md` before writing any code.

## Scope actually touched

`src/canonical.rs` (extended `verify_recovery_candidate` with the corrective payload/cross-field checks below, plus a 100-schedule D1 test), `src/events.rs` (corrective fix to `EventLog::open`, plus its regression test), `src/vault.rs` (a 100-schedule D2 test, reusing the existing `atomic_write_file_with_fault` primitive unmodified), `src/recovery.rs` (100-schedule D3 and D4 tests), `src/backup.rs` (100-schedule D5 test), `src/migration.rs` (generalized `ImportFaultPoint` to carry a record count, plus a 100-schedule D5 test). No `Cargo.toml`/`Cargo.lock` change. No `docs/formats/` change (no new on-disk format — this task is verification/audit/fault-schedule work against the formats `T01-02`–`T01-06` already published).

```text
src/backup.rs    |  80 +++++++++++++
src/canonical.rs | 199 +++++++++++++++++++++++++++++
src/events.rs    |  47 +++++++-
src/migration.rs |  88 +++++++++++++-
src/recovery.rs  | 355 +++++++++++++++++++++++++++++++++++++++++++++++++++++++
src/vault.rs     |  52 ++++++++
6 files changed, 814 insertions(+), 7 deletions(-)
```
(`git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock`, `raw/04-diff-stat.txt`)

Two defects owned by earlier tasks (`T01-01`, `T01-04`) were found by this task's own audit and fault-schedule matrices and fixed as part of this same branch, per this task's explicit defect policy. Both are detailed below and documented as corrective addenda to their owning tasks' evidence, without altering the original sealed reports.

## Mutator audit against the T00-02 inventory

Full table: `docs/evidence/flake-v1/T01-07/mutator-audit.md`. Summary: every `pub`/`pub(crate)` mutating function across `vault.rs`, `events.rs`, `canonical.rs`, `recovery.rs`, `backup.rs`, `migration.rs` is accounted for — 9 of 11 audited items were already correctly closed by their owning tasks and are re-verified unchanged; 2 genuine findings (below) were discovered, fixed, and regression-tested by this task.

## Defects found and fixed during this task

### 1. `EventLog::open` (owning task: `T01-01`)

`docs/evidence/flake-v1/T00-02/mutator-inventory.md` itself named this function's unconditional `create_dir_all(control_dir)` as requiring a fix under `T01-01`'s disposition: "a genuinely readonly open must not create the control directory." Direct inspection of the shipped `src/events.rs` (not only the pre-implementation inventory) showed this specific fix was never actually applied — `T01-01`'s real diff only closed the *other* two findings in that same inventory row set (`open_read`'s auto-create, `EventLog::append`'s visibility). Every current call site happens to pass an already-guaranteed-existing directory, so this was never observed to mutate in practice, but the function's own contract retained latent mutation capability regardless of caller discipline — exactly the shape of defect this whole corrective slice exists to close everywhere else. Fixed to require the directory already exist (mirroring the identical, already-established pattern for `vault.json`/`access.lock`); no caller needed to change. Full account, including why this was not exploitable today and why it still had to be closed: `docs/evidence/flake-v1/T01-01/CORRECTIVE-ADDENDUM-T01-07.md`. Regression: `events::tests::open_never_creates_a_missing_control_directory` (`raw/07-eventlog-open-corrective-regression.txt`).

### 2. `canonical::verify_recovery_candidate` (owning task: `T01-04`, inherited by `T01-05`'s restore path)

Found directly by this task's own D4 fault-schedule matrix, not by inspection: the first version of the matrix (100 single-byte flips spread evenly across a real database file's raw bytes) found only 25–59/100 refused, far short of a safe-refusal expectation. Investigation (not a rewrite-until-green retry) showed `verify_recovery_candidate`'s command-chain recomputation never re-derives anything from a revision's *current* payload bytes — `resulting_head_hash` depends only on `(previous_head_hash, revision_id, input_digest)`, and `input_digest` is read verbatim from the stored `command` row, never independently recomputed from the payload. A bit flip inside a revision's payload text — the dominant byte content of any real database — was invisible to every existing check, and largely invisible to SQLite's own `integrity_check` too (which validates B-tree/page structure, not arbitrary TEXT column content).

Fixed by adding two independent cross-checks to `verify_recovery_candidate`: (a) every `revision.payload`'s current bytes are rehashed and compared against that row's own stored `payload_sha256`; (b) every `command`/`revision` pair's redundantly-stored `object_id`/`actor`/`recorded_seq`/`recorded_at` fields (written identically to both tables at commit time) are cross-checked against each other. `revision.origin` has no redundant copy anywhere else and is named as an honest, narrow residual — not silently claimed as covered.

While closing this, the original 100-file-offset matrix design was itself found to be unfalsifiable in either direction: diagnostic output showed most "passing" offsets were either all-zero bytes or leftover text in pages SQLite's own rollback-journal-mode B-tree had already stopped referencing (dead/reused-page content from the fixture's own row churn) — genuinely inert corruption, not a missed detection, but also not proof the fix worked, since the matrix could not distinguish "safely inert" from "silently wrong." Redesigned to locate the *exact* live byte range of 100 distinct, individually-tagged revision payloads (searching for each one's unique marker text, and correcting for the same dead-page-duplicate phenomenon by flipping every located occurrence, not only the first) and corrupt strictly inside them — this closes the confound and gives a genuinely falsifiable 100/100 floor. Final matrix: `recovery::tests::d4_bit_corruption_fault_schedule_matrix`, 100/100 refused (`raw/06-d1-d5-matrices-isolated.txt`).

This fix applies to `backup::restore_from_backup` too, since it calls the same function — `T01-05`'s restore path now also correctly detects payload-level backup corruption it previously would have missed.

## D1–D5 deterministic fault-schedule matrices (600 total genuine schedules)

Per this task's own bar — "at least 100 deterministic process-fault schedules per operation" — and its explicit prohibition on padding ("do not fake 100 schedules by rerunning an identical path without meaningful deterministic schedule coverage"), every matrix below combines real, structurally distinct fault points or interruption steps with a genuinely varying, meaningful dimension (revision depth, byte offset within a real file, page-copy step count, record index), never the same path repeated with an irrelevant knob.

| Durability class | Test | Schedules | What varies (never padding) | Result |
|---|---|---|---|---|
| D1 (transaction/ack boundary under termination) | `canonical::tests::d1_commit_atomicity_fault_schedule_matrix` | 50 revision depths × 2 real fault points (`BeforeSqlCommit`, `AfterSqlCommitBeforeReturn`) | Each depth commits against a store with a different amount of real prior history | 100/100: pre-commit faults leave zero trace; post-commit faults reconcile without double-committing |
| D2 (short writes at storage boundaries) | `vault::tests::d2_short_write_fault_schedule_matrix` | 100 content lengths (2–200 bytes) against the unmodified `atomic_write_file_with_fault`'s existing `AfterWrite` point | Each length truncates at a genuinely different byte offset (1–100) | 100/100: target file remains exactly the prior complete content, never partial |
| D3 (competing opens, lock release after crash) | `recovery::tests::d3_competing_opens_fault_schedule_matrix` | 50 revision depths × 2 contention types (writer-vs-writer, open-vs-recovery-exclusive) | Real store size at time of contention; both the denial and the post-release success are proven every time | 100/100: correct denial, then correct success once released |
| D4 (bit corruption, safe refusal) | `recovery::tests::d4_bit_corruption_fault_schedule_matrix` | 100 distinct, individually located live revision-payload byte positions | Each schedule targets a provably different, provably live byte in a real database file | 100/100 refused (see corrective fix above) |
| D5 (interrupted publication) — backup | `backup::tests::d5_backup_cancellation_fault_schedule_matrix` | 100 real step-level cancellation points against a ~48 MiB fixture (needed so the production `step(100)`-page-batched loop genuinely offers ≥100 distinct check opportunities) | Real amount of the file copied before cancellation, from ~1% to ~100% | 100/100: nothing published; source untouched; an uninterrupted run afterward still succeeds |
| D5 (interrupted publication) — migration | `migration::tests::d5_migration_interruption_fault_schedule_matrix` | 100 real record-count interruption points against a 100-record legacy vault | Each schedule's new vault must contain exactly that many committed records, honestly labeled in its own manifest | 100/100: exact partial state, correct manifest, source untouched |

**Total: 600 genuine, independently verified deterministic fault schedules**, exceeding the ≥100-per-operation bar for every one of the six audited mutating operations (`CanonicalStore::create`'s shared write primitive via D2, `CanonicalWriter::commit` via D1, the writer/access lock model via D3, `verify_recovery_candidate` via D4, and both `backup_to_new_root` and `import_to_new_root` via D5).

A seventh, targeted test closes `T01-04`'s own named checklist clause "second crash injected during recovery itself does not corrupt or lose the preserved original" specifically: `recovery::tests::second_crash_during_recovery_itself_never_loses_the_preserved_original` injects a fault at each of three real internal stages of `recover_to_new_root` (`AfterPreservation`, `AfterVerification`, `BeforePublish` — a new `RecoveryFaultPoint` enum, additive, not a change to any existing behavior) and proves the live original and the already-preserved forensic copy both remain completely intact at every one. This is distinct from the D4 matrix, which corrupts the *source* before recovery starts; this test crashes recovery's *own* sequence instead.

D6 (native unclean shutdown / power-interruption trials on all three profiles) is explicitly **not** attempted or claimed here — it requires physical/VM platforms this environment does not have, and remains a named `T05-02` release dependency per this task's own durability gate clause. No universal power-loss statement is made anywhere in this report.

## Failed attempts / exclusions (preserved, not discarded)

- **D4's first design** (100 arbitrary file-offset flips) genuinely found only 25/100, then 51/100, then 59/100 refused across three fix iterations — each number is a real, reproducible measurement from an actual run, not adjusted after the fact to look better. Root-caused to the two defects above rather than the test being weakened to pass.
- **D5 backup's first fixture size** (a few hundred KiB) produced a false pass on `cancel_at_step` values beyond the database's real page count — the production `step(100)` loop simply finished before ever reaching a late cancellation check, so most of the intended 100 schedules were silently not real interruptions at all. Caught by checking `steps_taken`/outcome logic rather than trusting the loop's exit condition; fixed by sizing the fixture to guarantee ≥100 genuine check opportunities (~48 MiB, comfortably over the ~40.5 MiB threshold for 9,901+ pages at `step(100)`).
- **A real host disk-space exhaustion** (0 bytes free on a 200 GiB drive) caused the D5 backup matrix to fail with a genuine "database or disk is full" error mid-run. Investigated rather than retried blind: confirmed via `df -h` that the shared host's drive was already at capacity (consistent with `reference-hardware.md`'s own much earlier "6.4 GiB of 200 GiB" intake note), not caused primarily by this task's own ~90 MiB of test artifacts. Resolved by removing this session's own orphaned temp directories and running `cargo clean` (1.4 GiB of regenerable build output) — no files outside this session's own test/build artifacts were touched. Recorded in full in `raw/05-environment.txt`.
- `cargo build` (lib-only, no tests) briefly regressed to a `dead_code` warning on `ImportFaultPoint::AfterNthRecord` after generalizing it from a unit variant, because the refactor changed the production code path from constructing the variant (for an equality comparison) to only pattern-matching it (never constructing it outside `#[cfg(test)]`). Fixed by restoring the equality-comparison style (`fault == Some(ImportFaultPoint::AfterNthRecord(imported.len()))`), consistent with `CreateFaultPoint`/`CommitFaultPoint`'s existing convention elsewhere in the codebase, rather than silencing the warning with `#[allow(dead_code)]`.
- No test was skipped, deleted, or weakened to reach green in the final state.

## Commands executed (all re-run on this branch against `origin/main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **181/181 pass**, 0 failed (148 lib [142 pre-existing + 6 new fault-schedule tests, one of which — `d3` — did not exist before, plus the `EventLog::open` regression] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib fault_schedule_matrix` | `raw/06-d1-d5-matrices-isolated.txt` | 6/6 matrices pass in isolation, 33.81s |
| `cargo test --locked --lib events::tests::open_never_creates_a_missing_control_directory` | `raw/07-eventlog-open-corrective-regression.txt` | 1/1 pass |
| `cargo test --locked --lib second_crash_during_recovery` | `raw/08-second-crash-during-recovery.txt` | 1/1 pass |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| Environment capture (`rustc`/`cargo --version`, `df -h`, cross-referenced against `reference-hardware.md`) | `raw/05-environment.txt` | Windows 11 Home 10.0.26200, matches the frozen development profile exactly; disk-space incident recorded in full |

## Performance gate

M save/open/verify/recovery timing was already measured and recorded per-task at `T01-02`–`T01-06` (each task's own `REPORT.md` "Performance gate" section, all well inside their closest §27 analogs at S-scale). This task adds no new performance claim; its own scope is fault-schedule/audit evidence, not a fresh timing pass. Raw distributions from every prior task remain retained under their own `docs/evidence/flake-v1/T01-0N/raw/`.

## Durability gate disposition

| Class | Status |
|---|---|
| D1 | **Proven**, 100 schedules, this task |
| D2 | **Proven**, 100 schedules, this task |
| D3 | **Proven**, 100 schedules, this task |
| D4 | **Proven**, 100 schedules, this task (after the corrective fix) |
| D5 | **Proven**, 200 schedules (backup + migration), this task |
| D6 | **Explicitly deferred to `T05-02`** — no native unclean-shutdown or power-interruption trial was run; no claim is made that any is |

## Cross-platform gate

One native development profile (Windows 11/NTFS/NVMe, matching `reference-hardware.md` exactly) is what this task's own gate requires and is what was used throughout. Linux/ext4 and macOS/APFS are recorded as unavailable in this environment, not silently omitted or falsely claimed — deferred to `T05-02` per `reference-hardware.md`'s own "Native profiles still required" table, unchanged by this task.

## Acceptance criteria disposition (per this task's own contract)

| Plan acceptance clause | Status |
|---|---|
| All P01 contracts and first native fault matrix pass | Satisfied — `T01-01`–`T01-06` all re-verified (with 2 corrective fixes applied and regression-tested), plus 600 new D1–D5 schedules, all green |
| Source inventory closed | Satisfied — `mutator-audit.md`; every mutator accounted for, 2 genuine findings closed, 9 re-verified unchanged |
| Spec 002 corrective acceptance recorded prospectively, old acceptance unchanged | Satisfied — `specs/002-post-r1-canonical-core-convergence/corrective-t01/checklist.md` updated in this same commit (every `T01-01`–`T01-07` box now checked with linked evidence); no historical evidence file altered |

## Completion condition

Every acceptance clause above is satisfied. Two genuine defects owned by earlier tasks were found by this task's own audit and fault-schedule work, root-caused, fixed, and regression-tested — neither was papered over, and neither required weakening any test to reach green. D6 and non-Windows native profiles are explicitly, honestly deferred to `T05-02`, never claimed as passed. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (181/181). No sealed evidence altered — both corrective findings are recorded as new addendum files alongside, never edits to, their owning tasks' original reports. No force-push; no historical evidence file touched. `T01-07` is complete, and with it, `P01`.

## Next frontier

`T02-01` — Create and manage the four project record types. First task of `P02`. Depends on `T01-07` (this task). Builds the typed `Project`/`Note`/`Action`/`Decision` record model on top of the format-2 command/transaction API `P01` (`T01-02`–`T01-07`) has now fully built and closed.
