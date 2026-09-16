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
src/project.rs                                         | +2 golden-fixture tests
tests/flake_migrate_binary.rs                          | new
tests/fixtures/format-compat/...                       | new (2 fixtures + README)
tools/independent-verify/sqlite_reader.py              | extended: origin="migration" payload shape
specs/CURRENT.md                                       | frontier update + T04-06 merge-commit correction
```

No change to `src/migration.rs`, `src/canonical.rs`, or any typed-record struct — every already-shipped mechanism above is cited, not modified. No `Cargo.toml`/`Cargo.lock` dependency change: `src/bin/flake-migrate.rs` links only the existing `fehrest`/`serde_json` dependencies already in the workspace.

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

**S-scale (local, Windows, this development host):** 100 records, ~10 MiB total payload (plan §27's own S definition), release build. `preview` 1.62 s, `import` 0.47 s — both comfortably inside the closest §27 analog ("Full export/import M": target 60 s, maximum 180 s), independent verification (`head_hash_chain_verified: true`, exact count agreement across preview/import/independent-reader) passing. Raw: `results/s-scale-timing-windows-local.json`.

**M-scale (10,000 records, ~1 GiB payload) and L-scale (100,000 records, ~10 GiB payload, attempted disk permitting):** deliberately **not** run on this local host given the 4.5 GiB free-space constraint above — this repeats a real, previously-observed risk on this exact host (a prior task's evidence records free space dropping from ~1.5 GiB to ~22 MiB in about 15 minutes from unrelated background activity). Run instead on a `ubuntu-latest` GitHub Actions runner via the `m-scale-performance` job in `.github/workflows/t05-01-migration-qualification.yml`, which reports its own exact CPU/RAM/disk before running (plan §27's own "record actual CPU/RAM/SSD/OS/filesystem" instruction) and requires at least 24 GiB free before attempting L, else records why L was skipped rather than risking an uncontrolled runner failure mid-measurement.

*(Updated once the CI run referenced below completes — see "Cross-platform and CI qualification".)*

## 6. Standalone binary integration tests (`tests/flake_migrate_binary.rs`)

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

*(This section is completed with the actual CI run ID and M/L-scale results once that run finishes — see the follow-up evidence commit on this task's PR before merge.)*

## Explicit scope boundaries (recorded, not silently dropped)

- **No format-3.** This task freezes format 2 as the current epoch and documents the policy that would govern a future epoch; it does not invent one (§21: "no invented future production format" is a hard boundary this task does not work around).
- **No new CLI subcommand on the main `fehrest` binary.** The standalone tool is deliberately separate — see its own module docs.
- **M/L-scale timing measured on one native profile (`ubuntu-latest` CI), not all three**, for the disk-risk reasons stated above — an explicit, bounded limitation of this task's own execution environment, not of the product.
- **D1-D6 full native durability qualification is `T05-02`'s own task**, not repeated here beyond D5 for migration specifically (already fully covered by the pre-existing `d5_migration_interruption_fault_schedule_matrix`).

## Completion condition

Pending: the CI run above completing on all three platforms plus the M/L-scale job, then a follow-up evidence commit on this task's PR recording the exact run ID and results, `specs/CURRENT.md` marked `T05-01_STATUS=COMPLETE`, merge, and post-merge main/CI reverification — following the identical pattern already established at every prior `flake-v1` task.
