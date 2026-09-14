# Flake Muse execution handoff

Status: `ASTRO_PLAN_COMPLETE=YES`; implementation has not started.

## Canonical entry point

1. Reverify live GitHub/repository truth.
2. Read `specs/CURRENT.md`.
3. Read `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` completely.
4. Confirm the dependency frontier before changing implementation.
5. If unchanged and implementation is explicitly authorized, begin `T00-01`.

The canonical build plan is the sole implementation roadmap. Historical R1/Fehrest plans are evidence and lineage, not alternate execution roadmaps.

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
