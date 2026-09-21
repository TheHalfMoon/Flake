# Fehrest Rust Spec Traceability Matrix

**Status:** PROGRAM QUALITY CONTROL / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Founder direction:** Rust-first, Rust-owned product architecture  
**Canonical authority:** unchanged; see live repository governance.

> This matrix makes the Rust founder direction auditable across the proposed V2 Spec Kit sequence. It does not activate implementation or preselect libraries/frameworks.

---

## 1. Program rule

Every future executable Spec Kit MUST answer two questions:

```text
1. Which semantics does this spec own?
2. Which of those semantics are implemented and enforced in Rust?
```

The default answer for product, security, memory, authorization, data, search, sync, agent, provider and server semantics is:

```text
RUST_OWNED=YES
```

A non-Rust component may exist only as a bounded interoperability or presentation adapter when the active spec proves the necessity.

Hard invariant:

```text
NON_RUST_BRIDGE_MAY_TRANSLATE=YES
NON_RUST_BRIDGE_MAY_PRESENT=YES
NON_RUST_BRIDGE_MAY_OWN_CANONICAL_TRUTH=NO
NON_RUST_BRIDGE_MAY_OWN_AUTHORIZATION=NO
NON_RUST_BRIDGE_MAY_OWN_MEMORY_LIFECYCLE=NO
NON_RUST_BRIDGE_MAY_OWN_SYNC_POLICY=NO
NON_RUST_BRIDGE_MAY_OWN_SEARCH_AUTHORITY=NO
NON_RUST_BRIDGE_MAY_MINT_GRANTS=NO
```

---

## 2. Cross-spec Rust ownership map

| Spec | Rust-owned semantic responsibility | Permitted thin/non-Rust boundary | Mandatory Rust evidence |
|---|---|---|---|
| 002 Canonical Core | vault identity, canonical writes, writer boundary, event journal, recovery, upcasting | OS/filesystem syscall boundary only where required | cargo gates, crash/fault tests, native platform evidence |
| 003 Derived Index/Retrieval | index orchestration, invalidation, reconciliation, query/filter semantics, trace | external search engine process/library only behind typed Rust provider if justified | incremental-vs-clean equivalence, rebuild proof, latency/resource evidence |
| 004 Graph Experiment | experiment harness, normalized comparator contracts, metrics/evidence | comparator systems may be non-Rust because they are experimental baselines | preregistration, exact versions, raw benchmark evidence |
| 005 Graph Production | GraphProvider, identity mapping, rebuild, selection trace, scope | renderer/database/service adapter only if benchmark retained it | provider replacement/rebuild tests, authority-negative tests |
| 006 Temporal Memory | memory schema/lifecycle, temporal resolution, proposals, CI semantics, review transitions | presentation clients only | lifecycle/property tests, migration/recovery, poisoning/staleness/adversarial tests |
| 007 Universal Gateway | principals, grants, authorization chokepoint, context compiler, receipts, CLI/SDK/API/MCP semantics | protocol transport bindings | grant/scope negative tests, receipt/replay tests, resource bounds |
| 008 GitHub/IDE Link | binding parser, repository mapping, provenance, discovery semantics, gateway integration | IDE plugin shell/UI; GitHub-hosted API is external | discovery/auth separation tests, spoofing/path tests, provenance fixtures |
| 009 Trusted Vertical Proof | benchmark harness, task/context receipts, evidence recorder | tested agents/providers may be external/non-Rust | preregistered end-to-end evidence, fresh human/agent outcomes |
| 010 Workspace Object/Open Format | Space/Project/Note/Task/etc canonical semantics, open-format mapping, explicit links | file/editor presentation bridge | round-trip/open-format/golden/migration tests |
| 011 Personal Workspace | application orchestration, commands, persistence integration, history/restore logic | editor rendering bridge only if editor benchmark requires it | offline journey, large-vault, editor fidelity, accessibility/platform evidence |
| 012 Search/Graph/Bases UX | search state/query orchestration, graph projection/filter semantics, view definitions | rendering primitive/WebGPU/DOM bridge | AI-off/offline search, graph correctness, performance/accessibility evidence |
| 013 AI Provider/Ask | provider abstraction, capability probe, authorized request assembly, tool validation, output classification | model runtime/service itself is external; transport is adapter | provider contract tests, local/remote failure tests, authority-negative tests |
| 014 External Evidence/WebMCP | web authorization, origin/domain policy, tool classification, provenance/receipts, source state | browser/WebMCP API binding may require JS/WASM glue | prompt-injection/adversarial tests, origin/action authorization tests |
| 015 Import/Migration | import orchestration, mappings, provenance, rollback, destination validation | source parser library/foreign format adapter when justified | corpus fixtures, dry-run/rollback, unsupported-field reports |
| 016 Collaboration Experiment | benchmark harness, normalized collaboration semantics/tests | candidate CRDTs may expose foreign bindings for comparison | convergence/revocation/crash/partition evidence |
| 017 Sync/Multiwriter | replica identity, merge/conflict policy, sync protocol boundary, revocation/replay semantics | transport/native push service adapters | deterministic convergence, partition/revocation, crash/recovery evidence |
| 018 Organization/Admin | organization identity, membership, groups, roles, policy/grant enforcement, audit contracts | IdP/SSO/SCIM connector adapter | cross-tenant/guest/revocation/admin security tests |
| 019 Team Workspace | channel/topic/message/task/decision orchestration, knowledge crystallization proposal path | voice/video/media provider UI/SDK adapter | authorization/privacy/search/offline/team workflow evidence |
| 020 Mobile | shared domain logic, sync/search/memory/auth semantics | Swift/Kotlin/OS UI bridge only where required | offline/resume/sync/mobile platform tests; bridge semantic audit |
| 021 Extensions/Automation | extension manifest, capability grants, automation contract, provider registry | extension payload may be foreign/WASM/process if separately sandboxed | capability escape, network/process/secret boundary tests |
| 022 Hub | repository linkage, hosting semantics, provenance for copy/fork/proposal/review | CDN/object store/managed infra services external | local canonical independence, export/recovery, tenancy/security evidence |

---

## 3. Rust-first client policy

### Desktop

Preferred outcome:

```text
RUST_APPLICATION_CORE=YES
RUST_UI_OR_RUST_WASM=FIRST_CHOICE
```

Candidates such as Dioxus, Leptos, Iced, egui or a Tauri-hosted Rust/WASM UI are benchmark candidates, not commitments.

### Web

Preferred outcome:

```text
RUST_WASM=FIRST_CHOICE
```

JavaScript glue is allowed only for browser APIs or components that cannot be closed reasonably in Rust/WASM.

### Mobile

Preferred outcome:

```text
SHARED_RUST_DOMAIN_CORE=MANDATORY
RUST_FIRST_CLIENT=YES
```

A native shell is permissible only for platform integration when evidence requires it.

### CLI/server/agent gateway

```text
RUST_REQUIRED=YES
```

No exception is planned at program level.

---

## 4. Mandatory language declaration in every future plan

Every executable `plan.md` MUST contain a section equivalent to:

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

Activation fails when:

```text
PRIMARY_LANGUAGE_RUST!=YES
SEMANTIC_AUTHORITY_OUTSIDE_RUST!=NO
UNJUSTIFIED_NON_RUST_PRODUCT_LOGIC>0
UNDECLARED_FFI_BOUNDARIES>0
```

---

## 5. Non-Rust exception process

A future spec may request a narrow exception only when all are true:

```text
RUST_ALTERNATIVES_EVALUATED=YES
USER_OUTCOME_BLOCKED_WITHOUT_EXCEPTION=YES
EXCEPTION_PATHS_EXACT=YES
BRIDGE_CONTRACT_TYPED=YES
RUST_VALIDATES_INPUT=YES
NO_CANONICAL_AUTHORITY_OUTSIDE_RUST=YES
NO_AUTHORIZATION_OUTSIDE_RUST=YES
NO_MEMORY_SEMANTICS_OUTSIDE_RUST=YES
SECURITY_REVIEW=PASS
EXIT_STRATEGY_DEFINED=YES
```

Examples that may justify an exception after evidence:

- OS-native accessibility/input bridge;
- browser API binding;
- mobile platform service shell;
- mature rich-editor component when the editor benchmark demonstrates no acceptable Rust-native/Rust-WASM option;
- vendor SDK with no protocol-level alternative.

Examples that do **not** justify an exception:

- developer preference;
- faster prototype convenience;
- donor project is written in TypeScript/Python;
- existing UI framework familiarity;
- avoiding a Rust API design effort for canonical semantics.

---

## 6. Donor code reuse under Rust-first

The founder's permission to copy/adapt donor code does not mean non-Rust donor implementations become product architecture.

For non-Rust donors, preferred disposition order is:

```text
STUDY SEMANTICS
-> PORT/ADAPT THE NECESSARY BEHAVIOR TO RUST
-> TEST AGAINST DONOR FIXTURES/BEHAVIOR
-> RETAIN PROVENANCE
```

Use the donor implementation directly only where it is genuinely a replaceable adapter/provider and does not own Fehrest truth or authority.

Every port/adaptation records:

```text
SOURCE_REPOSITORY
SOURCE_COMMIT
SOURCE_PATHS
RIGHTS/LICENSE_EVIDENCE
BEHAVIOR_PORTED
LOCAL_RUST_PATHS
DEVIATIONS
TEST_VECTORS
UPDATE_POLICY
```

---

## 7. Rust workspace growth discipline

The conceptual crate map in `RUST_PLATFORM_ARCHITECTURE.md` is a destination hypothesis, not a scaffolding task.

Rules:

```text
NO_EMPTY_FUTURE_CRATES
NO_CRATE_ONLY_TO_MATCH_PROGRAM_SPEC_ID
NO_DUPLICATED_DOMAIN_TYPES_ACROSS_CRATES
NO_CIRCULAR_CRATE_DEPENDENCIES
NO_UI_CRATE_OWNS_CORE_DOMAIN_TYPES
```

A crate exists only when the active spec needs a stable semantic boundary.

The active plan must include a dependency-direction diagram or table for any multi-crate change.

---

## 8. CI/quality expectations by boundary

### Core Rust

Minimum candidate baseline:

```text
cargo fmt --check
cargo check --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Add where applicable:

```text
property tests
fuzz tests
Miri
fault injection
loom/concurrency testing or equivalent
cargo audit/advisory review
cargo deny/supply-chain policy
native platform matrices
```

### Rust/WASM

Applicable specs should add:

```text
wasm build verification
browser integration tests
memory/resource measurements
accessibility/keyboard/IME tests
browser security boundary tests
```

### FFI/native bridge

Applicable specs should add:

```text
ABI/version compatibility test
invalid-input tests
panic/error containment
resource lifetime tests
platform matrix
bridge deletion/replaceability test where practical
```

---

## 9. Architecture invariants to test continuously

```text
A-01 Removing UI preserves canonical behavior.
A-02 Replacing model provider preserves memory semantics.
A-03 Replacing graph/vector provider preserves canonical state.
A-04 Replacing IDE/browser adapter preserves authorization semantics.
A-05 Replacing sync transport does not redefine grant policy.
A-06 Non-Rust bridge deletion does not erase definition of Fehrest truth.
A-07 Provider IDs never become canonical IDs.
A-08 No UI cache/index becomes authority.
A-09 No foreign runtime can mutate canonical state without Rust validation/chokepoint.
A-10 AI OFF and network-off correctness paths remain Rust-owned.
```

---

## 10. Program closeout conditions for the Rust direction

Before the V2 program can ever claim the founder language direction is implemented coherently:

```text
ACTIVE_PRODUCT_SPECS_WITH_RUST_DECLARATION=100%
CANONICAL_SEMANTICS_RUST_OWNED=100%
AUTHORIZATION_SEMANTICS_RUST_OWNED=100%
MEMORY_SEMANTICS_RUST_OWNED=100%
UNDECLARED_NON_RUST_PRODUCT_LOGIC=0
UNDECLARED_FFI_BOUNDARIES=0
NON_RUST_CANONICAL_AUTHORITY=0
NON_RUST_GRANT_AUTHORITY=0
```

Rust is an architectural ownership rule, not a percentage-of-lines vanity metric.

---

## 11. Current status

```text
RUST_TRACEABILITY_MATRIX=PREPARED
PROGRAM_CANONICAL=NO
R1_CHANGED=NO
CURRENT_CHANGED=NO
IMPLEMENTATION_AUTHORIZED=NO
```
