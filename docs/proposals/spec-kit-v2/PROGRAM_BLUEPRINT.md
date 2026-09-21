# Fehrest V2 Spec Kit Program Blueprint

**Status:** PROGRAM PROPOSAL / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Change class:** E product direction; downstream C/D architecture and security gates expected  
**Canonical frontier:** `specs/CURRENT.md`  
**Canonical execution plan:** `docs/canonical/EXECUTION_MASTER_PLAN.md`  
**Current active frontier:** R1  
**Spec Kit reference reviewed:** `github/spec-kit` main at `5aa8bea7823dcd056f111f847bf2d576bad3f0a5`

> This document defines a proposed Spec Kit planning system for the Fehrest V2 product direction. It does not activate any future feature, change R1, modify sealed benchmark semantics, authorize Spec 002, authorize UI/MCP/WebMCP/AI/graph/vector/sync work, or replace the canonical execution plan.

---

## 1. Purpose

The V2 founder direction is intentionally broad:

```text
local-first memory repository
+ personal notes/knowledge workspace
+ search/graph
+ AI/local LLM
+ GitHub/IDE memory integration
+ web evidence/WebMCP
+ collaboration/sync
+ team communication
+ organization/admin
+ mobile
+ extension ecosystem
+ future hub/network
```

That breadth must **not** become one implementation specification.

The program rule is:

```text
CATEGORY VISION = BROAD
ACTIVE SPEC     = NARROW
ONE ACTIVE FRONTIER
ONE OWNER PER SEMANTIC RESPONSIBILITY
```

The program exists to ensure the broad vision can be implemented without:

- overlapping ownership;
- circular dependencies;
- premature technology choices;
- hidden authorization widening;
- user-facing feature silos backed by conflicting canonical models;
- agent/provider/web content gaining authority;
- derived state becoming canonical;
- UI forcing core semantics;
- later collaboration invalidating earlier single-user assumptions;
- undocumented migration or compatibility breaks.

---

## 2. Spec Kit model used by Fehrest

Current upstream Spec Kit 1.0 uses a core flow centered on:

```text
constitution
-> specify
-> plan
-> tasks
-> implement
-> converge
```

and provides an optional idea assessment workflow:

```text
intake
-> research
-> define
-> shape
-> decide
```

Fehrest keeps that separation and extends it with its existing governance and evidence gates.

### 2.1 Fehrest program-level flow

For product-thesis or architecture-changing ideas:

```text
INTAKE
-> RESEARCH
-> DEFINE
-> SHAPE
-> DECIDE
-> AUTHORIZATION
-> SPEC KIT
```

The current V2 documents map approximately to this assessment layer:

| Assess stage | Existing V2 evidence |
|---|---|
| Intake | `docs/product/FOUNDER_PRODUCT_VISION_V2.md` |
| Research | `docs/research/COMPETITIVE_CAPABILITY_MATRIX_2026-08-28.md` |
| Define | `docs/reviews/PRODUCT_GAP_REVIEW_V2_2026-08-28.md` |
| Shape | `docs/product/UX_BLUEPRINT_V2.md`, `docs/product/HUMAN_AGENT_FEATURE_CATALOG_V2.md`, `docs/proposals/EXECUTION_MASTER_PLAN_V2_PROPOSAL.md` |
| Decide | **NOT YET CLOSED** — R1 terminal route + founder/product/architecture reconciliation required |

Therefore:

```text
V2_ASSESSMENT=SHAPED_NOT_DECIDED
V2_IMPLEMENTATION_AUTHORIZATION=NO
```

### 2.2 Fehrest feature-level flow

Every production feature follows the repository engineering method:

```text
SPEC
-> CLARIFY
-> PLAN
-> CHECKLIST
-> TASKS
-> ANALYZE
-> PONYTAIL NECESSITY GATE
-> IMPLEMENT
-> TEST
-> BENCHMARK (where required)
-> SECURITY
-> REVIEW
-> CONVERGE
```

The upstream Spec Kit artifacts remain useful, but Fehrest adds evidence and governance artifacts where required.

---

## 3. Constitution reconciliation gate

Upstream Spec Kit expects a project constitution before feature planning.

The current GitHub bootstrap mirror does **not** contain the full historical Architecture Freeze / Constitution set referenced by repository governance. `AGENTS.md` explicitly forbids reconstructing missing historical sources from memory.

Therefore V2 planning introduces a mandatory non-implementation gate:

```text
V2-G0A — CONSTITUTION / ARCHITECTURE RECONCILIATION
```

### 3.1 Before any V2 architecture-semantic spec becomes active

All must be true:

```text
HISTORICAL_GOVERNANCE_SOURCE_AVAILABLE=YES
HISTORICAL_GOVERNANCE_RECONCILED=YES
CURRENT_AGENTS_RULES_RECONCILED=YES
CANONICAL_INVARIANT_SET_RECORDED=YES
CONFLICTS_RESOLVED_THROUGH_REQUIRED_CHANGE_CLASS=YES
```

### 3.2 Until then

Use live repository governance conservatively:

```text
AGENTS.md
specs/CURRENT.md
GITHUB_BOOTSTRAP_PROVENANCE.md
canonical EXECUTION_MASTER_PLAN.md
active benchmark preregistration/evidence when present
```

but do not label this bootstrap subset a reconstructed historical constitution.

---

## 4. V2 decision gate

After R1 terminal routing, if continued investment is permitted, the founder/product/architecture decision must answer:

```text
Does the evidence justify continuing from the bounded memory/context thesis
into the universal local-first Memory Repository + Workspace direction?
```

Possible results:

```text
V2_GO
V2_GO_WITH_CONSTRAINTS
V2_LIMIT_TO_CORE
V2_NEEDS_MORE_EVIDENCE
V2_RETHINK
V2_STOP
```

A `V2_GO` result still does not activate every planned spec.

Each spec has its own entry gate.

---

## 5. Feature decomposition rules

A future Spec Kit is allowed only when it satisfies all of these decomposition rules.

### D-01 — One dominant user outcome

Every spec MUST state one dominant user or system outcome that can be independently demonstrated.

### D-02 — One semantic owner

A canonical entity, mutation rule, authorization rule, lifecycle transition or external contract MUST have one owning spec.

Later specs may extend it only through an explicit compatibility/change path.

### D-03 — Independent testability

Every P1 user story MUST have an independent test that demonstrates value without requiring unrelated future surfaces.

### D-04 — No hidden platform creation

A product feature MUST NOT silently create a new database, agent framework, sandbox, sync protocol, media server, graph store, vector store or plugin runtime unless that is the explicit scoped requirement and passes Ponytail/benchmark/security review.

### D-05 — Canonical vs derived ownership is explicit

Every persisted field/store/index MUST be labeled:

```text
CANONICAL
DERIVED_REBUILDABLE
CONFIGURATION
CACHE
EVIDENCE_ARTIFACT
SECRET_REFERENCE
```

### D-06 — UI does not own canonical semantics

UI specs may present or orchestrate existing core contracts. If a UI requires new canonical semantics, that requirement must be owned by a core/data spec first or explicitly included with the proper change class.

### D-07 — Security boundary is not deferred

Any spec that changes principals, grants, sync, external tools, model providers, web acquisition, plugins, organization boundaries or sharing requires a security artifact before implementation.

### D-08 — Migration exists before format change

Any change to canonical formats/schemas/contracts must define forward/backward compatibility, migration/upcast behavior and unsupported-version behavior before implementation.

### D-09 — Offline behavior is specified

Every human-facing feature MUST state what happens in:

```text
NETWORK=OFF
AI=OFF
SYNC=UNAVAILABLE
PROVIDER=UNAVAILABLE
```

where those dimensions apply.

### D-10 — Exit and kill criteria exist

Every experiment/provider/adoption spec MUST state what evidence causes:

```text
RETAIN
DEFER
REJECT
STOP
```

before the experiment is run.

---

## 6. Mandatory artifact profile for executable Fehrest specs

A production Spec Kit should normally contain:

```text
specs/<id>-<name>/
├── spec.md                 # WHAT / WHY / user stories / acceptance / success
├── clarifications.md       # resolved ambiguity and founder decisions
├── research.md             # donor/options/current evidence, pinned revisions
├── plan.md                 # HOW, architecture, technical context
├── data-model.md           # entities, ownership, lifecycle, persistence class
├── contracts/              # APIs/events/formats/tool schemas
├── quickstart.md           # executable/demo validation journey
├── dependencies.md         # hard/soft dependencies and forbidden implicit deps
├── checklist.md            # requirement completeness gate
├── tasks.md                # executable tasks, story mapping, exact paths
├── analyze.md              # cross-artifact consistency analysis
├── ponytail-gate.md        # build/reuse/adapt/defer decision
├── security.md             # required for C/D or new trust boundaries
├── benchmark.md            # required for performance/provider/adoption decisions
├── migration.md            # required when durable formats/contracts change
└── verification.md         # exact closeout evidence
```

Not every spec needs every optional file, but omission must be justified in `plan.md` or `checklist.md`.

### 6.1 Spec-first rule

`spec.md` must describe WHAT/WHY and technology-agnostic measurable outcomes.

Do not put selected libraries or implementation framework choices into the product requirements unless the technology itself is a user-visible/frozen constraint.

### 6.2 Research before dependency

Donor/provider/library choices belong in `research.md` and are resolved before implementation through:

```text
requirement
-> options
-> source revision/provenance
-> rights/license
-> security
-> benchmark where material
-> Ponytail necessity
-> decision
```

### 6.3 Plan constitutional check

Every `plan.md` must include a Constitution/Governance Check before detailed design and repeat it after data-model/contracts are created.

---

## 7. Requirement traceability contract

Every executable requirement gets a stable ID.

Example:

```text
FR-011
US2
AS-2.3
CT-SEARCH-04
T087
VT-SEARCH-04
```

Minimum trace chain:

```text
Founder/Product Goal
-> User Story
-> Functional / Non-functional Requirement
-> Acceptance Scenario
-> Data/Contract owner where applicable
-> Test / Benchmark
-> Task(s)
-> Verification Evidence
```

A spec cannot close with an orphan requirement or an implementation task that traces to no approved requirement.

### 7.1 Required closeout checks

```text
ORPHAN_REQUIREMENTS=0
ORPHAN_ACCEPTANCE_SCENARIOS=0
ORPHAN_CONTRACTS=0
ORPHAN_TASKS=0
UNVERIFIED_MUST_REQUIREMENTS=0
UNRESOLVED_NEEDS_CLARIFICATION=0
```

---

## 8. User-story rules

Upstream Spec Kit prioritizes independent user journeys. Fehrest adopts this strictly.

Each story must contain:

```text
priority
persona/principal
starting state
user intent
independent test
acceptance scenarios
failure/offline state
security/privacy expectations
measurable outcome
```

### 8.1 Example pattern

```text
US1 — Find a note instantly with no AI

Given a local Memory Repository with indexed notes,
When the user opens Search and enters a phrase,
Then matching permitted notes appear with explainable match context,
without network access or an AI provider.
```

### 8.2 Anti-pattern

Do not write a story such as:

```text
US1 — Build the search architecture
```

That is implementation work, not a user outcome.

---

## 9. Acceptance scenario classes

Every relevant spec must consider applicable scenarios from this catalog.

### Correctness

```text
normal success
empty state
invalid input
boundary size
partial failure
restart/recovery
version skew
```

### Local-first/offline

```text
network unavailable
sync unavailable
remote replica unavailable
local device resumes after long offline interval
```

### AI/provider

```text
AI OFF
local model unavailable
remote provider unavailable
provider capability mismatch
context limit exceeded
stream interrupted
tool calling unsupported
```

### Authorization/security

```text
unauthorized principal
revoked grant
expired session
scope downgrade
malicious retrieved content
prompt injection
secret-bearing input
cross-space access attempt
```

### Collaboration

```text
concurrent edits
offline writer reconnect
revoked offline writer reconnect
partial sync
conflict
schema/version mismatch
cross-tenant leak attempt
```

### External evidence/WebMCP

```text
untrusted tool description
origin changes
read tool returns malicious instruction
action requires confirmation
source disappears
source changes
provenance metadata incomplete
```

---

## 10. Cross-cutting non-functional requirements

Each future spec must explicitly state applicable targets or `N/A` for:

```text
performance
startup latency
search latency
memory footprint
disk footprint
sync bandwidth
battery/mobile cost
offline operation
accessibility
keyboard navigation
internationalization/text correctness
backup/recovery
exportability
privacy/data location
security/audit
observability
migration/backward compatibility
failure visibility
```

Do not use `N/A` without a one-line justification when the dimension is plausibly relevant.

---

## 11. Test and evidence pyramid

Fehrest uses the smallest test level that can prove each requirement, plus higher-level journey evidence where necessary.

```text
static/type/lint
unit
property/fuzz
contract
integration
fault injection
security/adversarial
performance/benchmark
e2e/user journey
native platform
replay/recovery
```

### 11.1 Tests are requirements evidence

For Fehrest, tests are not optional when a MUST requirement is implementation-verifiable.

If a requirement cannot be automatically tested, `verification.md` must state the exact manual/native/evidence procedure.

---

## 12. Converge gate

Current upstream Spec Kit includes a convergence step. Fehrest treats convergence as an evidence gate, not a prose review.

Before closeout:

```text
spec <-> plan
spec <-> data model
spec <-> contracts
spec <-> tasks
spec <-> tests
plan <-> implementation
contracts <-> implementation
migration <-> fixtures
security <-> adversarial tests
benchmark <-> raw evidence
```

All differences must be one of:

```text
RESOLVED
EXPLICITLY_DEFERRED_WITH_OWNER
REJECTED_WITH_RATIONALE
BLOCKER
```

No unexplained drift.

---

## 13. Program-level source of truth

The V2 planning hierarchy is proposed as:

```text
1. Live repository/evidence truth
2. Founder authorization records
3. Reconciled Constitution / Architecture Freeze / security invariants
4. Active benchmark preregistration
5. specs/CURRENT.md
6. Canonical Execution Master Plan
7. Active Spec Kit
8. V2 program blueprint / future spec map
9. Product vision / UX / feature catalog
10. Donor research and external comparisons
```

The V2 program documents may propose future changes but cannot silently outrank the canonical layers above them.

---

## 14. Program quality gates

The future V2 program is considered ready to become canonical only when:

```text
R1 terminal route recorded
founder V2 decision recorded
historical provenance reconciled
constitution/architecture sources reconciled
future spec ownership map has no semantic overlap
future spec dependency graph has no cycles
all P1-P15 product pillars trace to owning specs or explicit defer/reject
all critical gaps have an owning gate
security boundaries have owning specs
migration/compatibility ownership is explicit
provider/donor choices remain uncommitted until research gates
failure/kill routing exists
```

---

## 15. Current decision

```text
PROGRAM_BLUEPRINT=PREPARED
PROGRAM_CANONICAL=NO
R1_CHANGED=NO
CURRENT_CHANGED=NO
SPEC_002_CHANGED=NO
PRODUCT_IMPLEMENTATION_AUTHORIZED=NO
```
