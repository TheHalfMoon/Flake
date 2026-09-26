# Code signing policy

This page states, per platform, how Pluma release artifacts are signed today, and links the
exact verification steps for each mechanism. Linked from the repository home page
(`README.md`) as required by any signing provider whose terms mandate a published code
signing policy (SignPath Foundation's terms do, see below).

## Status summary

| Platform | Mechanism | Status |
|---|---|---|
| Windows | Website-first direct distribution (unsigned; project GPG signature + GitHub provenance) | **DIRECT_WEB.** Founder amendment `docs/canonical/FOUNDER_WEBSITE_FIRST_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-26.md` — no Microsoft Store, no paid certificate, no SignPath dependency for v1; `WINDOWS_AUTHENTICODE_TRUST=NOT_AVAILABLE`, `WINDOWS_AUTHENTICODE_TRUST_CLAIMED=NO`, SmartScreen warnings expected and disclosed — see `docs/release/WINDOWS_DIRECT_DISTRIBUTION.md` (qualification pending exact CI evidence: `PENDING_DIRECT_DISTRIBUTION_QUALIFICATION_CI_RUN`) |
| Linux | Project-controlled GPG release-signing key | **PASS.** A real release candidate (CLI archive, its SHA-256 manifest, and the `.deb` bundle) was signed with the production key and independently re-verified in a clean keyring seeded only with the published public key — see `docs/release/LINUX_RELEASE_SIGNING.md` and CI run [35498327004](https://github.com/TheHalfMoon/Pluma/actions/runs/35498327004) |
| macOS | Zero-cost direct distribution (project GPG signature + GitHub attestation, ad-hoc codesign) | **PASS.** Founder decision, `docs/canonical/FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md` — no Apple Developer ID, no notarization, no App Store; not claimed to carry Apple platform trust. Real release candidate `.dmg` ad-hoc-signed, GPG-signed with the same identity as Linux, and independently re-verified — see `docs/release/MACOS_DIRECT_DISTRIBUTION.md` and CI run [35514419522](https://github.com/TheHalfMoon/Pluma/actions/runs/35514419522) |
| All platforms | GitHub artifact attestations (build provenance) | Implemented as an additional, non-substituting supply-chain evidence layer — see `docs/release/RELEASE_VERIFICATION.md` |

No release is published as final/production-signed until its platform's row above reads a
completed, non-blocked state. An `UNSIGNED_DEVELOPER_RC` label on any distributed artifact
means exactly that — a developer/test build, never a production release, regardless of which
row is closer to ready.

## Windows — website-first direct distribution (unsigned)

Pluma distributes its Windows installer and CLI archive directly from the web under
`docs/canonical/FOUNDER_WEBSITE_FIRST_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-26.md`: no
Microsoft Store, no mandatory app store, no paid certificate, and no paid cloud signing
service. Full design: [`docs/release/WINDOWS_DIRECT_DISTRIBUTION.md`](WINDOWS_DIRECT_DISTRIBUTION.md).

```text
WINDOWS_DISTRIBUTION_MODE=DIRECT_WEB
WINDOWS_UNSIGNED_DIRECT_DISTRIBUTION_ALLOWED=YES
WINDOWS_AUTHENTICODE_TRUST=NOT_AVAILABLE
WINDOWS_AUTHENTICODE_TRUST_CLAIMED=NO
WINDOWS_SMARTSCREEN_WARNING=EXPECTED_AND_DISCLOSED
WINDOWS_STORE_SIGNATURE=NOT_USED
WINDOWS_PAID_CERTIFICATE_REQUIRED=NO
```

Windows Authenticode is desirable but not required for v1 completion when no zero-cost trusted
certificate is available. A real Windows signature inspection showing `AUTHENTICODE=NOT_SIGNED`
is the expected result under this amendment — acceptable only when explicitly documented,
never represented as trusted/signed, and only when every other release-integrity gate
(checksum, project GPG signature, GitHub provenance, SBOM, install qualification) passes.

### Prior SignPath Foundation history (preserved, not deleted)

**Free code signing once intended via SignPath.io, certificate by SignPath Foundation.**

Pluma once intended to use [SignPath Foundation](https://signpath.org/)'s free code-signing
program for open-source projects rather than a purchased commercial certificate. The Founder
submitted the application (recorded 2026-09-21); SignPath Foundation / Phillip Deng declined
it on 2026-09-25 (`APPLICATION NOT APPROVED AT THIS TIME`) for insufficient public trust and
visibility signals, inviting reapplication after broader recognition and offering a paid
subscription, which the Founder declined. `docs/release/SIGNPATH_ELIGIBILITY_PACKET.md`
records the exact history. SignPath may be revisited after Pluma gains public adoption
(`SIGNPATH_FOUNDATION_REAPPLY_AFTER_PUBLIC_ADOPTION=YES`); it is not a v1 blocker
(`T05-04_SIGNPATH_EXTERNAL_BLOCKER=SUPERSEDED_BY_FOUNDER_DIRECT_DISTRIBUTION_DECISION`). No
SignPath approval, organization/project/signing-policy identifiers, or API credentials were
ever received — this repository claims none of those.

### Roles

SignPath Foundation requires a project to name Authors, Reviewers, and Approvers, each using
multi-factor authentication for both SignPath and source-repository access. Pluma is currently
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
requested. Pluma's own runtime privacy statement (unrelated to the signing pipeline itself) is
recorded in `src/about.rs` and shown in the CLI's `pluma license` command and the desktop
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

What changed is the Founder's product decision about which release model Pluma ships under:
per `docs/canonical/FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md`,
Pluma does not purchase Apple Developer Program membership and does not distribute through
the Mac App Store. Pluma instead distributes an ad-hoc-signed `.dmg`
(`scripts/release/sign_macos.sh`, `DIRECT_DISTRIBUTION_MODE=1` — distinct from
`TEST_SIGNING_MODE=1`, which is pipeline-mechanics-only and never the shipped artifact),
checksummed, signed with Pluma's own project-controlled GPG release-signing key (the same
identity already qualified for Linux), and covered by a GitHub build-provenance attestation.

This is **not** claimed to be an Apple Developer ID signature, notarization, or stapling, and
`spctl --assess` is expected — and confirmed in CI — to reject this artifact. Users open it via
macOS's own supported per-app override (System Settings → Privacy & Security → "Open Anyway",
or right-click → Open). Full design, exact CI evidence, and the user-facing override
instructions: [`docs/release/MACOS_DIRECT_DISTRIBUTION.md`](MACOS_DIRECT_DISTRIBUTION.md).

## Additional supply-chain evidence: GitHub artifact attestations

Independent of the three signing mechanisms above, Pluma generates
[GitHub artifact attestations](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations)
(build provenance bound to the exact commit and workflow run that produced an artifact) for
release-candidate archives. This is additional evidence, not a substitute for Windows
Authenticode, Apple Developer ID/notarization, or the Linux GPG signature where those apply.
See [`docs/release/RELEASE_VERIFICATION.md`](RELEASE_VERIFICATION.md) for verification commands.
