# Fehrest V2 Spec Sequence and Dependencies

**Status:** PROGRAM PROPOSAL / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Canonical frontier remains:** `specs/CURRENT.md`  
**Canonical master plan remains:** `docs/canonical/EXECUTION_MASTER_PLAN.md`

> This document proposes a conflict-minimized future Spec Kit decomposition for the V2 product direction. It does not activate any listed spec. Exact IDs/names may change only during an authorized master-plan reconciliation.

---

## 1. Sequencing principles

The sequence follows these rules:

```text
canonical correctness before derived intelligence
derived retrieval before search-dependent product surfaces
memory semantics before agent memory gateway
identity/grants before GitHub/IDE memory access
headless proof before broad UI
open object formats before polished workspace surfaces
AI provider runtime after deterministic search/context substrate
web tools after model/tool authorization boundaries
collaboration experiment before collaboration production
organization authorization before team communication
team product before ecosystem/network expansion
```

The program is not fully linear internally: some candidate specs may be `DEFERRED` or `REJECTED` by experiments. However, because Fehrest governance allows only one active product frontier, `specs/CURRENT.md` always points to exactly one active spec/gate.

---

## 2. Program gates before new implementation

### G-R1 — R1 terminal gate

Owned by the current benchmark protocol.

No V2 product implementation bypasses this gate.

### G-PROV — Historical implementation/evidence provenance reconciliation

Required before post-R1 implementation from the GitHub mirror.

### G-CONST — Constitution / Architecture reconciliation

Required before architecture-semantic V2 activation. Missing historical architecture/governance sources may not be reconstructed from memory.

### G-V2 — Founder V2 product decision

Required outcome:

```text
V2_GO
or
V2_GO_WITH_CONSTRAINTS
```

before widening the product roadmap beyond the already-prepared post-R1 route.

---

# 3. Proposed Spec Kit sequence

## 002 — Post-R1 Canonical Core Convergence

**Existing Spec Kit:** `specs/002-post-r1-canonical-core-convergence/`  
**Disposition:** KEEP

### Owns

```text
vault identity/version
crash-safe canonical replacement
writer-owned mutation boundary
versioned typed event journal
startup integrity/recovery
schema upcasting foundation
```

### Does not own

```text
search UX
memory product lifecycle
graph
AI
GitHub integration
sync
UI
```

### Entry

Existing Spec 002 activation gate remains authoritative.

### Exit

Zero silent canonical loss under required fault/recovery evidence and all Spec 002 closeout criteria.

---

## 003 — Derived Index and Lexical Retrieval Convergence

**Proposed name:** `003-derived-index-lexical-retrieval-convergence`

### Dominant outcome

A local repository can rebuild and incrementally maintain fast deterministic lexical/structured retrieval without making derived state authoritative.

### Owns

```text
derivation registry
lexical/FTS index generation
content-hash invalidation
incremental update
watch/reconcile behavior
fresh rebuild
incremental-vs-clean equivalence
search candidate trace baseline
```

### Does not own

```text
search desktop UX
semantic/vector default
graph intelligence
LLM answers
```

### Entry

```text
002 PASS
canonical format stable enough for derived projections
```

### Kill/failure routing

If incremental derived state cannot remain equivalent/rebuildable, stop and repair the derived model before any user-facing Search spec.

---

## 004 — Graph Intelligence Capability Experiment

**Proposed name:** `004-graph-intelligence-capability-experiment`

### Dominant outcome

Determine whether derived graph intelligence materially improves retrieval/continuation outcomes at acceptable cost.

### Owns

```text
preregistered graph comparator experiment
graph build/retrieval benchmark
cost/quality decision evidence
```

### Does not own

```text
Obsidian-style graph visualization
canonical explicit note links
production graph provider
```

### Entry

```text
003 PASS
benchmark tasks frozen
```

### Exit decisions

```text
GRAPH_INTELLIGENCE_RETAIN_NOW
GRAPH_INTELLIGENCE_DEFER
GRAPH_INTELLIGENCE_REJECT
```

This distinction prevents a successful capability experiment from automatically forcing production graph infrastructure into the critical path.

---

## 005 — Graph Production Integration

**Proposed name:** `005-graph-production-integration`  
**Status by default:** CONDITIONAL

### Entry

Only:

```text
004 = GRAPH_INTELLIGENCE_RETAIN_NOW
AND
measured downstream requirement requires production graph before memory/gateway work
```

### Owns

```text
replaceable derived GraphProvider boundary
production graph generation binding
rebuild/reconciliation
graph-specific selection trace
```

### Critical rule

If 004 returns `DEFER` or `REJECT`, 005 is not activated. It is recorded as `DEFERRED` or `REJECTED`, not falsely marked PASS.

---

## 006 — Temporal Memory Productization

**Proposed name:** `006-temporal-memory-productization`

### Dominant outcome

Humans and authorized agents can create, review, query, supersede and retract durable memory with provenance and temporal truth.

### Owns

```text
Memory canonical schema
memory lifecycle
current/as-of resolution
Memory Proposal semantic contract
Memory CI semantic contract
confirmation/review transitions
supersession/retraction
contradiction linkage
memory recovery/migration
```

### Candidate lifecycle

Exact states require the spec, but must cover the product concepts behind:

```text
OBSERVED / CANDIDATE / PENDING_REVIEW / ACTIVE / SUPERSEDED / RETRACTED / REJECTED
```

### Does not own

```text
LLM extraction runtime
automatic promotion
search UI
team chat
GitHub App
```

### Entry

```text
003 PASS
004 graph decision recorded
005 PASS only if 004 required immediate production graph
```

### Hard rule

```text
MODEL_INFERENCE != ACTIVE_MEMORY
MEMORY_CI_GREEN != AUTHORIZATION
```

---

## 007 — Universal Context and Memory Gateway

**Proposed name:** `007-universal-context-memory-gateway`

### Dominant outcome

Any authorized client or agent can request bounded task-specific Fehrest context and receive a receipted package without receiving arbitrary repository authority.

### Owns

```text
principal/session/grant request model
authorization chokepoint
Context Compiler production pipeline
scope filtering
budgeting
context package contract
receipt/manifest
SelectionTrace
CLI/SDK/local API core contracts
MCP adapter candidate when authorized
client adapter security rules
```

### Does not own

```text
GitHub-specific discovery
LLM execution/provider runtime
browser/WebMCP acquisition
organization RBAC
```

### Entry

```text
006 PASS
retrieval substrate available
```

### Required property

Every adapter is a projection of the same grant/authorization semantics. No adapter may mint wider authority.

---

## 008 — GitHub Link and IDE Discovery

**Proposed name:** `008-github-link-ide-discovery`

### Dominant outcome

Opening a GitHub repository in an arbitrary compatible IDE/agent is sufficient to discover and request authorized Fehrest project memory.

### Owns

```text
GitHub <-> Fehrest repository binding
repo-local non-secret discovery manifest
GitHub App integration candidate
GitHub repository/project mapping
issue/PR/discussion/Actions evidence intake
IDE-independent discovery flow
GitHub provenance binding
```

### Candidate artifact

```text
.fehrest/link.toml
```

with mandatory constraints:

```text
CONTAINS_SECRET=NO
GRANTS_PERMISSION=NO
OVERRIDES_FEHREST_AUTHORIZATION=NO
```

### Entry

```text
007 PASS
GitHub integration security review prepared
```

---

## 009 — Trusted Vertical Memory Proof

**Proposed name:** `009-trusted-vertical-memory-proof`

### Dominant outcome

Prove that the Fehrest substrate materially improves real continuation for fresh humans/agents versus strong simpler baselines before broad product UI investment.

### Owns

```text
frozen end-to-end continuation benchmark
GitHub-linked workflow proof
fresh-agent continuation
fresh-human continuation
trust failure cases
cost/safety measurements
current memory benchmark subset
```

### Must include

```text
stale memory
contradictory memory
untrusted agent proposal
revoked grant
secret-bearing evidence
historical/as-of question
```

### Exit

Only a preregistered result permitting continued product investment makes broad workspace work eligible.

---

## 010 — Workspace Canonical Object and Open-Format Foundation

**Proposed name:** `010-workspace-object-open-format-foundation`

### Why this spec exists

The V2 vision requires human workspace objects, but canonical schemas must not be invented inside UI code.

### Dominant outcome

Fehrest has a stable open canonical foundation for personal/workspace objects that can be rendered by multiple clients without proprietary lock-in.

### Owns

The minimum canonical semantics required for the next personal workspace, potentially including:

```text
Space
Project
Note/Document identity
Source
Task
Decision reference surface where not already owned by 006
Attachment reference
properties/metadata envelope
explicit canonical links
open-format mapping
trash/archive/recovery semantics
```

Exact entity split is decided by the spec and must avoid duplicating Memory semantics from 006.

### Critical ownership rule

`Decision` may reference memory/provenance semantics owned by 006; 010 must not redefine memory lifecycle.

### Entry

```text
009 PASS
founder authorizes human workspace tranche
```

---

## 011 — Personal Notes, Docs and Capture Workspace

**Proposed name:** `011-personal-notes-docs-capture-workspace`

### Dominant outcome

An individual can use Fehrest daily for local notes/documents/capture without needing Obsidian for the measured core workflow.

### Owns

```text
desktop/web shell for personal local mode
editor integration
quick capture/inbox
daily notes
templates
wikilinks/backlinks presentation
attachments
history/restore UX
keyboard-first navigation
basic tasks/projects presentation
```

### Does not own

```text
search engine semantics
AI provider runtime
team collaboration
```

### Entry

```text
010 PASS
editor gate CLOSED with evidence
```

---

## 012 — Search, Graph Exploration and Bases UX

**Proposed name:** `012-search-graph-bases-workspace-ux`

### Dominant outcome

A user can quickly find, filter, structure and visually explore local knowledge without AI or network access.

### Owns

```text
universal Search UX
deterministic filter/query UX
saved searches
Search <-> Graph scope synchronization
Global/Local/Search/Project graph visualization
explicit-link graph visualization
optional derived-graph overlay when available
structured collections/bases views
Table/Board/List/Calendar/Timeline presentation definitions
```

### Critical separation

```text
GRAPH_UI != GRAPH_INTELLIGENCE_PROVIDER
```

Graph visualization over explicit canonical relationships may exist even if 004 rejected production graph intelligence.

### Entry

```text
003 PASS
010 PASS
011 PASS
004 decision known
```

---

## 013 — AI Provider Runtime and Ask Fehrest

**Proposed name:** `013-ai-provider-ask-fehrest`

### Dominant outcome

A user can choose AI OFF, a local LLM, a self-hosted/custom endpoint or a connected provider and ask Fehrest to answer/draft using only authorized context.

### Owns

```text
AI provider abstraction
local/remote/custom provider configuration
capability probing
model selection UX
provider privacy/location UX
Ask Fehrest orchestration
inline AI edit preview/diff
model execution failure classes
cost/usage display where available
```

### Initial provider research family

```text
Ollama
LM Studio
llama.cpp server
self-hosted OpenAI-compatible endpoints
remote providers through adapters
```

### Hard rules

```text
MODEL != MEMORY
PROVIDER != AUTHORITY
AI_OFF_REMAINS_COMPLETE=YES
```

### Entry

```text
007 PASS
012 search/context surfaces available
```

---

## 014 — External Evidence and WebMCP

**Proposed name:** `014-external-evidence-webmcp`

### Dominant outcome

A user/agent can perform scoped web research, preserve source provenance and use structured WebMCP/browser tools without external content gaining Fehrest authority.

### Owns

```text
web authorization policy
source acquisition contract
external source record
WebMCP provider abstraction
search/browser/http connector boundaries
read vs action tool classification
origin/domain policy
prompt-injection defenses
web invocation receipts
source freshness/recheck workflow
```

### Entry

```text
007 PASS
013 tool/model execution boundary stable enough where model-driven research is included
```

### Hard rule

```text
WEB_CONTENT != INSTRUCTION
WEB_TOOL_DESCRIPTION != GRANT
```

---

## 015 — Import and Migration Lab

**Proposed name:** `015-import-migration-lab`

### Dominant outcome

A user can migrate existing knowledge into Fehrest with dry-run mapping, provenance and reversibility rather than starting over.

### Initial priority

```text
Markdown folder
Obsidian vault
GitHub-linked project knowledge
```

then measured/adoption-driven support for:

```text
Notion
Slack
Zulip
AFFiNE
AppFlowy
Anytype
CSV/JSON/HTML
```

### Owns

```text
import adapter contract
mapping preview
unsupported-field report
batch provenance
rollback/remove-import-batch behavior where safe
original source/export preservation options
migration quality benchmarks
```

### Entry

```text
010 canonical workspace model stable
011/012 destination UX sufficiently stable
```

---

## 016 — Collaboration Capability Experiment

**Proposed name:** `016-collaboration-capability-experiment`

### Dominant outcome

Determine which collaboration/sync mechanism, if any, satisfies Fehrest local-first, provenance, recovery and authorization requirements.

### Candidate family

```text
Automerge
Yjs/Yrs
Y-Octo
Loro
AFFiNE/OctoBase architecture patterns
```

### Owns

```text
preregistered collaboration benchmark
multi-writer/offline conflict experiment
revocation-under-partition adversarial cases
schema migration compatibility experiment
retain/defer/reject decision
```

### Exit

```text
COLLAB_RETAIN
COLLAB_DEFER
COLLAB_REJECT
```

No production team collaboration may begin merely because a CRDT library is popular.

---

## 017 — Sync and Multi-Writer Collaboration Substrate

**Proposed name:** `017-sync-multiwriter-collaboration-substrate`  
**Conditional:** requires `016 = COLLAB_RETAIN`

### Dominant outcome

Multiple authorized devices/users can edit supported shared state offline/online and converge without violating canonical provenance or revocation policy.

### Owns

```text
sync protocol/provider boundary
replica/device identity
multi-writer merge semantics
conflict representation
sync health
reconnect/replay rules
revocation behavior
self-hosted/managed sync protocol boundary
backup/recovery interaction
```

### Does not own

```text
organization roles
channels/topics UX
```

---

## 018 — Organization Identity, Policy and Admin Foundation

**Proposed name:** `018-organization-identity-policy-admin`

### Why this must precede team communication

Channels, DMs, guests and shared documents create multi-principal confidentiality and authorization boundaries. Those boundaries must exist before the communication UI that depends on them.

### Dominant outcome

A team/company can define organizations, members, groups, roles, guests and policy with auditable scope enforcement.

### Owns

```text
Organization identity
membership
roles/groups
guest/external collaborator model
policy/grant evaluation extension
session/device administration
audit/admin contracts
retention policy primitives
self-host/managed deployment policy surface
SSO/SCIM candidates where later justified
```

### Entry

```text
017 PASS
security architecture approved
```

---

## 019 — Team Communication and Shared Workspace

**Proposed name:** `019-team-communication-shared-workspace`

### Dominant outcome

A small team can run a realistic project in Fehrest without requiring separate Slack/Zulip + Notion/Obsidian for the measured scenario.

### Owns

```text
channels
topics
threaded replies
DM/group DM
mentions/reactions
read state
presence/typing
notifications
shared docs/tasks/decisions surfaces
files
agent participation/activity
knowledge crystallization UX
```

### Critical rule

Conversation may propose durable knowledge, but does not become canonical memory merely because an AI summarized it.

### Entry

```text
017 PASS
018 PASS
team security/privacy scenarios preregistered
```

---

## 020 — Mobile and Offline Capture Client

**Proposed name:** `020-mobile-offline-capture-client`

### Dominant outcome

A user can reliably capture/search/respond/review on mobile while offline and later synchronize according to the proven collaboration model.

### Owns

```text
mobile client presentation
quick text capture
voice/photo/share-sheet capture
offline note/search subset
topic reply
task updates
Memory Proposal review
sync status
notification UX
biometric local lock candidate
```

Mobile technology is selected in research/plan, not in this program map.

---

## 021 — Extension, Automation and Connector Platform

**Proposed name:** `021-extension-automation-connector-platform`

### Dominant outcome

Third parties/users can safely extend Fehrest without receiving unrestricted repository/network/process authority.

### Owns

```text
extension manifest
capability grants
connector/importer/exporter/view/automation provider contracts
extension lifecycle/update policy
sandbox/process boundary decision where needed
scheduled/event-driven automation contracts
```

### Entry

Authorization semantics from prior phases must be stable.

---

## 022 — Fehrest Hub and Network Effects

**Proposed name:** `022-fehrest-hub-network`

### Dominant outcome

Users/organizations can host, discover, share, cite, watch, fork/copy and propose changes to public/private Memory Repositories without making the hosted service the only canonical copy.

### Owns

```text
hosted repository identity/linkage
public/private repository discovery
organization hosting
citation/watch/follow
proposal/review across hosted repositories
copy/fork with provenance
agent-readable repository packages
```

### Hard invariant

```text
HOSTED_HUB != ONLY_CANONICAL_COPY
```

### Entry

Local-first personal/team product proof must already exist.

---

# 4. Dependency graph

```text
R1
 |
 +-> G-PROV
 +-> G-CONST
 +-> G-V2
       |
       v
      002
       |
      003
       |
      004 -----------+
       |             |
       | RETAIN_NOW  | DEFER/REJECT
       v             |
      005            |
       +------->-----+
               |
              006
               |
              007
               |
              008
               |
              009
               |
              010
               |
              011
               |
              012
               |
              013
               |
              014
               |
              015
               |
              016
               |
       if COLLAB_RETAIN
               v
              017
               |
              018
               |
              019
               |
              020
               |
              021
               |
              022
```

This graph is intentionally conservative because repository governance permits one active product frontier.

---

# 5. No-cycle proof by ownership direction

The dependency direction is designed so lower layers never depend on higher product surfaces:

```text
Core -> Derived -> Memory -> Gateway -> Integrations/Proof -> Workspace -> AI/Web -> Collaboration -> Team -> Ecosystem/Hub
```

Forbidden reverse dependencies include:

```text
Core depends on UI                     = NO
Canonical memory depends on graph DB   = NO
Gateway depends on GitHub              = NO
Search correctness depends on LLM      = NO
Notes persistence depends on AI        = NO
Authorization depends on chat content  = NO
Team policy depends on channel UI      = NO
Hub owns only canonical copy           = NO
```

---

# 6. Strategic checkpoints

The program should stop and reassess at these checkpoints rather than assuming every phase ships.

```text
CP-1 after R1 / V2 decision
CP-2 after 003 retrieval correctness
CP-3 after 004 graph decision
CP-4 after 006 memory product proof
CP-5 after 009 trusted vertical proof
CP-6 after 012 personal workspace/search/graph proof
CP-7 after 016 collaboration decision
CP-8 after 019 team product proof
CP-9 before Hub/network investment
```

Each checkpoint can route to:

```text
CONTINUE
CONTINUE_WITH_CONSTRAINTS
DEFER_OPTIONAL_CAPABILITY
REPAIR
RETHINK
STOP
```

---

# 7. Current program state

```text
SPEC_SEQUENCE_PROPOSED=YES
SPEC_SEQUENCE_CANONICAL=NO
ACTIVE_SPEC_CHANGED=NO
R1_CHANGED=NO
CURRENT_CHANGED=NO
```
