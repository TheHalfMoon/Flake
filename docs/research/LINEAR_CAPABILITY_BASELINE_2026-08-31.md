# Linear Capability Baseline for Fehrest V2

**Date:** 2026-08-31  
**Status:** RESEARCH / NON-AUTHORIZING  
**Purpose:** establish Linear as an additive mandatory benchmark inside the broader Fehrest V2 product program.  
**Execution effect:** NONE while R1 remains active.

> This document records observed Linear product capabilities and maps the resulting benchmark obligation. It does not authorize implementation, dependency adoption, code reuse, or changes to canonical sequencing.

---

## 1. Benchmark rule

Linear is added to the existing mandatory product benchmark set.

```text
Obsidian -> personal/local knowledge benchmark
Notion   -> docs/structured workspace benchmark
Slack    -> team communication benchmark
Zulip    -> durable topic benchmark
GitHub   -> code/review/integration benchmark
Linear   -> product planning/execution/triage/agentic delivery benchmark
Fehrest  -> canonical trusted human + agent memory substrate
```

The goal is not to reduce Fehrest to a Linear clone.

The goal is:

> No important Linear product-development workflow should force a target Fehrest team to keep Linear solely because Fehrest omitted that outcome.

---

## 2. Primary official sources reviewed

Observed capability sources include:

```text
https://linear.app/docs/initiatives
https://linear.app/docs/project-milestones
https://linear.app/docs/triage
https://linear.app/docs/linear-asks
https://linear.app/docs/customer-requests
https://linear.app/docs/custom-views
https://linear.app/docs/insights
https://linear.app/docs/api-and-webhooks
https://linear.app/docs/linear-agent
https://linear.app/docs/coding-sessions
https://linear.app/docs/diffs
https://linear.app/changelog/2026-03-24-introducing-linear-agent
https://linear.app/changelog/2026-04-23-linear-agent-mcp-support
https://linear.app/changelog/2026-05-14-code-intelligence
https://linear.app/changelog/2026-05-27-linear-diffs
https://linear.app/changelog/2026-06-11-coding-sessions
https://linear.app/changelog/2026-07-20-introducing-loops
https://linear.app/changelog/2026-08-20-coding-environments
```

This is a dated baseline. Linear changes continuously; a future living registry must refresh it.

---

## 3. Capability families

### L1 — Work item management

Observed Linear-class outcomes:

```text
issue/work-item creation
rich description/editor
assignee
status/workflow
priority
estimate
labels
team ownership
project membership
cycle membership
milestone membership
release linkage where applicable
due dates
sub-issues
parent/child hierarchy
relations/dependencies
blocking/blocked-by relationships
comments
attachments
subscribers
history/activity
recurring issues
issue templates
structured/form-like templates
keyboard-first creation and editing
bulk operations
```

**Fehrest requirement:** parity-level work objects must preserve these outcomes while binding them to provenance, decisions, memory and evidence.

---

### L2 — Projects and milestones

Observed Linear-class outcomes:

```text
project overview
project status
project lead/members
start/target dates
project description/docs
project milestones
milestone progress
project issue scope
project timeline
project updates
project health
project dependencies
project filtering/grouping
```

**Fehrest opportunity:** every project update and milestone should be able to trace to decisions, evidence, customer signals and agent/human activity.

---

### L3 — Initiatives and strategic planning

Observed Linear-class outcomes:

```text
workspace-level initiatives
initiative status
initiative priority
initiative labels
initiative description/context
initiative project membership
initiative progress
initiative updates
initiative views
initiative/project timeline visibility
```

**Fehrest requirement:** strategic intent must connect to execution without losing the evidence and decisions that justified the initiative.

---

### L4 — Cycles / iteration planning

Required benchmark family:

```text
bounded planning iterations
cycle membership
current/upcoming/completed cycles
carry-over behavior
cycle scope
cycle progress
team-level planning
capacity/velocity-aware planning where justified
```

Fehrest may use different terminology, but the user outcome is mandatory if Linear replacement is claimed.

---

### L5 — Roadmaps, timelines and dependencies

Required benchmark family:

```text
multi-project roadmap
initiative hierarchy
timeline views
project and milestone dates
dependencies
cross-project sequencing
progress visibility
health/status communication
planning filters/grouping
forecast/predictive completion where evidence justifies it
```

Fehrest must distinguish deterministic schedule state from AI/forecast-derived predictions.

---

### L6 — Triage

Observed Linear-class outcomes:

```text
dedicated triage state/inbox
incoming issue review
accept/reject/merge/reroute workflows
triage responsibility
responsibility rotation
rules/automation
required fields before leaving triage
support/integration-originated intake
nonstandard intake kept outside normal active workflow until accepted
```

**Fehrest opportunity:** triage decisions should retain why an item was accepted, rejected, deduplicated or routed.

---

### L7 — Internal requests / Asks-class intake

Observed Linear-class outcomes:

```text
requests from non-product users
Slack intake
email intake
web-form intake
request -> issue conversion
triage/prioritization
status/response flow
pattern discovery across requests
```

Fehrest should generalize this into an auditable Request/Intake surface rather than forcing every requester to become a full workspace operator.

---

### L8 — Customer Requests and customer intelligence

Observed Linear-class outcomes:

```text
customer records
customer attributes
revenue/size/tier-style attributes
customer requests
request -> issue linkage
request -> project linkage
customer request counts
important request markers
customer impact filtering
customer impact ordering
customer-specific request history
support-system intake integrations
```

**Fehrest supremacy target:** preserve the full trace:

```text
customer/source evidence
-> request
-> product decision
-> initiative/project/work item
-> code/change
-> verification
-> shipped result
-> later learning
```

---

### L9 — Custom Views and information architecture

Observed Linear-class outcomes:

```text
saved issue views
saved project views
saved initiative views
filters
sorting
ordering
grouping
list layout
board layout
timeline layout
favorites/sharing
workspace/team scoped views
```

Fehrest already proposes Bases/Views; Linear parity must become an explicit acceptance dimension for work objects.

---

### L10 — Insights and dashboards

Observed Linear-class outcomes:

```text
real-time issue analytics
breakdowns/grouping
resource allocation analysis
bug-fix speed
priority consistency
estimate accuracy
cycle/project analytics
delivery bottleneck visibility
saved/shared analytical views
dashboards/operational reporting
```

Fehrest should add provenance-aware analytics and make metric definitions inspectable.

---

### L11 — Docs and work context

Required benchmark family:

```text
project documents
issue-linked documents
mentions/references
comments
history/versioning
work-item references inside docs
project specs and updates
```

Fehrest's existing Notes/Docs direction is broader; Linear becomes an additional work-context usability benchmark.

---

### L12 — GitHub/code linkage and Code Intelligence

Observed Linear-class outcomes:

```text
GitHub integration
repository linkage
issue <-> pull request linkage
commit linkage
CI/check visibility
codebase-aware agent context
questions about implementation and constraints
code-informed product/spec planning
```

Fehrest should use its GitHub + Memory Repository architecture to exceed this by adding historical decisions, failed attempts, provenance and bounded context receipts.

---

### L13 — Native diffs and reviews

Observed Linear-class outcomes:

```text
review pull-request diffs inside the work product
changed-file navigation
review discussions
CI/check context
review state
GitHub synchronization
guided/assisted review
agent iteration from review feedback
merge/ship flow from the product surface
```

Fehrest should integrate with a forge rather than silently become a Git hosting product, but the native review user outcome is a mandatory benchmark if developer-team replacement is claimed.

---

### L14 — Linear Agent-class workspace agent

Observed Linear-class outcomes:

```text
workspace-aware conversational agent
issue/project planning assistance
spec/update drafting
workspace guidance/instructions
issue/comment/chat interaction
external context through tools
triage automation
agent session tracking
```

Fehrest must remain multi-agent/provider rather than binding the canonical product to one agent implementation.

---

### L15 — MCP-connected external context

Observed Linear-class outcomes:

```text
workspace-admin configured MCP servers
external tool/data access
agent use of external context
workspace-level permissions/allowlists
use across chat/comments/automations
```

Fehrest's MCP/WebMCP direction remains broader and more explicitly authority-bounded. Linear is an interoperability usability benchmark, not a security authority model to copy blindly.

---

### L16 — Coding sessions / delegated implementation

Observed Linear-class outcomes as of this baseline:

```text
delegate issue to coding agent
Claude Code / Codex execution paths
managed development environment
repository preparation
runtime/toolchain setup
configurable environment
runtime version configuration
prepare scripts
environment variables
repository guidance
agent/model selection
PR creation
diff returned to work item
steering/follow-up
review handoff
merge flow
```

Fehrest should expose a provider-neutral execution-session contract. The coding runtime may be supplied by Codex, Claude Code, OpenHands, Daytona, E2B, OpenSandbox or another later authorized provider rather than becoming a mandatory Fehrest-owned sandbox.

---

### L17 — Verification and browser testing

Observed Linear-class outcomes:

```text
start/run application in coding environment
browser automation
user-flow verification
screenshots
recordings
iteration after detected failure
verification artifacts attached to session/review
```

Fehrest should make verification artifacts first-class evidence and preserve their provenance.

---

### L18 — Loops / recurring and event-driven agent automation

Observed Linear-class outcomes:

```text
natural-language automation definition
schedule trigger
event trigger
workspace/team scoped automation
shared visibility
inspectable run history
workspace + connected-tool context
triage/research/dispatch workflows
meeting/incident follow-up workflows
project/spec maintenance workflows
```

Fehrest already plans event/schedule automation; Linear raises the required product usability bar and shared-operational visibility bar.

---

### L19 — API and webhooks

Observed Linear-class outcomes:

```text
public API
entity query
entity mutation
webhooks
real-time client observation after mutations
integration ecosystem support
```

Fehrest must expose stable, permission-equivalent APIs without creating a weaker authority path than its native clients.

---

### L20 — Notifications, inbox and personal work

Required benchmark family:

```text
assignment notifications
mentions
subscriptions
triage notifications
customer-request notifications
review notifications
agent completion/failure
personal assigned/delegated work view
notification preferences
```

Fehrest's calm-notification thesis remains; parity means no important operational signal is lost.

---

### L21 — Mobile

Observed current benchmark includes:

```text
issue/task workflows
notifications
comments
review participation
coding-session visibility
mobile diff review
line-specific feedback
agent steering
```

Fehrest mobile must eventually cover its broader capture/memory use cases while meeting the Linear-class work/review baseline.

---

### L22 — Enterprise and administration

Required benchmark family:

```text
private teams/scopes
guests/external collaborators
SSO/SAML-class identity federation
SCIM-class provisioning
roles/admin controls
audit/security visibility
workspace policy
agent/MCP administration
usage/cost controls where AI execution exists
```

Fehrest should add self-host/private/local-first modes rather than weakening them to match a cloud-first benchmark.

---

### L23 — Import, export and migration

A Linear replacement claim requires measured migration quality for Linear-origin data.

Future coverage should include, where available through official export/API:

```text
workspace/team structure
issues
projects
initiatives
milestones
cycles
statuses/workflows
labels
templates
relationships
comments
attachments
docs/references
customer/request relations
GitHub/PR references
timestamps
history where available
```

Unsupported constructs must be reported rather than silently discarded.

---

## 4. Capability disposition model

Every Linear capability entering the living registry receives one of:

```text
PARITY_REQUIRED
OUTCOME_REQUIRED_DIFFERENT_IMPLEMENTATION
INTEGRATE_INSTEAD_OF_BUILD
DEFER_WITH_EXPLICIT_REASON
REJECT_WITH_EXPLICIT_REASON
```

`DEFER` and `REJECT` require a user-outcome rationale and cannot be used merely to reduce implementation scope.

---

## 5. Fehrest superiority dimensions

Parity alone is not the target. Linear-class workflows should become stronger when projected over Fehrest's substrate.

### S1 — Evidence-backed work

Every consequential work item can expose the sources, requests and decisions that caused it to exist.

### S2 — Temporal truth

Users and agents can distinguish:

```text
what is true now
what was believed when work was planned
what changed afterward
what assumption became stale
```

### S3 — Agent context receipts

Delegated work can record exactly what bounded context an agent received.

### S4 — Failed-attempt memory

Previous unsuccessful human/agent attempts become discoverable evidence rather than repeated cost.

### S5 — Reviewable durable learning

Completion can propose durable Memory/Decision/Procedure updates through review rather than letting a model silently rewrite institutional knowledge.

### S6 — Open/local ownership

Canonical product and memory state remains recoverable without a mandatory Fehrest hosted service.

### S7 — Arbitrary agent/provider support

No single coding agent, LLM or sandbox vendor becomes canonical product authority.

---

## 6. Future parity evidence gates

A mature benchmark should include representative end-to-end journeys such as:

```text
customer request -> triage -> issue -> project -> initiative -> cycle -> implementation -> review -> ship
internal ask -> triage -> owner -> completion -> requester update
bug intake -> duplicate/related detection -> priority -> coding agent -> browser verification -> review
project planning -> milestones/dependencies -> updates/health -> completion
support evidence -> customer impact -> product decision -> implementation
agent loop -> event -> investigation -> follow-up work -> auditable run history
```

For every critical journey compare at least:

```text
workflow completeness
time to complete
interaction count
keyboard efficiency
information loss
migration fidelity
mobile availability
API availability
offline behavior where Fehrest claims it
agent authority/safety
provenance quality
recovery/exportability
```

---

## 7. Baseline status

```text
LINEAR_RESEARCH_BASELINE_DATE=2026-08-31
LINEAR_MANDATORY_BENCHMARK=YES
LINEAR_ADDITIVE_TO_EXISTING_V2=YES
LINEAR_REPLACES_EXISTING_BENCHMARKS=NO
CAPABILITY_FAMILIES_RECORDED=23
LIVING_REGISTRY_REQUIRED=YES
IMPLEMENTATION_AUTHORIZED=NO
R1_CHANGED=NO
CURRENT_CHANGED=NO
```
