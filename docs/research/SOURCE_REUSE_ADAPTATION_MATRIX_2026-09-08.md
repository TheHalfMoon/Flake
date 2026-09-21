# Fehrest Source Reuse and Adaptation Matrix — 2026-09-08

**Status:** PLANNING / NON-AUTHORIZING  
**Purpose:** identify the smallest future copy/port/adapt opportunities made possible by Founder source-use permission  
**Canonical frontier:** live `specs/CURRENT.md`

> This matrix is not a dependency list. It records candidate reuse units so future active specs can choose the smallest justified reuse mode instead of either rebuilding everything or importing an entire donor stack.

## 1. Reuse modes

```text
COPY        = substantially preserve source implementation
PORT        = translate implementation into Fehrest-owned language/runtime while preserving provenance
ADAPT       = reuse implementation structure with material Fehrest-specific changes
TEST_VECTOR = reuse fixtures/behavioral examples only
REFERENCE   = study only; no source-derived implementation unit
```

Selection rule:

```text
SMALLEST_CORRECT_REUSE_MODE_WINS
```

## 2. Candidate matrix

| Source | Upstream path/family | Candidate Fehrest use | Preferred reuse mode | Future owner | Gate |
|---|---|---|---|---|---|
| RoMem | `romem/kge/time_utils.py` | temporal precision/partial/relative-date fixtures | `TEST_VECTOR` / `PORT` | 006 | bitemporal contract + temporal correctness tests |
| RoMem | `romem/romem_reranker.py` | temporal score fusion/reranking comparator | `PORT` | 004 | must beat strong explicit-time baseline |
| RoMem | KGE/checkpoint machinery | retained temporal intelligence provider | `REFERENCE` then conditional `ADAPT` | 005 | only if 004 retains advanced provider |
| RoMem | benchmark harnesses | temporal contradiction benchmark cases | `ADAPT` / `TEST_VECTOR` | 004/009 | benchmark provenance review |
| WeKnora | memory pending/confirm/reject tests | confirmation, rejection tombstone, dedup semantics | `PORT` / `TEST_VECTOR` | 006 | Fehrest lifecycle semantics first |
| WeKnora | memory extraction cursor/pending session patterns | crash/replay-safe proposal intake | `ADAPT` | 006 + 007B | durability/idempotency tests |
| WeKnora | memory subject/scoping tests | principal/workspace isolation | `PORT` / `TEST_VECTOR` | 006/007A | Fehrest grant semantics remain owner |
| WeKnora | resident/situational memory patterns | context-residency comparator | `ADAPT` | 007A | privacy/token/staleness benchmark |
| WeKnora | sandbox session manager | persistent provider-resource lifecycle | `ADAPT` | 007B | Fehrest-owned fencing/receipt contract |
| WeKnora | Docker sandbox docs/tests | adversarial sandbox/security test cases | `TEST_VECTOR` / `REFERENCE` | 007B | threat-model reconciliation |
| WeKnora | `shell_exec` split between install/runtime | separate installer/runtime capability | `ADAPT` | 007B/021 | least-privilege proof |
| WeKnora | tenant skill install pipeline | bundle digest, activation pointer, rollback-to-old snapshot | `ADAPT` | 021 | durable install state machine required |
| WeKnora | persistent web-page fetch snapshot logic | shared acquisition job + waiter/retry semantics | `ADAPT` | 014 + 007B if background execution | freshness/cancellation/retry contract |
| SkillHone | evaluation skill | split-isolated eval runner and score provenance | `COPY` / `ADAPT` for dev tooling | future 021B | held-out leak tests |
| SkillHone | optimization skill | diagnosis/change/review loop | `COPY` / `ADAPT` for dev tooling | future 021B | no auto-merge authority |
| SkillHone | redaction/observation patterns | safe optimization evidence surface | `ADAPT` | future 021B | gold leakage tests |
| SkillHone | trajectory diagnostics | failure classification | `ADAPT` | future 021B | infra/solver/compiler/verifier/skill taxonomy |

## 3. Explicit non-adoptions

Permission does not make these defaults:

```text
ROMEM_PYTHON_RUNTIME_AS_CANONICAL_CORE=NO
WEKNORA_FULL_STACK_IMPORT=NO
WEKNORA_REDIS_AS_AUTHORITY=NO
WEKNORA_DEFAULT_ROOT_SANDBOX_POLICY=NO
WEKNORA_PATH_PREFIX_SECURITY_MODEL=NO
SKILLHONE_AUTO_OPTIMIZER_AS_PRODUCT_AUTHORITY=NO
SKILLHONE_EXAMPLE_SCORE_THRESHOLDS_AS_FEHREST_THRESHOLDS=NO
```

## 4. Reuse dossier template

Every actual source-derived implementation unit should carry a machine-readable or reviewable record equivalent to:

```yaml
source_id: SRC-...
source_repository: owner/repo
source_commit: <sha>
source_path: path/to/upstream
reuse_mode: COPY|PORT|ADAPT|TEST_VECTOR|REFERENCE
destination_path: path/to/fehrest
rights_basis: PUBLIC_LICENSE|FOUNDER_DIRECT_PERMISSION|BOTH
upstream_license_observed: <value-or-NOT_OBSERVED>
third_party_components: []
copyright_notice: <record>
attribution_location: <path>
transformation_summary: <text>
owner_spec: <id>
security_review: PENDING|PASS|N/A
benchmark_decision: PENDING|RETAIN|REJECT|N/A
test_evidence: <refs>
advisory_scan: <refs>
runtime_dependency_change: YES|NO
canonical_authority_change: YES|NO
```

## 5. Copy versus port decision

Prefer direct `COPY` where all are true:

```text
language/runtime fits the owning boundary
source unit is small and well-isolated
license/direct-permission evidence is clear
security model is compatible
copying reduces risk versus reimplementation
```

Prefer `PORT` where donor logic is valuable but Fehrest's Rust semantic-ownership invariant would otherwise be violated.

Prefer `TEST_VECTOR` where behavior is valuable but implementation/runtime coupling is not.

Prefer `REFERENCE` when the donor implementation carries architecture assumptions Fehrest should not inherit.

## 6. Current state

```text
FOUNDER_SOURCE_USE_PERMISSION=YES
REUSE_CANDIDATES_RECORDED=YES
CURRENT_SOURCE_CODE_IMPORT=NO
CURRENT_RUNTIME_DEPENDENCY_CHANGE=NO
R1_CHANGED=NO
PRODUCT_IMPLEMENTATION_AUTHORIZED=NO
```
