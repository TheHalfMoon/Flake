# Fehrest V2 Plan Amendment — Tencent Source and Code Convergence — 2026-09-08

**Status:** PLAN AMENDMENT / NON-AUTHORIZING  
**Applies to:** future V2 proposal and any later canonicalization of PR #2  
**Execution effect:** NONE while R1 remains open  
**Canonical frontier:** live `specs/CURRENT.md`  
**Source qualification:** `docs/research/TENCENT_SOURCE_QUALIFICATION_2026-09-08.md`  
**Founder source-use record:** `docs/research/FOUNDER_SOURCE_USE_AUTHORIZATION_2026-09-08.md`  
**Reuse matrix:** `docs/research/SOURCE_REUSE_ADAPTATION_MATRIX_2026-09-08.md`  
**First gap review:** `docs/reviews/V2_TENCENT_SOURCE_GAP_REVIEW_2026-09-08.md`  
**Code-level gap review:** `docs/reviews/V2_TENCENT_CODE_LEVEL_GAP_REVIEW_2026-09-08.md`

> This amendment is the current Tencent-source convergence layer for the future Fehrest V2 proposal. It incorporates both the original capability-level review and a deeper source-code-level review. It does not alter the frozen R1-v2 review candidate, authorize sealing/execution, activate Spec 002, authorize product implementation, or authorize merging PR #2.

# 1. Planning verdict

The donor set is valuable precisely because Fehrest does **not** need to copy the donor architectures wholesale.

```text
USE DONORS TO SHARPEN CONTRACTS
COPY/PORT SMALL PROVEN MECHANISMS WHEN JUSTIFIED
DO NOT MULTIPLY PLATFORMS
```

Deep review changed the plan in three important ways:

1. the initial 14 source-driven gaps expanded to 28 after code-level review;
2. Founder direct source-use permission now makes selective copy/port/adaptation legitimate future options;
3. the original proposal overloaded future Spec 007, so read/context authority and delegated side-effect execution should become separate authoring responsibilities (`007A` and `007B`).

Current planning state:

```text
SOURCE_DRIVEN_GAPS_TOTAL=28
GAPS_WITH_OWNER_OR_AUTHORING_GATE=28/28
FOUNDER_SOURCE_USE_PERMISSION=RECORDED
NEW_CURRENT_IMPLEMENTATION_AUTHORITY=0
R1_CHANGED=NO
R1_REVIEW_CANDIDATE_CHANGED=NO
```

# 2. Program invariants added by this convergence

These must be carried into any future post-R1 V2 canonicalization unless superseded through the repository's required change-control path.

## P-TS-01 — Temporal rank is never temporal truth

```text
TEMPORAL_RANK != TEMPORAL_TRUTH
DERIVED_VOLATILITY != VALIDITY_INTERVAL
RERANKER_OUTPUT != SUPERSESSION
```

## P-TS-02 — Time precision and source time are preserved

```text
PARTIAL_TIME != EXACT_TIME
EVENT_TIME != OBSERVATION_TIME
RELATIVE_TIME_REQUIRES_REFERENCE_TIME
PARSED_TIME_MUST_RETAIN_SOURCE_EXPRESSION_AND_PRECISION
```

## P-TS-03 — Derived edits may not create shadow canonical state

```text
USER_DURABLE_EDIT -> CANONICAL_WRITER_OR_CANONICAL_PROPOSAL
DIRECT_DURABLE_EDIT_OF_DERIVED_PROJECTION=NO
```

## P-TS-04 — Memory promotion is type/risk sensitive

High-influence decisions, constraints, identity/profile claims, preferences and procedures cannot silently become active from model extraction.

## P-TS-05 — Retrieval forgetting is not canonical deletion

```text
LOW_RANK != DELETE
LOW_USAGE != DELETE
HIDDEN_FROM_DEFAULT_RECALL != LOST_HISTORY
```

## P-TS-06 — Context residency is explicit and receipted

Resident versus on-demand recalled memory is a compiler/policy decision with privacy, staleness, token and cost consequences recorded in receipts.

## P-TS-07 — Behavioral signals are influence, not user truth

Derived interest/affinity/recurrence signals may affect ranking only under explicit policy, isolation and explainability. They cannot author canonical memory.

## P-TS-08 — Proposal extraction is durable and replay-safe

Automatic proposal pipelines require durable cursors, idempotency, deduplication and no silent source-range loss.

## P-TS-09 — Read authority and side-effect execution authority are separate responsibilities

Context retrieval/compilation must not require an executor runtime. Delegated side effects require a separate admission/fencing/receipt contract.

## P-TS-10 — Remote resource liveness is never grant liveness

```text
SANDBOX_LIVE != GRANT_LIVE
PROVIDER_HANDLE != AUTHORITY
QUEUE_LEASE != CAPABILITY
```

## P-TS-11 — Every side effect is generation-fenced

Principal, grant, policy, image, skill revision, secret scope and execution attempt identities must be bound at execution time.

## P-TS-12 — Artifact output is untrusted until promoted

A sandbox/file-tool output is an evidence/output artifact, not canonical workspace state, until it passes explicit promotion through a canonical owner.

## P-TS-13 — Egress policy is connection-effective, not configuration prose

Domain/network rules require DNS/redirect/private-network handling and evidence at actual use time.

## P-TS-14 — Installer authority is separate from runtime authority

Skill/package installation may need broader permissions; those permissions never implicitly transfer to ordinary skill execution.

## P-TS-15 — Model-visible truncation is not audit truncation

Bound model output for resource safety, while preserving complete digests/byte counts and protected evidence sufficient for execution reconciliation.

## P-TS-16 — Held-out evaluation is structurally isolated

Private gold/test evidence is isolated through process/filesystem/capability boundaries, not prompt promises.

## P-TS-17 — Evaluation evidence has lineage and redaction

Every score and diagnostic binds exact split, evaluator, skill/model/runtime revision and redaction policy.

## P-TS-18 — Optimizer evidence cannot self-authorize change

```text
OPTIMIZER_IMPROVEMENT != MERGE_AUTHORITY
EVAL_PASS != PRODUCT_AUTHORITY
DECISION_HISTORY != ACTIVE_MEMORY
```

## P-TS-19 — Source reuse is permission-aware and provenance-complete

Founder permission enables copy/port/adaptation. Every source-derived unit still gets an exact reuse dossier and nested third-party review.

# 3. Future spec-map corrections

These are proposal corrections only.

## 003 — Derived Index and Lexical Retrieval Convergence

Add:

```text
PROJECTION_PROVENANCE_COMPLETE=YES
DERIVED_REBUILD_CANNOT_ERASE_DURABLE_USER_EDIT=YES
DERIVED_USER_EDIT_ROUNDTRIP_DEFINED=YES
```

Derived search/chunk/wiki projections never become shadow canonical stores.

## 004 — Derived Graph/Temporal Intelligence Capability Experiment

The experiment should ask a broader but still bounded question:

> Does additional derived graph/temporal intelligence materially improve continuation and temporal correctness over the strong deterministic Fehrest baseline at acceptable cost?

Comparator ladder may include, when relevant:

```text
canonical time semantics + lexical/structured retrieval
explicit as-of filter
optional vector baseline if independently authorized
Graphify / Graphiti / Code-Graph-RAG family where applicable
RoMem-style temporal reranking
```

Bind all learned/provider identity:

```text
model revision
embedding revision
checkpoint digest
seed
training manifest
fusion formula/alpha
runtime/dependency lock
```

No post-hoc parameter selection.

## 005 — Conditional Derived Intelligence Production Integration

If 004 retains an advanced capability, create only the smallest replaceable interface justified by measured evidence. Do not generalize a `DerivedIntelligenceProvider` merely because multiple donors exist.

## 006 — Temporal Memory Productization

Mandatory design areas now include:

```text
bitemporal/event-observation-record time model
partial/relative date precision
memory-kind taxonomy
influence/risk classification
promotion matrix
confirmation/corroboration
rejection tombstones
extraction cursor/idempotency
canonical retention vs recall/archive policy
resident/recalled metadata
revalidation/expiry
supersession/retraction
conflict/merge key semantics
current/as-of deterministic resolution
behavioral signal influence class
```

Minimum promotion matrix:

```text
MEMORY_KIND
INFLUENCE_LEVEL
EVIDENCE_REQUIREMENT
CORROBORATION_REQUIREMENT
HUMAN_CONFIRMATION_REQUIREMENT
REVALIDATION_RULE
SUPERSESSION_RULE
RETRACTION_RULE
```

False destructive merge/supersession is higher severity than leaving duplicates.

## 007 — authoring correction: split into 007A and 007B responsibilities

Do not renumber the whole future roadmap now. During post-R1 authoring/canonicalization, treat the current 007 proposal as two independently testable responsibilities.

### 007A — Universal Context and Memory Gateway

Owns:

```text
principal/session/grant read baseline
Context Compiler
scope/time filtering
budgeting
resident/on-demand context policy
context package
context receipt
SelectionTrace
read-only CLI/SDK/local API
credential-to-principal baseline mapping for read access
```

Context receipts should expose selection dispositions such as:

```text
RESIDENT
RECALLED_ON_DEMAND
EXCLUDED_STALE
EXCLUDED_SCOPE
EXCLUDED_BUDGET
EXCLUDED_CONFLICTED
EXCLUDED_POLICY
```

### 007B — Delegated Execution, Fencing and Receipt Foundation

Owns:

```text
side-effect capability admission
ExecutionAttemptId
principal/grant/policy/config generation fencing
ExecutorAudience
DurableDispatchIntent
provider/sandbox resource mapping
retry/idempotency semantics
cancellation ownership
TerminalReceipt
IndeterminateExecution
ReconciliationEvidence
just-in-time secret injection
network/resource policy envelope
artifact ingress/egress boundary
full-evidence digest vs model-visible output contract
```

A remote sandbox, worker lease, provider handle or queue item can never mint or extend authority.

## 008 — GitHub Link and IDE Discovery

Depends on 007A. It should not require delegated execution merely to discover/request authorized context.

## 009 — Trusted Vertical Memory Proof

Expand falsification cases:

```text
multi-step temporal contradiction
partial/ambiguous source time
false durable-memory proposal
rejected inference recurring
resident-memory overexposure
behavioral affinity over-steering
crash/replay proposal extraction
revoked grant while sandbox remains alive
policy generation change during live sandbox
provider timeout after possible side effect
artifact symlink/TOCTOU attack
editable-derived-state roundtrip
```

## 010/011 — Workspace canonical objects and user editing

Require explicit transitions between:

```text
GENERATED_DRAFT
DERIVED_REBUILDABLE_VIEW
CANONICAL_ANNOTATION_OR_PROPOSAL
USER_SAVED_CANONICAL_DOCUMENT
MEMORY_PROPOSAL
APPROVED_MEMORY
```

A successful durable user edit must identify which canonical mutation/proposal owns it.

## 013 — AI Provider Runtime and Ask Fehrest

Read-only Ask can consume 007A. Tool/sandbox/action execution requires 007B.

```text
MODEL_RUNTIME != AUTHORITY
SECRET_BYTES_NEVER_MODEL_VISIBLE
CLIENT_CANCELLATION != CONFIRMED_PROCESS_TERMINATION
FAILURE_STATUS != SAFE_RETRY
```

## 014 — External Evidence and WebMCP

Read-only acquisition owns source/snapshot/freshness semantics and may consume 007A. Side-effectful external actions require 007B.

Shared acquisition jobs need request/snapshot identity, waiter ownership and retry classification so one caller's cancellation cannot silently corrupt a shared fetch.

Network policy must cover DNS rebinding, redirects, loopback/link-local/private networks and proxy bypass according to the future threat model.

## 018 — Organization Identity, Policy and Admin

Add service/API credential mapping and lifecycle:

```text
external credential -> Fehrest principal mapping
credential rotation/revocation
service account lifecycle
organization policy extension
credential scope audit
```

External credential IDs never become Fehrest canonical identity.

## 021 — Extension, Automation and Connector Platform

Require an authoring-time decomposition check.

Potential split if independent testability/ownership demands it:

```text
021A = Extension / Connector / Automation Runtime
021B = Skill Package / Evaluation / Evolution
```

### 021A candidate responsibilities

```text
extension manifest
capability grants
runtime/provider boundary
automation trigger/action contracts
007B execution integration
```

### 021B candidate responsibilities

```text
SkillPackage
SkillRevision
source/reuse provenance
install/build lifecycle
bundle digest
SBOM/component inventory
advisory/license scan
installer vs runtime grants
SkillEvalBinding
held-out split isolation
redacted observation surface
score provenance
release/rollback state
optimization/review lifecycle
```

Skill evolution law:

```text
candidate revision
-> isolated held-out evaluation
-> regression/security checks
-> review
-> authorized release
```

Never expose final-test gold during iteration and never copy donor example score thresholds as Fehrest thresholds without Fehrest-specific evidence.

# 4. Dependency correction

Future sequence remains mostly intact but 007 is no longer one overloaded block:

```text
R1 terminal truth
-> post-R1 core if authorized
-> 003 deterministic retrieval
-> 004 derived graph/temporal intelligence experiment
-> 005 only if retained and required
-> 006 temporal memory
-> 007A context gateway
-> 007B delegated execution foundation when side-effectful agents/tools are required
-> 008 GitHub/IDE discovery (007A)
-> 009 trusted vertical proof
-> broader workspace/search/AI/web/import
-> collaboration/team/mobile
-> 021 extension runtime and skill-evolution tranche
-> 022 hub/network
```

Important consequence:

```text
READ_ONLY_CONTEXT_DOES_NOT_DEPEND_ON_EXECUTOR_INFRA=YES
```

# 5. New benchmark families

Keep separate evidence families rather than one composite product score.

## B-TIME — temporal semantics

```text
partial-date preservation
relative-reference correctness
timezone/locale correctness
event-vs-observation time
current truth
as-of truth
```

## B-TEMP-RANK — advanced temporal retrieval

```text
strong explicit-time baseline
RoMem-style/graph-derived candidate
contradiction accuracy
stale-use rate
recall/precision
latency/tokens/cost/footprint
```

## B-MEM-PROMOTE — memory promotion safety

```text
false promotion
missed useful promotion
high-influence error
review burden
retraction/revalidation
rejection recurrence
```

## B-MEM-PIPE — proposal pipeline durability

```text
crash before proposal commit
crash after proposal before cursor advance
replay/duplicate source range
concurrent extraction
rejected-guess replay
```

## B-CONTEXT — residency and influence

```text
resident vs recalled quality
privacy exposure
stale exposure
behavioral affinity steering
scope isolation
tokens/latency/cost
```

## B-EXEC — delegated execution safety

```text
crash after dispatch intent
provider timeout after side effect
duplicate worker retry
revocation while resource lives
policy/config generation change
client cancellation without process termination
credential rotation
indeterminate execution reconciliation
```

## B-SANDBOX — sandbox boundary

```text
intermediate symlink
TOCTOU path swap
output escape
resource exhaustion
DNS rebinding
redirect to private network
secret persistence
installer/runtime privilege separation
hard lifetime expiry
```

## B-SKILL-EVOLVE — skill optimization

```text
prompt-only change vs whole-skill change
held-out leakage
regression rate
security violations
score provenance completeness
custom-validator drift
rollback success
review burden
```

# 6. Source reuse strategy

Founder permission changes the option set from `study-only` to selective code reuse.

## RoMem

```text
CURRENT=STUDY+BENCHMARK+CODE_PORT_CANDIDATE
PREFERRED=TEST_VECTOR/PORT
```

Highest-value candidates: temporal parser cases, temporal scoring formula/reference, benchmark harness patterns. Keep a Python/KGE runtime conditional on actual 004 evidence.

## WeKnora

```text
CURRENT=STUDY+BENCHMARK+COPY/PORT/ADAPT_CANDIDATE
```

Highest-value candidates: memory confirmation/rejection tests, extraction cursor/dedup patterns, subject isolation, sandbox lifecycle, separate installer/runtime capability, bundle digest/activation/rollback, persistent web-fetch snapshot lifecycle.

Do not inherit weak/default security assumptions merely because their code is reusable.

## SkillHone

```text
CURRENT=STUDY+BENCHMARK+COPY/ADAPT_CANDIDATE
```

Highest-value candidates: evaluation split isolation, redaction/observation tooling, score provenance, trajectory diagnosis and focused Git-native optimization loops.

# 7. Source reuse dossier is mandatory

For each copied/ported/adapted unit record the fields defined by `SOURCE_REUSE_ADAPTATION_MATRIX_2026-09-08.md`, including exact upstream path/commit, reuse mode, rights basis, notices, transformation, owner spec, security/benchmark/test evidence and authority/dependency effects.

# 8. Gap ownership closure

First-pass gaps remain `TG-01..TG-14`. Code-level gaps are `TG-15..TG-28`.

```text
TG_01_TEMPORAL_RANK_TRUTH=OWNED
TG_02_RELATION_VOLATILITY=OWNED
TG_03_PROMOTION_POLICY=OWNED
TG_04_CONTEXT_RESIDENCY=OWNED
TG_05_DERIVED_EDIT_SHADOW_STATE=OWNED
TG_06_EXECUTION_ADMISSION_RECEIPT=OWNED_BY_007B_PROPOSAL
TG_07_EXTERNAL_PRINCIPAL_MAPPING=OWNED
TG_08_QUEUE_IDEMPOTENCY_FENCING=OWNED_BY_007B_PROPOSAL
TG_09_SKILL_ARTIFACT_LIFECYCLE=OWNED_BY_021_AUTHORING_GATE
TG_10_HELD_OUT_EVAL_ISOLATION=OWNED_BY_021B_PROPOSAL
TG_11_DECISION_HISTORY_VS_MEMORY=OWNED
TG_12_GENERATED_WIKI_AUTHORITY=OWNED
TG_13_PERSISTENT_SANDBOX_REVOCATION=OWNED_BY_007B_PROPOSAL
TG_14_SOURCE_ADMISSION_RECORD=OWNED
TG_15_BITEMPORAL_PRECISION=OWNED
TG_16_TEMPORAL_MODEL_IDENTITY=OWNED
TG_17_RETENTION_VS_FORGETTING=OWNED
TG_18_PROPOSAL_PIPELINE_IDEMPOTENCY=OWNED
TG_19_DERIVED_AFFINITY_INFLUENCE=OWNED
TG_20_SUPERSESSION_KEY_COLLISION=OWNED
TG_21_REMOTE_RESOURCE_BINDING_AUTHORITY=OWNED_BY_007B_PROPOSAL
TG_22_POLICY_GENERATION_FENCING=OWNED_BY_007B_PROPOSAL
TG_23_ARTIFACT_PROMOTION_BOUNDARY=OWNED
TG_24_EGRESS_DNS_SSRF=OWNED
TG_25_SKILL_INSTALL_SUPPLY_CHAIN=OWNED_BY_021/007B
TG_26_AUDIT_EVIDENCE_TRUNCATION=OWNED_BY_007B_PROPOSAL
TG_27_EVAL_REDACTION_SCORE_LINEAGE=OWNED_BY_021B_PROPOSAL
TG_28_OPTIMIZER_ATTRIBUTION_CONTAMINATION=OWNED_BY_021B_PROPOSAL
```

# 9. PR #2 reconciliation requirement

PR #2 remains Draft / non-authorizing. Before it can ever become a canonical V2 program after R1, it must reconcile:

```text
REAL_R1_TERMINAL_OUTCOME
LIVE_MAIN
TG_01_TO_TG_28
007A_007B_DECOMPOSITION_DECISION
021A_021B_AUTHORING_DECISION
FOUNDER_SOURCE_USE_AUTHORIZATION
SOURCE_REUSE_DOSSIER_STANDARD
CURRENT_ARCHITECTURE/SECURITY_INVARIANTS
```

No stale R1 claim or donor-derived implementation assumption may survive that reconciliation silently.

# 10. Current plan state

```text
TENCENT_SOURCE_PLAN_CONVERGENCE=DEEP_REVIEW_COMPLETE
SOURCE_DRIVEN_GAPS=28
GAPS_WITH_OWNER_OR_AUTHORING_GATE=28/28
FOUNDER_SOURCE_CODE_USE_PERMISSION=YES
CODE_REUSE_OPTIONS=EXPANDED
CURRENT_CODE_IMPORT=NO
CURRENT_RUNTIME_DEPENDENCY_ADMISSION=NO
PROPOSED_007A_007B_SPLIT=YES_NON_AUTHORIZING
PROPOSED_021A_021B_AUTHORING_GATE=YES
R1_CHANGED=NO
R1_REVIEW_CANDIDATE_CHANGED=NO
SPEC_002_ACTIVATED=NO
PRODUCT_IMPLEMENTATION_AUTHORIZED=NO
PR_2_MERGE_AUTHORIZED=NO
```
