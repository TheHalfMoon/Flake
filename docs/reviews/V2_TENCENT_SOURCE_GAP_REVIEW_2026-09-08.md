# Fehrest V2 Tencent-Driven Gap Review — 2026-09-08

**Status:** REVIEW / NON-AUTHORIZING  
**Inputs:** current Fehrest V2 proposal set + qualified source review for Tencent/RoMem, Tencent/WeKnora and Tencent/SkillHone  
**Execution effect:** NONE  
**Canonical frontier:** unchanged; live `specs/CURRENT.md` wins  
**Superseding depth layer:** `V2_TENCENT_CODE_LEVEL_GAP_REVIEW_2026-09-08.md`

> This was the first capability-level source review. It identified `TG-01..TG-14`. A later code-level pass added `TG-15..TG-28` and refined ownership of side-effect execution from the earlier broad 007 assignment into the proposed `007A/007B` authoring split. This document remains preserved as the first-pass evidence; use the plan amendment and code-level review for the converged future plan.

## 1. Review verdict

The current V2 proposal already has strong phase decomposition, but the new sources expose contracts that were either implicit or split across several future specs.

This first pass did **not** recommend new top-level Spec IDs. The deeper review likewise avoids immediate roadmap renumbering, while adding an authoring-time decomposition for overloaded responsibilities.

```text
FIRST_PASS_GAPS_IDENTIFIED=14
FIRST_PASS_GAPS_WITH_OWNER_OR_EXPLICIT_GATE=14/14
DEEP_PASS_TOTAL_GAPS=28
DEEP_PASS_GAPS_WITH_OWNER_OR_AUTHORING_GATE=28/28
CURRENT_TOP_LEVEL_SPEC_RENUMBERING=NO
KNOWN_NEW_DEPENDENCY_CYCLES=0
R1_SEMANTICS_CHANGED=NO
R1_REVIEW_CANDIDATE_CHANGED=NO
IMPLEMENTATION_AUTHORIZED=NO
```

## 2. TG-01 — Temporal ranking can be mistaken for temporal truth

**Source trigger:** RoMem.

### Gap

The proposal states that temporal state is explicit and search rank is derived, but it does not explicitly prohibit a temporal reranker from deciding which fact is canonically current.

### Risk

A graph/vector/model-derived score could silently replace deterministic provenance-backed temporal resolution.

### Owner

```text
006 owns canonical temporal truth/resolution
004 owns temporal/graph capability experiment
005 may own a retained derived provider
007A may consume derived rank only after canonical scope/time filtering
```

### Required invariant

```text
TEMPORAL_RANK != TEMPORAL_TRUTH
DERIVED_VOLATILITY != CANONICAL_VALIDITY_INTERVAL
MODEL_TEMPORAL_SCORE != SUPERSESSION_EVENT
```

### Gate

Any temporal-ranking provider must be benchmarked against a simpler canonical-time filter + lexical/structured baseline before retention.

---

## 3. TG-02 — Relation volatility has no explicit persistence/authority class

**Source trigger:** RoMem semantic speed gate.

### Gap

A future relation-level volatility estimate could be useful, but the program does not say where such a value lives or whether it may influence lifecycle.

### Owner

004 experimental research; 005 derived provider if retained.

### Resolution

```text
RELATION_VOLATILITY_CLASS=DERIVED_REBUILDABLE
RELATION_VOLATILITY_CAN_AUTHOR_MEMORY=NO
RELATION_VOLATILITY_CAN_OVERRIDE_AS_OF=NO
```

If the signal adds no material outcome value, reject it and retain explicit temporal semantics only.

---

## 4. TG-03 — Memory promotion policy is too generic across memory types

**Source trigger:** WeKnora categories + confirmation stage.

### Gap

The proposed 006 Memory Proposal lifecycle is intentionally generic. It does not yet require different promotion rules for low-risk convenience memory versus high-influence decisions, constraints, preferences, identities or procedures.

### Risk

A single auto-promotion policy could turn a model extraction into durable authority.

### Owner

006.

### Required design rule

The authorized 006 spec must define a promotion matrix over at least:

```text
MEMORY_KIND
INFLUENCE_LEVEL
EVIDENCE_REQUIREMENT
CORROBORATION_REQUIREMENT
HUMAN_CONFIRMATION_REQUIREMENT
EXPIRY_OR_REVALIDATION_RULE
SUPERSESSION_RULE
RETRACTION_RULE
```

High-influence objects require explicit human confirmation unless a later founder/security decision authorizes a narrower deterministic rule.

---

## 5. TG-04 — Resident memory versus on-demand memory is not an explicit context policy

**Source trigger:** WeKnora long-term-memory retrieval behavior.

### Gap

Some memories may be consistently useful while others should only be retrieved for a relevant task. Treating both identically creates privacy, token and staleness problems.

### Owner

006 classifies memory semantics; 007A owns context residency/selection policy and receipts.

### Required rule

```text
MEMORY_CANONICAL_STATE != CONTEXT_RESIDENCY
RESIDENT_CONTEXT != ALL_ACTIVE_MEMORY
```

Context receipts should explain resident, recalled and excluded decisions.

---

## 6. TG-05 — Editable derived surfaces can create shadow canonical state

**Source trigger:** WeKnora document/chunk/wiki revision surfaces.

### Gap

A user may edit what appears to be a durable document while the underlying object is a retrieval chunk, generated wiki page or graph projection.

### Owner

003 projection semantics + 010 canonical object ownership + 011 editing UX.

### Required rule

```text
USER_DURABLE_EDIT -> CANONICAL_WRITER_OR_CANONICAL_PROPOSAL
DIRECT_DURABLE_EDIT_OF_DERIVED_PROJECTION=NO
```

---

## 7. TG-06 — General execution admission and receipt semantics need one owner

**Source trigger:** WeKnora sandbox/tool execution.

### First-pass finding

The original first pass assigned this to broad future 007.

### Deep-pass correction

The code-level review found that placing read-context compilation and side-effect execution in one semantic owner overloads 007. The converged plan proposes:

```text
007A = context/read gateway
007B = delegated execution/fencing/receipt foundation
```

### Owner

Proposed future 007B.

### Required primitives

```text
ExecutionAdmission
ExecutionAttemptId
FencingGeneration
ExecutorAudience
DurableDispatchIntent
TerminalReceipt
IndeterminateExecution
ReconciliationEvidence
```

Provider queue state does not own safe retry or side-effect truth.

---

## 8. TG-07 — External credential identity can be mistaken for Fehrest principal authority

**Source trigger:** WeKnora API/service key patterns.

### Gap

An API key ID, provider user ID or service account identifier can accidentally become the canonical principal.

### Owner

007A baseline principal mapping; 018 organization/service-account extension.

### Rule

```text
EXTERNAL_CREDENTIAL_ID != FEHREST_PRINCIPAL_ID
```

Rotation/revocation may change credentials without changing stable Fehrest identity.

---

## 9. TG-08 — Queue retry is not side-effect idempotency/fencing

**Source trigger:** WeKnora worker/task queue behavior.

### Gap

A queue can redeliver work, but cannot determine whether a prior external side effect happened.

### Owner

Proposed 007B.

### Required rule

```text
QUEUE_RETRY != SAFE_RETRY
WORKER_LEASE != EXECUTION_AUTHORITY
```

Every side-effect attempt has durable identity, fencing and reconciliation semantics.

---

## 10. TG-09 — Skill artifact lifecycle lacks explicit canonical ownership

**Source trigger:** SkillHone + WeKnora skill catalogs.

### Gap

A future skill system needs package/revision/source/eval/release/rollback ownership rather than treating a skill as a mutable prompt folder.

### Owner

021 authoring gate; potentially 021B if split is required.

### Candidate entities

```text
SkillPackage
SkillRevision
SkillSourceProvenance
SkillCapabilityRequirements
SkillRuntimeCompatibility
SkillEvalBinding
SkillReleaseState
SkillRollbackTarget
```

---

## 11. TG-10 — Held-out evaluation isolation is a security boundary

**Source trigger:** SkillHone private eval isolation.

### Gap

Prompt instructions not to inspect gold evidence are insufficient when an optimizer can access the same filesystem/process credentials.

### Owner

021 authoring gate / proposed 021B.

### Rule

```text
OPTIMIZER_ACCESS != HELD_OUT_GOLD_ACCESS
```

Enforce with filesystem/process/capability separation and leakage tests.

---

## 12. TG-11 — Persistent decision history can be mistaken for active memory

**Source trigger:** SkillHone Git-native diagnosis/change/outcome history.

### Gap

Optimization evidence and trajectories are valuable but are not automatically user-confirmed durable memory.

### Owner

021 evidence lifecycle; 006 only consumes explicit Memory Proposals.

### Rule

```text
DECISION_HISTORY != ACTIVE_MEMORY
TRAJECTORY != MEMORY
```

---

## 13. TG-12 — Generated wiki/summary state needs explicit authority transitions

**Source trigger:** WeKnora generated knowledge/wiki surfaces.

### Gap

A useful generated page may appear authoritative while still being a model/derived artifact.

### Owner

010 canonical object transitions + 006 memory proposal semantics + 011 UX.

### Required distinction

```text
GENERATED_DRAFT
DERIVED_REBUILDABLE_VIEW
USER_SAVED_CANONICAL_DOCUMENT
CANONICAL_PROPOSAL
MEMORY_PROPOSAL
APPROVED_MEMORY
```

---

## 14. TG-13 — Persistent sandbox can outlive the authority that created it

**Source trigger:** WeKnora session-persistent sandboxes.

### Gap

A sandbox may remain alive after grant/network/secret policy changes.

### Owner

Proposed 007B.

### Rule

Every side-effectful call rechecks generation-bound authority; sandbox existence is never enough.

The code-level review expands this through TG-21/TG-22.

---

## 15. TG-14 — Source-admission evidence needs a reusable contract

**Source trigger:** all three donors.

### Gap

Per-file code reuse at scale needs consistent provenance/rights/transformation/security evidence.

### Owner

Program engineering method / active owning spec per reused unit.

### Resolution

The follow-up now provides:

```text
docs/research/FOUNDER_SOURCE_USE_AUTHORIZATION_2026-09-08.md
docs/research/SOURCE_REUSE_ADAPTATION_MATRIX_2026-09-08.md
```

Founder direct source-use permission is recorded, while exact public-license and nested third-party status remain independently recorded.

---

# 16. Deep-pass continuation

The code-level pass adds:

```text
TG-15 bitemporal precision
TG-16 temporal model/checkpoint identity
TG-17 canonical retention vs retrieval forgetting
TG-18 proposal-pipeline durability/idempotency
TG-19 derived affinity/interest influence
TG-20 supersession-key collision
TG-21 remote resource binding vs authority
TG-22 policy/config/skill generation fencing
TG-23 artifact promotion boundary
TG-24 DNS/SSRF/egress semantics
TG-25 skill-install supply chain/privilege
TG-26 audit evidence vs output truncation
TG-27 eval redaction/score lineage
TG-28 optimizer attribution/contamination
```

See `V2_TENCENT_CODE_LEVEL_GAP_REVIEW_2026-09-08.md` for the load-bearing detail.

# 17. Final first-pass reconciliation

```text
FIRST_PASS_PRESERVED_AS_EVIDENCE=YES
FIRST_PASS_STALE_OWNERSHIP_CORRECTED=YES
TOTAL_SOURCE_DRIVEN_GAPS=28
GAPS_WITH_OWNER_OR_AUTHORING_GATE=28/28
FOUNDER_SOURCE_USE_PERMISSION=RECORDED
R1_CHANGED=NO
R1_REVIEW_CANDIDATE_CHANGED=NO
PRODUCT_IMPLEMENTATION_AUTHORIZED=NO
```
