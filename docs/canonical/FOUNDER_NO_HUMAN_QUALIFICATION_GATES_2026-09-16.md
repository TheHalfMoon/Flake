# Founder decision — remove external human qualification gates

Date: 2026-09-16
Decision class: E — product thesis / founder direction
Status: ADOPTED
Applies to: Flake v1 canonical execution after T03-07

## Authority

The founder explicitly directed: no human review gate; remove it and continue working.

Under `AGENTS.md`, a Class E change requires founder authorization plus architecture reconsideration. This document is the additive architecture-reconsideration record for that direction. It does not rewrite the canonical plan, historical evidence, T03-07 seals, or any accepted result.

The authority order in the canonical build plan places current explicit founder direction above the frozen planning document. This decision therefore prospectively supersedes only the human-participant / external-human-review qualification clauses named below.

## Scope boundary

This decision removes external human recruitment, participant, consent, voluntary-use, facilitator, and human-review requirements as blockers for Flake v1 execution and completion.

It does **not** remove product owner authority or human-in-the-loop product semantics. In particular, agent proposals still require owner review/acceptance where the product contract requires it; content still cannot mint authority; disclosure, provenance, durability, security, rights, native-platform, packaging, and independent-tool verification gates remain in force.

No synthetic or automated executor may be described as a human participant, user, reviewer, or adoption observation.

## Preserved historical evidence

`T03-07` remains COMPLETE exactly as merged. Its sealed preregistration package under `bench/flake-v1/T03-07/` remains immutable evidence of the earlier design. The confirmatory human trial was never run and must never be reported as run.

The T03-07 protocol, case bank, allocation, analysis, dry run, and seals may be reused only as technical fixtures where this decision explicitly allows it. Reuse does not convert synthetic or automated output into human evidence.

## Superseded clauses

Prospectively superseded for v1 completion:

1. Section 26 `P03 value gate before desktop` requirement for six human participants / 96 human attempts and equal human-review budgets.
2. T03-08 requirements that depend on participant outcomes, participant timing, or human maintenance-plus-resume time.
3. Section 26 `P05 adoption gate` requirement for eight consenting target users over ten workdays.
4. T05-06 requirements for voluntary human repeat use, consented participant observation, facilitator-free human completion, and human adoption evidence.
5. R11 wording requiring human product-value and voluntary-adoption evidence.
6. Section 39 access to consenting users as a completion blocker.
7. Any evidence-template wording that implies an independent human reviewer is mandatory. Independent executable/tooling checks remain required where the task contract calls for independence.

All other plan clauses remain authoritative unless a later explicit founder decision changes them.

## Replacement T03-08 contract — automated continuity qualification

T03-08 remains the same task ID and dependency position. Its replacement objective is to prove the bounded CLI continuity loop with real Flake Core paths and an independent maintained-Markdown baseline without making a human-benefit claim.

Required execution:

- Use the sealed T03-07 confirmatory case corpus as a fixed held-out engineering corpus. Do not alter answer keys, source-change/conflict facts, or case classes after observing results.
- Execute every case through the real Flake CLI/Core path. No scripted resolver may substitute for product behavior.
- Execute a separately implemented maintained Markdown + project index + task-list baseline against the same raw case inputs. The baseline must not import Flake Core, Flake parsers, Flake analysis code, or hidden answer keys.
- Start each arm from its declared initial state and preserve all setup/maintenance operations, failures, timeouts, bytes read, commands, and wall-clock measurements as engineering observations.
- Use independent raw-to-summary verification. The verifier must not import the producer implementation.
- Preserve every failed attempt and any repair repeat under a new additive seal; never rewrite T03-07 seals.

Automated P03 pass criteria:

- all 96 sealed case executions are accounted for in both arms;
- Flake successful-resume rate is at least 90%;
- Flake success is no more than five percentage points below the maintained-Markdown baseline;
- zero high-consequence planted conflicts are missed by Flake;
- zero unintended disclosures occur;
- every declared task class has at least one successful Flake case;
- section 27 Core performance/bounds gates applicable to this path pass;
- independent verification reproduces exact denominators and verdicts.

The former `>=20% lower median paired human maintenance-plus-resume time` criterion is removed because automated execution cannot establish human effort. Automated timings, operation counts, and maintenance bytes are still recorded, but they are descriptive engineering evidence only.

A T03-08 PASS unlocks T04-01. A FAIL follows the existing affected-slice reconsideration route. `Inconclusive` is reserved for invalid/missing technical evidence, not missing humans.

Allowed claim after PASS: the bounded CLI continuity behavior is technically qualified on the measured fixtures and environments.

Forbidden claim after PASS: proven user preference, reduced human effort, adoption, population superiority, retention, or market demand.

## Replacement T05-06 contract — automated long-horizon continuity soak

T05-06 remains the same task ID and dependency position. Replace the voluntary human adoption study with an automated, deterministic long-horizon continuity soak over disposable nonsecret workstreams.

Required execution:

- eight independent workstream lanes;
- ten simulated workdays per lane;
- at least six active days per lane;
- at least three later resumptions per lane;
- real capture -> evidence -> decision/action -> resume -> export paths;
- restart/close/reopen boundaries and derived-state rebuilds;
- at least one backup/restore or full export/import reconstruction per lane;
- independent final-state and history verification;
- all failures, maintenance operations, waits, and resource measurements preserved.

Automated T05-06 pass criteria:

- 8/8 lanes complete their sealed schedules;
- every lane satisfies the active-day and later-resumption minimums;
- every lane completes the full loop and independent exit/reconstruction;
- zero acknowledged canonical data loss;
- zero silent history loss, false recovery success, or unintended disclosure;
- all required native/profile claims remain bounded to actual technical evidence;
- independent verification reproduces the final state and run accounting.

No automated soak result may be called voluntary adoption, retention, user preference, or market evidence.

## R11 amendment

For Flake v1, R11 means:

`R11 automated continuity qualification and long-horizon repeat-use soak pass under this founder decision; no human adoption/value claim is required or permitted.`

R01-R10 and R12 are unchanged.

## Project-complete interpretation

Section 37 remains authoritative with this amendment: all 38 task IDs and six phases still require evidence-backed completion, and R01-R12 still require PASS, but T03-08, T05-06, and R11 use the replacement contracts above.

Project completion must explicitly state that v1 has **no human usability, adoption, retention, preference, or comparative-effort evidence**. Those questions are deferred rather than silently answered.

## Immediate frontier

The prior `T03-08-HUMAN-PARTICIPANTS` blocker is superseded by this founder decision.

After this decision is merged and all required checks pass:

`ACTIVE_IMPLEMENTATION_UNIT=T03-08`
`NEXT_DEPENDENCY_READY_UNIT=T03-08`
`EXECUTABLE_REPOSITORY_WORK=AVAILABLE`

Implementation must start from live `main`, preserve the T03-07 seals, create a new T03-08 evidence namespace, and continue through the canonical DAG without waiting for human recruitment.
