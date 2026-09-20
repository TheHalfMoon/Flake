# SignPath Foundation eligibility packet — Flake

Research performed against SignPath Foundation's own current, authoritative published
material: `https://signpath.org/terms.html` (conditions for open-source projects),
`https://signpath.org/` and `https://signpath.org/apply` (application), `https://signpath.org/about.html`,
and SignPath's GitHub Actions integration documentation
(`https://docs.signpath.io/trusted-build-systems/github`, the
`signpath/github-action-submit-signing-request` action). Retrieved 2026-09-18, rechecked
2026-09-20 (see "Release-eligibility recheck" below).

## Eligibility check against SignPath Foundation's own published conditions

| SignPath Foundation requirement (quoted/paraphrased from `signpath.org/terms.html`) | Flake's status |
|---|---|
| "The project must use an OSI-approved Open Source license without commercial dual-licensing for all components" | **Met.** Apache-2.0 (OSI-approved), no dual-licensing; `LICENSE` root file, `Cargo.toml` `license = "Apache-2.0"` |
| "The project may not contain any proprietary, non open-source component" | **Met.** `docs/legal/THIRD-PARTY-LICENSES.md` attributes all 361 shipped third-party components, every one open-source; `T05-04_ADVISORY_REFRESH` found 0 vulnerabilities |
| "The project must not contain malware or potentially unwanted programs" | **Met**, subject to SignPath's own independent review |
| "The project must be actively maintained" | **Met.** Continuous commit history through 2026-09-18 (`git log`) |
| "The project must already be released in the form that should be signed" | **Not yet met.** T05-03 produced `UNSIGNED_DEVELOPER_RC` candidates; no signed public release has shipped. SignPath's own review process is expected to accept an unsigned release candidate as the "already released" artifact for initial application — this is the one criterion that cannot be self-certified from published text alone and must be confirmed during the real application |
| "The project's functionality must be described on its download page or in the app store entry" | **Met.** `docs/release/USER_GUIDE.md`, `README.md` |
| "The team responsible for code signing must also be the team responsible for development and maintenance, including ownership of the source code repository" | **Met.** Sole maintainer TheHalfMoon owns `github.com/TheHalfMoon/Flake` (confirmed live: `gh repo view` — public, Apache-2.0, owner `TheHalfMoon`) |
| "The team must only sign software artifacts built from their own source code" | **Met by design.** `.github/workflows/t05-04-signing-pipeline-test.yml` and the release-candidate pipeline only ever sign artifacts this repository's own CI just built |
| Author/Reviewer/Approver roles, MFA on SignPath and repository access | **Prerequisite documentation complete** (`docs/release/CODE_SIGNING_POLICY.md` role table); actual MFA enrollment on the Founder's own GitHub account and future SignPath account is a Founder action, not repository-owned |
| Public "Code signing policy" page on the project home page | **Met.** `docs/release/CODE_SIGNING_POLICY.md`, linked from `README.md` |
| No vulnerability-scanning/hacking-tool features | **Met.** Flake is a local project-continuity tool; no such feature exists |
| Binary artifacts built from source in a verifiable way; consistent metadata (product name/version) | **Met.** `scripts/release/package_cli_archive.sh`/`generate_sbom.sh`/`reproducibility_check.sh` plus the new GitHub artifact attestations (`docs/release/RELEASE_VERIFICATION.md`) make the build verifiable; `Cargo.toml`/`tauri.conf.json` set consistent product name/version |
| Uninstallation instructions/facility; system-modification warnings; installation-time data-collection disclosure | **Met.** `docs/release/USER_GUIDE.md`; Flake collects no data (`src/about.rs`'s `PRIVACY_STATEMENT`) |

## Exact packet

```text
SIGNPATH_PROJECT_ELIGIBILITY=PENDING_EXTERNAL_APPROVAL
SIGNPATH_REQUIRED_ACTION=Founder submits the real application at
  https://signpath.org/apply using a real personal/GitHub identity and
  completes any follow-up review correspondence SignPath sends; this
  cannot be completed by repository automation because it requires a
  real human applicant identity and account creation with SignPath, and
  SignPath's own "already released" criterion is confirmed only through
  their actual review, not by self-certification against published text
SIGNPATH_APPLICATION_URL=https://signpath.org/apply
SIGNPATH_REQUIRED_PROJECT_DATA=project name (Flake); repository URL
  (https://github.com/TheHalfMoon/Flake); license (Apache-2.0); code
  signing policy page URL
  (https://github.com/TheHalfMoon/Flake/blob/main/docs/release/CODE_SIGNING_POLICY.md);
  Author/Reviewer/Approver contact (TheHalfMoon, all three roles
  pending delegation); a description of what is signed (Windows NSIS
  installer + flake.exe/fehrest.exe/flake-migrate.exe CLI binaries,
  produced by scripts/release/package_cli_archive.sh and the Tauri
  Windows bundle); build system (GitHub Actions, public repository,
  windows-latest runner)
SIGNPATH_REPOSITORY_WORK_COMPLETE=YES
WINDOWS_REMAINING_EXTERNAL_ACTION=Founder submits the SignPath Foundation
  application at https://signpath.org/apply and completes SignPath's own
  review/onboarding (organization id, project slug, signing policy slug,
  API token are issued only after approval and cannot be fabricated here)
```

## What "repository work complete" means concretely

Everything SignPath's own documented integration model requires on the repository side before
an approved project can start signing is already in place or scaffolded:

- **Code signing policy page**: `docs/release/CODE_SIGNING_POLICY.md`, linked from `README.md`.
- **Role documentation**: recorded in that same policy page.
- **Verifiable build**: the existing `scripts/release/package_cli_archive.sh` and Tauri Windows
  bundle build deterministically from this repository's own source in GitHub Actions;
  `scripts/release/reproducibility_check.sh` already proves byte-for-byte reproducibility.
- **GitHub Actions integration shape**: SignPath's own documented flow is
  `actions/upload-artifact` followed by `signpath/github-action-submit-signing-request@v3`,
  reading `organization-id`/`project-slug`/`signing-policy-slug` and a `SIGNPATH_API_TOKEN`
  secret, all of which are issued only after SignPath approves the application — so the actual
  workflow step is written as a documented, inert template
  (`docs/release/LINUX_RELEASE_SIGNING.md`'s sibling for Windows would be identical in shape)
  rather than committed as a live, always-run CI job that would otherwise fail every run for
  want of secrets that cannot exist yet:

  ```yaml
  # Activate only after SignPath Foundation approval issues real values for every
  # <PLACEHOLDER> below and a SIGNPATH_API_TOKEN repository secret is created.
  - name: Upload unsigned Windows artifact
    id: upload-unsigned-artifact
    uses: actions/upload-artifact@v4
    with:
      name: windows-unsigned-for-signpath
      path: path/to/flake-windows-installer.exe

  - name: Submit SignPath signing request
    uses: signpath/github-action-submit-signing-request@v3
    with:
      api-token: '${{ secrets.SIGNPATH_API_TOKEN }}'
      organization-id: '<SignPath organization id -- issued on approval>'
      project-slug: '<SignPath project slug -- issued on approval>'
      signing-policy-slug: '<SignPath signing policy slug -- issued on approval>'
      github-artifact-id: '${{ steps.upload-unsigned-artifact.outputs.artifact-id }}'
      wait-for-completion: true
      output-artifact-directory: 'dist/signed/windows'
  ```

  This is intentionally not wired into a live workflow yet — doing so before approval would
  either fail every CI run on a missing secret or silently no-op, neither of which is honest
  CI signal. It is committed here, in documentation, as the exact next step once approval
  values exist, per `scripts/release/sign_windows.sh`'s own existing abstraction (production
  branch activated purely by supplying real credentials, no script logic change required).

## Why this cannot close to `PASS` from repository work alone

SignPath Foundation's approval is, by their own design, a human review of a real applicant
identity and a real project state, concluding in values (organization id, project slug,
signing policy slug, API token) that do not exist until they issue them. No amount of
repository-owned preparation can fabricate those values or substitute for SignPath's own
review — doing so would be exactly the fabricated-approval outcome the Founder's instructions
prohibit. `WINDOWS_SIGNING_STATUS` therefore remains `PENDING_SIGNPATH_EXTERNAL_APPROVAL` until
the Founder actually applies and SignPath actually approves.

## Release-eligibility recheck (2026-09-20)

Rechecked live against SignPath Foundation's current published material — `signpath.org/terms.html`
(fetched fresh, 2026-09-20), `signpath.org/apply`, `signpath.org/about.html`, and
`docs.signpath.io` (the "Trusted Build Systems → GitHub" and "Origin Verification" pages). No
material change from the 2026-09-18 research above:

- Every core eligibility condition on `signpath.org/terms.html` — OSI-approved license without
  dual-licensing, no proprietary component, no malware/PUP, active maintenance, functionality
  documented on the download page, team-owns-repository, sign-only-own-builds, no
  security-circumvention features, no undisclosed system-configuration changes, uninstallation
  facility, Author/Reviewer/Approver roles with MFA, a published code signing policy, consistent
  signed-artifact metadata — reads identically to the 2026-09-18 text quoted in the table above.
  One nuance not previously noted: the page's linked Code of Conduct is itself marked "Draft"
  status by SignPath, with no revision date shown; this affects nothing in the eligibility table,
  which is drawn from `terms.html` itself, not the draft Code of Conduct.
- `docs.signpath.io/trusted-build-systems/github`'s documented `signpath/github-action-submit-signing-request@v3`
  action and its required inputs (`api-token`, `organization-id`, `project-slug`,
  `signing-policy-slug`, `github-artifact-id`, plus optional `wait-for-completion` and
  `output-artifact-directory`) are unchanged from what `docs/release/LINUX_RELEASE_SIGNING.md`'s
  Windows-sibling template above already assumes.
- **New, not previously recorded:** SignPath also offers an optional **Origin Verification**
  feature (`docs.signpath.io/origin-verification`), configured on a signing policy *after*
  project approval, which lets SignPath cryptographically bind a signing request to trusted
  build metadata (source repository URL, branch, commit, CI build-job URL, and a
  reproducible-build assertion) and restrict signing to an allowed branch list (e.g. `main`,
  `release/*`). This is not an eligibility requirement and does not block or change the
  application — it is recorded here as a recommended post-approval hardening step, consistent
  with this project's existing provenance/attestation posture, to be configured once the project
  exists in the SignPath dashboard.
- **Independently unverifiable via this recheck:** `signpath.org/apply`'s actual application
  form fields render only after page/script load that this session's fetch tooling could not
  execute — the exact field set on the live form was not observed. The packet below lists every
  value this repository can supply so the Founder can fill in whatever fields the live form
  actually presents; if the live form asks for anything not listed here, it will be a small,
  self-evident addition (e.g. an email address to receive review correspondence), not a
  contradiction of anything above.
- **Confirmed unchanged, and still the one real gap:** "The project must already be released in
  the form that should be signed." `gh release list` on this repository returns empty — Flake
  has not published a GitHub Release. See "Reconciling the 'already released' requirement"
  below.

## Reconciling the "already released" requirement against Flake's canonical governance

`FLAKE_CANONICAL_BUILD_PLAN.md` section 25 reserves "uploading, publishing a release" as an
action separately authorized beyond building verified release candidates — repository
automation and agent sessions may build and qualify `UNSIGNED_DEVELOPER_RC` candidates, but may
not publish them as a public GitHub Release without explicit Founder authorization
(`AGENTS.md` §8, "no unauthorized remote actions"). No release has been published under this
reservation, and none is published by this packet.

This creates a real, unresolved question for the SignPath application, not a repository-side
blocker: SignPath's own reviewers, not published text alone, will determine whether an
`UNSIGNED_DEVELOPER_RC` GitHub Release (clearly labeled as an unsigned developer build, exactly
as `docs/release/USER_GUIDE.md` §9 already labels every artifact) satisfies "already released,"
or whether they require a release the Founder considers more final. Two Founder-only paths
forward, neither performable by this session without further authorization:

1. **Apply now, without a published release**, and let SignPath's own review correspondence
   clarify what they need — applying costs nothing and starts the review clock; if they ask for
   a published artifact, the Founder can authorize publishing one at that point.
2. **Authorize publishing an `UNSIGNED_DEVELOPER_RC` pre-release first** (a GitHub Release
   tagged from the already-qualified `T05-03`/`T05-04` release-candidate artifacts, explicitly
   labeled as an unsigned developer build, not a final release), strengthening the application
   before submitting it.

This session recommends path 1 (apply first; it is reversible and non-committal) but takes no
action on either without the Founder's explicit choice — see the Founder-action request that
accompanies this packet.

## Complete SignPath application packet (copy-paste values)

Every value below is exact and ready to paste into whatever field the live application form
presents it for. Values SignPath issues only after approval are marked accordingly — they
cannot be filled in before that point by anyone, including the Founder.

| Field | Value |
|---|---|
| Project name | `Flake` |
| Repository URL | `https://github.com/TheHalfMoon/Flake` |
| License | `Apache License, Version 2.0` (OSI-approved; SPDX `Apache-2.0`) |
| Project description | `Flake is a local-first application for capturing project work, preserving the evidence behind decisions, and resuming after interruption with visible changes and next actions. External agents may receive bounded evidence packages and return reviewable proposals; the project remains understandable when an agent disappears.` (verbatim from `README.md`) |
| Functionality/download page URL | `https://github.com/TheHalfMoon/Flake` (README) and `https://github.com/TheHalfMoon/Flake/blob/main/docs/release/USER_GUIDE.md` |
| Release/download URL | **Not yet published** — see "Reconciling the 'already released' requirement" above; once published, `https://github.com/TheHalfMoon/Flake/releases` |
| Code signing policy URL | `https://github.com/TheHalfMoon/Flake/blob/main/docs/release/CODE_SIGNING_POLICY.md` (linked from `README.md`) |
| Maintainer / applicant identity | GitHub account `TheHalfMoon` (repository owner) — the Founder applies using their own real identity; this cannot be a repository-automation identity |
| Author role | `TheHalfMoon` |
| Reviewer role | `TheHalfMoon` (single-maintainer project; table in `docs/release/CODE_SIGNING_POLICY.md` will be updated the moment any role is delegated) |
| Approver role | `TheHalfMoon` |
| MFA on SignPath account and on GitHub (`TheHalfMoon`) | **Founder must confirm/enable directly** — this session's read-only GitHub API check (`gh api user`) cannot observe 2FA status (GitHub no longer exposes it via that field for privacy); enable in GitHub under Settings → Password and authentication, and on the SignPath account at signup |
| Build system | GitHub Actions, public repository, `windows-latest`-hosted runner |
| Artifact types to be signed | Windows NSIS installer (`.exe`, Tauri desktop bundle) and CLI binaries `flake.exe` / `fehrest.exe` / `flake-migrate.exe`, produced by `scripts/release/package_cli_archive.sh` and the Tauri Windows bundle step already qualified in `.github/workflows/t05-03-release-candidates.yml` |
| Organization id | **Issued by SignPath only after approval** — cannot be filled in now |
| Project slug | **Issued by SignPath only after approval** — cannot be filled in now |
| Signing policy slug | **Issued by SignPath only after approval** — cannot be filled in now |
| `SIGNPATH_API_TOKEN` | **Issued by SignPath only after approval**, then stored as a GitHub Actions repository secret, never in this repository's tracked files or any agent-visible channel |
| Origin Verification (optional, post-approval) | Recommended: enable on the signing policy once created, restricting signing to the `main` branch, per the recheck above |

`SIGNPATH_PROJECT_ELIGIBILITY` remains `PENDING_EXTERNAL_APPROVAL`; nothing above changes that —
it records exactly what the Founder needs to submit and confirms it against SignPath's live,
current terms rather than the 2026-09-18 snapshot alone.
