# Fehrest UX Blueprint v2

**Status:** PRODUCT UX PROPOSAL / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Change class:** E product direction with future C/D architecture and security gates  
**Canonical execution frontier:** `specs/CURRENT.md`

> This document describes the intended end-to-end experience for humans and agents. It does not authorize UI, MCP, WebMCP, model-provider, sync, collaboration, graph, vector, automatic-memory, or other implementation while R1 remains open.

---

## 1. Experience thesis

Fehrest should feel like one product, not a bundle of Slack, Notion, Obsidian and AI features.

The core UX rule is:

```text
ONE MEMORY
MANY SURFACES
```

A note, decision, source, task, message, meeting, memory or agent run is not trapped inside a feature silo. The same underlying object can appear in search, graph, project, topic, timeline, context package or review workflow without copying the canonical knowledge into unrelated stores.

The product promise is:

> **One Memory. Every Surface. Any Agent. Fully Yours.**

---

## 2. User classes

Fehrest must work without requiring a software-development identity.

Primary modes:

```text
PERSONAL
TEAM
COMPANY / ORGANIZATION
AGENT / AUTOMATION
```

Representative users:

- student collecting notes and sources;
- researcher maintaining literature, claims and experiments;
- writer or creator building a personal knowledge system;
- founder managing company decisions and institutional memory;
- product/operations team using topics, tasks, docs and decisions;
- engineering team linked to GitHub repositories;
- company with private/self-hosted memory repositories;
- AI agent consuming scoped memory and proposing durable updates.

---

## 3. First-run experience

### 3.1 Welcome

The first screen should answer one question:

> Where should your memory live?

Choices:

```text
Create a personal Memory Repository
Join a team/company
Import existing knowledge
Connect an existing Fehrest repository
Connect GitHub
```

Do not force AI setup before the user can write a note.

### 3.2 Local ownership first

Default personal flow:

1. Choose a local folder or accept a recommended local Fehrest location.
2. Fehrest explains in plain language that core content remains locally owned and exportable.
3. Create the repository.
4. Open directly into a clean Home/Today surface.

No account should be required for the smallest offline personal mode unless a later architecture/security decision proves an unavoidable need.

### 3.3 Optional sync/collaboration

After local repository creation, offer—not require—future sync or team features.

The UI must distinguish:

```text
LOCAL_ONLY
SYNCED
TEAM_SHARED
SELF_HOSTED_ORGANIZATION
```

The user should always know where the canonical bytes and synchronized replicas live.

### 3.4 Import

Import should be treated as onboarding, not a hidden settings feature.

First-class future import paths:

```text
Obsidian vault
Markdown folder
Notion export
Slack export
Zulip export
GitHub repository context
AFFiNE/AppFlowy/Anytype where formats permit
CSV/JSON
browser/web capture
```

Every import records provenance and preserves original source artifacts when appropriate.

---

## 4. Global navigation

The desktop/web shell should remain calm even when the product becomes broad.

Recommended primary navigation:

```text
Home
Search
Graph
Inbox
Notes
Spaces
Tasks
Agents
Activity
```

Team/company repositories additionally surface:

```text
Channels
People
Organization
```

Secondary/project-specific views appear inside a Space/Project rather than expanding the global sidebar forever.

---

## 5. Home

Home is not a social feed optimized for engagement.

It is the user's memory/action briefing.

Possible sections:

```text
Continue where you left off
Today
Recently changed
Needs your review
Memory Proposals
Assigned tasks
Mentions
Agent updates
Stale/contradictory knowledge
Upcoming meetings/deadlines
Quick capture
```

The Home surface should be useful with AI disabled.

With AI enabled, it may add a concise generated briefing, but every generated claim must be traceable to Fehrest evidence.

---

## 6. Universal capture

Capture must be faster than deciding where something belongs.

### 6.1 Quick capture

Global shortcut opens one input:

```text
Write anything...
```

The user can capture:

- text;
- checklist;
- link;
- image;
- file;
- voice note;
- quote;
- task;
- source;
- thought;
- meeting note.

The item lands in Inbox if no destination is chosen.

### 6.2 Capture first, organize later

Fehrest may suggest:

```text
possible space
possible tags
possible links
possible duplicate
possible existing topic
possible task/decision candidate
```

Suggestions remain suggestions. AI classification does not create authority by itself.

---

## 7. Notes and documents

The writing experience must meet or exceed the simplicity expected by Obsidian-class users.

Required long-term capabilities:

```text
Markdown-first editing
rich but reversible rendering
headings
lists
checklists
code blocks
math
callouts
footnotes
citations
properties/frontmatter
wikilinks
backlinks
block references
transclusion
attachments
comments
version history
restore
templates
daily notes
aliases
bookmarks/favorites
outline
focus mode
split panes
keyboard-first commands
```

The user must not need to understand internal object IDs, event journals or provenance schemas for ordinary note taking.

Advanced metadata is available on demand in an Inspector.

---

## 8. Search: the fastest way back to memory

Search is a defining surface, not a utility box.

### 8.1 One universal search

`Cmd/Ctrl+K` opens search from anywhere.

Search modes:

```text
Keyword
Semantic (optional derived capability)
Structured filters
Ask
Commands
People/agents
```

### 8.2 Search should work before AI

Core search must remain useful with:

```text
AI=OFF
NETWORK=OFF
```

Baseline candidates include deterministic lexical/index search plus structured metadata filters.

### 8.3 Query language

Human-friendly filters should support concepts such as:

```text
tag:
space:
type:
from:
author:
agent:
created:
updated:
source:
status:
contains:
linked-to:
mentioned-by:
valid-at:
recorded-at:
```

The UI should help users construct filters without memorizing syntax.

### 8.4 Results

Each result should make context obvious:

```text
title
matching excerpt
object type
space/project
author/source
time
why matched
trust/provenance indicator where relevant
```

### 8.5 Search ↔ Graph

Search and Graph are linked views of the same knowledge.

From search:

```text
Show in Graph
Explore neighbors
Show path between results
Show related decisions
Show source chain
```

Graph filters can mirror current search scope.

---

## 9. Graph experience

The graph should preserve the delight of Obsidian-style exploration while becoming more useful than a decorative network.

### 9.1 Graph modes

```text
Global Graph
Local Graph
Search Graph
Project/Space Graph
Temporal Graph
Provenance Graph
Decision Graph
Agent/Memory Graph
```

### 9.2 Node types

Possible nodes:

```text
note
document
source
decision
task
topic
person
agent
memory
project
artifact
meeting
repository
```

### 9.3 Edge types

Explicit edges should remain distinguishable from derived/inferred edges.

Examples:

```text
links-to
cites
supersedes
contradicts
decided-by
belongs-to
mentions
produced-by
derived-from
assigned-to
related-to
```

Inferred graph relationships must never silently become canonical authority.

### 9.4 Interaction

Users can:

- search/filter graph;
- select a node and open its inspector;
- expand 1/2/N hops;
- hide noisy types;
- filter by time, person, project, tag, trust or status;
- pin nodes;
- trace evidence paths;
- switch between graph and list/table;
- save graph views;
- animate historical evolution when useful;
- ask AI about a selected subgraph while preserving the exact selected evidence set.

### 9.5 Graph performance

Large graphs require virtualization/WebGL or another measured rendering strategy. Do not allow the visual layer to dictate the canonical graph storage model.

---

## 10. Bases / structured views

Users should be able to turn the same underlying objects into structured views without moving canonical knowledge into a proprietary database silo.

Views:

```text
Table
Board
List
Calendar
Timeline
Gallery
Graph
Map (where data supports it)
```

A view definition is derived/presentation state.

The underlying note/task/source/decision remains the same object.

---

## 11. Spaces and projects

A Space is the primary human organization boundary.

Examples:

```text
Personal
University
PhD Research
Company
Health
Book
Product Alpha
```

A Project is a work-oriented scope within a Space.

Recommended project tabs:

```text
Overview
Stream
Docs
Tasks
Decisions
Memory
Files
Canvas
Graph
Agents
Activity
```

Engineering projects may additionally show GitHub repository state, but non-developer projects remain equally first-class.

---

## 12. Team communication

Team communication should combine Slack's immediacy with Zulip's topic durability.

Hierarchy:

```text
Organization
  -> Space
     -> Channel
        -> Topic
           -> Messages
```

### 12.1 Channels

Channel types:

```text
open
private
announcement
project-linked
incident
community
```

### 12.2 Topics

Topics are mandatory or strongly encouraged for work channels where durable memory matters.

A topic can accumulate:

```text
messages
decisions
tasks
sources
files
docs
agent activity
```

### 12.3 DMs

Support 1:1 and small group DMs with clear retention/privacy semantics.

DM history should not automatically become shared organization memory.

### 12.4 Knowledge crystallization

A topic can produce proposed durable knowledge:

```text
conversation
-> candidate summary/decision/process/FAQ
-> Memory Proposal
-> review
-> canonical memory
```

The original conversation remains available as evidence.

---

## 13. Tasks and work

Tasks are first-class objects linked to the knowledge that explains them.

Capabilities:

```text
assignee
status
priority
due date
project
subtasks
dependencies
comments
sources
decisions
agent owner/run
activity history
```

The defining UX improvement over generic project managers is context:

> A task should be able to answer "what do I need to know to do this correctly?"

---

## 14. Decisions

Decision records should be effortless enough to use daily.

Decision object:

```text
statement
status
rationale
alternatives
participants
sources/evidence
date
scope
supersedes
contradictions
linked work
```

Fehrest should make stale decisions visible when their premises have changed.

---

## 15. AI experience

AI is a capability layer, not a mandatory runtime dependency for core correctness.

### 15.1 Three modes

```text
AI OFF
LOCAL AI
CONNECTED AI
```

The user can change modes without migrating canonical data.

### 15.2 Local AI

Fehrest should eventually support local runtimes through a provider boundary, prioritizing interoperable APIs rather than hard-coding one runtime.

Candidate local integrations include OpenAI-compatible endpoints exposed by systems such as:

```text
Ollama
LM Studio
llama.cpp
vLLM or other self-hosted compatible endpoints
```

The user experience should be:

1. Fehrest detects known localhost endpoints when explicitly permitted, or user adds one.
2. User selects a model.
3. Fehrest performs a capability check.
4. User sees privacy/location state: `LOCAL`.
5. User can ask Fehrest normally.

### 15.3 Connected providers

Provider settings should support explicit configured providers and custom compatible endpoints.

The UI must show before use:

```text
provider
model
local vs remote
what content scope may be sent
estimated/known context size
whether tools are enabled
```

Secrets remain outside canonical notes/memory/event detail.

### 15.4 Ask Fehrest

The user should not need to learn RAG terminology.

Natural requests:

```text
What did I write about X?
Summarize my research on Y.
What changed in this project this week?
Why did we make this decision?
Find contradictions in my notes.
Prepare me for tomorrow's meeting.
Show everything related to this person.
Draft this report from my sources.
What did the previous agent try?
Continue this task.
```

Answers should default to evidence-backed behavior when operating on repository knowledge.

### 15.5 AI action model

Separate:

```text
READ / ANSWER
DRAFT
PROPOSE CHANGE
EXECUTE TOOL
PROMOTE MEMORY
```

Higher-impact actions require stronger authorization and visible review.

The model itself cannot mint authority.

---

## 16. Web research and WebMCP experience

The user should be able to say:

```text
Research this topic and add the useful sources to my project.
Find the latest official documentation and update my note.
Compare these products and build a cited brief.
```

### 16.1 Web capability layers

Prefer structured and inspectable acquisition:

```text
WebMCP tool where a site exposes appropriate structured tools
approved browser/search provider
explicit HTTP/acquisition adapter
manual import
```

WebMCP is treated as a provider capability, not canonical authority.

### 16.2 User controls

Per repository/space/agent:

```text
WEB=OFF
WEB=ASK_EACH_TIME
WEB=ALLOWED_FOR_SCOPED_TASKS
```

Optional allow/deny lists:

```text
allowed domains
denied domains
read-only tools
action tools requiring confirmation
```

### 16.3 Receipts

A web-assisted answer/action should preserve:

```text
request
provider/tool
URL/origin
acquired_at
source revision/etag when available
raw/normalized hash where applicable
transforms
selected evidence
model/provider
result digest
```

### 16.4 Prompt-injection boundary

Web content and WebMCP tool descriptions are untrusted input.

They cannot:

```text
change user grants
promote memory
reveal secrets
expand repository scope
bypass confirmation
rewrite policy
```

---

## 17. AI writing inside notes

AI should be available where the user works, not only in a separate chat.

Actions:

```text
continue writing
rewrite
summarize
extract tasks
extract questions
find sources
add citations
compare notes
explain selection
translate
turn notes into outline
turn outline into draft
```

Every generated edit should support preview/diff before durable application when the change is significant.

---

## 18. Agent experience

Agents are first-class participants but not equivalent sources of authority.

### 18.1 Agent identity

Each agent/session has:

```text
identity
principal
provider/model
repository/space scope
tool grants
memory grants
web grants
expiry/lifetime
```

### 18.2 Agent startup

An external agent or IDE should be able to:

1. Discover the linked Fehrest repository.
2. Authenticate.
3. Request context for the current task.
4. Receive a bounded Context Package + Receipt.
5. Work using its own runtime.
6. Return evidence/trajectory/proposals.

The agent does not need to run inside Fehrest.

### 18.3 Agent directory

Human users can see:

```text
agent name
purpose
provider/model
owner
active scopes
last activity
current task
memory permissions
web/tool permissions
cost/usage where available
```

### 18.4 Agent Inbox

Agents can receive jobs/tasks, but job assignment does not widen their scope beyond explicit grants.

### 18.5 Agent memory

The agent may have derived/session memory, but durable shared memory follows the Memory Proposal process unless an explicit future policy authorizes a narrower automatic class.

---

## 19. Memory Proposal UX

A Memory Proposal adapts review discipline to durable knowledge.

Show:

```text
before
after
proposal author/agent
reason
sources
supporting messages/tasks/runs
contradictions
scope
security/privacy checks
who must approve
```

Actions:

```text
Approve
Edit and approve
Request changes
Reject
Defer
```

For personal repositories, approval can remain lightweight and configurable while preserving history.

---

## 20. Memory CI UX

Checks should be understandable to non-developers.

Examples:

```text
Source missing
Source is stale
Contradicts an active memory
Duplicate claim
Contains possible secret
Contains sensitive data
Source does not support claim
Untrusted agent proposed authority change
Broken reference
Supersedes an existing memory
```

A green check means checks passed. It does not itself grant permission to merge/promote.

---

## 21. Activity and history

Every important object has human-readable activity/history.

Users can ask:

```text
Who changed this?
What changed?
Why?
What did it replace?
Which agent produced it?
Which sources supported it?
What did we know at that time?
```

The default UI should present a simple timeline; advanced users can inspect exact provenance/receipts.

---

## 22. Time travel

Temporal truth is a defining feature.

Fehrest should eventually make these operations natural:

```text
Show current truth
Show what we believed on DATE
Show what changed since DATE
Show superseded decisions
Show conflicting claims over time
```

Time travel must distinguish valid time from recorded/observed time according to the canonical temporal model.

---

## 23. Collaboration UX

Future collaboration must preserve local-first expectations.

Users should see clear states:

```text
Saved locally
Synced
Pending sync
Conflict/review needed
Offline
```

Conflicts should be understandable and reversible.

Do not expose CRDT implementation vocabulary to ordinary users unless needed for diagnostics.

---

## 24. Organization UX

Company/team requirements eventually include:

```text
organizations
spaces
roles
groups
guests
external collaborators
SSO candidates
SCIM candidate
retention policy
legal/admin export
audit
self-hosting
managed hosting
backup/recovery
policy controls
```

Organization policy must not silently override the user's understanding of where data is stored or who can access it.

---

## 25. Notifications

Default toward calmness.

Notification classes:

```text
needs action
mention
assignment
review request
agent completion/failure
memory contradiction/staleness
security/privacy warning
followed topic update
```

Do not default every channel to noisy notifications.

---

## 26. Mobile

Mobile should prioritize capture, retrieval and review rather than replicating every desktop control.

Primary mobile jobs:

```text
quick note
voice capture
scan/photo
share-to-Fehrest
search
Ask Fehrest
read docs
reply to topic
approve/reject proposal
check tasks
meeting capture
offline access
```

---

## 27. Browser extension / web clipper

Future browser capture should support:

```text
save page
save selection
save link
snapshot metadata
citation
screenshot where permitted
add to project
ask about page
```

The original URL and acquisition metadata remain attached.

---

## 28. GitHub-connected UX

Engineering is the flagship integration path, not the total market.

Inside a linked repository/project the user sees:

```text
repository status
linked issues/PRs/discussions
project decisions
architecture notes
failed experiments
incident history
agent trajectories
Memory Proposals from work
```

An IDE/agent can use repo-local discovery to locate Fehrest but still authenticates separately.

Natural developer experience:

```text
clone repo
open IDE
IDE/agent discovers Fehrest link
agent asks Fehrest for task context
work proceeds
important outcomes return as evidence/proposals
```

---

## 29. Command palette

Power users need one fast command surface.

Examples:

```text
New note
Open note
Search
Open graph
Create task
Create decision
Ask Fehrest
Switch AI provider
Toggle AI OFF
Show provenance
Show history
Create Memory Proposal
Connect GitHub
Import vault
Run saved workflow
```

---

## 30. Automation

Automation should operate on explicit events and scoped capabilities.

Examples:

```text
when meeting ends -> propose summary
when GitHub PR merges -> attach event and propose related memory update
when source TTL expires -> mark for review
when task completes -> ask whether decision/process memory should be captured
when topic becomes inactive -> propose crystallization
```

Automation cannot silently promote agent inference to canonical memory unless a future explicitly bounded policy is authorized.

---

## 31. Accessibility and internationalization

The product must be designed for:

```text
keyboard-only use
screen readers
high contrast
reduced motion
zoom
RTL
multilingual content
mixed-language search
```

Arabic and RTL should be treated as first-class product capabilities rather than late cosmetic patches.

---

## 32. Privacy UX

Users should understand the privacy state without opening documentation.

Visible indicators:

```text
Local only
Synced to <destination>
Shared with <space/group>
AI provider: local/remote/off
Web tools: off/scoped/on
Agent scope: <repository/space>
```

Before sending repository content to a remote model, Fehrest should expose the effective scope and provider.

---

## 33. Recovery UX

The canonical engine may be sophisticated; recovery must be humane.

If corruption or incomplete persistence is detected:

```text
stop unsafe mutation
explain what is known
preserve forensic bytes
show last known safe state
show recovery options permitted by policy
never pretend repair was lossless when it was not
```

---

## 34. What the user should never have to understand

Ordinary users should not need to reason about:

```text
hash chains
CRDT algorithms
vector indexes
embedding providers
graph databases
RRF
MCP JSON-RPC
WebMCP registration
canonical event serialization
```

These mechanisms support the experience; they are not the product vocabulary.

---

## 35. North-star user journeys

### Journey A — Individual knowledge

```text
capture thought
-> write/link notes
-> search or graph later
-> ask local LLM
-> get answer with note citations
-> convert useful result into a durable note
```

### Journey B — Research

```text
ask Fehrest to research topic
-> scoped web/WebMCP acquisition
-> sources captured with provenance
-> notes/claims linked
-> graph exposes relationships
-> AI drafts synthesis with citations
-> human reviews
```

### Journey C — Team decision

```text
channel topic discussion
-> evidence attached
-> AI proposes decision
-> Memory Proposal reviewed
-> decision becomes canonical
-> linked tasks execute
-> future users/agents can ask why
```

### Journey D — Developer + GitHub + IDE

```text
open GitHub-backed project in IDE
-> Fehrest link discovered
-> agent requests context
-> receives scoped project memory
-> works in repository
-> GitHub events + trajectory recorded as evidence
-> durable outcome proposed to Fehrest memory
```

### Journey E — New employee/agent

```text
join project
-> ask "what do I need to know?"
-> Fehrest compiles bounded context
-> user/agent sees active decisions, constraints, prior failures and sources
-> can drill into receipts/history
```

---

## 36. Product test

The UX succeeds only if a user can answer all of these positively:

```text
Can I capture something instantly?
Can I find it months later?
Can I understand how it relates to everything else?
Can I work offline?
Can I leave Fehrest with my data?
Can I collaborate without losing ownership?
Can my preferred AI use my memory?
Can I use no AI at all?
Can I use a local model?
Can I connect a remote model?
Can an agent act without receiving excessive authority?
Can I see why Fehrest believes something?
Can I recover history and mistakes?
Can I trust that web content cannot silently become authority?
```

If any major persona requires a separate Slack/Notion/Obsidian-style product for the core daily workflow, the V2 workspace thesis remains incomplete.
