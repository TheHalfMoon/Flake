# Fehrest V2 — Product Measurement, Privacy, and Migration Evidence Contract

**Status:** PROGRAM PROPOSAL / NON-AUTHORIZING  
**Created:** 2026-08-31  
**Canonical frontier:** `specs/CURRENT.md`  
**Canonical execution plan:** `docs/canonical/EXECUTION_MASTER_PLAN.md`

> This document closes planning-quality gaps around product adoption measurement, privacy-preserving telemetry, migration fidelity, and time-to-value. It does not change R1, authorize scoring, activate Spec 002, authorize V2 implementation, or make any replacement claim canonical.

---

## 1. Purpose

Fehrest's V2 program already defines broad product capability, semantic ownership, dependency order, and evidence-first implementation gates. Product readiness also requires proof that people and agents can adopt the product, reach useful outcomes quickly, migrate without silent loss, and measure those outcomes without violating the local-first ownership promise.

The program therefore adds four cross-cutting requirements:

```text
PRODUCT_VALUE_MEASUREMENT=REQUIRED
PRIVACY_PRESERVING_TELEMETRY_POLICY=REQUIRED
MIGRATION_FIDELITY_ACCEPTANCE=REQUIRED
TIME_TO_VALUE_EVIDENCE=REQUIRED
```

These requirements are evidence contracts. They are not authority to implement analytics infrastructure while the current frontier is R1.

---

## 2. Measurement principles

Product metrics must not become an incentive to weaken Fehrest's trust model.

```text
METRIC != AUTHORITY
TELEMETRY != CANONICAL_MEMORY
DERIVED_ANALYTICS != USER_CONFIRMED_TRUTH
OBSERVED_USAGE != PERMISSION
OPT_IN != IRREVOCABLE_CONSENT
```

Every future metric MUST identify:

```text
metric_id
user outcome
population/profile
numerator
denominator
start event
success event
failure/censoring rule
data source
privacy class
aggregation rule
retention rule
owner
acceptance or decision use
```

A metric without a stable definition cannot be used to close a readiness gate.

---

## 3. Product outcome hierarchy

Future product specifications should select only the metrics relevant to their user outcome. The program-level candidate hierarchy is:

### 3.1 Activation

Measure whether a new user, team, or agent successfully reaches the first independently useful Fehrest outcome.

Candidate activation outcomes include:

```text
PERSONAL_ACTIVATION = create/import useful knowledge -> retrieve it successfully
DEVELOPER_ACTIVATION = bind repository -> request scoped context -> receive valid receipt
TEAM_ACTIVATION = create/join workspace -> complete a shared work/knowledge outcome
AGENT_ACTIVATION = discover Fehrest -> obtain authorized context -> return evidence or proposal
MIGRATION_ACTIVATION = import source workspace -> verify accepted fidelity report
```

Do not define activation as account creation, application launch, or a click with no user value.

### 3.2 Time to value

Future usability/product proof MUST measure elapsed user effort to declared value outcomes.

Candidate metrics:

```text
TIME_TO_FIRST_VALUE
TIME_TO_FIRST_TRUSTED_RETRIEVAL
TIME_TO_FIRST_SUCCESSFUL_IMPORT
TIME_TO_FIRST_TEAM_OUTCOME
TIME_TO_FIRST_AGENT_CONTEXT_RECEIPT
TIME_TO_FIRST_REVIEWED_MEMORY_PROPOSAL
```

No numeric target is invented in this planning document. A future owning spec MUST establish targets from a predeclared baseline or pilot before using them as a release gate.

### 3.3 Continued value

Where applicable, future product proof should measure:

```text
successful_return_rate
workflow_completion_rate
search_or_ask_success_rate
migration_completion_rate
repeated_context_usefulness
stale_premise_error_rate
repeated_failed_attempt_rate
human_review_burden
agent_task_success_rate
recovery_success_rate
export_success_rate
```

Retention alone is insufficient when continued use can coexist with poor correctness or lock-in.

### 3.4 Efficiency

For comparable workflows, measure:

```text
human_interaction_count
keyboard_or_command_steps
context_switch_count
wall_clock_time_to_outcome
model_visible_tokens
provider_cost_where_applicable
human_review_minutes
migration_operator_interventions
```

Efficiency comparisons MUST preserve equivalent task scope and user outcome.

---

## 4. Target-setting discipline

Readiness targets must be evidence-backed rather than aesthetically chosen.

Future owning specs MUST use one of:

```text
BASELINE_RELATIVE_TARGET
PILOT_DERIVED_TARGET
SAFETY_OR_CORRECTNESS_ABSOLUTE_TARGET
EXTERNAL_COMPETITOR_PARITY_TARGET_WHERE_METHODS_ARE_COMPARABLE
```

The target record MUST identify its evidence source and date.

Do not use a single aggregate KPI to conceal a safety, migration, correctness, or privacy regression.

---

## 5. Privacy-preserving telemetry policy

Fehrest's public promise includes local-first ownership, inspectability, portability, offline usefulness, and no mandatory AI. Product analytics must preserve that promise.

### 5.1 Default data-minimization rule

Where a product metric can be computed locally, prefer local computation.

Remote product telemetry MUST NOT require raw canonical content merely to measure adoption.

Prohibited remote telemetry payloads by default include:

```text
note/document/message bodies
memory bodies
source document contents
repository source code
raw prompts or model responses
secrets or credentials
clipboard contents
attachment contents
raw agent trajectories
raw imported workspace archives
full filesystem paths when a less identifying representation suffices
```

### 5.2 Consent and control

Any remote product telemetry beyond strictly necessary service-operation data MUST have an explicit user or organization policy basis appropriate to the deployment mode.

Future product requirements MUST define:

```text
remote_telemetry_enabled_state
consent_or_org_policy_source
event_classes_enabled
retention
inspection/export surface
deletion/disable behavior
self_hosted_or_local_measurement_path_where_applicable
```

Turning telemetry off MUST NOT disable local canonical correctness.

### 5.3 Pseudonymous identifiers

When stable identifiers are needed for aggregate product measurement, use the least identifying scope that satisfies the metric. Prefer scoped/pseudonymous identifiers and rotation where longitudinal linkage is not required.

Never treat a telemetry identifier as Fehrest object identity or authorization identity.

### 5.4 Sensitive dimensions

Future analytics plans MUST classify each field before collection:

```text
PUBLIC_OR_PRODUCT_METADATA
PSEUDONYMOUS_USAGE_METADATA
ORGANIZATION_OPERATIONAL_METADATA
CONTENT_DERIVED_AGGREGATE
CONTENT_OR_SECRET_PROHIBITED_BY_DEFAULT
```

Any content-derived aggregate requires an explicit threat/privacy review showing why local computation or coarser data is insufficient.

### 5.5 Telemetry failure behavior

```text
TELEMETRY_UNAVAILABLE -> PRODUCT_CORE_REMAINS_FUNCTIONAL
TELEMETRY_DISABLED -> PRODUCT_CORE_REMAINS_FUNCTIONAL
ANALYTICS_PIPELINE_CORRUPT -> CANONICAL_MEMORY_UNAFFECTED
ANALYTICS_PROVIDER_REMOVED -> CANONICAL_MEMORY_UNAFFECTED
```

Telemetry is derived/operational evidence, never the only copy of user state.

---

## 6. Migration fidelity acceptance model

A migration is not successful because an importer exits zero or because most visible text appears present.

Every migration source MUST define an explicit source snapshot and fidelity report.

### 6.1 Source binding

Where the source exposes the information, bind:

```text
source_system
source_export_or_api_version
source_workspace_or_export_identity
source_snapshot_time
source_revision_or_export_hash
importer_id
importer_version
mapping_contract_version
import_run_id
```

### 6.2 Fidelity classes

Every in-scope source construct MUST be classified as one of:

```text
EXACT
SEMANTICALLY_EQUIVALENT
TRANSFORMED_WITH_PROVENANCE
UNSUPPORTED_EXPLICIT
EXCLUDED_BY_DECLARED_SCOPE
FAILED
```

`SILENTLY_DROPPED` is never an acceptable class.

### 6.3 Required reconciliation dimensions

Where applicable to the source, migration verification MUST reconcile:

```text
object counts by type
critical object identities or stable source mappings
content
structured properties
workflow/status values
hierarchy
relations/dependencies
projects/initiatives/milestones/cycles
comments/replies
attachments
customer/request relationships
GitHub/PR references
author/member mappings
timestamps/timezones
history/version information exposed by the source
permissions or explicit unsupported permission semantics
source provenance
unsupported constructs
```

### 6.4 Hard acceptance invariants

For a claimed in-scope migration profile:

```text
SILENTLY_DROPPED_CRITICAL_OBJECTS=0
SILENTLY_DROPPED_CRITICAL_FIELDS=0
BROKEN_IN_SCOPE_REFERENTIAL_RELATIONS=0
UNREPORTED_UNSUPPORTED_CRITICAL_CONSTRUCTS=0
SOURCE_TO_FEHREST_MAPPING_COLLISIONS=0
```

Any exception must narrow the migration claim; it cannot be hidden in an aggregate percentage.

### 6.5 Idempotency and repeatability

Where the source snapshot and mapping contract are unchanged, repeated import MUST have a specified deterministic/idempotent outcome and MUST NOT create uncontrolled duplicate canonical objects.

The importer MUST preserve enough evidence to distinguish:

```text
newly imported
already imported
updated from source
conflicted
unsupported
operator-resolved
```

### 6.6 Rollback and removal

When safe rollback/removal of an imported batch is supported, it MUST be scoped by import provenance and MUST NOT delete unrelated user-created state.

When safe rollback cannot be guaranteed, the product must state that limitation before import.

---

## 7. Linear migration/replacement proof hardening

`LINEAR_ADDITIVE_PRODUCT_EXECUTION_TRACK.md` already requires Linear migration and replacement proof. This contract strengthens the evidence required before any scoped `REPLACES_LINEAR` claim.

For each claimed profile, L-PX5/L-GA evidence MUST include:

```text
source snapshot binding
capability coverage profile
migration fidelity report
unsupported construct report
workflow replay/journey results
time-to-value results
operator intervention count
critical information-loss count
mobile/API/agent availability results where claimed
export/recovery result
```

A claim is profile-scoped. Passing a startup profile does not prove a multi-team enterprise profile.

---

## 8. Adoption experiments and competitor comparisons

When comparing Fehrest against Obsidian, Notion, Slack/Zulip, Linear, GitHub-linked workflows, or other systems:

```text
same user outcome
same starting information
same declared migration scope
equivalent user role/permissions
comparable hardware/network constraints where material
predeclared success/failure criteria
raw evidence preserved where practical
```

Do not force feature-shape equivalence where the products intentionally solve the outcome differently.

The comparison should answer whether Fehrest achieves the user outcome with acceptable correctness, trust, effort, and portability—not whether every screen is copied.

---

## 9. Human and agent adoption must be separated

Human adoption and agent effectiveness are related but not interchangeable.

At minimum, future proof should distinguish:

```text
HUMAN_ONBOARDING
HUMAN_RETRIEVAL_OR_WORKFLOW
TEAM_COLLABORATION
AGENT_DISCOVERY
AGENT_CONTEXT_RETRIEVAL
AGENT_TASK_SUCCESS
HUMAN_REVIEW_OF_AGENT_OUTPUT
```

An agent benchmark cannot prove ordinary-user onboarding quality. A usability study cannot prove agent continuation correctness.

---

## 10. Readiness evidence bundle

A future product/adoption gate should preserve an inspectable evidence bundle containing, as applicable:

```text
measurement definitions
benchmark/usability protocol
source dataset/workspace profile
privacy/data-collection declaration
raw or minimally sufficient event evidence
aggregation code/version
migration source manifest
migration reconciliation report
failure/unsupported report
summary metrics
decision record
```

If raw product telemetry is intentionally not retained for privacy reasons, preserve the minimal reproducibility evidence and aggregation contract needed to audit the conclusion without retaining prohibited content.

---

## 11. Failure routing

Any of these outcomes blocks the corresponding readiness claim:

```text
CRITICAL_MIGRATION_LOSS_PRESENT
UNEXPLAINED_METRIC_DEFINITION_DRIFT
TIME_TO_VALUE_NOT_MEASURED_WHEN_REQUIRED
PRIVACY_POLICY_VIOLATION
TELEMETRY_REQUIRED_FOR_CORE_CORRECTNESS
UNSCOPED_REPLACEMENT_CLAIM
INCOMPARABLE_BASELINE_PRESENTED_AS_PARITY
UNREPRODUCIBLE_GATE_METRIC
```

Failure narrows or reopens the claim. It does not authorize weakening the evidence contract.

---

## 12. Ownership and future integration

This document is a cross-program planning contract. It does not create a duplicate analytics or migration implementation owner.

Expected future ownership:

```text
PROGRAM_BLUEPRINT                         -> measurement/privacy evidence requirements
015 Import/Migration                     -> generic importer and migration evidence mechanics
018 Organization/Admin                   -> organization telemetry/privacy policy controls
future hosted Hub/operational specs      -> hosted service operational telemetry where authorized
L-PX5/L-GA                               -> Linear migration/replacement proof
individual product specs                 -> their own activation/time-to-value/readiness metrics
```

Exact Spec Kit insertion and ownership reconciliation remains a post-R1 V2 decision task.

---

## 13. Current effect

```text
PRODUCT_MEASUREMENT_CONTRACT=PREPARED
PRIVACY_PRESERVING_TELEMETRY_POLICY=PREPARED
MIGRATION_FIDELITY_ACCEPTANCE=PREPARED
TIME_TO_VALUE_MEASUREMENT=PREPARED
LINEAR_REPLACEMENT_EVIDENCE_HARDENED=YES
R1_CHANGED=NO
R1_SCORING_AUTHORIZED=NO
CURRENT_CHANGED=NO
SPEC_002_CHANGED=NO
V2_PROGRAM_CANONICAL=NO
IMPLEMENTATION_AUTHORIZED=NO
```
