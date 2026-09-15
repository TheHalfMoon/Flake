# T03-07 evidence report — Preregister the local continuity proof

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` section 26 ("P03 value gate before desktop") and the `T03-07` task row (lines 1115-1137) — seventh task of `P03`, depends on `T03-06`
- **Baseline / tested source commit:** forked from `origin/main` `29263249e17fbaf93bd6a2d764fed418da255feb` (PR #86, `T03-06` merge; post-merge required checks all green — verified directly via `gh api .../commits/29263249.../check-runs` before this task started)
- **Dirty diff digest:** this task adds no `src/` changes; `git diff --stat origin/main -- src/` is empty (`raw/11-diff-stat-src.txt`)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only. The analysis/verification independence discipline (section 28's "avoid using the same implementation as both producer and oracle") is structural — `verify_independent.py` is a separately written module that never imports `analysis.py` — but both were written by the same executor in the same session; genuine cross-author independent review has not occurred.

## Live-truth reverification performed before this task started

- `git fetch origin --prune` confirmed `main` at `29263249e17fbaf93bd6a2d764fed418da255feb`, matching `specs/CURRENT.md`'s `T03-06_STATUS=COMPLETE` and `ACTIVE_IMPLEMENTATION_UNIT=T03-07`.
- `gh pr view 86 --json state,mergedAt,mergeCommit` confirmed `state=MERGED`, `mergeCommit.oid=29263249e17fbaf93bd6a2d764fed418da255feb`.
- `gh api repos/TheHalfMoon/Flake/commits/29263249.../check-runs` confirmed all 8 named post-merge checks (`manifest-check`, `canonical-equality`, `validate`, `test-review-binding`, `test-statistical-design`, `test-validator`, `test-scorer`, `verify-artifacts`) `completed`/`success`.
- Read the full `T03-07` task-contract row, section 26 ("Metrics and value gates"), section 28 ("Evidence artifact model"), and section 31's dependency DAG in full before writing anything, per this task's own scope.
- Confirmed `specs/CURRENT.md` still carried the stale `T03-06_MERGE_COMMIT=PENDING_PR_MERGE` pointer; corrected in this task's own commit (see "Frontier update" below), per the founder handoff's explicit instruction not to create a cosmetic standalone PR for it.

## Scope actually touched

- One new directory, `bench/flake-v1/T03-07/`: the full preregistration artifact family (protocol, consent/privacy package, case generator + sealed case files, allocation mechanism + sealed allocation table, harness, analysis, independent verification, synthetic dry run, seal, seven test suites, a test aggregator, and a `README.md` index).
- `specs/CURRENT.md`: corrected the stale `T03-06_MERGE_COMMIT` pointer, added the `T03-07` block, and advanced the frontier (see "Frontier update").
- No `src/` changes — this task's scope is exclusively the preregistration package, per its own "Allowed scope" clause.

## Architecture

**The protocol document is the actual specification, not a post-hoc description of the code.** `PROTOCOL.md` was written first, fixing the exact design (6 participants × 8 pairs × 2 conditions = 96 attempts, four difficulty tiers, exact counterbalancing formulas, exact success/failure/timeout/exclusion definitions, and the exact mechanical decision-routing order) before `generate_cases.py`, `allocation.py`, `harness.py`, or `analysis.py` were written against it. `test_protocol.py` checks the frozen document actually states every element the task contract and Section 26 require (a structural, not scientific-quality, check).

**Deterministic generation is the reconstructability mechanism.** `generate_cases.py` and `generate_allocation.py` are pure functions of a fixed seed; re-running either reproduces byte-identical output to the sealed files (verified directly: `raw/05-generate-cases.txt`/`raw/06-generate-allocation.txt` show the regenerated SHA-256 digests matching `SEALS.json`'s recorded digests, and `test_cases.py::test_generation_is_deterministic` asserts this in the test suite). This is what makes the design "independent reviewer can reconstruct pairing and total-time calculation before results" (the task's own acceptance criterion) actually true rather than aspirational: an independent reviewer runs the two generator scripts and diffs the output against the sealed files.

**Counterbalancing is exact, not merely "randomized."** `allocation.py`'s `case_to_condition_map` and `condition_attempted_first` are pure parity functions of `(participant_slot, pair_index)`, deliberately offset from each other so the two counterbalancing factors are not correlated. `generate_allocation.py`'s output confirms the exact promised balance: 24/24 case-role split, 24/24 first-condition split, and every tier appearing exactly twice per participant (`raw/06-generate-allocation.txt`).

**Equal source budget is enforced by construction, not by convention.** `generate_cases.py`'s `_equalize_source_budget` pads the shorter side of a pair byte-for-byte so both conditions receive an exactly identical total source byte count — `test_cases.py::test_equal_source_budget_within_each_pair` confirms zero byte difference across all 48 pairs, strictly stronger than the originally drafted "±64 bytes" tolerance (tightened during this task after the first generator draft produced a 74-byte gap — see "Failed attempts").

**Analysis (producer) and verification (oracle) are structurally independent.** `analysis.py` groups pairs with tuples and computes medians via the `statistics` module; `verify_independent.py` never imports `analysis.py`, reads the raw `.jsonl` manifest itself (not via `harness.RunManifest`), groups pairs with string-keyed dicts, and computes the median by sorting and indexing by hand. `test_verify_independent.py` cross-checks agreement across every routing branch (`PASS`, `INCONCLUSIVE`, each `FAIL` reason) and confirms the two implementations' numeric outputs match to floating-point precision.

**The append-only manifest proves resumability by actually being interrupted.** `dry_run.py`'s `run_collection_with_simulated_interruption` writes half of a 96-attempt synthetic run, then constructs a **brand-new** `RunManifest` object (no shared in-memory state, simulating a real process restart) and continues from `missing_cells()` — `test_dry_run.py::test_simulated_interruption_resumes_without_duplication` confirms all 96 cells exist exactly once afterward, and a duplicate-append attempt is confirmed to raise `DuplicateCellError` rather than silently double-recording.

**The confirmatory/dev-synthetic boundary is a hard runtime guard, not a naming convention.** `analysis.analyze(..., forbid_case_source="dev_synthetic")` raises `AnalysisError` if any record in the input carries `case_source="dev_synthetic"`; `dry_run.py::confirm_dev_guard_raises` and `test_dry_run.py::test_confirmatory_guard_rejects_dev_data` exercise this directly. This is the mechanism behind PROTOCOL.md section 14.2's promise that dev/synthetic data "cannot be accidentally analyzed as confirmatory."

## Failed attempts / exclusions

**First generator draft violated its own promised equal-source-budget tolerance.** The first version of `generate_cases.py` padded each source document to a per-document-index byte target independently for role A and role B; because entity names/verbs are chosen by an independent seeded draw per role, the two sides' total byte counts differed by up to 74 bytes — discovered by `test_cases.py`'s equal-budget check before this task claimed the requirement satisfied. Rather than loosen the check, the generator was restructured (`_build_docs` / `_equalize_source_budget` / `_finish_case`, generating both roles of a pair together and padding the shorter side byte-for-byte to match exactly) and `PROTOCOL.md` section 7 was tightened from "±64 bytes, unavoidable rounding" to an exact-equality promise the code now actually keeps (verified: `max byte diff after equalization: 0` across all 48 pairs). No test was loosened to reach green.

**No other defect was found.** The synthetic dry run (`raw/07-dry-run.txt`) passed on its first complete run once the harness/analysis/verification modules were written against the already-corrected case generator; no repair-repeat (PROTOCOL.md section 15) was invoked, since it applies only to defects discovered after sealing, and this one was caught by the test suite before `seal.py` was run.

## Commands executed

| Command | Raw artifact | Result |
|---|---|---|
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **358/358 pass** (325 lib + 10 integration + 23 kill_tests), 0 failed — no Rust source was changed by this task; re-run to confirm no regression |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `python3 bench/flake-v1/T03-07/run_tests.py` | `raw/04-python-test-suite.txt` | **53/53 pass** across 7 suites (`test_cases.py` 8, `test_allocation.py` 7, `test_harness.py` 10, `test_analysis.py` 9, `test_verify_independent.py` 7, `test_dry_run.py` 6, `test_protocol.py` 6), 0 failed |
| `python3 bench/flake-v1/T03-07/generate_cases.py` | `raw/05-generate-cases.txt` | Regenerated digests match `SEALS.json` exactly (reconstructability check) |
| `python3 bench/flake-v1/T03-07/generate_allocation.py` | `raw/06-generate-allocation.txt` | Regenerated digest matches `SEALS.json`; counterbalance summary: 24/24 case-role split, 24/24 first-condition split, 2 attempts per tier per participant |
| `python3 bench/flake-v1/T03-07/dry_run.py` | `raw/07-dry-run.txt` | `analysis_route: "PASS"`, `verify_independent_route: "PASS"`, `producer_and_oracle_agree: true` on the synthetic 96-attempt dev-case run; all six required dry-run scenarios (timeout, exclusion, protocol deviation, simulated interruption/resume, duplicate-append refusal, dev-guard rejection) confirmed present |
| `python3 bench/flake-v1/T03-07/seal.py` | `raw/08-seal.txt` | SHA-256 recorded for all 11 sealed files plus `product_commit=29263249e17fbaf93bd6a2d764fed418da255feb`, `product_src_tree=d237c5fd34119c45c3eb117144fe767e42fd3887` |
| `git rev-parse HEAD` / `git rev-parse origin/main` | `raw/09-baseline-head.txt` | Both `29263249e17fbaf93bd6a2d764fed418da255feb` before this task's own commit |
| Environment capture (`git --version`, `cargo --version`, `python3 --version`, `uname -a`) | `raw/10-environment.txt` | Matches this development host's already-recorded toolchain; Python 3.14.7 recorded explicitly since this task's own artifacts are Python, per the same convention `T03-06` established |
| `git diff --stat origin/main -- src/` | `raw/11-diff-stat-src.txt` | Empty — confirms no product code was touched |
| `git diff --check` | `raw/12-diff-check.txt` | Exit 0, no whitespace errors |

Raw artifact manifest with sizes and SHA-256 for every file above: `raw/00-manifest.txt`.

## What the preregistration package actually establishes

1. **A frozen, mechanically-routable protocol** (`PROTOCOL.md`): exact eligibility, recruitment, consent, withdrawal, privacy/data-minimization, training, condition-order/counterbalancing, case-assignment, timing/budget, maintenance-accounting, success/failure/timeout/exclusion, missing-data/protocol-deviation, class-loss, one-permitted-repair-repeat, analysis-plan, and decision-routing sections — checked structurally present by `test_protocol.py`.
2. **A sealed, disjoint, held-out case bank** (`cases_confirmatory.json`, 96 cases): one per `(participant_slot, pair_index, condition_role)` cell, exactly matching the Section 26 design shape, with zero duplicate content anywhere across the 96 confirmatory and 8 dev cases (`test_cases.py::test_no_duplicate_content_anywhere`), exact per-pair source-byte equality, and every tier (`T1`..`T4`) balanced at exactly 2 pairs per participant.
3. **A deterministic, reconstructable allocation table** (`allocation_confirmatory.json`): condition order and case-role mapping are pure functions of two integers, independently reproducible, with the exact promised counterbalance.
4. **A resumable, append-only, schema-validated raw manifest mechanism** (`harness.py`), proven under a simulated process interruption.
5. **A producer/oracle pair** (`analysis.py` / `verify_independent.py`) that independently agree on every one of the seven possible routes (`PASS`, `INCONCLUSIVE: INSUFFICIENT_VALID_PAIRS`, and the five `FAIL:` reasons) exercised by the test suite.
6. **A successful synthetic dry run** exercising every required scenario in PROTOCOL.md section 19, using only non-confirmatory dev cases.
7. **A hash seal** (`SEALS.json`) over every load-bearing artifact plus the exact product commit/tree this study is frozen against, computed and recorded before any confirmatory human observation exists.

## What has explicitly NOT been claimed

- **No participant has been recruited.** No consent has been obtained from a real person. No confirmatory attempt exists. `runs/confirmatory/` does not exist.
- **No human timing, human outcome, human error, or human qualitative data exists anywhere in this package.** Every number in `raw/07-dry-run.txt` is synthetic fixture data explicitly generated from `cases_dev.json` for harness/protocol validation only, never treated as evidence of Flake's value — `analysis.py`'s runtime guard makes accidental conflation a hard error, not a documentation promise.
- **No claim of `PASS`, `FAIL`, or human-study outcome for the actual continuity proof is made here.** That verdict can only exist after `T03-08` actually runs the sealed protocol with real participants.
- **This is a small, local, six-participant investment gate design**, not a population-level efficacy claim, per Section 26 and PROTOCOL.md section 22.

## Performance gate

Not applicable to this task's own scope in the Section 27 sense (no product code changed, no timed product operation measured). The protocol's own timing budgets (10-minute setup, 5-minute interruption, 8-minute resume) are interaction-design constraints for the eventual `T03-08` execution, not a performance claim being made now.

## Durability gate

`harness.py`'s append-only manifest is proven resumable under a simulated process interruption (`dry_run.py`, `test_dry_run.py::test_simulated_interruption_resumes_without_duplication`); this is the durability property the task contract requires ("raw records append with resumable state manifest and no silent lost attempts"). No new persistence engine is introduced — this is a plain `.jsonl` file, and Core's own already-proven durability (`T01-03`/`T01-07`) is untouched since no `src/` code changed.

## Cross-platform gate

This package was authored and tested on Windows 11/NTFS with Python 3.14.7 (`raw/10-environment.txt`), matching this development host's `reference-hardware.md` profile. The protocol itself requires recording "study host profiles" once real sessions run (`T03-08`'s scope); nothing about the preregistration package is platform-specific (pure-Python, stdlib-only, no OS-specific path assumptions beyond `pathlib`).

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Every outcome can be routed mechanically to Pass/Fail/Inconclusive | Satisfied — `analysis.py`'s seven-branch fixed-order routing, cross-checked by `verify_independent.py`; exercised for every branch by `test_analysis.py`/`test_verify_independent.py` |
| Independent reviewer can reconstruct pairing and total-time calculation before results | Satisfied — `generate_cases.py`/`generate_allocation.py` are deterministic and reproduce the sealed files exactly (`raw/05`, `raw/06`); `analysis.py`'s total-time calculation is a pure function of the raw manifest, independently re-derived by `verify_independent.py` |
| Section 26 six-participant/eight-pair design, held-out gold, counterbalancing, equal source/setup/access budgets, all maintenance costs | Satisfied — 6×8×2=96 exactly; `cases_confirmatory.json` is held-out/sealed; counterbalancing verified exact 24/24; source budget verified exact-equal; setup/access budgets fixed in `PROTOCOL.md` section 7; all five maintenance-cost phases (`setup`, `maintenance`, `navigation`, `recovery`, `answer`) captured in the raw manifest schema and summed by both analysis implementations |
| Seal product/harness/case/analysis hashes before execution | Satisfied — `SEALS.json` records all 11 load-bearing files plus the exact product commit/tree, and this is written before any confirmatory attempt exists |
| Exact success/timeout/exclusion rules, class-loss route, one allowed disjoint repair repeat | Satisfied — `PROTOCOL.md` sections 13/15; `analysis.py`'s class-loss branch and `test_analysis.py::test_fail_on_class_loss` exercise the route directly |
| Baseline uses maintained Markdown/index/task list, not an intentionally weak raw dump | Satisfied — `PROTOCOL.md` section 10 explicitly requires the baseline to be genuinely maintained with the same setup budget as Flake |
| No model execution or private data publication | Satisfied — no model/API call anywhere in this package; `CONSENT.md` section 6/9 and `PROTOCOL.md` section 6 bound what may ever be published |
| S07; opt-in data minimization and consented evidence; source content never enters public logs by default | Satisfied — `CONSENT.md` sections 5/6/9 |
| I12; raw run manifests include actual harness and previous state, no score-based authority | Satisfied — `harness.py`'s required-field schema includes `harness_version`, `previous_state`; outcome routing never depends on a numeric score, only the mechanical `RESUME_CORRECT`/`RESUME_FAILED`/`RESUME_TIMEOUT` definitions |
| Tests: V01/V05/V09/V15 protocol/dry-run validation using synthetic non-evaluation cases only | Satisfied — all seven `test_*.py` suites use only `cases_dev.json`-derived or purely synthetic fixtures; zero confirmatory-case data is ever exercised by a test |
| Verification method: dry-run collection/analysis scripts and independent arithmetic checks; seal exact artifacts and no-API execution path | Satisfied — `raw/07-dry-run.txt`, `raw/08-seal.txt` |

## Completion condition

Every acceptance clause above is satisfied by inspectable artifacts, not narrative claim. Per the task's own "Completion condition" ("Every acceptance clause above plus SC and predecessor/phase gates passes"), no AGENTS.md section 9 stop condition applies: no dependency or required-evidence gate is unmet, no Class C/D/E decision is required (this task made no architecture, security, or product-thesis change — it is a protocol/harness authorship task inside already-authorized scope), and no evidence is ambiguous or corrupted. Re-reading the task contract's own "Failure behavior" clause — "Missing participants/gold/budget enforcement prevents trial start; protocol drift invalidates affected run visibly" — confirms this describes what blocks the *trial itself* (`T03-08`), not this preregistration task's own closure; the "Acceptance criteria" clause (mechanical routing, independent reconstructability) contains no participant-count precondition and is fully satisfied without any participant.

**`T03-07` is complete.** No sealed evidence was altered after this report; no force-push; no historical evidence file touched.

## Next frontier

`T03-08` — Run the CLI continuity proof and decide desktop entry. This task's own files/components clause ("Preregistered local study runner/data/analysis and P03 acceptance report") point directly at the artifact family this task just built — there is no separate repository-ownable runner left to write. What remains for `T03-08` is exclusively the actual execution: recruiting six real, consenting, eligible participants and running the sealed protocol with them. That is a genuine external dependency this executor cannot supply (an execution agent cannot ethically create or impersonate consenting human participants), and is recorded as such in `specs/CURRENT.md`'s `T03-08` block and this report's sibling blocker packet.
