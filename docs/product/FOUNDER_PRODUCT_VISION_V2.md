# Fehrest Founder Product Vision v2

**Status:** FOUNDER DIRECTION CANDIDATE / NON-AUTHORIZING  
**Change class:** E — product thesis / founder direction  
**Created:** 2026-08-28  
**Execution effect:** NONE while R1 is open  
**Canonical frontier:** `specs/CURRENT.md`

> This document records a proposed future product direction. It does not reinterpret R1, mutate sealed R1 semantics, activate Spec 002, authorize UI/MCP/graph/vector/automatic-memory work, or change the canonical Execution Master Plan by itself.

---

## 1. Founder north star

Fehrest should become the default memory workspace used alongside GitHub and, over time, the primary durable workspace for individuals, teams and companies.

The intended category is not merely:

```text
note app
knowledge base
chat app
AI memory library
project manager
agent framework
```

The intended category is:

```text
LOCAL-FIRST MEMORY REPOSITORY + WORKSPACE
FOR HUMANS, TEAMS AND AI AGENTS
```

The simple strategic pairing is:

```text
GitHub  = where work and code are built and reviewed
Fehrest = where people and agents understand, communicate and remember
```

The product must remain useful for people who never write code. GitHub integration is the flagship developer path, not the boundary of the market.

Primary audiences:

```text
individuals
families and small groups
researchers
students
creators
founders
software teams
product teams
operations teams
professional services
companies
open-source communities
AI agents and agent fleets
```

---

## 2. Category thesis

Existing products converge on an all-in-one workspace:

- Slack expands from chat into documents, lists and workflow.
- Notion expands from documents/databases into search, projects, connected applications and agents.
- Obsidian expands from local notes into structured views, sync and collaboration.
- AFFiNE, AppFlowy and Anytype combine local-first knowledge/workspace primitives.
- Buzz combines humans, agents, channels, canvases, workflows and repository activity around a common event substrate.
- agent-memory systems such as Letta, Mem0 and Graphiti make long-lived context and temporal memory increasingly central.

Therefore `ALL_IN_ONE` is not a sufficient differentiator.

Fehrest's differentiator should be the substrate beneath the surfaces:

```text
CANONICAL HUMAN + AGENT MEMORY
WITH OWNERSHIP, HISTORY, PROVENANCE,
TEMPORAL TRUTH, SCOPE AND AUDITABILITY
```

The product should win because all surfaces operate on one trustworthy memory model, not because Fehrest contains the largest number of tabs.

---

## 3. Product promise

### Public promise

> **One Memory. Every Surface. Any Agent. Fully Yours.**

### Expanded promise

Fehrest is a local-first memory repository where people and agents can write, communicate, organize work, retrieve context and preserve durable knowledge without surrendering ownership of the underlying state.

A user's or organization's knowledge must remain:

```text
open
inspectable
portable
recoverable
versioned
traceable
usable offline
usable without mandatory AI
```

Derived intelligence may disappear and be rebuilt. Canonical memory may not silently disappear with it.

---

## 4. The primitive: Memory Repository

GitHub's durable primitive is the repository. Fehrest's proposed durable primitive is the **Memory Repository**.

A Memory Repository can belong to:

```text
one individual
one household/group
one project
one GitHub repository
one research program
one team
one company department
one company
one public community
```

A Memory Repository contains durable objects such as:

```text
Note
Document
Source
Decision
Task
Project
Topic
Message
Meeting
Person
Agent
Memory
Skill
Procedure
Artifact
Experiment
Incident
Trajectory
Preference
Constraint
Reference
```

These are not separate application databases. They are views and typed objects over one canonical substrate with stable identity and provenance.

Every canonical object should be able to bind, where applicable:

```text
stable object identity
author / agent identity
creation and modification history
recorded time
valid time
source evidence
provenance
scope
trust/lifecycle state
relationships
supersession/retraction
review state
```

No vector id, graph id, parser id, file path, model output or search rank becomes canonical authority by itself.

---

## 5. One core, many surfaces

The user should experience one coherent product rather than separate clones of Slack, Notion and Obsidian.

A space or project may expose surfaces such as:

```text
Overview
Stream
Topics
DMs
Notes
Docs
Tasks
Collections / Bases
Files
Canvas
Decisions
Meetings
Sources
Memory
Agents
Activity
Search / Ask
```

These surfaces act on the same underlying objects.

Examples:

```text
message -> task
message/topic -> decision proposal
meeting -> notes + tasks + memory candidates
source -> citation
agent run -> trajectory + memory candidates
decision -> implementation context
note -> table / board / calendar / graph view
```

Conversion must preserve origin and provenance rather than silently copying text between siloed databases.

---

## 6. Notes and personal knowledge: Obsidian-class or better

Fehrest must be excellent as a personal/local knowledge workspace even when collaboration and agents are disabled.

Expected eventual experience includes:

```text
Markdown-first authoring
ordinary-file ownership
fast startup and navigation
backlinks / wikilinks
tags and properties
daily notes
templates
references and transclusion
attachments
PDF/source annotation
full-text search
saved searches
collections / bases
outline
calendar
kanban
timeline
canvas
knowledge graph views
keyboard-first operation
mobile capture
```

The canonical representation should remain open and recoverable without proprietary cloud storage.

Views are derived/presentational. A table, board, calendar or graph view must not become the only place the underlying knowledge can be recovered.

---

## 7. Team communication: Slack-class usability with durable topics

Fehrest should eventually be credible as the primary communication workspace for a team.

Expected surface:

```text
channels
mandatory or strongly encouraged topics
threads/replies
DMs and group DMs
mentions
reactions
presence
typing
files
voice notes
notifications
pinned knowledge
guests/external collaborators
search
workflow events
agent presence and updates
meeting/huddle integration
```

Zulip's topic discipline is an important design reference because a durable topic is more useful for memory than an undifferentiated message timeline.

Proposed hierarchy:

```text
Workspace
  -> Space
    -> Channel
      -> Topic
        -> Messages
        -> Decisions
        -> Tasks
        -> Docs
        -> Sources
        -> Agent activity
```

A conversation should be able to crystallize into durable knowledge without erasing the original conversation.

---

## 8. Knowledge Crystallization

A defining Fehrest workflow should be **Knowledge Crystallization**.

Conversation and activity remain evidence. They do not automatically become canonical truth.

Proposed flow:

```text
conversation / meeting / agent trajectory / source
-> candidate knowledge
-> proposed memory / decision / procedure / task
-> contradiction + provenance checks
-> authorized human or policy review
-> canonical activation
-> later supersession / retraction when evidence changes
```

Example:

```text
Topic: database/concurrency

Evidence:
- SQLite experiment failed criterion C-3
- PostgreSQL experiment passed
- team discussion accepted the change

Candidate:
Decision: Use PostgreSQL for the service

Lifecycle:
PROPOSED -> CONFIRMED -> ACTIVE
```

Fehrest must preserve the original evidence and should make it easy to inspect why a memory became active.

---

## 9. Memory lifecycle and temporal truth

Agent memory should not mean "save model summaries forever."

Fehrest should support explicit lifecycle and temporal resolution for durable knowledge.

Illustrative lifecycle:

```text
OBSERVED
-> CANDIDATE
-> CONFIRMED
-> ACTIVE
-> SUPERSEDED / RETRACTED
```

Important queries:

```text
What is true now?
What did we believe on a specific date?
When did this change?
What evidence supports it?
What contradicts it?
What superseded it?
Which memories are stale?
Which memories are unconfirmed agent inference?
```

This is a primary differentiation from plain note search or embedding retrieval.

---

## 10. Memory Proposals: pull-request discipline for durable knowledge

Agents and automations should not silently edit high-trust durable memory.

Fehrest should eventually support a reviewable **Memory Proposal** mechanism inspired by the strongest parts of GitHub pull requests while using end-user language appropriate to knowledge work.

A proposal may show:

```text
current value
proposed value
evidence
reason
originating topic / source / trajectory
contradictions
superseded objects
requesting human/agent
required reviewers/policy
```

This allows teams to review durable changes without forcing ordinary users to understand Git internals.

---

## 11. Memory CI

A long-term iconic capability should be **Memory CI**: automated checks over proposed durable knowledge.

Candidate checks:

```text
missing provenance
missing citation
stale source
contradiction
secret leakage
PII/policy violation
broken link/source
unauthorized promotion
untrusted agent inference
duplicate memory
superseded claim
invalid temporal state
invalid scope
```

Illustrative result:

```text
MEMORY CHECKS
PASS provenance
PASS scope
PASS secret scan
WARN contradicts M-442
WARN cited source exceeded freshness policy
BLOCK unauthorized promotion to ACTIVE
```

Checks inform authorization; they do not mint authority themselves.

---

## 12. Search and Ask: query memory, not only text

Search is a primary product surface.

Fehrest should eventually support questions such as:

```text
What changed since my last session?
Why did we make this decision?
What is true now?
What did we believe in March?
What repeatedly failed?
What did the previous agent try?
What should this task know?
What should this task NOT trust?
Which assumptions are stale?
Which active memories lack evidence?
Where do people disagree?
Who has context on this subject?
```

Answers should preserve receipts and source trails.

Semantic, graph, lexical and model-assisted techniques remain derived mechanisms competing to answer these questions. None becomes canonical truth by rank alone.

---

## 13. Fehrest + GitHub: flagship integration

GitHub integration is strategically first-class because Fehrest should become the natural memory repository beside a code repository.

Target mental model:

```text
GitHub repository <-> Fehrest Memory Repository
```

### 13.1 Repository binding

A GitHub repository should be able to bind to one or more Fehrest memory spaces through a small, inspectable, non-secret discovery record.

Candidate future mechanism, subject to a dedicated spec and security review:

```text
.fehrest/link.toml
```

Possible fields:

```text
schema_version
memory_repository_id
workspace_id
project_id
preferred_gateway
policy/profile reference
```

The link file must contain no secret and must not itself grant authorization.

### 13.2 GitHub App

A future Fehrest GitHub App should provide permission-scoped integration for:

```text
repositories
issues
pull requests
discussions
commits/releases
Actions/check results
project metadata
selected organization metadata
```

GitHub activity becomes evidence/events visible to Fehrest. GitHub content does not automatically become canonical Fehrest memory.

### 13.3 Any IDE or agent working from GitHub should discover Fehrest

The key goal is not "build a Fehrest IDE."

The goal is:

> Any IDE, coding agent or automation that can open a GitHub repository should have a standard, low-friction way to discover and request that repository's Fehrest context.

Future interoperability surfaces should include, when authorized:

```text
CLI
SDK
HTTP/local API
MCP
ACP/client adapters where useful
GitHub App
repo-local discovery manifest
```

A coding agent opening a repository should be able to perform an equivalent of:

```text
fehrest context --task "fix issue 812"
```

and receive only its scoped, budgeted, receipted context.

IDE-specific integrations are optional convenience layers over the open Fehrest protocol, not the canonical integration model.

### 13.4 GitHub is a partner substrate, not a dependency for everyone

Fehrest must remain complete for users with no GitHub account.

Examples:

```text
student personal vault
research group
writer/creator
family knowledge space
nontechnical company team
legal/consulting team
operations workspace
personal life memory
```

GitHub is the flagship integration for software work, while Fehrest's memory model remains domain-general.

---

## 14. Agent gateway: every agent should be able to use Fehrest

Fehrest should not need to own or execute every agent.

Desired ecosystem position:

```text
Claude Code   \
Codex          \
Hermes          -> Fehrest Context/Memory Gateway -> canonical memory
OpenHands      /
Letta         /
IDE agents   /
enterprise agents
```

The gateway should eventually support:

```text
denied-by-default authorization
session/task grants
project/object scopes
subagent subset grants
model-visible receipts
request digest
canonical high-water mark
selection trace
budget accounting
```

A model, retrieved document or agent-generated instruction cannot widen the user's grant.

---

## 15. Local-first is a contract

The product should be designed so that these statements remain true:

```text
Internet unavailable -> local Fehrest remains useful.
Fehrest company unavailable -> canonical user files remain useful.
AI provider unavailable -> core remains useful.
Sync service unavailable -> local state remains available.
Derived index corrupt -> rebuild it.
Graph/vector backend removed -> canonical memory remains.
Plugin removed -> canonical content remains.
Model changed -> historical memories remain inspectable.
```

Collaboration and sync may require services for live multi-user operation, but they must not redefine service-hosted derived state as the only canonical user truth.

---

## 16. Collaboration becomes a product-level requirement, not an immediate implementation authorization

The founder direction requires an eventual credible product for teams and companies:

```text
multi-device
multi-user
real-time collaboration
offline edits
conflict resolution
guests
organization policy
agent collaboration
private/on-prem deployment where required
```

This creates a major future architecture gate because the currently known historical core uses conservative single-writer semantics.

Do not silently replace that invariant now.

Post-R1 architecture reconsideration should evaluate whether and where collaboration belongs relative to canonical ownership, and benchmark candidate mechanisms such as:

```text
Automerge
Yjs / Yrs
Y-Octo
Loro
AFFiNE/OctoBase-derived patterns
```

No CRDT implementation is selected by this document.

---

## 17. Communication media and meetings

Fehrest should eventually support communication experiences sufficient to replace a separate team-chat product for many teams.

This does not imply building every infrastructure layer internally.

For voice/video/huddles, the preferred future discipline is provider/adaptor evaluation before invention. Systems such as LiveKit or Jitsi may be benchmark/donor candidates if the need becomes authorized.

Fehrest's unique ownership should remain the memory/provenance/context around the communication, not a custom media transport stack.

---

## 18. Import and migration are core adoption features

Fehrest cannot become a default workspace by asking users to abandon years of history.

A future **Import Lab** should preserve provenance while importing from systems such as:

```text
Obsidian
Notion
Slack
Zulip
GitHub
AFFiNE
AppFlowy
Markdown folders
JSON/CSV
selected document/cloud sources
```

Import principles:

```text
never erase original source identity
record importer version
record acquisition time
record source revision/hash where available
preserve timestamps where trustworthy
preserve attachments
preserve relationship mapping where possible
report unsupported constructs explicitly
```

Migration quality should be benchmarked as part of product readiness, not treated as a one-time script.

---

## 19. Extension ecosystem

Long-term adoption requires an ecosystem while preserving the security model.

Potential categories:

```text
Connectors
Importers
Views
Automations
Templates
Skills
Agent adapters
Exporters
Optional derived providers
```

Extensions should be capability-scoped.

Illustrative permissions:

```text
notes:read
notes:write
space:research:read
network:none
process:none
secrets:none
```

A plugin ecosystem must not become a path around canonical authorization or provenance rules.

---

## 20. Public/private Fehrest Hub

Long-term network effects may require a hosted **Fehrest Hub** while preserving local-first ownership.

Possible future surfaces:

```text
private organization memory repositories
public knowledge repositories
research collections
playbooks
standards
skills/procedures
templates
citations
watch/follow
proposal/review
fork/copy with provenance
```

The strongest analogy is not "social network for notes." It is an ecosystem of durable knowledge repositories that can be consumed by humans and agents.

This is future product direction only; no Hub implementation is authorized by this document.

---

## 21. Product wedge and expansion

The founder direction is universal, but the initial high-leverage wedge should remain disciplined.

Proposed adoption sequence to evaluate after R1:

```text
1. Individual project memory
2. GitHub-linked developer/team memory
3. Team docs + topics + decisions + tasks
4. Agent context/memory gateway
5. Full personal/team knowledge workspace
6. Organization administration/collaboration
7. Public/private knowledge repository network
```

This is a proposed go-to-market/product sequence, not an execution authorization sequence.

The product should be useful at every step rather than requiring the final platform to exist first.

---

## 22. Success measures

Vanity feature count is not success.

Future product success should be measured by outcomes including:

```text
continuation correctness for fresh agents
human time-to-recover project context
successful task completion with bounded context
percentage of important decisions with recoverable rationale
memory contradiction/staleness detection
import fidelity
offline/local reliability
collaboration convergence
search/answer evidence quality
context token/cost efficiency
retention driven by accumulated useful memory
export/recovery success
GitHub-linked repository activation rate
percentage of active agents using Fehrest context
```

A defining product test remains:

> Can a fresh disposable human or agent continue real work more correctly and efficiently because Fehrest preserved and compiled the right memory?

---

## 23. Explicit non-goals of the vision

Even under this expanded direction, Fehrest should not become:

```text
a mandatory cloud service
a proprietary-only file format
a new Git implementation solely to replace GitHub
a general agent framework
a sandbox platform
a model provider
a vector database product
a graph database product
a custom video infrastructure company
an automatic-memory system where model inference equals user authority
```

Fehrest may integrate with these capabilities while keeping its unique ownership boundary clear.

---

## 24. Rights and donor-code discipline

The founder reports that permission exists to copy/adapt code from the supplied source set.

This direction increases the opportunity to `USE` or `ADAPT` donor code, but does not remove provenance discipline.

Every copied/adapted source slice must still record at minimum:

```text
source repository
source revision/commit
source paths
license and/or permission evidence
what was copied/adapted
local changes
security review
update policy
exit/replacement strategy
```

Permission is not a reason to create an untraceable code collage.

---

## 25. Required post-R1 architecture reconsideration

If R1 permits continued investment, this founder direction should trigger a dedicated architecture/product reconciliation before the roadmap is widened.

Questions to resolve include:

```text
Does the existing product thesis remain sufficient or require amendment?
How should universal personal/team workspace scope be staged?
Where does multi-user collaboration enter the architecture?
Does graph production still precede temporal memory productization?
Should the GitHub link/gateway move earlier than previously planned?
What is the minimum notes/team surface needed to prove the memory repository?
What collaboration mechanism can coexist with canonical ownership?
How is Fehrest Hub separated from local canonical authority?
Which donor systems should move from STUDY to BENCHMARK/ADAPT?
```

These questions must not be answered by changing R1 after the fact.

---

## 26. Current execution boundary

At the time this document was created, live repository truth states:

```text
ACTIVE_EXECUTION_FRONTIER=R1
NEXT_PRODUCT_SPEC=002-post-r1-canonical-core-convergence
NEXT_PRODUCT_SPEC_STATUS=BLOCKED_BY_R1_TERMINAL_GATE_AND_FOUNDER_AUTHORIZATION
```

Therefore:

```text
R1_SEMANTIC_MUTATION=NO
PRODUCT_IMPLEMENTATION_FROM_THIS_VISION=NO
SPEC_002_ACTIVATION_FROM_THIS_VISION=NO
CURRENT_UPDATE=NO
CANONICAL_MASTER_PLAN_REWRITE=NO
```

This file preserves founder direction so that the correct post-R1 architecture process can evaluate it with evidence.