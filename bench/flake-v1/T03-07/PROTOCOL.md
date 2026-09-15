# T03-07 — Local continuity proof preregistration protocol (v1)

```text
PREREGISTRATION_VERSION: FLAKE-T03-07-PREREG-v1
PLAN_CONTRACT: docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md, section 26 ("P03 value gate before desktop"), task T03-07
STATUS_AT_WRITING: no participant has been recruited, no attempt has been observed, no product outcome exists
```

This document is written and sealed **before** any confirmatory human observation exists. Nothing in it may change after a real participant's attempt has been recorded, except through the one bounded defect-repair repeat in section 15, which itself requires a newly sealed, disjoint case set.

## 1. Research question

> For a solo owner of an evolving personal/work project, does interrupting and later resuming work on that project cost less total time, at equal or only slightly lower resume-correctness, when the project is maintained in Flake instead of a genuinely maintained Markdown/index/task-list workflow?

This is a **local, small-N investment gate**, not a market or population claim (Section 26). It does not compare Flake against every possible tool, and it does not attempt to prove product-market fit. A pass licenses further investment in the desktop shell (T04-01); it does not certify Flake for general release.

## 2. Target participant definition

The target group is an adult who currently maintains, or has recently and voluntarily maintained, an ongoing personal or work project that:

- spans multiple sessions across at least several days (not a single sitting),
- involves notes, decisions, or follow-up actions that must be found again later,
- has been interrupted and resumed at least once in the participant's own experience (so the task is recognizable, not invented).

## 3. Eligibility

A candidate is eligible only if **all** of the following hold, checked before scheduling:

1. Adult (18+), able to give informed consent in English.
2. Matches the target participant definition (section 2), self-reported and not fabricated by the facilitator.
3. Comfortable typing at a keyboard and following short written instructions; no CLI expertise required, but the pre-session training (section 8) must be completed and understood.
4. Has access to a computer that can run the exact qualified Flake CLI binary for their own platform (native build, see `docs/evidence/flake-v1/T03-07/REPORT.md` for the exact build identity once sealed).
5. Is not, and has never been, a contributor, employee, or close collaborator of the Flake project (avoids evaluator bias).
6. Has not previously participated in this specific study (no repeat participants; prevents practice-effect contamination of the sealed cases).
7. Can commit to the full eight-pair session plan (section 9) inside the recruitment window without splitting a single pair across widely different days (section 9.2).

Eligibility is checked and recorded **before** condition order or case assignment is revealed to the participant or the facilitator for that participant's slot.

## 4. Recruitment rules

- Recruitment is voluntary and local; no compensation is required by this protocol, and if compensation is offered it must be fixed per participant regardless of outcome (never performance-based, which would bias effort asymmetrically between conditions).
- Recruitment messaging describes the study honestly as "comparing two ways of picking a project back up after a break" and does not mention which result is expected or desired.
- Exactly six participants are recruited to fill six participant slots (`P1`..`P6`, pseudonymous — see section 6). No more than six are analyzed under this seal; if a seventh person is needed to replace a withdrawal, the replacement is a **new** slot fill under the vacated pseudonymous ID, not an addition to N, and their earlier partial data (if any) is retained as a withdrawal record (section 12), never discarded.
- Recruitment channel and exact wording are recorded in the evidence report once recruitment actually starts; this protocol does not depend on which channel is used.

## 5. Consent process

Full text is in `bench/flake-v1/T03-07/CONSENT.md`. Summary of the mechanical steps:

1. Candidate reads the consent document in full before any eligibility screening question that reveals study content.
2. Candidate gives affirmative, recorded (written or typed) consent before any timed activity begins.
3. Consent is re-confirmed verbally/in writing at the start of each of the two sessions (section 9.2) if the two sessions are not on the same day.
4. Withdrawal is possible at any point with no requirement to give a reason (section 12).

## 6. Privacy and data minimization

- Participants are identified only by a pseudonymous slot ID (`P1`..`P6`) in every raw record, analysis file, and evidence report. No name, email, or other direct identifier is written into any file under `bench/flake-v1/T03-07/` or `docs/evidence/flake-v1/T03-07/`.
- The mapping from a pseudonymous ID to a real identity, if it must exist at all (e.g., for scheduling), is kept only by the facilitator, outside this repository, and is destroyed no later than 90 days after the study's completion report is sealed.
- Project content participants use during the study is their own **disposable or non-sensitive** material by design (section 7); the protocol never requires a participant to expose private, confidential, or otherwise sensitive content, and the facilitator must decline to record any such content that a participant volunteers anyway.
- Raw timing, outcome, and exclusion data may be published in this public repository in full. Any free-text qualitative remark a participant makes is redacted of identifying detail before it is committed, or omitted entirely if it cannot be redacted safely.
- No source content, note bodies, decision text, or file contents a participant types during the study are committed to public evidence by default — only structural facts (timings, outcome codes, case/condition identifiers, exclusion reasons) are. An participant may explicitly opt in, in writing, to have a specific redacted excerpt quoted; absent that, nothing of the kind is published.

## 7. Case material and equal budgets

Every attempt uses a **preregistered case**: a small bundle of source documents (the "raw sources") describing a fictional-but-realistic small project state, an interruption point, and a resumption question with a preregistered gold answer key. Cases are generated deterministically by `generate_cases.py` (section 14) and are described structurally, never handwritten ad hoc per participant.

Both conditions (section 10) receive, for their assigned case:

- **Equal source budget**: the same number of source documents and an exactly identical total source byte count regardless of condition (`generate_cases.py` pads the shorter side byte-for-byte to enforce this, never truncating either side).
- **Equal setup budget**: a fixed 10-minute wall-clock allowance to capture/organize the case's raw sources into the condition's own workflow (Flake project, or the maintained Markdown/index/task-list baseline) before the interruption.
- **Equal access budget**: a fixed 8-minute wall-clock allowance during the resume phase to answer the resumption question, starting only from what the participant captured/organized during setup (plus, for the baseline, ordinary local file search over their own maintained files — no web search, no Flake, no notes taken outside the condition's own workflow).
- **Equal training**: identical instructional material, adapted only in tool-specific mechanics (section 8), with the same time allowance to ask clarifying questions.

No maintenance action is ever "free" for either condition: setup time, any organization/reorganization time, source-checking time, correction time, navigation time, recovery time from a wrong turn, and the final resume/export time are **all** counted toward that condition's total maintenance-plus-resume time (section 16).

## 8. Training procedure

Before their first pair, each participant receives a fixed-length (20-minute maximum) walkthrough covering, for **both** conditions in the same session:

1. How to capture a note, mark a decision, and record a follow-up action (Flake), or how to add an entry to the maintained Markdown file and its index/task list (baseline).
2. How to search/find previously captured material in each condition.
3. How the interruption/resume cycle will work and what the resumption question will look like (using a **training-only example case**, drawn from `cases_dev.json`, never from the sealed confirmatory set).
4. Where the setup/access budgets and timers are visible to the participant during the real attempts.

The trainer does not demonstrate the confirmatory cases and does not tell the participant which condition Flake "expects" to win.

## 9. Session structure, condition order, and counterbalancing

### 9.1 Pairs and attempts

Each participant completes eight matched interruption/resumption pairs (**8 matched interruption/resumption pairs**), for six participants' eight pairs across the whole study. Each pair consists of exactly **2 attempts**: one under the Flake condition, one under the Baseline condition, using two different but difficulty-matched, disjoint cases (never the same case content twice, anywhere in the study — see section 14). `6 participants × 8 pairs × 2 attempts = 96 total attempts`, matching the Section 26 design exactly.

### 9.2 Interruption gap

Within a pair, after setup finishes (or the setup budget expires, whichever is first) and before the resume phase begins, the participant performs a **fixed 5-minute unrelated distractor task** (e.g., a short unrelated reading passage with a comprehension question, identical distractor material for both conditions of that pair) to simulate a genuine interruption without contaminating the resume phase with rehearsal. A pair's two attempts may be split across two sessions on different days if needed for the participant's schedule, but both attempts of a **single pair** are completed within the same calendar week to keep the "matched" comparison meaningful.

### 9.3 Order and counterbalancing — fully mechanical

Condition order (which of the pair's two attempts happens first) and case-to-condition mapping are **not** left to the facilitator's judgment. Both are computed deterministically from `(participant_slot, pair_index)` by `allocation.py` (section 15) before any participant is recruited, and are reconstructable by an independent reviewer from those two integers alone:

- **Case-to-condition mapping** alternates by `(participant_slot + pair_index) mod 2`: on even parity, the pair's first-generated case goes to Flake and the second to Baseline; on odd parity, the reverse. Across the 48 pairs in the study this yields exactly 24/24.
- **Condition attempted first** alternates by `(participant_slot + pair_index + 1) mod 2`, deliberately offset from the case-to-condition parity so the two counterbalancing factors are not perfectly correlated. Across the 48 pairs this yields exactly 24 Flake-first and 24 Baseline-first pairs.
- **Pair presentation order** (the sequence in which a participant's own 8 pairs, and their difficulty tiers, are presented across their sessions) is a deterministic pseudorandom permutation seeded by `sha256(f"pair-order:{participant_slot}:{MASTER_SEED}")`, so no participant does their pairs in a fixed 1..8 or difficulty-ascending order, but the exact order is reconstructable from the master seed once it is sealed.

`allocation.py`'s output is the single source of truth; the facilitator follows it and never improvises an order.

## 10. Conditions

| Condition | Construction |
|---|---|
| **Flake** | The qualified native Flake CLI build identified in the sealed evidence report. Capture, decisions, actions, source links, and resume/export all go through Flake's own commands. |
| **Baseline** | A genuinely maintained workflow: one Markdown file per case project, a maintained index/task-list section at the top of that same file (or a second small Markdown file acting as the index), and ordinary local file search (the operating system's file search or a text editor's find-in-files) over the participant's own maintained files. The baseline is maintained deliberately and given the same setup budget as Flake — it is never an intentionally weak raw-text dump with no index, and the participant is trained to keep it organized exactly as they would keep any Markdown project alive. |

## 11. Difficulty tiers and case assignment

Four difficulty tiers exist, `T1` (lowest) through `T4` (highest), distinguished structurally (not by subjective judgment) by the number of source documents, the number of intervening organizational actions between setup and resume, and whether a planted conflict or a planted high-consequence disclosure trap is present in the case (section 13). Each participant's 8 pairs use each tier **exactly twice**, so tier is balanced within participant and across the study. The tier assigned to each pair index is itself fixed by `generate_cases.py`'s deterministic tier rotation (section 14.1), not chosen by the facilitator.

Case assignment is **disjoint**: across all 6 participants and all 96 attempts, no two attempts ever use the same case content. This is what makes cases genuinely "held out" — a case a participant has seen cannot leak into another participant's attempt, and no case used for protocol/harness validation (`cases_dev.json`, section 14.2) is ever reused in the confirmatory set.

## 12. Withdrawal rules

- A participant may withdraw at any point, for any reason or no stated reason, with no pressure to continue.
- Any attempt already completed at the time of withdrawal is retained as valid data (it already happened under the sealed protocol) unless the participant specifically asks for their data to be deleted, which is honored and recorded as `WITHDRAWN_DATA_DELETED` for that slot.
- A withdrawal before a slot's first attempt leaves that slot's assigned cases and allocation unused; a replacement participant is recruited into the **same slot ID** so the sealed allocation table does not need to change, and the vacated slot's non-use is recorded, not hidden.
- A withdrawal mid-pair leaves that pair's incomplete attempt recorded as `EXCLUDED: PARTICIPANT_WITHDREW_MID_ATTEMPT`, never silently dropped.

## 13. Success, failure, timeout, and exclusion definitions

This section fixes the exact failure behavior for every attempt, defined once, mechanically, before any attempt occurs:

- **`RESUME_CORRECT` (attempt-level success)**: within the access budget, the participant (a) identifies the correct next action or current state for the case, matching the preregistered gold key, (b) does not act on or assert information that the case's gold key marks as superseded/stale, and (c) if the case plants a high-consequence conflict or disclosure-sensitive fact, the participant surfaces or correctly handles it rather than missing it. All three must hold; partial credit does not exist.
- **`RESUME_FAILED`**: the access budget is used in full (or the participant submits an answer) without satisfying `RESUME_CORRECT`.
- **`RESUME_TIMEOUT`**: the access budget elapses with no submitted answer at all. Recorded as a distinct outcome code from `RESUME_FAILED` for diagnostic reporting, but counted identically to a failure for the Section 26 qualification tally ("failed/timeout attempts remain failures" — no separate leniency).
- **`HIGH_CONSEQUENCE_MISS`**: a secondary flag (not a replacement for the above) set to true whenever a case's planted high-consequence conflict or disclosure trap is missed, regardless of whether the rest of the answer was otherwise correct. Section 26 treats even one such miss as gate-blocking on its own.
- **Exclusion**: an attempt is excluded from the primary Pass/Fail/Inconclusive computation only for a reason listed in section 13.1, decided and logged **before** the analysis script is run, never after seeing whether excluding it would change the result.

### 13.1 Preregistered exclusion reasons (exhaustive list)

```text
FACILITATOR_PROTOCOL_ERROR     wrong case, wrong budget, or wrong condition order was actually administered
TOOLING_CRASH_UNRELATED        Flake or the baseline tooling crashed for a reason unrelated to the product behavior under test (e.g., host OS fault)
PARTICIPANT_WITHDREW_MID_ATTEMPT
CASE_MATERIAL_DEFECT           a sealed case is discovered, before scoring, to contain an internal contradiction not intended by its design
MISSING_REQUIRED_FIELD         the raw manifest record for this attempt is missing a field listed in section 17.1 as required
```

No other reason may be used to exclude an attempt. An attempt is never excluded because its outcome was unfavorable to either condition.

## 14. Case generation and the development/confirmatory boundary

### 14.1 Confirmatory case bank

`generate_cases.py` deterministically produces `cases_confirmatory.json`: 96 disjoint cases, one per `(participant_slot 1..6, pair_index 1..8, condition_role A|B)` cell, each tagged with its difficulty tier (`T1`..`T4`, each tier appearing exactly twice per participant per section 11), its equal-budget source bundle, and its gold answer key (including whether it carries a planted conflict and/or a high-consequence disclosure trap). Generation is seeded by a single `MASTER_SEED` recorded in the case file itself and in the seal (section 18); re-running `generate_cases.py` with the same seed reproduces byte-identical output, which is exactly how an independent reviewer checks that the sealed file was not hand-edited after generation.

### 14.2 Development/synthetic validation cases

`cases_dev.json` holds a small, explicitly separate set of synthetic cases used **only** for: the training walkthrough (section 8), and the synthetic dry run (section 19). These cases are marked `"confirmatory": false` and are structurally excluded, by `analysis.py` itself, from ever contributing to a Pass/Fail/Inconclusive computation — the analysis script raises an error rather than silently accepting a dev-case attempt into the confirmatory tally.

### 14.3 Sealing boundary

Once `cases_confirmatory.json` is sealed (section 18), its content is never inspected for the purpose of tuning Flake, the baseline instructions, the training material, or the analysis thresholds. Any change to it after sealing invalidates the affected cases under the repair-repeat rule (section 15) and requires a freshly generated, disjoint replacement set under a new seal.

## 15. One permitted disjoint repair repeat

If, after the confirmatory seal, a genuine protocol or harness defect is discovered (not a disappointing result) — for example, a bug in the case generator that produces a case whose gold key contradicts its own source bundle — exactly **one** repair repeat is permitted:

1. The failed run and the defect are recorded in full, unmodified, as negative evidence (never deleted).
2. The exact fix is documented.
3. A **new**, disjoint case set is generated under a **new** seed and freshly sealed (section 18) before any further attempt is run.
4. A second failure or an Inconclusive result after this repeat stops the study for founder/architecture reconsideration (per the canonical plan's section 26 and section 39 stop rule) — no second repeat, no threshold adjustment, no cherry-picked rerun.

## 16. Maintenance-plus-resume time accounting

For every attempt, `harness.py` (section 17) records wall-clock durations for each of the following phases, and their sum is that attempt's **total maintenance-plus-resume time**:

```text
setup_seconds            capture/organize time during the setup budget (section 7)
maintenance_seconds      any additional organization/source-checking/correction time logged during setup (0 if none occurred)
navigation_seconds       time spent locating material during the resume phase before an answer is reached
recovery_seconds         time spent recovering from a wrong turn during resume (e.g., checking a stale lead, reopening a wrong file)
answer_seconds           time spent composing/submitting the final resume answer
```

A pair's comparison quantity is `(baseline_total − flake_total)` for that pair; the study-level quantity is the **median of the 48 pair-level flake totals and the median of the 48 pair-level baseline totals**, compared directly (section 20). No phase is dropped from either condition's total; nothing about Flake is measured as "instant."

## 17. Raw run manifest

### 17.1 Required fields, one JSON object per attempt

```text
schema_version            fixed literal "t03-07-run-v1"
participant_slot          "P1".."P6"
pair_index                1..8
condition                 "flake" | "baseline"
condition_order            "first" | "second"   (within the pair)
case_id                   from cases_confirmatory.json (or cases_dev.json for dry runs, see below)
case_source                "confirmatory" | "dev_synthetic"
attempt_index             monotonically increasing per (participant_slot, pair_index, condition)
harness_version           git commit of bench/flake-v1/T03-07 at run time
product_version           git commit + `fehrest --version` output of the Flake build used (baseline attempts record "n/a")
previous_state            the resulting state hash/summary from the setup phase, so resume can be checked against exactly what was captured
start_time / end_time     ISO-8601, both phases (setup start/end, resume start/end) recorded separately
phase_durations           the five fields from section 16
outcome                   "RESUME_CORRECT" | "RESUME_FAILED" | "RESUME_TIMEOUT"
high_consequence_miss     boolean
exclusion                 null, or one of the section 13.1 codes
protocol_deviation        null, or free-text description logged by the facilitator
```

### 17.2 Append-only, resumable

Records are appended, one per line, to `runs/<confirmatory|dry-run>/records.jsonl`; the file is never rewritten in place. `harness.py` can reconstruct exactly which `(participant_slot, pair_index, condition)` cells are already recorded by reading this file from the start, so a facilitator can resume mid-study after any interruption without risk of a duplicate or silently lost attempt — `harness.py` refuses to append a record for a cell that already has one, and refuses to skip a cell the allocation table says should exist once the study is declared complete.

## 18. Sealing

Before any confirmatory attempt is run, `seal.py` computes and records SHA-256 digests of every load-bearing artifact:

```text
this protocol document (PROTOCOL.md)
the consent/privacy package (CONSENT.md)
cases_confirmatory.json
cases_dev.json
allocation_confirmatory.json
analysis.py
verify_independent.py
harness.py
the exact product commit/tree this study will evaluate
```

These digests, plus the exact product commit, are written to `SEALS.json` and reproduced in the evidence report. **Any** change to a sealed file after this point requires a new seal and, if a confirmatory attempt has already been recorded against the old seal, invalidates that attempt under the repair-repeat rule (section 15) — there is no "minor edit" exception.

## 19. Synthetic dry run

Before recruitment, `dry_run.py` exercises the entire machinery end to end using **only** `cases_dev.json` (`case_source: "dev_synthetic"`), explicitly never treated as evaluation data:

1. Generate a full synthetic 96-attempt-shaped run (reusing the same allocation table shape, but against dev cases) including deliberately injected failures, a timeout, an exclusion, a protocol deviation, and a simulated mid-study interruption of the harness process itself (killed and restarted) to prove `harness.py`'s resume-without-duplication behavior.
2. Run `analysis.py` against the resulting manifest and confirm it reaches a mechanical Pass/Fail/Inconclusive route.
3. Run `verify_independent.py` against the same manifest and confirm it reaches the **same** numbers via an independently written computation path.
4. Confirm that a case from `cases_dev.json` cannot be accidentally analyzed as confirmatory data (section 14.2's guard actually raises).

The dry run proves the protocol's machinery is sound. It is never reported as, and never treated as, human evidence of Flake's value.

## 20. Analysis plan and decision routing

`analysis.py` computes, from the sealed confirmatory manifest only, in this fixed order:

1. **Data-completeness check.** If fewer than 44 of the 48 pairs (91.6%) have both attempts present, valid, and unexcluded, route to `INCONCLUSIVE: INSUFFICIENT_VALID_PAIRS` and stop. (44/48 keeps the threshold close to, but no laxer than, the Section 26 "at least 90%" resume-success bar itself, so data loss cannot itself manufacture a pass.)
2. **Safety check.** If any valid attempt has `high_consequence_miss = true`, route to `FAIL: HIGH_CONSEQUENCE_MISS` and stop, regardless of every other number.
3. **Class-loss check.** If any difficulty tier (`T1`..`T4`) has zero `RESUME_CORRECT` Flake attempts among its valid attempts, route to `FAIL: CLASS_LOSS` and stop.
4. **Resume-success gate.** Compute `flake_success_rate` and `baseline_success_rate` over valid attempts. If `flake_success_rate < 0.90`, route to `FAIL: RESUME_SUCCESS_BELOW_90PCT`.
5. **Relative-deficit gate.** If `(baseline_success_rate − flake_success_rate) * 100 > 5.0`, route to `FAIL: DEFICIT_EXCEEDS_5PP`.
6. **Time gate.** Compute the median pair-level Flake total and median pair-level Baseline total (section 16) over valid pairs. If `median_flake_total > 0.80 * median_baseline_total`, route to `FAIL: TIME_REDUCTION_BELOW_20PCT`.
7. Otherwise, route to `PASS`.

Every routing decision is a pure function of the sealed thresholds and the raw manifest — there is no researcher-discretion step, no "judgment call," and no threshold that can move after step 1 has been evaluated. `verify_independent.py` recomputes the same route from the same raw manifest using an independently written implementation (never importing `analysis.py`); the two must agree exactly, or the run is not reportable until the discrepancy is root-caused and fixed under section 15's repair rule if it requires touching sealed code.

Descriptive reporting (participant-clustered intervals, per-tier breakdowns) accompanies the routing decision per Section 26, but never substitutes for it and never overrides a `FAIL`/`INCONCLUSIVE` route with a narrative judgment.

## 21. Missing-data and protocol-deviation handling — summary

Missing data and protocol drift are handled exactly as follows, decided in advance:

- A single missing required field (section 17.1) on an otherwise-recorded attempt makes that attempt `MISSING_REQUIRED_FIELD`-excluded (section 13.1); it is not guessed or backfilled.
- A logged `protocol_deviation` does not automatically exclude the attempt — the facilitator (or, on review, an independent reader of the evidence report) classifies it against the exhaustive list in section 13.1; if it matches none of those reasons, the attempt is retained as valid and the deviation is reported as a limitation, not hidden and not used to justify exclusion after the fact.
- If total exclusions plus missing-data attempts exceed the section 20 step-1 threshold, the whole confirmatory batch routes to `INCONCLUSIVE`, and the repair-repeat rule (section 15) governs whether and how the study continues.

## 22. What this protocol does not claim

This is a six-participant local investment gate, not a population-level efficacy trial. A `PASS` licenses proceeding to T03-08's actual execution and, if T03-08 also passes, to T04-01; it is not a claim that Flake outperforms every possible baseline, for every user, in general use (Section 26). No model or paid API is used anywhere in this protocol, its harness, or its analysis.
