# Tencent Source Qualification for Fehrest V2 — 2026-09-08

**Status:** RESEARCH RECORD / NON-AUTHORIZING  
**Change class:** planning input only; no R1 or product execution effect  
**Trigger:** founder-supplied gap-driven source review plus source-code-level follow-up  
**Canonical authority:** live `AGENTS.md`, `specs/CURRENT.md`, and canonical execution governance remain superior  
**Founder rights record:** `docs/research/FOUNDER_SOURCE_USE_AUTHORIZATION_2026-09-08.md`  
**Code-level gap review:** `docs/reviews/V2_TENCENT_CODE_LEVEL_GAP_REVIEW_2026-09-08.md`

> This record qualifies three external sources as future planning, benchmark and selective code-reuse candidates. It does not admit a runtime dependency, activate a future spec, alter R1, mutate the frozen R1-v2 review candidate, or authorize product implementation.

## 1. Source pins and rights basis

| Source | Reviewed revision | Public-license observation | Founder direct permission | Current Fehrest disposition |
|---|---|---|---|---|
| `Tencent/RoMem` | `39ac1417b4db41ea729e5c3be71ac20de54da993` | No root `LICENSE` found in reviewed tree | `YES` | `STUDY + BENCHMARK + CODE_PORT/ADAPT_CANDIDATE` |
| `Tencent/WeKnora` | `647848f3954dae34473b8a8d0e0eef5e0fb3a58e` | Root MIT; separately licensed third-party components recorded upstream | `YES` | `STUDY + BENCHMARK + COPY/PORT/ADAPT_CANDIDATE` |
| `Tencent/SkillHone` | `7d565839fb4dc74f9c77f09ace660e1c0484e048` | MIT | `YES` | `STUDY + BENCHMARK + COPY/PORT/ADAPT_CANDIDATE` |

Revision pins MUST be refreshed before a load-bearing implementation decision.

## 2. Reuse law

```text
SOURCE_FOUND != SOURCE_ADMITTED
PERMISSION_TO_COPY != REQUIREMENT_TO_COPY
PERMISSION_TO_COPY != RUNTIME_DEPENDENCY_AUTHORIZATION
PUBLIC_LICENSE_STATUS != FOUNDER_DIRECT_PERMISSION_STATUS
DONOR_IMPLEMENTATION != FEHREST_SEMANTIC_AUTHORITY
```

Every actual reuse still passes:

```text
requirement
-> Ponytail necessity gate
-> exact source revision/path
-> reuse dossier
-> license/direct-permission/attribution evidence
-> third-party component review
-> security/privacy review
-> benchmark where material
-> active-spec implementation authority
-> tests/evidence
```

Rust semantic ownership remains unchanged. Prefer Rust ports or Fehrest-owned implementations with donor test vectors when donor runtime language would otherwise own canonical semantics.

## 3. RoMem — code-level qualification

### Reviewed code surfaces

```text
romem/romem_reranker.py
romem/kge/time_utils.py
README.md
```

### Useful mechanisms

- continuous-time temporal reranking;
- relation-dependent semantic speed/volatility gate;
- semantic + temporal score fusion;
- checkpointed temporal/KGE state;
- temporal parsing and normalization utilities;
- non-destructive treatment of temporally obsolete facts;
- temporal-memory benchmark/comparator methodology.

### New constraints exposed

```text
TEMPORAL_RANK != TEMPORAL_TRUTH
PARTIAL_TIME != EXACT_TIME
RELATIVE_TIME_REQUIRES_EXPLICIT_REFERENCE_TIME
EVENT_TIME != OBSERVATION_TIME
MODEL/CHECKPOINT/FUSION_IDENTITY_MUST_BE_BOUND
POST_HOC_ALPHA_TUNING=NO
```

### Preferred future reuse shape

```text
TIME_PARSER_TEST_VECTORS=HIGH_VALUE
TEMPORAL_SCORE_FORMULA_PORT=CONDITIONAL
RERANKER_RUST_PORT=CONDITIONAL_ON_004_BENCHMARK
PYTHON_KGE_PROVIDER=ONLY_IF_004_PROVES_MATERIAL_NEED_AND_PROVIDER_BOUNDARY_IS_AUTHORIZED
```

RoMem code use is permitted by Founder direct permission. The absence of an observed root public license must remain recorded; before public distribution of copied material, preserve durable evidence of the direct permission and any applicable upstream notices.

## 4. WeKnora — code-level qualification

### Reviewed code/docs surfaces

Representative reviewed areas include:

```text
internal/types/memory.go
internal/application/service/memory/retrieval_test.go
internal/sandbox/session_manager.go
docs/sandbox-docker-backend.md
internal/agent/tools/shell_exec.go
internal/application/service/tenant_skill_install.go
README.md / CHANGELOG.md
```

### High-value reusable patterns

```text
pending inferred memory excluded until confirmation
rejected-inference dedup/tombstone behavior
principal/workspace memory subject scoping
resident vs situational recall separation
extraction cursors and pending queues
conservative memory merge tests
session-persistent sandbox lifecycle
separate installer vs runtime executor capability
skill bundle SHA / catalog / activation-pointer patterns
old-version-keeps-serving on failed install
shared persistent web-fetch snapshot/waiter/retry pattern
```

### Important donor assumptions Fehrest should NOT inherit blindly

```text
USAGE_BASED_AUTO_FORGETTING_AS_CANONICAL_DELETION=NO
NORMALIZED_TOPIC_KEY_AS_SUPERSESSION_AUTHORITY=NO
REDIS_OR_PROVIDER_HANDLE_AS_AUTHORITY=NO
PATH_PREFIX_CHECK_AS_SYMLINK_SECURITY_BOUNDARY=NO
DEFAULT_ROOT_EXECUTION_AS_FEHREST_REQUIREMENT=NO
NETWORK_ON_OFF_AS_COMPLETE_EGRESS_POLICY=NO
QUEUE_RETRY_AS_SAFE_SIDE_EFFECT_RETRY=NO
TRUNCATED_MODEL_OUTPUT_AS_COMPLETE_AUDIT_EVIDENCE=NO
```

WeKnora is MIT at repository root, but copied paths must still be checked for nested third-party provenance/NOTICE obligations.

## 5. SkillHone — code-level qualification

### Reviewed surfaces

```text
skills/skillhone-optimization/SKILL.md
skills/skillhone-evaluation/SKILL.md
LICENSE
```

### High-value reusable patterns

```text
skill repo != private eval repo
per-item isolated workdir
redacted observation surface
diagnose-before-fix
focused improvement cycles
score provenance
trajectory analysis
issue/branch/PR evidence loop
standard compiler/toolchain preference
private final test split
```

### Required Fehrest constraints

```text
OPTIMIZER_ACCESS != HELD_OUT_GOLD_ACCESS
FINAL_TEST_VISIBLE_DURING_ITERATION=NO
FINAL_TEST_QUERIED_DURING_ITERATION=NO
EVAL_PASS != PRODUCT_AUTHORITY
DECISION_HISTORY != ACTIVE_MEMORY
CUSTOM_VALIDATOR_REQUIRES_PARITY_EVIDENCE=YES
DONOR_EXAMPLE_THRESHOLDS != FEHREST_STATISTICAL_THRESHOLDS
```

SkillHone is MIT. Future dev/eval tooling may directly reuse scripts/docs/patterns when an active spec or repository-development requirement owns the work, with attribution and exact revision/path provenance.

## 6. Comparative reuse priority

This is reuse leverage, not implementation order:

```text
P1_WEKNORA = memory safety tests + sandbox/install lifecycle patterns
P2_SKILLHONE = evaluation isolation + observation/redaction/provenance tooling
P3_ROMEM = temporal test vectors + bounded temporal scoring mechanisms after benchmark need exists
```

RoMem remains highest-value as a temporal scientific comparator; WeKnora currently offers the broadest directly reusable engineering patterns; SkillHone offers the cleanest reusable development/evaluation methodology.

## 7. Benchmark hypotheses

Future authorized benchmark/spec work should test at least:

1. temporal reranking versus explicit canonical-time filtering + lexical/structured retrieval;
2. memory-residency policy versus indiscriminate durable-memory injection;
3. risk-sensitive promotion versus generic auto-promotion;
4. durable proposal extraction under crash/replay/dedup;
5. delegated execution fencing/retry/reconciliation under failure;
6. sandbox revocation and policy-generation changes while resource remains alive;
7. artifact ingress/egress and canonical promotion under symlink/TOCTOU/adversarial inputs;
8. whole-skill optimization versus prompt-only change under structurally isolated held-out evaluation.

Every experiment requires a strong simple baseline and an explicit kill route.

## 8. Current decision

```text
SOURCE_QUALIFICATION_COMPLETE=YES
FOUNDER_DIRECT_SOURCE_USE_PERMISSION=RECORDED
ROMEM_CODE_REUSE_CANDIDATE=YES
WEKNORA_CODE_REUSE_CANDIDATE=YES
SKILLHONE_CODE_REUSE_CANDIDATE=YES
CURRENT_RUNTIME_DEPENDENCY_ADMISSION=NO
PRODUCT_IMPLEMENTATION_AUTHORIZED=NO
R1_CHANGED=NO
R1_REVIEW_CANDIDATE_CHANGED=NO
```
