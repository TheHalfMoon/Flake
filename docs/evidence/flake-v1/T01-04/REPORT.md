# T01-04 evidence report — Preserve forensic bytes and recover to a verified new root

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §14/§16/§20/§22, task `T01-04`
- **Baseline / tested source commit:** forked from `origin/main` `b59d39edf92c3214f68903f5af4977a03c1346fa` (PR #69, `T01-03` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5 / Claude Fable 5.1 session)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy` (unchanged limitation, recorded at every prior `flake-v1` task). All checks below were executed locally on this development host.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`; confirmed `origin/main` at `b59d39e...` (PR #69 merged) with all 8 canonical GitHub Actions checks reporting `success` on that exact head commit.
- `specs/CURRENT.md` on `main` at this baseline: `T01-03_STATUS=COMPLETE`, `ACTIVE_IMPLEMENTATION_UNIT=T01-04`, `NEXT_DEPENDENCY_READY_UNIT=T01-04`.
- Re-read the full `T01-04` task-contract row and plan §14 (component/process boundaries — the two-lock model) in full before writing any code.

## Scope actually touched

`src/recovery.rs` (new module: forensic preservation, independent verification, staged recovery-to-new-root, incident manifest), `src/canonical.rs` (extended: `verify_recovery_candidate` and `RecoveryVerification`, `access.lock` creation during staged publication, `CanonicalStore::open` now requires and holds shared access for its lifetime), `src/vault.rs` (new `AccessGuard` primitive plus the `ACCESS_LOCK_FILE` constant — **not** wired into format-1's `Vault`, see "A design correction" below), `src/lib.rs` (module registration, one new `Error::Recovery` variant), `docs/formats/format-2-canonical-sqlite.md` (extended: access-lock/recovery model, incident manifest shape). No `Cargo.toml`/`Cargo.lock` change: the shared/exclusive lock primitive uses `std::fs::File`'s stable advisory-lock API (`lock_shared`/`try_lock`/`try_lock_shared`/`unlock`), stable since Rust 1.89 — this crate's `rust-version` is `1.97` — so no new dependency was admitted or needed.

```text
docs/formats/format-2-canonical-sqlite.md |  79 +++-
src/canonical.rs                          | 212 ++++++++++-
src/lib.rs                                |   9 +
src/recovery.rs                           | 598 ++++++++++++++++++++++++++++++
src/vault.rs                              |  68 ++++
5 files changed, 959 insertions(+), 7 deletions(-)
```
(`git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock`, `raw/04-diff-stat.txt`)

The task's own `Files/components` also names "CLI recover/verify interfaces" — not built in this unit; recorded as an explicit scope boundary below, consistent with `T01-03`'s own precedent of deferring CLI wiring to a store the CLI does not otherwise touch yet.

## A design correction made during this task (recorded honestly)

The first implementation attempt wired the new shared `AccessGuard` into **both** `crate::vault::Vault::open_read`/`open_write` (format-1) and `crate::canonical::CanonicalStore::open` (format-2), reasoning that plan §14's two-lock model should close `T01-01`'s named residual ("recovery cannot race a reader") for both formats uniformly. Running the full suite immediately caught a real regression: `vault::tests::open_read_never_creates_metadata_on_legacy_vault` failed, because creating `access.lock` on a legacy, metadata-less `.fehrest` directory violates `T01-01`'s own deliberately tested invariant that a readonly open leaves the control directory byte-for-byte unchanged.

Re-reading `T01-01`'s own evidence resolved the ambiguity rather than papering over it: it explicitly states the §14 access-lock model is described "in terms of the future SQLite canonical store, which `T01-02` creates, not the current file-based store." Format-1's `Vault` was reverted to its exact pre-existing behavior (no `AccessGuard`, no new field, no test changes there); `AccessGuard` is used only by `crate::canonical::CanonicalStore` and `crate::recovery`. This is recorded here as a failed attempt genuinely caught by the test suite and corrected via evidence, not silently dropped or worked around by weakening the pre-existing test.

## What was built, and why — grounded in the task contract

1. **`AccessGuard`** (`src/vault.rs`): the "access" half of §14's two-lock model, using `std::fs::File`'s stable shared/exclusive advisory-lock API. `access.lock` is created empty and unlocked as part of `CanonicalStore::create`'s staged publication (published atomically alongside the guard and database via the same directory rename `T01-02` already uses) — never lazily by a read, mirroring the existing `vault.json`/`MissingMetadata` philosophy exactly. `CanonicalStore::open` now requires `access.lock` to already exist and holds it shared for the connection's entire lifetime.
2. **`crate::recovery::recover_to_new_root`**: acquires recovery ownership (writer lease, then exclusive access — the fixed order §14 mandates), refuses immediately (`Busy`-shaped) if any normal connection currently holds shared access, preserves the exact guard/database/(-journal, if present) bytes to a forensic directory before touching anything else, builds a disposable working copy from *those preserved bytes* (never the live original), and only publishes a new root if that working copy passes independent verification.
3. **`canonical::verify_recovery_candidate`**: deliberately a *stronger* check than `open`'s own trust level — `PRAGMA integrity_check`, the same exact-schema recognition `open` uses, then a from-scratch recomputation of the entire `command` chain (each row's `previous_head_seq`/`previous_head_hash` checked against the prior row's actual result, each `resulting_head_hash` independently recomputed and compared, not merely re-read), a referential-integrity sweep (`current_object` pointers and `revision` parent links must not dangle), and a final check that `canonical_vault`'s stored head matches the reconstructed chain's end exactly.
4. **Incident manifest**: every recovery attempt, successful or refused, writes `incident-manifest.json` into its preservation directory (schema, timestamps, outcome, and either the verified head/object-count or the exact refusal reason) — the task's own named "incident manifest" component, giving a recovery attempt a durable record independent of whatever the calling process does with the returned `Result`.
5. **Format documentation**: `docs/formats/format-2-canonical-sqlite.md` extended with the two-lock table, the exact lock-acquisition ordering, the recovery protocol, and the incident-manifest JSON shape.

## Explicit scope boundaries (recorded, not silently dropped)

- **No CLI wiring.** No `recover`/`verify` command exists. Same reasoning as `T01-03`: this store has no CLI surface yet for anything.
- **No backup-artifact restore.** This module only ever reads from the live root's own current bytes; restoring from a separately exported backup is `T01-05`'s objective.
- **All-or-nothing verification, no labeled partial salvage.** A single chain break, dangling reference, or head mismatch refuses complete publication entirely. A richer, owner-facing "here is the valid prefix, here is what was lost" UX is not built — the preserved bytes and incident manifest are the evidence trail this task provides toward that, not the UX itself.
- **`Busy` is immediate, not a bounded wait.** If a normal connection holds shared access, recovery refuses immediately rather than waiting a bounded interval and retrying. §14 permits either half of "waits... or returns Busy"; this task took the simpler, immediately-refusing half.
- **Format-1 untouched.** See "A design correction" above.
- **No genuine multi-process concurrency test** for the access lock — only in-process contention via two independently opened `fs::File` handles on the same lock file within one test process (a real, valid proof of the OS-level advisory-lock semantics, but not a second live OS process). Consistent with every prior `flake-v1` task's identical limitation for `WriteLock`.

## Tests added (8 new; all pass; full raw output `raw/01-full-test-run.txt`, isolated runs `raw/07`/`raw/08`)

| Test | What it proves | Verification hierarchy tier(s) |
|---|---|---|
| `clean_store_recovers_to_new_root_with_matching_state` | A healthy store recovers to a new root with byte-identical current state, and the original remains independently openable throughout | V03 |
| `tampered_resulting_head_hash_is_irrecoverable_and_original_untouched` | A hand-tampered stored hash is caught by the from-scratch recomputation (not the stored-value re-read an ordinary `open` would trust), publication is refused, and the live original file is provably byte-identical before and after the attempt | V05/V09 |
| `missing_database_with_guard_present_is_refused_before_any_preservation` | A guard-without-database root is refused with a specific message, and nothing is published | V02 |
| `destination_already_published_is_refused_without_touching_the_original` | An already-published destination refuses recovery without touching the original | V02/V09 |
| `recovery_is_refused_while_a_normal_connection_holds_shared_access` | A live reader/writer handle makes recovery's exclusive-access attempt fail with an explicit `Busy`-shaped message | V03/V09 |
| `a_new_normal_open_is_blocked_while_recovery_holds_exclusive_access` | The inverse contention direction: a held exclusive lock blocks a new shared attempt, which succeeds immediately once released — the OS-level mechanism a real `CanonicalStore::open` would block on | V03 |
| `successful_recovery_writes_a_verified_incident_manifest` | The incident manifest for a successful recovery contains the exact verified head/object-count and a null refusal reason | V02 |
| `refused_recovery_writes_a_refusal_incident_manifest_and_preserves_evidence` | The incident manifest for a refused recovery is written into the (still-present) preservation directory with the exact refusal reason | V02/V09 |

No pre-existing test was modified in this task's final state (the one regression the design correction caught was reverted to its original form, not weakened).

## Commands executed (all re-run on this branch against `origin/main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **161/161 pass**, 0 failed (128 lib [120 pre-existing + 8 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib recovery::` | `raw/07-recovery-module-tests-isolated.txt` | 8/8 pass |
| `cargo test --locked --lib canonical::` | `raw/08-canonical-module-tests-isolated.txt` | 23/23 pass (unchanged from `T01-03`; confirms the `access.lock` integration did not regress the canonical store's own suite) |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings (one `clippy::type_complexity` finding on an intermediate draft, fixed by introducing a named `CommandChainRow` struct instead of a 7-tuple) |
| `git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `cargo run --release --example recovery_timing` (throwaway harness, deleted before this commit) | `raw/05-recovery-timing.txt` | p50=24.1ms/p95=38.8ms/max=38.8ms (n=15, 5-revision stores) |
| Environment capture | `raw/06-environment.txt` | Windows 11 Home 10.0.26200, 12 logical CPUs, 16,106 MB RAM, rustc/cargo 1.97.1, vendored SQLite 3.50.2 (unchanged) |

## Failed attempts / exclusions

- **The access-lock/format-1 regression**, described in full under "A design correction" above: caught by `vault::tests::open_read_never_creates_metadata_on_legacy_vault` failing on the first full-suite run after the initial (overly broad) implementation. Root cause diagnosed against `T01-01`'s own evidence rather than assumed; fixed by scoping `AccessGuard` to format-2 only, not by weakening the failing test.
- One `clippy::type_complexity` finding (a 7-element tuple return type) on an intermediate draft of `verify_recovery_candidate`'s command-chain scan; fixed by introducing a local named `CommandChainRow` struct.
- Two test-authoring bugs caught before this final run: (a) an initial version of `tampered_resulting_head_hash_is_irrecoverable_and_original_untouched` captured its "before" byte-snapshot *before* the test's own tampering edit, so the final "original untouched" comparison would have failed for the wrong reason (fixed to snapshot after the test's own tamper, so the comparison isolates whether `recover_to_new_root` itself caused any further change); (b) an initial version of the exclusive-vs-shared contention test called the blocking `acquire_shared` API on the same thread that already held the exclusive lock, which would have deadlocked that test — rewritten to use only non-blocking `try_lock`/`try_lock_shared` calls on independently opened file handles.
- No test was skipped, deleted, or weakened to reach green in the final implementation.

## Performance gate

Closest applicable §27 analog, measured at S-scale (one store, 5 committed revisions, `raw/05-recovery-timing.txt`):

| Operation | Closest §27 analog | Target / Maximum | Observed (S-scale) |
|---|---|---|---|
| `recover_to_new_root` (ownership + preserve + verify + publish) | "Full verify / recovery working copy M": target 60s / max 180s | p50 24.1ms, p95 38.8ms, max 38.8ms — well inside |

No dataset-M/L measurement: those datasets need typed records that don't exist before `P02`, consistent with this task's own cross-platform gate note.

## Durability gate

D2/D4/D5 "interrupted recovery, zero overwrite of original evidence": `tampered_resulting_head_hash_is_irrecoverable_and_original_untouched` and `missing_database_with_guard_present_is_refused_before_any_preservation` both prove the live original is never mutated by a refused recovery attempt (the former proves it byte-for-byte, not just "still opens"). Forensic preservation happens strictly before verification touches anything derived from the original, and is never removed on a failure path — `refused_recovery_writes_a_refusal_incident_manifest_and_preserves_evidence` confirms the preservation directory and its manifest both survive a refusal.

## Cross-platform gate

Native development profile only (Windows 11, MSYS/MinGW toolchain, NTFS), matching this task's own "First native development report; profile-specific recovery results mandatory at T05-02" clause and the identical limitation recorded at every prior `flake-v1` task.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| All corrupt/unclean fixtures retain original digests | Satisfied — `tampered_resulting_head_hash_is_irrecoverable_and_original_untouched` (byte-for-byte comparison) |
| Recoverable fixtures restore exact committed history | Satisfied — `clean_store_recovers_to_new_root_with_matching_state` |
| Irrecoverable fixtures refuse complete publication | Satisfied — `tampered_resulting_head_hash_is_irrecoverable_and_original_untouched`, `missing_database_with_guard_present_is_refused_before_any_preservation` |

## Completion condition

Every acceptance clause above is satisfied or explicitly, honestly scoped with recorded justification (CLI wiring, backup-artifact restore, labeled partial salvage, bounded-wait Busy semantics, and multi-process lock testing all deferred, matching this task's own forbidden-scope boundary against successor work). A genuine design mistake (retrofitting the access lock onto format-1) was caught by the existing test suite and corrected against `T01-01`'s own evidence rather than papered over. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (161/161). No sealed evidence altered; no force-push; no historical evidence file touched. `T01-04` is complete.

## Next frontier

`T01-05` — Create and restore consistent verified backups. Depends on `T01-04` (this task). Uses the admitted SQLite snapshot/backup API to produce an owner-controlled, independently verified backup and restore path, distinct from this task's own live-root-only recovery.
