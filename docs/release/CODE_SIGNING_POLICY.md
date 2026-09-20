# Code signing policy

This page states, per platform, how Flake release artifacts are signed today, and links the
exact verification steps for each mechanism. Linked from the repository home page
(`README.md`) as required by any signing provider whose terms mandate a published code
signing policy (SignPath Foundation's terms do, see below).

## Status summary

| Platform | Mechanism | Status |
|---|---|---|
| Windows | SignPath Foundation (free OSS Authenticode signing) | Repository-owned prerequisites complete; application to SignPath Foundation is a Founder action not yet submitted — see `docs/release/SIGNPATH_ELIGIBILITY_PACKET.md` |
| Linux | Project-controlled GPG release-signing key | **PASS.** A real release candidate (CLI archive, its SHA-256 manifest, and the `.deb` bundle) was signed with the production key and independently re-verified in a clean keyring seeded only with the published public key — see `docs/release/LINUX_RELEASE_SIGNING.md` and CI run [35498327004](https://github.com/TheHalfMoon/Flake/actions/runs/35498327004) |
| macOS | Zero-cost direct distribution (project GPG signature + GitHub attestation, ad-hoc codesign) | **PASS.** Founder decision, `docs/canonical/FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md` — no Apple Developer ID, no notarization, no App Store; not claimed to carry Apple platform trust. Real release candidate `.dmg` ad-hoc-signed, GPG-signed with the same identity as Linux, and independently re-verified — see `docs/release/MACOS_DIRECT_DISTRIBUTION.md` and CI run [35514419522](https://github.com/TheHalfMoon/Flake/actions/runs/35514419522) |
| All platforms | GitHub artifact attestations (build provenance) | Implemented as an additional, non-substituting supply-chain evidence layer — see `docs/release/RELEASE_VERIFICATION.md` |

No release is published as final/production-signed until its platform's row above reads a
completed, non-blocked state. An `UNSIGNED_DEVELOPER_RC` label on any distributed artifact
means exactly that — a developer/test build, never a production release, regardless of which
row is closer to ready.

## Windows — SignPath Foundation

**Free code signing provided by SignPath.io, certificate by SignPath Foundation.**

Flake intends to use [SignPath Foundation](https://signpath.org/)'s free code-signing program
for open-source projects rather than a purchased commercial certificate, once the Founder has
submitted and SignPath has approved the application (`docs/release/SIGNPATH_ELIGIBILITY_PACKET.md`
records the exact eligibility check against SignPath's own published terms and the exact
remaining external action).

### Roles

SignPath Foundation requires a project to name Authors, Reviewers, and Approvers, each using
multi-factor authentication for both SignPath and source-repository access. Flake is currently
a single-maintainer project; until additional maintainers exist, one person (the Founder,
GitHub account [`TheHalfMoon`](https://github.com/TheHalfMoon)) holds all three roles:

| Role | Holder |
|---|---|
| Author (may modify source that gets signed) | TheHalfMoon |
| Reviewer (approves external contributions before they can be signed) | TheHalfMoon |
| Approver (authorizes each signing request) | TheHalfMoon |

This table will be updated the moment any role is delegated to a different person.

### Privacy

This program will not transfer any information to other networked systems unless specifically
requested. Flake's own runtime privacy statement (unrelated to the signing pipeline itself) is
recorded in `src/about.rs` and shown in the CLI's `flake license` command and the desktop
About screen: local-first, offline, account-free, no telemetry.

## Linux — project GPG release-signing key

Full architecture, one-time key generation runbook, CI secret-injection mechanics, rotation and
revocation procedure: [`docs/release/LINUX_RELEASE_SIGNING.md`](LINUX_RELEASE_SIGNING.md).

## macOS — zero-cost direct distribution

Apple requires an active, paid Apple Developer Program membership (US $99/year, per
[Apple's own enrollment documentation](https://developer.apple.com/programs/)) to obtain a
Developer ID certificate and to notarize/staple a distributed `.app`/`.dmg`. There is no
free/open-source exception in Apple's current program — that research is unchanged and
remains true.

What changed is the Founder's product decision about which release model Flake ships under:
per `docs/canonical/FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md`,
Flake does not purchase Apple Developer Program membership and does not distribute through
the Mac App Store. Flake instead distributes an ad-hoc-signed `.dmg`
(`scripts/release/sign_macos.sh`, `DIRECT_DISTRIBUTION_MODE=1` — distinct from
`TEST_SIGNING_MODE=1`, which is pipeline-mechanics-only and never the shipped artifact),
checksummed, signed with Flake's own project-controlled GPG release-signing key (the same
identity already qualified for Linux), and covered by a GitHub build-provenance attestation.

This is **not** claimed to be an Apple Developer ID signature, notarization, or stapling, and
`spctl --assess` is expected — and confirmed in CI — to reject this artifact. Users open it via
macOS's own supported per-app override (System Settings → Privacy & Security → "Open Anyway",
or right-click → Open). Full design, exact CI evidence, and the user-facing override
instructions: [`docs/release/MACOS_DIRECT_DISTRIBUTION.md`](MACOS_DIRECT_DISTRIBUTION.md).

## Additional supply-chain evidence: GitHub artifact attestations

Independent of the three signing mechanisms above, Flake generates
[GitHub artifact attestations](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations)
(build provenance bound to the exact commit and workflow run that produced an artifact) for
release-candidate archives. This is additional evidence, not a substitute for Windows
Authenticode, Apple Developer ID/notarization, or the Linux GPG signature where those apply.
See [`docs/release/RELEASE_VERIFICATION.md`](RELEASE_VERIFICATION.md) for verification commands.
