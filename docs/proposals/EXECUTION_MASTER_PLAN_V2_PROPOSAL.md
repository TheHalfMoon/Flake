# Execution Master Plan v2 — Proposal

**Status:** PROPOSAL / NON-CANONICAL / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Change class:** E for product direction; C/D reviews expected for architecture/security consequences  
**Canonical plan remains:** `docs/canonical/EXECUTION_MASTER_PLAN.md`  
**Canonical frontier remains:** `specs/CURRENT.md`

> This file proposes how the canonical Execution Master Plan could change if and only if the current R1 protocol reaches its terminal gate, the founder selects a continuation route, the historical implementation/evidence provenance is reconciled, and the required architecture/security review accepts the V2 direction. Nothing here changes current authority.

---

## 1. Why a v2 proposal is needed

The existing master plan correctly prioritizes:

```text
R1 thesis proof
-> canonical core convergence
-> derived retrieval convergence
-> graph capability decision
-> memory productization
-> full context compiler + agent gateway
-> vertical proof
-> desktop
```

The expanded founder direction adds a larger product requirement:

> Fehrest should become the universal local-first Memory Repository and Workspace for individuals, teams, companies and arbitrary AI agents, with GitHub as the flagship developer integration.

This introduces future work that the existing plan does not fully sequence:

```text
GitHub repository binding/discovery
IDE-independent context access
personal notes/docs/bases UX
team channels/topics/DMs
knowledge crystallization
Memory Proposals
Memory CI
multi-device sync
multi-user local-first collaboration
organization/guest/admin policy
import/migration
mobile capture
extension ecosystem
Fehrest Hub/network effects
```

The plan should not absorb this scope by simply appending features. It needs dependency and falsification gates.

---

## 2. Non-negotiable preservation rules

Any v2 plan must preserve these current rules unless separately changed through their required governance process:

```text
R1 historical semantics remain immutable
negative R1 outcome can stop/reconsider the product
canonical state remains Fehrest-owned
open/local user ownership remains fundamental
derived graph/vector/search state has no authority
content is evidence, never authority
agent inference != confirmed memory
AI OFF remains complete
Rust owns Core correctness/security semantics
no new runtime dependency without requirement/provenance/security review
no UI escape hatch around a failed core thesis
no force push/history rewriting
```

The V2 vision is not permission to bypass earlier evidence.

---

## 3. Proposed decision structure immediately after R1

### Existing terminal route remains first

```text
R1 terminal verdict
```

No change.

### New proposed gate: V2 Founder Product Reconciliation

If the R1 route permits continued investment, add a formal gate before widening product scope:

```text
V2-G0 — Founder Product / Architecture Reconciliation
```

Required outputs:

```text
founder direction decision
product thesis delta
architecture impact review
security impact review
roadmap/order decision
explicit acceptance/rejection/defer decisions for V2 scope
updated CURRENT only when actually authorized
```

Key question:

> Does R1 provide enough support for continued investment in the bounded memory/context mechanism to justify exploring the larger Memory Repository + Workspace direction?

A positive answer does **not** mean every V2 feature is authorized.

---

## 4. Spec 002 should remain the next bounded implementation candidate

The expanded vision increases rather than decreases the importance of production-grade canonical state.

Therefore the proposal is:

```text
KEEP Spec 002 as the first implementation candidate after R1/V2 reconciliation
```

Subject to the existing entry gate:

```text
R1 terminal verdict recorded
route permits continuation
founder authorization
live implementation/evidence source reconciled
historical R1 evidence verified
```

Reason:

Every later V2 capability depends on canonical state being trustworthy under crash/recovery/writer semantics.

Do not pull chat, UI, GitHub integration, CRDT, MCP or automatic memory into Spec 002.

---

## 5. Proposed dependency-ordered v2 route

The following is a planning proposal, not a task list.

```text
R1 terminal verdict
-> V2-G0 founder/product/architecture reconciliation
-> Phase 1 canonical core convergence (Spec 002)
-> Phase 2 derived lexical/index convergence
-> Phase 3A Graph Intelligence capability experiment
-> Phase 3 decision: RETAIN / DEFER / REJECT production graph
-> Phase 4 temporal memory productization
-> Phase 5 full Context Compiler + Universal Memory Gateway
-> Phase 5G GitHub Link + Repository Discovery integration
-> Phase 6 trusted vertical memory proof
-> Phase 7 personal knowledge workspace + Import Lab
-> Phase 8 collaboration capability experiment
-> Phase 9 team communication/workspace
-> Phase 10 organization/security/admin + mobile readiness
-> Phase 11 extension/connectors ecosystem
-> Phase 12 Fehrest Hub/network, only if local-first product proof succeeds
```

The important changes are explained below.

---

## 6. Proposed graph sequencing change

### Current concern

The existing plan permits production graph integration immediately after a successful graph capability experiment and before temporal memory productization.

The expanded founder direction makes this distinction strategically important:

```text
Memory = product identity
Graph  = derived mechanism/capability hypothesis
```

### Proposed change

Keep the **Graph Intelligence capability experiment** early enough to inform architecture, but decouple a positive experiment from automatic immediate production integration.

Proposed route:

```text
Phase 3A GI capability experiment
-> decision
   REJECT: no production graph
   DEFER: evidence positive but not required yet
   RETAIN_NOW: production integration has measured dependency
```

Then allow temporal memory productization to proceed before graph production when graph is not a blocking dependency.

### Why

This prevents a derived infrastructure layer from delaying the product's defining durable-memory capability.

### Governance

This is a phase-order change and therefore requires the architecture change process before becoming canonical.

---

## 7. Phase 4 proposal — Temporal Memory Productization expands slightly in product workflow, not authority

Keep the existing durable memory goals:

```text
canonical memory journal/store
explicit memory-write surface
versioning/upcasting
provenance
PENDING/ACTIVE/RETRACTED/SUPERSEDED transitions
current/as-of queries
recovery
```

Add design/specification for two future-facing primitives where justified:

### Memory Proposal

A reviewable proposed change to durable memory.

### Memory CI contract

Checks over provenance, contradiction, staleness, secret leakage, lifecycle and scope.

Important:

```text
Memory CI result != authorization
agent proposal != canonical activation
```

Do not turn the memory phase into a Slack/Notion implementation phase.

---

## 8. Phase 5 proposal — Universal Memory Gateway

Rename/expand the conceptual goal from a narrowly agent-facing gateway to a **Universal Memory Gateway** while preserving the deterministic Context Compiler and authorization model.

Consumers include:

```text
coding agents
IDE assistants
research agents
enterprise agents
Fehrest desktop/mobile
CLI scripts
external tools
```

Interfaces to evaluate:

```text
CLI
SDK
local API / HTTP where appropriate
MCP
ACP/client adapters where useful
```

All interfaces must converge on the same authorization and receipt semantics.

### Core request model

```text
principal/session
+ repository/workspace/project binding
+ task/request
+ grant
+ canonical high-water mark
+ retrieval policy
+ budget
-> context package + receipt
```

### Required guarantee

A client adapter cannot grant itself more access than the underlying Fehrest grant.

---

## 9. New Phase 5G proposal — GitHub Link + Repository Discovery

GitHub integration should be flagship but remain built on the Phase 5 authorization/context substrate.

### Goal

Any IDE or agent that opens a GitHub repository should be able to discover that repository's Fehrest memory connection and request authorized context without requiring a Fehrest-specific IDE.

### Candidate components

```text
GitHub App
repo-local non-secret link/discovery manifest
Fehrest repository binding model
CLI/SDK/MCP discovery
issue/PR/discussion/Actions evidence connectors
context receipt linked to repository/task identity
```

### Candidate discovery artifact

Subject to dedicated spec/security review:

```text
.fehrest/link.toml
```

It may identify the target Fehrest workspace/project/gateway, but:

```text
CONTAINS_SECRET=NO
GRANTS_PERMISSION=NO
OVERRIDES_SERVER_AUTHORIZATION=NO
```

### Minimum acceptance story

```text
1. Open GitHub repo in arbitrary IDE.
2. Client discovers Fehrest binding.
3. User/agent authenticates to Fehrest.
4. Client asks for task context.
5. Fehrest applies canonical scope and budget.
6. Context + receipt returned.
7. Work can return evidence/trajectory.
8. Durable memory change requires proposal/review.
```

### Non-goal

Fehrest does not need to replace GitHub hosting or build its own IDE.

---

## 10. Phase 6 proposal — Trusted Vertical Memory Proof

Expand the full vertical proof so the system must prove the V2 substrate before broad workspace UI.

### Agent continuation remains mandatory

Keep the defining fresh-agent continuation benchmark.

### Add fresh-human continuation

A new human joining a project should be able to understand and continue a realistic task materially faster/more correctly using Fehrest memory than using strong simpler baselines.

### Add GitHub-linked workflow

A representative GitHub project should test:

```text
repo discovery
issue/task context request
bounded context generation
work trajectory/evidence return
memory proposal
later continuation
```

### Add trust tests

```text
stale memory
contradictory memory
untrusted agent candidate
secret-containing evidence
revoked scope
historical/as-of query
```

### Add current memory research benchmarks

Where applicable:

```text
LongMemEval-V2
LongMemEval
LOCOMO
workflow knowledge
premise awareness
static/dynamic state tracking
abstention
```

Synthetic benchmarks remain secondary to downstream continuation outcomes.

---

## 11. Phase 7 proposal — Personal Knowledge Workspace + Import Lab

Only after the trusted vertical memory proof supports continued product investment should Fehrest attempt to replace personal knowledge tools broadly.

### Goal

Deliver an Obsidian-class local personal knowledge experience backed by the proven Fehrest memory substrate.

### Core surfaces

```text
Notes
Docs
Sources
Search/Ask
Properties
Backlinks
Collections/Bases
Table/board/calendar views
Canvas
Tasks
Decisions
Memory viewer/history
```

### Import Lab becomes an entry requirement

At least:

```text
Markdown/Obsidian import
GitHub project-memory import/link
```

before claiming serious migration readiness.

Then add benchmarked importers for Notion, Slack, Zulip, AFFiNE and others based on adoption need.

### Editor gate

Preserve a benchmark/prototype decision. AFFiNE/BlockSuite and CodeMirror remain important references; no editor is selected by this proposal.

### Product test

A strong individual user should be able to choose Fehrest without needing Obsidian/Notion for the tested personal-knowledge workflow.

---

## 12. New Phase 8 proposal — Collaboration Capability Experiment

Do not jump directly from local/single-writer history into a production team CRDT.

### Objective

Prove a collaboration mechanism compatible with Fehrest's canonical ownership, provenance, recovery and authorization requirements.

### Candidate baseline family

```text
Automerge
Yjs/Yrs
Y-Octo
Loro
AFFiNE/OctoBase architecture patterns
```

Additional systems may be added through evidence refresh.

### Measure

```text
offline convergence
multi-writer behavior
large docs/workspaces
sync bandwidth
CPU/memory
mobile
crash/recovery
history/provenance
permission/revocation semantics
schema migration
open export/recovery
```

### Security adversarial cases

```text
revoked offline writer reconnects
malicious/stale actor id
partial sync crash
permission downgrade during partition
cross-space data leak
conflict containing secret/private content
replay/duplicate mutation
```

### Decision

```text
COLLAB_MECHANISM_RETAIN
COLLAB_MECHANISM_DEFER
COLLAB_MECHANISM_REJECT
```

No production collaboration begins until this gate closes honestly.

---

## 13. New Phase 9 proposal — Team Communication and Workspace

Entry:

```text
trusted memory proof passes
personal workspace is viable
collaboration capability gate retains a mechanism
security review permits team model
```

### Goal

Make Fehrest sufficient as the primary project/team knowledge and communication workspace for a defined workload.

### Core team surface

```text
workspaces/spaces
channels
topics
threads/replies
DMs/group DMs
mentions
reactions
presence/typing
notifications
docs
shared notes
tasks
decisions
files
agent activity
search/Ask
```

### Topic model

Prefer durable topic organization so conversation history remains navigable and can bind directly to decisions/tasks/docs.

### Knowledge Crystallization

Conversation/meeting/agent evidence can generate candidates, but canonical promotion remains reviewed/policy-controlled.

### Product benchmark

A small team completes a preregistered project workflow using Fehrest without needing separate Slack + Notion + Obsidian for the measured scenario.

Competitors should be used as strong baselines, not caricatures.

---

## 14. Phase 10 proposal — Organization Security/Admin + Mobile Readiness

### Organization features to gate

```text
organization/workspace administration
roles/policies
guests/external collaborators
device/session management
backup/export
retention policy
private/on-prem deployment mode where justified
security/audit controls
```

### Encryption/product-mode decision

Explicitly decide trade-offs among:

```text
strict E2EE
server-side search/automation
enterprise compliance/eDiscovery
agent access
```

Do not promise incompatible properties under one vague "encrypted" label.

### Mobile readiness

Before broad workspace GA, require reliable:

```text
capture
offline notes
notifications
channels/DMs
search
tasks
file/photo/voice capture
sync
```

Mobile implementation technology is not selected here.

---

## 15. Phase 11 proposal — Extension and Connector Ecosystem

Entry only after authorization semantics are stable.

Potential extension types:

```text
Connector
Importer
View
Automation
Template
Skill
Agent Adapter
Exporter
Derived Provider
```

### Required capability security

An extension should receive explicit grants such as:

```text
notes:read
notes:write
space:research:read
network:allowed-hosts
process:none
secrets:none
```

An extension ecosystem must not recreate unrestricted plugin authority by default.

---

## 16. Phase 12 proposal — Fehrest Hub

This is the network-effect phase and should happen only if the local/product thesis has already earned it.

Potential product:

```text
public/private Memory Repositories
organization hosting
public knowledge collections
research/playbooks/standards
citation
watch/follow
proposal/review
fork/copy with provenance
agent-readable repository packages
```

### Hard invariant

```text
HOSTED_HUB != ONLY_CANONICAL_COPY
```

Local/open export and ownership remain product promises.

---

## 17. Proposed future Spec Kit map

Do not create or activate these specs solely from this proposal. Names are placeholders for planning.

```text
002 post-r1-canonical-core-convergence       KEEP existing
003 phase2-derived-index-convergence         KEEP concept
004 gi-cap                                   KEEP capability experiment
005 graph-production-integration             MAKE CONDITIONAL/DEFERABLE
006 phase4-memory-productization              EXPAND proposal/review semantics carefully
007 universal-context-memory-gateway          EVOLVE current Phase 5 concept
008 github-link-repository-discovery          NEW candidate
009 trusted-vertical-memory-proof             EVOLVE current Phase 6
010 personal-knowledge-workspace-import       NEW candidate
011 collaboration-capability-experiment       NEW candidate
012 team-communication-workspace              NEW candidate
013 organization-security-mobile-readiness    NEW candidate
014 extension-connector-ecosystem             NEW candidate
015 fehrest-hub                               NEW candidate
```

Exact numbering/order should be chosen only during the authorized master-plan reconciliation.

---

## 18. Donor routing under the proposed plan

### Phase 2 / retrieval

```text
Aider
OpenGrok
Qdrant/Chroma benchmark only
```

### Phase 3 graph capability

```text
Graphify
Code-Graph-RAG
Graphiti where semantics match
Microsoft GraphRAG where comparable
```

### Phase 4 memory

```text
Letta/Letta Code
Mem0
Graphiti
Chroma memory patterns
Hermes Agent
LongMemEval / LongMemEval-V2 / LOCOMO
```

### Phase 5 gateway

```text
MCP
ACP where useful
Hermes/Letta/OpenHands/mini-SWE integration patterns
DeepSeek Harness principles
```

### Phase 5G GitHub

```text
GitHub Apps/API/webhooks
GitHub repository/project semantics
GitHub review/check UX patterns
```

### Phase 7 personal workspace

```text
Obsidian
Notion UX baseline
AFFiNE/BlockSuite
AppFlowy
Anytype
CodeMirror
JSON Canvas/tldraw/Excalidraw as relevant
```

### Phase 8 collaboration

```text
Automerge
Yjs/Yrs
Y-Octo
Loro
AFFiNE/OctoBase patterns
Keyhive study
```

### Phase 9 team workspace

```text
Slack
Zulip
Buzz
Mattermost
Rocket.Chat
```

### Media adapters

```text
LiveKit
Jitsi
```

### Evaluation/export

```text
Harbor ATIF
Braintrust
Promptfoo
DeepEval
RAGAS
Phoenix
Langfuse
OpenTelemetry
```

---

## 19. Rights/provenance gate for donor-code reuse

The founder reports permission to copy/adapt the supplied source set.

The proposed master plan should add an explicit reusable donor-adoption record before any source is copied.

Suggested template:

```text
DONOR_ID=
SOURCE_REPOSITORY=
SOURCE_COMMIT=
SOURCE_PATHS=
SOURCE_LICENSE=
PERMISSION_EVIDENCE=
DISPOSITION=USE|ADAPT
REQUIREMENT=
WHY_REUSE_BEATS_REIMPLEMENTATION=
SECURITY_REVIEW=
LOCAL_CHANGE_BOUNDARY=
UPDATE_POLICY=
EXIT_STRATEGY=
```

No copied code should enter Fehrest as unattributed or unpinned historical material.

---

## 20. Proposed new benchmark families

### B-V2-1 — GitHub-linked continuation

Can a fresh human/agent open a GitHub repository, discover Fehrest, obtain the correct scoped memory and complete a real task better than strong simpler baselines?

### B-V2-2 — Knowledge crystallization accuracy

Can Fehrest convert conversation/activity into useful candidates without promoting unsupported or contradicted claims?

### B-V2-3 — Memory Proposal safety

Do agents propose useful durable changes while unauthorized/stale/secret-bearing changes remain blocked?

### B-V2-4 — Human continuation

Can a new teammate recover project state, rationale and pitfalls materially faster/more correctly?

### B-V2-5 — Import fidelity

Can real Obsidian/Notion/Slack/Zulip histories migrate without silent loss or provenance collapse?

### B-V2-6 — Local-first collaboration

Do concurrent/offline edits converge under required correctness/security conditions?

### B-V2-7 — Workspace replacement

For a preregistered individual/team workload, can participants use Fehrest without returning to the compared separate tools?

### B-V2-8 — Memory trust

Can users/agents distinguish current, stale, contradicted, unconfirmed and superseded knowledge correctly?

---

## 21. Proposed product north-star metrics

Future product metrics should connect directly to the memory thesis:

```text
fresh-agent continuation success
fresh-human continuation success
time to recover project context
context tokens/cost per successful task
important decisions with recoverable rationale
stale/contradicted memory detection
memory proposal acceptance/rejection quality
GitHub-linked repo activation
percentage of agent sessions using Fehrest context
import completion/fidelity
workspace replacement rate for tested workflows
offline reliability
collaboration convergence failures
export/recovery success
```

Do not optimize daily messages or stored notes as primary success metrics if they do not improve durable continuity.

---

## 22. What should remain out of core

Even under V2:

```text
Git hosting                  integrate GitHub; do not own by default
agent runtime                interoperate; do not own by default
sandbox platform             provider/adaptor
vector database              replaceable derived provider
graph database               replaceable derived provider
video SFU                    provider/adaptor
mandatory LLM                reject
mandatory cloud              reject
unrestricted plugin runtime  reject by default
```

The plan should become broader in user value while remaining narrow in unique infrastructure ownership.

---

## 23. Canonical edit proposal if/when authorized

If the founder and required reviews accept V2 after R1, the canonical `EXECUTION_MASTER_PLAN.md` should be edited in one reviewed change to:

1. Record the accepted V2 north star and its relationship to the original bounded thesis.
2. Add the V2 product/architecture reconciliation gate.
3. Preserve Spec 002 as the first bounded implementation slice unless evidence changes that decision.
4. Make graph production explicitly deferable after the capability experiment.
5. Strengthen memory productization around reviewed durable memory changes.
6. Evolve Phase 5 into a universal context/memory gateway.
7. Add the GitHub Link/Repository Discovery gate.
8. Expand vertical proof to fresh-human + fresh-agent + GitHub-linked continuation.
9. Add Personal Workspace + Import Lab after proof.
10. Add a collaboration capability experiment before team implementation.
11. Add Team Communication/Workspace, Organization/Admin/Mobile, Ecosystem and Hub as separately gated later phases.
12. Add current memory/collaboration/import benchmarks.
13. Add a standard donor-code provenance/adoption record.
14. Update planned Spec Kit sequence without activating any unearned phase.

Until that authorization exists:

```text
CANONICAL_MASTER_PLAN_CHANGE=NO
CURRENT_CHANGE=NO
R1_CHANGE=NO
SPEC_002_ACTIVATION=NO
PRODUCT_IMPLEMENTATION=NO
```

---

## 24. Proposal verdict

```text
KEEP_R1_ORDER_AND_SEMANTICS=YES
KEEP_SPEC_002_AS_NEXT_BOUNDED_CANDIDATE=YES
ADD_POST_R1_V2_RECONCILIATION_GATE=PROPOSED
MAKE_GRAPH_PRODUCTION_DEFERABLE=PROPOSED
MOVE_MEMORY_PRODUCT_IDENTITY_EARLIER_RELATIVE_TO_OPTIONAL_GRAPH_PRODUCTION=PROPOSED
EVOLVE_AGENT_GATEWAY_TO_UNIVERSAL_MEMORY_GATEWAY=PROPOSED
ADD_GITHUB_LINK_AND_IDE_DISCOVERY=PROPOSED
ADD_FRESH_HUMAN_CONTINUATION_PROOF=PROPOSED
ADD_PERSONAL_WORKSPACE_AND_IMPORT_LAB=PROPOSED
ADD_COLLABORATION_CAPABILITY_GATE=PROPOSED
ADD_TEAM_COMMUNICATION_WORKSPACE=PROPOSED
ADD_ORG_ADMIN_MOBILE_PHASE=PROPOSED
ADD_EXTENSION_ECOSYSTEM=PROPOSED
ADD_FEHREST_HUB_LAST=PROPOSED

AUTHORIZED_BY_THIS_DOCUMENT=NO
```

This proposal should remain separate from the canonical master plan until the live R1 gate and the required founder/architecture/security decisions make a canonical change legitimate.