# Fehrest V2 Cross-Spec Invariants and Ownership

**Status:** PROGRAM PROPOSAL / NON-AUTHORIZING  
**Created:** 2026-08-28

> This document prevents semantic overlap across future Spec Kits. It does not authorize implementation or replace existing canonical governance.

---

## 1. Why this file exists

The V2 product direction spans personal knowledge, search, graph, AI, GitHub, web research, collaboration, team communication and enterprise policy.

Without explicit ownership, multiple specs could independently define the same concept and create incompatible behavior.

The rule is:

```text
ONE SEMANTIC RESPONSIBILITY -> ONE OWNING SPEC
```

A later spec may consume or extend an earlier contract, but may not silently redefine it.

---

# 2. Program-wide invariants

These invariants apply to every future spec unless a separately authorized change-control process changes them.

## I-01 — One active frontier

Exactly one implementation/experiment frontier is active.

## I-02 — Canonical state is Fehrest-owned

No external provider becomes canonical authority.

```text
model != authority
graph != authority
vector store != authority
search rank != authority
web content != authority
GitHub content != Fehrest authorization
plugin != authority
```

## I-03 — Derived state is rebuildable

Any index, graph projection, embedding, rank cache or UI layout marked derived must be rebuildable from canonical/evidence inputs.

## I-04 — Stable identity is independent of paths/providers

Filesystem paths, graph IDs, vector IDs, provider IDs and UI routes must not become canonical object identity.

## I-05 — Content is evidence, never capability

Retrieved text, web pages, messages, notes, tool descriptions and model outputs cannot create a grant.

## I-06 — Agent inference is not confirmed memory

An agent may propose memory. Promotion to an authoritative lifecycle state follows canonical review/policy semantics.

## I-07 — AI OFF remains complete

Core repository correctness, local notes, deterministic search/navigation, recovery and export do not require a model or network service.

## I-08 — Local ownership remains a product invariant

A hosted/sync service must not become the only usable copy of user-owned canonical knowledge.

## I-09 — Secrets are not canonical knowledge

Credentials/tokens/secret material are referenced through dedicated secret/configuration mechanisms and are excluded from canonical notes/memory/event detail/trajectories by design.

## I-10 — Authorization is checked at the data boundary

UI hiding is not authorization. Every read/write/tool/context path must converge on canonical scope/grant enforcement.

## I-11 — External origin is preserved

Imported/web/GitHub/external evidence retains provenance sufficient to distinguish source bytes, normalized content and Fehrest-authored conclusions.

## I-12 — Temporal state is explicit

When a concept can change over time, current truth must not silently overwrite historical truth.

## I-13 — Failure is visible

Unsupported versions, stale sources, provider mismatch, sync conflict, recovery state and authorization denial fail visibly rather than silently guessing.

## I-14 — Migration precedes format break

No durable format/contract changes without explicit compatibility and migration/upcast policy.

## I-15 — UI is a projection of semantics

Product surfaces must not create alternate hidden sources of truth.

## I-16 — Provider boundaries are replaceable

Model, graph, vector, web, browser, sync, media and connector implementations are selected behind Fehrest-owned interfaces where the requirement warrants a provider boundary.

## I-17 — Rights/provenance survive code reuse

Any copied/adapted donor code retains exact source repository/revision/path and permission/license evidence.

## I-18 — Negative experimental results are preserved

A failed graph/collaboration/provider experiment remains evidence and cannot be relabeled to justify implementation.

## I-19 — Rust owns product semantics

Founder technical direction requires Rust to own Fehrest product semantics across the program.

```text
canonical semantics      -> Rust
authorization/grants     -> Rust
memory/provenance        -> Rust
search/context logic     -> Rust
sync/collaboration policy-> Rust
agent/tool gateways      -> Rust
server/CLI/native logic  -> Rust
```

Non-Rust code is permitted only as an explicit, typed, replaceable presentation/platform/provider bridge when the active spec proves the need.

```text
NON_RUST_CANONICAL_AUTHORITY=0
NON_RUST_GRANT_AUTHORITY=0
NON_RUST_MEMORY_AUTHORITY=0
UNJUSTIFIED_POLYGLOT_PRODUCT_LOGIC=0
```

Every future executable Spec Kit must pass the Rust language gate defined in `RUST_PLATFORM_ARCHITECTURE.md`, `RUST_SPEC_TRACEABILITY_MATRIX.md`, and `SPEC_AUTHORING_CHECKLIST.md`.

---

# 3. Canonical persistence classes

Every future `data-model.md` must classify persisted state using one of these classes.

| Class | Meaning | Recovery authority |
|---|---|---|
| `CANONICAL` | Irreplaceable user/system truth | Must survive according to owning durability contract |
| `DERIVED_REBUILDABLE` | Reconstructable projection/index/rank | May be deleted/rebuilt; no authority |
| `CONFIGURATION` | Local/user/org settings not themselves knowledge | Restored by config policy; may reference secrets |
| `SECRET_REFERENCE` | Pointer/handle to protected secret storage | Secret bytes excluded from canonical knowledge |
| `CACHE` | Disposable optimization | No recovery requirement beyond safe invalidation |
| `EVIDENCE_ARTIFACT` | Raw benchmark/import/acquisition/verification evidence | Preserved under evidence retention rules |

A future spec may add a class only through an explicit architecture review.

---

# 4. Semantic ownership matrix

## 4.1 Repository/core

| Responsibility | Owner | Consumers/extensions |
|---|---|---|
| Vault/Memory Repository stable identity | 002 | all later specs |
| Canonical format/version envelope | 002 | 006, 010, 017 |
| Crash-safe canonical write boundary | 002 | all canonical writers |
| Writer ownership baseline | 002 | 006, 010, 017 |
| Event journal envelope/history integrity | 002 | all later event-producing specs |
| Startup recovery baseline | 002 | 006, 010, 017 |

Later specs may add typed payloads/entities but may not redefine core event integrity.

---

## 4.2 Retrieval/graph

| Responsibility | Owner | Consumers/extensions |
|---|---|---|
| Lexical/FTS derived index semantics | 003 | 007, 012, 013 |
| Incremental/fresh equivalence | 003 | all retrieval consumers |
| Search candidate trace baseline | 003 | 007/012 |
| Graph capability experiment | 004 | program decision |
| Production derived graph provider | 005 conditional | 007/012 |
| Explicit canonical links between workspace objects | 010 | 012 visualization |
| Graph visualization/layout | 012 | UI only |
| Derived/inferred graph overlay | 005 + 012 | optional UI overlay |

Critical distinction:

```text
explicit canonical links != derived graph intelligence
```

---

## 4.3 Memory/context

| Responsibility | Owner | Consumers/extensions |
|---|---|---|
| Memory canonical schema/lifecycle | 006 | 007/013/019 |
| Temporal memory resolution | 006 | 007/012/013 |
| Memory Proposal semantic state machine | 006 | 011/013/019/020 UI |
| Memory CI semantic contract | 006 | 013/019 automation |
| Context Compiler | 007 | IDE, AI, web, agents |
| Principal/session/grant baseline | 007 | 008/013/014/018/021 |
| Context package/receipt | 007 | all agent/model consumers |
| SelectionTrace | 007 | 008/009/013 |

No UI/AI/team spec may define an alternate memory lifecycle.

---

## 4.4 GitHub/IDE

| Responsibility | Owner | Consumers/extensions |
|---|---|---|
| GitHub repository binding | 008 | IDE/agents/team UI |
| `.fehrest/link.toml` or equivalent discovery contract | 008 | IDE/CLI/SDK/MCP |
| GitHub event evidence mapping | 008 | memory proposals/search |
| GitHub-specific authentication integration | 008 with 007 authorization | IDE/GitHub App |
| IDE-specific presentation | adapters/clients consuming 008/007 | not canonical |

GitHub may identify project context but cannot grant Fehrest scope.

---

## 4.5 Workspace objects

| Responsibility | Owner | Consumers/extensions |
|---|---|---|
| Space/Project canonical object semantics | 010 | 011/012/018/019 |
| Note/Document canonical mapping/open format | 010 | 011/012/015/019 |
| Source canonical reference object | 010, provenance rules from core | 014/015 |
| Task canonical semantics | 010 | 011/018/019/020 |
| Decision workspace object | 010 referencing 006 memory/provenance semantics | 011/019 |
| Attachment reference semantics | 010 | personal/team/mobile |
| Archive/trash recovery semantics | 010 | 011/019 |
| Editor presentation | 011 | desktop/web |
| Capture/Inbox UX | 011 | mobile extends in 020 |
| Collections/Bases view definitions | 012 | personal/team views |

If `Decision` overlaps memory lifecycle, 006 owns memory truth and 010 owns workspace presentation/reference semantics.

---

## 4.6 AI/model

| Responsibility | Owner | Consumers/extensions |
|---|---|---|
| AI provider abstraction | 013 | Ask/inline/research/team assistants |
| Model capability probe | 013 | all model callers |
| Provider/model selection UX | 013 | personal/team/mobile |
| Local vs remote privacy indicator | 013 | all model surfaces |
| Ask Fehrest orchestration | 013 | desktop/mobile/team |
| Inline AI edit preview/diff | 013 + editor UI consumer | notes/docs |
| Model-produced Memory Proposal | 013 using 006 lifecycle | memory review |

Model configuration is not encoded into canonical knowledge objects unless a provenance record intentionally names the model used for an evidence-producing transformation.

---

## 4.7 Web/external evidence

| Responsibility | Owner | Consumers/extensions |
|---|---|---|
| Web authorization policy | 014 using 007 grants | agents/Ask/research |
| External source acquisition contract | 014 | import/research |
| WebMCP provider abstraction | 014 | agents/AI |
| Origin/domain enforcement | 014 | all web tools |
| Read/action tool classification | 014 | human confirmation/policies |
| Web invocation receipt | 014 | audit/provenance |
| External source freshness/recheck | 014 | search/memory |
| Prompt-injection boundary | 014 + program security invariant | all web content |

A source may become a canonical `Source` object through 010 semantics, but 014 owns how external evidence is acquired and attested.

---

## 4.8 Import/migration

| Responsibility | Owner | Consumers/extensions |
|---|---|---|
| Import adapter contract | 015 | 021 future extensions |
| Mapping/dry-run preview | 015 | UI |
| Imported-batch provenance | 015 | search/memory |
| Unsupported-field report | 015 | user/admin |
| Import rollback semantics | 015 | repository recovery |
| Export from canonical formats | owning canonical specs + shared export contract defined during 010/015 | Hub/team/mobile |

Importers must not bypass canonical writer/migration semantics.

---

## 4.9 Collaboration/sync

| Responsibility | Owner | Consumers/extensions |
|---|---|---|
| Collaboration mechanism experiment | 016 | decision only |
| Replica/device identity | 017 | 018/019/020 |
| Sync protocol/provider boundary | 017 | desktop/mobile/team |
| Multi-writer merge/conflict semantics | 017 | shared objects |
| Revocation-under-partition behavior | 017 + 018 policy | team/mobile |
| Sync health/status semantics | 017 | UI |
| Conflict representation | 017 | 019/020 UX |

No channel/editor UI may create its own hidden sync semantics.

---

## 4.10 Organization/team

| Responsibility | Owner | Consumers/extensions |
|---|---|---|
| Organization identity | 018 | 019/020/021/022 |
| Membership/groups/roles | 018 | team surfaces |
| Guest/external collaborator model | 018 | channels/docs |
| Organization policy/audit/admin | 018 | all shared access |
| Channels/topics/messages | 019 | mobile/extensions |
| DMs/group DMs | 019 | mobile |
| Presence/typing/read state | 019 | mobile |
| Notification semantics | 019, mobile delivery in 020 | clients |
| Knowledge crystallization UX | 019 using 006 Memory Proposal | team memory |

Team communication cannot precede the authorization model it depends on.

---

## 4.11 Mobile/extensions/Hub

| Responsibility | Owner | Consumers/extensions |
|---|---|---|
| Mobile presentation/offline capture | 020 | human users |
| Extension manifest/capability model | 021 | ecosystem |
| Automation trigger/action contract | 021 | user/team/admin |
| Public/private hosted Memory Repository discovery | 022 | humans/agents |
| Hub watch/follow/citation/proposal | 022 | network |
| Hub copy/fork provenance | 022 | local repositories |

Hub hosting does not redefine local canonical format.

---

# 5. Contract ownership rules

Every contract file must declare:

```text
CONTRACT_ID
OWNER_SPEC
VERSION
CANONICAL_OR_DERIVED
COMPATIBILITY_RULE
AUTHORIZATION_BOUNDARY
```

Examples:

```text
CTX-PACKAGE-v1        owner=007
GITHUB-LINK-v1        owner=008
MEMORY-PROPOSAL-v1    owner=006
AI-PROVIDER-v1        owner=013
WEB-TOOL-v1           owner=014
SYNC-v1               owner=017
ORG-POLICY-v1         owner=018
CHANNEL-EVENT-v1      owner=019
EXTENSION-MANIFEST-v1 owner=021
```

No two active specs may independently issue the same contract version.

---

# 6. Change protocol for shared contracts

When a later spec needs to change an earlier owned contract:

1. State the new requirement in the later `spec.md`.
2. Reference the owning contract/spec.
3. Classify the compatibility impact.
4. Update or version the owning contract through an authorized change.
5. Add backward/forward tests.
6. Update consumers.
7. Preserve old fixtures where durable compatibility is claimed.

Never fork an internal contract silently.

---

# 7. Data authority examples

### Example: semantic search

```text
Note Markdown                = CANONICAL
Note properties              = CANONICAL
Lexical index                = DERIVED_REBUILDABLE
Embedding                    = DERIVED_REBUILDABLE
Vector index row             = DERIVED_REBUILDABLE
Search ranking               = DERIVED_REBUILDABLE
Saved search definition      = CANONICAL or CONFIGURATION per owning spec decision
Search result UI state       = CACHE/CONFIGURATION
```

### Example: AI research

```text
User research task           = CANONICAL workspace object if saved
Web page bytes/snapshot      = EVIDENCE_ARTIFACT or canonical Source payload per spec
Normalized page              = EVIDENCE/DERIVED with provenance
LLM answer                   = DRAFT/derived unless explicitly saved
Memory Proposal              = CANONICAL proposal record
Approved memory              = CANONICAL memory
Provider API key             = SECRET_REFERENCE
```

### Example: team message to decision

```text
Message                      = CANONICAL shared workspace record
AI summary                   = DERIVED/DRAFT
Decision candidate           = CANONICAL proposal/workspace object as specified
Active durable memory        = CANONICAL only after 006 lifecycle authorization
```

---

# 8. Conflict prevention checklist

Before a new spec is authorized:

```text
[ ] Every canonical entity has one owner.
[ ] Every mutation transition has one owner.
[ ] Every external contract has one owner.
[ ] No derived provider becomes authority.
[ ] No UI-only store contains irreplaceable knowledge.
[ ] No model/web/tool can widen grants.
[ ] No later spec silently changes an earlier durable format.
[ ] Offline/failure behavior is explicit.
[ ] Security scope is enforced below presentation.
[ ] Migration and recovery ownership are known.
[ ] Donor/library selection is not embedded prematurely in product requirements.
[ ] Rust-owned semantic paths are explicit.
[ ] Non-Rust paths are declared and limited to approved bridge/presentation/provider roles.
[ ] No non-Rust component owns canonical, grant, memory, search authority or sync policy.
[ ] FFI/native/unsafe boundaries are declared and reviewed.
```

---

# 9. Current state

```text
CROSS_SPEC_INVARIANTS=PROPOSED
OWNERSHIP_MATRIX=PROPOSED
RUST_SEMANTIC_OWNERSHIP_INVARIANT=PROPOSED
CANONICAL_GOVERNANCE_CHANGED=NO
IMPLEMENTATION_AUTHORIZED=NO
```
