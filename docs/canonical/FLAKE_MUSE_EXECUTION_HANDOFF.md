# Flake Muse execution handoff

Status: `ASTRO_PLAN_COMPLETE=YES`; implementation is active. `specs/CURRENT.md` owns the live frontier.

## Canonical entry point

1. Reverify live GitHub/repository truth.
2. Read `specs/CURRENT.md`.
3. Read `docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md`.
4. Read `docs/canonical/FOUNDER_WEBVIEW2_NETWORK_BOUNDARY_2026-09-16.md`.
5. Read `docs/canonical/FOUNDER_T04-06_ACCESSIBILITY_WITNESS_AMENDMENT_2026-09-16.md`.
6. Read `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` completely.
7. Confirm the dependency frontier before changing implementation.
8. Execute the dependency-ready unit named by `CURRENT`; do not restart closed predecessors.

The canonical build plan is the sole implementation roadmap. Historical R1/Fehrest plans are evidence and lineage, not alternate execution roadmaps.

The 2026-09-16 no-human-gates founder decision is an additive Class E amendment to that roadmap. It removes external human participant/reviewer qualification gates, replaces T03-08 and T05-06 with automated technical qualification contracts, and amends R11. It does not alter product owner acceptance semantics or security boundaries.

The 2026-09-16 WebView2 network-boundary founder decision is an additive Class D amendment scoped only to WebView2-hosted (or any shared-OS-webview-hosted) desktop surfaces. It clarifies T04-01's "starts offline with no external requests" acceptance clause as two separately provable requirements (zero Flake-owned external requests; zero network dependency for normal operation) rather than one literal, unsatisfiable reading, after WebView2's own background telemetry traffic was found unfixable by application-level mitigation. It does not authorize any new Flake-initiated network request, and does not extend to any other task or acceptance clause without its own explicit founder ruling.

The 2026-09-16 T04-06 accessibility-witness amendment is an additive Class E amendment scoped only to `T04-06`. It removes that task's requirement for a live human operator to run a witnessed screen-reader/IME/sign-off checklist, replacing it with an automated technical accessibility qualification wherever one is technically reproducible; it does not reduce the three-native-platform (Windows/macOS/Linux) requirement, and does not extend to any other task's own human-review language without its own explicit ruling.

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
