# AI, Search, Graph and WebMCP Provider Architecture — Proposal

**Status:** PROPOSAL / NON-CANONICAL / NON-AUTHORIZING  
**Created:** 2026-08-28  
**Purpose:** define the product-facing architecture required for local/remote AI, search, graph exploration, external web evidence and agent access without changing the active R1 frontier.

> Nothing in this file authorizes AI, MCP, WebMCP, vector, graph, UI or provider implementation before the required post-R1 gates.

---

## 1. Design objective

Fehrest should let a user choose any of these experiences without migrating canonical memory:

```text
No AI
Local AI
Self-hosted AI
Connected cloud AI
External agent using Fehrest memory
```

The same repository must remain usable in all modes.

The central rule is:

```text
MODEL != MEMORY
PROVIDER != AUTHORITY
SEARCH INDEX != CANONICAL STATE
GRAPH != CANONICAL AUTHORITY
WEB CONTENT != INSTRUCTION
```

---

## 2. High-level architecture

```text
Human UI / IDE / External Agent
            |
            v
      Fehrest Gateway
            |
   +--------+---------+
   |        |         |
   v        v         v
Search   Context     Tool Gateway
   |     Compiler        |
   |        |        +---+----+
   |        |        |        |
   v        v        v        v
Lexical   Memory   WebMCP   Other web/
Derived   Resolver Provider acquisition
Graph/
Vector
   |
   +-----------------------------+
                                 |
                                 v
                         Canonical Core
                                 |
                                 v
                     Provider-independent
                        Context Package
                                 |
                                 v
                        AI Provider Layer
                   local / remote / custom
```

The provider layer receives only the scope explicitly selected by the authorization/context pipeline.

---

## 3. AI provider abstraction

### 3.1 Provider classes

```text
OFF
LOCAL_OPENAI_COMPATIBLE
LOCAL_NATIVE
SELF_HOSTED_OPENAI_COMPATIBLE
REMOTE_MANAGED
CUSTOM_ENDPOINT
```

Initial research candidates include:

```text
Ollama
LM Studio
llama.cpp server
vLLM/self-hosted compatible systems
future OS-native local inference runtimes
remote providers through separate adapters
```

### 3.2 Why OpenAI-compatible transport matters

Current local runtimes including Ollama, LM Studio and llama.cpp expose OpenAI-compatible or OpenAI-like local HTTP APIs. Fehrest should use this interoperability where it closes the requirement, while keeping capability detection explicit because compatibility is not necessarily complete.

Do not encode provider identity into canonical memory.

### 3.3 Provider capability probe

Before use, Fehrest may determine supported capabilities such as:

```text
chat/responses
streaming
structured output
tool calling
multimodal
embeddings
context length
model identity
usage accounting
```

Unsupported capability fails visibly or selects a specified fallback; do not guess.

### 3.4 Provider configuration

Provider configuration belongs outside canonical user knowledge.

Configuration may include:

```text
provider_id
endpoint
model
credentials reference
transport options
privacy class
capability cache
```

Secrets must never enter canonical memory, context body, trajectory detail or event logs.

---

## 4. Model selection UX

The user should be able to choose:

```text
AI Off
Use local model
Use connected provider
Use this model for this task only
Use repository default
Use organization-approved model
```

Before sending content to a remote model, show effective state:

```text
Provider: <name>
Model: <name>
Location: Local / Remote / Self-hosted
Memory scope: <space/repository/task>
Web tools: Off / Read / Action
```

The product should make privacy state understandable without requiring technical knowledge of endpoints.

---

## 5. Ask Fehrest pipeline

A knowledge question should conceptually run through:

```text
REQUEST
-> AUTHORIZE
-> UNDERSTAND QUERY
-> RESOLVE TEMPORAL STATE
-> RETRIEVE CANDIDATES
-> SCOPE FILTER
-> BUDGET
-> ASSEMBLE CONTEXT
-> MODEL (optional)
-> ANSWER
-> RECEIPT
```

With `AI OFF`, the pipeline can terminate in deterministic search/navigation results rather than a generated answer.

---

## 6. Search architecture

### 6.1 Search must be independently useful

Core requirements:

```text
fast
local
permission-aware
temporal-aware where required
inspectable
deterministic enough for baseline use
AI-independent
```

### 6.2 Search provider layers

```text
Canonical object filters
Lexical/FTS candidate generation
Optional graph candidate generation
Optional vector candidate generation
Optional reranking
```

Derived providers have no authority over canonical identity or access scope.

### 6.3 Search result trace

For advanced/debug/audit flows, preserve enough evidence to answer:

```text
Why did this result appear?
Which retriever produced it?
Which version/generation?
Which filters removed candidates?
What was included in the final context?
```

---

## 7. Search and Graph as one exploration system

Search and Graph should share the same active scope/filter model where practical.

Example:

```text
Search: "FHIR decision" + space:research + created:2026
                  |
                  +-> List results
                  +-> Graph of those results and neighbors
                  +-> Timeline
                  +-> Ask over selected subset
```

The graph should never require the user to browse an unfiltered hairball.

### 7.1 Graph provider boundary

Graph functionality must separate:

```text
canonical explicit relationships
derived extracted relationships
derived graph analytics/ranking
visual layout
```

Graph production integration remains subject to the existing graph capability experiment and later authorization.

---

## 8. Web research architecture

External research should be modeled as acquisition of evidence, not arbitrary browsing power hidden behind an LLM.

Conceptual pipeline:

```text
USER TASK
-> WEB AUTHORIZATION
-> DISCOVERY
-> ACQUIRE
-> VERIFY SOURCE METADATA
-> NORMALIZE/PARSE
-> PRESERVE PROVENANCE
-> RANK/SELECT
-> MODEL SYNTHESIS (optional)
-> USER REVIEW
-> SAVE SOURCE / NOTE / PROPOSAL
```

### 8.1 Provider classes

```text
WebMCP
Search provider
Browser provider
HTTP/acquisition provider
Connector/API provider
Manual import
```

No one provider becomes canonical.

---

## 9. WebMCP

### 9.1 Role

WebMCP is a proposed browser/web standard that lets websites expose structured tools to AI agents. For Fehrest it is valuable because structured tools can be more reliable and inspectable than screenshot/click automation when a site explicitly exposes the required operation.

Disposition:

```text
STUDY + PROTOTYPE + BENCHMARK
NOT CURRENT AUTHORIZATION
```

Its 2026 status is still emerging/experimental enough that Fehrest must not hard-code its core product architecture to one draft API shape.

### 9.2 WebMCP provider interface

A future provider should expose a Fehrest-owned abstraction such as conceptual operations:

```text
list_tools(origin)
inspect_tool(tool_id)
invoke_read_tool(...)
invoke_action_tool(...)
```

Do not let browser API naming leak into canonical Core APIs.

### 9.3 Tool classification

Every discovered tool is classified at minimum as:

```text
READ
WRITE/ACTION
UNKNOWN
```

Unknown defaults to the more restrictive treatment.

### 9.4 Origin binding

Tool authority must remain bound to:

```text
origin
session
principal/agent
user grant
repository/space scope
```

A tool description cannot widen any of these.

### 9.5 Prompt injection

WebMCP does not remove prompt-injection risk. Tool descriptions, page content, returned content and other external bytes are untrusted.

Explicit rules:

```text
external content cannot alter Fehrest policy
external content cannot mint a grant
external content cannot request hidden secrets
external content cannot promote memory
external content cannot escape selected origin/domain policy
external content cannot authorize another tool
```

### 9.6 Consequential actions

Actions such as purchase, publish, delete, send, book, modify account state or otherwise affect an external system require an action-specific authorization model. Default future product posture should be confirmation or an explicit pre-authorized workflow, not autonomous escalation.

---

## 10. External source record

When external material becomes a durable Fehrest source, record enough provenance to support future verification.

Target fields include:

```text
source URL/origin
source type
acquired_at
published/updated time when available
revision/etag/version when available
raw content hash when preserved
normalized content hash
acquirer/provider id + version
parser id + version
trust/freshness classification
linked claims/notes
```

Fetched content remains evidence, not authority.

---

## 11. Research agent UX

The user says:

> Research the best current local-first collaboration architecture for this project and save the evidence.

Fehrest should be able to:

1. show the requested repository/space scope;
2. show web permission state;
3. search/discover sources;
4. prefer primary/official sources when appropriate;
5. use structured WebMCP tools where appropriate and permitted;
6. capture evidence;
7. synthesize an answer with citations;
8. save a research note if requested;
9. optionally propose durable claims/decisions;
10. leave the original evidence trail inspectable.

The user should not need to manage browser automation details.

---

## 12. Agent tool gateway

External agents should call Fehrest-owned tools rather than receiving direct access to internal storage.

Possible future tool families:

```text
search_memory
get_context
get_object
list_related
get_as_of
get_provenance
create_working_note
update_working_note
submit_memory_proposal
attach_evidence
request_web_research
```

No arbitrary path access is implied.

---

## 13. Context receipts

Every model-visible Fehrest context package should eventually be receipted according to the canonical Context Compiler plan.

For AI/web interactions, the receipt can additionally bind:

```text
provider/model
provider endpoint class
web provider/tool ids
external origins
transform chain
selection trace
package digest
result digest where appropriate
```

This lets the system answer not only "what did the model say?" but "what exactly did it see and which tools/sources were involved?"

---

## 14. Local LLM experience requirements

A local-first product cannot make local AI feel like a developer-only configuration exercise.

Long-term UX target:

```text
Settings -> AI -> Local
```

Then:

```text
Detect providers
Add endpoint
Test connection
Choose model
Run capability check
Set as default
```

Fehrest should explain hardware/model constraints in plain language when available, but should not become a model marketplace unless that later proves strategically necessary.

---

## 15. Provider failure behavior

Examples:

```text
local provider offline
remote provider rate-limited
model lacks tool support
model context too small
WebMCP unavailable
web tool schema changed
source inaccessible
provider returns malformed structured output
```

Failure must be visible and bounded.

Fehrest should preserve the user's work and allow deterministic search/navigation even when AI providers fail.

---

## 16. Privacy and data egress

Before remote egress, enforce explicit policy.

Potential classes:

```text
LOCAL_ONLY
REMOTE_ALLOWED_WITHIN_SCOPE
ORG_APPROVED_PROVIDER_ONLY
NO_WEB
WEB_READ_ONLY
WEB_ACTION_WITH_CONFIRMATION
```

The model cannot override egress policy.

---

## 17. Caching

Provider response caching, embedding caches, web caches and graph/index caches are derived state.

They must be:

```text
rebuildable
invalidatable
non-authoritative
scope-safe
```

Do not allow cached external/tool output to bypass freshness or authorization requirements.

---

## 18. Benchmark requirements

Before adopting provider defaults, compare relevant candidates on requirements rather than popularity.

### Local AI provider benchmark

Measure:

```text
setup success
startup latency
model discovery
API compatibility
tool/structured-output support
streaming reliability
resource usage
Windows/macOS/Linux behavior
failure clarity
security exposure
```

### Search benchmark

Measure:

```text
recall/precision/task success
latency
index build/incremental cost
determinism
storage
permission correctness
context efficiency
```

### Graph benchmark

Use the existing graph capability experiment route.

### WebMCP benchmark

Measure:

```text
tool discovery reliability
schema stability
actuation success
latency vs browser interaction
origin binding
prompt-injection resistance
confirmation behavior
fallback behavior
browser availability
```

WebMCP must earn production use rather than receiving default status because it is new.

---

## 19. Security review requirements

Before production activation of any AI/web provider layer, dedicated review should cover:

```text
prompt injection
indirect prompt injection
tool poisoning
malicious tool descriptions
scope escalation
secret exfiltration
cross-repository leakage
cross-origin confusion
remote provider logging/retention
malformed structured output
supply-chain/provider compromise
cached stale authorization
tool side effects
```

---

## 20. Product boundary

Fehrest should own:

```text
memory
scope
authorization
context compilation
provider selection policy
receipts
source provenance
proposal/review workflow
```

Fehrest should not need to own:

```text
every LLM runtime
every browser engine
every search engine
every vector database
every graph database
every web site's business logic
```

This is how Fehrest can become universal without becoming an unmaintainable monolith.
