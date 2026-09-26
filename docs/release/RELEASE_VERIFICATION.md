# Release verification

How to independently verify a Pluma release artifact. This page collects verification commands
for every mechanism Pluma uses or intends to use; it does not restate their design (see
`docs/release/CODE_SIGNING_POLICY.md` for the policy overview and links to each mechanism's own
document).

Filenames below use the `pluma-`/`Pluma`-prefixed naming used from the 2026-09-20 Flake→Pluma
rename onward; a release published before that date carries the equivalent `flake-`/`Flake`
prefix instead — substitute accordingly.

A verification failure on any of the checks below means the artifact must not be trusted or
installed. Fail closed — do not proceed past a failed check.

## SHA-256 checksums

Every release-candidate CLI archive and desktop bundle ships alongside a `.sha256` manifest
(`scripts/release/package_cli_archive.sh`). Verify with:

```bash
sha256sum -c pluma-<version>-<platform>.sha256
```

## GitHub artifact attestations (build provenance)

Confirms an artifact was built by this repository's own GitHub Actions workflow, at an exact
commit, rather than substituted or tampered with after the fact. Requires the
[GitHub CLI](https://cli.github.com/) (`gh`, version with `attestation` support):

```bash
gh attestation verify pluma-<version>-<platform>.zip -R TheHalfMoon/Pluma
```

A successful verification prints the exact source repository, workflow, and commit SHA the
artifact was built from. This is additional supply-chain evidence — it confirms provenance, not
platform trust (it does not make Windows SmartScreen, macOS Gatekeeper, or a Linux package
manager trust the artifact; those still require the mechanisms below where applicable).

## Windows — website-first direct distribution (unsigned, disclosed)

Per `docs/canonical/FOUNDER_WEBSITE_FIRST_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-26.md`,
Pluma's Windows installer and CLI archive are distributed unsigned from the web — never the
Microsoft Store, never a paid certificate. Full design:
`docs/release/WINDOWS_DIRECT_DISTRIBUTION.md`.

```powershell
# 1. Checksum (PowerShell)
Get-FileHash pluma-<version>-windows-x86_64.zip -Algorithm SHA256
# Compare against the published .sha256 manifest byte-for-byte.
# (Or: certutil -hashfile pluma-<version>-windows-x86_64.zip SHA256)
```

```bash
# 2. Project GPG signature (same identity and fingerprint as the Linux/macOS release-signing key)
gpg --import docs/release/flake-release-signing-public.asc
gpg --verify pluma-<version>-windows-x86_64.zip.asc pluma-<version>-windows-x86_64.zip
gpg --verify Pluma_<version>_x64-setup.exe.asc Pluma_<version>_x64-setup.exe
# Compare the signing key's fingerprint against the one published in
# docs/release/LINUX_RELEASE_SIGNING.md before trusting anything signed with it.
```

```bash
# 3. GitHub artifact attestation (build provenance, not platform trust)
gh attestation verify Pluma_<version>_x64-setup.exe -R TheHalfMoon/Pluma
```

```powershell
# 4. Authenticode truth inspection (expected: NOT SIGNED -- this is disclosed, not hidden)
signtool verify /pa /v Pluma_<version>_x64-setup.exe
Get-AuthenticodeSignature -FilePath Pluma_<version>_x64-setup.exe | Format-List Status,StatusMessage
```

**Production state:**

```text
WINDOWS_DISTRIBUTION_MODE=DIRECT_WEB
WINDOWS_AUTHENTICODE_TRUST=NOT_AVAILABLE
WINDOWS_AUTHENTICODE_TRUST_CLAIMED=NO
WINDOWS_SMARTSCREEN_WARNING=EXPECTED_AND_DISCLOSED
```

**What this does and does not prove:** the checksum confirms the file was not corrupted or
altered in transit. The GPG signature confirms it was produced by Pluma's own project release
process. The GitHub attestation confirms it was built by this repository's own CI at an exact
commit. None of these, individually or together, are Windows platform trust — Pluma does not
have a trusted Authenticode certificate and does not claim one. Windows SmartScreen may warn;
see `docs/release/WINDOWS_DIRECT_DISTRIBUTION.md` for the safe per-file override path. Never
disable Defender/SmartScreen globally, weaken system policy, bypass enterprise controls, or
disable security services.

### Prior SignPath Foundation history (preserved)

Windows Authenticode via SignPath Foundation was previously the intended v1 path
(`docs/release/SIGNPATH_ELIGIBILITY_PACKET.md`). SignPath declined the application on
2026-09-25 for insufficient public visibility; the Founder superseded that path with direct
web distribution and declined the paid route. No SignPath approval or credential was ever
received or claimed.

## Linux — GPG detached signature

The production public key is now published. Verify its fingerprint before trusting any release signature:

```bash
gpg --import docs/release/flake-release-signing-public.asc
gpg --list-keys --with-colons "285091250+TheHalfMoon@users.noreply.github.com" \
  | awk -F: '/^fpr:/ {print $10; exit}'
# Compare the printed fingerprint against the one published in
# docs/release/LINUX_RELEASE_SIGNING.md before trusting anything signed with it.

gpg --verify pluma-<version>-linux-x86_64.tar.gz.asc pluma-<version>-linux-x86_64.tar.gz
gpg --verify pluma_<version>_amd64.deb.asc pluma_<version>_amd64.deb
```

**Production fingerprint:** `F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`

## macOS — zero-cost direct distribution (no Apple Developer ID, no notarization)

Per `docs/canonical/FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md`,
Pluma's macOS artifact is ad-hoc-signed, checksummed, and GPG-signed with Pluma's own project
release-signing key — never an Apple Developer ID signature, never notarized. Full design:
`docs/release/MACOS_DIRECT_DISTRIBUTION.md`.

```bash
# 1. Checksum
shasum -a 256 -c Pluma-<version>.dmg.sha256

# 2. Project GPG signature (same identity and fingerprint as the Linux release-signing key)
gpg --import docs/release/flake-release-signing-public.asc
gpg --verify Pluma-<version>.dmg.asc Pluma-<version>.dmg
gpg --verify Pluma-<version>.dmg.sha256.asc Pluma-<version>.dmg.sha256
# Compare the signing key's fingerprint against the one published in
# docs/release/LINUX_RELEASE_SIGNING.md before trusting anything signed with it.

# 3. Local codesign self-integrity check (NOT a trust-chain or notarization claim)
codesign --verify --deep --strict --verbose=2 Pluma.app

# 4. Gatekeeper's own assessment -- EXPECTED TO REJECT this artifact; that is correct,
#    not a bug. See docs/release/USER_GUIDE.md for how to open it anyway.
spctl --assess --type execute --verbose=4 Pluma.app
```

**Production fingerprint (same identity used for Linux):**
`F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`

**What this does and does not prove:** the checksum confirms the file was not corrupted or
altered in transit. The GPG signature confirms it was produced by Pluma's own project release
process. The GitHub attestation (below) confirms it was built by this repository's own CI at
an exact commit. None of these, individually or together, are Apple's platform trust — Pluma
does not have an Apple Developer ID and does not claim one.

## SBOM

Every release-candidate CLI archive and desktop bundle ships a CycloneDX SBOM
(`scripts/release/generate_sbom.sh`, `dist/sbom/*.cdx.json`) listing every dependency and its
license, cross-referenced against `docs/legal/THIRD-PARTY-LICENSES.md`.

## LICENSE / NOTICE / third-party licenses

`LICENSE` (Apache-2.0, unmodified upstream text), `NOTICE`, and
`docs/legal/THIRD-PARTY-LICENSES.md` (every shipped third-party component) are bundled inside
every distributed CLI archive and desktop installer (`T05-04`, verified against the actual
installed bundle by `scripts/release/install_test.sh`, not merely trusted from bundler config).
Also reachable at runtime via the `pluma license` CLI command and the desktop About screen
(`src/about.rs`).
