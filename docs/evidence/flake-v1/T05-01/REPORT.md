# T05-01 evidence report — Freeze compatibility and qualify migration tooling

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §21/§27/§28, task `T05-01`
- **Baseline / tested source commit:** forked from `origin/main` `6538dd974575a0fe0e8613b6fda86220479c2be6` (PR #96, `T04-06` merge — post-merge `main` reverified green on all 8 required checks before this task started)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed locally on Windows; native Windows/macOS/Linux qualification and M/L-scale timing run on GitHub-hosted Actions runners (`.github/workflows/t05-01-migration-qualification.yml`), matching the precedent already established at `T04-06`. No second human reviewer; no GitHub-enforced branch-protection check runs `cargo test`/`fmt`/`clippy` directly (unchanged limitation recorded at every prior `flake-v1` task) — the new workflow above runs as a normal (non-required) PR check, same as `t04-06-cross-platform.yml`.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`; confirmed `origin/main` at `6538dd9...` (PR #96 merged) with all 8 required GitHub Actions checks (`manifest-check`, `canonical-equality`, `validate`, `test-review-binding`, `test-statistical-design`, `test-validator`, `test-scorer`, `verify-artifacts`) reporting `success` on that exact head commit, checked via `gh api repos/TheHalfMoon/Flake/commits/6538dd9/check-runs`.
- `specs/CURRENT.md` on `main` at this baseline: `T04-06_STATUS=COMPLETE`, `P04_STATUS=CLOSED`, `ACTIVE_IMPLEMENTATION_UNIT=T05-01`, `NEXT_DEPENDENCY_READY_UNIT=T05-01`. This task also corrects `T04-06_MERGE_COMMIT` from `PENDING_PR_MERGE` (left in that state when the prior conversation was interrupted before observing the final CI result) to the actual merge SHA above.
- Re-read the full `T05-01` task-contract row (`docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` line 1307) before writing any code.

## What T05-01 actually needed, and what was already built

Before implementing, this task's own contract was checked against what earlier tasks (`T01-02`, `T01-03`, `T01-06`, `T02-01`, `T02-02`) had already built and tested, to avoid rebuilding already-shipped mechanisms under a new name (Ponytail: no unnecessary new abstraction). Found already implemented and tested, each cited below rather than reimplemented:

| §21 requirement | Already implemented as | Already tested by |
|---|---|---|
| Whole-database capability refusal ("unknown required capability... refuses") | `CANONICAL_MIN_READER_CAPABILITY` guard, `src/canonical.rs` | `min_reader_capability_higher_than_supported_is_refused` |
| Per-record-kind capability refusal | `RECORD_PAYLOAD_SCHEMA_VERSION` guard, `src/project.rs` | `a_payload_schema_version_newer_than_supported_is_refused` |
| Unknown optional bytes preserved | `#[serde(flatten)] unknown` on every typed record struct | `unknown_fields_survive_a_read_then_write_round_trip` |
| Never-lossy downgrade / copy-to-new-root migration | `migration::import_to_new_root` always creates a fresh root, never mutates source | `new_root_no_clobber`, module docs |
| D5 full migration failure schedule | `migration::import_to_new_root_with_fault` + `ImportFaultPoint::AfterNthRecord` | `d5_migration_interruption_fault_schedule_matrix` — **already a full 1..=100 schedule**, not a single fault point |

What was genuinely missing, and is what this task actually built:

1. A **standalone offline migration tool** — no binary existed that a format-1 owner could run without the full `fehrest`/desktop app installed.
2. A single **frozen, cross-referenced compatibility policy document** — the rules above existed but only in five separate module docs, with no one place stating the whole promise plainly.
3. **Golden fixtures** for the refusal/preservation rules specifically (the legacy format-1→2 golden fixtures already existed from `T01-06`; the future-unsupported/unknown-optional-preservation fixtures did not).
4. **M/L-scale migration performance timing** — `T01-06`'s own evidence explicitly recorded only S-scale (20 records) timing as "not a plan §27 M/L release measurement."
5. **Independent-reader verification of migration output specifically** — `tools/independent-verify/sqlite_reader.py` existed but assumed every payload was typed-record JSON, which a migrated legacy payload (raw bytes, `origin: migration`) is not; it would raise `SqliteFormatError` on any migrated vault.

## Scope actually touched

```text
.github/workflows/t05-01-migration-qualification.yml | new
docs/formats/format-compatibility-policy.md           | new
docs/evidence/flake-v1/T05-01/...                      | new (this report + raw + checks + results)
src/bin/flake-migrate.rs                               | new
src/migration.rs                                       | memory fix (see "A real defect..." below); no behavior/contract change
src/project.rs                                         | +2 golden-fixture tests
tests/flake_migrate_binary.rs                          | new
tests/fixtures/format-compat/...                       | new (2 fixtures + README)
tools/independent-verify/sqlite_reader.py              | extended: origin="migration" payload shape
specs/CURRENT.md                                       | frontier update + T04-06 merge-commit correction
```

No change to `src/canonical.rs` or any typed-record struct — those mechanisms are cited, not modified. `src/migration.rs`'s only change is the memory-footprint fix below — no change to its admission rules, byte-exactness guarantee, completeness semantics, or manifest shape. No `Cargo.toml`/`Cargo.lock` dependency change: `src/bin/flake-migrate.rs` links only the existing `fehrest`/`serde_json` dependencies already in the workspace.

## 1. Standalone offline migration tool (`src/bin/flake-migrate.rs`)

A thin CLI/JSON shell over `migration::preview_migration`/`migration::import_to_new_root` — see its own extensive module docs for why this is a separate binary (`cargo build --release --bin flake-migrate`) rather than a `fehrest` subcommand: a subcommand bundled into the main app would tie a format-1 owner's only import path to the main app's own release cadence, exactly the dependency §21's "old owned data must remain readable without keeping an obsolete main app" promise exists to avoid. It adds no new migration policy — every admission/omission/completeness rule it reports is `T01-06`'s own, unchanged.

Commands: `preview <source-root>`, `import <source-root> <new-root> [--select id1,id2,...]`, `--version`, `--help`. JSON on stdout, non-zero exit only on a real refusal (`Err` from the library) — an `Ok` report's own `complete` field communicates partial-vs-complete state, never conflated with process failure.

## 2. Format compatibility policy (`docs/formats/format-compatibility-policy.md`)

One document stating the epoch model (format 1 / format 2, `min_reader_capability` for in-epoch additions, `payload_schema_version` for per-record-kind capability) and cross-referencing every §21 clause to its exact already-shipped enforcement point and test. Explicitly records the one honest limitation: only one format-2 epoch has ever existed, so "current epoch reads its immediately previous epoch" is proven for format-1→2 specifically, not for a format-2→3 boundary that does not exist — this task does not invent one (§21: "no invented future production format").

## 3. Golden fixtures (`tests/fixtures/format-compat/`)

Two checked-in JSON payload fixtures (not generated inline), each loaded via `include_str!` by new tests in `src/project.rs`:

- `future-unsupported-note-payload.json` — `payload_schema_version: 999999` (deliberately far beyond any real version, so this fixture keeps proving the same refusal across future version bumps without regeneration). `golden_fixture_future_unsupported_payload_is_refused` proves `RecordPayload::from_json` refuses it with "newer than this build supports".
- `unknown-optional-fields-note-payload.json` — current `payload_schema_version` plus two fields no `Note` struct field names. `golden_fixture_unknown_optional_fields_survive_a_read_then_write_round_trip` proves both fields survive parse, and survive a further `to_json`→`from_json` round trip, byte-for-byte.

Chose human-readable JSON payload fixtures over a checked-in binary `canonical.sqlite`: both rules live at the typed-payload layer, independent of the surrounding SQL schema (which already has its own dedicated test), and a JSON fixture is directly inspectable by any reader with a text editor — a binary SQLite file would not be, and would require this exact `rusqlite` build to open at all, working against the fixture's own purpose (see `tests/fixtures/format-compat/README.md`).

## 4. Independent-reader migration support (`tools/independent-verify/sqlite_reader.py`)

`read_revision_envelopes` previously called `json.loads` unconditionally on every payload and required a `kind` field, raising `SqliteFormatError` on any `origin: migration` row (legacy raw bytes, per `docs/formats/legacy-migration.md`'s own published contract). Extended with one new branch: `origin == "migration"` rows skip the JSON/`kind` check (the `payload_sha256` re-hash immediately above already proves the bytes are exactly what is stored) and are still incorporated into the independent head-hash-chain proof exactly like every other revision. No change to behavior for any non-migration origin — verified by re-running every existing consumer of this module (`tools/independent-verify/crosscheck.py`, `run_all.py`, `export_reader.py`, `tools/interchange-clients/run_interchange.py`, `T04-02`/`T04-03`'s own evidence scripts) importing cleanly, and this task's own new usage exercising the changed branch directly.

## 5. Migration performance timing (`docs/evidence/flake-v1/T05-01/checks/m_scale_migration_timing.py`)

New harness: generates a synthetic format-1 legacy vault (`N` records, target average body size), times `flake-migrate preview`/`import` as real release-build subprocesses, independently verifies the result via the extended `sqlite_reader.py` above (no Flake Rust code executed by the verification step), records exact counts/timings/free-disk-space checkpoints as JSON, and **always deletes the generated dataset before exiting** (success or failure) — this development host's own local disk is severely constrained (`docs/evidence/flake-v1/T04-06/../../desktop/src-tauri/Cargo.toml`'s own prior comment on this same constraint; independently reconfirmed here: `df -h /c` showed 4.5 GiB free at the start of this task, out of a 200 GiB volume 98% full).

## 6. A real defect this task's own L-scale measurement found and fixed (`src/migration.rs`)

The first CI attempt at L-scale (100,000 records, ~10 GiB) was killed (`exit 143`) on a `ubuntu-latest` runner with 15 GiB RAM — roughly double plan §27's own documented reference-minimum (4 cores / 8 GiB RAM / local SSD). Root cause, confirmed by reading the code, not guessed at: `preview_migration` read every admitted record's full file content into a `raw_bytes: String` retained on every entry of its returned `Vec<LegacyRecord>` for the whole call — for 100,000 records averaging 100 KB, roughly 10 GiB of live String data held simultaneously. `import_to_new_root` made this worse by calling `preview_migration` internally and then `.clone()`-ing the entire admitted list into a second `to_import` vector before its commit loop — a second full in-memory copy. §27's own closing instruction is explicit: *"If the specified minimum hardware cannot meet a maximum after one measured bounded optimization pass, stop the affected unit for ADR/scope reconsideration."* This task's own named performance gate is exactly "M/L migration and memory/cancel ceilings," so fixing this is squarely in scope, not scope creep — and a genuinely bounded, one-pass fix was available, so no ADR/scope escalation was needed.

**The fix:** `LegacyRecord` no longer carries `raw_bytes` at all — `preview_migration` now reads each candidate file, computes its `content_sha256`, and lets the content drop at the end of that loop iteration (peak memory O(one record), not O(every admitted record)); `import_to_new_root`'s commit loop re-reads each admitted record's exact bytes fresh from disk immediately before committing it, then lets that copy drop too. The content itself is still read directly from disk and committed completely unmodified — never round-tripped through `identity::parse`/`serialize` — this only changes *when* each record's bytes are held in memory, not what bytes end up committed or how "exact-byte preservation" is proven (T01-06's own byte-identity tests, `complete_migration_of_a_clean_vault_preserves_exact_bytes_and_identity` and `gold_fixtures_with_crlf_unicode_and_unknown_fields_migrate_byte_identically`, both still pass unchanged in what they prove, adjusted only to check the preview-stage hash instead of a retained preview-stage copy of the bytes themselves — see their own updated inline comments).

All 328 lib tests (including the pre-existing D5 migration schedule) and all 4 standalone-binary tests pass unchanged after this fix. Re-run via a follow-up commit on this task's own PR; final CI results recorded below.

**S-scale (local, Windows, this development host):** 100 records, ~10 MiB total payload (plan §27's own S definition), release build. `preview` 1.62 s, `import` 0.47 s — both comfortably inside the closest §27 analog ("Full export/import M": target 60 s, maximum 180 s), independent verification (`head_hash_chain_verified: true`, exact count agreement across preview/import/independent-reader) passing. Raw: `results/s-scale-timing-windows-local.json`.

**M-scale (10,000 records, ~1 GiB payload) and L-scale (100,000 records, ~10 GiB payload, attempted disk permitting):** deliberately **not** run on this local host given the 4.5 GiB free-space constraint above — this repeats a real, previously-observed risk on this exact host (a prior task's evidence records free space dropping from ~1.5 GiB to ~22 MiB in about 15 minutes from unrelated background activity). Run instead on a `ubuntu-latest` GitHub Actions runner via the `m-scale-performance` job in `.github/workflows/t05-01-migration-qualification.yml`, which reports its own exact CPU/RAM/disk before running (plan §27's own "record actual CPU/RAM/SSD/OS/filesystem" instruction) and requires at least 24 GiB free before attempting L, else records why L was skipped rather than risking an uncontrolled runner failure mid-measurement.

**Actual CI run: [35081331572](https://github.com/TheHalfMoon/Flake/actions/runs/35081331572)**, `m-scale-performance` job, head SHA `673497a11b0bd6981aa6e2233b1b70bc8c241138` (this task's final commit, after the memory fix in section 6 above). Runner: `ubuntu-24.04` GitHub-hosted, 4 vCPU (AMD EPYC 9V74), 15 GiB RAM, `/dev/root` 145 GiB total / 86 GiB available at job start — well above the 24 GiB L-scale threshold, so L ran (not skipped).

| Scale | n | payload bytes | generate | preview | import | independent verify | gate (target/max) | result |
|---|---|---|---|---|---|---|---|---|
| M | 10,000 | 1,001,213,445 | 62.21 s | 2.43 s | 17.82 s | 4.41 s | 60 s / 180 s | within target |
| L | 100,000 | 10,012,272,794 | 614.02 s | 44.58 s | 217.54 s | 93.06 s | 600 s / 1800 s | within target |

Both runs: `counts_agree: true`, `import_complete: true`, `import_exit_code: 0`, `independent_head_hash_chain_verified: true`, `dataset_deleted: true` (disk reclaimed before the job's next step). Raw JSON (byte-identical to the job's own stdout, downloaded via `gh run download 35081331572`): `results/m-scale-timing-35081331572.json`, `results/l-scale-timing-35081331572.json` (manifest: `raw/00-manifest.txt`).

Import time is well inside both gates at both scales (17.82 s / 60 s target at M; 217.54 s / 600 s target at L), confirming the section-6 memory fix did not trade a memory ceiling for a time regression.

## 7. Standalone binary integration tests (`tests/flake_migrate_binary.rs`)

Proves the **compiled, separately-invoked** `flake-migrate` binary itself, as a real subprocess against the checked-in `T01-06` gold fixtures (`tests/fixtures/migration/`) — the one property no `src/migration.rs` unit test can prove, since those call the library functions directly, not the separately-built tool a format-1 owner without the main app would actually run:

- `standalone_binary_preview_reports_both_gold_fixtures_admitted`
- `standalone_binary_import_produces_a_real_openable_format_2_vault` — cross-checks the produced vault by opening it with a fresh `CanonicalStore::open` independent of the tool's own JSON report.
- `standalone_binary_import_refuses_to_clobber_an_existing_format_2_root`
- `standalone_binary_version_and_help_do_not_touch_any_filesystem_path`

## Gates run locally (Windows, this development host)

| Gate | Command | Result | Raw |
|---|---|---|---|
| Format | `cargo fmt --all -- --check` | pass | `raw/01-fmt-check-windows.txt` |
| Lint | `cargo clippy --all-targets --locked -- -D warnings` | pass, zero warnings | `raw/02-clippy-windows.txt` |
| Unit/property tests | `cargo test --locked --lib` | **328 passed**, 0 failed (includes the pre-existing `d5_migration_interruption_fault_schedule_matrix` and the 2 new golden-fixture tests) | `raw/03-lib-tests-windows.txt` |
| Standalone binary integration tests | `cargo test --locked --test flake_migrate_binary` | 4 passed | `raw/04-standalone-binary-tests-windows.txt` |
| Dependency advisories | `cargo audit` | 0 vulnerabilities, 53 crates scanned | `raw/05-cargo-audit-windows.txt` |
| Pre-existing regression suites | `cargo test --locked --test integration --test kill_tests` | 10 + 23 passed, unchanged | not separately captured (no change in this scope; ran to confirm no regression) |

Full raw-artifact manifest with SHA-256: `raw/00-manifest.txt`.

## Cross-platform and CI qualification

`.github/workflows/t05-01-migration-qualification.yml` (new, mirrors the `T04-06` precedent of using GitHub-hosted native runners rather than assuming this workstation's own OS is the only reachable platform):

- `qualify` (matrix: `windows-latest`, `macos-latest`, `ubuntu-latest`): full root Rust gates (fmt/clippy/`cargo test --locked --lib`, so the D5 migration schedule and both golden-fixture tests run natively on all three), `cargo audit`, the standalone-binary integration tests as real native subprocesses, and an independent-reader import smoke check. Satisfies this task's own cross-platform gate: "Migration and refusal behavior native on all profiles."
- `m-scale-performance` (`ubuntu-latest` only, `needs: qualify`): M-scale timing always; L-scale attempted only if the runner reports ≥24 GiB free, else explicitly recorded as skipped with the exact reason. Not repeated across all three OSes — a deliberate, disk-risk-driven scope decision recorded honestly here, not a silent gap (refusal/preservation/full-migration *correctness*, the part that can genuinely differ per OS, is still proven natively on all three by `qualify`).

**Final PR CI run, head `673497a`:** `qualify` passed on all three of `windows-latest`/`macos-latest`/`ubuntu-latest` (runs `35081331483`/`35081331572`), `m-scale-performance` passed (run `35081331572`, results above), `verify-artifacts` passed (run `35081331370`). `CodeRabbit`: skipped ("manual review required for this OSS repository" — the same pre-existing hosted-review limitation recorded at every prior `flake-v1` task). `cubic`: `NEUTRAL`/skipping. No GitHub-enforced required check runs `cargo test`/`fmt`/`clippy` directly on this repo (unchanged limitation, recorded again here) — this PR's own two new workflows are the actual gate, both green on the exact merged head.

## Explicit scope boundaries (recorded, not silently dropped)

- **No format-3.** This task freezes format 2 as the current epoch and documents the policy that would govern a future epoch; it does not invent one (§21: "no invented future production format" is a hard boundary this task does not work around).
- **No new CLI subcommand on the main `fehrest` binary.** The standalone tool is deliberately separate — see its own module docs.
- **M/L-scale timing measured on one native profile (`ubuntu-latest` CI), not all three**, for the disk-risk reasons stated above — an explicit, bounded limitation of this task's own execution environment, not of the product.
- **D1-D6 full native durability qualification is `T05-02`'s own task**, not repeated here beyond D5 for migration specifically (already fully covered by the pre-existing `d5_migration_interruption_fault_schedule_matrix`).

## Completion condition

Met: `qualify` green natively on all three platforms, `m-scale-performance` green with both M- and L-scale within their target gates, `verify-artifacts` green, all on this task's own final head `673497a`. `specs/CURRENT.md` marked `T05-01_STATUS=COMPLETE` in this same commit. Remaining steps (merge, post-merge main/CI reverification, frontier advance to `T05-02`) follow the identical pattern already established at every prior `flake-v1` task.
