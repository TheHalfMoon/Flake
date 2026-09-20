# Flake

**Local work continuity for people and replaceable agents.**

Flake is a local-first application for capturing project work, preserving the evidence behind decisions, and resuming after interruption with visible changes and next actions. External agents may receive bounded evidence packages and return reviewable proposals; the project remains understandable when an agent disappears.

## Canonical status

```text
ASTRO_PLAN_COMPLETE=YES
IMPLEMENTATION_STARTED=NO
NEXT_DEPENDENCY_READY_UNIT=T00-01
PROJECT_COMPLETE=NO
SPEC_003_AUTO_ACTIVATION=PROHIBITED
```

The canonical implementation roadmap is:

- [`docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md`](docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md)
- [`docs/canonical/FLAKE_MUSE_EXECUTION_HANDOFF.md`](docs/canonical/FLAKE_MUSE_EXECUTION_HANDOFF.md)
- [`specs/CURRENT.md`](specs/CURRENT.md)

Migration/provenance record:

- [`docs/canonical/FLAKE_PLAN_MIGRATION_PROVENANCE.md`](docs/canonical/FLAKE_PLAN_MIGRATION_PROVENANCE.md)

## Historical evidence

This repository contains substantial Fehrest, Phase T, R1, benchmark, and Spec 002 history. Those artifacts remain preserved evidence and architecture lineage. They are not alternate active roadmaps unless the Flake canonical build plan explicitly incorporates them.

Historical identifiers are not rewritten to fit later GitHub history. The final Astro plan was migrated to GitHub with an exact recovered source SHA-256 recorded in the migration provenance file.

## Core constraints

- local-first canonical ownership and offline core workflows;
- Rust owns correctness/security/canonical data semantics;
- canonical and derived state remain separate;
- deterministic, evidence-linked resolution and recovery;
- replaceable agents receive bounded authority, not implicit trust;
- no mandatory graph/vector/model platform in v1;
- no OpenAI API dependency or required paid AI/model service;
- no `PROJECT_COMPLETE=YES` without all canonical completion gates.

## Contributing / agent execution

Read [`AGENTS.md`](AGENTS.md) first. Reverify live GitHub truth before each unit and execute only the dependency-ready frontier named by `specs/CURRENT.md` and the canonical plan.

## License

The recovered Astro plan records Apache-2.0 intent, but final rights/notice/package clearance remains an execution gate until the repository license surface is explicitly qualified by the canonical task graph.

## Code signing policy

Release artifact signing status, the free/open-source signing mechanisms Flake uses, and how
to verify a release: [`docs/release/CODE_SIGNING_POLICY.md`](docs/release/CODE_SIGNING_POLICY.md).

## Download Flake

Flake has not yet published a numbered release (see the canonical status above — release
qualification is still in progress under `T05-04`). This repository's own README/docs and,
once a version is published, its [GitHub Releases](https://github.com/TheHalfMoon/Flake/releases)
page are the canonical distribution surface — Flake does not operate separate website
infrastructure.

| Platform | Signing/trust model | Notes |
|---|---|---|
| Windows | SignPath Foundation (free OSS Authenticode) — pending Founder application/approval | See `docs/release/SIGNPATH_ELIGIBILITY_PACKET.md` |
| macOS | Direct download — independently verifiable (checksum, project GPG signature, GitHub attestation), **not Apple notarized** — **PASS** | See `docs/release/MACOS_DIRECT_DISTRIBUTION.md`; no Apple endorsement implied |
| Linux | Project-controlled GPG release-signing key — **PASS** | See `docs/release/LINUX_RELEASE_SIGNING.md` |

Every platform's published artifact, once a release exists, will list its version,
architecture, SHA-256 checksum, signature/provenance verification command, release notes,
installation instructions, and links to `LICENSE`/`NOTICE`/source — see
[`docs/release/RELEASE_VERIFICATION.md`](docs/release/RELEASE_VERIFICATION.md) and
[`docs/release/USER_GUIDE.md`](docs/release/USER_GUIDE.md).
