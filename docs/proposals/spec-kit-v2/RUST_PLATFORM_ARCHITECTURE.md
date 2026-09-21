# Fehrest Rust-First Platform Architecture

**Status:** FOUNDER TECHNICAL DIRECTION / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Language direction:** Rust-first, Rust-owned product architecture  
**Canonical authority:** unchanged; see `AGENTS.md`, `specs/CURRENT.md`, and the canonical Execution Master Plan.

> This document records the founder direction that Fehrest should be built in Rust. It does not authorize implementation while R1 remains open, does not select a UI/sync/search/provider dependency by itself, and does not override the Constitution/Architecture reconciliation gate.

---

## 1. Founder language decision

Fehrest is a Rust product.

The intended rule is stronger than "the backend is Rust":

```text
RUST_OWNS_PRODUCT_LOGIC=YES
RUST_OWNS_CANONICAL_SEMANTICS=YES
RUST_OWNS_SECURITY_AND_AUTHORIZATION=YES
RUST_OWNS_MEMORY_AND_PROVENANCE=YES
RUST_OWNS_SEARCH_AND_CONTEXT_LOGIC=YES
RUST_OWNS_SYNC_AND_COLLAB_SEMANTICS=YES
RUST_OWNS_AGENT_AND_TOOL_GATEWAY=YES
RUST_OWNS_SERVER_AND_CLI=YES
RUST_OWNS_DESKTOP_NATIVE_LOGIC=YES
RUST_FIRST_WEB_AND_MOBILE=YES
POLYGLOT_BUSINESS_LOGIC=NO
```

Fehrest must not gradually become a TypeScript application with a small Rust core.

---

## 2. What "Rust-first" means

### 2.1 Mandatory Rust-owned layers

The following must be implemented in Rust unless a later approved architecture change explicitly proves a different boundary is required:

```text
canonical repository core
object identity and versioning
event journal and recovery
memory lifecycle and temporal truth
provenance and receipts
authorization and grant evaluation
search/index orchestration
Context Compiler
graph-provider abstraction
AI-provider abstraction
web/tool authorization
GitHub/IDE memory gateway
sync/collaboration semantics
organization policy enforcement
extension capability enforcement
server APIs
CLI
local daemon/gateway
background workers
import/export orchestration
security-sensitive parsing/validation
```

No JavaScript, TypeScript, Python, JVM, Swift, Kotlin, Dart or other runtime may own these semantics merely because a UI/framework makes it convenient.

### 2.2 Rust-first presentation

Human-facing clients should prefer Rust-native or Rust-to-WASM approaches.

Candidate families to benchmark later include:

```text
Dioxus
Leptos
Iced
egui
Tauri shell + Rust/WASM frontend
other mature Rust UI systems discovered during the active UI spec
```

No candidate is selected by this document.

### 2.3 Thin interoperability is allowed

Some platform APIs, browser APIs, OS SDKs, WebMCP bindings, rich-editor internals or third-party components may require JavaScript/TypeScript, Swift, Kotlin, Objective-C, C/C++, WASM glue or another language at an integration boundary.

This is permitted only when all are true:

```text
INTEROP_IS_THIN=YES
INTEROP_HAS_NO_CANONICAL_AUTHORITY=YES
INTEROP_HAS_NO_MEMORY_SEMANTICS=YES
INTEROP_HAS_NO_PERMISSION_SEMANTICS=YES
INTEROP_HAS_NO_HIDDEN_BUSINESS_LOGIC=YES
INTEROP_CONTRACT_IS_TYPED_AND_VERSIONED=YES
RUST_SIDE_VALIDATES_ALL_UNTRUSTED_INPUT=YES
```

The bridge may translate platform events. It may not become the product brain.

---

## 3. Rust workspace direction

A future implementation may evolve toward a Cargo workspace whose boundaries follow semantic ownership rather than UI pages.

Conceptual example only:

```text
crates/
  fehrest-core/             canonical types, identity, invariants
  fehrest-store/            canonical persistence/recovery
  fehrest-events/           event journal/upcasting
  fehrest-memory/           temporal memory/proposals/CI contracts
  fehrest-auth/             principals/grants/policy
  fehrest-search/           lexical/structured retrieval
  fehrest-context/          Context Compiler/receipts
  fehrest-graph/            replaceable derived graph boundary
  fehrest-ai/               provider abstraction/orchestration
  fehrest-web/              external evidence/WebMCP boundary
  fehrest-github/           GitHub binding/provenance
  fehrest-sync/             collaboration/sync semantics
  fehrest-org/              organization policy/admin domain
  fehrest-import/           migration/import contracts
  fehrest-extensions/       capability-based extensions
  fehrest-protocol/         stable external contracts where needed
  fehrest-cli/              human/agent CLI
  fehrest-server/           hosted/self-hosted service
  fehrest-desktop/          desktop native shell/application
  fehrest-web-client/       Rust/WASM web client if retained
  fehrest-mobile/           Rust-first mobile client/shared logic
```

This is not an authorization to create every crate early.

Ponytail still applies:

```text
NO_CRATE_WITHOUT_CURRENT_REQUIREMENT
NO_PREMATURE_MICROCRATES
NO_ARCHITECTURE_BY_DIRECTORY
```

Early specs should use the minimum crate split that preserves ownership and testability.

---

## 4. Async/runtime direction

Where async I/O is required, the active spec should evaluate Rust-native runtime choices and prefer the smallest mature option that closes the requirement.

Likely research family:

```text
Tokio
async-std/smol only if a measured requirement favors them
```

Do not make an async runtime part of canonical file-format identity.

---

## 5. Server and network services

Fehrest server, sync relay, gateway and hosted control planes should be Rust services.

Candidate ecosystem families for later plans may include:

```text
Axum / Tower
Hyper
Tokio
Rustls
tonic for gRPC only where a contract requires it
Serde for typed serialization
```

Protocol choice remains owned by the relevant spec.

Rust is the implementation direction; this document does not pre-authorize any dependency.

---

## 6. Local-first storage

Rust must own the persistence semantics even when Fehrest uses a platform or external database engine.

Important distinction:

```text
APPLICATION_LANGUAGE = Rust
STORAGE_ENGINE_IMPLEMENTATION_LANGUAGE != product authority
```

A mature external database may still be used if a spec proves it is the best fit. The database does not own Fehrest object identity, grants, memory lifecycle or canonical policy.

Canonical human-owned content should remain open/inspectable according to the relevant format spec.

Candidate storage approaches remain research questions, for example:

```text
open files + Fehrest metadata/journal
SQLite where measured needs justify it
Rust-native embedded stores where justified
PostgreSQL for hosted/team projections where justified
```

No database is selected here.

---

## 7. Search

Search should be Rust-owned end to end.

The expected layering remains:

```text
canonical filters
-> Rust lexical/structured retrieval
-> optional derived graph
-> optional derived vector
-> optional reranker
-> authorization/scope filtering
-> trace/receipt
```

Candidate Rust-native lexical/search technology such as Tantivy may be studied and benchmarked in Spec 003, but no library is selected by this founder direction.

A UI client must not maintain an independent authoritative search pipeline.

---

## 8. Graph

Graph visualization and graph intelligence remain separate.

Rust responsibilities:

```text
canonical explicit link model
query/filter graph projection
optional GraphProvider boundary
provenance/identity mapping
selection trace
graph authorization/scope
```

The rendering client may use Rust/WASM, WebGPU/wgpu, SVG/canvas bindings or another measured Rust-first presentation strategy.

A JavaScript graph renderer, if ever required as a thin bridge, cannot own graph identity or retrieval semantics.

---

## 9. AI and local LLMs

AI integration is provider-driven but Rust-owned.

Fehrest should expose one Rust provider abstraction capable of supporting:

```text
AI OFF
local HTTP endpoints
self-hosted endpoints
remote managed providers
custom compatible endpoints
```

Likely local integrations include OpenAI-compatible or similar endpoints exposed by systems such as Ollama, LM Studio, llama.cpp and self-hosted inference servers.

Rust owns:

```text
provider configuration model
capability probing
context authorization
request assembly
stream handling
structured-output validation
tool-call validation
failure classification
usage/cost metadata normalization
receipt generation
```

The model cannot become authority.

---

## 10. WebMCP and browser interoperability

WebMCP/browser integration must preserve the Rust-owned trust boundary.

Conceptual flow:

```text
Browser/WebMCP API
      |
      v
thin binding / WASM bridge
      |
      v
Rust Web Tool Gateway
      |
      +-> origin validation
      +-> grant validation
      +-> READ/ACTION classification
      +-> prompt-injection boundary
      +-> receipt/provenance
      +-> canonical save/proposal path
```

Browser-native APIs may require `wasm-bindgen`, `web-sys` or equivalent bindings. That does not transfer authority out of Rust.

---

## 11. GitHub and IDE integration

The GitHub/IDE path should be Rust-owned and client-independent.

Potential components:

```text
fehrest CLI
local Rust daemon/API
Rust SDK
MCP server implemented in Rust
GitHub App/server integration implemented in Rust
repo discovery parser implemented in Rust
```

IDE-specific plugins should be optional adapters.

The target remains:

> Any IDE or agent that can invoke the stable Fehrest protocol can use the same memory without Fehrest implementing the IDE itself.

---

## 12. Collaboration and CRDTs

The founder's Rust direction strengthens the candidate set for the collaboration experiment.

Spec 016 should strongly benchmark Rust-capable candidates such as:

```text
Automerge Rust
Loro
Yrs / Yjs-compatible Rust implementations
Y-Octo or current successor architecture
other current Rust-capable local-first candidates
```

Selection still depends on correctness, convergence, recovery, provenance, authorization, performance and mobile evidence.

`RUST_IMPLEMENTATION` does not waive the collaboration experiment.

---

## 13. Desktop

Desktop must preserve Rust ownership.

Candidate approaches:

### A. Rust-native UI

```text
Dioxus desktop
Iced
egui
other mature Rust-native candidate
```

### B. Rust/WASM UI in a native shell

```text
Tauri
+ Rust/WASM frontend such as Leptos/Dioxus where viable
+ Rust application core
```

Tauri itself does not imply that Fehrest must use TypeScript application logic.

The desktop UI spec must benchmark:

```text
editor quality
startup
memory
large-vault behavior
accessibility
keyboard UX
IME/international text
rendering performance
graph performance
platform integration
packaging/update/security
```

Do not choose the framework before this evidence exists.

---

## 14. Web

If a web client is authorized, Rust/WASM is preferred.

Candidate UI families may include Leptos or Dioxus and browser bindings through Rust/WASM.

Rules:

```text
WEB_CLIENT_CAN_BE_DERIVED/PRESENTATION=YES
WEB_CLIENT_CAN_OWN_CANONICAL_RULES=NO
WEB_CLIENT_CAN_MINT_AUTHORITY=NO
```

Any required JavaScript glue remains minimal and audited.

---

## 15. Mobile

Mobile should reuse Rust domain/core logic to the maximum practical extent.

Potential approaches are to be benchmarked, including:

```text
Dioxus mobile
Tauri mobile + Rust core / Rust-WASM where viable
native Swift/Kotlin shell with Rust core only if platform requirements force it
```

If Swift/Kotlin is required for OS integration:

```text
NATIVE_SHELL != PRODUCT_SEMANTICS
```

Rust remains the source of truth for memory, sync, authorization, search and data transformations.

---

## 16. Rich editor risk

The most important Rust-first product risk is the editor.

Fehrest wants an Obsidian/Notion-class writing experience, and mature browser editor ecosystems are often JavaScript-heavy.

Therefore the personal workspace spec must include an explicit editor capability benchmark rather than quietly abandoning Rust-first constraints.

Required benchmark dimensions:

```text
Markdown fidelity
IME/composition correctness
Arabic/RTL and international text
large documents
undo/redo
selection
copy/paste
code/math/embed blocks
wikilinks/backlinks interactions
collaborative editing compatibility
accessibility
mobile behavior
plugin/extensibility boundary
crash recovery
```

Allowed outcomes:

```text
RUST_NATIVE_EDITOR_RETAIN
RUST_WASM_EDITOR_RETAIN
THIN_FOREIGN_EDITOR_BRIDGE_REQUIRED
EDITOR_DIRECTION_RECONSIDER
```

If a foreign editor bridge is required, the exception must be explicit, narrow and architecture-reviewed. It does not authorize moving data semantics or business logic out of Rust.

---

## 17. Rust dependency policy

Rust-first does not mean dependency-heavy.

Every crate still passes:

```text
requirement
-> std/existing Fehrest option
-> donor/adapt option
-> dependency option
-> provenance/license
-> security
-> maintenance
-> benchmark where material
-> Ponytail decision
```

Prefer:

```text
small dependency graph
mature crates
pinned lockfile
minimal features
default-features=false where appropriate
no unsafe in Fehrest Core unless a future dedicated policy explicitly changes this
```

Transitive native code and `unsafe` in external dependencies must be visible in security/dependency review when relevant.

---

## 18. FFI and unsafe policy

Current repository rules forbid `unsafe` in Fehrest Core.

Rust-first must not be used as an excuse to add opaque FFI everywhere.

For any FFI/native dependency:

```text
WHY_REQUIRED
TRUST_BOUNDARY
MEMORY_SAFETY_MODEL
INPUT_VALIDATION
FAILURE_MODEL
UPDATE_POLICY
PLATFORM_MATRIX
EXIT_STRATEGY
```

must be documented.

Unsafe code outside the protected Core boundary, if ever considered, requires explicit review and may be rejected by the reconciled Constitution.

---

## 19. Spec Kit language gate

Every future executable `plan.md` must state:

```text
Primary language: Rust
Rust edition/toolchain: pinned by repository
Non-Rust code required: YES|NO
If YES: exact paths + purpose + why Rust cannot close the requirement
Semantic authority outside Rust: MUST be NO
```

Activation must fail if material product/business/security logic is placed outside Rust without an approved architecture exception.

Suggested readiness conditions:

```text
RUST_PRIMARY_LANGUAGE=YES
RUST_OWNS_CANONICAL_SEMANTICS=YES
RUST_OWNS_AUTHORIZATION=YES
RUST_OWNS_MEMORY=YES
RUST_OWNS_PROVIDER_GATEWAYS=YES
UNJUSTIFIED_POLYGLOT_PRODUCT_LOGIC=0
```

---

## 20. Testing and quality baseline

Every Rust implementation spec should include the applicable subset of:

```text
cargo fmt --check
cargo check --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo deny / equivalent dependency policy where adopted
cargo audit / advisory review where adopted
Miri where useful
property tests
fuzzing for parsers/untrusted boundaries
fault injection for persistence/sync
native-platform tests
WASM/browser tests where applicable
```

Exact tools remain spec-owned and must not be claimed PASS without evidence.

---

## 21. Performance philosophy

Rust is chosen to help Fehrest become:

```text
fast to start
low-memory
responsive on large repositories
offline-capable
battery-conscious
predictable under load
safe under concurrency
suitable for local and server deployment
```

Performance claims must still be benchmarked. Rust alone is not evidence of speed.

---

## 22. Architecture test

A proposed feature passes the Rust architecture test only when:

1. Removing the UI leaves canonical behavior intact in Rust.
2. Replacing the AI provider leaves canonical memory intact.
3. Replacing the sync transport leaves canonical semantics explicit in Rust.
4. Replacing a web/IDE adapter does not change authorization semantics.
5. Non-Rust bridges can be deleted without losing the definition of Fehrest truth.
6. A Rust client can consume the same contracts as any other client.

---

## 23. Current decision

```text
FOUNDER_LANGUAGE_DIRECTION=RUST
RUST_FIRST_PLATFORM=YES
RUST_OWNED_SEMANTICS=YES
NON_RUST_INTEROP=THIN_ONLY_WHEN_JUSTIFIED
UI_FRAMEWORK_SELECTED=NO
SYNC_LIBRARY_SELECTED=NO
SEARCH_LIBRARY_SELECTED=NO
AI_PROVIDER_SELECTED=NO
R1_CHANGED=NO
CURRENT_CHANGED=NO
IMPLEMENTATION_AUTHORIZED=NO
```
