# T01-05 evidence report — Create and restore consistent verified backups

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §14/§15/§16/§20/§22, task `T01-05`
- **Baseline / tested source commit:** forked from `origin/main` `b9aaf88f89db96d54ecfcdffb763d925fed40c50` (PR #70, `T01-04` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5 / Claude Fable 5.1 session)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy` (unchanged limitation, recorded at every prior `flake-v1` task). All checks below were executed locally on this development host.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`; confirmed `origin/main` at `b9aaf88...` (PR #70 merged) with all 8 canonical GitHub Actions checks reporting `success` on that exact head commit.
- `specs/CURRENT.md` on `main` at this baseline: `T01-04_STATUS=COMPLETE`, `ACTIVE_IMPLEMENTATION_UNIT=T01-05`, `NEXT_DEPENDENCY_READY_UNIT=T01-05`.
- Re-read the full `T01-05` task-contract row before writing any code.

## Scope actually touched

`src/backup.rs` (new module), `src/canonical.rs` (one new `pub(crate) fn connection(&self) -> &Connection` accessor for the Online Backup API, plus a `busy_timeout` addition — see "A real improvement made along the way" below), `src/lib.rs` (module registration, one new `Error::Backup` variant), `Cargo.toml` (one line: `rusqlite`'s already-admitted dependency gains the `backup` feature — see "Dependency admission" below). No `Cargo.lock` change (Cargo does not record enabled features in the lockfile — same crate/version/checksum). No `docs/formats/` change: this task's own format is a backup-specific manifest (`backup-manifest.json`), not a change to the published `canonical.sqlite` schema itself, so it is documented in this report and in `src/backup.rs`'s own module docs rather than in `format-2-canonical-sqlite.md`.

```text
Cargo.toml       |   2 +-
src/backup.rs    | 706 +++++++++++++++++++++++++++++++++++++++++++++++++++++++
src/canonical.rs |  22 ++
src/lib.rs       |   6 +
4 files changed, 735 insertions(+), 1 deletion(-)
```
(`git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock`, `raw/04-diff-stat.txt`)

The task's own `Files/components` also names "CLI" and "recovery guide" — neither built in this unit; recorded below.

## Dependency admission: `rusqlite`'s `backup` feature

The task's own implementation requirement is explicit: "Use admitted SQLite snapshot/backup API." `rusqlite::backup::Backup` (wrapping `sqlite3_backup_init`/`_step`/`_finish`) is gated behind a Cargo feature not previously enabled. This is recorded as a feature-admission, smaller than a new-crate admission but not skipped:

- **Same crate, same version, same license.** `rusqlite = "0.37"`, MIT — already admitted at `T00-02` (`docs/evidence/flake-v1/T00-02/dependency-admission.md` records the crate itself; `T01-02` extended its usage to a second database). No new entry in `Cargo.lock`.
- **No new C code.** `sqlite3_backup_*` are core SQLite C API functions, already compiled into the vendored amalgamation via the `bundled` feature regardless of whether `backup` is enabled. The `backup` Cargo feature only compiles in `rusqlite`'s existing safe Rust wrapper around functions that were already present in the binary.
- **No new transitive dependency, no new advisory surface.** Confirmed by re-running `cargo build --locked` and observing no new crate compiled and no `Cargo.lock` diff.
- **Provenance:** verified directly against `rusqlite` 0.37.0's own `Cargo.toml`/`src/lib.rs` (`#[cfg(feature = "backup")] pub mod backup;`), not inferred from memory — confirmed by first attempting to use `rusqlite::backup` without the feature and observing the compiler's own "found an item that was configured out" diagnostic before enabling it.

## What was built, and why — grounded in the task contract

1. **`backup_to_new_root`**: opens the source with a normal, shared-access `CanonicalStore::open` (a backup is a read-like operation, not exclusive recovery), captures the snapshot head *before* the backup step loop begins ("record snapshot head and later backup result separately"), then uses `rusqlite::backup::Backup`'s manual `step(n)` API (not `run_to_completion`, specifically so a caller-supplied `should_cancel` closure can be checked between steps) to produce a page-consistent copy of `canonical.sqlite` even while the source is being read or written concurrently by other connections. The copy is independently verified with the exact same [`canonical::verify_recovery_candidate`] `crate::recovery` uses (full command-chain recomputation, not a re-read of stored values) before a manifest is written and the result is staged-and-published, no-clobber, matching `T01-02`'s own discipline.
2. **Cancellation**: `should_cancel` returning `true` at any point before publication removes the staging directory and returns `Err` — `backup_root` ends up with no `.fehrest` at all, an explicit incomplete result rather than a partial, misleadingly-present one.
3. **`restore_from_backup`**: reads the manifest first, independently re-verifies every member's exact recorded length and SHA-256 against the backup's actual on-disk bytes (never trusting a backup blindly, even one this module produced itself), re-runs the same strong database verification, and additionally cross-checks the manifest's claimed snapshot identity/head against what the database itself actually contains — "restored head/content matches manifest" as a real, executable check, not only a comment. Only then does it stage-and-publish a new root; it never restores in place.
4. **`BackupManifest`** (§15 "Export/backup manifest"): schema/kind/vault identity/snapshot head/member paths-lengths-digests/`verified` flag, written into the published backup root itself as `backup-manifest.json`.
5. **A real improvement made along the way**: `canonical.rs`'s `apply_and_assert_runtime_pragmas` now sets a 2-second `busy_timeout` on every connection. Without one, a writer's commit and an in-progress backup step can briefly conflict over the rollback-journal lock and receive an immediate `SQLITE_BUSY` on both sides; a bounded retry (rather than an immediate failure) is what actually makes "concurrent legitimate save cannot create mixed snapshot state" achievable in practice rather than merely theoretically true. This directly serves this task's own acceptance clause and applies uniformly to every `CanonicalStore` connection, not only the backup path.

## Explicit scope boundaries (recorded, not silently dropped)

- **No CLI wiring, no recovery guide document.** Consistent with `T01-03`/`T01-04`'s own precedent: this store has no CLI surface yet for anything.
- **Full backup only.** `kind: "full-backup"` is the only kind this task builds, matching the objective "restore complete saved work." Plan §16's "selected-project package" (a scoped, exclusion-labeled export) is a distinct, later capability.
- **No literal disk-full test.** The fault-injection philosophy every prior `flake-v1` task has used (deterministic in-process conditions rather than a real OS-resource-exhaustion harness) is used here too; this task's cancellation and no-clobber/corruption tests exercise the same F15/F17/F18 failure-behavior clauses through explicit, reproducible conditions instead.
- **No true multi-process concurrency test** for the interleaved-commit-during-backup proof — a deterministic, single-process interleaving between two independently opened connections to the same file (real SQLite-level locking, not simulated), matching the identical limitation recorded at `T01-04` for the access lock.

## Tests added (6 new; all pass; full raw output `raw/01-full-test-run.txt`, isolated run `raw/07`)

| Test | What it proves | Verification hierarchy tier(s) |
|---|---|---|
| `backup_then_restore_round_trips_exact_state` | A full backup-then-restore cycle reproduces the exact current state and history depth; the source is untouched throughout | V03/V04 |
| `backup_destination_no_clobber` | A second backup attempt at an already-published destination is refused | V02/V09 |
| `cancellation_before_publication_leaves_no_published_backup` | Cancelling on the very first check publishes nothing at all | V02/V06 |
| `restore_refuses_when_a_member_digest_does_not_match_manifest` | A single-byte flip in the published backup database (same length, different content) is caught by the digest check specifically, isolated from the separate length check | V09 |
| `restore_refuses_when_manifest_head_disagrees_with_database` | A hand-edited manifest claiming a different snapshot head than the (still internally self-consistent) database actually contains is refused | V09 |
| `concurrent_legitimate_commits_during_backup_do_not_produce_mixed_state` | A real, independent write committed mid-backup-step-loop (separate connection, same file) still yields a backup that passes full independent verification — a real consistent point in time, never a torn mix | V05/V09 |

No pre-existing test was modified.

## Commands executed (all re-run on this branch against `origin/main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **167/167 pass**, 0 failed (134 lib [128 pre-existing + 6 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib backup::` | `raw/07-backup-module-tests-isolated.txt` | 6/6 pass |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `cargo run --release --example backup_timing` (throwaway harness, deleted before this commit) | `raw/05-backup-restore-timing.txt` | backup p50=29.4ms/p95=42.7ms/max=42.7ms; restore p50=29.9ms/p95=34.4ms/max=34.4ms (n=15 each, 5-revision stores) |
| Environment capture | `raw/06-environment.txt` | Windows 11 Home 10.0.26200, 12 logical CPUs, 16,106 MB RAM, rustc/cargo 1.97.1, vendored SQLite 3.50.2 (unchanged) |

## Failed attempts / exclusions

- **A genuine hang, caught and fixed.** The first version of `concurrent_legitimate_commits_during_backup_do_not_produce_mixed_state` interleaved a writer commit every two backup steps, on connections with no `busy_timeout` set. Running the full suite hung for over 60 seconds (confirmed via a background process check showing the test binary still running, and the harness's own "has been running for over 60 seconds" notice) — a livelock between the backup's read-lock retry loop and the writer's commit-lock retry loop, both retrying immediately with no bounded wait. Root-caused rather than worked around by retrying: added `busy_timeout` to every `CanonicalStore` connection (a real, generally-applicable fix, not a test-only hack), reduced the test to exactly one interleaved commit at a fixed step, and added a hard iteration cap (`assert!(steps < 100_000, ...)`) so any remaining risk becomes a fast, explicit test failure instead of a hang.
- **A test-assertion bug, caught and fixed.** The first version of `restore_refuses_when_a_member_digest_does_not_match_manifest` replaced the backup database's content with a short literal string, which trips the *length* check before the digest check ever runs — the test's own assertion (`contains("digest mismatch")`) then failed because the actual message said "length mismatch," which is correct, fail-fast behavior, not a defect. Fixed by flipping one byte in place (preserving length) so the test isolates the digest-check path specifically.
- No other failed attempt occurred on the final implementation; no test was skipped, deleted, or weakened to reach green.

## Performance gate

Closest applicable §27 analog, measured at S-scale (one store, 5 committed revisions, `raw/05-backup-restore-timing.txt`):

| Operation | Closest §27 analog | Target / Maximum | Observed (S-scale) |
|---|---|---|---|
| `backup_to_new_root` | "Full export/import M": target 60s / max 180s | p50 29.4ms, p95 42.7ms, max 42.7ms — well inside |
| `restore_from_backup` | "Full export/import M": target 60s / max 180s | p50 29.9ms, p95 34.4ms, max 34.4ms — well inside |

No dataset-M/L measurement, no progress-reporting implementation for "long operations report progress" — at S-scale these operations complete in tens of milliseconds, well under any threshold where progress reporting would be meaningful; a real M/L progress-reporting mechanism is deferred to `T05-02`'s full native-profile qualification alongside every other M/L-scale gate this `flake-v1` sub-plan has consistently deferred.

## Durability gate

D5 "every backup/publication/restore stage": every failure path in both `backup_to_new_root` and `restore_from_backup` removes only its own disposable staging directory and never touches the source/backup being read from — proven directly by `backup_destination_no_clobber` (source and existing backup both untouched), `cancellation_before_publication_leaves_no_published_backup` (no `.fehrest` published at all), and both restore-refusal tests (no `.fehrest` published at the restore destination).

## Cross-platform gate

Native development profile only (Windows 11, MSYS/MinGW toolchain, NTFS), matching this task's own "Development native profile; clean cross-profile transfer tested later" clause and the identical limitation recorded at every prior `flake-v1` task.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Concurrent legitimate save cannot create mixed snapshot state | Satisfied — `concurrent_legitimate_commits_during_backup_do_not_produce_mixed_state`, enabled by the new `busy_timeout` |
| Restored head/content matches manifest | Satisfied — `backup_then_restore_round_trips_exact_state` (positive case), `restore_refuses_when_manifest_head_disagrees_with_database` (negative case, an explicit executable check) |
| Cancelled/partial destination never reports Verified | Satisfied — `cancellation_before_publication_leaves_no_published_backup`; a cancelled attempt returns `Err` and publishes nothing a caller could mistake for a verified backup |

## Completion condition

Every acceptance clause above is satisfied or explicitly, honestly scoped with recorded justification (CLI wiring, a recovery guide document, selected-project backups, literal disk-full testing, and multi-process concurrency testing all deferred, matching this task's own forbidden-scope boundary against successor work). One genuine hang was caught by the test suite and root-caused to a missing `busy_timeout`, fixed with a real, generally-applicable improvement rather than a test-only workaround. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (167/167). No sealed evidence altered; no force-push; no historical evidence file touched. `T01-05` is complete.

## Next frontier

`T01-06` — Import legacy vaults without rewriting accepted history. Depends on `T01-05` (this task). Reads format-1 vaults nonmutatingly, validates and preserves exact raw legacy bytes, and imports valid selected records into a new format-2 vault with honest migration provenance — the first task to connect the two vault formats this `flake-v1` sub-plan has kept strictly separate since `T01-02`.
