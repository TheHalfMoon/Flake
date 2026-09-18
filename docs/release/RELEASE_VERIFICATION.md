# Release verification

How to independently verify a Flake release artifact. This page collects verification commands
for every mechanism Flake uses or intends to use; it does not restate their design (see
`docs/release/CODE_SIGNING_POLICY.md` for the policy overview and links to each mechanism's own
document).

A verification failure on any of the checks below means the artifact must not be trusted or
installed. Fail closed — do not proceed past a failed check.

## SHA-256 checksums

Every release-candidate CLI archive and desktop bundle ships alongside a `.sha256` manifest
(`scripts/release/package_cli_archive.sh`). Verify with:

```bash
sha256sum -c flake-<version>-<platform>.sha256
```

## GitHub artifact attestations (build provenance)

Confirms an artifact was built by this repository's own GitHub Actions workflow, at an exact
commit, rather than substituted or tampered with after the fact. Requires the
[GitHub CLI](https://cli.github.com/) (`gh`, version with `attestation` support):

```bash
gh attestation verify flake-<version>-<platform>.zip -R TheHalfMoon/Flake
```

A successful verification prints the exact source repository, workflow, and commit SHA the
artifact was built from. This is additional supply-chain evidence — it confirms provenance, not
platform trust (it does not make Windows SmartScreen, macOS Gatekeeper, or a Linux package
manager trust the artifact; those still require the mechanisms below where applicable).

## Windows — Authenticode (pending SignPath Foundation approval)

Not yet active — see `docs/release/SIGNPATH_ELIGIBILITY_PACKET.md`. Once active, verify with:

```powershell
signtool verify /pa /v flake-<version>-windows-installer.exe
```

A trusted result names SignPath Foundation as the certificate issuer (per SignPath's own model:
they vouch that the binary was built from Flake's own open-source repository, rather than
verifying a personally-identified certificate holder).

## Linux — GPG detached signature

The production public key is now published. Verify its fingerprint before trusting any release signature:

```bash
gpg --import docs/release/flake-release-signing-public.asc
gpg --list-keys --with-colons "285091250+TheHalfMoon@users.noreply.github.com" \
  | awk -F: '/^fpr:/ {print $10; exit}'
# Compare the printed fingerprint against the one published in
# docs/release/LINUX_RELEASE_SIGNING.md before trusting anything signed with it.

gpg --verify flake-<version>-linux-x86_64.tar.gz.asc flake-<version>-linux-x86_64.tar.gz
gpg --verify flake_<version>_amd64.deb.asc flake_<version>_amd64.deb
```

**Production fingerprint:** `F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`

## macOS — Developer ID and notarization (blocked, external, paid)

Genuinely blocked on a paid Apple Developer Program membership; no free/OSS path exists in
Apple's current program (`docs/release/CODE_SIGNING_POLICY.md`). Once available, verify with:

```bash
codesign --verify --deep --strict --verbose=2 Flake.app
spctl --assess --type execute --verbose=4 Flake.app
```

## SBOM

Every release-candidate CLI archive and desktop bundle ships a CycloneDX SBOM
(`scripts/release/generate_sbom.sh`, `dist/sbom/*.cdx.json`) listing every dependency and its
license, cross-referenced against `docs/legal/THIRD-PARTY-LICENSES.md`.

## LICENSE / NOTICE / third-party licenses

`LICENSE` (Apache-2.0, unmodified upstream text), `NOTICE`, and
`docs/legal/THIRD-PARTY-LICENSES.md` (every shipped third-party component) are bundled inside
every distributed CLI archive and desktop installer (`T05-04`, verified against the actual
installed bundle by `scripts/release/install_test.sh`, not merely trusted from bundler config).
Also reachable at runtime via the `flake license` CLI command and the desktop About screen
(`src/about.rs`).
