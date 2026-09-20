# CURRENT — Flake v1 Execution Frontier

**Purpose:** one authoritative pointer for what repository work may happen now.

> This file is operational state, not historical evidence. Live GitHub truth and the canonical build plan control execution.

## Current frontier

```text
PRODUCT_IDENTITY=FLAKE
ASTRO_PLAN_COMPLETE=YES
CANONICAL_BUILD_PLAN=docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md
CANONICAL_BUILD_PLAN_SHA256=b555f83ff12882ae6f55f90bbeaa411de52b84661a7f3953350cbfc6bd789fb2
CANONICAL_PLAN_REMOTE_STATUS=MIGRATED_TO_GITHUB
CANONICAL_PLAN_LOCAL_DEPENDENCY=NONE
ACTIVE_IMPLEMENTATION_UNIT=T05-04
NEXT_DEPENDENCY_READY_UNIT=T05-04
FOUNDER_DECISION_NO_HUMAN_GATES=docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md
FOUNDER_DECISION_WEBVIEW2_NETWORK_BOUNDARY=docs/canonical/FOUNDER_WEBVIEW2_NETWORK_BOUNDARY_2026-09-16.md
FOUNDER_DECISION_T04-06_ACCESSIBILITY_WITNESS=docs/canonical/FOUNDER_T04-06_ACCESSIBILITY_WITNESS_AMENDMENT_2026-09-16.md
FOUNDER_DECISION_ZERO_COST_MACOS_DISTRIBUTION=docs/canonical/FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md
T03-08_EXECUTION_CONTRACT=AUTOMATED_CONTINUITY_QUALIFICATION
T05-06_EXECUTION_CONTRACT=AUTOMATED_LONG_HORIZON_CONTINUITY_SOAK
R11_CONTRACT=AUTOMATED_CONTINUITY_AND_SOAK
T04-06_EXECUTION_CONTRACT=AUTOMATED_TECHNICAL_ACCESSIBILITY_QUALIFICATION
T04-06_HUMAN_ACCESSIBILITY_REVIEW_REQUIRED=NO
T04-06_NATIVE_PLATFORM_PROFILES_REQUIRED=WINDOWS,MACOS,LINUX
MACOS_DEVELOPER_ID_REQUIRED=NO
MACOS_NOTARIZATION_REQUIRED=NO
MACOS_DISTRIBUTION_MODE=DIRECT_WEBSITE
MACOS_GATEKEEPER_TRUST=NOT_CLAIMED
EXECUTABLE_REPOSITORY_WORK=ZERO
EXECUTABLE_REPOSITORY_WORK_BLOCKER_1=WINDOWS_SIGNPATH_APPROVAL (Founder/external action only -- the sole remaining T05-04 blocker; macOS zero-cost qualification closed PASS, see T05-04_MACOS_SIGNING_STATUS)
P03_STATUS=CLOSED
T04-01_STATUS=COMPLETE
T04-01_EVIDENCE=docs/evidence/flake-v1/T04-01/REPORT.md
T04-01_NETWORK_DENIED_TEST=docs/evidence/flake-v1/T04-01/network-denied-test/README.md
T04-01_PRIOR_BLOCKER_ID=T04-01-WEBVIEW2-BACKGROUND-NETWORK-TRAFFIC
T04-01_PRIOR_BLOCKER_STATUS=SUPERSEDED_BY_FOUNDER_DECISION
T04-01_MERGE_COMMIT=5a9a738cdfcd72505c45ac6f85b92ae4dcfc1ceb
T04-02_STATUS=COMPLETE
T04-02_EVIDENCE=docs/evidence/flake-v1/T04-02/REPORT.md
T04-02_MANUAL_CHECKLIST=docs/evidence/flake-v1/T04-02/MANUAL_COMPOSITION_CHECKLIST.md
T04-02_MERGE_COMMIT=ec3b1e7fec33df7068d982238b4f4af6fb8c907d
T04-03_STATUS=COMPLETE
T04-03_EVIDENCE=docs/evidence/flake-v1/T04-03/REPORT.md
T04-03_MERGE_COMMIT=bd3c3930ab235f00e385d72a1f3f3693b3a3e6b1
T04-04_STATUS=COMPLETE
T04-04_EVIDENCE=docs/evidence/flake-v1/T04-04/REPORT.md
T04-04_MERGE_COMMIT=7eb19a2275fd85b4059a9fc7660175db08e5d1b3
T04-05_STATUS=COMPLETE
T04-05_EVIDENCE=docs/evidence/flake-v1/T04-05/REPORT.md
T04-05_MERGE_COMMIT=0c8dceba29b523adbfb166135241a10202717df6
P04_STATUS=CLOSED
T04-06_STATUS=COMPLETE
T04-06_EVIDENCE=docs/evidence/flake-v1/T04-06/REPORT.md
T04-06_CROSS_PLATFORM_CI_RUN=35066083071
T04-06_MERGE_COMMIT=6538dd974575a0fe0e8613b6fda86220479c2be6
T05-01_STATUS=COMPLETE
T05-01_EVIDENCE=docs/evidence/flake-v1/T05-01/REPORT.md
T05-01_CROSS_PLATFORM_CI_RUN=35081331572
T05-01_MERGE_COMMIT=b0bdfafe2d78d9db5b97b60a511bfa0d98b3cf32
FOUNDER_DECISION_T05-02_PHYSICAL_POWER_LOSS=docs/canonical/FOUNDER_T05-02_PHYSICAL_POWER_LOSS_AMENDMENT_2026-09-16.md
T05-02_D6_PHYSICAL_TRIALS_REQUIRED=NO
T05-02_D6_VM_CYCLES_REQUIRED=YES
T05-02_STATUS=COMPLETE
T05-02_EVIDENCE=docs/evidence/flake-v1/T05-02/REPORT.md
T05-02_D6_LINUX_CYCLES=30 (local WSL2/KVM) + 8 (CI ubuntu-latest)
T05-02_MERGE_COMMIT=9bc94895109a364370f131ea8359b0d42a03a55f
T05-03_STATUS=COMPLETE
T05-03_EVIDENCE=docs/evidence/flake-v1/T05-03/REPORT.md
T05-03_CROSS_PLATFORM_CI_RUN=35182353415
T05-03_MERGE_COMMIT=f4e919e7627c3070898ffce358e2c743b2105183
T05-03_WINDOWS_SIGNING_CREDENTIALS=UNAVAILABLE
T05-03_MACOS_DEVELOPER_ID=UNAVAILABLE
T05-03_MACOS_NOTARIZATION_CREDENTIALS=UNAVAILABLE
T05-03_LINUX_RELEASE_SIGNING_KEY=UNAVAILABLE
T05-03_SIGNING_BLOCKED_TASK=T05-04
T05-04_STATUS=IN_PROGRESS
T05-04_EVIDENCE=docs/evidence/flake-v1/T05-04/REPORT.md
T05-04_LICENSE_NOTICE=COMPLETE
T05-04_THIRD_PARTY_ATTRIBUTION=COMPLETE (361 components -- corrected from 457, see docs/evidence/flake-v1/T05-04/REPORT.md addendum)
T05-04_ADVISORY_REFRESH=COMPLETE (root: 0 findings; desktop: 0 vulnerabilities, 7 non-blocking unmaintained/unsound warnings)
T05-04_BUILD_SCRIPT_REVIEW=COMPLETE
T05-04_ABOUT_HELP_DISTRIBUTION=COMPLETE
T05-04_ABOUT_HELP_DISTRIBUTION_CI_RUN=35211292367
T05-04_ABOUT_HELP_DISTRIBUTION_MERGE_COMMIT=87b3b73a4cd2fec0e0580fcfa8e375af0417e891
T05-04_WINDOWS_SIGNING_CREDENTIALS=UNAVAILABLE
T05-04_MACOS_DEVELOPER_ID=NOT_REQUIRED (Founder decision, FOUNDER_DECISION_ZERO_COST_MACOS_DISTRIBUTION above)
T05-04_MACOS_NOTARIZATION_CREDENTIALS=NOT_REQUIRED (Founder decision, FOUNDER_DECISION_ZERO_COST_MACOS_DISTRIBUTION above)
T05-04_LINUX_RELEASE_SIGNING_KEY=GENERATED_FOUNDER_HELD_NOT_YET_INJECTED_TO_CI
T05-04_BLOCKED_SUBSCOPE=Windows Authenticode signing only -- pending SignPath Foundation external application/approval, the one remaining external/Founder-only action. Linux release-signing (PASS) and macOS zero-cost direct distribution (PASS) are closed.
T05-04_SIGNING_MECHANICS_TEST=COMPLETE (TEST_SIGNING_IDENTITY_ONLY=YES, disposable identity per platform -- production signature clause remains blocked, see T05-04_BLOCKED_SUBSCOPE above)
T05-04_SIGNING_MECHANICS_CI_RUN=35294005232
T05-04_SIGNING_MECHANICS_MERGE_COMMIT=c21db04823e3b67764f806f61b21ae95826a711b
T05-04_WINDOWS_SIGNING_STATUS=PENDING_SIGNPATH_EXTERNAL_APPROVAL
T05-04_LINUX_SIGNING_STATUS=PASS
T05-04_LINUX_PRODUCTION_SIGNING_WORKFLOW=.github/workflows/t05-04-linux-production-signing.yml
T05-04_LINUX_PRODUCTION_SIGNING_WORKFLOW_MERGE_COMMIT=bcea246f7cc84ca19477800622da46a71a92ff9e
T05-04_LINUX_PRODUCTION_SIGNING_CI_RUN=35498327004
T05-04_LINUX_PRODUCTION_SIGNING_EVIDENCE=docs/evidence/flake-v1/T05-04/REPORT.md (addendum: "Real Linux production signing qualification (PASS)")
T05-04_MACOS_SIGNING_STATUS=PASS
T05-04_MACOS_ZERO_COST_TECHNICAL_QUALIFICATION=PASS
T05-04_MACOS_DIRECT_DISTRIBUTION_DESIGN=docs/release/MACOS_DIRECT_DISTRIBUTION.md
T05-04_MACOS_DIRECT_DISTRIBUTION_WORKFLOW=.github/workflows/t05-04-macos-direct-distribution.yml
T05-04_MACOS_DIRECT_DISTRIBUTION_WORKFLOW_MERGE_COMMIT=a85f906fa579f611c4c05f6464f955805cd8fd32
T05-04_MACOS_DIRECT_DISTRIBUTION_CI_RUN=35514419522
T05-04_MACOS_DIRECT_DISTRIBUTION_EVIDENCE=docs/evidence/flake-v1/T05-04/REPORT.md (addendum: "zero-Apple-fee macOS direct distribution (PASS)")
T05-04_MACOS_DEVELOPER_ID_REQUIRED=NO
T05-04_MACOS_NOTARIZATION_REQUIRED=NO
T05-04_MACOS_GATEKEEPER_TRUST=NOT_CLAIMED
T05-04_SIGNPATH_ELIGIBILITY_PACKET=docs/release/SIGNPATH_ELIGIBILITY_PACKET.md
T05-04_LINUX_SIGNING_DESIGN=docs/release/LINUX_RELEASE_SIGNING.md
T05-04_LINUX_SIGNING_IDENTITY_DECISION=docs/canonical/FOUNDER_RELEASE_SIGNING_IDENTITY_DECISION_2026-09-18.md
T05-04_CODE_SIGNING_POLICY=docs/release/CODE_SIGNING_POLICY.md
T05-04_RELEASE_VERIFICATION_DOC=docs/release/RELEASE_VERIFICATION.md
T05-04_GITHUB_ARTIFACT_ATTESTATIONS=ENABLED (.github/workflows/release-provenance-attestation.yml, workflow_dispatch-only)
T05-04_SIGNING_QUALIFICATION_CI_RUN=35305041091
T05-04_SIGNING_QUALIFICATION_MERGE_COMMIT=c20a5d773b830e250c401cc734af71a6530256a6
T05-04_LINUX_SIGNING_IDENTITY_FINGERPRINT=F779807C73F29F4DB1E7DC9F78F7D4B92287FE22
T05-04_LINUX_SIGNING_IDENTITY_PUBLIC_KEY=docs/release/flake-release-signing-public.asc
T05-04_LINUX_SIGNING_IDENTITY_PUBLISHED_CI_RUN=35320964978
T05-04_LINUX_SIGNING_IDENTITY_PUBLISHED_MERGE_COMMIT=aba1b4abc495c24822e617cf0ee7cec18f76b4b1
P01_STATUS=CLOSED
T00-01_STATUS=COMPLETE
T00-01_EVIDENCE=docs/evidence/flake-v1/T00-01/REPORT.md
T00-01_MERGE_COMMIT=6389f6512ea0feb90fd2da7dcdbde6f442e7eb29
T00-02_STATUS=COMPLETE
T00-02_EVIDENCE=docs/evidence/flake-v1/T00-02/REPORT.md
T00-02_SPEC_KIT=specs/002-post-r1-canonical-core-convergence/corrective-t01/
T00-02_MERGE_COMMIT=7c4a5a6521676effa1d5151bec96c271daa22aae
T01-01_STATUS=COMPLETE
T01-01_EVIDENCE=docs/evidence/flake-v1/T01-01/REPORT.md
T01-01_MERGE_COMMIT=0ca5408a324da075f3a868928296bca910644cb5
T01-02_STATUS=COMPLETE
T01-02_EVIDENCE=docs/evidence/flake-v1/T01-02/REPORT.md
T01-02_FORMAT_DOC=docs/formats/format-2-canonical-sqlite.md
T01-02_MERGE_COMMIT=fdc15c42f1fde706336d1f9b0d7b5faadd83fa92
T01-03_STATUS=COMPLETE
T01-03_EVIDENCE=docs/evidence/flake-v1/T01-03/REPORT.md
T01-03_FORMAT_DOC=docs/formats/format-2-canonical-sqlite.md
T01-03_MERGE_COMMIT=b59d39edf92c3214f68903f5af4977a03c1346fa
T01-04_STATUS=COMPLETE
T01-04_EVIDENCE=docs/evidence/flake-v1/T01-04/REPORT.md
T01-04_FORMAT_DOC=docs/formats/format-2-canonical-sqlite.md
T01-04_MERGE_COMMIT=b9aaf88f89db96d54ecfcdffb763d925fed40c50
T01-05_STATUS=COMPLETE
T01-05_EVIDENCE=docs/evidence/flake-v1/T01-05/REPORT.md
T01-05_MERGE_COMMIT=3e1e30cba341e3a1301e0af646e37f771debb2c7
T01-06_STATUS=COMPLETE
T01-06_EVIDENCE=docs/evidence/flake-v1/T01-06/REPORT.md
T01-06_FORMAT_DOC=docs/formats/legacy-migration.md
T01-06_MERGE_COMMIT=7804f202921c3bb0f7b1dcce8b8db971c6e55ea6
T01-07_STATUS=COMPLETE
T01-07_EVIDENCE=docs/evidence/flake-v1/T01-07/REPORT.md
T01-07_MUTATOR_AUDIT=docs/evidence/flake-v1/T01-07/mutator-audit.md
T01-07_MERGE_COMMIT=57de9bbd77bfec64a73c8c654a6d9d1459802d0c
T02-01_STATUS=COMPLETE
T02-01_EVIDENCE=docs/evidence/flake-v1/T02-01/REPORT.md
T02-01_FORMAT_DOC=docs/formats/typed-records.md
T02-01_MERGE_COMMIT=06818675b6bc4b5ab2904e1f5e827c028da3586b
T02-02_STATUS=COMPLETE
T02-02_EVIDENCE=docs/evidence/flake-v1/T02-02/REPORT.md
T02-02_FORMAT_DOC=docs/formats/typed-records.md
T02-02_MERGE_COMMIT=bfff06c96917726ffff00881afc09c93dcd22599
T02-03_STATUS=COMPLETE
T02-03_EVIDENCE=docs/evidence/flake-v1/T02-03/REPORT.md
T02-03_MERGE_COMMIT=e8a285d2672fb6093125739f4307675ebc08195a
T02-04_STATUS=COMPLETE
T02-04_EVIDENCE=docs/evidence/flake-v1/T02-04/REPORT.md
T02-04_MERGE_COMMIT=dd7ff40bec97d14d966ec1c6e9bd993e351bd366
T02-05_STATUS=COMPLETE
T02-05_EVIDENCE=docs/evidence/flake-v1/T02-05/REPORT.md
T02-05_FORMAT_DOC=docs/formats/portable-export-v1.md
T02-05_MERGE_COMMIT=a92dce62eca038cd643620bf158175f916acf394
T02-06_STATUS=COMPLETE
T02-06_EVIDENCE=docs/evidence/flake-v1/T02-06/REPORT.md
T02-06_MERGE_COMMIT=dc2f1c16f8331be873937b6b6bc39888eedda9c1
T02-07_STATUS=COMPLETE
T02-07_EVIDENCE=docs/evidence/flake-v1/T02-07/REPORT.md
T02-07_MERGE_COMMIT=e7d9445b87f48c99ad2ed0f59b32194255520af3
T03-01_STATUS=COMPLETE
T03-01_EVIDENCE=docs/evidence/flake-v1/T03-01/REPORT.md
T03-01_MERGE_COMMIT=07694d572c295940e933f51fd72cfeddffb92e6d
T03-02_STATUS=COMPLETE
T03-02_EVIDENCE=docs/evidence/flake-v1/T03-02/REPORT.md
T03-02_MERGE_COMMIT=4325e505f31dc5bb5ae47282ea390b1bd623732e
T03-03_STATUS=COMPLETE
T03-03_EVIDENCE=docs/evidence/flake-v1/T03-03/REPORT.md
T03-03_MERGE_COMMIT=615e5f75e19437e49cc7883000fa4825ff6badc1
T03-04_STATUS=COMPLETE
T03-04_EVIDENCE=docs/evidence/flake-v1/T03-04/REPORT.md
T03-04_MERGE_COMMIT=8acbbf95399dc698750b8b183107a1ba45191615
T03-05_STATUS=COMPLETE
T03-05_EVIDENCE=docs/evidence/flake-v1/T03-05/REPORT.md
T03-05_MERGE_COMMIT=5799539346e32047ef70dec7c2f9ec37ac4ff339
T03-06_STATUS=COMPLETE
T03-06_EVIDENCE=docs/evidence/flake-v1/T03-06/REPORT.md
T03-06_MERGE_COMMIT=29263249e17fbaf93bd6a2d764fed418da255feb
T03-07_STATUS=COMPLETE
T03-07_EVIDENCE=docs/evidence/flake-v1/T03-07/REPORT.md
T03-07_PREREG_PACKAGE=bench/flake-v1/T03-07/
T03-07_SEAL=bench/flake-v1/T03-07/SEALS.json
T03-07_MERGE_COMMIT=4b7f56b74a9e7b8ce074a8b682b06c3d78772716
T03-08_STATUS=COMPLETE
T03-08_EVIDENCE=docs/evidence/flake-v1/T03-08/REPORT.md
T03-08_ROUTE=PASS
T03-08_PRIOR_BLOCKER_ID=T03-08-HUMAN-PARTICIPANTS
T03-08_PRIOR_BLOCKER_STATUS=SUPERSEDED_BY_FOUNDER_DECISION
T03-08_MERGE_COMMIT=c5f1f1ad3ca5a90129660ede814291b9e7e909b2
SPEC_003_AUTO_ACTIVATION=PROHIBITED
PROJECT_COMPLETE=NO
```

## Authority

The sole implementation roadmap is `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md`.
`docs/canonical/FLAKE_MUSE_EXECUTION_HANDOFF.md` is the short execution entry point.

Historical Fehrest, Phase T, R1, and Spec 002 artifacts remain immutable evidence. They are not the active product roadmap and must not be rewritten to make their historical identifiers match later GitHub history.

The historical local-only planning SHAs `4246f6d...` and `852e44b...` are provenance references only. Execution does not require those Git objects or any OneDrive/local path; `docs/evidence/flake-v1/T00-01/REPORT.md` records exactly how a pre-existing local branch carrying those identifiers was reconciled (not adopted as-is) against live GitHub truth.

## Next action

**macOS zero-cost direct distribution qualification: real CI run, PASS.** After merging PR #114
(the amendment and implementation, below), this session ran
`.github/workflows/t05-04-macos-direct-distribution.yml` once directly on `main` at merge commit
`a85f906fa579f611c4c05f6464f955805cd8fd32`: CI run
[`35514419522`](https://github.com/TheHalfMoon/Flake/actions/runs/35514419522), conclusion
`success` in 5m32s. Independently confirmed from that run's own log (not merely the green
checkmark): the sanity gate held (default/production signing mode still fails closed without
Developer ID credentials); `codesign --verify` passed and `spctl --assess` correctly rejected the
ad-hoc-signed, unnotarized `.dmg` (`SPCTL_EXIT=3`); the project GPG signature
(`F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`, the same identity already qualified for Linux) was
independently re-verified against both the `.dmg` and its checksum manifest in a clean keyring
seeded only from the published public key; a GitHub build-provenance attestation was created for
the exact signed artifact digest; and `install_test.sh` reconfirmed install/launch, bundled
license files, and vault retention against this specific signed artifact. Full evidence:
`docs/evidence/flake-v1/T05-04/REPORT.md`'s "zero-Apple-fee macOS direct distribution (PASS)"
addendum. `T05-04_MACOS_SIGNING_STATUS=PASS` and `T05-04_MACOS_ZERO_COST_TECHNICAL_QUALIFICATION=PASS`
above reflect this real, independently-checked run — never asserted from the amendment's design
document alone. `T05-04` remains `IN_PROGRESS`: Windows (`PENDING_SIGNPATH_EXTERNAL_APPROVAL`) is
now the *only* remaining blocker, and it is a Founder/external action, not repository work —
submit the real SignPath Foundation application at `signpath.org/apply` (real applicant identity,
MFA-enrolled account) and await SignPath's own review/approval; nothing else in this repository's
own DAG is dependency-ready, so `T05-05` correctly stays not dependency-ready and
`PROJECT_COMPLETE=NO`.

**Founder governance amendment: zero-Apple-fee macOS direct distribution (2026-09-20).** The
Founder ruled, in `docs/canonical/FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md`,
that Flake will not purchase Apple Developer Program membership and will not distribute
through the Mac App Store; `T05-04`'s macOS signing/notarization/stapling acceptance clause is
amended, prospectively and for macOS only, to a zero-cost technical qualification (ad-hoc
codesign of the actual distributed artifact, SHA-256 checksum, the same project GPG
release-signing identity already qualified for Linux, and a GitHub build-provenance
attestation — never a claim of Apple Developer ID, notarization, or Gatekeeper trust). This
session implemented `scripts/release/sign_macos.sh`'s new `DIRECT_DISTRIBUTION_MODE=1`
(distinct from the existing pipeline-mechanics-only `TEST_SIGNING_MODE=1`),
`.github/workflows/t05-04-macos-direct-distribution.yml` (`workflow_dispatch`-only, mirroring
`t05-04-linux-production-signing.yml`'s structure), `docs/release/MACOS_DIRECT_DISTRIBUTION.md`
(full design and the exact per-app Gatekeeper override instructions — never a system-wide
Gatekeeper-disable recommendation), and updated `docs/release/CODE_SIGNING_POLICY.md`,
`docs/release/RELEASE_VERIFICATION.md`, `docs/release/USER_GUIDE.md`, and `README.md`'s new
"Download Flake" section accordingly. `T05-04_MACOS_SIGNING_STATUS=PENDING_ZERO_COST_QUALIFICATION_CI_RUN`
above reflects that this is implemented but not yet independently proven in CI with real
evidence — per `AGENTS.md` §6 ("Never claim PASS ... without exact evidence"), this field will
not read `PASS` until the workflow has actually run green and been independently checked, not
on the strength of this design alone. "Website-first distribution" is implemented as this
repository's own README/docs/GitHub-Releases surface, not new hosted infrastructure (no
canonical task authorizes building one); an optional web/PWA path was investigated and
explicitly deferred (would require its own Class E founder-authorized ADR and full
re-qualification against sections 10/38's local-first/offline/no-required-network invariants —
not a byproduct of closing this signing subscope), and does not block this native
direct-download release. Windows remains `PENDING_SIGNPATH_EXTERNAL_APPROVAL` (Founder/external
action, unaffected by this amendment) as the one remaining blocker once macOS's CI run is
recorded.

**Frontier reverified (superseded by the amendment above, preserved for history): zero repository-executable work remains, three genuine external blockers stand between `T05-04` and close.** After merging PR #109 (Linux signing-identity publication) and PR #110 (its pointer follow-up), this session re-read `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` section 31's dependency DAG ("The graph is an intentionally serial topological chain ... No task is independent of the preceding phase exit") and confirmed `T05-05`'s own contract lists `T05-04` as its sole dependency. `T05-04`'s three signing subscopes are each now blocked on one external, non-repository action, not on any further design, CI, code, or documentation work this session (or any agent session) can perform:

1. **`WINDOWS_SIGNPATH_APPROVAL`** -- `docs/release/SIGNPATH_ELIGIBILITY_PACKET.md` and `docs/release/CODE_SIGNING_POLICY.md` are published and every self-certifiable SignPath Foundation criterion is met; the remaining action is the Founder submitting the real SignPath Foundation application (real applicant identity, MFA-enrolled account) and SignPath's own review/approval, which cannot be fabricated or completed by repository automation.
2. **`MACOS_APPLE_DEVELOPER_ID_NOTARIZATION`** -- reconfirmed against Apple's current enrollment terms (`developer.apple.com/programs/`): Developer ID and notarization require an active, paid Apple Developer Program membership (US $99/year) with no free/OSS exception. No ad-hoc signing, GPG, SignPath, or GitHub attestation substitutes for it.
3. **`LINUX_PRODUCTION_KEY_CI_INJECTION_AND_ARTIFACT_SIGNING`** -- the production GPG key and its public fingerprint (`F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`) are generated and published, but `gh secret list` on this repository returns empty: the private key has not been injected into the `GPG_PRIVATE_KEY` CI secret `scripts/release/sign_linux.sh` already supports, and no real release-candidate artifact has been signed or independently verified with it. This is a Founder-only action, on the machine already holding the key; per the key-generation runbook's own instruction, the key must never be generated or moved inside an agent tool-call transcript, and no session on this Windows host (nor any reachable teammate/agent session) has access to it.

`T05-04_STATUS=IN_PROGRESS` and `EXECUTABLE_REPOSITORY_WORK=ZERO` accurately reflect this: nothing in the repository's own DAG is dependency-ready, `T05-05` stays correctly unstarted, and no forbidden prep work on it or on the unrelated, still-open Fehrest V2 proposal PRs (`#2`, `#40` -- explicitly out of scope; `SPEC_003_AUTO_ACTIVATION=PROHIBITED` above) was performed. `PROJECT_COMPLETE=NO`.

**Linux production signing identity published (follow-up), production signature still not qualified:** the Founder ran the one-time local key-generation runbook from `docs/release/LINUX_RELEASE_SIGNING.md` §2 outside any agent session on 2026-09-18, then this session verified and merged PR #109 (`docs(release): publish Linux production signing identity`, merge commit `aba1b4abc495c24822e617cf0ee7cec18f76b4b1`, all 24 checks green including `linux-release-signing-test`/`verify-artifacts`/CI run `35320964978` and its sibling matrix runs) publishing `docs/release/flake-release-signing-public.asc` and updating `docs/release/CODE_SIGNING_POLICY.md`/`docs/release/LINUX_RELEASE_SIGNING.md`/`docs/release/RELEASE_VERIFICATION.md` with the real fingerprint. Before merging, this session independently re-derived the fingerprint from the published public key in a clean, disposable GnuPG keyring (not merely trusted from the PR body): `F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`, uid `Flake Release Signing <285091250+TheHalfMoon@users.noreply.github.com>`, matching `docs/canonical/FOUNDER_RELEASE_SIGNING_IDENTITY_DECISION_2026-09-18.md` exactly; the diff was confirmed to contain only public-key/documentation content, no private key, passphrase, or secret value. **This does not close `T05-04_LINUX_SIGNING_STATUS`.** `gh secret list` on the repository returns empty -- the production private key exists only where the Founder generated it and has not been injected into the `GPG_PRIVATE_KEY` CI secret `scripts/release/sign_linux.sh` already supports, and no real release artifact has been signed or independently verified with it yet. Per the key-generation runbook's own instruction not to place long-lived private key material in any agent tool-call transcript, this session (running on Windows, with no access to the environment holding the real key) did not attempt to fabricate, request, or move that key. `T05-04_LINUX_SIGNING_STATUS=PENDING_PRODUCTION_ARTIFACT_SIGNATURE_QUALIFICATION`: the remaining action is Founder-run, on the machine already holding the key -- inject it into the `GPG_PRIVATE_KEY` (and `GPG_KEY_FINGERPRINT`/`GPG_KEY_PASSPHRASE` if used) repository secrets, sign a real release-candidate artifact, and independently verify the signature against `docs/release/flake-release-signing-public.asc` in a clean keyring before this field can read `PASS`. `T05-04` remains `IN_PROGRESS`; with Windows still `PENDING_SIGNPATH_EXTERNAL_APPROVAL` and macOS still `BLOCKED_EXTERNAL_APPLE_CREDENTIALS`, all three signing subscopes now wait on an external/Founder-only action rather than repository-executable work, so `T05-05` remains not dependency-ready under the plan's own sequential-DAG discipline.

**Free/OSS signing qualification maximized (follow-up), blocker set narrowed from WINDOWS+MACOS+LINUX to effectively MACOS-only:** per an explicit Founder decision ("maximize free release qualification... do not stop at the existing production-signing blockers"), each of the three T05-04 signing blockers was reinvestigated individually against authoritative current external documentation, not re-asserted from the prior pass. **Windows:** researched SignPath Foundation's actual published eligibility terms (`signpath.org/terms.html`, `/apply`, `docs.signpath.io/trusted-build-systems/github`) and checked Flake against every stated condition point by point -- `docs/release/SIGNPATH_ELIGIBILITY_PACKET.md`. Flake meets every self-certifiable criterion (OSI-approved Apache-2.0 license, no proprietary components, actively maintained, functionality documented, single maintainer owns the repository, artifacts only ever built from its own source); the one criterion requiring SignPath's own review ("already released in the form that should be signed") and the application itself (real applicant identity, MFA-enrolled accounts, organization id/project slug/signing policy slug/API token issued only on approval) cannot be fabricated or completed by repository automation -- `WINDOWS_SIGNING_STATUS=PENDING_SIGNPATH_EXTERNAL_APPROVAL`, no longer a paid-certificate blocker. The required public code-signing-policy page (`docs/release/CODE_SIGNING_POLICY.md`, linked from `README.md`) and role documentation are published now, ahead of application, per SignPath's own terms. **Linux:** designed a full production GPG release-signing identity and lifecycle -- `docs/release/LINUX_RELEASE_SIGNING.md` (one-time key-generation runbook, CI secret-injection architecture, verification commands, rotation/revocation procedure) -- under an explicit Founder decision on the exact identity string (`docs/canonical/FOUNDER_RELEASE_SIGNING_IDENTITY_DECISION_2026-09-18.md`: `Flake Release Signing <285091250+TheHalfMoon@users.noreply.github.com>`, the GitHub-issued noreply alias for the repository-owning account, chosen over the Founder's personal email). `scripts/release/sign_linux.sh` now supports importing a production key from a `GPG_PRIVATE_KEY` secret into a fresh, ephemeral `GNUPGHOME` (never a persistent keyring), with optional `GPG_KEY_FINGERPRINT` pinning that fails closed on any mismatch and optional `GPG_KEY_PASSPHRASE` handling that never places the passphrase in argv or a log -- proven locally (fingerprint match/mismatch, passphrase-protected key, production-mode fail-closed with no credentials, all four exercised directly against a real `gpg` binary before any CI change) and proven live in CI by two new checks added to `linux-release-signing-test` in `.github/workflows/t05-04-signing-pipeline-test.yml`, fed a disposable throwaway key through that exact new mechanics (never a real secret). The real production private key was deliberately **not** generated in this session: doing so inside an agent tool-call transcript would place long-lived private key material in session logs, which the key-generation runbook itself forbids -- `LINUX_SIGNING_STATUS=PENDING_ONE_TIME_KEY_CREATION`, with every repository-owned prerequisite otherwise complete, not a paid-provider blocker. **macOS:** reconfirmed against Apple's own current enrollment documentation (`developer.apple.com/programs/`) that Developer ID and notarization still require an active, paid Apple Developer Program membership (US $99/year) with no free/open-source exception; this remains the one genuine, unavoidable external blocker -- `MACOS_SIGNING_STATUS=BLOCKED_EXTERNAL_APPLE_CREDENTIALS`, and no ad-hoc signing, GPG, SignPath, or GitHub attestation is claimed to substitute for it. **Additional supply-chain evidence, all platforms:** GitHub-native artifact attestations (`actions/attest-build-provenance@v2`) are now generated for release-candidate CLI archives and SBOMs via a new, deliberately `workflow_dispatch`-only workflow (`.github/workflows/release-provenance-attestation.yml`, least-privilege `id-token`/`attestations: write` scoped to only that one job, every other workflow's permissions unchanged) -- never wired to run automatically on every PR, since a public repository's attestations are permanent provenance records and plan section 25 reserves "publishing a release" as separately authorized beyond building verified candidates. `docs/release/RELEASE_VERIFICATION.md` consolidates checksum/attestation/Authenticode/GPG/notarization verification commands (the Windows and Linux sections marked not-yet-active pending the two Founder actions above) so this does not duplicate or contradict the platform-specific documents. `T05-04` remains `IN_PROGRESS` -- its "all signatures/notarization/stapling verify" clause is still not closed, and the plan's own sequential DAG still keeps `T05-05` not dependency-ready -- but the blocker set is now effectively macOS-only pending two minimal, already-scoped Founder actions (submit the real SignPath application; run the one-time local GPG key generation) rather than three unresolved credential gaps. This work merged as PR #107, merge commit `c20a5d773b830e250c401cc734af71a6530256a6`, all 24 CI checks green (CI run `35305041091` and its sibling matrix runs) after one real defect found and fixed by this same PR's own CI: `import_key_from_secret`'s `export GNUPGHOME` was made inside a command-substitution subshell, invisible to the calling script the instant that subshell exited, so the first CI attempt at the new secret-injection path failed with `gpg: signing failed: No secret key` -- fixed by creating/exporting the ephemeral `GNUPGHOME` at both call sites' top-level script scope before invoking the now-`GNUPGHOME`-agnostic import function, root-caused in `docs/evidence/flake-v1/T05-04/REPORT.md`'s addendum. Full details: that same addendum.

**Signing pipeline mechanics built and CI-proven (follow-up), production signature still blocked:** plan section 25's "Sign Windows payloads/installer, macOS app/notarize/staple, and Linux/checksum manifests" clause has an independently-executable half (the actual command construction, secret-injection interface, CI wiring, verification wiring, failure handling) and a credential-dependent half. `scripts/release/sign_windows.sh`/`sign_macos.sh`/`sign_linux.sh` wrap the real production commands (`signtool`; `codesign`+`notarytool`+`stapler`+`spctl`; `gpg`) behind an explicit `TEST_SIGNING_MODE=1` branch (disposable self-signed cert / ad-hoc identity / ephemeral GPG key, generated and destroyed per run) and a production branch proven, live in CI, to fail closed without the named credential env vars. `.github/workflows/t05-04-signing-pipeline-test.yml` runs all three against the exact `UNSIGNED_DEVELOPER_RC` artifacts T05-03 already produces, on all three native platforms, green (CI run `35294005232`, PR #104, merge commit `c21db04823e3b67764f806f61b21ae95826a711b`) after five real defects found and fixed by this same PR's own CI (lost executable bit from `core.filemode=false`; an unconverted MSYS path; `signtool`'s `/fd` flag order; Git Bash's argv path-mangling of bare `/flag`-style arguments eating `/fd` outright even correctly ordered, fixed with `MSYS_NO_PATHCONV=1`; and the disposable test certificate needing `LocalMachine\Root` trust, not `CurrentUser\Root`, for `signtool verify /pa`'s chain-trust check to pass) -- each recorded with its own root cause in `docs/evidence/flake-v1/T05-04/REPORT.md`'s addendum, not glossed over. Every test-mode run asserts `TEST_SIGNING_IDENTITY_ONLY=YES`/`PRODUCTION_SIGNATURE_CLAIMED=NO` (macOS also `NOTARIZATION_CLAIMED=NO`, with `spctl --assess` confirmed to correctly reject the unnotarized test artifact rather than false-passing it). This closes the entire independently-executable remainder of T05-04's signing subscope; it does not and cannot close T05-04's own acceptance clause, which still requires the exact credentials named in `T05-04_WINDOWS_SIGNING_CREDENTIALS`/`T05-04_MACOS_DEVELOPER_ID`/`T05-04_MACOS_NOTARIZATION_CREDENTIALS`/`T05-04_LINUX_RELEASE_SIGNING_KEY` above, still all `UNAVAILABLE`. `T05-05` remains not dependency-ready for the same reason recorded below, unchanged by this follow-up.

**Correction (follow-up):** the "457 shipped third-party components" figure in the paragraph immediately below was over-inclusive. An independent `cargo-about` cross-check (once disk headroom allowed installing it) found `generate_third_party_licenses.py`'s original unfiltered `cargo metadata` call was counting wasm32-only phantom dependencies (`wasm-bindgen`/`js-sys`/`r-efi`/etc., transitively pulled in by `getrandom`/`uuid` but never reachable for any of the three actually-shipped target triples). Fixed by filtering to exactly those three triples; the corrected, independently-verified count is **361** (root: 38, exactly matching `cargo-about`'s own count; desktop: 353). The four special-attention license buckets (Unicode/zlib/MPL-2.0/the two individually-noted components) are unchanged -- every removed entry was Apache-2.0/MIT-only, so no legally-relevant attribution was lost, only never-shipped phantom entries. `T05-04_THIRD_PARTY_ATTRIBUTION` above reflects the corrected count; this does not change T05-04's own completion assessment or its one remaining blocked acceptance clause (signing/notarization/stapling). Full details: `docs/evidence/flake-v1/T05-04/REPORT.md`'s addendum.

**`T05-04` is IN_PROGRESS and remains the active unit** -- every independently executable part of plan section 25/28's T05-04 row is now done: `LICENSE` (unmodified upstream Apache-2.0 text) and `NOTICE`/`docs/legal/THIRD-PARTY-LICENSES.md` (457 shipped third-party components across both Cargo projects plus desktop's shipped npm dependencies, every non-Apache-2.0-only license family -- Unicode License v3, zlib, MPL-2.0, ISC -- fully attributed with complete text, generated by the new reusable `scripts/release/generate_third_party_licenses.py`); a fresh `cargo audit` on both dependency graphs (root: 0 findings; desktop: 0 vulnerabilities, 7 non-blocking unmaintained/unsound warnings, recorded not hidden); a build-script review (only `cc`, for the already-reviewed bundled-SQLite compilation -- no `bindgen`/`cmake`/`cxx` anywhere in either graph); and, closed by a follow-up PR after this row was first reviewed, plan section 25's own UX-behavior clause "About/help/distribution include license, source, privacy and support/reporting route" (PR #102, merge commit `87b3b73a4cd2fec0e0580fcfa8e375af0417e891`, CI run `35211292367` green on Windows/macOS/Linux) -- a new `flake license` CLI command and desktop "About" screen (`src/about.rs`, one canonical fact module both surfaces read, so they cannot state diverging license/source/privacy/support text), plus actually bundling `LICENSE`/`NOTICE`/`THIRD-PARTY-LICENSES.md` into every distributed CLI archive and desktop installer (verified against the real installed bundle on all three platforms by a new `install_test.sh` check, not merely trusted from the bundler config). That PR's own CI caught and fixed two real defects along the way: an invalid `tauri.conf.json` field name (`nsis.license` does not exist in the pinned tauri-utils schema; the real fields are top-level `bundle.license`/`licenseFile`), and, after fixing that, a second real defect where setting `licenseFile` made Tauri's macOS DMG bundler embed a EULA that broke non-interactive `hdiutil attach` in the existing install-test script -- fixed by dropping `licenseFile` (the unrelated `resources` field, `bundle.license` SPDX string, CLI command, About screen and USER_GUIDE.md section together still fully close the clause without an EULA gate). `T05-04_WINDOWS_SIGNING_CREDENTIALS`/`T05-04_MACOS_DEVELOPER_ID`/`T05-04_MACOS_NOTARIZATION_CREDENTIALS`/`T05-04_LINUX_RELEASE_SIGNING_KEY` are all still `UNAVAILABLE` -- a genuine external blocker, not fabricated, and now the *only* remaining T05-04 acceptance clause. Per section 40's own sequential-DAG discipline ("Completion condition ... Otherwise remain at this task"), `T05-05` (which depends on `T05-04`, and whose own acceptance criteria explicitly requires "signatures/installations verify") is not dependency-ready until `T05-04` actually closes, so execution correctly remains parked at `T05-04` pending the signing/notarization/stapling credentials named above, not advanced past it. Full details and the exact blocker packet: `docs/evidence/flake-v1/T05-04/REPORT.md`.

`T05-03` closed: built and fully qualified every unsigned CLI/desktop release-candidate artifact producible without private signing credentials, across all three native platforms. Added the `flake` product-command binary (same `src/main.rs` as `fehrest`, proven functionally identical, not merely claimed, by `tests/flake_fehrest_alias_parity.rs`); closed a real deferred `T01-05` obligation by wiring `backup-run`/`backup-restore`/`vault-recover` CLI commands over the already-reviewed `crate::backup`/`crate::recovery` library functions (`tests/flake_cli_backup_recover.rs`); added a desktop default-vault-parent-directory suggestion; wrote a genuinely new user-facing `docs/release/USER_GUIDE.md` (the repository's own root `README.md` is developer-facing); and built the full packaging pipeline (`scripts/release/package_cli_archive.sh`, `generate_sbom.sh` via `cargo-cyclonedx`, `install_test.sh`, `reproducibility_check.sh`) plus `.github/workflows/t05-03-release-candidates.yml` tying it together across all three native platforms (CI run `35182353415`, green on Windows/macOS/Linux). A real defect was found and fixed during this task's own qualification, not merely observed: the Windows install-test job hung indefinitely because this shell (MSYS bash) silently rewrites a bare `/S` argument into a Windows drive-relative path (`S:/`) before NSIS ever sees it, so the installer's silent flag was never actually applied and it launched its interactive GUI wizard instead -- confirmed by capturing the installer's own live argv during the hang, not assumed; two other plausible theories (WebView2 elevation, Windows Defender scanning) were checked directly against the runner and ruled out first. Fixed with `//S` (MSYS's standard escape, already used elsewhere in the same script). Full details, including the exact `WINDOWS_SIGNING_CREDENTIALS=UNAVAILABLE`/`MACOS_DEVELOPER_ID=UNAVAILABLE`/`MACOS_NOTARIZATION_CREDENTIALS=UNAVAILABLE`/`LINUX_RELEASE_SIGNING_KEY=UNAVAILABLE` blocker fields recorded for `T05-04`, the full hang investigation, and honest limitations on "update"/"rollback" testing (no distinct second historical release exists yet to upgrade from) and the Linux offline-dependency-bundle clause (not yet built): `docs/evidence/flake-v1/T05-03/REPORT.md`.

`T05-03` closed: built and fully qualified every unsigned CLI/desktop release-candidate artifact producible without private signing credentials, across all three native platforms. Added the `flake` product-command binary (same `src/main.rs` as `fehrest`, proven functionally identical, not merely claimed, by `tests/flake_fehrest_alias_parity.rs`); closed a real deferred `T01-05` obligation by wiring `backup-run`/`backup-restore`/`vault-recover` CLI commands over the already-reviewed `crate::backup`/`crate::recovery` library functions (`tests/flake_cli_backup_recover.rs`); added a desktop default-vault-parent-directory suggestion; wrote a genuinely new user-facing `docs/release/USER_GUIDE.md` (the repository's own root `README.md` is developer-facing); and built the full packaging pipeline (`scripts/release/package_cli_archive.sh`, `generate_sbom.sh` via `cargo-cyclonedx`, `install_test.sh`, `reproducibility_check.sh`) plus `.github/workflows/t05-03-release-candidates.yml` tying it together across all three native platforms (CI run `35182353415`, green on Windows/macOS/Linux). A real defect was found and fixed during this task's own qualification, not merely observed: the Windows install-test job hung indefinitely because this shell (MSYS bash) silently rewrites a bare `/S` argument into a Windows drive-relative path (`S:/`) before NSIS ever sees it, so the installer's silent flag was never actually applied and it launched its interactive GUI wizard instead -- confirmed by capturing the installer's own live argv during the hang, not assumed; two other plausible theories (WebView2 elevation, Windows Defender scanning) were checked directly against the runner and ruled out first. Fixed with `//S` (MSYS's standard escape, already used elsewhere in the same script). Full details, including the exact `WINDOWS_SIGNING_CREDENTIALS=UNAVAILABLE`/`MACOS_DEVELOPER_ID=UNAVAILABLE`/`MACOS_NOTARIZATION_CREDENTIALS=UNAVAILABLE`/`LINUX_RELEASE_SIGNING_KEY=UNAVAILABLE` blocker fields recorded for `T05-04`, the full hang investigation, and honest limitations on "update"/"rollback" testing (no distinct second historical release exists yet to upgrade from) and the Linux offline-dependency-bundle clause (not yet built): `docs/evidence/flake-v1/T05-03/REPORT.md`.

`T05-02` closed: qualified durability, confinement and performance on every platform. D1-D5 process-fault-schedule matrices (already implemented) now run natively on all three CI platforms. D6 (genuine forced native-VM-unclean-shutdown fault injection, new: QEMU/KVM-backed harness) ran 30 forced-kill cycles locally (WSL2/KVM) plus 8 more on CI (`ubuntu-latest`) -- zero acknowledged canonical loss across all 38 cycles, `head_hash_chain_verified: true` on every one. Two harness-only bugs found and fixed along the way (never product defects): the verification step needed to also copy the SQLite `-journal` file alongside `canonical.sqlite` (Flake uses `journal_mode=DELETE`, a rollback journal), and needed to briefly open its own disposable copy read-write before the independent read-only verification pass, since SQLite cannot roll back a hot journal on a strictly read-only connection. `docs/canonical/FOUNDER_T05-02_PHYSICAL_POWER_LOSS_AMENDMENT_2026-09-16.md` removed the original 10-physical-device-trials/profile requirement (no such hardware/lab report available); Windows/macOS VM cycles recorded as `NOT_EXECUTED_INFRASTRUCTURE_UNAVAILABLE` (non-blocking under that same amendment) -- D1-D5 process-level coverage plus the Linux D6 VM-level proof stand as the qualifying evidence for those two platforms. Full section-27 CLI performance matrix measured at real M-scale (10,000 records): every row within its own maximum, both locally (Windows) and on CI (`ubuntu-latest`). One flagged-not-fixed finding for a future product decision: `crate::recovery::recover_to_new_root`'s own `RecoveryGuard::acquire` is blocked by the exact same stale `WriteLock` marker it exists to remediate after a crash. Full details: `docs/evidence/flake-v1/T05-02/REPORT.md`.

`T05-01` closed: froze the format-1/2 compatibility policy and shipped a standalone offline `flake-migrate` tool, qualified natively on all three platforms plus M/L-scale performance timing on `ubuntu-latest` CI (M: 10,000 records/~1 GiB, import 17.82s within the 60s/180s gate; L: 100,000 records/~10 GiB, import 217.54s within the 600s/1800s gate; both `independent_head_hash_chain_verified: true`). Found and fixed one real defect during L-scale measurement: `preview_migration`/`import_to_new_root` held the full payload bytes of every admitted record in memory simultaneously (`raw_bytes: String` per entry, plus a second full `.clone()`'d copy in the import commit loop) -- roughly 10 GiB of live `String` data for the L-scale case, which killed the first CI attempt (`exit 143`) on a 15 GiB-RAM runner. Fixed to O(one record) peak memory by hashing/dropping each candidate's content during preview and re-reading each admitted record's bytes fresh from disk immediately before commit -- no change to admission rules, byte-exactness guarantee, or committed content (T01-06's own byte-identity tests still pass unchanged in what they prove). Full details: `docs/evidence/flake-v1/T05-01/REPORT.md`.

Preserved historical blocker record (superseded, not deleted -- the underlying WebView2 observation itself remains true and unchanged):

```text
BLOCKER_ID=T04-01-WEBVIEW2-BACKGROUND-NETWORK-TRAFFIC
STATUS=SUPERSEDED_BY_FOUNDER_DECISION
SUPERSEDED_BY=docs/canonical/FOUNDER_WEBVIEW2_NETWORK_BOUNDARY_2026-09-16.md
EXACT_GATE=T04-01 acceptance clause "starts offline with no external requests"
WHY_EXTERNAL=the traffic originates in the shared OS WebView2 runtime's own
  telemetry/service layer, confirmed by process-tree parentage to this app's
  own msedgewebview2.exe host; 2 rounds of documented Chromium/Edge
  background-networking-disable command-line flags did not eliminate it;
  an open upstream Microsoft feature request (WebView2Feedback#5224) asking
  for exactly this capability remains unresolved as of this evidence
CURRENT_EVIDENCE=docs/evidence/flake-v1/T04-01/REPORT.md
  ("What was discovered but not resolved"), raw/09-native-launch-network-observation.txt
RESOLUTION=founder ruled OPTION_1_AMENDED: the acceptance clause is read,
  narrowly and only for a WebView2-hosted surface, as
  FLAKE_APPLICATION_NETWORK=NONE (reverified) plus
  NETWORK_REQUIRED_FOR_FLAKE_OPERATION=NO (proven live via a network-denied
  functional test, docs/evidence/flake-v1/T04-01/network-denied-test/);
  WEBVIEW2_PLATFORM_BACKGROUND_TRAFFIC=OBSERVED remains true and is recorded
  as a documented platform limitation, not a Flake product failure
```

`T04-06` closed: native usability and accessibility of the complete desktop qualified, natively, on all three required platforms -- closing `P04`. `docs/canonical/FOUNDER_T04-06_ACCESSIBILITY_WITNESS_AMENDMENT_2026-09-16.md` replaced this task's own live-human-witness accessibility checklist with an automated technical qualification, without reducing the three-native-platform requirement. `.github/workflows/t04-06-cross-platform.yml` (a Windows/macOS/Linux matrix using already-authorized GitHub-hosted native runners, not only this session's own Windows workstation) proved, natively on all three, in one successful run (`35066083071`): root Rust `fmt`/`clippy`/`test --locked --lib` (326/326, identical deterministic suite -- "canonical results match CLI oracle" proven cross-platform, not merely asserted), `cargo audit`/`npm audit` (0 vulnerabilities each), desktop build, a new automated static accessibility checker (9 checks: interactive-element semantics, focus-order integrity, accessible names, no active imported content, system-theme respect, layout-overflow risk, non-color-only state indicators, computed WCAG AA contrast, bundle network-surface), and a native launch/clean-shutdown smoke test. One real product fix was found only because this task built on Linux for the first time: `tauri-plugin-dialog`'s disabled default features had left `rfd` (the crate `pick_directory` depends on) with zero native dialog backend selected there, a hard Linux build failure invisible on Windows/macOS -- fixed by re-adding exactly the `gtk3` feature. The static checker also found and fixed three real missing-accessible-name gaps and one real WCAG AA contrast failure (the shared error-red color was 3.86:1 against a dark background, below the 4.5:1 minimum -- this app declares `color-scheme: light dark`, so that is a legitimate rendering, not an edge case). A chained Windows golden-path E2E additionally ran the task's own named create->capture->evidence->decision/action->interrupt->resume->proposal->export->restore flow end to end in one continuous session, plus live keyboard-focusability and 200% zoom-overflow checks -- honestly recording that CDP's synthetic Tab-key dispatch does not reliably drive WebView2's native focus traversal, so that specific check uses direct element-focusability instead, cross-checked against the static tabIndex-integrity result. Full details: `docs/evidence/flake-v1/T04-06/REPORT.md`.

`T04-05` closed: backup, recovery, import and export exposed safely from the desktop. Additive-only `src/` change: `#[derive(Serialize)]` on `backup.rs`'s `BackupReport`/`RestoreReport`, `recovery.rs`'s `RecoveryReport`, `export.rs`'s `ExportReport`/`ExportPreview`, `import.rs`'s `ImportPreview`/`ImportReport` -- zero logic change, confirmed by the unchanged 326/326 Rust suite. 9 new typed desktop commands: `vault_backup` (genuinely cancellable -- a real `Arc<AtomicBool>` flag checked inside Core's own copy loop, run via `spawn_blocking` so a concurrent `cancel_operation` call can actually land) / `vault_restore_from_backup`, `vault_recover`, `export_preview`/`vault_export`, `import_preview`/`vault_import_selected` (merge into an *existing* vault, distinct from `T04-01`'s `vault_restore` into a brand-new one). No new native dialog capability -- every destination/source reuses `T04-01`'s already-admitted `pick_directory`. Verified via a scripted E2E combining this task's own two named techniques: independent SHA-256 byte verification of every backup/export member (never trusting Core's own `verified` flag alone) and original-vault comparison (reading the source vault directly before/after every operation via T02-07's unmodified `sqlite_reader.py`, proving it is untouched). A genuinely-raced concurrent cancellation actually interrupted a backup mid-flight on the qualifying run, confirmed to leave no published destination. Full details: `docs/evidence/flake-v1/T04-05/REPORT.md`.

`T04-04` closed: resumption and agent review as one evidence-linked flow. No `src/` change -- every Core capability this task exposes (`checkpoint.rs`/`grant.rs`/`disclosure.rs`/`proposal.rs`/`source_check.rs`) was already built, tested, and `Serialize`-ready from `T03-01`/`T03-03`/`T03-04`/`T03-05`. 13 new typed desktop commands: checkpoint mark/reset (behind an explicit, named confirmation -- "no accidental checkpoint"), full source-status listing/checking (generalizing `resume()`'s own stale-evidence scan to every source, not only non-`Match` ones), grant issue/revoke, package preview (no receipt persisted) and compile (receipt-before-emission, persisted, still no file write -- the destination-file write itself is `T04-05`'s own scope), and full proposal review (admit by pasting raw text -- no new native file-open dialog -- list/accept-selected/reject). "Desktop package/proposal result matches CLI" is proven as genuine same-vault interoperability in both directions (a desktop-issued grant/receipt admitted+accepted via the CLI, and a CLI-issued grant/receipt admitted+accepted via the desktop bridge), plus byte-identical deterministic package compilation (`emitted_sha256` equality) between the CLI and desktop code paths on an identical grant/request. A real security property was confirmed live rather than assumed: accepting a proposal's `DraftDecision` operation only admits a `Draft`-lifecycle decision -- it never auto-grants `Accepted` authority; only a separate, explicit owner `decision_accept` call does, confirmed visible to the CLI only after that separate call (§16 "agent content is evidence, never authority"). This task's own E2E run also found and fixed two real bugs: a same-second timestamp tie-break bug in the new `list_sources` (fixed to tie-break on the check object's own UUIDv7 ID) and a missing struct-level `#[serde(default)]` on a bundled options struct. Full details: `docs/evidence/flake-v1/T04-04/REPORT.md`.

`T04-03` closed: search, inspect and complete project work in the desktop, without CLI knowledge. Two small, additive Core changes (`src/project.rs`'s `tombstone_note`/`untombstone_note` -- explicitly named as `T04`'s own deferred scope by that module's existing doc comments, since `Note::tombstoned` had no setter until now; `#[derive(Serialize)]` added to `decision_state.rs`/`resume.rs`'s already-fully-typed structs, no logic change, so `resume()` can cross the IPC boundary without a parallel DTO hierarchy) plus 19 new typed desktop commands covering the full action lifecycle (create/start/block/cancel/reopen/complete), the full decision lifecycle (create/accept/withdraw/supersede), evidence-linking relations, project archive/unarchive, resume/history, and a bounded substring search. This task's own named acceptance clause -- "produces identical canonical state to CLI" -- is proven directly: the exact same workflow (project -> note -> action lifecycle -> decision lifecycle -> relation) driven once through the real desktop app and once through the real `fehrest` CLI, against two independent vaults, produces content-identical `canonical.sqlite` state (object IDs/revisions/timestamps aside, which are never expected to match across independently created vaults) -- verified by `docs/evidence/flake-v1/T04-03/e2e-test/compare_canonical_state.py`, reusing T02-07's unmodified `sqlite_reader.py`. A genuine `expected revision conflict` (blocking an action, then replaying a stale pre-block revision) is refused and independently confirmed non-destructive. Every existing Rust test still passes unchanged (326/326); no `src/` behavior changed, only two additive capabilities Core's own docs had already named as this task's to add. Full details: `docs/evidence/flake-v1/T04-03/REPORT.md`.

`T04-02` closed: an owner note editor with an honest save-state machine (`Saved`/`Unsaved`/`Saving…`/two distinct not-saved states/outcome-unknown), Ctrl/Cmd+S keyboard save, native undo/redo, and a dependency-free, security-inert Markdown preview (headings/bold/italic/code/lists; links and images render as plain text, never a clickable `<a>`/loaded `<img>` -- zero network/process activity by construction, not by a runtime check). Three new typed commands (`list_notes`, `note_create`, `note_update`, `desktop/src-tauri/src/commands.rs`) call only already-audited, unchanged `fehrest::project::{create_note,update_note}` -- no `src/` change, no new conflict logic (Core's own `expected_revision_id` check is the entire "never a silent overwrite" guarantee). A scripted E2E (`docs/evidence/flake-v1/T04-02/e2e-test/`) drove the real compiled app through its exact real typed-IPC bridge and independently cross-checked every claimed save by reading `canonical.sqlite` directly (reusing T02-07's unmodified `sqlite_reader.py`, never trusting the IPC response alone) -- proving byte-exact save/update, a refused stale-revision conflict that did not silently overwrite the committed state, a ~900 KB near-limit body preserved byte-exact, and a >1 MiB body correctly refused by Core's own existing limit. A real accessibility gap (no `aria-live` on the save-state announcement) was found while drafting the manual composition checklist and fixed, not merely noted. Native IME/screen-reader confirmation is honestly recorded as not yet performed (`MANUAL_COMPOSITION_CHECKLIST.md`) -- consistent with this task's own contract, which names exhaustive native composition/keyboard confirmation across three profiles as `T04-06`'s cross-platform gate, not this task's. Full details: `docs/evidence/flake-v1/T04-02/REPORT.md`.

`T04-01` closed: the founder-amended completion of the thin desktop shell scaffold. `docs/canonical/FOUNDER_WEBVIEW2_NETWORK_BOUNDARY_2026-09-16.md` records the full ruling on the WebView2 background-network finding this task surfaced (see `T04-01_PRIOR_BLOCKER_ID` above). Before applying it, every claim in the original evidence report's dependency-admission, plugin-registration, ACL, and CSP sections was independently reverified on a fresh checkout (`raw/10-founder-decision-reverification.txt`) -- unchanged from the original submission. Then a self-contained, fail-closed Node test harness (`docs/evidence/flake-v1/T04-01/network-denied-test/`) proved the load-bearing new requirement live, not merely asserted: with the host's only physical network adapter confirmed down by two independent methods, the real compiled `flake-desktop.exe`, driven through its exact real typed-IPC bridge (`window.__TAURI_INTERNALS__.invoke`, the same transport `@tauri-apps/api`'s `invoke()` uses), created a vault, listed and created a project, survived a simulated restart with the vault and project both persisted, and shut down cleanly twice -- all while genuinely offline. Two earlier attempts in the same session are preserved on record specifically because connectivity was not yet actually down and were correctly not reported as a network-denied result, which is why the harness's own fail-closed connectivity-precondition gate exists. `T04-01_STATUS=COMPLETE`. Full details: `docs/evidence/flake-v1/T04-01/REPORT.md`, `docs/evidence/flake-v1/T04-01/network-denied-test/README.md`.

`T03-08`'s `PASS` route satisfied the founder decision's "A T03-08 PASS unlocks T04-01," and T04-01 work began, surfaced the WebView2 finding above, and is now closed under the founder's amended reading.

```text
T03-08_ROUTE=PASS
P03_STATUS=CLOSED
T04-01_STATUS=COMPLETE
HUMAN_EVIDENCE_REQUIRED=NO
HUMAN_EVIDENCE_CLAIMED=NO
```

`T03-08` closed: the founder-authorized automated replacement for T03-07's human confirmatory trial. `bench/flake-v1/T03-08/` — `flake_arm.py` drives the real compiled `fehrest` CLI (never `src/` directly) through every one of T03-07's 96 sealed confirmatory cases; `baseline_arm.py` is a from-scratch, independently-implemented maintained-Markdown + index/status-log baseline importing nothing from Flake. Both arms derive their setup only from each case's `tier`/`sources` (public case-construction metadata) and never read `case["gold"]` — grading happens exclusively afterward, in `analysis.py`/`verify_independent.py`, against the unmodified T03-07 gold keys (`PROTOCOL_ADDENDUM.md` documents this boundary and the honest scope it implies: a round-trip technical-continuity qualification, not a reading-comprehension or human-effort claim). `96 cases x 2 arms = 192 total attempts`, all executed with zero raised exceptions; `seal.py` confirmed the reused `cases_confirmatory.json` digest matches T03-07's own seal before any confirmatory attempt ran. Result: 96/96 Flake `RESUME_CORRECT`, 96/96 baseline `RESUME_CORRECT`, 0 high-consequence misses, all 4 tiers at 24/24 — `analysis.py` (producer) and `verify_independent.py` (independent oracle, never importing the producer) produced byte-identical `PASS` output. This task also fixed a real cross-platform clippy defect discovered during re-verification: `src/capture.rs`/`src/source_check.rs`'s symlink tests declared a variable under `#[cfg(not(windows))]` but only read it under `#[cfg(windows)]`, which would fail `-D warnings` on macOS/Linux CI though it was invisible on this Windows host; fixed by collapsing both files' windows-only logic into one `#[cfg(windows)]` block, no lint suppressed, no test weakened, full 358/358 Rust suite re-confirmed with no regression. No human participant, adoption, retention, or comparative-effort claim is made anywhere in this package, per the founder decision. Full details: `docs/evidence/flake-v1/T03-08/REPORT.md`.

The canonical dependency DAG remains sequential: `T03-08` (COMPLETE, PASS) unlocked `T04-01` (COMPLETE, under the founder's amended WebView2 reading), which unlocked `T04-02` (COMPLETE), which unlocks `T04-03`. Spec 003 auto-activation remains prohibited. `EXECUTABLE_REPOSITORY_WORK=AVAILABLE` and `PROJECT_COMPLETE=NO`.

`T03-07` closed: the complete preregistration package for the Section 26 P03 value gate, under `bench/flake-v1/T03-07/` — `PROTOCOL.md` (research question, eligibility, recruitment, consent/withdrawal/privacy, training, exact-parity counterbalancing formulas, equal source/setup/access budgets, success/failure/timeout/exclusion definitions, class-loss route, one-permitted-repair-repeat, and a fixed seven-step mechanical Pass/Fail/Inconclusive decision route) and `CONSENT.md`, both structurally checked by `test_protocol.py`. `generate_cases.py`/`generate_allocation.py` deterministically produce the sealed, disjoint, held-out `cases_confirmatory.json` (96 cases, exactly matching the 6-participant×8-pair×2-condition design) and `allocation_confirmatory.json` (exact 24/24 counterbalancing, verified, not merely intended); re-running either script reproduces the sealed files byte-for-byte. `harness.py`'s append-only manifest was proven resumable under a simulated process interruption; `analysis.py` (producer) and a separately-written `verify_independent.py` (oracle, never importing the producer) independently agree on every one of the seven possible routing outcomes. `dry_run.py` exercised the entire machinery end to end using only non-confirmatory `cases_dev.json` data, including an injected timeout, exclusion, protocol deviation, and a simulated harness-restart. `SEALS.json` records SHA-256 digests of all 11 load-bearing files plus the exact qualified product commit (`29263249e17fbaf93bd6a2d764fed418da255feb`) this study is frozen against — sealed before any confirmatory human observation exists. No participant has been recruited and no human evidence of any kind exists in this package; `analysis.py` structurally refuses to treat dev/synthetic data as confirmatory (a runtime guard, not a convention). 53/53 Python tests pass across 7 suites; the full Rust suite (358/358) was re-run to confirm no regression, since this task touched no `src/` code. Full details: `docs/evidence/flake-v1/T03-07/REPORT.md`.

`T03-06` closed: a versioned protocol document, `docs/formats/agent-disclosure-protocol-v1.md`, specifying both halves of §18's wire protocol (the line-oriented disclosure package; the single-JSON-document agent proposal) precisely enough for an independent implementer. Two independent, offline, stdlib-only Python client fixtures (`tools/interchange-clients/client_a.py`/`client_b.py`) were written directly from that document — never from `src/disclosure.rs`/`src/proposal.rs`, and never sharing code with each other (deliberately different internal styles, matching `T02-07`'s own established independence discipline). `run_interchange.py` drives the real `fehrest` CLI end to end: client A reads a real compiled package and proposes a note edit; the owner reviews/accepts through the real CLI; client B reads the *resulting* package and continues with a genuinely different, compatible proposal (a draft decision citing client A's own accepted edit); three adversarial cases (unknown declared identity, an unsupported operation kind, duplicate delivery) are run directly against the real CLI. Every content claim is independently verified by reading `canonical.sqlite` directly (reusing `T02-07`'s own already-independent `sqlite_reader.py`), never by trusting the CLI's own stdout. Designing the client fixtures surfaced and fixed a real protocol gap in already-merged `T03-04` code: `compile_disclosure_package` discarded its own committed receipt's `object_id`, leaving no way for an external agent to learn what `receipt_id` to cite in a proposal (structurally, the ID cannot be embedded in the wire bytes themselves without a self-hashing recursion) — fixed by returning it and having `package-export` print it out-of-band. Full Rust suite re-run after the fix, no regressions (358/358); the interchange run itself passes twice in a row (`ALL CHECKS PASSED`), including a full independent re-verification of the transaction head-hash chain across the whole 16-command sequence. Full details: `docs/evidence/flake-v1/T03-06/REPORT.md`.

`T03-05` closed: an eleventh `RecordPayload` kind, `AgentProposal` (`src/proposal.rs`) — the inbound half of §18's protocol, whose outbound half (`ExportGrant`/`DisclosureReceipt`) `T03-04` already built. `admit_proposal` parses a bounded (≤1 MiB, ≤100 operations) inbound JSON document into exactly four allowed operation kinds (`NoteEdit`/`DraftDecision`/`EvidenceRelation`/`CompleteAction` — serde's own internally-tagged-enum dispatch rejects any other `kind` at parse time, so the allowlist is the type, not a bypassable runtime check), requires `receipt_id` to name a real `DisclosureReceipt` in the same project, and requires every existing-object-targeting operation's target to have actually appeared in that receipt's own `selected` list. `accept_proposal` reuses `T02-01`-`T02-03`'s own mutation functions unchanged (`update_note`/`create_decision` with `basis: DecisionBasis::AgentProposal`/`create_relation`/`complete_action`) rather than their private internals, committing content with an agent-attribution `actor` string distinct from the reviewing owner's own `actor` on the proposal's accept/reject transition — "owner transition and agent origin recorded separately" via the existing `actor` field, no new field invented. A stale `expected_revision_id` on any selected operation fails that operation's own already-proven conflict check, leaving the proposal `Pending`, never partially/silently applied; replaying an already-`Accepted` proposal's accept call is refused by the same `Pending`-only guard, never double-applying. §16 ("no authority survives export/import") extended to this new kind at both layers already established by `T03-04`. CLI: `propose-import`, `propose-show`, `project-proposals`, `propose-accept`, `propose-reject`, `propose-expire`. 21 new tests (19 `proposal.rs` + 1 `cli.rs` end-to-end + 1 `import.rs` extending the §16 boundary; `export.rs`'s own boundary test was extended in place to also cover the new kind, not duplicated), all passing (358/358 full suite); `fmt`/`clippy -D warnings`/`git diff --check` all clean. Full details: `docs/evidence/flake-v1/T03-05/REPORT.md`.

`T03-04` closed: two new `RecordPayload` kinds — `ExportGrant` (`src/grant.rs`, ninth kind, `Relation`'s shape: owner-issued, project-scoped, kind/ID-allowlisted, privacy-excludable, byte-budget-capped, TTL-capped at seven days) and `DisclosureReceipt` (`src/disclosure.rs`, tenth kind, immutable — never updated after creation). `disclosure::compile_disclosure_package` persists the receipt via `CommandTarget::CreateObject` *before* ever returning wire bytes to its caller (receipt-before-emission, D1/D5); the wire format is a header line plus one JSON object per disclosed item (never one big wrapped document — an earlier draft that nested items inside a shared envelope broke exact budget accounting, since the envelope's own byte cost wasn't attributable to any single item; fixed during self-review, see the evidence report's "Failed attempts"). Every candidate is grant-scope-checked first (kind allowlist, ID allowlist, privacy exclusions, each with its own reason) then budget-fit (full/truncated/omitted, each with its own reason) — `rejected` entries live only on the receipt, never on the budget-capped wire itself. `Relation`s are disclosed last and only when both endpoints were themselves disclosed, never revealing a relation naming an object the recipient can't otherwise see. §16 enforced at two independent layers: `export.rs`'s project-scoped path never emits a grant or receipt (proven negatively), and `import_selected_merge` explicitly refuses a package containing either (defense in depth; full-restore is unaffected — a same-owner backup, not a shareable disclosure). `Decision` disclosure reuses `T03-02`'s resolver unchanged (only `NeedsReview`/`CurrentSet` decisions are ever candidates; a solely-`Draft`/`Withdrawn` key is never offered at all, not even as a rejection). CLI: `grant-issue`, `grant-revoke`, `package-preview`, `package-export` (stage-then-reread-verify before reporting success). 23 new tests (7 `grant.rs` + 12 `disclosure.rs` + 1 `cli.rs` end-to-end + 2 `import.rs` + 1 `export.rs`, the last three proving the §16 boundary), all passing (337/337 full suite); `fmt`/`clippy -D warnings`/`git diff --check` all clean (a stray CRLF conversion in `project.rs` from a Windows-Python text-mode write, caught during self-review before commit, was reverted to LF to avoid a spurious whole-file diff). Full details: `docs/evidence/flake-v1/T03-04/REPORT.md`.

`T03-03` closed: an eighth `RecordPayload` kind, `ReviewCheckpoint` (`src/checkpoint.rs`), mirroring `Relation`'s shape — one canonical object per project recording an explicit, owner-driven "reviewed through this recorded sequence" marker, monotonic unless explicitly reset with a reason (`mark_reviewed_through`/`reset_checkpoint`). `src/resume.rs`'s `resume()` composes the owner resume view purely from already-canonical reads (never writes a checkpoint, so opening/computing resume can never itself mark reviewed — §12): conflicts (`NeedsReview` decision keys) and stale/missing evidence (non-`Match` latest source checks) precede current accepted decisions, next actions (non-terminal), relevant notes (changed since checkpoint) and the raw changes-since-checkpoint log, exactly §18's own priority-group ordering; the whole view is pinned to one fixed `head_seq` snapshot per call. CLI: `resume`, `checkpoint-mark`, `checkpoint-reset`, `checkpoint-history`. Self-review before commit caught a real gap: `import_selected_merge`'s explicit per-kind pass list initially omitted `review_checkpoint`, which would have silently dropped every checkpoint object during a merge-import — fixed (checkpoints now ride the same simple project_id-only pass as `Note`/`Decision`) and regression-tested (`merge_review_checkpoint_is_carried_through_with_its_project_id_rewritten`, `project_export_includes_a_review_checkpoint`). 19 new tests, all passing (314/314 full suite); `fmt`/`clippy -D warnings`/`git diff --check` all clean. Full details: `docs/evidence/flake-v1/T03-03/REPORT.md`.

`T03-02` closed: a standalone module, `src/decision_state.rs` (`resolve_decision_state`), deterministically resolves "what decision is currently accepted" for one `(project, decision_key)` as of a caller-chosen valid-time instant and recorded-sequence cutoff — `CurrentSet`/`NeedsReview`/`NoAcceptedDecision`, with `considered` always listing every candidate decision and, for each excluded one, why (lifecycle, out-of-interval, or its own `withdrawal_reason`). Not a reuse of the historical Phase T `temporal.rs`/`memory.rs` (immutable evidence, `AGENTS.md` §3): `Decision`'s already-explicit lifecycle/supersession model needs none of Phase T's five-rung confidence ladder, so the only rule this resolver applies is "two independently-accepted decisions sharing a key with overlapping valid time are always `NeedsReview`, never silently ranked" — structurally forced by `supersede_decision` always demoting the loser's lifecycle in the same step that records the supersession edge. Verified against a from-scratch, independently-written reference oracle (`tests::reference_oracle`, using `CanonicalStore::history`/`revisions_since` rather than the production path's `all_revisions` fold) across 200 seeded random scenarios (seed `20260915`); all agree. CLI: `decision-state --project <uuid> --key K [--as-of-valid TS] [--as-of-recorded N]`. 13 new tests, all passing (295/295 full suite); `fmt`/`clippy -D warnings`/`git diff --check` all clean. Full details: `docs/evidence/flake-v1/T03-02/REPORT.md`.

`T03-01` closed: a seventh `RecordPayload` kind, `SourceCheck` (`src/source_check.rs`), mirroring `Relation`'s architectural shape — an append-only observation of whether a previously admitted `Source`'s selected local file still matches, changed, went missing or became unreadable. `capture::import_file` now populates the previously-always-`None` `Source::claimed_path` locator hint. Three functions, deliberately separated: `check_source` (pure observation, never mutates `Source`), `reselect_source` (relocation, refused unless the new location's bytes are digest-identical to the last saved capture), `admit_changed_source` (explicit owner admission of changed bytes, always re-opening and re-hashing fresh). Both mutating functions require the caller's `expected_revision_id` (F21/I05). A symlink or any non-regular-file swapped in at the checked path is refused as `Denied`, never followed (S03). `export.rs`'s project-scope union, `import.rs`'s merge/rewrite passes and `index.rs` each got the minimal, mechanical extension a new `RecordPayload` kind structurally requires, matching every prior new-kind task's own precedent. `Unchecked` is a derived read-path label (zero rows in history), not a stored `CheckStatus` variant — recorded as a deliberate reading of §15, not an omission. 15 new tests, all passing (282/282 full suite); `fmt`/`clippy -D warnings`/`git diff --check` all clean. Full details: `docs/evidence/flake-v1/T03-01/REPORT.md`.

`T02-07` closed: a standalone Python (stdlib-only, no Flake crate dependency) independent verifier under `tools/independent-verify/` proves Flake's canonical/exported state is reconstructable without Flake. Two genuinely separate raw readers — `sqlite_reader.py` (direct `canonical.sqlite` inspection via `sqlite3`, independently re-deriving `payload_sha256` and the full `resulting_head_hash` chain from the format document's own published algorithm) and `export_reader.py` (a `.fehrest-export/` package reader, independently recomputing `integrity_root` and every member/payload digest) — each build a semantic report (record counts, current state per object, relations, action dependencies, source byte digests, project membership) from a disposable fixture built through the real `fehrest` CLI's full create/capture/find/complete/export loop across two projects. The two independently-derived reports agree exactly on the full-store export and on the project-scoped subset (`crosscheck.py`), including after the derived FTS index (`derived-fts.sqlite`) is deleted and the vault is re-exported — proving canonical export does not depend on derived state. An 11-case adversarial suite (`adversarial.py`) hand-corrupts copies of a valid export (missing member, length/digest mismatch, tampered `integrity_root`, duplicate `(object_id, revision_id)`, broken relation endpoint, absent action dependency, truncated JSON, a fully-removed referenced object, an unrecognized manifest schema, and a source `capture.bytes_hex`/`capture.sha256` mismatch) and confirms the verifier detects and clearly reports every one — no product defect was found in `T02-05`/`T02-06` by this task. Full details: `docs/evidence/flake-v1/T02-07/REPORT.md`.

Do not activate Spec 003 automatically. Do not invent a replacement roadmap. Do not use OpenAI API or a required paid AI/model service.
