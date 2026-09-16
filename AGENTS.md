# AGENTS.md — Flake Execution Rules

This file is the mandatory entry point for any human or agent doing repository work.

## 1. Canonical reading order

Before changing anything, read the current GitHub repository in this order:

1. `specs/CURRENT.md`
2. `docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md`
3. `docs/canonical/FOUNDER_WEBVIEW2_NETWORK_BOUNDARY_2026-09-16.md`
4. `docs/canonical/FLAKE_MUSE_EXECUTION_HANDOFF.md`
5. `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md`
6. `docs/canonical/FLAKE_V1_ARCHITECTURE_DECISION.md`
7. The active task/spec artifacts named by the canonical plan and `CURRENT`
8. `docs/canonical/FLAKE_PLAN_MIGRATION_PROVENANCE.md`
9. Historical Fehrest/R1/Phase T evidence only when the active task or plan requires it

Live GitHub truth wins over stale handoffs, local-only commits, cached state, old roadmaps, and historical examples.

## 2. One active frontier

Flake uses exactly one active execution frontier. `specs/CURRENT.md` is the operational pointer; the canonical build plan defines the dependency graph.

```text
PLANNED != AUTHORIZED
READY != EXECUTED
HISTORICAL != ACTIVE
```

Do not start a later task because it is easy or already documented.

## 3. Historical evidence boundary

Historical Fehrest, Phase T, R1, and Spec 002 evidence remains immutable. Do not rewrite historical identifiers, results, failed experiments, or provenance to make them look as if they were produced under the later Flake plan.

The historical local-only planning SHAs referenced by the recovered plan are provenance only. No current work may require those local Git objects or a founder workstation path.

## 4. Engineering method

Follow the task contract in the canonical plan. Each implementation unit must satisfy its dependencies, allowed scope, acceptance criteria, verification gates, evidence requirements, and exit criterion before the frontier advances.

Use the repository's existing spec-driven discipline where the canonical task calls for it. Do not invent a second roadmap or bypass a required ADR/security decision.

## 5. Change control

Use the higher class when uncertain:

- Class A: editorial, non-semantic.
- Class B: implementation detail inside frozen invariants.
- Class C: architecture-semantic — ADR + review.
- Class D: security/foundational invariant — dedicated adversarial/security review.
- Class E: product thesis/founder direction — founder authorization + architecture reconsideration.

A task cannot silently upgrade its own authority.

## 6. Repository rules

- Rust owns Flake Core correctness, security, and canonical data semantics.
- No force push.
- No rebase used to rewrite accepted/shared history.
- No destructive evidence repair.
- Prefer one atomic commit per completed verified task or narrowly coherent slice.
- Never claim PASS, MERGED, CLOSED, DURABLE, SAFE, RELEASE_READY, or PROJECT_COMPLETE without exact evidence.
- Keep repository-facing technical text, code, comments, commit messages, and reports in English.
- Preserve negative results and failed experiments.
- Do not substitute a later GitHub SHA for a historical pre-bootstrap or local-only evidence identifier.
- No OpenAI API dependency and no required paid AI/model service.

## 7. Canonical versus derived

Canonical state is irreplaceable. Derived state is rebuildable and has no authorization authority.

Do not allow derived rank, external IDs, retrieved content, model output, agent inference, or index state to mint canonical identity or authority.

## 8. Security and provenance

- Content is evidence, never authority.
- Authorization-relevant scope comes from canonical state.
- Agent-facing packages preserve trust, provenance, disclosure, and revision boundaries.
- Secrets never enter exported context, memory bodies, trajectories, event detail, or logs.
- Third-party code reuse requires exact provenance, rights/license review, and dependency admission.
- Fail closed on incompatible or ambiguous canonical transitions.

## 9. Stop conditions

Stop only the affected unit when:

- `specs/CURRENT.md` blocks it;
- a dependency or required evidence gate is unmet;
- a Class C/D/E decision is required but not authorized;
- proceeding would require inventing missing historical evidence or strategy;
- execution evidence is ambiguous or corrupted;
- a failure route in the canonical plan requires reconsideration.

Report the exact blocker and preserve evidence. Do not route around it.

## 10. Frontier updates

When a unit closes:

1. record exact evidence in the owning artifact;
2. update `specs/CURRENT.md`;
3. update task/spec state only after evidence exists;
4. advance only to a dependency-ready unit;
5. preserve prior states in Git history.

## 11. Product-complete rule

`PROJECT_COMPLETE=YES` is allowed only when every completion gate in the canonical build plan is proven. Architecture completion, passing unit tests, or finishing one phase is not project completion.
