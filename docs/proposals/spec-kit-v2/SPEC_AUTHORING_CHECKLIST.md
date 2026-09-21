# Fehrest Executable Spec Kit Authoring Checklist

**Status:** PROGRAM QUALITY CHECKLIST / NON-AUTHORIZING  
**Created:** 2026-08-28

> Use this checklist before any future Fehrest Spec Kit is marked ready for implementation. It supplements, and never weakens, live repository governance.

---

# A. Authority and frontier

- [ ] `AGENTS.md` re-read from live repository truth.
- [ ] `specs/CURRENT.md` re-read from live repository truth.
- [ ] Canonical Execution Master Plan re-read.
- [ ] Active benchmark/evidence gate re-read where applicable.
- [ ] Historical governance sources required by this change are present/reconciled.
- [ ] Exactly one active frontier remains.
- [ ] Spec status is correctly one of `DRAFT / SPECIFIED_BLOCKED / ACTIVE / CLOSED / DEFERRED / REJECTED` or equivalent repository-approved state.
- [ ] Explicit founder authorization exists when required by change class.
- [ ] No future planning document is mistaken for authorization.

**Hard fail if any authority source is missing and proceeding requires guessing.**

---

# B. Problem and user outcome

- [ ] `spec.md` states WHAT and WHY before HOW.
- [ ] Dominant user/system outcome is singular and understandable.
- [ ] Target persona/principal is explicit.
- [ ] P1 story delivers independently demonstrable value.
- [ ] User stories are prioritized.
- [ ] Every story has an independent test.
- [ ] Non-goals are explicit.
- [ ] Out-of-scope adjacent features are listed where confusion is likely.
- [ ] Success criteria are measurable and technology-agnostic.
- [ ] No requirement exists solely because a donor/library has the feature.

---

# C. Clarification completeness

- [ ] All material ambiguities are recorded in `clarifications.md` or equivalent.
- [ ] No unresolved `[NEEDS CLARIFICATION]` remains at activation.
- [ ] Founder decisions are quoted/recorded without broadening them.
- [ ] Reasonable defaults are labeled as assumptions, not facts.
- [ ] Unknowns owned by later specs are explicitly deferred with owner IDs.

Required activation condition:

```text
UNRESOLVED_NEEDS_CLARIFICATION=0
```

---

# D. Requirement quality

- [ ] Every functional MUST has a stable requirement ID.
- [ ] Applicable non-functional requirements have stable IDs.
- [ ] Requirements are testable or have an exact verification procedure.
- [ ] Requirements do not embed premature implementation choices.
- [ ] Failure behavior is specified, not only success behavior.
- [ ] Offline/provider/network behavior is specified where applicable.
- [ ] Security/privacy behavior is specified where applicable.
- [ ] Migration/backward compatibility behavior is specified for durable changes.
- [ ] Accessibility/keyboard/internationalization expectations are stated for human-facing work.

---

# E. Ownership and overlap

- [ ] Every new canonical entity has exactly one owning spec.
- [ ] Every lifecycle transition has exactly one owner.
- [ ] Every external/internal contract has exactly one owner.
- [ ] No durable semantics are hidden inside UI code.
- [ ] No provider-specific ID becomes canonical identity.
- [ ] No later spec silently forks an earlier contract.
- [ ] Shared contract updates include compatibility/versioning tests.
- [ ] `CROSS_SPEC_INVARIANTS_AND_OWNERSHIP.md` has been checked for overlap.

Required activation condition:

```text
KNOWN_SEMANTIC_OVERLAP_WITHOUT_OWNER=0
```

---

# F. Data model and persistence classification

For every persisted field/store/artifact:

- [ ] Classified as `CANONICAL`, `DERIVED_REBUILDABLE`, `CONFIGURATION`, `SECRET_REFERENCE`, `CACHE`, or `EVIDENCE_ARTIFACT`.
- [ ] Canonical identity source is explicit.
- [ ] Canonical mutation path is explicit.
- [ ] Derived rebuild source is explicit.
- [ ] Secret bytes are excluded from canonical knowledge/events/trajectories.
- [ ] Lifecycle states/transitions are explicit.
- [ ] Temporal fields distinguish observed/recorded/effective time where required.
- [ ] Unknown/unsupported schema behavior is fail-visible.
- [ ] Recovery behavior is defined.
- [ ] Export behavior is defined where user-owned knowledge changes.

---

# G. Research and donor discipline

For every external donor/provider/library candidate:

- [ ] Exact repository/source is recorded.
- [ ] Immutable revision/version is pinned for load-bearing review.
- [ ] Relevant source paths are recorded when code reuse is considered.
- [ ] License/permission evidence is recorded.
- [ ] Disposition is one or more of `USE / ADAPT / STUDY / BENCHMARK / DEFER / REJECT`.
- [ ] Requirement closed by the candidate is explicit.
- [ ] Simpler/no-new-dependency alternative is evaluated.
- [ ] Security implications are evaluated.
- [ ] Maintenance/update/exit strategy is evaluated.
- [ ] Benchmark is preregistered when comparative performance/quality is material.

No donor enters production because it is popular or because code reuse permission exists.

---

# H. Plan quality

- [ ] `plan.md` reads the spec and does not silently invent new user requirements.
- [ ] Technical context is complete.
- [ ] Constitution/governance check is performed before Phase 0 research.
- [ ] Constitution/governance check is repeated after data model/contracts.
- [ ] Project/source layout uses real repository paths.
- [ ] Complexity is justified when a simpler architecture is rejected.
- [ ] Build-vs-adapt-vs-provider decisions are documented.
- [ ] Failure/rollback strategy is documented.
- [ ] Performance/resource targets are documented.
- [ ] Platform matrix is documented.
- [ ] Observability/audit evidence is documented where applicable.

---

# H-RUST. Rust-first language architecture gate

Every executable Fehrest plan must apply `RUST_PLATFORM_ARCHITECTURE.md` and `RUST_SPEC_TRACEABILITY_MATRIX.md`.

Required declaration:

```text
Primary language: Rust
Rust toolchain: <repository-pinned toolchain>
Rust edition: <repository-pinned edition>
Rust-owned semantic paths: <exact paths/crates>
Non-Rust code required: YES|NO
Non-Rust paths: <exact paths or N/A>
Reason non-Rust is required: <evidence or N/A>
Interop contract owner: <Rust module/crate or N/A>
Semantic authority outside Rust: NO
Unsafe in Fehrest Core: NO
FFI/native dependencies: <explicit list or NONE>
WASM/browser bridge: <explicit list or NONE>
```

Checklist:

- [ ] Rust is the primary product implementation language for the spec.
- [ ] Rust-owned semantic responsibilities are listed explicitly.
- [ ] Canonical state semantics remain Rust-owned.
- [ ] Authorization/grant semantics remain Rust-owned.
- [ ] Memory/provenance semantics remain Rust-owned where applicable.
- [ ] Search/context/sync/provider semantics remain Rust-owned where applicable.
- [ ] Every non-Rust file/path is declared.
- [ ] Every non-Rust boundary is presentation/platform/provider interoperability only.
- [ ] Non-Rust code cannot mint identity, grants, memory state or canonical authority.
- [ ] Rust validates all untrusted data crossing a foreign boundary before authority-sensitive use.
- [ ] Rust alternatives were evaluated before approving a foreign runtime/component.
- [ ] Donor code in another language is ported/adapted to Rust when it would otherwise own Fehrest semantics.
- [ ] FFI/native dependencies have explicit trust, safety, update and exit analysis.
- [ ] `unsafe` remains absent from Fehrest Core under current governance.
- [ ] UI/editor/mobile/browser bridges have typed/versioned Rust-owned contracts.
- [ ] No UI or foreign runtime maintains an independent authoritative search, permission, memory or sync model.
- [ ] Rust/WASM/native platform test requirements are included where applicable.

Required activation condition:

```text
RUST_LANGUAGE_GATE=PASS
PRIMARY_LANGUAGE_RUST=YES
SEMANTIC_AUTHORITY_OUTSIDE_RUST=NO
UNJUSTIFIED_NON_RUST_PRODUCT_LOGIC=0
UNDECLARED_FFI_BOUNDARIES=0
```

A blocked Rust language gate prevents implementation unless the founder/architecture governance explicitly changes the language direction.

---

# I. Contract quality

Every contract declares:

```text
CONTRACT_ID
OWNER_SPEC
VERSION
CANONICAL_OR_DERIVED
AUTHORIZATION_BOUNDARY
COMPATIBILITY_RULE
```

Checklist:

- [ ] Inputs/outputs are typed or structurally specified.
- [ ] Error/failure classes are specified.
- [ ] Authorization checks are specified below presentation.
- [ ] Version negotiation/unsupported behavior is specified.
- [ ] Size/resource limits are specified where untrusted input exists.
- [ ] Idempotency/retry semantics are specified where operations can repeat.
- [ ] Receipts/provenance are specified where model/tool/external evidence is involved.

---

# J. Security and threat review

Required when the spec adds or changes any of:

```text
principal
grant
sharing
network
web/tool access
model/provider
plugin/extension
sync/multi-writer
organization/tenant
secret handling
external acquisition
```

Checklist:

- [ ] Assets and trust boundaries identified.
- [ ] Attacker/principal classes identified.
- [ ] Deny-by-default posture documented where relevant.
- [ ] Cross-scope/tenant leak cases included.
- [ ] Prompt-injection/content-as-authority cases included for AI/web.
- [ ] Revocation/expiry cases included.
- [ ] Secret exfiltration paths considered.
- [ ] Replay/duplicate/retry abuse considered.
- [ ] Failure does not widen access.
- [ ] Audit/receipt requirements specified.
- [ ] Security tests/adversarial cases trace to requirements.

---

# K. Local-first/offline review

For human-facing/local/sync features:

- [ ] Behavior with `NETWORK=OFF` is explicit.
- [ ] Behavior with `AI=OFF` is explicit.
- [ ] Behavior with sync server unavailable is explicit.
- [ ] Local canonical state remains usable where promised.
- [ ] Conflict/reconnect behavior is explicit when multi-writer applies.
- [ ] User can understand where data lives.
- [ ] Export/backup does not depend solely on hosted service availability.

---

# L. AI/provider review

When models are involved:

- [ ] Model/provider is not canonical identity.
- [ ] AI OFF behavior remains complete for core feature correctness.
- [ ] Local/remote/custom provider modes are explicit where supported.
- [ ] Capability probe behavior is explicit.
- [ ] Provider mismatch fails visibly.
- [ ] Context scope is authorized before model invocation.
- [ ] Token/byte limits are bounded.
- [ ] Model output state is classified as draft/evidence/proposal/canonical.
- [ ] Model cannot mint permission.
- [ ] Provider credentials remain secret references.
- [ ] Significant AI edits have preview/diff/review where required.

---

# M. Web/WebMCP review

When external web/tools are involved:

- [ ] Web authorization mode is explicit.
- [ ] Origin/domain binding is explicit.
- [ ] READ vs ACTION tools are distinguished.
- [ ] UNKNOWN tools default restrictive.
- [ ] Consequential actions require explicit approval/pre-authorization.
- [ ] Tool descriptions/page content/results are treated as untrusted.
- [ ] External content cannot widen grants or reveal secrets.
- [ ] Source URL/origin/acquisition time/provenance are recorded.
- [ ] Failed/stale/unavailable source behavior is preserved.
- [ ] WebMCP is behind a Fehrest-owned provider abstraction.

---

# N. Collaboration/sync review

When multi-device/multi-user behavior applies:

- [ ] Offline convergence criterion exists.
- [ ] Conflict behavior is user-visible and deterministic enough to audit.
- [ ] Revoked offline writer reconnect is tested.
- [ ] Permission downgrade during partition is tested.
- [ ] Duplicate/replay mutation is tested.
- [ ] Partial sync crash/restart is tested.
- [ ] Schema/version skew is tested.
- [ ] Cross-space/tenant leakage is tested.
- [ ] Recovery interaction with local canonical state is tested.
- [ ] Sync provider failure does not destroy local ownership.

---

# O. UX review

For human-facing specs:

- [ ] Primary journey works without requiring expert terminology.
- [ ] Empty state is designed.
- [ ] Error state is designed.
- [ ] Loading/offline/sync state is understandable.
- [ ] Keyboard path exists for primary desktop workflow where appropriate.
- [ ] Accessibility acceptance criteria exist.
- [ ] Destructive actions have recoverability/confirmation proportional to impact.
- [ ] Trust/privacy/AI state is visible without technical jargon.
- [ ] UI does not expose internal IDs/provenance complexity by default.
- [ ] Advanced inspector exists only when useful, not as mandatory workflow.

---

# P. Tasks quality

- [ ] `tasks.md` is generated/validated from `spec.md`, `plan.md`, data model and contracts.
- [ ] Tasks are grouped by user story where applicable.
- [ ] Every task has an exact target file/path or exact evidence artifact.
- [ ] `[P]` is used only for genuinely non-conflicting work.
- [ ] Tests precede implementation where the test-first flow applies.
- [ ] Foundational tasks block dependent user stories explicitly.
- [ ] No task implements a requirement absent from the spec.
- [ ] No MUST requirement lacks implementation/verification tasks.
- [ ] Closeout tasks include security/benchmark/native-platform evidence as applicable.

Required condition:

```text
ORPHAN_TASKS=0
```

---

# Q. Analyze gate

Before implementation:

- [ ] Spec/plan/data-model/contracts/tasks checked for contradictions.
- [ ] Requirement terminology is consistent.
- [ ] Entity names are consistent.
- [ ] Contract versions are consistent.
- [ ] Dependency ordering has no cycle.
- [ ] No forbidden future feature leaked into scope.
- [ ] No security gate was deferred to “later” when it is required now.
- [ ] No benchmark threshold was chosen after seeing results.
- [ ] No unresolved critical ambiguity remains.
- [ ] Rust language ownership declarations match the implementation plan and task paths.
- [ ] No foreign/UI path silently owns semantics assigned to Rust.

---

# R. Ponytail necessity gate

For each dependency/subsystem:

- [ ] Can Rust std/existing dependency close the requirement?
- [ ] Can an existing Fehrest subsystem be extended safely?
- [ ] Can a provider boundary avoid owning specialized infrastructure?
- [ ] Can donor code be adapted with lower risk than a new dependency?
- [ ] Is building in-house necessary for Fehrest's unique correctness/authority semantics?
- [ ] Maintenance burden is justified by measured value.

Record one of:

```text
KEEP_EXISTING
BUILD_MINIMAL
ADAPT_DONOR
USE_DEPENDENCY
USE_PROVIDER
DEFER
REJECT
```

---

# S. Implementation evidence

- [ ] Exact branch/head recorded.
- [ ] Atomic/coherent commit discipline followed.
- [ ] No force push/rebase/destructive rewrite.
- [ ] No unauthorized paths changed.
- [ ] Generated artifacts identified.
- [ ] Raw benchmark/security/fault evidence preserved.
- [ ] Native-platform claims backed by genuine native execution.
- [ ] Rust/non-Rust path inventory matches the approved plan.
- [ ] New FFI/unsafe/native boundaries match the approved review.

---

# T. Verification and converge

Before close:

- [ ] All MUST requirements have evidence.
- [ ] All P1 acceptance scenarios pass.
- [ ] Applicable lower-priority promised stories pass.
- [ ] Contract tests pass.
- [ ] Migration/backward fixtures pass.
- [ ] Security/adversarial tests pass.
- [ ] Benchmark thresholds pass or route honestly to fail/defer/reject.
- [ ] Performance/resource limits pass.
- [ ] Native platform matrix is truthful.
- [ ] Applicable Rust/WASM/FFI boundaries have their required tests.
- [ ] `verification.md` records exact commands/results/heads/artifacts.
- [ ] Final `analyze.md` finds no unexplained drift.
- [ ] Converge compares spec/plan/tasks/contracts/code/tests.
- [ ] Converge confirms semantic authority did not move outside Rust.
- [ ] Deferred items have explicit owners and are not silently omitted.

Required closeout conditions:

```text
ORPHAN_REQUIREMENTS=0
ORPHAN_ACCEPTANCE_SCENARIOS=0
ORPHAN_CONTRACTS=0
ORPHAN_TASKS=0
UNVERIFIED_MUST_REQUIREMENTS=0
UNEXPLAINED_SPEC_IMPLEMENTATION_DRIFT=0
UNJUSTIFIED_NON_RUST_PRODUCT_LOGIC=0
UNDECLARED_FFI_BOUNDARIES=0
```

---

# U. Closeout/frontier update

- [ ] Spec closeout state reflects evidence, not intent.
- [ ] Negative results preserved.
- [ ] `specs/CURRENT.md` updated only when the next entry criteria are actually met.
- [ ] Future spec is not activated merely because it is documented.
- [ ] Canonical master plan updated only when order/authority genuinely changed.
- [ ] Exact post-merge/post-close evidence recorded before claiming canonical closure where repository process requires it.

---

# Final readiness verdict template

```text
AUTHORITY_GATE=PASS|BLOCKED
CLARIFICATIONS=PASS|BLOCKED
REQUIREMENT_COMPLETENESS=PASS|BLOCKED
OWNERSHIP_OVERLAP=PASS|BLOCKED
DATA_MODEL=PASS|BLOCKED
RUST_LANGUAGE_GATE=PASS|BLOCKED
CONTRACTS=PASS|BLOCKED
SECURITY=PASS|BLOCKED|N/A
MIGRATION=PASS|BLOCKED|N/A
BENCHMARK_PREREGISTRATION=PASS|BLOCKED|N/A
PONYTAIL=PASS|BLOCKED
TASK_TRACEABILITY=PASS|BLOCKED
IMPLEMENTATION_MAY_BEGIN=YES|NO
```

`IMPLEMENTATION_MAY_BEGIN=YES` is valid only when every required gate above is genuinely PASS and live repository authorization permits it.
