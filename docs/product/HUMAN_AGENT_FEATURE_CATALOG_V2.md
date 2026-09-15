# Fehrest Human + Agent Feature Catalog v2

**Status:** PRODUCT CATALOG / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Purpose:** enumerate the complete long-term product surface without implying implementation authorization.

> This catalog is a scope map, not a promise that every capability ships in one release. Each capability still requires the proper Spec Kit, benchmark, security, provenance and authorization gates.

---

## 1. Canonical product pillars

```text
P1  Memory Repository
P2  Notes / Knowledge Workspace
P3  Search / Ask / Graph
P4  Team Communication
P5  Work / Projects / Decisions
P6  Human + Agent Memory
P7  AI Provider Layer
P8  Web / External Evidence
P9  GitHub / IDE Integration
P10 Collaboration / Sync
P11 Organization / Enterprise
P12 Extensions / Automation
P13 Import / Export / Portability
P14 Mobile / Capture
P15 Trust / Security / Provenance
```

---

## 2. P1 — Memory Repository

Defining capabilities:

- local-first repository creation;
- stable repository identity;
- stable object identity independent of paths;
- open canonical formats where human-owned content requires them;
- version/schema metadata;
- event/history model;
- durable provenance;
- object lifecycle;
- temporal truth;
- supersession/retraction;
- canonical vs derived separation;
- repository-level permissions/scopes;
- backup/recovery;
- export without Fehrest dependency;
- optional hosted/synced replicas;
- repository linking between personal/team/project scopes;
- public/private repositories in a future Hub.

Object families may include:

```text
Note
Document
Source
Claim
Decision
Task
Project
Space
Topic
Message
Meeting
Person
Agent
Memory
Skill
Procedure
Artifact
Trajectory
Incident
Experiment
RepositoryLink
```

---

## 3. P2 — Notes and knowledge workspace

### Writing

- Markdown-first source;
- rich text editing without destructive proprietary conversion;
- slash commands;
- headings;
- lists/checklists;
- tables;
- code blocks;
- math;
- diagrams;
- callouts;
- footnotes;
- citations;
- embeds;
- transclusion;
- block references;
- comments;
- suggestion mode;
- version compare;
- restore.

### Organization

- folders where users want them;
- tags;
- properties/frontmatter;
- aliases;
- backlinks;
- outgoing links;
- favorites/bookmarks;
- daily notes;
- templates;
- saved searches;
- collections;
- smart folders derived from queries;
- inbox;
- archive;
- trash with recoverability.

### Reading

- preview/read mode;
- table of contents;
- outline;
- side-by-side panes;
- tabs;
- focus mode;
- backlinks panel;
- source/provenance panel;
- history panel;
- linked graph panel;
- citation hover previews.

---

## 4. P3 — Search / Ask / Graph

### Search

- instant lexical search;
- phrase search;
- fuzzy typo tolerance where measurable;
- metadata filters;
- property filters;
- date/time filters;
- author/person filters;
- agent filters;
- type filters;
- space/project filters;
- provenance/source filters;
- temporal `as-of` filters;
- saved searches;
- search history;
- keyboard navigation;
- result snippets;
- exact-match highlighting;
- explain-why-matched;
- offline operation.

### Optional derived retrieval

- semantic/vector search only when benchmark-authorized;
- hybrid retrieval;
- graph-assisted retrieval only when benchmark-authorized;
- reranking only when justified;
- query-conditioned context selection;
- retrieval traces.

### Ask

Questions should support:

```text
what?
where?
when?
who?
why?
what changed?
what contradicts this?
what is stale?
what did we know at date X?
what should I know for task Y?
```

Answers can show:

- citations;
- source cards;
- confidence/uncertainty where meaningful;
- omitted/insufficient-evidence state;
- selected-context receipt;
- `AI OFF` fallback to search/navigation.

### Graph

- global graph;
- local graph;
- project graph;
- search-filtered graph;
- provenance graph;
- temporal graph;
- decision graph;
- person/agent graph;
- memory graph;
- configurable node types;
- configurable edge types;
- depth expansion;
- path between nodes;
- clusters as derived navigation;
- saved graph views;
- graph/list/table interoperability;
- graph filter synchronized with search;
- explicit vs inferred edge distinction.

---

## 5. P4 — Team communication

### Channels and topics

- open channels;
- private channels;
- announcement channels;
- topic-first organization;
- threaded replies where useful;
- topic move/merge/rename;
- pins;
- bookmarks;
- files;
- reactions;
- mentions;
- typing/presence;
- read state;
- message edit/history;
- soft delete/audit policy;
- search;
- topic summary;
- topic crystallization.

### Direct messaging

- 1:1 DMs;
- group DMs;
- files;
- reactions;
- search;
- privacy/retention boundaries distinct from shared memory.

### Meetings / huddles

Fehrest should integrate rather than build a media infrastructure platform.

Future features:

- voice/video provider integration;
- meeting notes;
- transcript import where permitted;
- speaker attribution;
- agenda;
- action items;
- linked decisions;
- linked tasks;
- meeting memory proposals;
- searchable recordings/transcripts under explicit retention policy.

---

## 6. P5 — Work / Projects / Decisions

### Projects

- project overview;
- goals;
- status;
- members;
- docs;
- tasks;
- decisions;
- sources;
- files;
- channels/topics;
- graph;
- agents;
- activity;
- linked GitHub repositories where applicable.

### Tasks

- title/description;
- assignee;
- agent assignee;
- state;
- priority;
- due date;
- start date;
- subtask;
- dependency;
- blockers;
- project/space;
- labels;
- comments;
- attachments;
- linked decision;
- linked source;
- context request;
- agent run history;
- completion evidence.

### Views

- list;
- board;
- table;
- calendar;
- timeline;
- dependency view;
- personal My Work.

### Decisions

- decision statement;
- status;
- rationale;
- alternatives;
- owner;
- participants;
- sources;
- evidence;
- affected scope;
- effective date;
- expiry/review date;
- supersedes;
- contradicts;
- linked tasks/projects;
- change history.

---

## 7. P6 — Human + Agent Memory

### Memory types

Do not force a single taxonomy, but support at least conceptual distinctions such as:

```text
semantic facts/claims
procedural knowledge
experience/episodic history
preferences
constraints
decisions
project state
lessons/failures
```

### Lifecycle

Possible lifecycle states:

```text
OBSERVED
CANDIDATE
PENDING_REVIEW
ACTIVE
SUPERSEDED
RETRACTED
REJECTED
```

Exact canonical states require later specification.

### Memory Proposal

- proposed add;
- proposed edit;
- proposed supersession;
- proposed retraction;
- evidence;
- author/agent;
- diff;
- contradiction detection;
- review;
- approval policy;
- audit history.

### Memory CI

- missing source;
- unsupported claim;
- stale source;
- contradiction;
- duplicate;
- broken relationship;
- suspected secret;
- sensitive-data policy;
- unauthorized scope;
- agent authority violation;
- malformed provenance;
- lifecycle inconsistency.

### Memory viewer

- current memory;
- history;
- evidence;
- related notes/messages;
- graph;
- as-of timeline;
- who/what generated it;
- who confirmed it;
- affected agents/scopes.

---

## 8. P7 — AI provider layer

### Modes

```text
AI OFF
LOCAL
REMOTE
CUSTOM
```

### Local candidates

- Ollama;
- LM Studio;
- llama.cpp server;
- compatible self-hosted APIs such as vLLM where appropriate;
- future OS-native runtimes through a provider interface.

### Remote/provider candidates

Provider integration should be modular and explicit rather than making one vendor canonical.

Capabilities may include:

- chat/responses;
- tool calling;
- structured output;
- embeddings only when separately authorized;
- multimodal input;
- model capability discovery;
- token/context accounting;
- streaming;
- rate/cost metadata;
- retry/failure classification.

### Model UX

- provider picker;
- model picker;
- local/remote badge;
- privacy scope preview;
- context budget;
- model capabilities;
- tool permissions;
- default model per repository/space/task;
- temporary override;
- no mandatory AI.

### AI surfaces

- Ask Fehrest;
- inline writing;
- research;
- meeting assistant;
- task assistant;
- project briefing;
- contradiction review;
- proposal drafting;
- context compilation;
- agent handoff.

---

## 9. P8 — Web and external evidence

### Acquisition

- URL capture;
- browser clipper;
- manual file import;
- Git/GitHub ingestion;
- structured connectors;
- WebMCP tool usage where available and authorized;
- search provider adapters;
- parser adapters;
- OCR/document conversion where justified.

### Web research

- query planning;
- source discovery;
- source selection;
- capture exact URLs;
- capture timestamps;
- source snapshot/hash where appropriate;
- source trust labels;
- citations;
- compare sources;
- detect stale sources;
- re-check source;
- preserve failed/unavailable acquisition.

### WebMCP

- discover page-exposed tools;
- inspect tool schema;
- classify read vs write/action tool;
- enforce origin/domain policy;
- enforce user/agent scope;
- confirmation for consequential actions;
- treat tool descriptions/content as untrusted;
- preserve tool invocation receipt;
- no tool may mint Fehrest authorization.

---

## 10. P9 — GitHub / IDE integration

### Repository binding

- link Fehrest Memory Repository to one or more GitHub repositories;
- repo-local discovery metadata;
- GitHub App candidate;
- organization mapping;
- repository mapping;
- issue/PR/discussion references;
- webhook/event intake;
- provenance-bound GitHub events.

### IDE / coding agent

IDE-independent access via future combinations of:

```text
CLI
SDK
MCP
local daemon/API
ACP adapter
repository discovery file
```

### Developer workflows

- project briefing;
- issue context;
- PR rationale context;
- architecture decision lookup;
- failed-attempt lookup;
- incident history;
- related source lookup;
- previous agent trajectory lookup;
- Memory Proposal after work;
- GitHub event to Fehrest evidence;
- Fehrest decision to GitHub reference.

### Supported clients should be open-ended

Fehrest should not require special-case product coupling to every IDE.

Target compatibility principle:

> If a client can call a stable Fehrest CLI/API/MCP/SDK surface and authenticate, it can use Fehrest memory.

---

## 11. P10 — Collaboration / sync

Future requirements:

- offline editing;
- local-first persistence;
- multi-device;
- multi-user;
- real-time collaboration;
- deterministic convergence;
- conflict visibility;
- permissions;
- revocation;
- guest sharing;
- encrypted transport;
- optional E2EE where compatible with product/security requirements;
- sync health/status;
- self-hosted sync;
- managed sync;
- backup;
- recovery;
- migration.

Candidate CRDT/sync donors must be benchmarked rather than preselected.

---

## 12. P11 — Organization / enterprise

- organization creation;
- workspaces/spaces;
- groups;
- roles;
- guest roles;
- external collaborator boundaries;
- admin console;
- audit;
- retention;
- legal/export workflows where appropriate;
- policy controls;
- self-hosting;
- managed hosting;
- backup policy;
- recovery policy;
- identity provider integration candidates;
- SSO candidate;
- SCIM candidate;
- domain verification candidate;
- data residency architecture candidate;
- key-management architecture candidate;
- air-gapped deployment candidate.

These are future enterprise gates and must not silently weaken local-first ownership.

---

## 13. P12 — Extensions / automation

### Extension types

- importers;
- exporters;
- views;
- parsers;
- connectors;
- AI providers;
- model providers;
- search providers;
- WebMCP/browser providers;
- automations;
- agent adapters;
- templates;
- skills.

### Extension security

Every extension should eventually declare capabilities such as:

```text
repository:read
repository:write
space:<id>:read
space:<id>:write
network:<domains>
process:none
secrets:<named>
webmcp:read
webmcp:act
```

No extension receives unrestricted vault/network/process access merely because it is installed.

### Automation triggers

- schedule;
- object changed;
- topic idle;
- task status;
- GitHub event;
- source stale;
- meeting ended;
- agent completed;
- proposal submitted;
- review approved.

---

## 14. P13 — Import / export / portability

### Import

- Obsidian;
- Markdown;
- Notion;
- Slack;
- Zulip;
- GitHub;
- AFFiNE/AppFlowy/Anytype where supported;
- JSON;
- CSV;
- HTML;
- PDFs/documents;
- bookmarks;
- browser history only under explicit user choice;
- other note managers through adapters.

### Export

- Markdown/files;
- JSON/structured canonical export;
- CSV for tabular objects;
- graph relationships;
- provenance manifest;
- conversation archive;
- tasks/decisions;
- backup bundle;
- agent-readable context package.

### Migration quality

- dry run;
- mapping preview;
- warnings;
- duplicate detection;
- unsupported-field report;
- provenance record;
- rollback/remove imported batch where safe;
- preservation of original export as evidence when desired.

---

## 15. P14 — Mobile / capture

- iOS/Android candidate clients;
- quick text capture;
- voice capture;
- camera scan;
- share sheet;
- offline notes;
- offline search subset;
- notifications;
- topic replies;
- task updates;
- proposal review;
- Ask Fehrest;
- local/remote AI selection where practical;
- biometric app lock candidate;
- sync status;
- widgets/quick actions candidate.

---

## 16. P15 — Trust / security / provenance

User-facing trust features:

- local/remote indicators;
- data location;
- sharing scope;
- AI provider scope;
- agent grant viewer;
- tool/web permission viewer;
- source provenance;
- change history;
- proposal review;
- recovery status;
- audit log;
- secret scanning boundaries;
- sensitive-data policy;
- content trust labels;
- external source freshness;
- action confirmation;
- revocation.

Core invariants remain stronger than UX convenience.

---

## 17. Human feature completeness test

A mature Fehrest should let an individual perform the core daily workflows that otherwise require a combination of:

```text
Obsidian
Notion
personal task manager
web clipper
AI chat
```

without sacrificing local ownership or requiring AI.

A mature team Fehrest should let a team perform the core daily workflows that otherwise require a combination of:

```text
Slack/Zulip
Notion/wiki
project/task tool
AI assistant
institutional-memory search
```

while keeping GitHub or another specialized forge where specialization is valuable.

---

## 18. Agent feature completeness test

An arbitrary agent should eventually be able to:

```text
discover Fehrest
authenticate
request scoped context
search permitted memory
inspect provenance
ask temporal questions
use approved web tools
read/write permitted working objects
submit Memory Proposals
attach evidence
return trajectories
handoff to another agent
```

without:

```text
receiving arbitrary filesystem authority
receiving the whole repository by default
minting permissions from retrieved content
automatically turning inference into canonical memory
requiring a specific model vendor
```

---

## 19. Explicit non-goal warning

This catalog must not become a justification for building all capabilities simultaneously.

The governing principle remains:

```text
CATEGORY VISION = BROAD
ACTIVE SPEC = NARROW
```

Every implementation slice must still prove that it earns its place.
