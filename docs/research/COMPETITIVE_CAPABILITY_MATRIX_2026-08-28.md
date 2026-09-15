# Fehrest Competitive Capability Matrix

**Date:** 2026-08-28  
**Status:** RESEARCH / NON-AUTHORIZING  
**Purpose:** compare the expanded founder direction against major product categories, donors and benchmarks.  
**Execution effect:** NONE while R1 remains active.

> A source in this matrix is not a dependency authorization. Disposition remains `USE / ADAPT / STUDY / BENCHMARK / DEFER / REJECT` and any copied/adapted code still requires exact provenance, rights/license evidence, security review and a scoped adoption decision.

---

## 1. Strategic benchmark statement

Fehrest is no longer evaluated against one category.

The target product must eventually be credible against the strongest relevant parts of:

```text
GitHub          repository/workflow/network
Obsidian        local personal knowledge
Notion          structured collaborative workspace
Slack           team communication
Zulip           durable topic-based communication
AFFiNE          local-first docs/canvas/database workspace
AppFlowy        local-first structured workspace
Anytype         sovereign object workspace
Buzz            human+agent event workspace
Mattermost      self-hosted team communication
Matrix          federated/open messaging patterns
Letta/Mem0      agent memory
Graphiti        temporal graph memory
```

Fehrest does not need to clone every implementation. It must provide one coherent memory substrate whose user-facing surfaces make separate products unnecessary for a growing set of users.

---

## 2. Capability scorecard

Legend:

```text
MUST       required for the long-term category
WEDGE      needed early to prove adoption
LATER      important, but may follow proof
BENCH      benchmark/donor target
NO-CORE    integrate rather than own
```

| Capability | Individual | Team/company | Agent | Fehrest priority | Primary references |
|---|---:|---:|---:|---|---|
| Local/open notes | High | Medium | Medium | MUST/WEDGE | Obsidian, AFFiNE, AppFlowy |
| Structured properties/bases | High | High | Medium | MUST | Notion, Obsidian Bases, AFFiNE, AppFlowy |
| Backlinks/graph navigation | High | Medium | Medium | MUST | Obsidian, Logseq-style patterns |
| Canvas/spatial workspace | High | Medium | Low | LATER | AFFiNE/BlockSuite, JSON Canvas, tldraw, Excalidraw |
| Channels/topics | Low | Critical | High | MUST | Slack, Zulip, Buzz, Mattermost |
| DMs/presence/notifications | Medium | Critical | Medium | MUST/LATER | Slack, Mattermost, Zulip, Buzz |
| Durable topic history | Medium | Critical | High | MUST | Zulip |
| Docs/wiki | High | Critical | High | MUST | Notion, Outline, AFFiNE |
| Tasks/projects | Medium | Critical | High | MUST | Notion, Linear-style workflows, GitHub Issues/Projects |
| Decisions/rationale | High | Critical | Critical | DEFINING | GitHub review discipline + Fehrest memory model |
| Provenance/citations | High | Critical | Critical | DEFINING | OpenLineage, in-toto, Fehrest canonical model |
| Temporal truth | High | High | Critical | DEFINING | Graphiti/Zep research, Fehrest memory model |
| Memory proposals/review | Medium | High | Critical | DEFINING | GitHub PR pattern adapted to memory |
| Memory CI | Medium | High | Critical | DEFINING | GitHub Actions/check pattern adapted to memory |
| Search/Ask with receipts | High | Critical | Critical | DEFINING | Notion Search, Buzz receipts, RAG/eval systems |
| Agent context compiler | Low | Medium | Critical | DEFINING | Fehrest architecture, Letta, Mem0, retrieval systems |
| Any-agent gateway | Low | Medium | Critical | DEFINING | MCP, ACP, CLI/SDK |
| GitHub repository binding | Medium | Critical for dev teams | Critical | FLAGSHIP | GitHub Apps/integrations |
| IDE discovery of Fehrest | Low | High for dev teams | Critical | FLAGSHIP | repo-local discovery + CLI/MCP/SDK |
| Offline-first | Critical | High | High | MUST | Obsidian, local-first research, Anytype, AFFiNE |
| Multi-device sync | High | Critical | Medium | MUST | Automerge/Yjs/Yrs/Y-Octo/Loro patterns |
| Real-time collaboration | Medium | Critical | Medium | MUST/LATER | Yjs ecosystem, Automerge, AFFiNE |
| E2EE/private deployment | High | Critical for some orgs | High | MUST/GATED | Anytype, Matrix, enterprise self-host patterns |
| Import/migration | Critical | Critical | Medium | MUST | Obsidian/Notion/Slack/Zulip/GitHub importers |
| Extension ecosystem | High | Critical | High | LATER | Obsidian plugins, GitHub Apps/Actions |
| Voice/video/huddles | Low | Medium/High | Low | NO-CORE | LiveKit, Jitsi |
| Git hosting | Low | Medium | Medium | NO-CORE | GitHub, Buzz patterns |
| Sandbox execution | Low | Medium | High | NO-CORE | OpenSandbox, E2B, Daytona |
| Graph database product | Low | Low | Medium | NO-CORE | Graphify/Graphiti/Code-Graph-RAG as derived candidates |
| Vector database product | Low | Low | Medium | NO-CORE | Qdrant/Chroma as replaceable providers |
| Mandatory hosted LLM | Low | Low | Low | REJECT | AI-OFF completeness |

---

## 3. Product-category comparisons

### 3.1 GitHub

**Category strength:** repository identity, collaboration workflow, issue/PR/review discipline, organizations, Apps/Actions ecosystem, durable project history.

**Disposition:** `STUDY + INTEGRATE + BENCHMARK UX`.

**Fehrest lessons:**

```text
repository as primary durable primitive
reviewable changes
history visible by default
stable links/identities
organization and project boundaries
checks around proposed changes
integration ecosystem
network effects around durable artifacts
```

**Do not copy:** Git hosting as an early product responsibility.

**Fehrest differentiator:** GitHub stores code/work artifacts exceptionally well but does not own canonical cross-session human+agent project memory, temporal beliefs, source-grounded decisions or context compilation.

**Flagship future integration:**

```text
GitHub repo <-> Fehrest Memory Repository
GitHub App
repo-local discovery record
Fehrest CLI/SDK/MCP gateway
IDE-independent context access
```

---

### 3.2 Obsidian

**Category strength:** local ownership, Markdown, speed, backlinks, plugins, personal knowledge workflows, graph/canvas ecosystem.

**Disposition:** `BENCHMARK + STUDY + ADAPT selectively`.

**Fehrest must match/exceed:**

```text
local files
fast capture/search/navigation
Markdown portability
backlinks/wikilinks
properties
daily notes
templates
attachments
canvas
keyboard-first workflows
extension ecosystem quality
```

**Fehrest differentiator:** durable team/agent memory semantics, provenance, temporal truth, authorized promotion, integrated communication and repository-grade history.

---

### 3.3 Notion

**Category strength:** onboarding, polished docs, structured databases/views, sharing, collaboration, project/task UX, templates, enterprise search and AI integration.

**Disposition:** `BENCHMARK UX + STUDY`.

**Fehrest must match/exceed:**

```text
simple workspace creation
beautiful docs
properties and database-like collections
table/board/calendar/timeline views
sharing and comments
team admin
templates
search
mobile quality
```

**Fehrest differentiator:** local-first canonical ownership, open formats, AI-OFF completeness, temporal memory, verifiable provenance and agent-safe context receipts.

---

### 3.4 Slack

**Category strength:** team communication, channels, DMs, notification behavior, integrations, workflows, presence, enterprise adoption.

**Disposition:** `BENCHMARK UX + STUDY`.

**Fehrest must match/exceed where team replacement is claimed:**

```text
channels
DMs
group DMs
mentions
reactions
presence
typing
notifications
files
search
guests
workflow activity
agent participation
```

**Fehrest differentiator:** conversation becomes durable evidence that can crystallize into reviewed memory, decisions, tasks and procedures instead of remaining an isolated timeline.

---

### 3.5 Zulip

**Category strength:** topic-first conversation that remains usable across live and asynchronous work.

**Disposition:** `ADAPT CONCEPT + BENCHMARK UX`.

**High-value concept:** topics should be durable namespaces that connect messages, decisions, docs, tasks, sources and agent activity.

**Risk to avoid:** copying Zulip's whole product architecture when only its conversation-organization semantics are needed.

---

### 3.6 Block Buzz

**Repository:** `block/buzz`

**Category strength:** humans and agents in the same workspace; channels, canvases, workflows, agent CLI, git events and signed event substrate.

**Disposition:** `STUDY + BENCHMARK + ADAPT selectively`.

**High-value concepts:**

```text
humans and agents as first-class workspace participants
agents with their own identity/audit trail
event-oriented workspace activity
question answering with receipts
branch/project activity co-located with discussion
agent-first CLI
standard protocol composition
knowledge crystallization concept
```

**Do not blindly adopt:**

```text
relay as Fehrest canonical memory architecture
Git hosting as mandatory Fehrest scope
custom media infrastructure
agent runtime ownership as a core requirement
```

**Fehrest differentiation:** Buzz explicitly positions the workspace/event relay as the shared pipe. Fehrest should own the deeper durable memory, provenance, temporal truth, review and context-compilation layer.

---

### 3.7 AFFiNE / BlockSuite / OctoBase / Y-Octo

**Category strength:** local-first unified docs/canvas/database workspace with collaboration-oriented infrastructure.

**Disposition:** `MANDATORY BENCHMARK + STUDY + ADAPT after gate`.

**High-value areas:**

```text
editor architecture
block/document model
canvas interaction
local-first UX
collaboration foundation
structured knowledge surfaces
Rust/native collaboration infrastructure patterns
```

**Important:** maintained AFFiNE/BlockSuite code should be reviewed rather than relying on stale standalone assumptions.

---

### 3.8 AppFlowy

**Category strength:** open/local-oriented structured workspace, Rust backend components, docs/databases/tasks, cross-platform product execution.

**Disposition:** `BENCHMARK + STUDY + ADAPT selectively`.

**Use in Fehrest evaluation:** onboarding, database/view UX, mobile/cross-platform lessons, import/export expectations.

---

### 3.9 Anytype

**Category strength:** sovereignty, local-first object system, encryption/privacy orientation, self-hosting/networking direction.

**Disposition:** `BENCHMARK + STUDY`.

**Use in Fehrest evaluation:** object UX, ownership promise, encrypted/local collaboration, user mental model for structured personal knowledge.

**Risk:** Fehrest must not expose object-model complexity merely because its internals are structured.

---

### 3.10 Mattermost / Rocket.Chat / Matrix

**Category strength:** self-hosted/enterprise communication, administrative control, open deployment; Matrix additionally supplies federation/E2EE protocol lessons.

**Disposition:**

```text
Mattermost    BENCHMARK + STUDY
Rocket.Chat   BENCHMARK + STUDY
Matrix        STUDY
```

**Use:** enterprise deployment expectations, audit/admin, air-gap/private-hosting requirements, interoperability lessons.

**Do not adopt:** protocol/federation complexity without measured need.

---

## 4. Agent memory comparisons

### 4.1 Letta / Letta Code / MemGPT

**Strength:** persistent agent context, memory filesystem/repository patterns, reflection/consolidation, skills and session continuity.

**Disposition:** `MANDATORY BENCHMARK + STUDY + ADAPT selectively`.

**Fehrest advantage to prove:**

```text
memory shared across humans and arbitrary agents
canonical authority independent of one agent runtime
review/provenance/temporal state
local/open user ownership
receipted context delivery
```

---

### 4.2 Mem0

**Strength:** memory extraction, retrieval, entity relationships and production-oriented agent memory interfaces.

**Disposition:** `MANDATORY BENCHMARK + STUDY`.

**Fehrest rule:** extracted memory remains evidence/candidate until authorized by the owning policy. Model extraction cannot mint truth.

---

### 4.3 Graphiti / Zep temporal knowledge graph

**Strength:** temporal relationships, episodic updates, historical/current querying.

**Disposition:** `MANDATORY BENCHMARK + STUDY`.

**Fehrest rule:** graph remains derived unless an independently authorized canonical object says otherwise. External graph identity/rank never becomes authority.

---

### 4.4 Chroma agentic memory

**Strength:** semantic/procedural/episodic memory research patterns and retrieval infrastructure.

**Disposition:** `STUDY + BENCHMARK`.

---

### 4.5 Hermes Agent

**Strength:** session continuity, memory consolidation, skills and practical agent workflows.

**Disposition:** `MANDATORY BENCHMARK + STUDY`.

---

## 5. Repository understanding / retrieval comparisons

### Mandatory repository-context set

```text
Aider repo-map
Graphify
Code-Graph-RAG
OpenGrok
Microsoft GraphRAG where comparable
Qdrant / Chroma optional derived retrieval
```

These systems answer measured retrieval/context questions. They are not canonical stores.

### Core evaluation question

> Does Fehrest deliver better downstream human/agent continuation at a fair budget than strong repository-native docs, ordinary files, mature search and specialized context systems?

---

## 6. Collaboration substrate candidates

The expanded product direction creates a future multi-user/local-first architecture question.

**No winner is selected.**

Candidate benchmark set:

| Candidate | Why it matters | Initial disposition |
|---|---|---|
| Automerge | mature local-first/CRDT model, Rust core | BENCHMARK |
| Yjs/Yrs | broad editor ecosystem, Rust compatibility via Yrs | BENCHMARK |
| Y-Octo | Rust/Yjs-compatible path associated with AFFiNE ecosystem | BENCHMARK |
| Loro | Rust-capable CRDT with rich structured data support | BENCHMARK |
| AFFiNE/OctoBase patterns | real workspace collaboration architecture | STUDY/BENCHMARK |
| Keyhive | capability/authorization-oriented local-first research | STUDY; security maturity gate required |

Required future measurements:

```text
offline convergence
concurrent edit correctness
large documents
large workspaces
sync bandwidth
memory/CPU
startup
mobile behavior
crash recovery
schema evolution
history/audit compatibility
permission interaction
Rust integration
open-format recovery
```

---

## 7. Media / huddle providers

Fehrest's differentiated value is the memory around communication, not media transport.

Candidates:

```text
LiveKit   STUDY/BENCHMARK
Jitsi     STUDY/BENCHMARK
```

Default direction:

```text
BUILD_CUSTOM_SFU=REJECT_BY_DEFAULT
ADAPTER_PROVIDER=EXPECTED_IF_REQUIRED
```

---

## 8. Import/migration competitors and source systems

A future Import Lab should treat migration quality as a first-class benchmark.

Priority source set:

```text
P0: Markdown folders / Obsidian
P0: GitHub repository/project context
P1: Notion
P1: Slack
P1: Zulip
P1: AFFiNE
P2: AppFlowy
P2: structured JSON/CSV
P2+: selected cloud/document systems
```

Import success dimensions:

```text
content fidelity
relationship fidelity
timestamp fidelity
attachment fidelity
source provenance
unsupported-construct reporting
repeatability/idempotency
large-workspace performance
```

---

## 9. Open standards and infrastructure references

### Canonical/open format

```text
CommonMark
ordinary filesystem semantics
Git/gix/jj as history/provenance references where appropriate
```

### Provenance

```text
OpenLineage
in-toto attestations
```

### Authorization

```text
Cedar / Cedar for Agents
OpenFGA
SpiceDB
cap-std
```

### Agent interoperability

```text
MCP official ecosystem
ACP where applicable
CLI/JSON tooling
standard HTTP/local APIs
```

### Evaluation/observability

```text
Braintrust
Promptfoo
DeepEval
RAGAS
Phoenix
Langfuse
OpenTelemetry
Harbor ATIF
```

Fehrest should own a local open experiment/trajectory record first and export optionally.

---

## 10. Memory benchmark set

The mandatory future memory evaluation set should include at least:

```text
LongMemEval
LongMemEval-V2
LOCOMO where applicable
continuation benchmarks
premise-awareness / stale-state tests
workflow-knowledge recall
static and dynamic state tracking
abstention
contradiction handling
temporal reasoning
```

The product-defining benchmark remains downstream outcome, not synthetic memory recall alone.

---

## 11. GitHub integration capability matrix

| Capability | Desired behavior | Authority rule |
|---|---|---|
| Repository binding | Link GitHub repo to Fehrest Memory Repository | Link is identity/discovery, not grant |
| GitHub App | Permission-scoped ingestion/events | GitHub data = evidence |
| Issues/PRs/discussions | Search/link/context | No automatic canonical promotion |
| Actions/checks | Event/evidence ingestion | CI result does not mint user authority |
| Repo-local link manifest | IDE/agent discovery | Contains no secrets/capabilities |
| CLI | Request scoped context | Authorization checked by Fehrest |
| MCP | Agent interoperability | Grant is server-owned and bounded |
| SDK/API | IDE/platform integration | Same authorization chokepoint |
| Context receipt | Bind what an agent received | Derived evidence; auditable |
| Memory proposal | Agent proposes durable change | Review/policy required |
| Memory CI | Validate proposal | Checks cannot self-authorize |

### Primary user story

```text
Developer or agent opens GitHub repository in any IDE
-> repository exposes Fehrest link/discovery metadata
-> client connects to local/authorized Fehrest gateway
-> identity/grant is resolved
-> task request is submitted
-> Fehrest compiles scoped context
-> client receives context + receipt
-> work proceeds
-> trajectory/events may return as evidence
-> agent may submit a Memory Proposal
-> proposal is reviewed before canonical activation
```

This flow must not require a Fehrest-specific IDE.

---

## 12. Feature accumulation guardrail

This matrix must not be read as a backlog of features to implement immediately.

For every capability:

```text
REQUIREMENT
-> USER/AGENT OUTCOME
-> EXISTING FEHREST PRIMITIVE
-> DONOR/BASELINE
-> PONYTAIL NECESSITY
-> RIGHTS/PROVENANCE
-> SECURITY
-> BENCHMARK
-> AUTHORIZATION
-> IMPLEMENT OR REJECT
```

The correct product can contain fewer internal systems while replacing more external applications.

---

## 13. Mandatory review set v2

The post-R1 product/architecture review should not proceed without explicitly considering at least:

### Human workspace

```text
Obsidian
Notion
AFFiNE / BlockSuite / OctoBase / Y-Octo
AppFlowy
Anytype
Zulip
Slack
Mattermost
Buzz
```

### Agent memory/context

```text
Letta / Letta Code / MemGPT
Mem0
Graphiti
Chroma
Hermes Agent
```

### Repository/context intelligence

```text
GitHub
Aider
Graphify
Code-Graph-RAG
OpenGrok
Qdrant
```

### Local-first collaboration

```text
Automerge
Yjs/Yrs
Y-Octo
Loro
Keyhive (study maturity)
```

### Agent/runtime interoperability

```text
MCP
ACP
LangGraph
LangChain
DeepSeek Harness
mini-SWE-agent
OpenHands
```

### Safe execution/evaluation

```text
OpenSandbox
E2B
Daytona
Harbor ATIF
Braintrust
Promptfoo
DeepEval
RAGAS
Phoenix
Langfuse
OpenTelemetry
```

### Standards/security

```text
CommonMark
OpenLineage
in-toto
Cedar
OpenFGA
SpiceDB
cap-std
```

---

## 14. Rights/provenance note

The founder reports permission to copy/adapt the supplied source set.

Before any actual reuse, the repository must preserve exact evidence per adopted source:

```text
repository URL
commit/release pin
source paths
license and/or permission record
reuse scope
local modifications
security review
upgrade/update strategy
```

This matrix records product research only. It does not itself authorize copying any particular code slice.