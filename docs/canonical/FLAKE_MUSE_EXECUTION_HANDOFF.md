# Flake Muse execution handoff

Status: `ASTRO_PLAN_COMPLETE=YES`; implementation is active. `specs/CURRENT.md` owns the live frontier.

## Canonical entry point

1. Reverify live GitHub/repository truth.
2. Read `specs/CURRENT.md`.
3. Read `docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md`.
4. Read `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` completely.
5. Confirm the dependency frontier before changing implementation.
6. Execute the dependency-ready unit named by `CURRENT`; do not restart closed predecessors.

The canonical build plan is the sole implementation roadmap. Historical R1/Fehrest plans are evidence and lineage, not alternate execution roadmaps.

The 2026-09-16 founder decision is an additive Class E amendment to that roadmap. It removes external human participant/reviewer qualification gates, replaces T03-08 and T05-06 with automated technical qualification contracts, and amends R11. It does not alter product owner acceptance semantics or security boundaries.

## Execution rules

- Execute one dependency-ready unit at a time.
- Satisfy every dependency, acceptance criterion, verification gate, evidence requirement, and exit criterion before advancing.
- Do not invent product strategy, architecture, security semantics, data semantics, UX semantics, agent authority, or roadmap ordering already resolved by the plan.
- Do not automatically activate Spec 003.
- Preserve failed attempts and negative evidence.
- Do not force-push, rebase shared history, or rewrite accepted historical evidence.
- Do not claim tests, CI, performance, durability, security, licensing, signing, release readiness, or `PROJECT_COMPLETE=YES` without exact evidence.
- No OpenAI API dependency and no required paid AI/model service.

## Remote-only authority

Execution does not depend on the historical local planning commits, OneDrive paths, or any unpushed artifact. Those identifiers are provenance only. The GitHub repository and its current canonical files are sufficient to determine the execution frontier.
