# Fehrest V2 — Product Evidence Hardening Review

**Status:** PROGRAM REVIEW / NON-AUTHORIZING  
**Date:** 2026-08-31  
**Scope:** targeted planning hardening only  
**Canonical authority:** unchanged; live R1 and repository governance remain authoritative.

> This review reassesses the V2 proposal specifically for product/adoption KPIs, privacy-preserving telemetry, migration acceptance/fidelity, Linear migration/replacement proof, and time-to-value measurement. It does not replace the broader program convergence review and creates no implementation authority.

---

## 1. Inputs reviewed

```text
AGENTS.md
specs/CURRENT.md
docs/canonical/EXECUTION_MASTER_PLAN.md
docs/canonical/R1_REPLACEMENT_EXECUTION_RUNBOOK.md
docs/canonical/R1_REPLACEMENT_EXECUTION_RUNBOOK_V11.md
specs/002-post-r1-canonical-core-convergence/**
docs/product/FOUNDER_PRODUCT_VISION_V2.md
docs/product/FOUNDER_DIRECTION_ADDENDUM_LINEAR_2026-08-31.md
docs/proposals/spec-kit-v2/PROGRAM_BLUEPRINT.md
docs/proposals/spec-kit-v2/LINEAR_ADDITIVE_PRODUCT_EXECUTION_TRACK.md
docs/proposals/spec-kit-v2/CONFLICT_AND_GAP_REVIEW.md
docs/proposals/spec-kit-v2/PROGRAM_CONVERGENCE_REVIEW.md
docs/proposals/spec-kit-v2/PRODUCT_MEASUREMENT_PRIVACY_AND_MIGRATION_EVIDENCE.md
```

Live repository truth at review time still keeps R1 as the sole active execution frontier. No valid V11 replacement execution result is claimed by this review.

---

## 2. Review findings

### PEH-01 — Product/adoption KPI definitions were under-specified

**Previous state:** Product and Linear planning contained many capability and workflow acceptance dimensions, but no common contract required stable activation/adoption metric definitions with numerator, denominator, population, event boundaries, privacy class, retention, and decision use.

**Hardening:** `PRODUCT_MEASUREMENT_PRIVACY_AND_MIGRATION_EVIDENCE.md` now requires stable metric definitions and separates activation, time-to-value, continued value, and efficiency.

**Status:** PLANNING GAP CLOSED

### PEH-02 — Time-to-value was present as an outcome idea but not a program evidence contract

**Previous state:** Linear replacement proof included `time to outcome`, and onboarding emphasized immediate value, but no common rule required time-to-value evidence or evidence-backed target setting.

**Hardening:** The new contract defines candidate time-to-value metrics and requires future owning specs to derive numerical targets from a baseline, pilot, safety/correctness invariant, or comparable competitor evidence.

**Status:** PLANNING GAP CLOSED

### PEH-03 — Privacy-preserving telemetry policy was missing

**Previous state:** The V2 program required privacy/data-location and observability consideration, but it did not state what remote product analytics may collect or how analytics remains subordinate to local-first ownership.

**Hardening:** The new contract now requires data minimization, prefers local computation, prohibits raw canonical content from default remote product telemetry, requires explicit consent/organization-policy handling for non-essential remote telemetry, and keeps telemetry non-authoritative and non-essential to core correctness.

**Status:** PLANNING GAP CLOSED

### PEH-04 — Migration dimensions existed without a common acceptance taxonomy

**Previous state:** The product vision, gap review, Import Lab plan, and Linear track all required migration fidelity, but did not share one evidence taxonomy for exact/equivalent/transformed/unsupported/excluded/failed constructs.

**Hardening:** The new contract defines source snapshot binding, fidelity classes, field/object/relation reconciliation, idempotency/repeatability, rollback constraints, and hard zero-silent-critical-loss invariants.

**Status:** PLANNING GAP CLOSED

### PEH-05 — Linear replacement proof needed a stronger migration evidence boundary

**Previous state:** L-PX5/L-GA already required migration fidelity and no blocking migration gaps, but a future aggregate result could still be ambiguous without source binding, profile scope, operator-intervention evidence, and explicit unsupported-construct reporting.

**Hardening:** The new contract makes Linear replacement evidence profile-scoped and requires a source snapshot, capability profile, fidelity report, unsupported report, workflow evidence, time-to-value evidence, operator intervention count, information-loss count, and export/recovery result.

**Status:** PLANNING GAP CLOSED

---

## 3. What was deliberately not changed

No architecture replacement is justified by these findings.

```text
R1_SEMANTICS_CHANGED=NO
R1_ORDER_CHANGED=NO
R1_SCORING_AUTHORIZED=NO
R1_UNBLINDING_AUTHORIZED=NO
R1_POWER_ANALYSIS_AUTHORIZED=NO
R1_CONFIRMATORY_AUTHORIZED=NO
SPEC_002_ACTIVATED=NO
SPEC_002_TASKS_CHANGED=NO
CANONICAL_MAIN_CHANGED=NO
LINEAR_REPLACES_EXISTING_V2_SCOPE=NO
TELEMETRY_IMPLEMENTATION_SELECTED=NO
ANALYTICS_PROVIDER_SELECTED=NO
NUMERIC_ADOPTION_TARGETS_INVENTED=NO
```

The correct response was targeted planning hardening rather than a new architecture or implementation track.

---

## 4. Remaining evidence-dependent questions

The planning gaps above are closed only at the proposal level. Future executable specs still need empirical answers for:

```text
actual baseline activation rate
actual time-to-value distribution
actual migration fidelity by source/profile
actual operator intervention burden
actual human adoption/retention
actual agent task success and review burden
actual telemetry event set after privacy/security review
actual acceptable remote retention where remote telemetry is authorized
actual product parity/superiority thresholds
```

These values must be measured at their owning future gates. They are intentionally not invented here.

---

## 5. R1 and Spec 002 boundary re-check

The product evidence hardening does not satisfy any R1 empirical prerequisite.

```text
ACTIVE_EXECUTION_FRONTIER=R1
ACTIVE_R1_SUBGATE=REPLACEMENT_VARIANCE_PILOT_EXECUTION
R1_REPLACEMENT_EXECUTION_RESULT=NOT_PRESENT
SPEC_002_T037=COMPLETE
SPEC_002_T038=BLOCKED_BY_R1_TERMINAL_VERDICT
SPEC_002_T039=NOT_ELIGIBLE_BEFORE_T038
SPEC_002_T040=NOT_ELIGIBLE_BEFORE_T038_AND_T039
```

This review therefore does not authorize any Spec 002 implementation task.

---

## 6. Verdict

```text
TARGETED_REVIEW_SCOPE=PRODUCT_EVIDENCE_HARDENING
PRODUCT_ADOPTION_KPI_CONTRACT=PASS_AT_PLANNING_LEVEL
TIME_TO_VALUE_CONTRACT=PASS_AT_PLANNING_LEVEL
PRIVACY_PRESERVING_TELEMETRY_POLICY=PASS_AT_PLANNING_LEVEL
MIGRATION_FIDELITY_ACCEPTANCE=PASS_AT_PLANNING_LEVEL
LINEAR_MIGRATION_REPLACEMENT_PROOF=HARDENED_AT_PLANNING_LEVEL
MAJOR_ARCHITECTURE_REPLACEMENT_REQUIRED=NO
PROGRAM_CANONICAL=NO
IMPLEMENTATION_AUTHORIZED=NO
R1_GATE_EFFECT=NONE
```

The V2 proposal should remain draft and non-authorizing while R1 is open. After the R1 terminal route exists, these product-evidence contracts should be reconciled with the actual route before any V2 canonicalization decision.
