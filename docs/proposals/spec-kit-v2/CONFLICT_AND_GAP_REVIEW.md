# Fehrest V2 Spec Kit Conflict and Gap Review

**Status:** REVIEW / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Reviewed inputs:** live `AGENTS.md`, live `specs/CURRENT.md`, canonical Execution Master Plan, Spec 002, V2 vision/UX/catalog/proposals, current upstream Spec Kit 1.0 workflow/templates.

> This review checks the proposed V2 program for internal contradictions, hidden ownership conflicts, missing prerequisites and sequencing errors. It does not close R1, publish the original historical Git object graph, or activate future work. Historical governance/implementation inputs have since been recovered and conservatively reconciled; that evidence creates no implementation authority.

---

## 1. Review verdict

The previous V2 proposal had the correct product direction, but it still contained several sequencing and ownership ambiguities that would become implementation conflicts if converted directly into executable specs.

The Spec Kit V2 program blueprint resolves the known structural conflicts identified below.

Current review status:

```text
KNOWN_STRUCTURAL_CONFLICTS_IDENTIFIED=12
KNOWN_STRUCTURAL_CONFLICTS_WITH_PROPOSED_RESOLUTION=12
KNOWN_CRITICAL_PRODUCT_GAPS_WITH_OWNER_OR_GATE=26/26
HISTORICAL_GOVERNANCE_RECONCILIATION=RECONCILED_CONSERVATIVELY
HISTORICAL_IMPLEMENTATION_EVIDENCE=RECOVERED_AND_RECONCILED
GITHUB_ORIGINAL_HISTORICAL_OBJECT_GRAPH=NOT_YET_PUBLISHED
CURRENT_R1_GATE_RESOLVED=NO
PROGRAM_AUTHORIZED=NO
```

---

# 2. Conflict C-01 — Broad V2 vision versus one active frontier

### Problem

The product vision spans many categories, while repository governance requires exactly one active frontier and at most one active implementation Spec Kit.

### Risk

Creating one giant V2 spec or activating several specs would violate governance and produce coupled tasks that cannot be independently tested.

### Resolution

```text
V2 vision = program/assessment layer
execution = narrow feature Spec Kits
specs/CURRENT.md = single active pointer
```

Future specs may be documented as plans but remain non-authorizing.

**Status:** RESOLVED IN PROGRAM DESIGN

---

# 3. Conflict C-02 — Historical Constitution/Architecture Freeze reconciliation

### Original problem

At V2 planning start, the GitHub bootstrap mirror did not contain the full historical Architecture Freeze/Constitution source set. `AGENTS.md` correctly forbids reconstructing missing historical sources from memory.

### Live reconciliation

The historical Architecture Freeze / Constitution source set has since been recovered and conservatively reconciled against operational bootstrap governance. The sealed implementation tree and R1 evidence identities were also recovered and independently verified.

Current status:

```text
G-CONST=RECONCILED_CONSERVATIVELY
G-PROV=HISTORICAL_SOURCE_AND_SEAL_RECOVERED
G-PROV-GITHUB-ORIGINAL-HISTORY=NOT_YET_PUBLISHED
```

### Risk retained

A fresh V2 constitution or bootstrap convenience must still never replace, weaken, or silently reinterpret the recovered historical frozen invariants.

### Resolution

Keep `G-CONST` as the explicit historical-architecture input boundary for any future V2 canonicalization. Treat the recovered sources as authoritative historical inputs; do not regenerate them from V2 vision. Track original Git object-graph publication separately until its provenance closure criteria are met.

**Status:** RECONCILED CONSERVATIVELY / NO AUTHORIZATION EFFECT

---

# 4. Conflict C-03 — Graph product versus graph visualization

### Problem

The product requires an Obsidian-class graph experience, while the existing architecture correctly treats production graph intelligence as an optional benchmarked derived capability.

### Risk

A UI requirement could accidentally force Graphify/Graphiti/another graph backend into production even if the graph experiment rejects it.

### Resolution

Separate:

```text
004/005 = derived Graph Intelligence capability/provider
010      = explicit canonical object relationships
012      = graph visualization/navigation UX
```

An explicit-link graph UI can exist with no derived graph provider.

**Status:** RESOLVED

---

# 5. Conflict C-04 — Context Gateway versus AI provider runtime

### Problem

The gateway and AI experience were conceptually adjacent and could be merged accidentally.

### Risk

Core context authorization becomes coupled to a model vendor/runtime, violating `AI OFF` completeness and making arbitrary external agents harder to support.

### Resolution

```text
007 = Universal Context and Memory Gateway
013 = AI Provider Runtime and Ask Fehrest
```

007 produces authorized context/receipts independently of model execution. 013 consumes 007.

**Status:** RESOLVED

---

# 6. Conflict C-05 — WebMCP versus general web/tool authority

### Problem

WebMCP is an attractive structured-tool interface, but its existence could be mistaken for permission to expose arbitrary browser/web actions to an LLM.

### Risk

Prompt injection, domain escape, secret leakage, or action escalation.

### Resolution

Create 014 as a separate external-evidence/tool spec after grant and AI/tool boundaries exist.

Hard separation:

```text
WebMCP = provider candidate
Web authorization = Fehrest-owned
Tool description = untrusted content
Action authority = explicit grant/confirmation
```

**Status:** RESOLVED

---

# 7. Conflict C-06 — Team communication before organization authorization

### Problem

The earlier V2 sequence placed team communication before organization/security/admin hardening.

### Risk

Channels/DMs/guests/shared docs would create confidentiality boundaries before roles, membership, guest scope and audit semantics were defined.

### Resolution

Reorder:

```text
016 collaboration experiment
-> 017 production sync/multi-writer
-> 018 organization identity/policy/admin
-> 019 team communication/shared workspace
```

**Status:** RESOLVED

---

# 8. Conflict C-07 — Current single-writer core versus multi-user local-first collaboration

### Problem

The current core intentionally has one-writer-per-vault semantics. The V2 team thesis requires multiple devices/users and offline collaboration.

### Risk

Silently replacing writer semantics with a CRDT/sync layer could invalidate durability, recovery, provenance and authorization assumptions.

### Resolution

Do not treat collaboration as a library choice.

```text
016 = capability experiment
017 = production substrate only if retained
```

017 must explicitly state how the proven collaboration mechanism composes with or evolves writer ownership, event history, recovery and grants.

**Status:** OWNED; REQUIRES FUTURE CLASS C/D/E DECISION

---

# 9. Conflict C-08 — Memory semantics versus workspace Decision/Task objects

### Problem

The V2 product contains `Memory`, `Decision`, `Task`, `Project`, notes and messages. A naive object model could duplicate lifecycle/provenance semantics across them.

### Risk

A Decision object and a Memory object could disagree about what is currently true or superseded.

### Resolution

```text
006 owns durable Memory lifecycle/temporal truth/Memory Proposal
010 owns workspace object schemas and references
```

`Decision` may be a workspace object that references/participates in 006 memory/provenance semantics, but 010 cannot redefine memory state transitions.

Exact mapping remains a 010/006 compatibility decision.

**Status:** OWNERSHIP RESOLVED; DATA-MODEL DETAIL DEFERRED

---

# 10. Conflict C-09 — Search engine versus Search UX versus Ask

### Problem

Search appears in several product layers.

### Risk

Multiple indexes/query semantics or hidden duplicate retrieval pipelines.

### Resolution

```text
003 = lexical/structured derived retrieval engine
007 = context selection/authorization/trace
012 = human Search/Graph/Bases UX
013 = model-based Ask consuming authorized context
```

All higher layers consume lower contracts rather than building independent search stores.

**Status:** RESOLVED

---

# 11. Conflict C-10 — Import/export versus canonical format ownership

### Problem

The importer cannot define destination canonical schema, but workspace specs cannot embed every source-specific migration rule.

### Risk

Importers create source-specific shadow schemas or destructive transformations.

### Resolution

```text
010 = canonical workspace/open-format model
015 = importer/mapping/batch-provenance contract
```

The final structured export contract needs one explicit owner during 010/015 planning; until decided, it is a named review point rather than being duplicated.

**Status:** MOSTLY RESOLVED; SINGLE EXPORT OWNER TO BE DECIDED IN AUTHORIZED PLAN

---

# 12. Conflict C-11 — GitHub integration versus Fehrest authorization

### Problem

GitHub repo identity, issue identity or GitHub login could be mistaken for Fehrest authority.

### Risk

Repo-local metadata or external project membership widens memory access.

### Resolution

```text
008 owns GitHub mapping/discovery
007 owns Fehrest grant/context authorization
```

A `.fehrest/link.toml`-style artifact is discovery metadata only:

```text
secret = no
grant = no
authority override = no
```

**Status:** RESOLVED

---

# 13. Conflict C-12 — “All-in-one” scope versus no-core specialized infrastructure

### Problem

The product goal includes chat, AI, web research, mobile and rich workspace behavior. It would be easy to interpret this as a requirement to build every underlying infrastructure platform.

### Risk

Fehrest becomes a collection of custom media servers, sandboxes, graph databases, model runtimes and sync stacks rather than a coherent memory product.

### Resolution

Program decomposition distinguishes product surface from infrastructure ownership.

Examples:

```text
voice/video media infra       = integrate/provider, not core by default
sandbox execution             = integrate/provider, not core by default
LLM inference runtime         = provider, not canonical core
graph/vector engines          = derived provider candidates
WebMCP/browser                = provider candidates
```

Every adoption passes Ponytail + provenance/rights + security + benchmark where material.

**Status:** RESOLVED AS PROGRAM INVARIANT

---

# 14. Gap review beyond the existing 26 gaps

The Spec Kit review found additional planning-quality gaps that were not purely product capabilities.

## SG-01 — No formal requirement-to-test traceability rule

**Resolution:** added to `PROGRAM_BLUEPRINT.md`.

Required closeout:

```text
ORPHAN_REQUIREMENTS=0
ORPHAN_TASKS=0
UNVERIFIED_MUST_REQUIREMENTS=0
```

## SG-02 — No single semantic ownership registry

**Resolution:** added `CROSS_SPEC_INVARIANTS_AND_OWNERSHIP.md`.

## SG-03 — No explicit provider failure/offline acceptance catalog

**Resolution:** added mandatory scenario classes to the program blueprint.

## SG-04 — No explicit cross-spec contract versioning rule

**Resolution:** contracts now require owner/version/compatibility metadata.

## SG-05 — No program-wide data persistence classification

**Resolution:** added `CANONICAL / DERIVED_REBUILDABLE / CONFIGURATION / SECRET_REFERENCE / CACHE / EVIDENCE_ARTIFACT` classification.

## SG-06 — No explicit program stop checkpoints

**Resolution:** added strategic checkpoints after R1, retrieval, graph decision, memory, vertical proof, personal product, collaboration and team proof.

## SG-07 — No explicit coverage proof from P1-P15 to specs

**Resolution:** added `TRACEABILITY_AND_COVERAGE_MATRIX.md`.

## SG-08 — Future specs could select libraries inside product requirements

**Resolution:** technology choices are routed to `research.md`/`plan.md`, not `spec.md`, unless technology is itself a frozen/user-visible requirement.

## SG-09 — No explicit “unsupported decision ownership” rule

**Resolution:** unresolved architecture choices must have an owning future spec/gate; explicit unresolved state is preferable to guessing.

## SG-10 — No explicit convergence artifact requirement

**Resolution:** `verification.md` plus cross-artifact `CONVERGE` is mandatory at closeout.

---

# 15. Remaining genuine blockers / unresolved points

The planning system deliberately does not pretend open gates or future design choices are solved:

```text
R1 exact terminal result                         = unresolved
V2 founder post-R1 authorization                 = not yet effective
GitHub original historical object graph          = not yet published; separate provenance closure obligation
historical implementation/R1 evidence             = recovered and reconciled
historical Constitution/Architecture Freeze       = recovered and reconciled conservatively
production graph result                           = future experiment
exact memory lifecycle                            = future Spec 006
exact workspace object mapping                    = future Spec 010
editor technology                                 = future Spec 011 research/gate
AI provider implementation set                    = future Spec 013
WebMCP production API shape                       = future Spec 014 research
authorized collaboration mechanism               = future Spec 016
multi-writer evolution of writer semantics        = future Spec 017 + architecture review
E2EE/compliance/server-search trade-off            = future Spec 017/018 security/product decision
```

The recovered historical implementation and architecture inputs are no longer unresolved. The remaining open execution/authorization gates and future design choices each have an explicit owner or closure condition.

---

# 16. Final consistency verdict

The proposed Spec Kit V2 planning layer is internally consistent at the **program architecture** level if interpreted with the ownership/dependency documents and the live-base reconciliation record.

It is not yet canonical because R1 is open and the required post-R1 founder/architecture decision has not closed.

```text
PLAN_STRUCTURE=CONSISTENT_PROPOSAL
KNOWN_SEMANTIC_OVERLAP_WITHOUT_OWNER=0
KNOWN_CRITICAL_CAPABILITY_WITHOUT_OWNER=0
KNOWN_DEPENDENCY_CYCLE=0
KNOWN_AUTHORIZATION_BYPASS=0
CURRENT_EXECUTION_BLOCKERS_RESOLVED=NO
PROGRAM_CANONICAL=NO
IMPLEMENTATION_AUTHORIZED=NO
```
