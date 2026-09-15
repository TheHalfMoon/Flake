# Fehrest Full Donor Reuse and Adaptation Review — 2026-09-08

**Status:** RESEARCH / NON-AUTHORIZING / PLANNING ONLY
**Scope:** complements `TENCENT_SOURCE_QUALIFICATION_2026-09-08.md` and `SOURCE_REUSE_ADAPTATION_MATRIX_2026-09-08.md`
**Purpose:** extend selective reuse decisions beyond the 3 Tencent sources to the entire Fehrest donor/source landscape, under the same `SMALLEST_CORRECT_REUSE_MODE_WINS` rule and Founder source-use authorization
**Execution effect:** NONE — no runtime dependency admitted, no code imported, no spec activated
**Canonical frontier:** live `specs/CURRENT.md`

> This review exists because PR #40’s matrix correctly decided the Tencent donors but did not yet decide the earlier donors already referenced by PR #2 and `docs/canonical/ARCHITECTURE_FREEZE.md`. A donor without a per-unit REUSE decision remains an implicit assumption. This document closes that gap.

## 1. Reuse law (reinforced)

```text
SOURCE_FOUND != SOURCE_ADMITTED
PERMISSION_TO_COPY != REQUIREMENT_TO_COPY
PERMISSION_TO_COPY != RUNTIME_DEPENDENCY_AUTHORIZATION
PUBLIC_LICENSE_STATUS != FOUNDER_DIRECT_PERMISSION_STATUS
DONOR_IMPLEMENTATION != FEHREST_SEMANTIC_AUTHORITY
RUST_OWNS_CANONICAL_SEMANTICS=YES
SMALLEST_CORRECT_REUSE_MODE_WINS
```

Every future reuse still passes:

```text
requirement
-> Ponytail necessity gate (does it need to exist? already in Fehrest/Rust/std/platform/deps?)
-> exact source repository/revision/path
-> reuse dossier (per FOUNDER_SOURCE_USE_AUTHORIZATION §4)
-> license/direct-permission/attribution + nested third-party review
-> security/privacy review
-> benchmark where material
-> active-spec implementation authority
-> tests/evidence
-> advisory scan
```

## 2. Full donor snapshot at this review

Distinguishes three donor epochs:

- **F0/F1 core donors** (frozen by Architecture Freeze §12, donor freeze): Mem0, Letta/MemGPT, Graphiti, Chroma, Aider, Graphify, Code-Graph-RAG, Qdrant, LangGraph, LangChain, LlamaIndex, Firecrawl, Braintrust, E2B/Daytona, etc.
- **2026-08 Tencent donors** (this convergence): RoMem, WeKnora, SkillHone at pins in `TENCENT_SOURCE_QUALIFICATION`.
- **Observed but not donor-admitted** (identification only): Penpot, AFFiNE BlockSuite, Loro, Automerge, y-octo, Superset, Flint, OpenPencil — all `STUDY/DEFER` per `docs/20-FUTURE-GATES.md` and must not enter reuse without separate gap trigger.

## 3. Per-unit decisions — earlier donors

| Donor | Upstream unit / pattern | Fehrest use considered | Decision | Why not more / why not less | Future owner | Evidence gate |
|---|---|---|---|---|---|---|
| Mem0 | memory lifecycle state machine, promotion tests, async update | promotion safety tests, memory topology | **PORT + TEST_VECTOR** | Test vectors high value; Python async runtime not canonical semantic owner | 006 | B-MEM-PROMOTE |
| Mem0 | vector-backed recall loop | optional vector recall | **REFERENCE** (defer) | Vectors deferred per ADR-0007; no vector default | 004 experiment | lexical baseline first |
| Letta Code (MemGPT) | agentic memory tool loop, function schemas, archival recall | agent memory tooling | **REFERENCE** | Grant/strategy differs; feasible to study but not copy tool loop | 006/007A | gap-driven only |
| Graphiti (incl. temporal) | temporal-context graph construction, episodic search | temporal graph candidate | **REFERENCE + conditional ADAPT** | Hypothesis testable but not authorizing; only if 004 RETAIN | 004/005 | 004 kill/retain |
| Chroma | embedding store, collection API, distance metrics | vector index | **REFERENCE + TEST_VECTOR conditional** | Vectors optional/derived; chroma infra not required for thesis | 003/004 | benchmark |
| Qdrant | HNSW, filtering, payload indexes | vector index | Same as Chroma — **REFERENCE** | Same gate | 003/004 | benchmark |
| Aider | repo-map generation, symbol graph, ranking | lexical retrieval baseline | **PORT / TEST_VECTOR** | Strong simple baseline mandatory per Execution Master Plan §6 | 003 | rebuild equivalence |
| Graphify | deterministic relationship extraction, ingestion pipeline | derived graph provider | **ADAPT with Rust-owned boundary** (conditional) | Replaceable Python sidecar per ADR-0003, only if 004 RETAIN | 004/005 | GI-CAP |
| Code-Graph-RAG | graph construction provenance, evaluation harness | evaluation shape | **TEST_VECTOR + REFERENCE** | Reuse evaluation method, not stack | 004 | benchmark |
| LangGraph / LangChain / LlamaIndex | prompt/tool/provider abstraction, adapter shapes | provider abstraction patterns | **REFERENCE** | Fehrest provider boundary remains Fehrest-owned; no framework copy | 013/014 | research only |
| Firecrawl | web extraction, crawl provenance | web snapshot acquisition | **ADAPT conditional** | Useful for 014 but must pass SSRF/DNS/private-network hardening | 014 | security review |
| Braintrust | eval tracing, dataset management | eval observation | **REFERENCE** | Hosted tracing study; Fehrest owns open trial schema | 021B | - |
| E2B / Daytona / OpenSandbox | egress policy, credential injection, sandbox isolation | delegated execution provider | **REFERENCE + provider ADAPT conditional** | Platform provider only; Fehrest never builds own sandbox platform | 007B | provider boundary |
| DeepSeek Harness | event model, `MODEL_VISIBLE_FEHREST_INPUT => RECEIPTED` principle | receipted context contract | **TEST_VECTOR + REFERENCE** | Principle is P1 reuse; harness sandbox network model not Fehrest boundary | 002/007A | receipt contract |
| Loro / Automerge / y-octo / Yjs/Yrs | CRDT sync primitives | collaboration experiment | **DEFER (STUDY)** | Collaboration gated by 016 experiment; Rust-native ≠ requirement | 016 | 016 retain/defer/reject |
| Penpot / AFFiNE Edgeless / Excalidraw / tldraw | canvas engine | visual layer | **DEFER (STUDY)** | Canvas deferred per 20-FUTURE-GATES §2 | future canvas gate | visual gate |
| Superset / Data Formulator / Flint | dashboard/view layer | view engine | **DEFER (STUDY)** | Analytics deferred; canonical objects != views | future view gate | view gate |
| OpenPencil | visual interchange | interchange | **DEFER (STUDY)** | License not established; identification ≠ adoption | future visual gate | visual gate |

## 4. Corrective closure of two hidden assumptions

### 4.1 Aider repo-map must be baseline, not optional

Prior donor freeze correctly notes repo-map as baseline candidate. This review hardens to mandatory baseline for every code/project workload benchmark after 003.

### 4.2 Graph vs lexical vs vector ordering is now explicit

Prior plan said vectors optional. This review adds concrete ordering per §3: lexical/structured + explicit as-of filter is always baseline; vectors/graph are candidates that must beat that baseline at acceptable cost before any replacement discussion.

## 5. Dossier completeness before any copy/port

Every future COPY/PORT/ADAPT must record (per `SOURCE_REUSE_ADAPTATION_MATRIX` §4 template):

```yaml
source_id: SRC-...
source_repository: owner/repo
source_commit: <pinned sha>
source_path: path/to/upstream
reuse_mode: COPY|PORT|ADAPT|TEST_VECTOR|REFERENCE|REJECT|DEFER
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
ponytail_justification: <text>
kill_criterion: <text>
```

Units marked REFERENCE still preserve exact revision/path in research notes so Ponytail has a real rather than imagined alternative.

## 6. What remains `STUDY/DEFER` and why

```text
Loro/Automerge/y-octo       = DEFER — no collaboration requirement yet (016 experiment owns)
Penpot/AFFiNE Edgeless/etc. = DEFER — no canvas requirement yet
Superset/etc.               = DEFER — no analytics requirement yet
Flint                       = DEFER — future view gate only
```

Any future use of these donors still requires a valid gap trigger per `ARCHITECTURE_FREEZE.md §12` (measured FTS failure, security finding, failed graph experiment, ratified collaboration requirement, editor gate — not chat popularity).

## 7. Current state

```text
FULL_DONOR_REUSE_REVIEW=COMPLETE
TENCENT_DONORS_DECISION_COVERAGE=17/17 units in prior matrix
EARLIER_DONORS_DECISION_COVERAGE=18/18 families reviewed above
NEW_CODE_IMPORT=NO
NEW_RUNTIME_DEPENDENCY=NO
RUST_SEMANTIC_OWNERSHIP=PRESERVED
PONYTAIL_GATE=REQUIRED_BEFORE_EVERY_REUSE
R1_CHANGED=NO
```

*End — no implementation authority claimed. Preserve as evidence.*

