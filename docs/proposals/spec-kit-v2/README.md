# Fehrest V2 Spec Kit Planning Control Index

**Status:** PROPOSAL CONTROL INDEX / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Updated:** 2026-08-31  
**Canonical authority:** unchanged; see `AGENTS.md`, `specs/CURRENT.md`, and `docs/canonical/EXECUTION_MASTER_PLAN.md`.

> The files in this directory refine the earlier V2 planning proposal into a Spec Kit-compatible program decomposition. Within the non-canonical V2 proposal branch only, if these files conflict with older V2 proposal sequencing or ownership language, this directory is the newer planning refinement. It does not supersede canonical repository governance.

---

## Read order

```text
1. PROGRAM_BLUEPRINT.md
2. RUST_PLATFORM_ARCHITECTURE.md
3. RUST_SPEC_TRACEABILITY_MATRIX.md
4. RUST_CONVERGENCE_REVIEW.md
5. CROSS_SPEC_INVARIANTS_AND_OWNERSHIP.md
6. SPEC_SEQUENCE_AND_DEPENDENCIES.md
7. LINEAR_ADDITIVE_PRODUCT_EXECUTION_TRACK.md
8. PRODUCT_MEASUREMENT_PRIVACY_AND_MIGRATION_EVIDENCE.md
9. TRACEABILITY_AND_COVERAGE_MATRIX.md
10. CONFLICT_AND_GAP_REVIEW.md
11. PROGRAM_CONVERGENCE_REVIEW.md
12. PRODUCT_EVIDENCE_HARDENING_REVIEW_2026-08-31.md
13. SPEC_AUTHORING_CHECKLIST.md
```

Then read the broader V2 product documents:

```text
docs/product/FOUNDER_PRODUCT_VISION_V2.md
docs/product/FOUNDER_DIRECTION_ADDENDUM_LINEAR_2026-08-31.md
docs/product/UX_BLUEPRINT_V2.md
docs/product/HUMAN_AGENT_FEATURE_CATALOG_V2.md
docs/research/COMPETITIVE_CAPABILITY_MATRIX_2026-08-28.md
docs/research/LINEAR_CAPABILITY_BASELINE_2026-08-31.md
docs/reviews/PRODUCT_GAP_REVIEW_V2_2026-08-28.md
docs/proposals/AI_SEARCH_WEBMCP_PROVIDER_ARCHITECTURE.md
docs/proposals/EXECUTION_MASTER_PLAN_V2_PROPOSAL.md
```

---

## What this refinement changes versus the older V2 proposal

### 1. Formal Spec Kit program layer

V2 is treated as a program/assessment, not one giant feature spec.

### 2. Rust-first platform direction

Founder technical direction is explicit and traceable across every proposed future spec:

```text
RUST_PRIMARY_PRODUCT_LANGUAGE=YES
RUST_OWNS_CANONICAL_SEMANTICS=YES
RUST_OWNS_SECURITY_AND_AUTHORIZATION=YES
RUST_OWNS_MEMORY_SEARCH_SYNC_GATEWAYS=YES
NON_RUST_INTEROP=THIN_ONLY_WHEN_JUSTIFIED
```

`RUST_SPEC_TRACEABILITY_MATRIX.md` maps Specs 002–022 to their Rust-owned semantics, allowed adapter boundaries and required evidence. `RUST_CONVERGENCE_REVIEW.md` checks the program for language/authority leakage.

UI, editor, sync, search and provider libraries remain subject to future Spec Kit research/benchmark gates. Rust-first does not pre-authorize a framework or dependency.

### 3. Constitution reconciliation gate

Missing historical Constitution/Architecture Freeze sources are treated as a hard reconciliation gate, not reconstructed from memory.

### 4. Single semantic ownership

Every durable entity/lifecycle/contract/authorization responsibility gets one owning future spec.

### 5. Graph separation

```text
Graph Intelligence experiment/provider != Graph visualization UX
```

A useful Obsidian-style graph interface may exist over explicit canonical links even when derived graph intelligence is rejected/deferred.

### 6. Gateway/AI separation

```text
007 Universal Context/Memory Gateway
!=
013 AI Provider Runtime/Ask Fehrest
```

Core context/authorization remains model-independent.

### 7. Organization before team communication

The refined order is:

```text
collaboration experiment
-> production sync/multi-writer substrate
-> organization identity/policy/admin
-> channels/topics/DMs/shared team workspace
```

This removes a security dependency inversion in the earlier proposal.

### 8. Workspace object foundation before UI

Canonical Note/Task/Project/Space/open-format semantics are specified before desktop presentation owns them accidentally.

### 9. WebMCP separated into an external-evidence/tool spec

WebMCP remains a provider candidate under Fehrest-owned authorization/origin/prompt-injection controls.

### 10. Traceability and converge requirements

Every MUST requirement must trace through acceptance/test/tasks/evidence and close with no orphan requirements or tasks.

### 11. Rust language gate

Every future executable Spec Kit must declare:

```text
PRIMARY_LANGUAGE_RUST=YES
SEMANTIC_AUTHORITY_OUTSIDE_RUST=NO
UNJUSTIFIED_NON_RUST_PRODUCT_LOGIC=0
UNDECLARED_FFI_BOUNDARIES=0
```

A blocked language gate prevents implementation unless the founder/architecture governance explicitly changes the Rust direction.

### 12. Program-level convergence verdict

`PROGRAM_CONVERGENCE_REVIEW.md` records one consolidated planning verdict across governance, Spec Kit methodology, dependency order, semantic ownership, Rust, human/agent UX, GitHub, Search/Graph, AI, WebMCP, collaboration and donor discipline.

### 13. Linear added as a mandatory product-execution benchmark

The founder clarified on 2026-08-31 that Linear must be **added to**, not substituted for, the existing Fehrest V2 direction.

The additive rule is:

```text
EXISTING_FEHREST_V2_SCOPE=RETAIN
LINEAR_CLASS_PRODUCT_EXECUTION=ADD
MEMORY_REPOSITORY_CORE=RETAIN
```

`LINEAR_ADDITIVE_PRODUCT_EXECUTION_TRACK.md` adds explicit planning ownership for Linear-class work management, initiatives, cycles, milestones, roadmaps, triage, internal requests, customer intelligence, views, insights, code review, agentic delivery, verification, automation, API/mobile requirements and replacement proof without deleting Specs 002–022 or the Obsidian/Notion/Slack/GitHub/agent-memory targets.

`docs/research/LINEAR_CAPABILITY_BASELINE_2026-08-31.md` records the dated capability baseline and requires a future living capability registry so material Linear features cannot remain implicitly unmapped.

This planning addendum is non-authorizing and does not alter R1 or `specs/CURRENT.md`.

### 14. Product measurement, privacy, and migration evidence hardening

`PRODUCT_MEASUREMENT_PRIVACY_AND_MIGRATION_EVIDENCE.md` adds a cross-program evidence contract for:

```text
activation and product-value metrics
time-to-value measurement
efficiency/adoption evidence
privacy-preserving and data-minimized telemetry
explicit telemetry consent/policy boundaries
migration source binding
field/object/relationship fidelity classes
zero silent critical migration loss
idempotency/repeatability
profile-scoped replacement claims
Linear migration/replacement proof hardening
```

It deliberately does not invent numerical adoption targets before a baseline exists and does not create a new analytics implementation owner. Remote analytics cannot require raw canonical content merely to measure adoption, and telemetry remains derived operational evidence rather than canonical memory or authority.

`PRODUCT_EVIDENCE_HARDENING_REVIEW_2026-08-31.md` records the targeted reassessment that led to these additions and confirms that a major architecture replacement is not justified by the identified planning gaps.

This evidence hardening is non-authorizing and does not alter R1, `specs/CURRENT.md`, or Spec 002.

---

## Current state

```text
V2_PROGRAM_REFINEMENT=PREPARED
FOUNDER_LANGUAGE_DIRECTION=RUST
RUST_TRACEABILITY_002_TO_022=YES
RUST_CONVERGENCE_REVIEW=PREPARED
PROGRAM_CONVERGENCE_REVIEW=PREPARED
PRODUCT_EVIDENCE_HARDENING_REVIEW=PREPARED
LINEAR_ADDITIVE_TRACK=PREPARED
LINEAR_MANDATORY_PRODUCT_EXECUTION_BASELINE=YES
LINEAR_REPLACES_EXISTING_V2_SCOPE=NO
PRODUCT_MEASUREMENT_CONTRACT=PREPARED
PRIVACY_PRESERVING_TELEMETRY_POLICY=PREPARED
MIGRATION_FIDELITY_ACCEPTANCE=PREPARED
TIME_TO_VALUE_MEASUREMENT=PREPARED
V2_PROGRAM_CANONICAL=NO
R1_CHANGED=NO
CURRENT_CHANGED=NO
SPEC_002_CHANGED=NO
IMPLEMENTATION_AUTHORIZED=NO
```
