# Founder amendment — zero-Apple-fee direct macOS distribution (2026-09-20)

## Status

`FOUNDER_DECISION=RATIFIED`, effective 2026-09-20. This is an explicit Founder governance
amendment under `AGENTS.md` §5 Class E ("product thesis/founder direction — founder
authorization + architecture reconsideration"), following the same pattern already used by
`FOUNDER_WEBVIEW2_NETWORK_BOUNDARY_2026-09-16.md`, `FOUNDER_T05-02_PHYSICAL_POWER_LOSS_AMENDMENT_2026-09-16.md`,
and `FOUNDER_T04-06_ACCESSIBILITY_WITNESS_AMENDMENT_2026-09-16.md`.

## What this supersedes

The prior reading of `FLAKE_CANONICAL_BUILD_PLAN.md` section 25's `T05-04` acceptance clause
— "all signatures/notarization/stapling verify" — required an active, paid Apple Developer
Program membership (US $99/year) to obtain a Developer ID certificate and to notarize/staple
the macOS `.app`/`.dmg`, per `docs/release/CODE_SIGNING_POLICY.md`'s prior text and
`specs/CURRENT.md`'s prior `T05-04_MACOS_SIGNING_STATUS=BLOCKED_EXTERNAL_APPLE_CREDENTIALS`.

That requirement is **superseded prospectively, for macOS only**, effective from this
amendment forward. It does not apply retroactively to any other platform's acceptance clause
(Windows Authenticode via SignPath Foundation and the Linux GPG release-signing key are
unaffected) and it does not reopen or reinterpret any other closed task.

The Founder's decision, in the exact terms given:

```text
APPLE_DEVELOPER_PROGRAM_PAYMENT_AUTHORIZED=NO
MAC_APP_STORE_DISTRIBUTION=NO
MACOS_PAID_APPLE_SIGNING_REQUIRED=NO
MACOS_DEVELOPER_ID_REQUIRED=NO
MACOS_NOTARIZATION_REQUIRED=NO
MACOS_STAPLING_REQUIRED=NO
MACOS_DISTRIBUTION_MODE=DIRECT_WEBSITE
```

Flake's macOS release path is direct distribution of a project-controlled, ad-hoc-signed,
checksummed, project-GPG-signed, and GitHub-attested artifact from Flake's own distribution
surface (today: this repository's README/docs and, once a version is published, its GitHub
Releases page — see "Distribution surface" below) — never the Mac App Store, never requiring
Apple Developer Program enrollment.

## What is explicitly NOT claimed

This amendment does not assert, and no repository artifact may assert, that an ad-hoc-signed,
unnotarized macOS build carries Apple's platform trust. Record truthfully, everywhere this
applies:

```text
MACOS_APPLE_PLATFORM_TRUST=NOT_CLAIMED
MACOS_GATEKEEPER_TRUST=NOT_CLAIMED
MACOS_NOTARIZATION=NOT_CLAIMED
MACOS_DEVELOPER_ID_SIGNATURE=NOT_CLAIMED
```

`spctl --assess` is expected, and must continue to be independently confirmed in CI, to
reject this artifact — that is Gatekeeper correctly doing its job against an unnotarized
binary, not a defect to hide or explain away. Users open the app via macOS's own supported
per-app override (System Settings → Privacy & Security → "Open Anyway", or right-click →
Open on the first launch) — see `docs/release/USER_GUIDE.md` §9 and
`docs/release/RELEASE_VERIFICATION.md`. This amendment does not recommend, and no
implementation under it may recommend, disabling Gatekeeper globally (`spctl --master-disable`)
or disabling macOS security system-wide. Only Apple's supported per-app user override path is
used.

## Historical evidence preserved

Nothing in `docs/evidence/flake-v1/T05-04/REPORT.md`'s prior addenda, `specs/CURRENT.md`'s
prior narrative history, or `docs/release/CODE_SIGNING_POLICY.md`'s prior text is deleted or
rewritten to look as if this were always the plan. The genuine external-blocker research (Apple's
enrollment terms as they stand, reconfirmed multiple times across this project's history) remains
true and preserved: a paid Apple Developer Program membership is still the only way to obtain
Developer ID signing and notarization. What changes is the Founder's product decision about
which release model Flake ships under, not the factual research about Apple's program terms.

## New macOS technical qualification (replaces the paid-Apple portion of T05-04)

The `T05-04` acceptance clause for macOS is amended to read: *the macOS release candidate
satisfies the zero-cost technical qualification below, with exact CI evidence, in place of
Developer ID signature/notarization/stapling verification.* Every other `T05-04` acceptance
clause (license/notice completeness, advisory refresh, build-script review, About/help
distribution content) is unaffected and unchanged.

Qualification checklist (see `docs/release/MACOS_DIRECT_DISTRIBUTION.md` for the full design
and exact CI evidence once run):

1. Reproducible, CI-controlled macOS build (GitHub-hosted `macos-latest` runner, pinned
   toolchain, same source `T05-03` already qualifies).
2. Native macOS launch and clean shutdown — already proven for the unsigned candidate by
   `T05-03`'s `scripts/release/install_test.sh` cross-platform run; reconfirmed against this
   specific ad-hoc-signed artifact.
3. Offline operation — already proven cross-platform (`T04-01`'s network-denied test,
   `T04-06`'s macOS-native static network-surface check, `T05-03`'s no-network-on-launch
   observation); reconfirmed against this specific artifact.
4. No required account/cloud service — unchanged product invariant (`src/about.rs`).
5. No unexpected application-controlled network traffic — same evidence basis as (3).
6. Package integrity — SHA-256 manifest, produced and verified byte-identical before/after
   signing.
7. SHA-256 checksum publication — `docs/release/RELEASE_VERIFICATION.md`.
8. GitHub artifact provenance/attestation — `actions/attest-build-provenance@v2`, extended to
   the macOS `.dmg`, in the same least-privilege, `workflow_dispatch`-only posture as the
   existing CLI-archive attestation workflow.
9. Project-controlled cryptographic signature for the downloadable artifact and its checksum
   manifest — the existing Flake release-signing GPG identity
   (`F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`, already qualified for Linux artifacts in
   `T05-04`), extended to sign the macOS `.dmg` and its manifest. This is a
   project-provenance signature, not an Apple platform-trust signature; it is documented as
   exactly that everywhere it appears.
10. Independent verification instructions — `docs/release/RELEASE_VERIFICATION.md`.
11. Malware/static/dependency/security checks already required by Flake — unchanged
    (`cargo audit`, build-script review, `cargo clippy -D warnings`), reconfirmed for this
    artifact.
12. Install/copy/uninstall flow — already proven for macOS by `T05-03`'s
    `install_test.sh` (DMG mount, `.app` copy, launch, remove); reconfirmed against this
    specific ad-hoc-signed artifact.
13. Upgrade/rollback qualification — out of `T05-04`'s own scope per the canonical plan
    (`T05-04`'s durability gate: "Signing touches packages, not vaults"); this is `T05-05`'s
    independent-reproduction scope, unaffected by this amendment.
14. Explicit documentation of Gatekeeper behavior — `docs/release/MACOS_DIRECT_DISTRIBUTION.md`,
    `docs/release/CODE_SIGNING_POLICY.md`.
15. User-facing instructions for opening an unnotarized build — `docs/release/USER_GUIDE.md`
    §9, `docs/release/RELEASE_VERIFICATION.md`.

Expected resulting state once the qualification workflow has actually run green in CI (not
before — no field below may read `PASS` on the strength of this document alone):

```text
MACOS_DISTRIBUTION_MODE=DIRECT_WEBSITE
MACOS_DEVELOPER_ID_REQUIRED=NO
MACOS_NOTARIZATION_REQUIRED=NO
MACOS_GATEKEEPER_TRUST=NOT_CLAIMED
MACOS_ZERO_COST_TECHNICAL_QUALIFICATION=PASS
```

## Distribution surface

Flake does not operate separate website infrastructure and this amendment does not create any.
"Website-first distribution" is implemented as this repository's own README, `docs/release/`
pages, and — once a version is actually published (a distinct, separately Founder-authorized
action under plan section 25: "Do not purchase credentials or publish remotely without actual
authority") — its GitHub Releases page. Inventing separate hosted web infrastructure as a new
canonical distribution channel would be a further Class C/E change beyond what this amendment
authorizes; it is out of scope here.

## Optional web/PWA path

Investigated and explicitly deferred. Flake's core invariants (Rust-owned canonical
correctness, local-first offline operation, no required account/cloud service, no mandatory
network platform) are not something a browser-hosted PWA can provide without either (a)
shipping Core's correctness/security logic to WebAssembly and re-qualifying every durability
and security gate against it from scratch, or (b) depending on a remote service for canonical
storage, which would contradict the local-first/data-ownership invariants outright. Neither
option is a small extension of the existing native architecture; both would be a new Class E
architecture decision requiring its own founder-authorized ADR and full re-qualification against
sections 10/38's invariants, not a byproduct of closing `T05-04`'s macOS signing subscope. No
PWA work is started under this amendment, and none of it blocks or delays the native
direct-download macOS release path.

## Authority

This amendment governs `T05-04` and any later task whose acceptance criteria reference macOS
signing/notarization. It does not authorize skipping any other acceptance clause, evidence
requirement, or cross-platform gate in the canonical plan.
