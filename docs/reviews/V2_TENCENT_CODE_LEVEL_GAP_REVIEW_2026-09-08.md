# Fehrest V2 Tencent Code-Level Gap Review — 2026-09-08

**Status:** REVIEW / NON-AUTHORIZING  
**Review layer:** source-code-level convergence after the initial 14-gap source review  
**Sources:** Tencent/RoMem, Tencent/WeKnora, Tencent/SkillHone at pinned revisions  
**Execution effect:** NONE  
**Canonical frontier:** live `specs/CURRENT.md`

> This review inspects concrete donor code/contracts rather than README-level capabilities. It finds additional semantic, security, durability and evaluation gaps without turning donor implementations into requirements. No R1 artifact, review candidate, current frontier or implementation authority is changed.

## 1. Verdict

The first source pass identified `TG-01..TG-14`. The source-code pass identifies a second set, `TG-15..TG-28`.

```text
FIRST_PASS_GAPS=14
CODE_LEVEL_GAPS=14
TOTAL_SOURCE_DRIVEN_GAPS=28
CODE_LEVEL_GAPS_WITH_OWNER_OR_AUTHORING_GATE=14/14
NEW_CURRENT_IMPLEMENTATION_AUTHORITY=0
R1_SEMANTICS_CHANGED=NO
R1_REVIEW_CANDIDATE_CHANGED=NO
```

The most important architectural correction is that the proposed future Spec 007 should not simultaneously own read/context compilation and side-effectful execution semantics. A proposed `007A/007B` decomposition is recorded as an authoring-time planning correction, without renumbering the existing future program today.

---

## 2. TG-15 — Bitemporal semantics and temporal precision are under-specified

**Source trigger:** RoMem `romem/kge/time_utils.py` and timed-triple handling.

### Gap

RoMem accepts event/happening time, observation time and query/reference time, and parses partial/relative temporal expressions. A convenient parser may map ambiguous values such as a year, month, season or relative phrase into a precise timestamp. That is acceptable for a derived model input only if the original uncertainty is preserved.

### Owner

```text
006 = canonical temporal semantics
004 = temporal-intelligence experiment input semantics
003/007A = retrieval/context consumers
```

### Required future contract

```text
EVENT_TIME
OBSERVATION_TIME
RECORD_TIME
VALID_FROM
VALID_TO
QUERY_AS_OF
TIME_PRECISION
TIMEZONE_OR_UNKNOWN
LOCALE_OR_UNKNOWN
SOURCE_EXPRESSION
REFERENCE_TIME_FOR_RELATIVE_PARSE
PARSE_CONFIDENCE_OR_UNCERTAINTY
```

### Hard rule

```text
PARTIAL_DATE_NORMALIZATION != EXACT_CANONICAL_TIME
RELATIVE_DATE_PARSE_WITHOUT_REFERENCE_TIME=INVALID_FOR_CANONICAL_TRUTH
```

A source saying `March 2020` must not silently become canonically equivalent to `2020-03-01T00:00:00Z`.

---

## 3. TG-16 — Derived temporal-model identity and calibration are not fully bound

**Source trigger:** RoMem `RoMemReranker`.

### Gap

RoMem-style scoring depends on the embedding model, relation-speed gate checkpoint, KGE parameters, random seed, training triples and semantic/temporal fusion coefficient. A benchmark can drift while appearing to run the same algorithm.

### Owner

004 experiment; 005 conditional retained provider.

### Required binding

```text
EMBEDDING_MODEL_ID
EMBEDDING_MODEL_REVISION
GATE_CHECKPOINT_SHA256
KGE_CONFIGURATION
TRAINING_CORPUS_MANIFEST
SEED
FUSION_FORMULA
FUSION_ALPHA
DEPENDENCY_LOCK
FALLBACK_BEHAVIOR
```

No post-hoc alpha/checkpoint selection after benchmark outcomes are visible.

---

## 4. TG-17 — Canonical retention is conflated with retrieval forgetting/archival

**Source trigger:** WeKnora memory capacity/automatic archiving behavior.

### Gap

A memory system may demote or archive low-ranked items to bound context and storage. In Fehrest, retrieval visibility must remain separate from canonical retention and historical truth.

### Owner

006.

### Required states/policies

At minimum distinguish outcomes equivalent to:

```text
ACTIVE
HIDDEN_FROM_DEFAULT_RECALL
SUPERSEDED
RETRACTED
ARCHIVED_BY_USER
EXPIRED_BY_EXPLICIT_POLICY
DELETED_BY_USER
```

Usage/rank alone may not erase canonical truth or historical evidence.

---

## 5. TG-18 — Memory-proposal extraction needs durable completeness and idempotency

**Source trigger:** WeKnora extraction cursors, pending sessions and repeated-pending deduplication.

### Gap

Automatic proposal generation can silently skip messages after a crash, generate duplicate proposals after replay, or advance a cursor before durable proposal creation.

### Owner

006 proposal semantics; 007B when delegated background execution is used.

### Required mechanism

```text
DURABLE_SOURCE_CURSOR
EXTRACTION_BATCH_ID
IDEMPOTENCY_KEY
SOURCE_RANGE
PROPOSAL_SET_DIGEST
AT_LEAST_ONCE_INPUT_PROCESSING
IDEMPOTENT_PROPOSAL_CREATION
CURSOR_ADVANCE_AFTER_DURABLE_COMMIT
REPLAY_SAFE=YES
```

Exactly-once side effects must not be claimed from a queue alone.

---

## 6. TG-19 — Derived interests/affinity can become hidden behavioral authority

**Source trigger:** WeKnora interest recurrence and document affinity influencing retrieval.

### Gap

A derived interest or affinity is not an explicit user statement, yet it can shape what context the user or agent sees. This influence needs stronger classification than an ordinary rank feature.

### Owner

006 semantics + 007A context policy.

### Required controls

```text
INFLUENCE_CLASS=DERIVED_BEHAVIORAL_SIGNAL
PER_PRINCIPAL_ISOLATION=YES
PER_WORKSPACE_ISOLATION=YES
EXPLAINABLE_IN_RECEIPT=YES
USER_DISABLE_OR_RESET=YES
EXPIRY_OR_REVALIDATION=DEFINED
CAN_AUTHOR_CANONICAL_MEMORY=NO
```

---

## 7. TG-20 — Normalized-key supersession/merge can destroy incomparable claims

**Source trigger:** WeKnora normalized-key replacement/merge behavior and conservative merge tests.

### Gap

A normalized topic key is useful for deduplication but too weak to authorize canonical supersession by itself.

### Owner

006.

### Required rule

A semantic identity key must account for the relevant subject, scope, memory kind and claim identity. False merge/supersession is treated as higher severity than leaving two duplicates.

Rejected inferred claims should leave a scoped rejection/tombstone record sufficient to prevent the same unsupported guess from recurring silently.

---

## 8. TG-21 — Persistent sandbox/resource binding cannot be execution authority

**Source trigger:** WeKnora session-persistent sandbox manager and external binding store.

### Gap

A provider handle or Redis/session binding can survive a process and look authoritative even after Fehrest grants/policy change.

### Owner

Proposed 007B Delegated Execution, Fencing and Receipt Foundation.

### Required rule

```text
PROVIDER_RESOURCE_HANDLE=DERIVED_EXECUTION_STATE
RESOURCE_LIVENESS != AUTHORIZATION_LIVENESS
CANONICAL_ATTEMPT_RESOURCE_MAPPING=FEHREST_OWNED
RECONCILIATION_REQUIRED_AFTER_PROCESS_RESTART=YES
```

---

## 9. TG-22 — Every side effect needs policy/config/skill generation fencing

**Source trigger:** persistent sandboxes, config replacement and per-skill execution contexts.

### Gap

A sandbox created under an old grant or network policy may remain alive after revocation. Checking only at sandbox creation is insufficient.

### Owner

007B.

### Every side-effectful command must bind

```text
PRINCIPAL_ID
GRANT_GENERATION
POLICY_GENERATION
EXECUTION_ATTEMPT_ID
SANDBOX_IMAGE_DIGEST
SKILL_REVISION
SECRET_SCOPE
NETWORK_POLICY_DIGEST
EXECUTOR_AUDIENCE
```

Future calls fail closed if an authority-bearing generation is stale.

---

## 10. TG-23 — Sandbox artifact ingress/egress and canonical promotion are incomplete

**Source trigger:** WeKnora input/output workspace conventions and file APIs.

### Gap

Attachments and sandbox-generated files cross a trust boundary. A writable output directory is not a canonical workspace.

### Owners

```text
007B = execution boundary
010 = canonical object promotion
021 = extension/skill artifact contracts
```

### Required controls

```text
INGRESS_SOURCE_IDENTITY
IMMUTABLE_INPUT_OR_COPY_SEMANTICS
PATH_RESOLUTION_AT_USE
NO_FOLLOW_OR_EQUIVALENT_CONFINEMENT
SIZE_AND_RESOURCE_LIMITS
CONTENT_TYPE_VALIDATION
MALWARE/UNTRUSTED_CLASSIFICATION_WHERE_REQUIRED
OUTPUT_DIGEST
OUTPUT_PROVENANCE
EXPLICIT_CANONICAL_PROMOTION
```

Intermediate symlinks and TOCTOU must be included in adversarial tests; string-prefix path checks are not a security boundary.

---

## 11. TG-24 — Egress policy needs DNS/redirect/private-network semantics

**Source trigger:** WeKnora Docker backend network modes and documented external-proxy requirement for domain policy.

### Gap

`NETWORK=ON/OFF` is insufficient for a scoped external-tool platform. Domain allowlists alone are vulnerable to DNS rebinding and redirects unless policy is revalidated at connection/use time.

### Owners

007B general egress capability; 014 web/tool-specific origin rules.

### Required future security cases

```text
DNS_REBINDING
REDIRECT_TO_PRIVATE_IP
LOOPBACK
LINK_LOCAL
RFC1918_OR_PLATFORM_PRIVATE_NETWORK
IP_LITERAL
CNAME_CHAIN
DNS_CHANGE_DURING_SESSION
PROXY_BYPASS
TOOL_REQUESTED_SCOPE_WIDENING
```

Every action receipt records effective network policy and resolved destination evidence appropriate to the threat model.

---

## 12. TG-25 — Skill installation is a separate privileged supply-chain operation

**Source trigger:** WeKnora separate install executor, root install mode, bundle SHA and catalog/snapshot activation.

### Gap

Installing/building a skill may require broader filesystem/package authority than running it. Combining install and runtime grants creates avoidable privilege.

### Owners

021 package lifecycle; 007B execution foundation.

### Required separation

```text
INSTALLER_PRINCIPAL_OR_CAPABILITY != RUNTIME_PRINCIPAL_OR_CAPABILITY
INSTALLER_NETWORK_SCOPE != RUNTIME_NETWORK_SCOPE
INSTALLER_SECRET_SCOPE != RUNTIME_SECRET_SCOPE
```

A release should bind:

```text
SOURCE_REPOSITORY
SOURCE_COMMIT
SOURCE_PATH
BUNDLE_SHA256
DEPENDENCY_LOCK
SBOM_OR_EQUIVALENT_COMPONENT_INVENTORY
ADVISORY_SCAN
LICENSE/NOTICE_SET
BUILD_IMAGE_DIGEST
INSTALL_RECEIPT
RUNTIME_COMPATIBILITY
```

Root execution is not a default Fehrest requirement; it must be justified by the active spec/provider boundary.

---

## 13. TG-26 — Model-visible output truncation must not truncate audit truth

**Source trigger:** WeKnora `shell_exec` bounded head/tail output.

### Gap

Bounding model-visible logs is good resource hygiene. But a truncated excerpt cannot be the complete execution evidence.

### Owner

007B.

### Required receipt distinctions

```text
MODEL_VISIBLE_OUTPUT=BOUNDED
OUTPUT_TRUNCATED=<bool>
FULL_STDOUT_DIGEST
FULL_STDERR_DIGEST
FULL_OUTPUT_BYTE_COUNTS
RAW_LOG_RETENTION_CLASS
EXIT_STATUS
TERMINATION_REASON
SIDE_EFFECT_DISPOSITION
```

Differentiate at least transport timeout, explicit kill, process exit, provider disappearance and `INDETERMINATE_SIDE_EFFECT`.

---

## 14. TG-27 — Eval observation/redaction and score lineage need a formal contract

**Source trigger:** SkillHone skill/eval/workdir/observation separation and score provenance.

### Gap

A future skill optimizer could leak private probes/gold answers through issue text, logs, trajectories or aggregate diagnostics even if the eval repository path itself is hidden.

### Owner

Proposed 021B Skill Package, Evaluation and Evolution authoring boundary.

### Required score/evidence binding

```text
SKILL_REVISION
EVAL_SUITE_REVISION
SPLIT_ID
SPLIT_ROLE=PROBE|PR_VALIDATION|FINAL_TEST
MODEL_RUNTIME
EVALUATOR_REVISION
WORKDIR_OR_EVIDENCE_DIGEST
SCORE_PROVENANCE
REDACTION_POLICY_REVISION
RUN_ID
TIMESTAMP
```

The observation surface must be explicitly redacted and reviewed for leakage.

---

## 15. TG-28 — Optimization causal attribution, evaluator drift and final-test contamination

**Source trigger:** SkillHone one-focused-change loop, private final test and compiler/verifier discipline.

### Gap

Repeated multi-factor optimizer changes make it hard to know what improved a score. A custom local validator can also drift from the real evaluator, creating false confidence.

### Owner

021B authoring gate.

### Required rules

```text
FINAL_TEST_VISIBLE_DURING_ITERATION=NO
FINAL_TEST_QUERIED_DURING_ITERATION=NO
CUSTOM_VALIDATOR_REQUIRES_PARITY_EVIDENCE=YES
STANDARD_TOOLCHAIN_PREFERRED_WHEN_IT_IS_THE_REAL_CONTRACT=YES
FAILURE_CLASSIFICATION=INFRA|SOLVER|COMPILER|VERIFIER|SKILL|SECURITY
```

Use focused changes per measured cycle where practical to preserve attribution. Do not copy SkillHone example thresholds as Fehrest statistical thresholds without a Fehrest preregistration/evidence basis.

---

# 16. Architecture correction — split read/context semantics from delegated execution semantics

The first Tencent plan amendment placed general `ExecutionAdmission`, fencing and receipts into future Spec 007 alongside the Context Compiler. Code-level review shows this violates the program's own `ONE SEMANTIC RESPONSIBILITY -> ONE OWNER` principle.

## Proposed future authoring decomposition

Do not renumber the existing roadmap yet. At the post-R1 authoring/canonicalization gate, split the conceptual 007 responsibility into:

### 007A — Universal Context and Memory Gateway

Owns:

```text
principal/session/grant baseline for reads
Context Compiler
scope filtering
context budgeting
resident/on-demand context policy
context packages
context receipts
SelectionTrace
read-only CLI/SDK/local API contracts
```

### 007B — Delegated Execution, Fencing and Receipt Foundation

Owns:

```text
side-effect capability admission
ExecutionAttemptId
policy/grant/config generation fencing
DurableDispatchIntent
provider/sandbox resource binding
retry/idempotency semantics
cancellation ownership
TerminalReceipt
IndeterminateExecution
reconciliation evidence
just-in-time secret injection contract
resource/network policy envelope
artifact ingress/egress trust boundary
```

### Dependency consequences

```text
008 GitHub/IDE discovery -> 007A
009 trusted vertical proof -> 007A; 007B only if delegated side effects are in the proof
013 Ask/read-only AI -> 007A
013 tool/sandbox actions -> 007B
014 read-only acquisition -> 007A + 014
014 action-capable tools -> 007B + 014
021 extension/automation runtime -> 007B
```

This split keeps ordinary context retrieval lightweight and prevents executor infrastructure from becoming a prerequisite for read-only clients.

---

# 17. 021 authoring gate

SkillHone shows that extension runtime and skill evolution can become independent semantic systems. Do not split the numbered roadmap yet, but require the future 021 authoring phase to decide whether it must become:

```text
021A = Extension / Connector / Automation Runtime
021B = Skill Package / Evaluation / Evolution
```

Decision criterion: split only if one spec cannot remain independently testable and narrowly owned without creating lifecycle/evaluation coupling.

---

# 18. Code-reuse consequences

The Founder has explicitly authorized code reuse. Therefore donor code may become a future `COPY`, `PORT`, `ADAPT` or `TEST_VECTOR` candidate after the owning requirement is active.

This review still recommends selective reuse:

### RoMem

Prefer Rust ports/test vectors for time semantics and small scoring mechanisms. A Python/KGE runtime remains conditional on 004 evidence and an explicit provider-boundary decision.

### WeKnora

Strong future candidates for selective port/adaptation include confirmation/rejection tests, extraction cursor/dedup patterns, subject scoping tests, sandbox lifecycle patterns, separate installer/runtime capability patterns, bundle-digest/activation patterns and shared web-fetch snapshot waiter/retry patterns.

Do **not** inherit documented weak/default assumptions such as root execution, prefix-only path checks or coarse network policy as Fehrest security decisions.

### SkillHone

Strong future candidates include evaluation split-isolation patterns, redaction/observation tooling, score provenance, trajectory diagnosis and focused change/review loops. Keep optimizer/evaluator authority separate from canonical product authority.

---

# 19. Final state

```text
TOTAL_SOURCE_DRIVEN_GAPS=28
TG_01_TO_TG_28_HAVE_OWNER_OR_AUTHORING_GATE=YES
PROPOSED_007A_007B_SPLIT=YES_NON_AUTHORIZING
PROPOSED_021A_021B_AUTHORING_GATE=YES
FOUNDER_CODE_REUSE_PERMISSION_RECOGNIZED=YES
CURRENT_RUNTIME_DEPENDENCY_ADMISSION=NO
R1_CHANGED=NO
R1_REVIEW_CANDIDATE_CHANGED=NO
PRODUCT_IMPLEMENTATION_AUTHORIZED=NO
```
