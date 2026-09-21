# Fehrest Rust-First Convergence Review

**Status:** REVIEW / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Reviewed inputs:** V2 product vision, UX blueprint, feature catalog, AI/Search/WebMCP proposal, Spec Kit V2 program blueprint, spec sequence, ownership matrix, coverage matrix, conflict/gap review, Rust platform architecture, Rust spec traceability matrix.

> This review asks whether the current V2 planning documents can accidentally produce a polyglot product core, duplicate semantic ownership outside Rust, or force a framework decision before evidence exists.

---

## 1. Review verdict

The V2 program is compatible with the founder's Rust-first direction after the Rust platform architecture and cross-spec traceability controls are applied.

Current review state:

```text
RUST_DIRECTION_RECORDED=YES
RUST_SEMANTIC_OWNERSHIP_DEFINED=YES
RUST_CROSS_SPEC_TRACEABILITY_DEFINED=YES
UI_FRAMEWORK_PRESELECTED=NO
EDITOR_IMPLEMENTATION_PRESELECTED=NO
SEARCH_LIBRARY_PRESELECTED=NO
SYNC_LIBRARY_PRESELECTED=NO
AI_PROVIDER_PRESELECTED=NO
WEBMCP_IMPLEMENTATION_PRESELECTED=NO
KNOWN_POLYGLOT_SEMANTIC_CONFLICTS_WITHOUT_CONTROL=0
IMPLEMENTATION_AUTHORIZED=NO
```

This is a planning convergence result only. It is not implementation evidence.

---

## 2. Review question R-C01 — Can the UI become the real application?

### Risk

A Tauri/browser/mobile UI could accumulate state transitions, authorization decisions, search logic and AI orchestration until Rust becomes a thin persistence library.

### Control

```text
UI owns presentation/interactions
Rust owns commands/domain/application semantics
UI sends typed requests
Rust validates and executes
UI receives typed results/events
```

Any future UI spec must list all non-Rust paths and prove they do not own canonical/security/memory/search/sync semantics.

**Status:** CONTROLLED

---

## 3. R-C02 — Can rich-editor pressure force TypeScript business logic?

### Risk

Obsidian/Notion-class editing is the area most likely to tempt the project into adopting a mature JavaScript editor with substantial document semantics.

### Control

The editor benchmark must separate:

```text
rendering/input/composition
from
canonical document identity/format/links/properties/history/permissions
```

Allowed foreign editor bridge:

```text
text/layout/input event handling
selection/cursor/rendering
editor-native transient state
```

Forbidden foreign editor ownership:

```text
canonical file format authority
stable object identity
memory lifecycle
permissions
durable provenance
repository mutation rules
```

If no acceptable bridge boundary exists, the editor direction must be reconsidered explicitly rather than silently moving the product core.

**Status:** CONTROLLED BY FUTURE BENCHMARK

---

## 4. R-C03 — Can donor-code permission create a language architecture conflict?

### Risk

The project has broad permission to copy/adapt donor code. Many high-value donors use TypeScript, Python or mixed stacks.

### Control

Permission changes reuse economics, not architecture authority.

Default for non-Rust donors:

```text
STUDY
-> isolate required behavior/contracts/tests
-> port/adapt to Rust
-> preserve provenance
```

Direct foreign-runtime reuse is reserved for replaceable provider/adapter boundaries.

**Status:** RESOLVED AT PROGRAM LEVEL

---

## 5. R-C04 — Can local LLM support require Python?

### Risk

AI ecosystems often expose Python SDKs first.

### Control

Fehrest does not need to embed model runtimes. Rust owns the provider abstraction and speaks stable HTTP/protocol contracts to:

```text
Ollama
LM Studio
llama.cpp server
vLLM/self-hosted endpoints
remote providers
```

If a provider offers only a Python SDK, the active spec must prefer a protocol endpoint or a replaceable sidecar boundary before embedding Python into product semantics.

**Status:** RESOLVED

---

## 6. R-C05 — Can WebMCP/browser integration require JavaScript authority?

### Risk

Browser APIs may require JS/WASM glue and WebMCP is browser-facing.

### Control

Browser bridge may discover/serialize events and tool schemas. Rust remains responsible for:

```text
origin validation
grant validation
tool classification
consequential-action confirmation policy
prompt-injection boundary
secret policy
receipts/provenance
canonical save/proposal
```

**Status:** RESOLVED

---

## 7. R-C06 — Can GitHub/IDE plugins own memory behavior?

### Risk

Each IDE plugin could implement its own retrieval, caching and authorization behavior.

### Control

IDE plugins are clients of one Rust gateway/CLI/protocol. Repository discovery metadata grants no authority.

```text
PLUGIN != MEMORY ENGINE
PLUGIN != AUTHORIZATION ENGINE
PLUGIN != CONTEXT COMPILER
```

**Status:** RESOLVED

---

## 8. R-C07 — Can graph visualization force a JavaScript graph stack into core?

### Risk

Popular graph rendering libraries are commonly JavaScript-first.

### Control

Graph semantics/projection/filter/identity remain Rust-owned. Rendering is replaceable presentation. Rust-native/WASM/WebGPU candidates are benchmarked first; a thin renderer bridge may be accepted only if necessary.

**Status:** RESOLVED

---

## 9. R-C08 — Can search depend on a non-Rust service and lose local-first behavior?

### Risk

A remote vector/search service could become mandatory for search quality.

### Control

Baseline deterministic local lexical/structured search must remain Rust-owned and work with AI/network off. Graph/vector/rerank providers remain optional derived layers subject to benchmarks.

**Status:** RESOLVED

---

## 10. R-C09 — Can collaboration library semantics replace Fehrest semantics?

### Risk

A CRDT implementation could become the source of object identity, authorization or historical truth.

### Control

Spec 016 evaluates collaboration mechanisms. Spec 017, if activated, owns the Rust mapping from the retained collaboration mechanism into Fehrest identity/provenance/revocation/recovery semantics.

The CRDT is a mechanism, not the product constitution.

**Status:** RESOLVED IN PROGRAM DESIGN; FUTURE C/D REVIEW REQUIRED

---

## 11. R-C10 — Can mobile create a second product implementation?

### Risk

Native iOS/Android apps could duplicate domain logic in Swift/Kotlin.

### Control

Shared Rust domain core is mandatory. Native shells, if required, are limited to platform UI/services and typed bridges. Search, memory, sync, permissions and transformations remain Rust-owned.

**Status:** RESOLVED

---

## 12. R-C11 — Can extensions execute unrestricted foreign code?

### Risk

A plugin ecosystem can defeat Rust safety and capability boundaries if arbitrary Node/Python code gets full repository/process/network access.

### Control

Spec 021 must design a capability-based extension boundary before ecosystem activation. Foreign/WASM/process extensions may be supported only behind explicit manifests, grants and sandbox/process decisions. The extension cannot bypass Rust authorization.

**Status:** OWNED BY SPEC 021

---

## 13. R-C12 — Can FFI/native dependencies undermine the Rust safety claim?

### Risk

A Rust codebase can still inherit unsafe/native memory risks through FFI and transitive crates.

### Control

FFI/native dependencies require explicit trust-boundary, memory-safety, platform and exit documentation. `unsafe` remains forbidden in Fehrest Core under current governance. Supply-chain and unsafe review are evidence requirements where relevant.

**Status:** CONTROLLED

---

## 14. Rust-first success is not percentage-of-lines

Do not optimize for:

```text
99% Rust lines
```

while allowing the remaining 1% to own the editor model, permissions or sync semantics.

The correct metric is semantic ownership:

```text
CANONICAL_SEMANTICS_RUST_OWNED
AUTHORIZATION_RUST_OWNED
MEMORY_RUST_OWNED
SEARCH_CONTEXT_RUST_OWNED
SYNC_POLICY_RUST_OWNED
PROVIDER_GATEWAYS_RUST_OWNED
```

A substantial foreign rendering library can be acceptable when its authority is zero. A tiny JavaScript authorization shortcut is not acceptable.

---

## 15. Future framework selection gates

No framework is selected by this planning branch.

Future active specs must benchmark the relevant candidate family.

### UI/editor

Measure at minimum:

```text
UX fidelity
IME/Arabic/RTL
accessibility
startup
memory
large repositories/documents
offline behavior
mobile
WASM/native integration
maintenance/community maturity
interop boundary size
```

### Search

Measure:

```text
correctness
incremental/fresh equivalence
latency
memory/disk
index rebuild
Unicode/text behavior
large repository scale
```

### Sync/CRDT

Measure:

```text
convergence
revocation under partition
history/provenance compatibility
crash recovery
bandwidth
mobile/resource cost
schema evolution
```

Framework/library selection before those active-spec gates is prohibited by the V2 program proposal.

---

## 16. New required Spec Kit activation gate

Every future executable spec must include a Rust language ownership verdict:

```text
RUST_LANGUAGE_GATE=PASS|BLOCKED
```

PASS requires:

```text
PRIMARY_LANGUAGE_RUST=YES
RUST_OWNED_SEMANTICS_IDENTIFIED=YES
NON_RUST_PATHS_DECLARED=YES|N/A
NON_RUST_EXCEPTION_JUSTIFIED=YES|N/A
SEMANTIC_AUTHORITY_OUTSIDE_RUST=NO
UNDECLARED_FFI=0
UNJUSTIFIED_POLYGLOT_PRODUCT_LOGIC=0
```

A blocked Rust language gate prevents implementation activation unless the founder/architecture governance explicitly changes the language direction.

---

## 17. Remaining honest unknowns

The following remain deliberately unresolved until their active specs:

```text
UI framework
rich editor implementation
WASM framework
mobile shell strategy
lexical search crate
CRDT/sync library
server framework details
protocol choices
AI provider list
WebMCP browser binding implementation
extension sandbox/runtime
```

This is not a gap in the program. Prematurely choosing them would be a planning defect.

---

## 18. Convergence verdict

```text
KNOWN_RUST_ARCHITECTURE_CONFLICTS_IDENTIFIED=12
KNOWN_RUST_ARCHITECTURE_CONFLICTS_WITH_CONTROL_OR_OWNER=12
KNOWN_NON_RUST_SEMANTIC_OWNER=0
KNOWN_PREMATURE_FRAMEWORK_COMMITMENT=0
RUST_LANGUAGE_GATE_DEFINED=YES
RUST_TRACEABILITY_002_TO_022=YES
PROGRAM_CANONICAL=NO
R1_CHANGED=NO
CURRENT_CHANGED=NO
IMPLEMENTATION_AUTHORIZED=NO
```
