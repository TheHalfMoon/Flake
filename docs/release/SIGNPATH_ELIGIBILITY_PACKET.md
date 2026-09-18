# SignPath Foundation eligibility packet — Flake

Research performed against SignPath Foundation's own current, authoritative published
material: `https://signpath.org/terms.html` (conditions for open-source projects),
`https://signpath.org/` and `https://signpath.org/apply` (application), `https://signpath.org/about.html`,
and SignPath's GitHub Actions integration documentation
(`https://docs.signpath.io/trusted-build-systems/github`, the
`signpath/github-action-submit-signing-request` action). Retrieved 2026-09-18.

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
