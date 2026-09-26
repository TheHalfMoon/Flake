# Founder amendment — website-first direct distribution and Windows trust (2026-09-26)

## Status

`FOUNDER_DECISION=RATIFIED`, effective 2026-09-26. This is an explicit Founder governance
amendment under `AGENTS.md` section 5 Class E ("product thesis/founder direction — founder
authorization + architecture reconsideration"), following the same pattern already used by
`FOUNDER_WEBVIEW2_NETWORK_BOUNDARY_2026-09-16.md`,
`FOUNDER_T05-02_PHYSICAL_POWER_LOSS_AMENDMENT_2026-09-16.md`,
`FOUNDER_T04-06_ACCESSIBILITY_WITNESS_AMENDMENT_2026-09-16.md`, and
`FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md`.

## What this supersedes

The prior reading of `FLAKE_CANONICAL_BUILD_PLAN.md` section 25's `T05-04` acceptance clause
— "all signatures/notarization/stapling verify" — required a trusted Windows Authenticode
signature for the Windows payloads/installer, pursued through a free third-party SignPath
Foundation certificate (`docs/release/SIGNPATH_ELIGIBILITY_PACKET.md`,
`docs/release/CODE_SIGNING_POLICY.md`, and `specs/CURRENT.md`'s prior
`T05-04_WINDOWS_SIGNING_STATUS=PENDING_SIGNPATH_EXTERNAL_APPROVAL`).

That requirement is **superseded prospectively, for Windows distribution only**, effective from
this amendment forward. It does not apply retroactively to any other platform's acceptance
clause (Linux GPG release-signing and the macOS zero-cost direct-distribution qualification
are unaffected) and it does not reopen or reinterpret any other closed task.

The Founder's decision, in the exact terms given:

```text
WEBSITE_FIRST_DIRECT_DISTRIBUTION=YES
MICROSOFT_STORE_DISTRIBUTION=NO
MAC_APP_STORE_DISTRIBUTION=NO
MANDATORY_APP_STORE_DISTRIBUTION=NO
FOUNDER_ZERO_COST_DISTRIBUTION_REQUIRED=YES
WINDOWS_DISTRIBUTION_MODE=DIRECT_WEB
WINDOWS_UNSIGNED_DIRECT_DISTRIBUTION_ALLOWED=YES
WINDOWS_AUTHENTICODE_TRUST=NOT_AVAILABLE
WINDOWS_AUTHENTICODE_TRUST_CLAIMED=NO
WINDOWS_SMARTSCREEN_WARNING=EXPECTED_AND_DISCLOSED
WINDOWS_STORE_SIGNATURE=NOT_USED
WINDOWS_PAID_CERTIFICATE_REQUIRED=NO
```

Pluma is distributed directly from the web. Users download Pluma directly from the web, in the
same general distribution model used by independent desktop applications that publish direct
downloads. The project website/download surface may initially use GitHub Releases as the binary
hosting backend, which keeps founder cost at zero. Pluma will not use the Microsoft Store, the
Mac App Store, any other mandatory app store, paid Windows signing solely to satisfy the
current release, paid Apple Developer Program membership, or paid cloud signing services.

## What this amendment states

1. Pluma is distributed directly from the web.
2. Microsoft Store is not part of the v1 distribution contract.
3. Mac App Store is not part of the v1 distribution contract.
4. Windows Authenticode is desirable but not required for v1 completion when no zero-cost
   trusted certificate is available.
5. Unsigned Windows distribution is permitted only with explicit trust limitations.
6. SmartScreen/security warnings must never be hidden or misrepresented.
7. No instruction may tell users to disable Windows security globally (no disabling Microsoft
   Defender globally, no disabling SmartScreen globally, no weakening system policy, no
   bypassing enterprise controls, no disabling security services).
8. No instruction may tell macOS users to disable Gatekeeper globally.
9. Checksums, SBOM, provenance, updater signatures (where an updater exists), installation
   qualification, and release integrity remain mandatory.
10. Future trusted Authenticode signing can be added without changing persisted data formats or
    product behavior.
11. SignPath and OSSign may be revisited after Pluma gains public adoption.
12. This amendment is prospective and does not rewrite historical T05-04 evidence.

## What is explicitly NOT claimed

This amendment does not assert, and no repository artifact may assert, that an unsigned Windows
build carries Authenticode platform trust. Record truthfully, everywhere this applies:

```text
WINDOWS_AUTHENTICODE_STATUS=NOT_SIGNED
WINDOWS_AUTHENTICODE_TRUST=NOT_AVAILABLE
WINDOWS_AUTHENTICODE_TRUST_CLAIMED=NO
WINDOWS_SMARTSCREEN_WARNING=EXPECTED_AND_DISCLOSED
WINDOWS_STORE_SIGNATURE=NOT_USED
```

In particular, `WINDOWS_AUTHENTICODE_STATUS=PASS` must never be set unless a real trusted
Authenticode signature is independently verified. A real Windows signature inspection showing
`AUTHENTICODE=NOT_SIGNED` is the expected result under this amendment, and is acceptable only
when explicitly documented, never represented as trusted/signed, and only when every other
release-integrity gate passes.

No documentation under this amendment may instruct users to disable Microsoft Defender
globally, disable SmartScreen globally, weaken system policy, bypass enterprise controls, or
disable security services. The only permitted Windows trust-UX path is: verify SHA-256,
confirm the download came from the official Pluma website/release, inspect GitHub provenance,
and use Windows "More info" / "Run anyway" when the user personally chooses to trust the
verified artifact. The macOS rule is unchanged: only Apple's supported per-app override path,
never a system-wide Gatekeeper disable.

## Historical evidence preserved

Nothing in `docs/evidence/flake-v1/T05-04/REPORT.md`'s prior addenda, `specs/CURRENT.md`'s
prior narrative history, `docs/release/CODE_SIGNING_POLICY.md`'s prior text, or
`docs/release/SIGNPATH_ELIGIBILITY_PACKET.md`'s prior text is deleted or rewritten to look as
if this were always the plan. The genuine record is preserved:

- SignPath Foundation was applied to (Founder-confirmed submission, recorded 2026-09-21);
- SignPath Foundation later declined the application (external response dated 2026-09-25,
  recorded below);
- the Founder then changed the v1 distribution/trust contract (this amendment).

## SignPath Foundation response (Founder-supplied external evidence, 2026-09-25)

The Founder received a real response from SignPath Foundation / Phillip Deng, dated
2026-09-25, with outcome `APPLICATION NOT APPROVED AT THIS TIME`. The stated reason is that
the project does not yet have enough public trust and visibility signals for the Foundation
program. Signals cited include GitHub stars, forks, contributors, external articles,
independent references/discussions, Reddit, Stack Overflow, YouTube, institutional backing, and
sustained activity and engagement. SignPath explicitly invited the project to reapply after
broader recognition, and also offered a regular paid subscription. The Founder decision is: do
not use the paid SignPath route.

Recorded state (no invented identifiers — no SignPath rejection ID, project ID, organization
ID, policy slug, certificate, API token, exact internal scoring, or minimum star count is
claimed anywhere in this repository):

```text
T05-04_SIGNPATH_APPLICATION_STATUS=REJECTED_INSUFFICIENT_PUBLIC_VISIBILITY
T05-04_SIGNPATH_FOUNDATION_APPROVAL=NO
T05-04_SIGNPATH_REAPPLY_AFTER_ADOPTION=YES
T05-04_SIGNPATH_PAID_ROUTE=DECLINED_BY_FOUNDER
T05-04_SIGNPATH_EXTERNAL_BLOCKER=SUPERSEDED_BY_FOUNDER_DIRECT_DISTRIBUTION_DECISION
```

## OSSign (deferred future option)

OSSign may remain a future option. Current known external state from Founder research: free OSS
signing exists for qualifying projects; applications are currently suspended/backlogged;
eligibility includes public/open-source requirements, public CI/build provenance,
project/community activity, and a minimum activity history. Pluma v1 is not blocked on OSSign.

```text
OSSIGN_STATUS=DEFERRED_FUTURE_OPTION
OSSIGN_V1_BLOCKER=NO
OSSIGN_REAPPLY_WHEN_ELIGIBLE=YES
```

No OSSign eligibility or approval is fabricated or claimed.

## New Windows technical qualification (replaces the external-signing-only exit condition)

The `T05-04` acceptance clause for Windows is amended to read: *the Windows release candidate
satisfies the direct-distribution technical qualification below, with exact CI evidence, in
place of trusted Authenticode signature verification.* Every other `T05-04` acceptance clause
(license/notice completeness, advisory refresh, build-script review, About/help distribution
content) is unaffected and unchanged.

Qualification checklist (see `docs/release/WINDOWS_DIRECT_DISTRIBUTION.md` for the full design
and exact CI evidence once run):

A. BUILD — real Windows x86_64 production installer builds; real canonical `pluma.exe`
   builds; required compatibility aliases still behave identically; no arbitrary
   developer-only artifacts are presented as production downloads.
B. INSTALL — installer works on a clean/native Windows environment; current-user install
   behavior is correct; installation paths are deterministic/documented; app starts
   successfully after installation.
C. DATA CONTINUITY — existing Pluma/legacy-Flake vaults are retained; app-data compatibility
   remains intact; preserved legacy bundle identifier behavior does not orphan data; upgrade
   from an earlier qualified build does not lose user data; uninstall does not silently
   destroy vault data unless explicitly documented and user-authorized.
D. UPDATE — Pluma has no auto-updater (see `docs/release/USER_GUIDE.md` section 7: updates
   are manually downloaded verified packages, never silent migration). If a Tauri updater is
   ever introduced, its signatures must be cryptographically verified, the private updater
   signing key must never enter the repo, the public updater key may be shipped as required,
   an invalid/tampered update must fail closed, and rollback behavior must be qualified where
   canonical requirements require it. Until then, the documented manual-update model is the
   actual update model — none is invented.
E. INTEGRITY — for every downloadable production artifact: SHA-256, exact source commit,
   build workflow/run, architecture, version, SBOM, provenance/attestation when technically
   available, file size, release notes, installation instructions, uninstall instructions, and
   known trust limitations.
F. AUTHENTICODE TRUTH — a real Windows signature inspection runs on the production artifact.
   Expected result under this amendment: `AUTHENTICODE=NOT_SIGNED`. Acceptable only when
   explicitly documented, never represented as trusted/signed, and only when all other
   release-integrity gates pass.
G. SMARTSCREEN / SECURITY UX — Windows may show SmartScreen or reputation warnings; this is
   documented truthfully with safe instructions only (verify SHA-256, confirm the official
   Pluma website/release source, inspect GitHub provenance, use "More info" / "Run anyway"
   only on the user's own trust decision). Never instruct disabling Defender/SmartScreen
   globally, weakening system policy, bypassing enterprise controls, or disabling security
   services.
H. CLEAN DOWNLOAD REVERIFICATION — after publishing: download artifacts again from the actual
   public distribution URL, verify SHA-256 byte identity, verify release metadata, verify
   files are actually downloadable, and verify no stale Flake-brand production filename is
   accidentally presented as the new Pluma final artifact unless intentionally preserved as
   historical evidence.

Expected resulting state once the qualification workflow has actually run green in CI (not
before — no field below may read `PASS` on the strength of this document alone):

```text
WINDOWS_DISTRIBUTION_MODE=DIRECT_WEB
WINDOWS_DIRECT_WEB_DISTRIBUTION=PASS
WINDOWS_AUTHENTICODE_TRUST=NOT_AVAILABLE
WINDOWS_AUTHENTICODE_TRUST_CLAIMED=NO
WINDOWS_SMARTSCREEN_WARNING=EXPECTED_AND_DISCLOSED
```

## Distribution surface

Pluma does not operate separate paid website infrastructure and this amendment does not create
any. "Website-first distribution" is implemented as this repository's own README,
`docs/release/` pages (including the new `docs/release/DOWNLOAD.md` landing document, prepared
so a future domain can point to it without changing release semantics), and — once a version
is actually published (a distinct, separately Founder-authorized action under plan section 25:
"Do not purchase credentials or publish remotely without actual authority") — its GitHub
Releases page, which may serve as the artifact backend. No paid CDN, paid hosting, paid
database, required cloud backend, analytics dependency, or account/sign-in requirement is
introduced. Inventing separate paid hosted infrastructure as a new canonical channel would be a
further Class C/E change beyond what this amendment authorizes; it is out of scope here.

If the repository does not yet contain a standalone website, the repository/release download
page is the canonical web distribution surface for v1.

## macOS (unchanged)

The existing Founder zero-Apple-fee decision
(`FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md`) is preserved. macOS
remains direct-download. No Developer ID, notarization, stapling, or Mac App Store requirement
is resurrected. Truth labels stand:

```text
MACOS_DISTRIBUTION_MODE=DIRECT_WEBSITE
MACOS_DEVELOPER_ID_REQUIRED=NO
MACOS_NOTARIZATION_REQUIRED=NO
MACOS_GATEKEEPER_TRUST=NOT_CLAIMED
MACOS_APPLE_PLATFORM_TRUST=NOT_CLAIMED
```

Existing independent verification mechanisms (checksum, project GPG signature, GitHub
attestation, install qualification) remain required.

## Linux (unchanged)

Linux remains direct-download and project-signed. The already-qualified production GPG signing
model is preserved. Linux verification is not weakened.

## Historical release (unchanged)

`v0.0.1-phase-t-rc.1` is preserved as historical/qualification evidence. Its files are never
silently replaced and its existing SHA-256 identities are never changed. Any new
final/production-ready Pluma artifact needed after this amendment is built as a new
artifact/release from exact fresh `main`, never by mutating the historical RC.

## Version naming

No `v1.0.0` tag is invented by this amendment. The next version/tag follows repository
governance; Phase-T development artifacts are not represented as the final product unless all
required gates are complete.

## Future trust improvements (not v1 blockers)

```text
SIGNPATH_FOUNDATION_REAPPLY_AFTER_PUBLIC_ADOPTION=YES
OSSIGN_REAPPLY_WHEN_ELIGIBLE=YES
```

These are future trust improvements, not v1 blockers.

## Authority

This amendment governs `T05-04` and any later task whose acceptance criteria reference Windows
signing/trust. It does not authorize skipping any other acceptance clause, evidence
requirement, or cross-platform gate in the canonical plan.
