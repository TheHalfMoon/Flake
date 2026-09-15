# Fehrest V2 — Linear Additive Product Execution Track

**Status:** PROGRAM ADDENDUM / NON-AUTHORIZING  
**Date:** 2026-08-31  
**Canonical frontier:** `specs/CURRENT.md`  
**Canonical plan:** `docs/canonical/EXECUTION_MASTER_PLAN.md`

> This track adds Linear-class product planning and execution outcomes to the existing V2 program. It does not replace, narrow, or reorder the existing V2 scope by itself. It does not activate implementation while R1 remains open.

---

## 1. Additive rule

The existing V2 program remains responsible for:

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

This addendum strengthens P5 and introduces a cross-pillar **Linear-class Product Execution Track** spanning P5, P7, P9, P11, P12, P13, P14 and P15.

It does not delete or downgrade any existing pillar.

```text
LINEAR_TRACK=ADDITIVE
EXISTING_P1_TO_P15=RETAINED
MEMORY_REPOSITORY_CORE=RETAINED
```

---

## 2. Why an explicit track is required

The previous V2 program maps generic `Task` and `Project` semantics but does not explicitly prove coverage of the full modern Linear product surface.

A serious Linear replacement claim requires explicit ownership for:

```text
issues/work items
projects
milestones
initiatives
cycles
roadmaps
dependencies
triage
internal requests
customers/customer requests
custom views
insights/dashboards
code intelligence
diffs/reviews
agent planning
MCP context
coding sessions
environments
browser verification
scheduled/event-driven agent loops
API/webhooks
mobile review/delegation
enterprise controls
Linear-origin migration
```

These cannot remain implied by a generic `Tasks / Projects` bullet.

---

## 3. Planning model

This document uses temporary track IDs rather than renumbering the already-proposed Specs 002–022.

Exact future Spec Kit IDs and insertion points require the post-R1 V2 reconciliation gate.

```text
L-PX0  Linear Living Capability Registry and Coverage Contract
L-PX1  Product Work Model, Planning and Iteration Semantics
L-PX2  Triage, Requests and Customer Intelligence
L-PX3  Roadmaps, Views, Insights and Delivery Intelligence
L-PX4  Code Review and Agentic Delivery Sessions
L-PX5  Linear Migration and Replacement Proof
L-GA   Linear-Class Readiness Gate
```

The track consumes existing V2 foundations rather than duplicating them.

---

## 4. L-PX0 — Living capability registry

### Dominant outcome

Every material Linear product-development capability has an explicit Fehrest disposition and owner.

### Owns

```text
observed Linear capability registry
source/date evidence
capability-family classification
owner mapping
parity requirement
surpass requirement
migration requirement
mobile/API/agent/security dimensions
future Linear change review
```

### Required fields

```text
LINEAR_CAPABILITY_ID
LINEAR_SOURCE
OBSERVED_DATE
CAPABILITY_DESCRIPTION
USER_OUTCOME
FEHREST_DISPOSITION
FEHREST_OWNER
DEPENDENCIES
PARITY_TEST
SURPASS_TEST
MIGRATION_TEST
MOBILE_TEST
API_TEST
AGENT_TEST
SECURITY_TEST
STATUS
```

### Allowed dispositions

```text
PARITY_REQUIRED
OUTCOME_REQUIRED_DIFFERENT_IMPLEMENTATION
INTEGRATE_INSTEAD_OF_BUILD
DEFER_WITH_EXPLICIT_REASON
REJECT_WITH_EXPLICIT_REASON
```

### Completion rule

```text
UNMAPPED_MATERIAL_LINEAR_CAPABILITIES=0
MATERIAL_LINEAR_CAPABILITIES_WITHOUT_OWNER=0
```

This is a planning/coverage gate, not implementation authorization.

---

## 5. L-PX1 — Product Work Model, Planning and Iteration Semantics

### Dominant outcome

A product/engineering team can represent and operate its planning hierarchy and execution workflow in Fehrest without losing the capabilities it expects from Linear.

### Depends on

```text
002 canonical core convergence
006 temporal memory semantics where decisions/memory are referenced
010 workspace canonical object/open-format foundation
```

### Owns product-execution semantics for

```text
WorkItem / Issue-class object
parent/sub-work hierarchy
relations/dependencies
blocker semantics
priority
estimate
workflow/status model
team ownership
assignee
labels
due dates
recurrence
templates
bulk mutation semantics
Project execution semantics
Milestone
Initiative / strategic grouping
Cycle / bounded planning iteration
carry-over rules
roadmap membership
release linkage where included
planning history
```

### Shared ownership rule

This track must not redefine:

```text
Memory lifecycle                -> 006
canonical repository integrity  -> 002
base workspace identity         -> 010
organization principals         -> 018
```

### Required baseline journeys

```text
create -> prioritize -> assign -> plan -> execute -> complete work
break work into parent/sub-work
block/unblock dependent work
place work into project/milestone/cycle
connect projects to strategic initiative
carry unfinished work according to declared iteration rules
preserve history after workflow changes
```

### Fehrest superiority requirements

Every work object should be able to link to:

```text
source evidence
customer/request evidence
decisions
active and historical memory
failed attempts
agent trajectories
completion evidence
```

without those derived links silently becoming authority.

---

## 6. L-PX2 — Triage, Requests and Customer Intelligence

### Dominant outcome

Incoming work from employees, customers, support systems and agents can enter one controlled intake flow, be triaged, deduplicated, routed, prioritized and connected to product decisions.

### Depends on

```text
L-PX1 work item semantics
014 external evidence/connectors where external acquisition is used
018 organization identity/policy for shared enterprise operation
021 connector/automation platform where generic connector runtime is required
```

### Owns

```text
Triage inbox/state
accept/reject/defer/reroute/merge outcomes
triage responsibility
rotation/schedule integration contract
triage rules
required-field exit policies
duplicate/related candidate workflow
Request / Ask-class object
requester communication state
email/chat/form intake mapping
Customer identity/reference model
Customer Request
customer attributes
customer impact relations
important/urgent request markers
support-system linkage
request -> work trace
```

### Required trace

```text
external/internal evidence
-> request
-> triage decision
-> accepted work
-> project/initiative
-> decision
-> implementation
-> verification
-> shipped result
```

The original request/evidence remains inspectable.

---

## 7. L-PX3 — Roadmaps, Views, Insights and Delivery Intelligence

### Dominant outcome

Teams can plan and understand execution across work items, projects and initiatives using saved operational views, roadmaps and inspectable analytics.

### Depends on

```text
003 deterministic retrieval/index
010 workspace objects
012 Search/Graph/Bases UX
L-PX1 product work model
L-PX2 customer intelligence where customer-impact analytics are shown
```

### Owns product-execution UX/analytics for

```text
issue/work views
project views
initiative views
saved/custom views
filters
sorting
ordering
grouping
list/board/timeline layouts
roadmaps
cross-project dependencies
milestone visualization
cycle planning views
project updates
project health
delivery analytics
cycle-time/throughput-style metrics
resource allocation views
bug-fix speed
priority consistency
estimate accuracy
customer impact views
dashboards
operational reporting
forecast/predictive completion only behind derived-evidence rules
```

### Authority rule

```text
MEASURED_EVENT_DATA = EVIDENCE
DERIVED_METRIC = DERIVED
AI_FORECAST = DERIVED
FORECAST != CANONICAL_COMMITMENT
```

Metric definitions must be inspectable and reproducible where practical.

---

## 8. L-PX4 — Code Review and Agentic Delivery Sessions

### Dominant outcome

A product/engineering team can move from scoped work to delegated implementation, verification, review and ship while Fehrest preserves the work context, agent authority and evidence trail.

### Depends on

```text
007 Universal Context and Memory Gateway
008 GitHub Link and IDE Discovery
013 AI Provider Runtime where model orchestration is used
018 organization policy for shared enterprise operation
021 extension/connector platform where execution providers are adapters
L-PX1 work item semantics
```

### Owns the Fehrest product contract for

```text
code-aware work context
PR/change linkage
native diff presentation
review discussions
CI/check presentation
forge synchronization
review state
agent execution session
session steering/follow-up
provider/model identity
bounded context receipt
execution environment profile
runtime/toolchain requirements
prepare/setup contract
environment configuration references
repository-specific guidance
provider-neutral coding-agent adapter
PR/change result
verification artifact attachment
browser verification result where applicable
screenshots/recordings as evidence
review-feedback -> agent iteration
merge/ship handoff
execution cost/usage records where available
```

### Provider boundary

Fehrest should not require one coding runtime.

Candidate future providers may include:

```text
Codex
Claude Code
OpenHands
Daytona
E2B
OpenSandbox
other authorized execution providers
```

The provider performs execution; Fehrest owns the user-facing work/session/context/evidence contract and authorization chokepoint.

### Hard rules

```text
EXECUTION_PROVIDER != AUTHORITY
MODEL != AUTHORITY
RETRIEVED_CONTENT != GRANT
SESSION_CONTEXT_MUST_BE_RECEIPTED=YES
VERIFY_BEFORE_COMPLETION_WHERE_REQUIRED=YES
```

---

## 9. Loops / recurring agent automation ownership

Linear-class Loops outcomes are mandatory, but should not create a duplicate automation platform.

Primary ownership remains in existing proposed Spec 021, strengthened by this track.

### Spec 021 must cover

```text
natural-language automation definition UX where authorized
schedule triggers
event triggers
workspace/team scope
shared visibility
enable/disable
inspectable run history
input/context receipts
action/effect receipts
failure state
cost/usage where applicable
triage/research/dispatch workflows
meeting/incident follow-up workflows
project/spec maintenance workflows
```

L-PX4 may consume these automations for coding-agent dispatch, but does not own the generic automation runtime.

---

## 10. API/webhook ownership

Linear-class API completeness is a cross-program requirement, not a separate weaker authority path.

Existing owning surfaces should expose stable API contracts as they mature:

```text
007 generic gateway/API authorization chokepoint
008 GitHub integration contracts
010 workspace objects
018 organization/admin contracts
021 extensions/connectors/webhooks
L-PX1 work semantics
L-PX2 intake/customer semantics
L-PX3 analytics query contracts
L-PX4 delivery/review session contracts
```

Read/write API behavior must enforce the same authorization semantics as native clients.

---

## 11. Mobile ownership

Existing proposed Spec 020 remains the mobile client owner.

Its acceptance scope is strengthened to include Linear-class work outcomes:

```text
work item create/update
assignment/status/priority changes
comments/mentions
project/cycle visibility
notifications
triage actions where authorized
customer/request context read
coding-session status
native diff review
line-specific review feedback
agent steering
verification artifact review
```

This is additive to Fehrest's broader mobile capture/search/Ask/Memory Proposal scope.

---

## 12. Migration ownership

Existing proposed Spec 015 remains the import/migration owner.

It must add Linear as a first-class migration source before any `REPLACES_LINEAR` claim.

### Required migration dimensions

```text
content fidelity
identity fidelity
workflow/status fidelity
project/initiative/milestone/cycle fidelity
relation/dependency fidelity
comment fidelity
attachment fidelity
customer/request relationship fidelity
GitHub/PR reference fidelity
timestamp fidelity
history fidelity where source APIs/exports expose it
unsupported construct reporting
repeatability/idempotency
rollback/remove imported batch where safe
```

### Hard rule

```text
SILENTLY_DROPPED_CRITICAL_LINEAR_FIELDS=0
```

---

## 13. L-PX5 — Linear Migration and Replacement Proof

### Dominant outcome

Demonstrate with representative real-world-style workspaces that Fehrest can absorb and run Linear-class product-development workflows without requiring Linear for the tested workload.

### Entry

All critical capability owners for the tested profile are complete and verified.

### Required test profiles

At least:

```text
small software startup
multi-team product organization
support-heavy product team
agent-heavy engineering workflow
mobile review/delegation workflow
```

### Required end-to-end journeys

```text
customer request
-> triage
-> issue/work item
-> project
-> initiative
-> cycle
-> agent/human implementation
-> review
-> verification
-> completion
-> durable learning/memory proposal
```

and:

```text
internal request
-> triage
-> owner
-> work
-> completion
-> requester-visible outcome
```

and:

```text
incoming bug
-> triage
-> duplicate/related decision
-> priority
-> coding session
-> browser verification
-> review
-> ship
```

### Compare

```text
workflow completeness
interaction count
time to outcome
keyboard efficiency
context-switch count
information loss
migration fidelity
mobile availability
API availability
agent success rate
human review burden
provenance quality
recovery/exportability
```

---

## 14. L-GA — Linear-Class Readiness Gate

Fehrest may claim Linear-class replacement readiness only when the scoped profile satisfies all required evidence.

Candidate future gate:

```text
MATERIAL_LINEAR_CAPABILITIES_MAPPED=100_PERCENT
CRITICAL_LINEAR_CAPABILITIES_WITHOUT_OWNER=0
CRITICAL_LINEAR_PARITY_GAPS=0
UNTESTED_CRITICAL_LINEAR_WORKFLOWS=0
BLOCKING_LINEAR_MIGRATION_GAPS=0
BLOCKING_LINEAR_MOBILE_GAPS=0
BLOCKING_LINEAR_API_GAPS=0
BLOCKING_LINEAR_AGENT_DELIVERY_GAPS=0
BLOCKING_LINEAR_ENTERPRISE_GAPS=0
```

This does not require copying every cosmetic implementation detail. It requires full coverage of material user outcomes for the claimed replacement profile.

---

## 15. Fehrest supremacy tests remain separate

Passing Linear parity is not evidence that Fehrest's defining thesis is superior.

Separate tests should evaluate:

```text
Does evidence-backed context improve planning/execution?
Do fresh humans/agents make fewer stale-premise mistakes?
Does failed-attempt memory reduce repeated work?
Do context receipts improve audit/review quality?
Does customer evidence -> decision -> work trace improve prioritization?
Does temporal truth improve handoff and incident/product understanding?
Can teams recover/exit without proprietary cloud dependence?
Can multiple agent/provider ecosystems use the same trusted memory?
```

The product target is:

```text
LINEAR_PARITY
+
FEHREST_MEMORY/TRUST/OWNERSHIP_SUPREMACY
```

---

## 16. Relationship to existing Spec 002–022 proposal

Nothing in this addendum replaces the existing proposed sequence.

The post-R1 V2 reconciliation must insert or merge the L-PX responsibilities without semantic duplication.

Expected ownership relationships:

| Existing proposed spec | Linear-track effect |
|---|---|
| 002 Canonical Core | unchanged prerequisite |
| 003 Retrieval | consumed by work search/views |
| 004/005 Graph | optional derived intelligence remains independent |
| 006 Temporal Memory | work/decision/customer learning links consume it |
| 007 Gateway | API/agent context authorization base |
| 008 GitHub | code/review integration base |
| 009 Vertical Proof | unchanged proof gate before broad expansion unless later governance changes order |
| 010 Workspace Objects | base identities; L-PX1 owns product-execution semantics |
| 011 Notes/Docs | retained; not displaced by Linear work UX |
| 012 Search/Bases | consumed by L-PX3 work views |
| 013 AI Runtime | consumed by agent assistance |
| 014 External Evidence | consumed by request/customer intake |
| 015 Import Lab | add Linear migration source |
| 016/017 Collaboration | shared work depends on proven collaboration |
| 018 Organization | team/customer/admin security prerequisite |
| 019 Team Communication | retained; work and communication interoperate |
| 020 Mobile | add Linear-class mobile work/review parity |
| 021 Extensions/Automation | add Loops-class automation requirements |
| 022 Hub | retained; not displaced by product-execution track |

---

## 17. Current program state

```text
LINEAR_ADDITIVE_TRACK=PREPARED
LINEAR_TRACK_CANONICAL=NO
EXISTING_V2_SCOPE_REPLACED=NO
EXISTING_SPEC_002_TO_022_PROPOSAL_RETAINED=YES
EXACT_TRACK_SPEC_IDS=DEFER_TO_POST_R1_RECONCILIATION
IMPLEMENTATION_AUTHORIZED=NO
R1_CHANGED=NO
CURRENT_CHANGED=NO
SPEC_002_CHANGED=NO
```
