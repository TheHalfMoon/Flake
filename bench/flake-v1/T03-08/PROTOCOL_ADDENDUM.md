# T03-08 — Automated continuity qualification addendum (v1)

```text
ADDENDUM_VERSION: FLAKE-T03-08-AUTOMATED-v1
SUPERSEDES: the human-participant confirmatory execution described in
  bench/flake-v1/T03-07/PROTOCOL.md sections 2-12 (recruitment, consent,
  training, session structure) -- for T03-08 execution only
AUTHORITY: docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md
STATUS_AT_WRITING: no confirmatory attempt has been run against this code
```

This document is written and this package's harness/analysis/verification
code is sealed (`seal.py`) **before** any confirmatory attempt is run
against the sealed `bench/flake-v1/T03-07/cases_confirmatory.json` case
bank, mirroring `PROTOCOL.md` section 18's sealing discipline even though
this is an execution task, not a preregistration task.

## 1. What this replaces and what it keeps

T03-07's sealed case bank, gold keys, and success/failure/exclusion
definitions (`PROTOCOL.md` section 13) are reused **unmodified**. What is
replaced is exclusively the *executor*: instead of six human participants
each completing eight matched pairs, this package runs **every** one of the
96 sealed confirmatory cases through two independently-scripted arms:

- `flake_arm.py` -- the real Flake CLI/Core path (subprocess against the
  compiled `fehrest` binary).
- `baseline_arm.py` -- an independent maintained-Markdown +
  index/task-list baseline, written from scratch, importing nothing from
  Flake.

`96 cases x 2 arms = 192 total attempts` -- not the original protocol's
48/48 condition split, since that split existed only to prevent human
practice-effect contamination across conditions (`PROTOCOL.md` section
9.3), which does not apply to a deterministic script with no learning
effect. The founder decision's own pass criterion ("all 96 sealed case
executions are accounted for in both arms") confirms this is the intended
automated shape.

## 2. The gold-key independence boundary

Neither `flake_arm.py` nor `baseline_arm.py` ever reads `case["gold"]`.
Both derive their setup structure only from `case["tier"]` and
`case["sources"]` -- specifically, from the rule (openly documented in the
already-sealed `bench/flake-v1/T03-07/generate_cases.py`'s `TIER_PARAMS`
table, not a secret) that tiers `T3`/`T4` plant a genuine conflict between
their last two source documents, and `T4` additionally plants a
high-consequence disclosure trap on the first document. This is public
case-CONSTRUCTION knowledge, not the sealed per-case answer key.

`case["gold"]` (`correct_next_action`, `superseded_doc_ids`,
`planted_conflict_doc_ids`, `high_consequence_trap`) is read **exclusively**
by the grading step inside `analysis.py`/`verify_independent.py`, after
both arms have already produced their raw, gold-blind execution records.
This mirrors how a human participant is scored against a gold key they
never see.

## 3. What is, and is not, being measured

Because no LLM or human judgment participates in either arm, this is
honestly a **round-trip technical continuity test**, not a reading-
comprehension test: both arms are handed the same tier-structural shape
(which documents chain-supersede which, and which pair is left genuinely
competing) and the question is whether each system's own storage/retrieval
path -- Flake's `decision-create`/`decision-accept`/`decision-supersede`/
`resume`/`decision-state`/`record-show` CLI surface, or the maintained
Markdown file's own index -- correctly preserves and surfaces that shape
after a simulated interruption (a fresh process re-read, never shared
in-memory state; see `baseline_arm.resume_case`'s and
`flake_arm.resume_case`'s own docstrings).

**Allowed claim after PASS**: the bounded CLI continuity behavior (capture,
structured supersession, conflict surfacing, no unintended disclosure,
correct resume retrieval) is technically qualified on these 96 fixtures and
this environment.

**Forbidden claim**: that this proves reading comprehension, human
usability, adoption, retention, or reduced human effort. No such claim is
made anywhere in this package or its evidence report.

## 4. Operationalized RESUME_CORRECT (PROTOCOL.md section 13, automated form)

For a given raw record (`current_doc_ids`, `conflict_flagged`) graded
against its case's gold key:

- **Non-conflict case** (`gold.planted_conflict_doc_ids` empty):
  `RESUME_CORRECT` iff `conflict_flagged` is false, `current_doc_ids`
  equals exactly `{sources[-1].doc_id}`, and no `gold.superseded_doc_ids`
  entry appears in `current_doc_ids`.
- **Conflict case**: `RESUME_CORRECT` iff `conflict_flagged` is true,
  `current_doc_ids` equals exactly `gold.planted_conflict_doc_ids`, and no
  `gold.superseded_doc_ids` entry appears in `current_doc_ids`.
- **High-consequence miss**: if `gold.high_consequence_trap` is set and its
  `trap_doc_id` appears in `current_doc_ids`, the attempt is forced to
  `RESUME_FAILED` regardless of the above (matching PROTOCOL.md section
  13's three-conjunct definition -- partial credit does not exist).
- A raised exception during setup/resume is recorded as `error` on the raw
  record and always grades `RESUME_FAILED`, never silently retried.

## 5. Automated P03 pass criteria (verbatim from the founder decision)

Computed by `analysis.py` in this fixed order, and independently
recomputed by `verify_independent.py` from the same raw manifest:

1. All 96 sealed case executions are accounted for in both arms, or route
   `INCONCLUSIVE: MISSING_EXECUTIONS`.
2. Zero high-consequence misses on the Flake arm, or route
   `FAIL: HIGH_CONSEQUENCE_MISS`.
3. Every tier (`T1`-`T4`) has at least one `RESUME_CORRECT` Flake case, or
   route `FAIL: CLASS_LOSS`.
4. `flake_success_rate >= 0.90`, or route
   `FAIL: RESUME_SUCCESS_BELOW_90PCT`.
5. `(baseline_success_rate - flake_success_rate) * 100 <= 5.0`, or route
   `FAIL: DEFICIT_EXCEEDS_5PP`.
6. Otherwise, `PASS`.

The former `>=20% lower median paired maintenance-plus-resume time`
criterion (`PROTOCOL.md` section 20 step 6) is **not** computed as a pass
gate here -- the founder decision removes it because automated execution
cannot establish human effort. `setup_seconds`/`navigation_seconds` are
still recorded on every raw record as descriptive engineering
measurements only (wall-clock CLI subprocess timings, not human effort).

## 6. Confirmatory/dev-synthetic boundary

`analysis.analyze(..., forbid_case_source="dev_synthetic")` and
`verify_independent.verify(..., forbid_case_source="dev_synthetic")` both
raise a hard error if any input record carries `case_source:
"dev_synthetic"` -- the same guard discipline `T03-07`'s `analysis.py`
established, exercised here by `test_analysis.py::test_forbid_case_source_guard_raises`
and its `verify_independent.py` counterpart. The dry run
(`runs/dry-run/records.jsonl`) uses only `cases_dev.json`
(`case_source: "dev_synthetic"`), never `cases_confirmatory.json`.

## 7. Sealing

Before any confirmatory attempt is run, `seal.py` records SHA-256 digests
of every load-bearing file in this package (this document, both execution
arms, the harness, and both analysis implementations) plus a confirmation
that the reused `bench/flake-v1/T03-07/cases_confirmatory.json` is
byte-identical to what T03-07 itself sealed, plus the exact product
commit/tree this run is executed against. Any change to a sealed file
after a confirmatory record exists against it invalidates that record set
under the same one-permitted-repair-repeat discipline `PROTOCOL.md` section
15 already establishes for this case bank.

## 8. What this addendum does not claim

- No participant, human or otherwise, is described as a study participant,
  reviewer, or adoption observation anywhere in this package (founder
  decision's explicit prohibition).
- `setup_seconds`/`navigation_seconds` are automated CLI wall-clock
  timings on this development host, not human-effort measurements.
- A `PASS` licenses proceeding to `T04-01`; it is not a population-level or
  comparative-effort claim (`PROTOCOL.md` section 22, unchanged).
