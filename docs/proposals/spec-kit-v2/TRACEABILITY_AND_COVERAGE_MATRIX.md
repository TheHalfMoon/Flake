# Fehrest V2 Traceability and Coverage Matrix

**Status:** PROGRAM PROPOSAL / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Purpose:** prove that the V2 product pillars and critical gaps have an explicit owning future Spec Kit, gate, or defer/reject decision.

---

## 1. Coverage rule

A broad product vision is not complete merely because every feature is mentioned.

Program completeness requires:

```text
VISION CAPABILITY
-> OWNING SPEC OR GATE
-> ENTRY DEPENDENCIES
-> MEASURABLE OUTCOME
-> FAILURE/DEFER ROUTE
```

No critical capability may live only in a feature catalog with no planned owner.

---

# 2. Product pillar coverage

## P1 — Memory Repository

| Capability | Owner |
|---|---|
| Stable repository/vault identity | 002 |
| Canonical version envelope | 002 |
| Crash-safe canonical writes | 002 |
| Event/history integrity | 002 |
| Durable memory lifecycle | 006 |
| Context/receipt access | 007 |
| Workspace open-format object layer | 010 |
| Multi-device/shared replicas | 017 conditional |
| Hosted/public repository layer | 022 |

**Coverage:** COMPLETE IN PROGRAM MAP  
**Current authorization:** NONE beyond live R1 planning rules

---

## P2 — Notes / Knowledge Workspace

| Capability | Owner |
|---|---|
| Note/Document canonical mapping | 010 |
| Open-format rules | 010 |
| Markdown/editor UX | 011 |
| Capture/Inbox/Daily Notes/Templates | 011 |
| Wikilinks/backlinks presentation | 011 |
| Search/graph exploration | 012 |
| Structured bases/views | 012 |
| AI writing | 013 |
| Import from existing notes | 015 |
| Mobile capture | 020 |

**Coverage:** COMPLETE IN PROGRAM MAP

---

## P3 — Search / Ask / Graph

| Capability | Owner |
|---|---|
| Local lexical/structured retrieval | 003 |
| Incremental/fresh equivalence | 003 |
| Graph intelligence experiment | 004 |
| Production graph provider | 005 conditional |
| Context selection/trace | 007 |
| Search UX | 012 |
| Graph visualization | 012 |
| Search <-> Graph scope sync | 012 |
| Ask Fehrest model execution | 013 |
| Web-augmented research | 014 |

**Coverage:** COMPLETE  
**Conflict resolved:** graph visualization no longer depends on production graph intelligence.

---

## P4 — Team Communication

| Capability | Owner |
|---|---|
| Organization membership/policy prerequisite | 018 |
| Channels/topics | 019 |
| Threads/replies | 019 |
| DMs/group DMs | 019 |
| Reactions/mentions/read state | 019 |
| Presence/typing | 019 |
| Notification semantics | 019 |
| Mobile communication client | 020 |
| Knowledge crystallization | 019 using 006 Memory Proposal semantics |

**Coverage:** COMPLETE  
**Order correction:** organization/authorization foundation precedes communication surfaces.

---

## P5 — Work / Projects / Decisions

| Capability | Owner |
|---|---|
| Space/Project semantics | 010 |
| Task semantics | 010 |
| Decision workspace object | 010, memory/provenance semantics from 006 |
| Personal project/task UX | 011/012 |
| Team shared project/task/decision UX | 019 |
| GitHub-linked engineering project context | 008 |
| Agent task context request | 007 |

**Coverage:** COMPLETE WITH SHARED-SEMANTIC RULE  
**Review point:** 010 must not redefine 006 memory lifecycle.

---

## P6 — Human + Agent Memory

| Capability | Owner |
|---|---|
| Memory schema/lifecycle | 006 |
| Temporal current/as-of resolution | 006 |
| Memory Proposal | 006 |
| Memory CI | 006 |
| Scoped agent context | 007 |
| Agent proposal/return evidence | 007 + 006 |
| Agent continuation proof | 009 |
| Human continuation proof | 009 |
| Team crystallization UX | 019 |

**Coverage:** COMPLETE

---

## P7 — AI Provider Layer

| Capability | Owner |
|---|---|
| AI OFF contract | program invariant + 013 |
| Local model provider interface | 013 |
| Remote/custom provider interface | 013 |
| Provider capability probe | 013 |
| Model picker/privacy state | 013 |
| Ask Fehrest | 013 |
| Inline AI edits | 013 |
| Model-generated Memory Proposal | 013 consuming 006 |

**Coverage:** COMPLETE

---

## P8 — Web / External Evidence

| Capability | Owner |
|---|---|
| External evidence acquisition | 014 |
| Web authorization | 014 using 007 grants |
| WebMCP provider | 014 |
| Browser/search/HTTP provider boundaries | 014 |
| Origin/domain policy | 014 |
| Prompt-injection boundary | 014 + security invariant |
| Web receipts | 014 |
| Source freshness/recheck | 014 |
| Import external exports/files | 015 |

**Coverage:** COMPLETE

---

## P9 — GitHub / IDE Integration

| Capability | Owner |
|---|---|
| Fehrest <-> GitHub binding | 008 |
| Repo-local discovery metadata | 008 |
| GitHub App candidate | 008 |
| Issue/PR/discussion/event provenance | 008 |
| Arbitrary IDE/agent access | 007 generic + 008 discovery |
| GitHub-linked continuation benchmark | 009 |

**Coverage:** COMPLETE  
**Key invariant:** GitHub identifies context; Fehrest grants authority.

---

## P10 — Collaboration / Sync

| Capability | Owner |
|---|---|
| CRDT/sync candidate experiment | 016 |
| Multi-writer production semantics | 017 conditional |
| Replica/device identity | 017 |
| Offline convergence/conflict | 017 |
| Revocation under partition | 017 + 018 |
| Sync health/status | 017 |
| Self-hosted/managed sync boundary | 017 |
| Team shared usage | 019 |
| Mobile sync consumer | 020 |

**Coverage:** COMPLETE WITH EXPERIMENT GATE

---

## P11 — Organization / Enterprise

| Capability | Owner |
|---|---|
| Organization identity | 018 |
| Members/groups/roles | 018 |
| Guests/external collaborators | 018 |
| Admin/audit | 018 |
| Retention policy primitives | 018 |
| Device/session admin | 018 |
| Self-hosted/managed policy modes | 018 |
| SSO/SCIM/data-residency/key-management candidates | 018 research/plan where justified |
| Team communication surfaces | 019 |
| Hub organization hosting | 022 |

**Coverage:** COMPLETE AT PROGRAM LEVEL  
**Important:** enterprise claims remain candidates until their own requirements/benchmarks exist.

---

## P12 — Extensions / Automation

| Capability | Owner |
|---|---|
| Extension manifest | 021 |
| Capability grants | 021 consuming 007/018 authorization |
| Connectors/importers/exporters/views | 021 |
| Automations/triggers/actions | 021 |
| Provider/agent adapters | 021 |
| Extension lifecycle/update policy | 021 |

**Coverage:** COMPLETE

---

## P13 — Import / Export / Portability

| Capability | Owner |
|---|---|
| Open canonical format foundation | 010 |
| Import adapter/mapping contract | 015 |
| Markdown/Obsidian import | 015 |
| Notion/Slack/Zulip/etc import candidates | 015 |
| Batch provenance/rollback | 015 |
| Structured export/backup mapping | 010 + 015 owning export contract decision |
| Hub copy/fork provenance | 022 |

**Coverage:** COMPLETE WITH ONE REVIEW POINT  
**Review point:** final export contract owner must be singular during 010/015 planning.

---

## P14 — Mobile / Capture

| Capability | Owner |
|---|---|
| Mobile client | 020 |
| Quick text/voice/photo/share capture | 020 |
| Offline notes/search subset | 020 |
| Notifications | 020 consuming 019 semantics |
| Topic/task updates | 020 |
| Memory Proposal review | 020 consuming 006 |
| Sync status | 020 consuming 017 |

**Coverage:** COMPLETE

---

## P15 — Trust / Security / Provenance

This is cross-cutting and must not be left to one late security spec.

| Capability | Owner/gate |
|---|---|
| Core canonical integrity | 002 |
| Derived-state non-authority | 003/005 + program invariant |
| Memory lifecycle authority | 006 |
| Principal/grant/context authorization | 007 |
| GitHub trust boundary | 008 |
| Vertical adversarial proof | 009 |
| Open-format/migration integrity | 010/015 |
| AI provider privacy/tool boundary | 013 |
| Web/prompt-injection boundary | 014 |
| Collaboration/revocation/convergence | 016/017 |
| Organization/tenant policy | 018 |
| Team privacy | 019 |
| Extension capability security | 021 |
| Hub hosted/local ownership | 022 |

**Coverage:** COMPLETE AS CROSS-CUTTING OWNER MATRIX

---

# 3. Critical gap coverage from V2 gap review

| Gap | Program owner | Coverage result |
|---|---|---|
| G-V2-01 Universal product identity | V2-G0 / founder decision | COVERED |
| G-V2-02 GitHub repository binding | 008 | COVERED |
| G-V2-03 IDE-independent discovery | 008 + 007 | COVERED |
| G-V2-04 Obsidian-class notes | 010/011 | COVERED |
| G-V2-05 Structured collections/bases | 012 | COVERED |
| G-V2-06 Channels/topics/DMs | 018 prerequisite + 019 | COVERED |
| G-V2-07 Knowledge crystallization | 006 semantics + 019 UX | COVERED |
| G-V2-08 Multi-user local-first collaboration | 016/017 | COVERED |
| G-V2-09 Multi-device sync | 017 | COVERED |
| G-V2-10 Organization/guest/admin authorization | 018 | COVERED |
| G-V2-11 Agent-agnostic gateway | 007 | COVERED |
| G-V2-12 Memory Proposal | 006 | COVERED |
| G-V2-13 Memory CI | 006 | COVERED |
| G-V2-14 Import/migration | 015 | COVERED |
| G-V2-15 Unified search/Ask | 003/007/012/013 | COVERED WITH LAYERED OWNERSHIP |
| G-V2-16 Privacy/encryption model | 017/018/019 security gates | COVERED BUT DECISION DEFERRED |
| G-V2-17 Mobile | 020 | COVERED |
| G-V2-18 Extensions/connectors | 021 | COVERED |
| G-V2-19 Hub/network | 022 | COVERED |
| G-V2-20 Voice/video/huddles | 019 provider integration, NO-CORE media infra | COVERED/DEFERRED PROVIDER CHOICE |
| G-V2-21 Onboarding/templates | 011 + 015 import; team onboarding 019 | COVERED |
| G-V2-22 Team/company deployment | 017/018 | COVERED |
| G-V2-23 Memory benchmark refresh | 009 | COVERED |
| G-V2-24 Donor rights/provenance registry | program gate + per-spec research | COVERED |
| G-V2-25 R1 does not test V2 thesis | V2-G0 + 009 broader proof | COVERED HONESTLY |
| G-V2-26 Historical implementation/evidence recovery | G-PROV | RESOLVED: SOURCE + SEAL RECOVERED; ORIGINAL GIT HISTORY PUBLICATION TRACKED SEPARATELY |

---

# 4. User journey coverage

The program must eventually prove these end-to-end journeys, not merely individual features.

## J-01 — Personal local knowledge

```text
install/open
-> create local repository
-> write/import notes
-> search
-> graph explore
-> organize task/project
-> close app
-> reopen offline
-> recover same knowledge
```

Owners: `002,003,010,011,012,015`

## J-02 — Local AI assistant

```text
local repository
-> connect local LLM
-> ask question
-> receive evidence-backed answer
-> preview AI edit
-> accept/reject
-> no canonical promotion without authorized path
```

Owners: `006,007,012,013`

## J-03 — GitHub developer/IDE memory

```text
open GitHub repo
-> discover Fehrest link
-> authenticate
-> request task context
-> receive receipt
-> work in arbitrary IDE/agent
-> return evidence
-> submit Memory Proposal
```

Owners: `006,007,008,009`

## J-04 — Web research

```text
ask research task
-> authorize web scope
-> discover/acquire sources
-> preserve provenance
-> synthesize with citations
-> save source/note
-> optionally submit memory proposal
```

Owners: `007,013,014,006,010`

## J-05 — Team project

```text
create/join organization
-> enter shared space
-> channel/topic discussion
-> shared doc/task/decision
-> agent participates under grant
-> conversation creates reviewed memory proposal
-> teammate later searches/asks and sees evidence
```

Owners: `017,018,019,006,007,012,013`

## J-06 — Offline collaborative recovery

```text
two devices edit offline
-> reconnect
-> converge/conflict visibly
-> revoked actor cannot regain scope
-> restart/recovery remains correct
```

Owners: `016 experiment, 017 production, 018 policy`

## J-07 — Migration

```text
select Obsidian/Markdown source
-> dry run
-> preview mapping/warnings
-> import
-> verify links/properties/attachments
-> search/graph immediately useful
-> retain provenance
-> undo batch where supported
```

Owners: `010,012,015`

## J-08 — Mobile capture to team memory

```text
capture offline on phone
-> sync later
-> link to project/topic
-> agent/human can retrieve
-> proposal/review remains consistent
```

Owners: `017,018,019,020,006`

## J-09 — Exit Fehrest safely

```text
export repository
-> obtain human-readable/structured content + provenance
-> validate no proprietary hosted service is required to read core knowledge
```

Owners: `010/015`, later Hub `022` must preserve this property.

---

# 5. Success metric families by program tranche

Exact thresholds belong in each spec/benchmark, but every tranche must select measurable outcomes from the applicable family.

### Core

```text
canonical loss
recovery correctness
migration correctness
startup/open correctness
```

### Retrieval

```text
p50/p95 query latency
recall/ranking metrics where appropriate
incremental/fresh equivalence
rebuild time/space
```

### Memory/context

```text
continuation correctness
stale/contradiction handling
as-of correctness
authority violations
context token/byte cost
```

### Human workspace

```text
time-to-capture
time-to-find
primary journey completion
keyboard workflow success
offline success
import fidelity
```

### AI

```text
evidence grounding
unsupported claim rate
tool/provider failure handling
local/remote privacy correctness
cost/context usage
```

### Collaboration/team

```text
convergence
conflict correctness
revocation correctness
cross-tenant leakage = 0
message/search latency
project workflow completion
```

### Ecosystem/Hub

```text
capability escape = 0
extension isolation failures
portable repository fidelity
public/private boundary correctness
```

---

# 6. Remaining unresolved decisions

A complete program map may still contain explicitly unresolved decisions. These must not be guessed.

Current examples:

```text
historical Constitution/Architecture Freeze input              = RECOVERED_AND_RECONCILED_CONSERVATIVELY
GitHub original historical object graph publication           = OPEN_PROVENANCE_CLOSURE
exact post-R1 V2 authorization                                = NOT YET DECIDED
production graph mechanism                                    = OWNED BY 004/005 DECISION
exact Memory lifecycle states                                 = OWNED BY 006
exact open Note/Document metadata representation              = OWNED BY 010
editor implementation                                          = OWNED BY 011 RESEARCH/GATE
AI provider implementation set                                = OWNED BY 013
WebMCP API adoption shape                                     = OWNED BY 014 RESEARCH
CRDT/sync mechanism                                           = OWNED BY 016 DECISION
E2EE vs server-side search/compliance trade-off               = OWNED BY 017/018 SECURITY/PRODUCT DECISION
mobile implementation technology                              = OWNED BY 020
extension runtime/sandbox policy                              = OWNED BY 021
Hub hosting architecture                                      = OWNED BY 022
```

Explicit ownership of an unresolved decision is considered coverage. Guessing the answer is not. Recovered historical inputs are recorded as resolved evidence state, not as authorization to execute future specs.

---

# 7. Coverage verdict

```text
PRODUCT_PILLARS_WITH_OWNER=15/15
CRITICAL_GAPS_WITH_OWNER_OR_HARD_GATE=26/26
KNOWN_UNOWNED_CRITICAL_CAPABILITIES=0
HISTORICAL_IMPLEMENTATION_EVIDENCE_RECONCILED=YES
HISTORICAL_CONSTITUTION_ARCHITECTURE_RECONCILED=YES_CONSERVATIVELY
GITHUB_ORIGINAL_HISTORICAL_OBJECT_GRAPH_PUBLISHED=NO
KNOWN_EXECUTION_AUTHORIZATION_BLOCKERS_RESOLVED=NO
PROGRAM_CANONICAL=NO
IMPLEMENTATION_AUTHORIZED=NO
```

The program map is coverage-complete as a proposal. Historical implementation evidence and the Constitution/Architecture inputs are recovered/reconciled; original-history publication remains a separate provenance closure obligation. Execution remains unauthorized because the live R1 route and the required post-R1 founder/architecture decision are still open.