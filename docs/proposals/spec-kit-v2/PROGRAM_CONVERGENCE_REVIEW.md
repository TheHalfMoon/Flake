# Fehrest V2 Program Convergence Review

**Status:** PROGRAM REVIEW / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Scope:** V2 product direction and Spec Kit program decomposition  
**Canonical authority:** unchanged; live R1 and repository governance remain authoritative.

> This is the final planning-level consistency review for the current V2 proposal branch. It does not claim the product is implemented, does not close R1, does not activate Spec 002, and does not make the V2 program canonical.

---

## 1. Executive verdict

The V2 planning package is structurally converged enough to serve as the input to a future post-R1 founder/architecture reconciliation.

It is **not executable today** because the live R1 terminal route is still open and the V2 founder/architecture decision is not closed. Historical governance/implementation sources have been recovered and conservatively reconciled; the unpublished original historical Git object graph remains a separate provenance closure obligation and does not create execution authority.

Planning verdict:

```text
PRODUCT_VISION_DEFINED=YES
UX_FROM_A_TO_Z_DEFINED=YES
HUMAN_FEATURE_SCOPE_MAPPED=YES
AGENT_FEATURE_SCOPE_MAPPED=YES
GITHUB_IDE_MEMORY_PATH_DEFINED=YES
SEARCH_GRAPH_UX_DIRECTION_DEFINED=YES
LOCAL_REMOTE_AI_DIRECTION_DEFINED=YES
WEBMCP_EXTERNAL_EVIDENCE_DIRECTION_DEFINED=YES
RUST_FOUNDER_DIRECTION_DEFINED=YES
SPEC_KIT_PROGRAM_DECOMPOSED=YES
SINGLE_SEMANTIC_OWNERSHIP_DEFINED=YES
PRODUCT_PILLARS_OWNED=15/15
CRITICAL_GAPS_OWNED_OR_GATED=26/26
KNOWN_STRUCTURAL_CONFLICTS_WITH_RESOLUTION=12/12
KNOWN_RUST_ARCHITECTURE_CONFLICTS_WITH_CONTROL=12/12
KNOWN_DEPENDENCY_CYCLES=0
KNOWN_CRITICAL_CAPABILITY_WITHOUT_OWNER=0
KNOWN_SEMANTIC_OVERLAP_WITHOUT_OWNER=0
RUST_LANGUAGE_GATE_DEFINED=YES
PROGRAM_CANONICAL=NO
IMPLEMENTATION_AUTHORIZED=NO
```

---

## 2. Governance convergence

### Preserved

```text
one active frontier
SPECIFIED != AUTHORIZED
R1 historical semantics immutable
no product behavior mutation while R1 is open
no UI/MCP/graph/vector/automatic-memory implementation while blocked
no force push/rebase/destructive history rewrite
canonical truth outranks derived/provider state
```

### V2 planning gate state

```text
G-R1    R1 terminal verdict                                    = OPEN
G-PROV  historical source and sealed implementation recovery   = RECOVERED
G-PROV-GITHUB-ORIGINAL-HISTORY                                 = NOT_YET_PUBLISHED
G-CONST Constitution/Architecture Freeze reconciliation        = RECONCILED_CONSERVATIVELY
G-V2    founder V2 product/architecture decision               = NOT_CANONICALIZED
```

No V2 architecture-semantic implementation may bypass the open R1 or founder/architecture authorization gates. Recovered provenance and governance inputs remain evidence, not implementation authorization.

**Verdict:** CONVERGED

---

## 3. Spec Kit methodology convergence

The program uses the current Spec Kit philosophy but keeps Fehrest's stricter evidence method.

Program/idea layer:

```text
INTAKE
-> RESEARCH
-> DEFINE
-> SHAPE
-> DECIDE
-> AUTHORIZATION
```

Feature layer:

```text
SPEC
-> CLARIFY
-> PLAN
-> CHECKLIST
-> TASKS
-> ANALYZE
-> PONYTAIL
-> IMPLEMENT
-> TEST
-> BENCHMARK where required
-> SECURITY
-> REVIEW
-> CONVERGE
```

Mandatory closeout properties include:

```text
ORPHAN_REQUIREMENTS=0
ORPHAN_ACCEPTANCE_SCENARIOS=0
ORPHAN_CONTRACTS=0
ORPHAN_TASKS=0
UNVERIFIED_MUST_REQUIREMENTS=0
UNEXPLAINED_SPEC_IMPLEMENTATION_DRIFT=0
```

**Verdict:** CONVERGED

---

## 4. Dependency-order convergence

Proposed future sequence:

```text
R1
-> reconciliation gates
-> 002 canonical core
-> 003 derived lexical retrieval
-> 004 graph intelligence experiment
-> 005 graph production only if RETAIN_NOW and required
-> 006 temporal memory
-> 007 universal context/memory gateway
-> 008 GitHub link + IDE discovery
-> 009 trusted vertical proof
-> 010 workspace canonical objects/open formats
-> 011 personal notes/docs/capture
-> 012 search/graph/bases UX
-> 013 AI provider runtime + Ask Fehrest
-> 014 external evidence/WebMCP
-> 015 import/migration
-> 016 collaboration experiment
-> 017 sync/multi-writer only if retained
-> 018 organization identity/policy/admin
-> 019 team communication/shared workspace
-> 020 mobile/offline capture
-> 021 extensions/automation/connectors
-> 022 Fehrest Hub/network
```

### Corrected dependency inversions

- organization authorization now precedes channels/DMs;
- workspace object semantics precede UI ownership;
- deterministic gateway precedes model runtime;
- gateway/model security boundaries precede WebMCP actions;
- collaboration experiment precedes production CRDT/sync adoption;
- graph visualization no longer depends on production graph intelligence;
- memory product identity is no longer forced to wait on optional graph production.

**Known dependency cycles:** 0

**Verdict:** CONVERGED AT PROGRAM LEVEL

---

## 5. Semantic ownership convergence

One owner exists for every critical durable semantic family.

```text
002 canonical persistence/recovery
003 lexical derived retrieval
004 graph experiment
005 optional graph provider
006 memory lifecycle/proposals/CI/temporal truth
007 grants/context compiler/receipts
008 GitHub binding/discovery
009 end-to-end proof
010 workspace canonical objects/open formats
011 personal workspace presentation/editor/capture
012 search/graph/bases presentation
013 AI provider/model execution
014 external evidence/WebMCP
015 import/migration
016 collaboration experiment
017 sync/multi-writer semantics
018 organization identity/policy
019 communication/shared workspace
020 mobile client
021 extension/automation capability platform
022 hosted Hub/network
```

Later specs may consume earlier contracts but may not silently redefine them.

**Known critical semantic overlap without owner:** 0

**Verdict:** CONVERGED

---

## 6. Rust-first convergence

Founder direction:

```text
FEHREST_PRIMARY_LANGUAGE=RUST
RUST_OWNS_CANONICAL_SEMANTICS=YES
RUST_OWNS_AUTHORIZATION=YES
RUST_OWNS_MEMORY_PROVENANCE=YES
RUST_OWNS_SEARCH_CONTEXT=YES
RUST_OWNS_SYNC_POLICY=YES
RUST_OWNS_AGENT_TOOL_GATEWAYS=YES
POLYGLOT_BUSINESS_LOGIC=NO
```

Non-Rust code is allowed only as a declared, typed, replaceable bridge/presentation/provider boundary where an active spec proves necessity.

Every executable spec must report:

```text
RUST_LANGUAGE_GATE=PASS|BLOCKED
```

Activation requires:

```text
PRIMARY_LANGUAGE_RUST=YES
SEMANTIC_AUTHORITY_OUTSIDE_RUST=NO
UNJUSTIFIED_NON_RUST_PRODUCT_LOGIC=0
UNDECLARED_FFI_BOUNDARIES=0
```

**Verdict:** CONVERGED AT PLANNING LEVEL

---

## 7. Human experience convergence

The UX blueprint covers the intended lifecycle:

```text
first run
-> local Memory Repository
-> optional import/GitHub/team
-> capture
-> notes/docs
-> search
-> graph
-> tasks/projects/decisions
-> optional AI
-> optional web research
-> collaboration/team
-> mobile
-> extensions
-> Hub
```

Critical product rule:

```text
ONE MEMORY
MANY SURFACES
```

Search and Graph are linked views. AI is optional. Local ownership remains explicit.

**Verdict:** COVERED

---

## 8. Agent experience convergence

An arbitrary authorized agent should eventually be able to:

```text
discover Fehrest
authenticate
request scoped task context
receive Context Package + Receipt
search permitted memory
inspect provenance/as-of state
use approved web/tools
perform work in its own runtime
return evidence/trajectory
submit Memory Proposal
handoff to another agent
```

It may not:

```text
receive whole repository by default
mint grants from content
promote inference automatically
bypass Rust authorization
use GitHub discovery metadata as permission
```

**Verdict:** COVERED

---

## 9. GitHub/IDE convergence

Flagship developer pairing:

```text
GitHub = code/work repository
Fehrest = memory/context repository
```

Target path:

```text
open GitHub repo in any compatible IDE/agent
-> discover non-secret Fehrest binding
-> authenticate
-> request scoped context
-> Rust gateway authorizes/compiles
-> return context + receipt
-> work
-> return evidence/proposal
```

Discovery metadata:

```text
CONTAINS_SECRET=NO
GRANTS_PERMISSION=NO
OVERRIDES_FEHREST_AUTHORIZATION=NO
```

**Verdict:** COVERED BY 007/008/009

---

## 10. Search/Graph convergence

Baseline search remains:

```text
LOCAL
AI_INDEPENDENT
NETWORK_INDEPENDENT
PERMISSION_AWARE
TRACEABLE
```

Graph is split into:

```text
explicit canonical relationships
optional derived graph intelligence
visual projection/navigation
```

Therefore an Obsidian-style useful graph does not require accepting a graph database/provider.

**Verdict:** CONVERGED

---

## 11. AI/provider convergence

Supported product modes remain:

```text
AI OFF
LOCAL AI
SELF-HOSTED/CUSTOM AI
CONNECTED AI
```

Rust owns the provider abstraction, request validation, authorized context assembly, tool validation, output classification and receipts.

The model/runtime itself remains replaceable.

```text
MODEL != MEMORY
PROVIDER != AUTHORITY
```

**Verdict:** CONVERGED

---

## 12. Web/WebMCP convergence

Web research is modeled as evidence acquisition under scope, not autonomous authority.

```text
USER/AGENT TASK
-> WEB AUTHORIZATION
-> DISCOVERY/TOOL
-> ACQUIRE
-> PROVENANCE
-> OPTIONAL MODEL SYNTHESIS
-> REVIEW
-> SAVE SOURCE/NOTE/PROPOSAL
```

Hard rules:

```text
WEB_CONTENT != INSTRUCTION
WEB_TOOL_DESCRIPTION != GRANT
UNKNOWN_TOOL = RESTRICTIVE
CONSEQUENTIAL_ACTION = EXPLICIT AUTHORIZATION
```

WebMCP remains a provider candidate rather than a core-format dependency.

**Verdict:** CONVERGED

---

## 13. Collaboration/team convergence

The plan does not assume a CRDT choice.

```text
016 experiment
-> RETAIN / DEFER / REJECT
```

Only `RETAIN` can make 017 eligible.

Organization/member/guest/policy semantics in 018 then precede private/shared communication in 019.

This prevents collaboration convenience from weakening confidentiality or local ownership.

**Verdict:** CONVERGED AT PROGRAM LEVEL

---

## 14. Donor/code-reuse convergence

Permission to copy/adapt donor code is treated as an advantage, not a reason to assemble uncontrolled dependencies.

Every adoption still requires:

```text
requirement
source repository/revision/paths
rights/license evidence
disposition
security
benchmark where material
Ponytail necessity
maintenance/update/exit strategy
```

For non-Rust donors that would otherwise own Fehrest semantics, default path is behavior/contract study followed by Rust port/adaptation with provenance.

**Verdict:** CONVERGED

---

## 15. Remaining blockers and unknowns

### Current gate state

```text
R1_TERMINAL_GATE=NOT_RECORDED_ON_LIVE_GITHUB
V2_FOUNDER_ARCHITECTURE_DECISION_CLOSED=NO
HISTORICAL_IMPLEMENTATION_EVIDENCE=RECOVERED_AND_RECONCILED
HISTORICAL_CONSTITUTION_ARCHITECTURE=RECONCILED_CONSERVATIVELY
GITHUB_ORIGINAL_HISTORICAL_OBJECT_GRAPH=NOT_YET_PUBLISHED
```

The first two lines are the current execution/authorization blockers for V2. The recovered historical implementation and Constitution/Architecture inputs are no longer missing. Original-history publication remains an open provenance/repository-closure obligation and must not be hidden or rewritten, but its absence does not turn recovered evidence back into `NOT_RECONCILED`.

### Intentionally unresolved implementation choices

```text
UI framework
rich editor implementation
search crate/provider
production graph provider
CRDT/sync mechanism
server framework/protocol details
mobile shell
AI provider adapters
WebMCP browser binding
extension sandbox/runtime
hosted infrastructure
```

Choosing these now would be premature and would weaken Spec Kit discipline.

---

## 16. Final program readiness verdict

```text
VISION_COHERENCE=PASS
UX_COVERAGE=PASS
FEATURE_COVERAGE=PASS
SPEC_DECOMPOSITION=PASS
DEPENDENCY_GRAPH=PASS
SEMANTIC_OWNERSHIP=PASS
RUST_DIRECTION=PASS_AT_PLANNING_LEVEL
SECURITY_ORDERING=PASS_AT_PLANNING_LEVEL
GITHUB_IDE_PATH=PASS_AT_PLANNING_LEVEL
AI_PROVIDER_SEPARATION=PASS_AT_PLANNING_LEVEL
WEBMCP_BOUNDARY=PASS_AT_PLANNING_LEVEL
COLLABORATION_ORDERING=PASS_AT_PLANNING_LEVEL
DONOR_DISCIPLINE=PASS_AT_PLANNING_LEVEL
CURRENT_AUTHORITY_GATE=BLOCKED
PROGRAM_MAY_BECOME_CANONICAL=NO
IMPLEMENTATION_MAY_BEGIN=NO
```

The correct next repository action is **not** to start Spec 003–022. The correct future action, after the live R1 route and required founder/architecture decision close, is to activate exactly one eligible Spec Kit through `specs/CURRENT.md`. Original-history publication must remain tracked separately until its own provenance closure criteria are met.
