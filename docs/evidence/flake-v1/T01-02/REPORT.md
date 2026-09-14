# T01-02 evidence report — Create an independently specified format-2 transaction store

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §13/§15/§16, task `T01-02`
- **Baseline / tested source commit:** forked from `origin/main` `0ca5408a324da075f3a868928296bca910644cb5` (PR #67, `T01-01` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5 / Claude Fable 5.1 session)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer and no GitHub-enforced CI runs `cargo test`/`fmt`/`clippy` locally in this repository today (per `docs/evidence/flake-v1/T00-01/REPORT.md` and `T01-01/REPORT.md` limitations, still true at this baseline). All checks below were executed locally on this development host; the repository's GitHub Actions checks (`manifest-check`, `canonical-equality`, `validate`, `test-review-binding`, `test-statistical-design`, `test-validator`, `verify-artifacts`, `test-scorer`) cover the R1/Spec-002 corpus, not this Rust crate, and are unaffected by this change (no files under their paths are touched).

## Live-truth reverification performed before this task started

- `git fetch origin --prune`; `origin/main` and local `main` both at `0ca5408a324da075f3a868928296bca910644cb5`.
- `gh pr list --repo TheHalfMoon/Flake --state all`: `#67`, `#66`, `#65` confirmed merged, matching the handoff.
- `gh api repos/TheHalfMoon/Flake/commits/0ca5408.../check-runs`: all 8 checks (`manifest-check`, `canonical-equality`, `validate`, `test-review-binding`, `test-statistical-design`, `test-validator`, `verify-artifacts`, `test-scorer`) report `success` on the exact head commit.
- `specs/CURRENT.md` on `main` at this baseline: `T00-01_STATUS=COMPLETE`, `T00-02_STATUS=COMPLETE`, `T01-01_STATUS=COMPLETE`, `ACTIVE_IMPLEMENTATION_UNIT=T01-02`, `T01-02_STATUS=READY_NOT_STARTED`. Confirmed consistent with the handoff and with the canonical plan's dependency chain `T01-01 -> T01-02 -> T01-03`.
- Read `AGENTS.md`, `docs/canonical/FLAKE_MUSE_EXECUTION_HANDOFF.md`'s pointer, and the full `T01-02` task-contract row in `FLAKE_CANONICAL_BUILD_PLAN.md` §"P01" before writing any code, per §"Canonical reading order".

## Scope actually touched

`src/canonical.rs` (new module), `src/lib.rs` (module registration + one new `Error::Canonical` variant), `docs/formats/format-2-canonical-sqlite.md` (new), `tests/fixtures/canonical/malformed_not_sqlite.bin` (new fixture). No `Cargo.toml`/`Cargo.lock` change: the task's own `dependency-admission.md` (written at `T00-02`) already establishes `rusqlite`/`libsqlite3-sys` as admitted for exactly this purpose, and this task adds no new dependency. No file outside the task's named `Files/components` (`src storage/vault modules, Cargo.toml/Cargo.lock only admitted changes, docs/formats/ and new format fixtures`) was changed.

```text
docs/formats/format-2-canonical-sqlite.md         | 193 ++++
src/canonical.rs                                   | 851 +++++++++++++++++
src/lib.rs                                         |   7 +
tests/fixtures/canonical/malformed_not_sqlite.bin  | Bin 0 -> 96 bytes
4 files changed, 1051 insertions(+)
```
(`git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock`, `raw/04-diff-stat.txt`)

## What was built, and why — grounded in the task contract

The task's objective is "Initialize a new isolated canonical store with the selected engine and published logical format," not the transaction/command save path (`T01-03`). `src/canonical.rs` implements exactly that:

1. **A new `CanonicalStore` type**, isolated from the format-1 `crate::vault::Vault` (different module, different guard `format_version`, different database). Format-1's own code is completely unchanged by this task — `git diff --stat` above touches no line of `src/vault.rs`.
2. **Guard reuse, not guard duplication.** The format-2 guard is the exact same `.fehrest/vault.json` file and `VaultMeta` JSON shape format-1 already parses, with `format_version = 2`. This is deliberate: format-1's pre-existing `read_vault_meta` already refuses any `format_version` greater than its own `SUPPORTED_FORMAT_VERSION` (`1`) — see `vault.rs::unsupported_newer_format_fails_visibly`, already passing on `main` before this task. Publishing a real format-2 guard is therefore what makes an old binary refuse it, with zero code change to the old binary. `canonical::tests::legacy_format1_reader_refuses_format2_store` proves this against a *genuinely published* format-2 store (not only the pre-existing synthetic fixture that test used before).
3. **Staged creation, independently verified before publication** (§16): build the guard + `canonical.sqlite` inside a uniquely named `.fehrest.staging-<uuid7>` sibling directory; reopen the staged database with a **fresh, independent connection** and run the full schema-recognition check against it; only then `fs::rename` the staging directory onto `.fehrest`. Every failure path removes the staging directory and never touches `.fehrest`. Correctness of "never overwrite an existing path" rests on `fs::rename`'s own refusal to replace an existing directory (both POSIX `rename(2)` and Win32 `MoveFileExW`), documented explicitly in the module so a future editor does not mistake the upfront `exists()` checks for the actual enforcement mechanism.
4. **Required engine configuration, set and immediately read back** on every connection: `page_size=4096`, `journal_mode=DELETE`, `synchronous=EXTRA`, `foreign_keys=ON`, `trusted_schema=OFF`, `cache_size=-8192` (8 MiB). §13's own tie-break ("4096 and 8 MiB") is used directly; no bake-off was run because the plan already resolved that tie. Every pragma set is followed by a query-back assertion — a pragma that silently failed to apply is caught as an `Error::Canonical`, not trusted.
5. **A minimal `canonical_vault` identity/head-summary table** (one singleton row: `vault_id`, `schema_version`, `min_reader_capability`, `created_by_version`, `created_at`, `transaction_head_seq`, `transaction_head_hash`), matching plan §15's `Vault` entity ("current schema/min-reader capabilities and transaction head") without building the `Project`/`Note`/`Action`/`Decision`/`Transaction`/... tables §15 also describes — those belong to `T01-03` onward, once there is an actual writer/command API to populate them; building them now with no writer would be exactly the "successor work" this task's own forbidden scope excludes.
6. **Exact schema recognition, applied on every open** (both the internal pre-publish verification and every later `CanonicalStore::open`): table list must be exactly `["canonical_vault"]`; its columns must exactly match the eight `(name, type)` pairs the module expects; the identity row's `vault_id` must equal the guard's; the identity row's `min_reader_capability` must not exceed what this build implements; `page_size` must read back `4096`. Any deviation is refused with a specific message, not silently admitted.
7. **`docs/formats/format-2-canonical-sqlite.md`** publishes the exact guard shape, the exact `canonical_vault` DDL and column semantics, the exact required pragma values, the exact schema-recognition rule, a generic (non-Flake) `sqlite3` CLI reader example, and the publication protocol — satisfying "Publish exact format tables/payload encoding and generic-reader examples" and I10 portability directly, not by cross-reference to code comments alone.

## Explicit scope boundaries (recorded, not silently dropped)

- **No writer-lease integration.** `CanonicalStore::create`/`open` do not take `crate::vault::WriteLock`. The task's own security row (§22, S06) cites `T01-01..03` collectively for "OS-held one-writer lease and Rust admission," and T01-03's own contract clause is "Bind private mutators to owning vault writer" — that binding, not this task's bare create/open, is where the lease is wired to this store. Recorded explicitly, matching the precedent set by `T01-01`'s own "recovery cannot race a reader" scope note.
- **No genuine concurrent-multi-process creation race test.** Only a single interrupted creator (four fault points, each independently exercised) and a second *sequential* creation attempt against an already-published root are tested. The task's acceptance clause is "interrupted creation leaves old paths intact," which this satisfies; a `T01-01`-style multi-thread contention stress test is not this task's clause and will make sense once `T01-03` wires the writer lease into this store.
- **No dataset-M/L performance measurement.** Plan §27's M/L datasets require typed record shapes (`Project`/`Note`/`Action`/...) that do not exist before `P02`. This task records a bounded, S-scale (one empty store) creation/reopen timing only (below), consistent with the task's own "Native development profile mandatory now; all remaining profiles retained for `T05-02`" cross-platform gate note.
- **Timestamp placeholder carried forward.** `created_at` in both the guard and the `canonical_vault` row uses the same non-calendar-correct RFC3339-shaped placeholder already present and already documented as a limitation in `vault.rs::chrono_like_now_iso8601` (no `chrono`/`time` dependency is admitted for this minimal task). Named in `docs/formats/format-2-canonical-sqlite.md` "Known limitations," not hidden.

## Tests added (12 new; all pass; full raw output `raw/01-full-test-run.txt`, isolated module run `raw/07-canonical-module-tests-isolated.txt`)

| Test | What it proves | Verification hierarchy tier(s) |
|---|---|---|
| `fresh_creation_publishes_guard_and_database_with_matching_identity` | A fresh `create` publishes both files, a valid UUIDv7 `vault_id`, `schema_version=1`, transaction head `(0, None)` | V02/V03 |
| `reopen_after_create_is_stable_and_pragmas_hold` | Reopening reasserts every required pragma; each reads back the exact required value on a connection that never set it in this process run | V02/V03 |
| `generic_sqlite_connection_can_inspect_published_schema` | A bare `rusqlite::Connection::open` outside this module's API sees the same table/row a generic SQLite tool would (I10) | V15 |
| `create_refuses_to_clobber_existing_control_dir` | A pre-existing `.fehrest` (simulated with a sentinel file) is completely untouched; creation refuses before ever staging | V02/V09 |
| `second_creation_on_a_published_root_is_refused_and_first_is_untouched` | A second `create` on an already-published root is refused; the original store's identity is unchanged afterward | V02/V09 |
| `legacy_format1_reader_refuses_format2_store` | A **real published** format-2 guard is refused by both `Vault::open_write` and `Vault::open_read`, with the exact pre-existing "unsupported vault format_version 2" message (S03/S09, F05, V10) | V03/V10 |
| `guard_database_identity_mismatch_is_refused` | A hand-corrupted guard `vault_id` (divergent from the database) is refused with an explicit identity-mismatch message | V09 |
| `malformed_database_file_is_refused_not_panicking` | A non-SQLite byte fixture in place of `canonical.sqlite` produces an `Error::Canonical`, not a panic | V09 |
| `unrecognized_extra_table_in_schema_refuses_open` | An extra table added by a raw connection after publication is refused on the next `open` (S04 "incompatible or unexpected schema refuses writes") | V09 |
| `min_reader_capability_higher_than_supported_is_refused` | A hand-raised `min_reader_capability` value is refused rather than silently opened | V09/V10 |
| `interrupted_creation_at_each_fault_point_leaves_root_without_published_control_dir` | All four `CreateFaultPoint` stages independently leave `.fehrest` unpublished and no orphaned staging directory behind | V06 |
| `repeated_creations_have_unique_vault_ids_and_all_reopen_correctly` | 20 independent creations never collide on `vault_id`, and every one reopens to the same identity it was created with (bounded-generation invariant) | V05 |

No pre-existing test was modified. `crate::vault`'s own `unsupported_newer_format_fails_visibly` (fixture-based) and every other pre-existing test are unaffected and still pass — see the full suite count below.

## Commands executed (all re-run on this branch against `origin/main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (not separately archived; folded into the test run) | Clean build, no warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **142/142 pass**, 0 failed (109 lib [97 pre-existing + 12 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib canonical::` | `raw/07-canonical-module-tests-isolated.txt` | 12/12 pass, isolated confirmation |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff (one intermediate non-conforming draft was caught by this exact check and corrected with `cargo fmt --all` before this final run — recorded under "Failed attempts" below) |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat origin/main -- src/ tests/ docs/formats/ Cargo.toml Cargo.lock` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `cargo run --release --example canonical_timing` (throwaway harness, deleted before this commit) | `raw/05-timing-and-sqlite-version.txt` | create p50=23.1ms/p95=38.3ms/max=40.5ms (n=30, cold); open p50=0.7ms/p95=1.1ms/max=2.6ms (n=90, warm) |
| `cargo run --release --example zz_sqlite_version` (throwaway harness, deleted before this commit) | `raw/05-timing-and-sqlite-version.txt` | Vendored SQLite `3.50.2` |
| Environment capture (`rustc --version`, `cargo --version`, `systeminfo`, `nproc`) | `raw/06-environment.txt` | Windows 11 Home 10.0.26200, 12 logical CPUs, 16,106 MB RAM, rustc/cargo 1.97.1 |

## Failed attempts / exclusions

- The first draft of `src/canonical.rs` did not derive `Debug` on `CanonicalStore`, which every `.unwrap_err()` call in the test module requires (`Result::unwrap_err`'s trait bound). `cargo test --locked --lib canonical::` failed to compile with seven `E0277` errors naming the missing `Debug` bound. Fixed by adding `#[derive(Debug)]` (rusqlite's `Connection` already implements it, so no manual impl was needed); recompiled clean.
- The first `cargo fmt --all -- --check` run on the new module reported a nonzero exit with a real diff (line-wrapping only, no semantic change) — recorded honestly rather than treated as already-passing; `cargo fmt --all` was then run and the check re-verified clean (`raw/02-fmt-check.txt` reflects the final, passing run).
- No other failed attempt occurred on the final implementation; no test was skipped, deleted, or weakened to reach green.

## Performance gate

Plan §27's dataset-M/L gate does not apply yet (see "Explicit scope boundaries" above — typed records do not exist before `P02`). The closest applicable analogs, measured at S-scale (one empty store, `raw/05-timing-and-sqlite-version.txt`):

| Operation | Closest §27 analog | Target / Maximum | Observed (S-scale) |
|---|---|---|---|
| Cold `create` (stage + verify + publish) | "Save/write acknowledgement ≤64 KiB": p95 150ms / max 750ms | p95 38.3ms, max 40.5ms — well inside |
| Warm `open` (reassert pragmas + recognize schema) | "Clean startup integrity quick checks M": target 250ms / max 1s | p95 1.1ms, max 2.6ms — well inside |

Page size 4096 / cache 8 MiB were used directly per §13's own tie-break, not re-derived by a fresh bake-off; no measured PASS beyond the S-scale figures above is claimed. Full native-profile and dataset-M/L qualification remains `T05-02`, per this task's own cross-platform gate note.

## Durability gate

D2/D5 "initialization publication and persisted guard/database agreement": proven by `fresh_creation_publishes_guard_and_database_with_matching_identity`, `reopen_after_create_is_stable_and_pragmas_hold`, and `guard_database_identity_mismatch_is_refused` (the negative case: divergence is caught, not silently trusted). `journal_mode=DELETE` with `synchronous=EXTRA` is asserted on every open via `apply_and_assert_runtime_pragmas`, matching §13's "rollback journal with EXTRA synchronization" — no cross-process crash-durability proof (`kill`-style test) is claimed here; that pattern belongs to the actual write path `T01-03` introduces, since `T01-02` writes exactly once, at creation, before publication.

## Cross-platform gate

Native development profile only (Windows 11, MSYS/MinGW toolchain, NTFS), matching the task's own "Native development profile mandatory now; all remaining profiles retained for `T05-02`" clause. No Linux/ext4 or macOS/APFS evidence exists for this task, consistent with `T01-01`'s identical, already-recorded limitation.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| New vault reopens with generic SQLite inspection | Satisfied — `generic_sqlite_connection_can_inspect_published_schema` |
| Required settings are asserted | Satisfied — `apply_and_assert_runtime_pragmas`/`assert_page_size`, exercised by `reopen_after_create_is_stable_and_pragmas_hold` |
| Incompatible or unexpected schema refuses writes | Satisfied — `unrecognized_extra_table_in_schema_refuses_open`, `min_reader_capability_higher_than_supported_is_refused`, `malformed_database_file_is_refused_not_panicking` |
| Interrupted creation leaves old paths intact | Satisfied — `interrupted_creation_at_each_fault_point_leaves_root_without_published_control_dir`, `create_refuses_to_clobber_existing_control_dir` |
| Guard/DB identity agrees (I03/I04/I10) | Satisfied — `guard_database_identity_mismatch_is_refused` |
| Old binary refuses format 2 (I03/I04/I10) | Satisfied — `legacy_format1_reader_refuses_format2_store` (real published store, not only the pre-existing synthetic fixture) |
| No existing directory is overwritten (I03/I04/I10) | Satisfied — `create_refuses_to_clobber_existing_control_dir`, `second_creation_on_a_published_root_is_refused_and_first_is_untouched`; enforcement mechanism documented as `fs::rename`'s own refusal, not the upfront check |

## Completion condition

Every acceptance clause above is satisfied or explicitly, honestly scoped with recorded justification (writer-lease integration and multi-process creation races deferred to `T01-03`; dataset-M/L performance and non-Windows platforms deferred to later tasks per the plan's own gates). Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (142/142). No sealed evidence altered; no force-push; no historical evidence file touched. `T01-02` is complete.

## Next frontier

`T01-03` — Commit save, full history and command result atomically. Depends on `T01-02` (this task). Binds private mutators to the owning vault writer, introduces the actual transaction/command admission API and the `Note`/`Action`/`Decision`/history tables against the `canonical_vault` store this task publishes.
