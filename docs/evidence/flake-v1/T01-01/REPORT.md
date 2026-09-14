# T01-01 evidence report — Make legacy inspection nonmutating and establish ownership

## Identification

- **Task ID / phase / slice:** T01-01 / P01 / VS01
- **Spec Kit inputs:** `specs/002-post-r1-canonical-core-convergence/corrective-t01/{spec,clarify,plan,checklist,tasks,analyze,ponytail-gate,mutator-inventory}.md`
- **Plan contract:** canonical build plan §34, `T01-01`
- **Baseline / tested source commit:** forked from `origin/main` (`T00-02` merge, PR #66)
- **Executor:** Muse (Claude Code implementation agent)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer or external CI runs `cargo test`/`fmt`/`clippy` in this repository today (see `docs/evidence/flake-v1/T00-01/REPORT.md` limitations). All checks below were executed locally on this development host.

## Provenance note

The source change and its test suite were originally authored the same day on local branch `codex/flake-product-review` (commit `9b64d72`), against a source tree independently proven identical to current `origin/main` (`docs/evidence/flake-v1/T00-01/REPORT.md`: `git diff --stat` between the two baselines over `src/`, `Cargo.toml`, `Cargo.lock` is empty). The patch was applied cleanly to a fresh branch off `origin/main` with `git apply --check` before this report was written, and every command below — including the empirical pre-fix regression check — was **re-run independently on this branch**, not copied from the original run's logs.

## Scope actually touched

`src/vault.rs`, `src/events.rs`, `src/cli.rs`, `src/lib.rs` (new `Error::MissingMetadata` variant only), `tests/integration.rs`, `tests/kill_tests.rs` — every file named in the task's `Files/components` field, plus the two test files whose direct-`EventLog::append` call sites needed updating once that function's visibility changed. No `Cargo.toml`, `Cargo.lock`, CI workflow, or schema/migration file was touched, consistent with the task's forbidden scope.

```text
 src/cli.rs           |  80 ++++++++++++-
 src/events.rs        |  16 ++-
 src/lib.rs           |  11 ++
 src/vault.rs         | 324 +++++++++++++++++++++++++++++++++++++++++++++++++--
 tests/integration.rs |   4 +-
 tests/kill_tests.rs  |  35 ++++--
 6 files changed, 444 insertions(+), 26 deletions(-)
```
(re-run `git diff --stat origin/main`, `raw/04-diff-stat.txt` — matches the original same-day diff exactly)

## What was fixed, and why — grounded in `mutator-inventory.md`

`docs/evidence/flake-v1/T00-02/mutator-inventory.md` independently confirmed three concrete defects against actual source, matching the canonical plan's own stated T01-01 rationale. Each is closed here:

1. **`Vault::open_read` created metadata.** It called `ensure_vault_meta`, which durably writes a fresh `vault.json` when none exists — a "read" that mutates. Fixed: `open_read` now calls `read_vault_meta` directly and returns the new `Error::MissingMetadata` variant when no identity file exists, performing zero filesystem mutation. The sanctioned legacy upgrade path remains `open_write`, which performs the upcast under proven writer ownership.
2. **`Vault::open_write` mutated before acquiring ownership.** `ensure_vault_meta` and `startup_integrity_check` (which can quarantine/repair the event log) both ran before `WriteLock::acquire`. Fixed: `WriteLock::acquire` is now the first fallible step; the returned lock is dropped (releasing the lease) on any subsequent `?` early return, so a call that fails never leaves a stuck lock, and a call denied the lease never reaches the mutating steps at all. `Vault::create`'s redundant, equally-racy early `ensure_vault_meta` call was removed — it now relies entirely on the now-safe `open_write`.
3. **`EventLog::append` was an unbound public mutator.** Changed from `pub fn` to `pub(crate) fn`, with a doc comment naming `append_for_writer` as the sanctioned path and explicitly warning against widening it back. Two **real production bypasses** were found and fixed as part of this: `cli.rs`'s `init` and `compile` commands both called `log.append(...)` directly instead of through the already-held `VaultWriter`, despite the correctly-written `add` command demonstrating the sanctioned pattern existed. Both now call `writer.append_event(&log, ...)`.

A fourth item, not in the original three-clause rationale but discovered during this task and explicitly in scope (S03, "root/control-directory handle validation and native reparse behavior"): `Vault::require_vault` used `Path::is_dir()`, which follows symlinks — a `.fehrest` that was itself a symlink, Windows junction, or other reparse point would be silently followed rather than refused. Fixed with a new `is_reparse_point` helper using `symlink_metadata` plus, on Windows, the raw `FILE_ATTRIBUTE_REPARSE_POINT` bit via the safe `MetadataExt::file_attributes()` accessor (no `unsafe`, matching this crate's `forbid(unsafe_code)` lint) — this catches NTFS junctions/mount points that plain `is_symlink()` misses (they carry a different reparse tag).

## Explicit scope boundary: "recovery cannot race a reader"

T01-01's acceptance criteria names "recovery cannot race a reader" alongside "two writers cannot coexist." This task does **not** introduce the second shared/exclusive access-lock primitive canonical plan §14 describes (an access lock held shared by every normal open connection and exclusively by recovery/snapshot-preservation). Reasoning, recorded rather than silently skipped:

- That two-lock model is described in §14 in terms of the future SQLite canonical store, which `T01-02` creates, not the current file-based store.
- The current torn-tail repair mechanism (`EventLog::quarantine_and_repair_torn_tail`, in-place truncate-and-rewrite) is explicitly slated for wholesale replacement by `T01-04` ("Preserve forensic bytes and recover to a verified new root" — a working-copy-only model). Building a new access-lock now to coordinate a mechanism that T01-04 will replace entirely would be exactly the "successor work" T01-01's own forbidden scope excludes.
- What T01-01 *does* close is the narrower, concrete instance of this risk within its own scope: two writers (including a writer attempting the repair sequence) can no longer race each other, because the repair now only ever runs under a held `WriteLock`, and a losing writer performs zero mutation (see the empirical regression check below). A pure reader (`open_read`, which still correctly takes no lock, per architecture) racing with an in-progress repair remains a real, narrow, pre-existing window; it is not newly introduced or widened by this task, and closing it properly belongs to T01-04's access-lock design, not a throwaway mechanism here.

This is recorded as a named, deliberate scope boundary, not a silently dropped acceptance clause.

## Tests added (7 new; all pass; full raw output `raw/01-full-test-run.txt`)

| Test | File | What it proves |
|---|---|---|
| `open_read_never_creates_metadata_on_legacy_vault` | vault.rs | Readonly open of a metadata-less legacy vault returns `MissingMetadata`; before/after directory listing is byte-identical (empty both times) |
| `open_read_on_vault_with_existing_metadata_is_still_byte_identical` | vault.rs | Readonly open of an already-valid vault changes no file (name+length inventory identical before/after) |
| `open_write_acquires_lock_before_any_startup_mutation_no_split_brain_identity` | vault.rs | 8-thread concurrent-load stress test: no panic, no corruption, exactly one stable reopenable identity after contention settles |
| `losing_writer_performs_no_mutation_before_lock_denial` | vault.rs | **The deterministic regression proof.** A lock file is planted directly, then `open_write` is called once on a metadata-less root; it must fail with a locked-writer error and `vault.json` must never be created. Empirically re-verified to fail under the pre-fix ordering (below) |
| `control_dir_reparse_point_is_refused_not_followed` | vault.rs | `.fehrest` planted as a real Windows directory junction (via `mklink /J`, no admin rights) is refused by both `open_read` and `open_write` with `Error::Containment`, and nothing is written through the junction |
| `init_appends_vault_created_event_via_writer_and_chain_verifies` | cli.rs | End-to-end: `fehrest init` still creates a vault, records exactly one `VaultCreated` event via the writer path, and the chain verifies intact |
| `compile_appends_context_compiled_event_via_writer_and_chain_verifies` | cli.rs | End-to-end: `init` → `add` → `compile` records exactly 3 events (`VaultCreated`, `ObjectRegistered`, `ContextCompiled`) via the writer path, chain verifies intact |

Additionally, 4 pre-existing external integration/kill tests (`tests/integration.rs` AS-8; `tests/kill_tests.rs` k05, k18, k24b) called the now-`pub(crate)` `EventLog::append` directly and would no longer compile from outside the crate. Each was rewritten to construct a real `Vault`/`VaultWriter` and call `append_event`, which is a strictly more representative exercise of the real API, not a weakening.

### Empirical regression verification — independently re-run on this branch

The claim that `losing_writer_performs_no_mutation_before_lock_denial` would fail under the pre-fix ordering was **re-checked on this branch, not copied from the original run**: `Vault::open_write` in `src/vault.rs` was temporarily edited in place to restore the pre-fix mutate-then-lock order, `cargo test --locked --lib losing_writer_performs_no_mutation_before_lock_denial` was run and confirmed to fail (exit 101) with exactly the predicted panic message ("a writer denied the lease must perform zero mutation"), then the fix was restored from a pre-edit backup and confirmed byte-identical (`diff`, exit 0, no output) before the full suite was re-run green. Raw output of the failing run: `raw/05-empirical-prefix-ordering-fails.txt`; restore/final-green confirmation: `raw/06-restore-and-final-green-confirmation.txt`.

## Commands executed (all re-run on this branch against `origin/main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **130/130 pass**, 0 failed (97 lib + 10 integration + 23 kill_tests) |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` (empty = clean) | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat origin/main -- src/... tests/...` | `raw/04-diff-stat.txt` | Scope confirmed as listed above; matches original same-day diff exactly |
| Temporary revert of `open_write` ordering + single-test run, then byte-identical restore + full suite re-run | `raw/05-empirical-prefix-ordering-fails.txt`, `raw/06-restore-and-final-green-confirmation.txt` | Confirmed FAILED under old ordering (exit 101); confirmed restored file byte-identical to the fix; confirmed full suite green again afterward |

## Failed attempts / exclusions

None on the final implementation.

## Limitations

- No cross-platform (Linux/ext4, macOS/APFS) evidence — matches the task's own cross-platform gate ("Native development profile mandatory now; all remaining profiles retained for T05-02") and `reference-hardware.md`'s recorded unavailability of those hosts.
- `Vault::add_object` (the inherent, runtime-checked legacy path distinct from `VaultWriter::add_object`) was deliberately left unchanged — it already checks `has_write_lock()` at runtime (unlike `EventLog::append`, which had zero check before this task), so it is a lesser, already-mitigated case outside the three-clause rationale this task targets.
- The reader-vs-recovery race window described above remains open, by deliberate, recorded scope decision, pending T01-04.
- No CI in this repository runs `cargo test`/`clippy`/`fmt` today (`docs/evidence/flake-v1/T00-01/REPORT.md`); this evidence rests on locally-executed, independently-repeated runs, not a GitHub-enforced gate.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| All nominal readers preserve a before/after byte inventory | Satisfied — `open_read_never_creates_metadata_on_legacy_vault`, `open_read_on_vault_with_existing_metadata_is_still_byte_identical` |
| Missing guard is not created | Satisfied — `losing_writer_performs_no_mutation_before_lock_denial` (empirically re-verified against pre-fix code on this branch) |
| Two writers cannot coexist | Satisfied — pre-existing `second_writer_fails_visibly`/`second_writer_still_fails_visibly_and_no_auto_steal`, unaffected and still passing |
| Crash releases the OS lease | Satisfied — pre-existing `Drop` impl on `WriteLock`, unaffected; still exercised by `second_writer_fails_visibly` |
| Recovery cannot race a reader | **Partially satisfied, explicitly scoped** — see "Explicit scope boundary" above; writer-vs-writer race (including repair-capable writers) is closed; reader-vs-in-progress-repair is a named, deferred residual pending T01-04 |

## Completion condition

Every acceptance clause is satisfied or explicitly, honestly scoped with recorded justification. Shared contract (SC) gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green. No sealed evidence altered; no force-push; `4246f6d`/`852e44b` untouched (this task's branch is forked from `origin/main`, not from that local branch). `T01-01` is complete.

## Next frontier

`T01-02` — Create an independently specified format-2 transaction store. First task to introduce the new `canonical.sqlite` store, reusing the already-admitted `rusqlite`/`libsqlite3-sys` dependency per `dependency-admission.md`.
