# Fehrest V2 Future Plan Deep Architecture Review — 2026-09-08

**Status:** REVIEW / NON-AUTHORIZING / PLANNING ONLY
**Trigger:** founder request for independent architecture/product/technical planning review of PR #40 and its relationship to PR #2
**Execution effect:** NONE — does not mutate R1, does not activate Spec 002, does not merge PR #2 or PR #40, does not authorize implementation
**Canonical frontier:** live `specs/CURRENT.md` (R1_V2_PREREGISTRATION_REBUILD_COMPLETE_AWAITING_REVIEW)
**Inputs reviewed:**
```text
AGENTS.md
specs/CURRENT.md
docs/canonical/ARCHITECTURE_FREEZE.md
docs/canonical/EXECUTION_MASTER_PLAN.md
docs/canonical/FEHREST_EXECUTION_ROADMAP.md
docs/00-PRODUCT-THESIS.md
docs/01-ARCHITECTURE-CONSTITUTION.md
docs/02-THREAT-MODEL.md
docs/10-BENCHMARK-PLAN.md
docs/13-RECOVERY-MODEL.md
docs/20-FUTURE-GATES.md
PR #2  — docs/fehrest-founder-vision-v2 @ a99413d
PR #40 — docs/v2-tencent-source-plan-convergence @ b1ab8a0
  docs/proposals/V2_TENCENT_SOURCE_PLAN_AMENDMENT_2026-09-08.md
  docs/research/TENCENT_SOURCE_QUALIFICATION_2026-09-08.md
  docs/research/FOUNDER_SOURCE_USE_AUTHORIZATION_2026-09-08.md
  docs/research/SOURCE_REUSE_ADAPTATION_MATRIX_2026-09-08.md
  docs/reviews/V2_TENCENT_SOURCE_GAP_REVIEW_2026-09-08.md
  docs/reviews/V2_TENCENT_CODE_LEVEL_GAP_REVIEW_2026-09-08.md
PR #2 program docs:
  docs/proposals/spec-kit-v2/CROSS_SPEC_INVARIANTS_AND_OWNERSHIP.md (pr2)
  docs/proposals/spec-kit-v2/PROGRAM_BLUEPRINT.md (pr2)
  docs/proposals/spec-kit-v2/SPEC_SEQUENCE_AND_DEPENDENCIES.md (pr2)
  docs/proposals/spec-kit-v2/CONFLICT_AND_GAP_REVIEW.md (pr2)
  docs/proposals/spec-kit-v2/PROGRAM_CONVERGENCE_REVIEW.md (pr2)
Earlier donor registry: docs/research/FEHREST_SOURCE_REGISTRY.md (when present), prior donor set {Mem0, Letta, Graphiti, Chroma, Aider, Graphify, Code-Graph-RAG, Qdrant, LangGraph, LangChain, LlamaIndex, Firecrawl, etc.}
```

> This review is intentionally adversarial. It treats donor presence as hypothesis, not authority, and applies Ponytail/YAGNI aggressively. It does not praise the plan for being thorough; it searches for what will fail during implementation.

---

## 1. Executive verdict

```text
PLAN_GAPS_FOUND=51 total (28 prior TG + 23 new NG)
PRIOR_GAPS_REVIEWED=28/28
NEW_GAPS_IDENTIFIED=23
NEW_GAPS_WITH_OWNER_OR_GATE=23/23
OWNERSHIP_CONFLICTS_FOUND=3 (now resolved by reconciliation below)
KNOWN_DEPENDENCY_CYCLES=0
UNJUSTIFIED_MAJOR_SUBSYSTEMS=2 flagged for kill-criteria hardening
UNOWNED_SECURITY_CRITICAL_SEMANTICS=0 after this review (3 were unowned before)
UNOWNED_DURABILITY_CRITICAL_SEMANTICS=0 after this review (2 were unowned before)
SOURCE_REUSE_DECISIONS_EXPLICIT=PARTIAL_BEFORE (now explicit for full donor landscape)
BENCHMARK_KILL_RULES_EXPLICIT=PARTIAL_BEFORE (now explicit per family)
PRODUCT_PRIORITY_EXPLICIT=PARTIAL_BEFORE (now explicit MUST_WIN / SHOULD_HAVE / LATER / EXPERIMENTAL / REJECT)
R1_CHANGED=NO
R1_REVIEW_CANDIDATE_CHANGED=NO
IMPLEMENTATION_AUTHORIZED=NO
PR40_CANONICALIZATION=NO
```

**One-line verdict:** PR #40 is a strong convergence step and its 28 gaps are real, but it is not yet sufficient to canonicalize the future plan. It leaves open a second layer of lifecycle, identity, durability and product-priority semantics that will cause implementation failure if not owned before post-R1 authoring. This review closes that layer without adding implementation authority.

---

## 2. Methodology

Review proceeded as:

1. Re-read live canonical governance (AGENTS.md, CURRENT.md, Freeze, Execution Master Plan, Execution Roadmap) to establish hard invariants that no planning document may weaken.
2. Re-read PR #2 V2 program blueprint, cross-spec invariants, spec sequence, conflict and gap reviews to establish the proposed V2 spine and its ownership claims.
3. Re-read PR #40's six artifacts line-by-line, then re-inspected donor code semantics for RoMem, WeKnora, SkillHone at their pinned revisions plus the earlier donor registry (Mem0, Letta, Graphiti, etc.) to test whether 28 gaps are complete.
4. Performed a fresh first-principles gap search across the 20 audit dimensions named in the review brief, treating every proposed mechanism as guilty until proven necessary.
5. Constructed the future spec dependency DAG and checked for cycles, reverse dependencies, and shared ownership.
6. Classified each gap by smallest correct owner and by reuse/benchmark/governance effect.
7. Applied explicit REJECT/DEFER for donor-shaped features that lack Fehrest requirements.

No R1 artifact, bench/R1/** path, or canonical R1_V2* file was read as mutable.

---

## 3. Fresh gap search — what PR #40 gets right and what it still misses

### 3.1 Correctly owned by PR #40 (28 gaps — confirmed real)

TG-01 through TG-28 are confirmed as real semantic/security/durability issues, not donor-shaped feature requests. Each correctly identifies a place where the pre-convergence plan was implicit or split across specs. Examples that are particularly load-bearing:

- TG-01/TG-02/TG-15/TG-16 (temporal rank != truth, volatility authority, partial time, model checkpoint identity) — directly protects F-CORE-06/F-CORE-07.
- TG-05/TG-12/TG-23 (derived edit shadow state, generated wiki authority, artifact promotion) — protects CANONICAL != DERIVED.
- TG-06/TG-08/TG-13/TG-21/TG-22 (execution admission, queue != idempotency, sandbox liveness, generation fencing) — fixes a real overload of 007.
- TG-10/TG-27/TG-28 (held-out isolation, redaction lineage, optimizer attribution) — prevents evaluation self-deception.

**No prior gap is rejected.** The amendment's invariants P-TS-01..P-TS-19 and its proposal to split 007 into 007A/007B and gate 021 into 021A/021B are directionally correct (see corrections in §§5–6).

### 3.2 New gaps found (23)

Each new gap below is named NG-01..NG-23, with trigger, risk, and required owner. All are planning gaps, not implementation tasks.

#### NG-01 — Canonical persistence class for annotations and inline discussion

- **Trigger:** prior donor `CROSS_SPEC_INVARIANTS` distinguishes CANONICAL vs DERIVED_REBUILDABLE vs CACHE, but PR #40's TG-05/TG-12 do not explicitly classify inline annotations, threaded discussion, or task outcome notes that may live on top of a canonical document.
- **Risk:** an inline comment could be stored as a separate canonical object, as a derived annotation, or as ephemeral UI state with different retention; inconsistent choices produce a shadow source of truth.
- **Owner:** 010 workspace canonical objects must define ANNOTATION / DISCUSSION persistence class; 005/012 derived overlays remain DERIVED_REBUILDABLE.

#### NG-02 — Supersession key and conflicting timeline reconciliation

- **Trigger:** TG-20 warns false merge is worse than duplicates, but does not define how two sources that assert conflicting valid intervals for the same subject are deterministically resolved, nor how future-effective facts are handled.
- **Risk:** silent last-write-wins or normalized-key overwrite destroys incomparable claims.
- **Owner:** 006 — define supersession key as `(subject, scope, memory_kind, claim_id)` and define conflict state `CONFLICTED` vs `SUPERSEDED` vs `UNRESOLVED`, with deterministic `as_of` resolution.

#### NG-03 — Memory kind taxonomy remains donor-shaped

- **Trigger:** WeKnora/Letta/Mem0 each propose different memory taxonomies; PR #40 TG-03 says promotion must be type/risk sensitive but defers the Fehrest taxonomy to 006.
- **Risk:** inheriting donor taxonomy without Fehrest requirement justification; also missing `procedure`, `constraint`, `relationship` lifecycle differences.
- **Owner:** 006 — must define Fehrest's minimal taxonomy from first principles (fact, decision, constraint, preference, procedure, task/goal, profile/identity, relationship, interest, hypothesis, working memory) and justify each against product thesis; reject donor taxonomy otherwise.

#### NG-04 — Context compiler versioned identities and replay contract

- **Trigger:** PR #40's 007A correctly separates residency/recalled dispositions but does not fully bind `ContextPackageId`, `ContextPolicyVersion`, `RetrieverVersion`, `RankingVersion`, `CompilerVersion`, `MemorySnapshotId`, `SourceSnapshotIds`, `TokenizerVersion`.
- **Risk:** non-reproducible context, un-debuggable rank drift, non-auditable receipts.
- **Owner:** 007A — every receipt/manifest must bind these identities; replay contract must define IDENTICAL / DIVERGED / UNRECONSTRUCTABLE (already in Roadmap §10) and make it testable.

#### NG-05 — Token accounting determinism under multiple tokenizers

- **Trigger:** Roadmap §5 budget model mentions hard-byte safety ceiling + pinned tokenizer, but PR #40 does not own which tokenizer versions are canonical, how budget truncation is recorded, or how re-tokenization drift is detected.
- **Risk:** token budget becomes non-deterministic across model providers.
- **Owner:** 007A — own tokenizer identity, version pins, byte-vs-token budget semantics, truncation artifact linkage.

#### NG-06 — Cancellation vs termination ownership in delegated execution

- **Trigger:** TG-22/TG-26 mention timeout/termination but prior plan conflates `CLIENT_CANCELLATION` with `PROCESS_TERMINATION` / `PROVIDER_TERMINATION`.
- **Risk:** cancellation treated as proof of no side effect, enabling unsafe retry.
- **Owner:** 007B — define cancellation token lifecycle, explicit termination verification, and that `CANCELLED != TERMINATED` and `TIMEOUT != NO_SIDE_EFFECT`.

#### NG-07 — Capability lease attenuation and recursive delegation

- **Trigger:** Architecture Freeze F-CORE-11 and Roadmap §10 CapabilityLease define attenuation but PR #40 does not fully own recursive delegation depth, least-privilege narrowing per dimension, or sub-agent grant validation.
- **Risk:** confused-deputy, privilege escalation via delegated grant widening.
- **Owner:** 007B — formal attenuation proof: every descendant lease is monotonic narrowing across all dimensions (tool, scope, filesystem, network, credential, budget, time) with atomic admission check.

#### NG-08 — Service/API credential lifecycle and generation fencing

- **Trigger:** TG-07 notes external credential != principal, but lifecycle of rotation/revocation/expiry and its interaction with generation fencing (TG-22) is not fully owned.
- **Risk:** rotated credential remains valid in long-lived sandbox; service account outlives org policy.
- **Owner:** 007A baseline mapping + 018 organization extension + 007B generation fencing; every side-effect must bind `grant_generation`, `policy_generation`, `credential_generation`.

#### NG-09 — External evidence mutable/deleted/duplicate and citation lineage staleness

- **Trigger:** TG-24 covers DNS rebinding/private networks, but not mutable pages, deleted pages, duplicate sources, stale evidence, or citation lineage decay.
- **Risk:** stale or vanished evidence silently remains authoritative; provenance chain breaks when source mutates.
- **Owner:** 014 — own snapshot identity `(source_id, retrieval_time, content_hash, redirect_chain)`, freshness TTL, staleness receipt, lineage graph, and rule that external evidence never promotes to canonical without explicit promotion.

#### NG-10 — Retrieval fusion formula and per-stage baseline/kill criteria incompleteness

- **Trigger:** 004 comparator ladder in PR #40 amendment lists fusion without requiring per-workload simple baselines; B-TEMP-RANK etc. lack workload-specific kill/retention thresholds.
- **Risk:** advanced retrieval retained because it beats a weak baseline, or rejected because workload was wrong.
- **Owner:** 003 baseline retrieval + 004 experiment — each workload must pre-register baseline (lexical + structured/temporal), candidate, metric, cost, kill criterion; no composite umbrella score.

#### NG-11 — Full donor landscape not yet in reuse matrix

- **Trigger:** PR #40 matrix covers only 3 Tencent donors; earlier donors (Mem0, Letta, Graphiti, Chroma, Aider, Graphify, Code-Graph-RAG, Qdrant, LangGraph, LlamaIndex, Firecrawl) remain in `CROSS_SPEC_INVARIANTS` and freeze donor freeze list but lack per-unit COPY/PORT/ADAPT/TEST_VECTOR/REFERENCE/REJECT with Rust-ownership justification.
- **Risk:** stale donor assumption re-enters via PR #2's broad donor list.
- **Owner:** program-level reuse dossier (extend matrix) — each donor unit must be decided under Ponytail + security + benchmark before admission; donor freeze remains unless gap-driven and authorized.

#### NG-12 — 021 split decision gate remains ambiguous

- **Trigger:** PR #40 correctly proposes authoring-time check for 021A/021B split but does not define decision criterion strongly enough to prevent one large spec from coupling runtime and evaluation concerns.
- **Risk:** extension runtime and skill evaluation become one deployment/security boundary with shared privilege.
- **Owner:** 021 authoring gate — split iff runtime and evaluation cannot remain independently testable, independently releasable, and independently privilege-bounded; evaluation isolation requires separate filesystem/capability boundary.

#### NG-13 — Hidden local-first portability dependencies

- **Trigger:** PR #40 identifies Redis/provider-handle != authority and similar, but does not audit per-spec hidden assumptions requiring GitHub cloud, PostgreSQL, Qdrant/Chroma, model API, or Linux-only filesystem semantics.
- **Risk:** offline/single-user/portable project breaks when derived store unavailable.
- **Owner:** CROSS_SPEC_INVARIANT + each spec's Offline behavior section — every spec must enumerate required services and prove `NETWORK=OFF`, `AI=OFF`, `SYNC=UNAVAILABLE` still satisfies core correctness; derived corruption never becomes canonical loss.

#### NG-14 — Recovery: rebuild equivalence per derived store not yet contracted

- **Trigger:** TG-17 warns retention != forgetting, and Execution Master Plan requires canonical loss = 0 under fault matrix, but per-store incremental-vs-clean equivalence and rebuild identity recording are not yet owned as test contracts.
- **Risk:** index/graph/cache corruption silently diverges.
- **Owner:** 002 (event journal recovery) + 003 (derivation registry, projection checkpoints, incremental-vs-clean equivalence proof) + 005 if retained.

#### NG-15 — Schema/format forward/backward compatibility policy

- **Trigger:** 002 owns vault identity/format/version envelope but full forward migration, backward compatibility, unknown-field handling, extension fields, downgrade behavior, migration receipts/rollback not yet canonically enumerated in future plan.
- **Risk:** long-lived vault cannot be opened by newer/older Fehrest, or silently drops fields.
- **Owner:** 002 + CROSS_SPEC_INVARIANT I-14 — every canonical format must declare schema version, forward migration, backward read policy, unknown-field handling, export compatibility, migration receipt, rollback.

#### NG-16 — Observability without second source of truth

- **Trigger:** Roadmap §5 SelectionTrace and PR #40 residency dispositions aim to explain why memory was selected/excluded, but risk is that observability store becomes de facto authority or leaks secrets.
- **Risk:** derived trace store diverges from canonical receipt; secret material enters logs.
- **Owner:** 007A — SelectionTrace is derived evidence reconstructible from receipt/manifest + derived-generation bindings; permanent receipt/manifest is canonical audit evidence; no secret bytes in trace/logs.

#### NG-17 — Privacy and data governance per-visibility classification

- **Trigger:** PR #40 P-TS-06/P-TS-09 mention privacy/staleness, but not explicit classification of local-only vs exportable vs sensitive vs secret vs provider-visible vs model-visible vs logged.
- **Risk:** context minimization failure; secret or sensitive memory exposed to model/provider/log.
- **Owner:** CROSS_SPEC_INVARIANT + 006 (memory sensitivity) + 007A (context minimization, redaction, scope assertion) + 014 (provider-visible evidence).

#### NG-18 — Performance and resource budgets without hard SLOs

- **Trigger:** PR #40 B-* families list latency/tokens/cost/footprint as measurements but no hard per-spec budget is proposed, and large-repo/large-memory stress not an explicit benchmark requirement.
- **Risk:** feasible at demo scale, impractical at scale.
- **Owner:** each spec's plan must state hard budgets (startup, compile latency, retrieval latency, index rebuild, memory/disk footprint, token use, sandbox startup, background CPU) and include large-repo/many-memory stress fixtures.

#### NG-19 — Developer experience first-five-minutes journey not acceptance-gated

- **Trigger:** PR #2 UX blueprint covers end-to-end lifecycle, but developer's first five minutes (import/init/inspect/status/doctor/context/search/diff/explain) is not an explicit Spec 002/003 acceptance criterion.
- **Risk:** architecture complete, product still unusable.
- **Owner:** 002 quickstart + 003 search quickstart + 007 CLI/SDK quickstart — each must have a 5-minute doctorable journey with `fehrest doctor` / `fehrest status` style observability.

#### NG-20 — Testing strategy missing fault-injection and cross-platform matrix

- **Trigger:** Program blueprint lists test pyramid genres but not per-spec minimums for property/fault-injection/replay/concurrency/security/golden-fixture/cross-platform/migration/performance.
- **Risk:** security/boundary tests invented ad hoc.
- **Owner:** CROSS_SPEC_INVARIANT (SPEC_AUTHORING_CHECKLIST) + each spec's verification.md — mandatory adversarial corpora C-INJECT/C-PATH/C-MALFORMED/C-POISON/C-TAMPER per threat model.

#### NG-21 — Benchmark family proliferation vs minimality

- **Trigger:** PR #40 amendment proposes 8 new benchmark families B-TIME through B-SKILL-EVOLVE in addition to existing plan's B-0..B-13; overlap with GI-CAP, B-TIME, B-7 etc. not reconciled.
- **Risk:** one giant aggregate product score, or benchmark busywork that obscures thesis-critical measurement.
- **Owner:** benchmark plan owner (10-BENCHMARK-PLAN.md future revision) — merge/retain only families that ask distinct falsifiable questions; each family binds question/baseline/candidate/dataset/metrics/cost/kill rule.

#### NG-22 — Dependency spine enforcement for 007A/007B split

- **Trigger:** PR #40 correctly states READ_ONLY_CONTEXT_DOES_NOT_DEPEND_ON_EXECUTOR_INFRA=YES, but PR #2's spec sequence still shows 007 as monolithic dependency for 008/013/014 and does not enforce that 008/013-read can proceed on 007A alone.
- **Risk:** hidden reverse dependency re-introduces executor infra as prerequisite for reads.
- **Owner:** SPEC_SEQUENCE_AND_DEPENDENCIES — enforce `008 -> 007A`, `013-read -> 007A`, `013-actions/014-actions/021 -> 007B`.

#### NG-23 — Product value dilution identification

- **Trigger:** PR #2 vision is intentionally broad (Obsidian + Notion + Slack + Linear + GitHub + agents) while product thesis demands portable memory as falsifiable core.
- **Risk:** donor features dilute Fehrest identity; V2 becomes impressive but not indispensable.
- **Owner:** 00-PRODUCT-THESIS + program blueprint — explicitly separate MUST_WIN vs SHOULD_HAVE vs LATER vs EXPERIMENTAL vs REJECT; see §13.

---

## 4. Dependency / DAG review

### 4.1 Reconstructed future DAG after PR #40 corrections

```text
R1 (terminal verdict)
 |
 +-> G-PROV (recovered) + G-CONST (reconciled) + G-V2 (founder decision)
          |
         002  canonical core (writer-owned, journal, recovery, schema upcast)
          |
         003  derived index / lexical retrieval (incremental, rebuild equivalence, trace baseline)
          |
         004  graph/temporal intelligence CAPABILITY EXPERIMENT (preregistered, decision only)
          |
     RETAIN? --YES--> 005 conditional derived provider (only if 004 RETAIN_NOW and downstream needs it)
          |                |
          +------>---------+
                   |
                  006  temporal memory (bitemporal, kind taxonomy, promotion matrix, tombstones, cursor idempotency)
                   |
          +--------+--------+
          |                 |
        007A                007B
 Universal Context    Delegated Execution, Fencing & Receipt
 & Memory Gateway     (admission, attempt, fencing, indemnity)
          |                 |
          |  007A alone     |  007B required for side effects
          v                 v
         008  GitHub link & IDE discovery (007A)
          |
         009  trusted vertical memory proof (007A; 007B only if delegated side effects in proof)
          |
         010  workspace canonical objects & open-format foundation
          |
         011  personal notes/docs/capture
          |
         012  search / graph exploration / bases UX  (needs 003 + 010 + 011; may overlay 005)
          |
         013  AI provider runtime & Ask Fehrest (reads via 007A; actions via 007B)
          |
         014  external evidence & WebMCP (reads via 007A; actions via 007B)
          |
         015  import/migration lab (needs 010 stable)
          |
         016  collaboration capability experiment (Automerge/Yjs/Loro etc. retain/defer/reject)
          |
     RETAIN? --YES--> 017 sync/multi-writer (016 RETAIN only)
                        |
                       018  org identity/policy/admin (needs 017 + security approval)
                        |
                       019  team communication/shared workspace (needs 017 + 018)
                        |
                       020  mobile/offline capture (needs 017)
                        |
                       021  extension/automation/connector platform
                        |     gate decides 021A runtime vs 021B evaluation/evolution split
                        |
                       022  hub/network (only after personal/team local-first proof)
```

### 4.2 Cycle and hidden-dependency audit

```text
CYCLE_CHECK=PASS  (no cycles detected)
HIDDEN_REVERSE_DEPENDENCY_CHECK:
  002 -> 007A/007B  = correct (core before gateway)
  007A -> 007B       = CORRECTLY ABSENT (PR #40's key fix)
  007B -> 007A       = ABSENT (execution may consume context but not own it)
  010 -> 003        = ABSENT (workspace objects do not define retrieval engine)
  012 -> 005        = CONDITIONAL overlay only (visualization != provider)
  013-actions -> 007B = REQUIRED (enforced)
  014-actions -> 007B = REQUIRED (enforced)
  021 -> 007B       = REQUIRED (enforced)
  006 -> 005        = ABSENT (memory does not depend on optional graph provider)
VERIFICATION: all higher product surfaces depend on lower canonical/retrieval/memory/gateway layers, never the reverse.
```

### 4.3 Shared semantic ownership after reconciliation

```text
NO_OWNER_CONFLICT_AFTER_RECONCILIATION=YES
100% of canonical entities have one owner (see §5)
```

---

## 5. Ownership conflicts — found and reconciled

### OC-01 — 007 overloaded vs split

- **Conflict:** PR #40 proposes 007A (read/context) vs 007B (execution/fencing) but PR #2's CROSS_SPEC_INVARIANTS still assigns `principal/session/grant`, `Context Compiler`, `authorization chokepoint`, and execution-admission-like semantics all to single 007.
- **Resolution:** Accept PR #40 split as valid Class C architecture-semantic correction during post-R1 canonicalization. Update CROSS_SPEC_INVARIANTS_AND_OWNERSHIP.md to reflect:
  - 007A owns grants for reads, Context Compiler, scope, budgeting, package/receipt/SelectionTrace, read-only CLI/SDK;
  - 007B owns side-effect admission, attempt/fencing/reconciliation, JIT secrets, network envelope, artifact ingress/egress;
  - consumers re-bound per §4.2.
- **Gap assignment:** CROSS_SPEC_INVARIANT (requires ADR at canonicalization) + NEW_AUTHORING_SPLIT (007A/007B).

### OC-02 — Skill lifecycle owner ambiguous between 021 and 007B

- **Conflict:** PR #40 assigns sandbox lifecycle to 007B, skill install lifecycle to 021, and supply-chain bundle digest/rollback to both 021/007B without crisp boundary.
- **Resolution:** Boundary is privileged operation vs privileged execution:
  - 021 owns SkillPackage/SkillRevision/Bundle/SBOM/advisory/install transaction/activation pointer/rollback/release state;
  - 007B owns execution admission/fencing/receipt for every side-effect including install execution;
  - install-time privilege separation is co-owned contract between 021 (what is installed) and 007B (how it executes).
- **Gap assignment:** CROSS_SPEC_INVARIANT + NEW_AUTHORING_SPLIT (021A/021B gate).

### OC-03 — Memory promotion vs confirmation vs retrieval residency ownership

- **Conflict:** TG-03 assigns promotion matrix to 006, TG-04 assigns residency to 006+007A; cross-spec matrix could duplicate.
- **Resolution:** 006 owns memory kind taxonomy, influence classification, evidence/corroboration requirements, lifecycle state machine, supersession/retraction rules, tombstone; 007A owns residency policy (resident vs recalled), retrieval ranking, budget, residency receipt dispositions. No duplication.
- **Gap assignment:** CROSS_SPEC_INVARIANT clarification.

No unresolved ownership conflicts remain if the above reconciliation is landed during post-R1 canonicalization.

---

## 6. Over-design findings (Ponytail / YAGNI)

| # | Finding | Severity | Recommendation |
|---|---|---|---|
| OD-01 | **8 new benchmark families before baseline retrieval proven.** B-TIME, B-TEMP-RANK, B-MEM-PROMOTE, B-MEM-PIPE, B-CONTEXT, B-EXEC, B-SANDBOX, B-SKILL-EVOLVE overlap prior families (GI-CAP B-13, B-7 continuation, B-3 retrieval). Measuring all 8 before 003 passes risks benchmark busywork obscuring thesis-critical signal. | MEDIUM | Keep families as planning hypotheses, but gate execution: only B-TIME and B-MEM-PROMOTE plus incremental-vs-clean equivalence are mandatory before 009; the rest are CONDITIONAL on 004/006/007B/021 being retained. Merge B-TEMP-RANK into 004's experiment; merge B-SANDBOX/B-EXEC criteria into 007B's security/recovery verification rather than standalone suites. Each retained family must have explicit kill rule. |
| OD-02 | **19 new invariants (P-TS-01..19) duplicate existing I-01..I-19 and Freeze F-CORE-01..17.** Several P-TS are restatements of I-06, I-12, F-CORE-06 etc. at finer granularity. If both sets survive as canonical, divergence risk. | LOW | During canonicalization, fold P-TS into owning spec invariants and retire the temporary P-TS numbering. Cross-spec invariants document owns the merged set; do not keep two parallel invariant enumerations. |
| OD-03 | **Splitting 007 eagerly adds authoring overhead if single spec could modularize internally.** Two specs mean two entry gates, two plan.md, two verification.md. | LOW | Retained as correct: the split is justified because read-only context is independently valuable, independently testable, has lower privilege than execution, and must remain network/offline-capable while execution needs sandbox/network. Benefit outweighs overhead; document decision in ADR. |
| OD-04 | **RoMem Python KGE provider conditional path creates speculative non-Rust provider boundary before 004 measures need.** | LOW | Correctly gated as CONDITIONAL in PR #40 (only if 004 proves material need + provider boundary authorized). Enforce that no spec mentions KGE implementation detail until 004 pre-registration freezes. Keep test-vector/port path only. |
| OD-05 | **WeKnora full-stack patterns (Redis, Docker root, path-prefix checks) could contaminate invariants if copied wholesale.** PR #40 correctly flags them as non-adoptions but does not yet place them in a REJECT list with rationale. | LOW | Add explicit REJECT list to reuse matrix (see §11). |
| OD-06 | **Video/media/sync server infra implied by team vision not needed for P1 thesis proof.** PR #2's product vision includes media capture; turning it into infrastructure prematurely is over-design. | MEDIUM | Per SPEC_SEQUENCE, media remains provider/integrate, not core, until measured requirement exists. |

No unjustified major subsystem is added by PR #40; the above are hardening corrections, not rejections of the 28 gaps.

---

## 7. Missing lifecycle semantics — closure

All missing semantics are now owned; none remain vague future work:

| Lifecycle family | Missing before | Owner after this review | Key contract |
|---|---|---|---|
| Memory taxonomy (fact/decision/constraint/preference/procedure/task/goal/profile/interest/relationship/hypothesis/working memory) | generic promotion only | 006 | kind + influence + evidence + corroboration + human-confirmation + revalidation + supersession + retraction matrix |
| Bitemporal precision (event_time, observation_time, recorded_at, valid_from/to, superseded_at, query/as_of, source_pub_time, source_retrieval, precision, timezone/locale, source expression, reference time) | parser normalization gap | 006 + 003/007A consumers | PARTIAL_TIME != EXACT_TIME; RELATIVE requires reference_time; every time field carries precision and source expression |
| Current vs as-of resolution, retroactive correction, future-effective facts, late-arriving evidence, superseded-but-historically-valid | implicit | 006 | deterministic resolution with CONFLICTED/UNRESOLVED states; historical access remains honest |
| Resident vs recalled vs excluded context | not explicit | 007A | residency policy + receipt dispositions RESIDENT/RECALLED_ON_DEMAND/EXCLUDED_{STALE,SCOPE,BUDGET,CONFLICTED,POLICY} |
| Proposal extraction durability (cursor, batch id, idempotency key, source range, digest, at-least-once input, idempotent creation, cursor-after-commit, replay safe) | idempotency only partially owned | 006 + 007B if background execution | durable cursor + deduplication + no source-range loss |
| Rejection tombstones for rejected inferences | weak | 006 | scoped tombstone prevents same unsupported guess recurring without new evidence |
| Canonical object lifecycle (GENERATED_DRAFT -> DERIVED_REBUILDABLE_VIEW -> CANONICAL_ANNOTATION/PROPOSAL -> USER_SAVED_CANONICAL_DOCUMENT -> MEMORY_PROPOSAL -> APPROVED_MEMORY) | wiki authority gap | 010 + 006 | explicit transition; durable user edit always maps to canonical writer/proposal owner |
| Extension/skill lifecycle (manifest, package, revision, provenance, bundle digest, SBOM, activation pointer, rollback) | not yet specified | 021A (runtime) + 021B (evolution) | install separates from runtime privilege |
| External evidence lifecycle (snapshot identity, freshness/recheck, staleness receipt, duplicate detection, lineage) | DNS only partially covered | 014 | snapshot = (source_id, retrieval_time, content_hash, redirect_chain, DNS evidence) |
| Schema/format lifecycle (version, forward migration, backward compat, unknown fields, migration receipt, rollback) | envelope only | 002 + invariants | per §8 |

---

## 8. Security / threat-model findings

```text
SEPARATION_PASS=YES  (identity/authn/authz/roles/capabilities/leases/scopes/generations/resources/secrets/network/fs/tools are distinct)
ATTESTATION: reviewed against 02-THREAT-MODEL T-1..T-4 and Freeze §5 negative claims
```

| # | Finding | Owner | Required control |
|---|---|---|---|
| S-01 | **Stale sandbox outliving grant/policy/credential generation.** TG-13/TG-21/TG-22 already identify, but NG-08 adds credential generation. | 007B | Every side-effect re-validates `grant_generation + policy_generation + credential_generation` atomically before dispatch; expiry is generation-scoped, not wall-clock alone; revocation invalidates existing handles. |
| S-02 | **Installer privilege vs runtime privilege conflation for skills.** TG-25 correctly separates but not yet a spec-level hard gate. | 021/007B | Co-owned contract: installer capability (`network/filesystem/secret/mount`) strictly greater-than runtime is never inherited; install receipt records builder image digest and SBOM. Root execution must be justified per active spec, never default. |
| S-03 | **Recursive delegation attenuation not mechanically proven.** Roadmap §10 states attenuation but not per-dimension monotonic proof. | 007B | Descendant lease must narrow every dimension; admission verifies complete ancestor chain live and containing; deny if executor cannot enforce any requested restriction. |
| S-04 | **Confused-deputy via tool description or web content widening scope.** Existing invariant I-05 + THREAT T-1 covers, but external evidence spec needs explicit enforcement. | 014 + 007B | Tool description = untrusted content (never authority); web tool classification requires explicit grant/confirmation; network policy revalidated at connection time with DNS/redirect evidence. |
| S-05 | **Secret scope: JIT injection vs model visibility vs log visibility.** TG-22 mentions JIT, but log/diagnostic leakage not fully owned. | 007B + CROSS_SPEC_INVARIANT I-09 | Raw secrets never enter model-visible context, memory, trajectories, event detail, or logs; only opaque credential references may appear; injection occurs only inside qualified executor boundary. |
| S-06 | **Held-out evaluation boundary is capability + process, not prompt.** TG-10/TG-27 correctly identify but need filesystem/capability isolation proof. | 021B | Privately held gold requires separate filesystem/process/capability boundary with leak tests; optimizer cannot mint final-test access; optimizer improvement != merge authority. |
| S-07 | **Path confinement: prefix check insufficient; TOCTOU and symlink attacks need at-use resolution.** TG-23 identifies but must become spec-level adversarial test. | 007B + 010 | Require `PATH_RESOLUTION_AT_USE + NO_FOLLOW_OR_EQUIVALENT + intermediate_symlink + TOCTOU` adversarial cases; never prefix-only. |
| S-08 | **Browser session authority and credential leakage in web context.** Not yet explicit in PR #40. | 014 | Browser/WebMCP session scoped to principal/grant; credential leakage tests; SSRF/private-network denial at connection time. |

All are assigned; no unowned security-critical semantic remains after correction.

---

## 9. Recovery and durability findings

| # | Finding | Owner | Contract |
|---|---|---|---|
| R-01 | **Canonical loss under fault matrix must be 0, but derived stores need explicit equivalence proofs.** PR #40 focuses on canonicity of temporal truth; rebuild equivalence per derived store not fully contracted. | 002 (journal) + 003 (indexes) + 005 (graph if retained) | Crash matrix: process crash, machine crash, disk full, partial write, corrupt index, missing derived, stale cache — each must define reconstruction from canonical; incremental-vs-clean equivalence test required. |
| R-02 | **Missing durability for extraction cursor advancement.** TG-18 identifies crash-before/after cursor, but not concurrent extraction or clock skew interaction. | 006 | Require durable cursor, at-least-once input, idempotent proposal creation, cursor-after-commit, concurrent extraction serialization, clock-skew invariant (monotonic commit ordering independent of wall clock). |
| R-03 | **Provider outage / indeterminate execution / duplicate delivery not fully owned as recovery state machine.** PR #40 correctly identifies indeterminate execution but not full reconciliation taxonomy. | 007B | Roadmap §10 durable lifecycle `PREPARED -> DISPATCHED (durable intent) -> STARTED -> terminal receipt` with `INDETERMINATE` reconciliation appending evidence; failed status never proves safe retry without side-effect disposition. |
| R-04 | **Schema migration failure and downgrade not yet owned as recovery case.** | 002 | Require golden old-version fixtures, upcasting skeleton, migration receipt/rollback, unknown-field preservation, explicit downgrade behavior (fail-visible, never silent drop). |
| R-05 | **Repository/branch rewrite, workspace move/relocation, Git history loss.** | 002 + 010 | Project identity = stable UUID, path = location; move/relocation preserves identity; Vet/verify provenance after detached-Git scenarios; export/backup/restore proof. |

No unowned durability-critical semantic remains after assignment.

---

## 10. Performance and resource risks

| # | Risk | Why load-bearing | Required budget / mitigant |
|---|---|---|---|
| P-01 | **Context compile latency under large memory/repo.** Deterministic state resolution + lexical + temporal filter + budget assembly could exceed interactive latency if not bounded. | User-visible per-request path | 007A must state p50/p95 compile latency budget, early-scope filtering before retrieval, derivation registry avoiding full scan, incremental index. Stress fixture: N=100k memories + 10k docs. |
| P-02 | **Index rebuild time and disk footprint at scale.** Content-hash incremental vs fresh rebuild cost not yet quantified. | Derived correctness gate before UX | 003 must benchmark rebuild time, incremental update cost, memory/disk footprint; define invalidation completeness proof. |
| P-03 | **Graph build cost if retained.** Graphify extraction throughput measured but not production-bound. | Optional capability decision | 004 must measure build time, incremental cost, footprint, tokens, API cost; retain only if materially beats lexical at acceptable cost. |
| P-04 | **Token/cost budget explosion from resident memory.** Residency policy without hard budget becomes token sink. | Cost + privacy | 007A must enforce atomic budget: hard-byte ceiling + tokenizer-accurate accounting; resident set size policy with staleness TTL. |
| P-05 | **Sandbox startup latency and background CPU for delegated execution.** Persistent sessions reduce startup but increase stale-resource risk. | Agent execution path | 007B must budget sandbox creation/startup, lifecycle, reclamation, and prove fencing cost does not dominate. |
| P-06 | **Large-repo watcher/debounce overhead.** Filesystem watcher on 100k+ files. | Derived freshness | 003 must define debounce, reconciliation scan interval, resumable/cancellable rebuild, and prove no event loss under bursts. |

No hidden hard-coded cloud/DB assumption is acceptable as implicit requirement; every spec must declare required services and prove offline completeness (see NG-13).

---

## 11. Source-reuse corrections — full donor landscape

### 11.1 Prior state

PR #40's `SOURCE_REUSE_ADAPTATION_MATRIX` correctly applies `SMALLEST_CORRECT_REUSE_MODE_WINS` and records RoMem/WeKnora/SkillHone per-unit modes. It is incomplete in one dimension: the earlier donor registry (Mem0, Letta, Graphiti/Graphiti-temporal, Chroma, Aider/repo-map, Graphify, Code-Graph-RAG, Qdrant, LangGraph, LangChain, LlamaIndex, Firecrawl, DeepSeek Harness, OpenSandbox, etc.) is still referenced by `CROSS_SPEC_INVARIANTS` and `SPEC_SEQUENCE` as comparator/option candidates but lacks per-unit reuse decisions.

### 11.2 Extended decisions (amendment to matrix — non-authorizing)

| Donor family | Candidate unit | Proposed mode | Rationale | Owner | Gate |
|---|---|---|---|---|---|
| **Mem0** | Memory lifecycle/promotion tests, async memory update patterns | PORT + TEST_VECTOR | Valuable test vectors for promotion safety; Python runtime not canonical semantic owner | 006 | Ponytail + security + B-MEM-PROMOTE |
| **Letta Code / MemGPT** | Agentic memory tool loop, memory function schemas | REFERENCE | Agent-tool-loop pattern is study-only; Fehrest's grant model differs fundamentally | 006/007A | REFERENCE only unless benchmark proves narrower reuse |
| **Graphiti** | Temporal-context graph construction, episodic reasoning patterns | REFERENCE + ADAPT (conditional) | Temporal hypothesis relevant but graph semantics differs; only if 004 experiment retains graph | 004/005 | 004 decision gate |
| **Chroma / Qdrant** | Vector store index patterns, HNSW tuning | REFERENCE + TEST_VECTOR (conditional) | Vectors deferred per ADR-0007; lexical baseline must win/lose first; no vector default | 003/004 | DEFER until lexical inadequacy proven |
| **Aider repo-map** | Repo-map generation, symbol ranking, context ranking baseline | PORT / TEST_VECTOR | Strong simple baseline per Execution Master Plan §6 — mandatory comparator for code/project workloads | 003 | benchmark harness |
| **Graphify** | Deterministic relationship extraction pipeline | ADAPT + provider boundary | Candidate replacement space per ADR-0003; Python sidecar only, Rust-owned interface | 004/005 | 004 RETAIN only |
| **Code-Graph-RAG** | Graph construction provenance, evaluation harness | TEST_VECTOR + REFERENCE | Strong provenance/method reference; reuse evaluation shape not stack | 004 | 004 benchmark |
| **LlamaIndex / LangChain / LangGraph** | Tool/provider abstraction, adapter patterns | REFERENCE | Provider abstraction study; Fehrest owns provider boundary — do not copy framework assumptions | 013/014 | REFERENCE |
| **Firecrawl / web donors** | Web extraction, source snapshot, provenance binding | ADAPT (conditional) | Useful patterns for 014 acquisition but must pass SSRF/DNS/private-network hardening; no wholesale import | 014 | 014 security review |
| **DeepSeek Harness** | Event model, receipted context principle, approval-via-branded-identifier | TEST_VECTOR + REFERENCE | Principle `MODEL_VISIBLE_FEHREST_INPUT => RECEIPTED` is P1; harness event taxonomy useful but harness sandbox network model is not Fehrest's boundary | 002/007A | 007A receipt contract |
| **OpenSandbox / E2B / Daytona** | Egress policy, credential injection, isolation provider | REFERENCE + provider ADAPT (conditional) | Provider integration only; Fehrest never builds own sandbox platform per plan | 007B | provider boundary decision |
| **RoMem** (Tencent) | time_utils fixtures, temporal score fusion, reranker comparator | TEST_VECTOR / PORT / conditional ADAPT | As correctly decided in PR #40 — keep conditional on 004 evidence | 006/004 | 004 benchmark |
| **WeKnora** (Tencent) | pending/confirm/reject tests, cursor, subject isolation, sandbox lifecycle, installer/runtime separation, activation pointer, web snapshot waiter | PORT / ADAPT (selective) | PR #40's 8 high-value pattern selections confirmed — adopt only smallest correct unit, reject weak/default assumptions | 006/007A/007B/014/021 | per-unit dossier |
| **SkillHone** (Tencent) | split isolation, redaction, score provenance, diagnosis loop | COPY/ADAPT for dev tooling (021B) | Confirmed — constrained to dev/eval tooling under 021B; never promotes to product authority | 021B | held-out leak tests |

### 11.3 Explicit non-adoptions (reinforced)

```text
VECTOR_DEFAULT=NO
GRAPH_PRODUCTION_INTEGRATION_BEFORE_FALSIFICATION_TEST=NO
CRDT_BEFORE_COLLABORATION_REQUIREMENT=NO
WEKNORA_REDIS_AS_AUTHORITY=NO
WEKNORA_PATH_PREFIX_AS_SECURITY_BOUNDARY=NO
WEKNORA_ROOT_SANDBOX_AS_DEFAULT=NO
ROMEM_PYTHON_KGE_RUNTIME_AS_CANONICAL_DEPENDENCY=NO
SKILLHONE_AUTO_OPTIMIZER_AS_PRODUCT_AUTHORITY=NO
SKILLHONE_EXAMPLE_SCORE_THRESHOLDS_AS_FEHREST_THRESHOLDS=NO
HOSTED_HUB_AS_ONLY_CANONICAL_COPY=NO
```

No current code import or runtime dependency is admitted. Every future reuse still passes requirement → Ponytail → exact path/revision → reuse dossier → license/direct-permission/attribution → third-party review → security/benchmark → active-spec authority.

---

## 12. Benchmark corrections

### 12.1 Existing benchmark plan retains primacy

The canonical benchmark plan's thesis-proof ladder (B-7 fresh-agent continuation etc.) remains the defining product gate, not any of the 8 new families. PR #40's families are hypotheses to be tested, not substitutes for thesis proof.

### 12.2 Consolidated families (merges / retains)

| Family | Disposition | Question | Simple baseline | Why distinct |
|---|---|---|---|---|
| **B-TIME** | RECONCILE with existing temporal correctness obligations | Does bitemporal precision + as-of truth return correct current/past state? | canonical time semantics + lexical | NG-02 distinct question — retain but fold into 006 and 010 acceptance |
| **B-TEMP-RANK** | MERGE into 004 experiment | Does temporal/graph reranking materially improve continuation/temporal correctness? | strong explicit-time + lexical/structured baseline | not standalone — use 004's preregistered kill/retain |
| **B-MEM-PROMOTE** | RETAIN | Does type/risk-sensitive promotion reduce false durable memory vs generic auto-promotion? | single generic promotion rule | P1 memory safety gate |
| **B-MEM-PIPE** | RETAIN but narrow | Does durable cursor/idempotency prevent proposal loss/dup under crash/replay? | no-cursor replay | durability contract before memory trust |
| **B-CONTEXT** | MERGE into 007A verification | Does residency + influence policy preserve quality at lower token/privacy exposure? | all-active-memory resident | residency is 007A policy decision, not separate suite |
| **B-EXEC** | MERGE into 007B verification | Does generation-fenced idempotent retry correctly handle crash/timeout/revocation/indeterminate? | unfenced queue retry | security/correctness, not benchmark |
| **B-SANDBOX** | MERGE into 007B security verification | Do TOCTOU/DNS/private-network/installer-vs-runtime boundaries hold under adversarial inputs? | no special hardening | already covered by kill tests 11-SECURITY-VERIFICATION-PLAN |
| **B-SKILL-EVOLVE** | RETAIN under 021B | Does held-out isolation prevent leakage vs naive optimizer? | optimizer sees gold | evaluation integrity gate |

### 12.3 Per-family kill/retention rules (mandatory before execution)

Every retained family must pre-register:

```text
question
candidate
simple baseline (the bar that matters)
fixed dataset/fixture identity (pinned revision)
primary metrics (correctness/precision/recall where applicable)
cost metrics (latency/tokens/API cost/footprint where applicable)
failure conditions (not just score — resource exhaustion, provider mismatch, revocation)
kill rule (when to RETAIN / DEFER / REJECT — fixed before results)
retention rule (if retained, what interface + replacement criterion)
```

No aggregate product score. No post-hoc alpha selection (per NG-10). No composite that hides a weak sub-measure.

---

## 13. Product-priority corrections — what makes Fehrest indispensable

### 13.1Indispensability test

> What can Fehrest do that a competent agent with files + BM25 + Aider repo-map + a maintained Markdown wiki cannot trivially reproduce?

PR #2's vision breadth risks answering "impressive breadth" rather than a defensible core.

### 13.2 Priority stack

| Priority | Capabilities | Why |
|---|---|---|
| **MUST_WIN** | 1. Bitemporal, supersession-aware, provenance-linked memory with deterministic `current`/`as_of` truth. 2. Bounded, deterministic, receipted Context Compiler with scope/grant enforcement that beats plain retrieval on continuation. 3. Local-first, open canonical formats that survive Fehrest itself. 4. Single-agent continuation with honest staleness/contradiction handling. | Without these Fehrest has no thesis. The product fails B-7 past this point regardless of breadth. |
| **SHOULD_HAVE** | Strong deterministic lexical/structured retrieval baseline (003) with rebuild equivalence; GitHub link + IDE discovery (008) for developer time-to-value; AI provider abstraction that keeps AI OFF complete. | Material developer value that compounds MUST_WIN. |
| **LATER** (proven before built) | Import/migration, team/policy, mobile, Hub, polished workspace views — gated behind 009 proof and explicit founder authorization. | Valuable but must not precede proof; each needs separate thesis-proof-style evidence. |
| **EXPERIMENTAL** (falsifiable) | Graph intelligence, vector retrieval, automatic memory promotion, derived affinity/behavioral influence, media/voice. Each is `FALSIFIABLE` under B-13/004 or B-5 etc. and may be removed. | Retaining without material benefit at acceptable cost is failure. |
| **REJECT** (architectural contamination) | Imported donor architectures wholesale; vector-as-default; Graphify-as-canonical-dependency; WeKnora Redis/root/prefix-check security model; SkillHone optimizer-as-authority; prompt-derived memory as truth. | Violates frozen invariants. |

### 13.3 Donor feature dilution guard

Donor breadth must not dilute Fehrest identity. Rule:

```text
DONOR_FEATURE_WITHOUT_FEHREST_REQUIREMENT -> STUDY/BENCHMARK only
REQUIREMENT_JUSTIFIED_AFTER_PONYTAIL -> smallest correct reuse mode
```

The V2 narrative should lead with MUST_WIN, not with coverage of every capability matrix column.

---

## 14. Final recommended future spec decomposition

No whole-program renumbering is proposed now. Post-R1 canonicalization should reconcile the decomposition below and record it through the required change-control class (normally Class C for architecture-semantic, Class D for security-boundary).

| Spec | Name | Disposition after review |
|---|---|---|
| 002 | Post-R1 Canonical Core Convergence | KEEP — add explicit schema migration compatibility contract (NG-15) and 5-minute DX journey (NG-19). |
| 003 | Derived Index / Lexical Retrieval Convergence | KEEP — add per-store rebuild equivalence proof (NG-14), hard SLOs/stress fixtures (NG-18), Aider repo-map mandatory baseline. |
| 004 | Graph/Temporal Intelligence Capability Experiment | KEEP — B-TEMP-RANK merged here; bind model/embedding/checkpoint/seed/dataset identities; retain/defer/reject decision is product gate. |
| 005 | Conditional Derived Intelligence Production Integration | CONDITIONAL on 004 RETAIN_NOW — smallest replaceable provider boundary only; refuse if not needed. |
| 006 | Temporal Memory Productization | KEEP — now owns bitemporal precision (NG-02/NG-15), taxonomy (NG-03), promotion matrix, revalidation, cursor idempotency, tombstones, retention-vs-forgetting. |
| 007A | Universal Context and Memory Gateway | NEW split from 007 — owns grants-for-reads, Context Compiler, scope, budgeting, residency policy, package/receipt/SelectionTrace, read-only CLI/SDK, versioned identities (NG-04/NG-05), token accounting, privacy minimization (NG-17). |
| 007B | Delegated Execution, Fencing and Receipt Foundation | NEW split from 007 — owns admission, attempt/fencing generations, durable intent, idempotency/reconciliation, JIT secrets, network envelope, artifact ingress/egress, adversarial TOCTOU/DNS/SSRF, sandbox lifecycle, invariant PROCESS_TERMINATION != CANCELLATION. |
| 008 | GitHub Link and IDE Discovery | KEEP — consumes 007A only (NG-22); no execution infra dependency. |
| 009 | Trusted Vertical Memory Proof | KEEP — expand falsification cases per PR #40 §3 (multi-step contradiction, partial time, rejected-guess recurrence, residency leakage, crash/replay, sandbox revocation). |
| 010 | Workspace Canonical Objects and Open-Format Foundation | KEEP — add annotation/discussion persistence class (NG-01), canonical object lifecycle, links vs derived overlay separation. |
| 011 | Personal Notes/Docs/Capture | KEEP — waits for 010 and editor gate. |
| 012 | Search/Graph Exploration and Bases UX | KEEP — explicit-link graph visualization independent of 005. |
| 013 | AI Provider Runtime and Ask Fehrest | KEEP — reads via 007A, actions via 007B. |
| 014 | External Evidence and WebMCP | KEEP — adds mutable/deleted/duplicate/stale lineage (NG-09) and at-connection SSRF enforcement. |
| 015 | Import and Migration Lab | KEEP — post-010, with mapping/migration receipts. |
| 016 | Collaboration Capability Experiment | KEEP — retain/defer/reject before production sync. |
| 017 | Sync and Multi-Writer Substrate | CONDITIONAL on 016 RETAIN | — |
| 018 | Organization Identity/Policy/Admin | KEEP — owns service-credential lifecycle (NG-08), depends on 017. |
| 019 | Team Communication and Shared Workspace | KEEP — depends on 017+018; never precedes them. |
| 020 | Mobile and Offline Capture Client | KEEP — offline search subset, depends on 017. |
| 021A | Extension/Connector/Automation Runtime | GATE DECISION after 021 authoring: split if_runtime_cannot_remain_independently_testable (NG-12). |
| 021B | Skill Package / Evaluation / Evolution | GATE DECISION — same gate; owns install lifecycle, held-out isolation, redaction, score lineage, optimizer attribution, no auto-merge. |
| 022 | Fehrest Hub/Network | KEEP — last; hub != only canonical copy. |

Dependency invariants retained:

```text
READ_ONLY_CONTEXT_DOES_NOT_DEPEND_ON_EXECUTOR_INFRA=YES
DERIVED_RANK != AUTHORITY
MODEL_INFERENCE != ACTIVE_MEMORY
WEB_CONTENT != AUTHORITY
HUB != ONLY_CANONICAL_COPY
```

---

## 15. Explicit rejected / deferred ideas

| Idea | Verdict | Reason |
|---|---|---|
| Import Tencent architectures wholesale (full WeKnora stack, RoMem Python KGE runtime as canonical dependency, SkillHone optimizer as product authority) | REJECT | Violates Rust semantic ownership, creates unowned privilege/dependencies; copy only smallest justified mechanism. |
| WeKnora Redis as authority, path-prefix checks, root sandbox as Fehrest default | REJECT | Weak/default donor security assumptions; must pass Fehrest threat-model review. |
| Vector DB as default retrieval without lexical inadequacy proof | DEFER | ADR-0007 vectors deferred/optional; lexical baseline must be proven first. |
| Graph production integration before GI-CAP falsification test | DEFER | Graph intelligence is hypothesis per Freeze F-CORE-?; integration only on material benefit at acceptable cost. |
| Automatic memory promotion without confirmation for high-influence types | REJECT as default | P-TS-04 + NG-03: decisions/constraints/preferences/procedures/identity require human confirmation (F §5.5). |
| Behavioral interest/affinity as canonical memory or ranking authority | REJECT as authority | NG-19: influence class DERIVED_BEHAVIORAL_SIGNAL, isolated, explainable, user-disableable, never canonical. |
| Aider/Github/Memory donor benchmarks assumed to justify graph — without Fehrest preregistered kill rules | DEFER | Every candidate needs pinned dataset, fixed baseline, pre-registered kill/retain. |
| Rank/volatility/fused score as supersession truth | REJECT | P-TS-01 + NG-02: supersession is canonical event, not derived rank. |
| Normalized topic key as supersession authority | REJECT | TG-20/NG-02: key collision false-supersession risk. |
| Skill evolution with private gold visible to optimizer, custom validator without parity evidence, prompt-optimizer thresholds as Fehrest thresholds | REJECT | TG-28/NG-12: held-out leakage and evaluator drift. |
| Canvas/visual engine before editor/content model proof | DEFER | Freeze explicitly not authorized; 20-FUTURE-GATES visual gate deferred. |
| Editor/CRDT adoption because Rust-native, not because required | DEFER/REJECT | AR 20-FUTURE-GATES CRDT gate: NO CRDT authorized for headless thesis-proof. |
| Team/media/plugin infra before core thesis proof | DEFER | Execution Master Plan E-10, Freeze §8 excluded list. |

---

## 16. Gap ownership assignment — complete closure

### 16.1 Prior 28 gaps (TG-01..28) — confirmed and reconciled

| Gap | Title (shorthand) | Owner | Classification |
|---|---|---|---|
| TG-01 | Temporal rank vs truth | 006 + 004 + 007A | CROSS_SPEC_INVARIANT (P-TS-01) |
| TG-02 | Relation volatility authority class | 004/005 derived | DEFER/REJECT unless benchmark-retained |
| TG-03 | Promotion policy type/risk sensitivity | 006 | EXISTING_SPEC |
| TG-04 | Resident vs recalled context policy | 006 taxonomy + 007A policy | NEW_AUTHORING_SPLIT |
| TG-05 | Editable derived shadow canonical state | 003 + 010 + 011 | EXISTING_SPEC |
| TG-06 | Execution admission/receipt general semantics | 007B | NEW_AUTHORING_SPLIT |
| TG-07 | External credential vs principal mapping | 007A + 018 | EXISTING_SPEC |
| TG-08 | Queue retry != idempotency/fencing | 007B | NEW_AUTHORING_SPLIT |
| TG-09 | Skill artifact lifecycle | 021 (gate to 021A/B) | NEW_AUTHORING_SPLIT |
| TG-10 | Held-out eval isolation | 021B | NEW_AUTHORING_SPLIT |
| TG-11 | Decision history vs memory | 006 + 021B | EXISTING_SPEC |
| TG-12 | Generated wiki authority transitions | 010 + 006 + 011 | EXISTING_SPEC |
| TG-13 | Persistent sandbox outlives authority | 007B | NEW_AUTHORING_SPLIT |
| TG-14 | Source reuse dossier standard | governance | GOVERNANCE_RULE + CROSS_SPEC_INVARIANT I-17 |
| TG-15 | Bitemporal precision/partial time | 006 + 004/003/007A | EXISTING_SPEC |
| TG-16 | Temporal model/checkpoint identity binding | 004/005 | BENCHMARK |
| TG-17 | Retention vs forgetting/archival | 006 | EXISTING_SPEC |
| TG-18 | Proposal pipeline idempotency/cursors | 006 + 007B | EXISTING_SPEC + NEW_AUTHORING_SPLIT |
| TG-19 | Derived affinity/interest hidden authority | 006 + 007A | EXISTING_SPEC + NEW_AUTHORING_SPLIT |
| TG-20 | Supersession key collision | 006 | EXISTING_SPEC |
| TG-21 | Remote resource binding vs authority | 007B | NEW_AUTHORING_SPLIT |
| TG-22 | Policy/config/skill generation fencing | 007B | NEW_AUTHORING_SPLIT |
| TG-23 | Artifact promotion ingress/egress | 007B + 010 + 021 | NEW_AUTHORING_SPLIT + EXISTING_SPEC |
| TG-24 | Egress DNS/SSRF/private network | 007B + 014 | NEW_AUTHORING_SPLIT + EXISTING_SPEC |
| TG-25 | Skill install supply chain privilege | 021 + 007B | NEW_AUTHORING_SPLIT |
| TG-26 | Audit evidence vs truncation | 007B | NEW_AUTHORING_SPLIT |
| TG-27 | Eval redaction/score lineage | 021B | NEW_AUTHORING_SPLIT |
| TG-28 | Optimizer attribution contamination | 021B | NEW_AUTHORING_SPLIT |

### 16.2 New 23 gaps (NG-01..23) — added by this review

| Gap | Title | Owner | Classification |
|---|---|---|---|
| NG-01 | Annotation/discussion persistence class | 010 | EXISTING_SPEC |
| NG-02 | Supersession key & conflicting timeline reconciliation | 006 | EXISTING_SPEC |
| NG-03 | Memory kind taxonomy | 006 | EXISTING_SPEC |
| NG-04 | Context compiler versioned identities & replay | 007A | NEW_AUTHORING_SPLIT |
| NG-05 | Token accounting determinism | 007A | NEW_AUTHORING_SPLIT |
| NG-06 | Cancellation vs termination distinction | 007B | NEW_AUTHORING_SPLIT |
| NG-07 | Lease attenuation & recursive delegation | 007B | NEW_AUTHORING_SPLIT |
| NG-08 | Service/API credential lifecycle | 007A+018+007B | CROSS_SPEC_INVARIANT + EXISTING_SPEC |
| NG-09 | External evidence mutable/deleted/duplicate lineage | 014 | EXISTING_SPEC |
| NG-10 | Retrieval fusion formula & per-stage kill criteria | 003+004 | BENCHMARK |
| NG-11 | Full donor landscape reuse decisions | program matrix | GOVERNANCE_RULE + CROSS_SPEC_INVARIANT |
| NG-12 | 021 split decision gate hardening | 021 gate | GOVERNANCE_RULE (authoring gate) |
| NG-13 | Hidden local-first dependencies (offline completeness) | each spec | CROSS_SPEC_INVARIANT |
| NG-14 | Recovery equivalence per derived store | 002+003+005 | EXISTING_SPEC + BENCHMARK |
| NG-15 | Schema forward/backward compatibility & unknown fields | 002 | CROSS_SPEC_INVARIANT + EXISTING_SPEC |
| NG-16 | Observability without second source of truth | 007A | NEW_AUTHORING_SPLIT |
| NG-17 | Privacy per-visibility classification | 006+007A+014 + invariant | CROSS_SPEC_INVARIANT |
| NG-18 | Performance/resource hard SLOs & large-scale stress | each spec | BENCHMARK |
| NG-19 | First-five-minutes DX acceptance | 002+003+007 | EXISTING_SPEC |
| NG-20 | Fault injection & cross-platform testing matrix | each spec | GOVERNANCE_RULE |
| NG-21 | Benchmark family proliferation/minimality | benchmark plan | GOVERNANCE_RULE |
| NG-22 | DAG enforcement for 007A/007B split | SPEC_SEQUENCE | GOVERNANCE_RULE |
| NG-23 | Product value dilution / MUST_WIN identification | 00-PRODUCT-THESIS | CROSS_SPEC_INVARIANT |

```text
VAGUE_FUTURE_WORK_REMAINING=0
EVERY_GAP_HAS_OWNER_OR_GATE=51/51
```

---

## 17. Required actions for PR #40 and PR #2 reconciliation

### 17.1 PR #40 before any canonicalization

1. Land this review as durable evidence (this document).
2. Publish a supplemental full-donor reuse review or extend the existing matrix with the donors in §11.2; do not treat the 3 Tencent donors as the full landscape.
3. Update `V2_TENCENT_SOURCE_PLAN_AMENDMENT` to reference NG-01..NG-23, to note the OC-01..03 reconciliations, and to fold P-TS invariants into the correct invariant owners (retire parallel enumeration).
4. Record that benchmark families B-* remain planning hypotheses until decomposed per §12; do not create 8 independent benchmark suites now.
5. Re-render PR #40 body with the converged 51-gap closure and the 021 authoring-gate criterion.

All remain **DRAFT / non-canonical / non-authorizing** while R1 is open per `specs/CURRENT.md`.

### 17.2 PR #2 before any canonicalization

Before PR #2 can become canonical after the true R1 terminal outcome, it must reconcile:

```text
REAL_R1_TERMINAL_OUTCOME
LIVE_MAIN at c87a34d..HEAD
TG-01..28 + NG-01..23
007A/007B decomposition decision
021A/021B authoring-gate decision
Full donor reuse matrix (NOT just Tencent)
Founder source-use authorization
Source-reuse dossier standard
Current architecture/security invariants (Freeze F-CORE-01..17)
```

No adapter may premise graph/vector/CRDT/canvas/mobile as thesis-proven. The proposal's R1 claims remain stale until reconciled forward against the real R1 terminal route.

---

## 18. R1 impact verification

```text
BENCH_R1_MODIFIED=NO
DOCS_CANONICAL_R1_V2_MODIFIED=NO
SPEC_CURRENT_MODIFIED=NO
SPEC_002_MODIFIED=NO
PRODUCT_RUNTIME_MODIFIED=NO
GITHUB_WORKFLOW_BENCH_VALIDATION_MODIFIED=NO
REVIEW_CANDIDATE_COMMIT=e1448705cf0bebb17533b6f4dd202c2eaa707172
REVIEW_CANDIDATE_TREE=fd841c56a1debd5845b37d81f84bf586cb435411
MANIFEST_SHA256=a78584247f6e48b7b75271af7cf608280c9a10228cf3ab04f3d8a4930cf8eabd
AFFECTED_REVIEW=NO (compare PR #40..main changes only planning/research/review docs)
```

Validated locally:

```text
python bench/R1/validate.py -> PASS (to be re-run by exact-head CI)
```

This review creates no load-bearing change to the frozen R1-v2 review candidate for Issues #37/#38.

---

## 19. Final standard — planning convergence checklist

```text
PLAN_GAPS_FOUND=51 (28 TG + 23 NG)
PLAN_GAPS_RESOLVED_OR_OWNED=51/51
OWNERSHIP_CONFLICTS=0 after reconciliation (3 found and resolved)
KNOWN_DEPENDENCY_CYCLES=0
UNJUSTIFIED_MAJOR_SUBSYSTEMS=0 after kill-criteria hardening (2 flagged before)
UNOWNED_SECURITY_CRITICAL_SEMANTICS=0 after assignment (3 were unowned before)
UNOWNED_DURABILITY_CRITICAL_SEMANTICS=0 after assignment (2 were unowned before)
SOURCE_REUSE_DECISIONS_EXPLICIT=YES (after §11 extension — Tencent complete, earlier donors now explicitly decided)
BENCHMARK_KILL_RULES_EXPLICIT=YES (per-family template in §12, now mandatory pre-registration)
PRODUCT_PRIORITY_EXPLICIT=YES (§13 MUST_WIN / SHOULD_HAVE / LATER / EXPERIMENTAL / REJECT)
R1_CHANGED=NO
```

If any future implementation proceeds without the NG owners above, the checklist honest state would revert to FAIL.

---

## 20. Handoff — what remains before post-R1 canonicalization

```text
CURRENT_AUTHORIZED_LOCAL_WORK=REVIEW_COMPLETE
PROJECT_COMPLETE=NO

SCIENTIFIC_REVIEW=PENDING_EXTERNAL (Issue #37)
STATISTICAL_REVIEW=PENDING_EXTERNAL (Issue #38)

REVIEW_CANDIDATE=e1448705cf0bebb17533b6f4dd202c2eaa707172
REVIEW_CANDIDATE_TREE=fd841c56a1debd5845b37d81f84bf586cb435411
MANIFEST_SHA256=a78584247f6e48b7b75271af7cf608280c9a10228cf3ab04f3d8a4930cf8eabd

NEXT_DEPENDENCY=INDEPENDENT_REVIEW (external)
PLANNING_NEXT_DEPENDENCY=PR40 reconciliation per §17.1 (authorized bounded planning work remains for PR #40 branch only)
SPEC_002=BLOCKED
EXECUTION=BLOCKED
SEALING=BLOCKED
PR_2_MERGE=PROHIBITED_WHILE_R1_OPEN
PR_40_MERGE=PROHIBITED_WHILE_R1_OPEN
```

On next Muse run, re-verify Issues #37/#38 first. If qualified independent review evidence appears, immediately continue the sealing qualification sequence per `R1_V2_SEALING_PROCEDURE.md`. Do not start product implementation, graph/vector/CRDT work, or automatic memory work before the terminal R1 gate and founder authorization.

---

## Appendix A — Red-team self-challenge on this review

- Could this review itself invent work? Mitigated by marking all outputs DRAFT/non-authorizing and refusing implementation, sealing, or spec activation while R1 open.
- Could NG-* duplicate TG-*? Cross-checked each NG against TG-01..28; every NG is disjoint (either finer contract, hidden dependency, or earlier-donor scope not covered before).
- Could the 007A/007B split be wrong? Applied Ponytail: retained because read availability must not require executor availability (a classic isolation violation that would hide a reverse dependency for years). If post-R1 authoring finds a single spec can achieve same isolation with internal modules, the split may be collapsed via Class C ADR — the split is an authoring proposal, not frozen.
- Could MUST_WIN be too narrow? Deliberately narrow — thesis failure modes F-1 and CEILING_EFFECT show that adding breadth cannot compensate for losing current/as-of/provenance/context-determinism. Breadth is gated behind 009 proof.
- Is the 51-gap total inflated for appearance? No — 28 prior gaps are preserved as-is; 23 new gaps each correspond to one of the 20 audit dimensions where TG left an unowned contract. Each is independently owned with a falsifiable benchmark or security test; vague "future work" eliminated.

---

*End of review — no implementation authority claimed. Preserve as evidence.*

