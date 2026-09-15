# Fehrest Product Gap Review v2

**Date:** 2026-08-28  
**Status:** REVIEW / NON-AUTHORIZING  
**Change class:** identifies Class C/D/E questions; does not resolve them  
**Founder direction reviewed:** `docs/product/FOUNDER_PRODUCT_VISION_V2.md`  
**Competitive input:** `docs/research/COMPETITIVE_CAPABILITY_MATRIX_2026-08-28.md`

> This review does not modify R1, authorize implementation, activate Spec 002, or claim the historical architecture already covers the expanded founder direction.

---

## 1. Executive conclusion

The existing Fehrest architecture is unusually strong in the areas that matter most for a trustworthy memory substrate:

```text
canonical vs derived separation
local-first/open-state intent
Rust-owned correctness/security semantics
provenance discipline
temporal memory concepts
bounded context compilation
agent authority separation
recovery/failure thinking
benchmark-driven feature admission
```

The expanded founder direction is therefore **compatible in spirit** with Fehrest, but it is **not already covered in product scope**.

The major gap is not "more AI."

The major gap is the transition from:

```text
single-user headless thesis-proof memory/context core
```

toward:

```text
universal local-first memory workspace
for individuals + teams + companies + arbitrary agents
```

That transition introduces several architecture-semantic and product-thesis questions that must be reconciled after R1 rather than smuggled into current work.

---

## 2. Gap summary

| ID | Gap | Status | Severity | Class tendency |
|---|---|---|---|---|
| G-V2-01 | Universal product identity not canonicalized | MISSING | Critical | E |
| G-V2-02 | GitHub <-> Fehrest repository binding | MISSING | Critical | C/E |
| G-V2-03 | IDE-independent Fehrest discovery | MISSING | Critical | C |
| G-V2-04 | Human notes UX competitive with Obsidian | PARTIAL/PLANNED | Critical | C/E |
| G-V2-05 | Structured collections/bases/views | MISSING | High | C |
| G-V2-06 | Team channels/topics/DMs | MISSING | Critical | E/C |
| G-V2-07 | Durable topic -> knowledge crystallization | PARTIAL CONCEPT | Critical | C/D |
| G-V2-08 | Multi-user local-first collaboration | MISSING / ARCH CONFLICT | Critical | C/D/E |
| G-V2-09 | Multi-device sync model | MISSING | Critical | C/D |
| G-V2-10 | Organization/guest/admin authorization | MISSING | Critical | D/E |
| G-V2-11 | Agent-agnostic memory/context gateway | PLANNED LATER | Critical | C/D |
| G-V2-12 | Memory Proposal review workflow | MISSING | High | C/D |
| G-V2-13 | Memory CI | MISSING | High | C/D |
| G-V2-14 | Import/migration product | MISSING | Critical for adoption | C |
| G-V2-15 | Unified cross-surface search/Ask | PARTIAL CORE IDEAS | Critical | C |
| G-V2-16 | Personal/team privacy and encryption model | PARTIAL/UNRESOLVED | Critical | D/E |
| G-V2-17 | Mobile capture/sync expectations | MISSING | High | C/E |
| G-V2-18 | Extension/connectors ecosystem | FUTURE/UNRESOLVED | High | C/D |
| G-V2-19 | Fehrest Hub/network effects | MISSING | Strategic | E |
| G-V2-20 | Voice/video/huddles product boundary | MISSING | Medium | C |
| G-V2-21 | Product onboarding/template system | MISSING | High | E |
| G-V2-22 | Team/company deployment model | MISSING | Critical | D/E |
| G-V2-23 | Memory benchmark set lacks current V2 expansion | PARTIAL | High | C |
| G-V2-24 | Donor rights/provenance registry for actual reuse | PARTIAL | High | D |
| G-V2-25 | R1 does not test the expanded V2 thesis | EXPECTED GAP | Critical | E |
| G-V2-26 | Current GitHub bootstrap lacks historical implementation/evidence bytes | ACTIVE BLOCKER | Critical current | provenance gate |

---

## 3. G-V2-01 — Universal product identity

### Finding

The existing repository describes Fehrest primarily through the memory/context thesis and phased proof. The founder direction now explicitly requires Fehrest to become the primary workspace for:

```text
individuals
teams
companies
non-developers
developers
humans
agents
```

### Risk

Without a canonical product identity, roadmap work can fragment into:

```text
AI memory tool
Obsidian clone
Notion clone
Slack clone
agent framework
```

### Recommendation

After R1, conduct a Class E product-thesis reconciliation. Preserve the current technical invariants unless evidence shows they conflict, but explicitly decide whether the canonical product category becomes:

```text
LOCAL-FIRST MEMORY REPOSITORY + WORKSPACE
```

### Current action

Documentation only. Do not reinterpret R1 as proof of the expanded market/product thesis.

---

## 4. G-V2-02/03 — GitHub binding and IDE-independent discovery

### Finding

The expanded strategy depends on Fehrest becoming a natural companion to GitHub, but no canonical integration contract currently exists.

### Required future capability

```text
GitHub repository
-> discover bound Fehrest Memory Repository
-> connect through an authorized local/remote gateway
-> request task-scoped context
-> receive context + receipt
-> optionally return evidence/trajectory/memory proposal
```

### Design requirements

A repository discovery record should be:

```text
small
versioned
human-readable
non-secret
portable
IDE-independent
incapable of granting authorization
```

A future GitHub App should use minimum permissions and treat GitHub activity as evidence, not Fehrest authority.

### Key acceptance test

A fresh IDE/agent with repository access and valid Fehrest credentials can discover and request project memory **without** a custom Fehrest IDE.

### Risk

If each IDE requires bespoke integration logic, Fehrest will fail to become infrastructure.

---

## 5. G-V2-04/05 — Personal notes and structured workspace

### Finding

The architecture contains strong open/canonical data ideas and a future editor gate, but the product direction now requires Obsidian-class personal knowledge and Notion-class structured views.

### Missing product requirements

```text
fast local note creation
backlinks/wikilinks
properties
attachments
PDF/source annotation
daily notes
templates
saved searches
collections/bases
tables
boards
calendar/timeline
canvas
excellent keyboard UX
mobile capture
```

### Architecture rule

Views must remain derived/presentational. The canonical memory must remain recoverable without the view implementation.

### Recommendation

Create a future `Human Knowledge Surface` spec only after the underlying canonical/memory proof remains justified. Keep editor technology selection benchmark-gated.

---

## 6. G-V2-06/07 — Communication and knowledge crystallization

### Finding

The current plan does not make Slack/Zulip-class communication a first-class product surface.

The founder direction requires teams to be able to stay inside Fehrest rather than splitting communication and memory across products.

### Proposed product semantics

```text
Workspace
-> Space
-> Channel
-> Topic
-> messages + tasks + decisions + docs + sources + agent activity
```

### Defining differentiation

Chat is evidence; durable knowledge is reviewed state.

Proposed crystallization flow:

```text
conversation
-> candidate knowledge
-> review/checks
-> confirmed memory/decision/procedure
-> temporal lifecycle
```

### Risk

If Fehrest simply stores chat and runs embeddings over it, Slack plus an AI search layer remains sufficient.

### Recommendation

Treat topic organization and crystallization as product-defining mechanisms in the post-R1 architecture review.

---

## 7. G-V2-08/09 — Multi-user collaboration and sync

### Finding

This is the largest architecture gap.

The historical/current design emphasizes conservative single-writer canonical semantics. The founder direction requires eventual:

```text
multi-user
multi-device
offline edits
real-time collaboration
conflict resolution
team authorization
```

### Conflict

These goals cannot be assumed compatible with the historical writer model without explicit architecture work.

### Required gate

Create a dedicated **Collaboration Capability Experiment** before selecting a CRDT/sync architecture.

Candidate benchmark set:

```text
Automerge
Yjs/Yrs
Y-Octo
Loro
AFFiNE/OctoBase patterns
```

Keyhive may be studied for authorization/local-first research subject to maturity/security evidence.

### Must measure

```text
convergence
offline merge
large document/workspace cost
sync bandwidth
mobile performance
crash recovery
history/provenance compatibility
permission changes during offline work
schema evolution
canonical export/recovery
```

### Security questions

```text
Can revoked users publish offline edits later?
How are actor/device identities bound?
What does authorization mean at merge time?
How are secrets and private objects scoped?
Can derived collaboration metadata become authority accidentally?
```

### Recommendation

Do not choose a CRDT in advance. This needs Class C/D review and may require Class E sequencing decisions.

---

## 8. G-V2-10/16/22 — Organization security and deployment

### Missing requirements

```text
organizations/workspaces
roles/policies
guests
external collaborators
private spaces
team administration
device/session management
self-hosting/on-prem where required
backup/export
retention policy
eDiscovery/compliance boundaries where claimed
E2EE/private data model where claimed
```

### Critical design distinction

Fehrest should not promise mutually incompatible enterprise models simultaneously.

For example:

```text
server-readable enterprise search/eDiscovery
vs
strict end-to-end encrypted content
```

requires explicit product/security modes and honest trade-offs.

### Recommendation

Create a security/product modes decision before enterprise claims.

---

## 9. G-V2-11 — Universal agent gateway

### Finding

The master plan already places the agent gateway later, which aligns with the architecture's caution. The expanded direction makes the **universality** of this gateway strategically more important.

### Future requirement

Agents from different runtimes should consume one Fehrest memory substrate through standard interfaces.

```text
CLI
SDK
local/HTTP API
MCP
ACP/client adapters where useful
```

### Invariant

All interfaces must converge on one authorization/context chokepoint.

No adapter can implement a weaker permission model.

### Strategic acceptance test

The same project memory can serve multiple agent/IDE ecosystems without exporting the entire vault to each vendor.

---

## 10. G-V2-12/13 — Memory Proposals and Memory CI

### Finding

The current architecture has strong provenance/authority concepts but no product workflow equivalent to reviewable durable-memory change.

### Proposed mechanism

```text
agent/human proposes durable memory change
-> diff
-> evidence/provenance
-> contradiction/staleness checks
-> authorization/review
-> activation
```

### Memory CI candidate checks

```text
provenance
citation
staleness
contradiction
secret/PII
scope
duplicate
superseded state
unauthorized promotion
invalid lifecycle
```

### Why it matters

This can become to Fehrest what PR checks are to GitHub: a durable trust workflow that competitors cannot replicate with a simple chatbot feature.

### Security rule

A green check does not grant authorization.

---

## 11. G-V2-14 — Import Lab

### Finding

Migration is not yet a first-class product phase, but replacement of Obsidian/Notion/Slack requires importing existing history.

### Priority

```text
P0 Markdown/Obsidian
P0 GitHub project/repository context
P1 Notion
P1 Slack
P1 Zulip
P1 AFFiNE
P2 AppFlowy
```

### Required fidelity dimensions

```text
content
attachments
relationships
source timestamps
identity mapping
provenance
unsupported construct reporting
repeatable import
```

### Recommendation

Move migration/import into the product proof plan before broad workspace launch.

---

## 12. G-V2-15 — Unified Search/Ask

### Finding

The existing Context Compiler/retrieval direction is a strong base, but the expanded workspace requires a human-facing query product in addition to agent context packages.

### Defining queries

```text
What is true now?
Why did we decide this?
What changed?
What did the previous agent try?
What failed repeatedly?
What is stale or contradicted?
Who knows about this?
What should I read before this meeting/task?
```

### Recommendation

Use one retrieval/authorization/provenance substrate for human Search/Ask and agent context generation, with different presentation layers.

---

## 13. G-V2-17 — Mobile

### Finding

Universal personal/team workspace adoption requires excellent mobile capture and retrieval.

### Minimum eventual expectations

```text
fast capture
offline notes
notifications
DM/channel participation
photo/file capture
voice note capture
search
basic task handling
reliable sync
```

### Risk

A desktop-only local-first app can succeed with developers but is unlikely to replace Slack/Notion/Obsidian for broad users.

### Recommendation

Make mobile a product acceptance dimension before broad GA, without pulling mobile UI into the current R1/post-R1 core gates prematurely.

---

## 14. G-V2-18 — Extension ecosystem

### Finding

GitHub and Obsidian both benefit from ecosystems. Fehrest eventually needs connectors, importers, views, templates, automations and agent adapters.

### Risk

A plugin system can bypass the exact trust model Fehrest is trying to create.

### Recommendation

Design capability-scoped extensions only after core authorization semantics are stable.

Potential extension categories:

```text
connector
importer
view
automation
template
skill
agent adapter
exporter
derived provider
```

Do not expose unrestricted filesystem/network/process access by default.

---

## 15. G-V2-19 — Fehrest Hub

### Finding

GitHub-scale importance requires network effects beyond one local installation.

### Potential long-term model

```text
private organization memory repositories
public knowledge repositories
research/playbook repositories
watch/follow
citation
proposal/review
fork/copy with provenance
agent-readable packages
```

### Core constraint

Hosted Hub state must not erase the local-first ownership promise or become the only canonical copy.

### Recommendation

Treat Hub as a later Class E product/network decision, not an excuse to introduce mandatory cloud services into the core.

---

## 16. G-V2-20 — Voice/video boundary

### Finding

Slack replacement may eventually require huddles/meetings, but media infrastructure is not a Fehrest differentiator.

### Recommendation

Prefer adapters/providers such as LiveKit/Jitsi after a product requirement exists.

```text
CUSTOM_MEDIA_STACK=REJECT_BY_DEFAULT
MEMORY_AROUND_MEETINGS=CORE_VALUE
```

Meeting transcripts, notes, decisions and memory proposals are strategically more important than owning the media transport.

---

## 17. G-V2-21 — Onboarding and product simplicity

### Finding

The architecture can be correct and still lose to Notion/Slack/Obsidian if the first-run experience exposes internal complexity.

### Required product principle

Progressive disclosure:

```text
new individual -> create note immediately
new team -> create/join workspace and topic immediately
new developer -> link GitHub repo immediately
new agent -> request scoped context through standard interface
```

Users should not need to understand canonical/derived stores, bitemporal semantics, CRDTs, receipts or hash chains to get value.

### Recommendation

Define UX budgets alongside performance/security budgets when UI work becomes authorized.

---

## 18. G-V2-23 — Benchmark expansion

### Finding

The current benchmark philosophy is strong, but the founder direction requires additional product outcome tests.

### Mandatory additions to consider after R1

```text
LongMemEval-V2
LongMemEval
LOCOMO where applicable
fresh-agent continuation
fresh-human continuation
static state recall
dynamic state tracking
workflow knowledge
premise awareness/stale assumption detection
multi-user collaboration convergence
import fidelity
search/Ask evidence quality
GitHub->Fehrest IDE discovery/context flow
```

### Critical rule

R1 remains historical evidence for its preregistered question only. These new tests do not retroactively change what R1 measured.

---

## 19. G-V2-24 — Donor reuse provenance

### Finding

The founder reports broad permission to copy/adapt the supplied source set. This can accelerate implementation materially.

### Remaining requirement

Permission must be made repository-verifiable at adoption time.

For each actual code reuse:

```text
source URL
commit SHA/release
exact source paths
permission/license evidence
copy/adaptation boundary
local modifications
security assessment
dependency/vendor decision
upgrade strategy
```

### Risk

Without this, Fehrest could become impossible to audit or maintain despite having permission.

---

## 20. G-V2-25 — R1 does not test the expanded product thesis

### Finding

This is expected and must be stated explicitly.

R1 was sealed before this founder direction and must remain interpreted according to its own preregistration.

### Consequence

Even a positive R1 route would only permit the next authorized work under existing rules. It would not automatically prove:

```text
Slack replacement
Notion replacement
Obsidian replacement
multi-user collaboration
GitHub companion adoption
universal individual/company fit
Fehrest Hub
```

### Recommendation

If R1 permits continuation, use its result as evidence for the bounded memory/context thesis, then separately review and stage the V2 founder direction.

---

## 21. G-V2-26 — Active repository provenance blocker

### Live repository fact at review time

GitHub `main` is an operational bootstrap/snapshot mirror. Historical R1 implementation/evidence objects are not all present as live GitHub bytes.

### Consequence

This remains an active gate before post-R1 implementation source can be reconciled and used.

### Recommendation

Do not allow product-vision enthusiasm to route around the existing provenance gate.

The V2 planning branch may record future direction, but it does not repair the missing historical source/evidence mirror.

---

## 22. Proposed strategic sequencing changes to evaluate

The current master plan should not be edited while R1 is open. After R1, the following changes deserve explicit review.

### Proposal A — Add a Product Vision Reconciliation gate after R1

Before broad post-R1 expansion:

```text
R1 terminal result
-> founder route decision
-> V2 product/architecture reconciliation
-> bounded canonical-core work
```

Do not allow V2 scope to invalidate necessary canonical-core hardening.

### Proposal B — Introduce GitHub Link/Discovery earlier than full agent gateway

A tiny non-authorizing repository-link/discovery mechanism may be valuable before the full Phase 5 gateway, provided it does not expose memory or widen authority.

Implementation timing must be decided by a dedicated spec.

### Proposal C — Reconsider graph-production-before-memory sequencing

The founder direction makes durable memory a product identity while graph remains a mechanism/capability hypothesis.

Evaluate:

```text
Phase 2 lexical/index convergence
-> Graph Intelligence capability experiment
-> graph retain/reject decision
-> temporal memory productization
-> graph production integration only when measured need justifies timing
```

This is a proposal, not an authorized reorder.

### Proposal D — Add Collaboration Capability Experiment before team productization

Do not jump from single-writer semantics directly to a production CRDT architecture.

### Proposal E — Move Import Lab before broad workspace launch

Migration is a prerequisite for replacing existing knowledge/team products.

### Proposal F — Add Human Workspace + Communication proof before full desktop GA

The full vertical proof should eventually test not only agent continuation but whether a small team can conduct real project work without returning to separate Slack/Notion/Obsidian tools for the tested workload.

---

## 23. Recommended roadmap layers

A useful distinction for future planning is:

```text
Layer 0  Canonical trustworthy core
Layer 1  Derived retrieval/context intelligence
Layer 2  Durable temporal memory
Layer 3  GitHub/open agent interoperability
Layer 4  Human personal workspace
Layer 5  Team communication/collaboration
Layer 6  Organization/security/admin
Layer 7  Ecosystem/import/connectors
Layer 8  Fehrest Hub/network
```

This is a dependency model to review, not an authorization sequence.

---

## 24. Highest-priority gaps after the existing canonical gates

If the bounded thesis survives R1 and canonical-core convergence, the most strategically important unresolved questions are:

1. Can Fehrest become the **Memory Repository** primitive without weakening open/local ownership?
2. Can any GitHub-connected IDE/agent discover and consume Fehrest through one safe open gateway?
3. Can temporal/provenance-aware memory materially outperform ordinary workspace search for real continuation?
4. Can multi-user local-first collaboration coexist with Fehrest's authority/recovery model?
5. Can one unified object model support notes, topics, tasks and decisions without creating a complicated Notion-style internal database UX?
6. Can Fehrest import years of existing Obsidian/Notion/Slack history with trustworthy provenance?
7. Can the product stay fast and simple enough for individuals while becoming governable enough for companies?

---

## 25. Review verdict

```text
FOUNDER_V2_DIRECTION=STRATEGICALLY_COHERENT
EXISTING_CORE_PRINCIPLES=STRONG_FOUNDATION
EXISTING_ROADMAP=INSUFFICIENT_FOR_FULL_V2_SCOPE
R1_REINTERPRETATION=REJECT
CURRENT_PRODUCT_IMPLEMENTATION_EXPANSION=REJECT
POST_R1_ARCHITECTURE_RECONCILIATION=REQUIRED
GITHUB_INTEGRATION=FLAGSHIP_FUTURE_REQUIREMENT
UNIVERSAL_NONDEVELOPER_SCOPE=RETAIN
TEAM_COLLABORATION=FUTURE_PRODUCT_REQUIREMENT
CRDT_SELECTION_NOW=REJECT
MEMORY_PROPOSALS_AND_MEMORY_CI=HIGH_VALUE_NEW_PRIMITIVES
IMPORT_MIGRATION=CORE_ADOPTION_REQUIREMENT
```

The next repository artifact is a non-authorizing proposal describing how these findings could alter the canonical Execution Master Plan **after** the current R1 gate is resolved through its existing protocol.