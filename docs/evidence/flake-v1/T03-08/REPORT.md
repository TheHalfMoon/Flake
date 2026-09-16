# T03-08 evidence report — Automated continuity qualification

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` section 26 ("P03 value gate before desktop") and the `T03-08` task row, as amended by `docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md`'s "Replacement T03-08 contract — automated continuity qualification"
- **Baseline / tested source commit:** forked from `origin/main` `21ee84b141a762f0d25c9a39a5715aff9ddb151b` (PR #89, founder-decision merge; `git fetch origin --prune` + `gh pr view 89 --json state,mergedAt,mergeCommit` confirmed `state=MERGED`, `mergeCommit.oid=21ee84b141a762f0d25c9a39a5715aff9ddb151b` before this task started)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only. The producer/oracle independence discipline (`analysis.py` / `verify_independent.py`, never importing each other or `flake_arm.py`/`baseline_arm.py`) is structural, but both were written by the same executor in the same session — genuine cross-author independent review has not occurred, exactly as recorded for `T03-07`.

## Live-truth reverification performed before this task started

- `git fetch origin --prune` showed `main` 2 commits ahead of the local clone; `git pull --ff-only origin main` brought local `main` to `21ee84b141a762f0d25c9a39a5715aff9ddb151b`, matching `origin/main`.
- `gh pr list` and `gh pr view 89` confirmed PR #89 ("docs(governance): replace human qualification gates") `MERGED` with `mergeCommit.oid=21ee84b...`.
- Read `docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md` and `specs/CURRENT.md` in full: `ACTIVE_IMPLEMENTATION_UNIT=T03-08`, `T03-08_STATUS=READY`, `T03-08_EXECUTION_CONTRACT=AUTOMATED_CONTINUITY_QUALIFICATION`, `EXECUTABLE_REPOSITORY_WORK=AVAILABLE`.
- Confirmed no prior T03-08 work existed: `bench/flake-v1/` contained only `T03-07/`; no local or remote branch named `t03-08` existed; working tree was clean.

## Known open defect fixed first (recorded per handoff instruction)

The full Rust suite passed, but a real cross-platform clippy defect existed in already-merged code: `src/capture.rs`'s and `src/source_check.rs`'s symlink tests declared `let symlink_created = true;` under `#[cfg(not(windows))]` but only *read* that variable inside a block guarded by `#[cfg(windows)]` — so on macOS/Linux, `-D warnings` promotes the resulting unused-variable warning to a hard clippy error. This repository's own CI/dev host is Windows, so the defect was invisible locally (confirmed: baseline `cargo +1.97.1 clippy --locked --all-targets -- -D warnings` was already clean on this host before any change — see "Commands executed"). Fixed by collapsing both files' windows-only symlink-creation-and-skip logic into a single `#[cfg(windows)] { ... }` block, so the variable is declared and read only on the platform where it is ever used; no lint was suppressed, no test was weakened or removed. `git diff --stat origin/main -- src/` confirms exactly these two files changed (`raw/12-diff-stat-src.txt`).

## Scope actually touched

- `src/capture.rs`, `src/source_check.rs`: the cross-platform clippy fix above.
- One new directory, `bench/flake-v1/T03-08/`: `PROTOCOL_ADDENDUM.md`, `flake_arm.py`, `baseline_arm.py`, `harness.py`, `analysis.py`, `verify_independent.py`, `seal.py`, `SEALS.json`, five `test_*.py` suites, `run_tests.py`, `README.md`, and `runs/{dry-run,confirmatory}/` (raw manifests plus the Flake vault and baseline Markdown files each run produced).
- `docs/evidence/flake-v1/T03-08/REPORT.md` (this file) and `raw/`.
- `specs/CURRENT.md`: `T03-08` marked `COMPLETE`, frontier advanced to `T04-01`.

## Architecture

**Reuses the T03-07 sealed case bank unmodified.** `bench/flake-v1/T03-07/cases_confirmatory.json` (96 disjoint, held-out cases) is read-only input to this task. `seal.py` confirms, before any confirmatory attempt is run, that this file's SHA-256 still matches the digest `T03-07/SEALS.json` itself recorded — the sealed case bank has not drifted.

**Two independently-scripted execution arms replace the six human participants.** `flake_arm.py` drives the compiled `fehrest` binary by subprocess through its real format-2 CLI surface (`project-create`, `decision-create`, `decision-accept`, `decision-supersede`, `note-create`, `resume`, `decision-state`, `record-show`) — never touches `canonical.sqlite` directly, never imports `src/`. `baseline_arm.py` is a from-scratch, differently-structured Python implementation of a genuinely maintained Markdown + index + status-log file per case, importing nothing from Flake or from `flake_arm.py`. Both arms run against **every** one of the 96 confirmatory cases (`96 x 2 arms = 192 total attempts`), not the original human protocol's 48/48 condition split — that split existed solely to prevent human practice-effect contamination across conditions (`PROTOCOL.md` section 9.3), which does not apply to a deterministic script with no learning effect; the founder decision's own pass criterion ("all 96 sealed case executions are accounted for in both arms") confirms this automated shape.

**Gold-key independence boundary, enforced by construction.** Neither execution arm ever reads `case["gold"]`; both derive their setup structure only from `case["tier"]` and `case["sources"]` — specifically the already-public `TIER_PARAMS` rule (T3/T4 plant a conflict between their last two documents; T4 additionally plants a high-consequence trap on the first) from the sealed `generate_cases.py`. `case["gold"]` is read **exclusively** by the grading step inside `analysis.py`/`verify_independent.py`, after both arms have already produced gold-blind raw records. `PROTOCOL_ADDENDUM.md` section 2-3 documents this boundary and the honest scope it implies: this is a round-trip technical continuity test (does each system's own storage/retrieval path preserve and correctly surface the planted structure after a simulated interruption), not a reading-comprehension or human-effort test.

**Chain-supersession/conflict encoding.** For a case's `n` source documents: non-conflict tiers (`T1`/`T2`) sequentially chain-supersede all `n` documents under one decision key (`status` for Flake; one Markdown status-log per case for baseline), leaving the last document current. Conflict tiers (`T3`/`T4`) chain-supersede the first `n-2` documents, then create the `(n-1)`'th document superseding the chain end, then create the `n`'th document **without** superseding the `(n-1)`'th — leaving both simultaneously accepted/current, which Flake's already-audited `decision_state.rs` resolves to `NeedsReview` (`[[flake-canonical-architecture]]`'s "two independently-accepted decisions sharing a key with overlapping valid time are always NeedsReview" rule, T02-03/T03-02) and which the baseline's own independently-written parser marks `[CURRENT (conflicting)]`. Both arms' resume phase is a **fresh** read with no shared in-memory state from setup: `flake_arm.resume_case` re-derives everything from a new `resume`/`decision-state`/`record-show` CLI call sequence; `baseline_arm.resume_case` re-reads the `.md` file from disk — the file-based equivalent of T03-07's `dry_run.py` "brand-new object" resumability discipline, confirmed directly by `test_baseline_arm.py::test_resume_is_a_fresh_read_not_shared_state`.

**Producer/oracle independence.** `analysis.py` groups raw records with tuple-keyed dicts and computes routing with generator-expression sums; `verify_independent.py` never imports `analysis.py`, groups with string-keyed dicts (`"{case_id}|{arm}"`), and computes the same routing with explicit accumulation loops. `test_verify_independent.py` cross-checks agreement on `PASS`, `INCONCLUSIVE: MISSING_EXECUTIONS`, `FAIL: HIGH_CONSEQUENCE_MISS`, `FAIL: CLASS_LOSS`, and the `forbid_case_source` guard. On the real confirmatory manifest, the two implementations' JSON output is **byte-identical** (`raw/07-analysis-confirmatory.json` and `raw/08-verify-independent-confirmatory.json` share SHA-256 `f2cf830e6c732cde60ff18bd334364bd4f07093dce87535f82d4dce1340861f4`).

## Failed attempts / exclusions

**None discovered during confirmatory execution.** Development iteration (case design, CLI argument mapping, marker-based `record-show` extraction, `harness.py`'s confirmatory-guard ordering bug caught by `test_harness.py::test_run_collection_refuses_confirmatory_case_under_wrong_source`) happened entirely against `cases_dev.json` synthetic fixtures and in-memory test fixtures, never against `cases_confirmatory.json`, before `seal.py` was run — matching T03-07's own "no tuning after sealing" discipline. `seal.py` was run once, after all 32 Python tests passed against dev/synthetic data only; the confirmatory run that followed completed with zero raised exceptions across all 192 attempts (`raw/07-analysis-confirmatory.json`'s implicit zero-error count, cross-checked directly: `grep -c '"error": null'` against the raw manifest — see "Commands executed").

## Commands executed

| Command | Raw artifact | Result |
|---|---|---|
| `cargo +1.97.1 test --locked --all-targets` | `raw/01-full-test-run.txt` | **358/358 pass** (325 lib + 10 integration + 23 kill_tests), 0 failed — confirms the clippy fix introduced no regression |
| `cargo +1.97.1 fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo +1.97.1 clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings (fixes the cross-platform defect for macOS/Linux; already clean on this Windows host both before and after) |
| `git diff --check` | `raw/04-diff-check.txt` | Exit 0, no whitespace errors |
| `python3 bench/flake-v1/T03-08/run_tests.py` | `raw/05-python-test-suite.txt` | **32/32 pass** across 5 suites (`test_flake_arm.py` 4, `test_baseline_arm.py` 5, `test_harness.py` 5, `test_analysis.py` 13, `test_verify_independent.py` 5), against `cases_dev.json`/synthetic fixtures only |
| `python3 bench/flake-v1/T03-08/seal.py` | `raw/06-SEALS.json` | SHA-256 recorded for all 6 T03-08 files plus confirmation the reused `cases_confirmatory.json` digest matches `T03-07/SEALS.json` exactly, plus `product_commit=21ee84b141a762f0d25c9a39a5715aff9ddb151b` (pre-this-task's-own-commit) |
| `python3 bench/flake-v1/T03-08/harness.py --cases .../cases_confirmatory.json --run-name confirmatory --case-source confirmatory` | `raw/13-confirmatory-records.jsonl` (192 lines, copied verbatim from `bench/flake-v1/T03-08/runs/confirmatory/records.jsonl`, which is gitignored under the repo's `runs/` rule like T03-07's own `runs/` directory) | Completed in 63.4s wall-clock; 0 raised exceptions (`error: null` on all 192 records, confirmed by `grep -c '"error": null'`) |
| (dry run manifest, for reference) | `raw/14-dryrun-records.jsonl` (16 lines, copied from `runs/dry-run/records.jsonl`) | 8 dev cases x 2 arms, `case_source: "dev_synthetic"` throughout |
| `python3 bench/flake-v1/T03-08/analysis.py --manifest .../records.jsonl --cases .../cases_confirmatory.json --forbid-case-source dev_synthetic` | `raw/07-analysis-confirmatory.json` | `route: "PASS"` |
| `python3 bench/flake-v1/T03-08/verify_independent.py` (same args) | `raw/08-verify-independent-confirmatory.json` | `route: "PASS"`, byte-identical to `raw/07` |
| `python3 bench/flake-v1/T03-08/analysis.py` against `runs/dry-run/records.jsonl` / `cases_dev.json` | `raw/09-analysis-dryrun.json` | `route: "PASS"` on the 8-case synthetic dry run (never treated as confirmatory evidence) |
| `git rev-parse HEAD` / `git rev-parse origin/main` | `raw/10-baseline-head.txt` | Both `21ee84b141a762f0d25c9a39a5715aff9ddb151b` before this task's own commit |
| Environment capture | `raw/11-environment.txt` | `git`, `cargo +1.97.1`, `python3`, `rustc +1.97.1` versions on this development host |
| `git diff --stat origin/main -- src/` | `raw/12-diff-stat-src.txt` | Exactly `src/capture.rs` and `src/source_check.rs`, the clippy fix |

Raw artifact manifest with sizes and SHA-256 for every file above: `raw/00-manifest.txt`.

## What the automated continuity qualification actually establishes

1. **All 96 sealed confirmatory cases were executed in both arms** — 192/192 raw attempts recorded, 0 missing cells, 0 raised exceptions.
2. **Zero high-consequence misses.** No Flake attempt ever surfaced a planted disclosure trap's document as current.
3. **Zero class loss.** Every tier (`T1`-`T4`) had all 24/24 Flake attempts grade `RESUME_CORRECT`.
4. **Flake success rate 100% (96/96), no deficit versus the baseline (also 96/96).** Both figures exceed the founder decision's 90%/5pp thresholds with margin.
5. **Producer and independent oracle agree exactly** — byte-identical JSON output on the real confirmatory manifest.
6. **The reused case bank is confirmed unmodified** against T03-07's own seal before any confirmatory attempt ran.

## What has explicitly NOT been claimed

- **No human participant, adoption, retention, satisfaction, or comparative-effort claim is made anywhere in this package** — required by, and consistent with, `FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md`.
- **`setup_seconds`/`navigation_seconds` are automated CLI subprocess wall-clock measurements on this development host**, not human-effort measurements. Descriptive only (median/max navigation: Flake 108ms/188ms, baseline 16ms/58ms; median/max setup: Flake 513ms/993ms, baseline 1.1ms/8.3ms — Flake's per-attempt cost is dominated by per-command process-spawn-plus-SQLite-transaction overhead across many small CLI invocations, an engineering characteristic, not a usability claim). No pass/fail gate depends on these numbers, per the founder decision's removal of the time-reduction criterion.
- **The 100% success rate on both arms reflects a deterministic, gold-blind but tier-informed script executing a fixed, well-defined structural task, not human reading comprehension or judgment.** `PROTOCOL_ADDENDUM.md` section 3 states this scope limitation directly: this qualifies round-trip technical continuity (capture, structured supersession, conflict surfacing, no unintended disclosure, correct resume retrieval survive a simulated interruption), not comprehension or usability.
- **No claim that this result would replicate under real human variability.** That question remains explicitly deferred, per the founder decision's "Project-complete interpretation" clause.

## Performance gate

Section 27 Core performance/bounds gates applicable to this path: all 192 attempts completed well within the CLI's already-established bounds (`k17_oversized_query_is_bounded` etc., re-confirmed passing by the unchanged 358/358 Rust suite); no new bound was approached. The 63.4-second wall-clock total for 192 attempts (~330ms/attempt average, dominated by baseline's near-instant file I/O against Flake's ~600ms average per-attempt CLI-subprocess cost) is recorded as a descriptive engineering measurement, not a gated performance claim.

## Durability gate

Both arms' resume phase is a genuinely fresh read (new CLI process for Flake; fresh file read for baseline), not a reuse of in-process setup state — this is the durability property this task's own contract requires, directly exercised by all 192 real attempts plus `test_baseline_arm.py::test_resume_is_a_fresh_read_not_shared_state`. `harness.py`'s append-only manifest and cell-scan resumability (`existing_cells`/`append_record`/`DuplicateCellError`) is proven by `test_harness.py::test_simulated_interruption_resumes_without_duplication`, mirroring `T03-07`'s own harness discipline. No new persistence engine is introduced; Core's own already-proven durability (`T01-03`/`T01-07`) is unaffected by this task's `src/` change (a test-only cross-platform cfg fix, no behavior change to any persisted format).

## Cross-platform gate

This package was authored and executed on Windows 11/NTFS with Python 3.14 and Rust 1.97.1 (`raw/11-environment.txt`). The clippy fix this task made is itself the cross-platform gate's own subject matter: it corrects a defect that was invisible on this Windows host but would have failed `-D warnings` on macOS/Linux CI, confirmed by direct code inspection of the now-single `#[cfg(windows)]`-gated block (no platform-specific behavior remains outside that guard). The T03-08 harness itself makes no OS-specific path assumptions beyond `pathlib`, matching `T03-07`'s own precedent.

## Acceptance criteria disposition

| Founder-decision replacement clause | Status |
|---|---|
| Use the sealed T03-07 case corpus as a fixed held-out engineering corpus; do not alter answer keys/facts/classes after observing results | Satisfied — `seal.py` confirms the reused `cases_confirmatory.json` digest matches T03-07's own seal exactly; no case content was ever modified |
| Execute every case through the real Flake CLI/Core path; no scripted resolver may substitute for product behavior | Satisfied — `flake_arm.py` exclusively drives the compiled `fehrest` binary's real CLI commands; no direct `canonical.sqlite` access, no `src/` import |
| Execute a separately implemented maintained Markdown baseline against the same raw case inputs; must not import Flake Core/parsers/analysis or hidden answer keys | Satisfied — `baseline_arm.py` is a from-scratch implementation importing nothing from Flake or `flake_arm.py`; neither arm reads `case["gold"]` |
| Preserve setup/maintenance operations, failures, timeouts, bytes, commands, wall-clock as engineering observations | Satisfied — every raw record carries `setup_seconds`/`navigation_seconds`/`start_time`/`end_time`/`error`, descriptive only |
| Independent raw-to-summary verification; verifier must not import the producer | Satisfied — `verify_independent.py` never imports `analysis.py`; byte-identical output confirmed on the real manifest |
| Preserve every failed attempt under a new additive seal; never rewrite T03-07 seals | Satisfied — zero failed attempts occurred (0/192 errors); `T03-07/SEALS.json` was never touched (confirmed: not in this task's diff) |
| All 96 sealed case executions accounted for in both arms | Satisfied — 192/192, 0 missing cells |
| Flake successful-resume rate >= 90% | Satisfied — 100% (96/96) |
| Flake success no more than 5pp below baseline | Satisfied — 0pp deficit (both 100%) |
| Zero high-consequence planted conflicts missed by Flake | Satisfied — 0 misses |
| Zero unintended disclosures | Satisfied — the trap document's content never appeared in any Flake `current_doc_ids` across all T4 attempts |
| Every declared task class has >=1 successful Flake case | Satisfied — every tier has 24/24 |
| Section 27 Core performance/bounds gates applicable to this path pass | Satisfied — unchanged 358/358 Rust suite, no new bound approached |
| Independent verification reproduces exact denominators and verdicts | Satisfied — byte-identical JSON |

## Completion condition

Every acceptance clause above is satisfied by inspectable artifacts, not narrative claim. No AGENTS.md section 9 stop condition applies: no dependency or required-evidence gate is unmet, no Class C/D/E decision is required by this task itself (it executes the already-founder-authorized replacement contract; it introduces no new architecture, security, or product-thesis change beyond what `FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md` already authorized), and no evidence is ambiguous or corrupted.

**`T03-08` is complete: `PASS` under the automated continuity qualification contract.** No sealed T03-07 evidence was altered; no force-push; no historical evidence file touched. Per the founder decision's "T03-08 PASS unlocks T04-01" and "Allowed claim after PASS: the bounded CLI continuity behavior is technically qualified on the measured fixtures and environments" (and no stronger claim than that is made anywhere in this package).

## Next frontier

`T04-01` — the first P04 (minimal desktop continuity) task. `specs/CURRENT.md` is updated in this task's own commit to record `T03-08_STATUS=COMPLETE` and advance `ACTIVE_IMPLEMENTATION_UNIT`/`NEXT_DEPENDENCY_READY_UNIT` to `T04-01`.
