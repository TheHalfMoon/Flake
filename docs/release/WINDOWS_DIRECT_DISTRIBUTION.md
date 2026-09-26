# Windows direct distribution (website-first, zero-cost)

Full design for Pluma's Windows release path under
[`docs/canonical/FOUNDER_WEBSITE_FIRST_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-26.md`](../canonical/FOUNDER_WEBSITE_FIRST_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-26.md).
Mirrors the structure of [`MACOS_DIRECT_DISTRIBUTION.md`](MACOS_DIRECT_DISTRIBUTION.md) for the
equivalent Windows decision.

## What this is, and is not

Pluma distributes a Windows x86_64 NSIS installer and CLI archive directly from its own
README/docs and, once a version is published, its GitHub Releases page. This is:

- **not** Authenticode-signed with a publicly trusted certificate;
- **not** distributed through the Microsoft Store;
- **not** claimed to carry any Windows platform trust;
- **not** purchased, rented, or borrowed signing under any paid program.

It **is**:

- built reproducibly in CI from pinned source on a GitHub-hosted `windows-latest` runner
  (same build `T05-03` already qualifies);
- checksummed (SHA-256 manifest, produced and verified byte-identical);
- signed with Pluma's own project-controlled GPG release-signing identity (the same key
  already qualified for Linux artifacts in `T05-04`, fingerprint
  `F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`) — this proves the artifact came from the Pluma
  project's own release process, not from a Windows trust root;
- attested via GitHub's native build-provenance attestations, binding the artifact to the
  exact commit and workflow run that produced it;
- shipped with a CycloneDX SBOM, release notes, installation instructions, uninstall
  instructions, and an explicit statement of known trust limitations.

## Why distribute unsigned when SmartScreen will warn

Because the only zero-cost path to a trusted Authenticode signature available to this project
(SignPath Foundation) declined the application for insufficient public visibility on
2026-09-25, and the Founder explicitly declined the paid route, declined paid certificates,
declined paid cloud signing, and declined Store distribution. An unsigned direct download with
fully disclosed limitations and fully independent verification material (checksum, project GPG
signature, GitHub provenance) is the honest release model — a weaker, self-attested guarantee
than an Authenticode chain-of-trust signature, and documented as exactly that.

## SmartScreen / security UX, exactly as it will appear to a user

On first launch of an installer or binary downloaded from a browser, Windows SmartScreen
and/or Windows Defender reputation checks may warn that the app is from an unknown publisher,
is unrecognized, or might be unsafe, and may initially block execution. This is Windows
correctly doing its job against an unsigned, low-reputation binary — not a bug in Pluma's
build, and not something this project's CI or documentation works around or hides.

### Verifying and running Pluma anyway (safe instructions only)

1. Confirm the download came from the official Pluma distribution surface
   (`docs/release/DOWNLOAD.md` and the linked GitHub Release).
2. Verify the SHA-256 checksum byte-for-byte
   (`docs/release/RELEASE_VERIFICATION.md`, Windows section).
3. Verify the project GPG signature and optionally the GitHub attestation (same page).
4. Only then, if you personally choose to trust the verified artifact, use Windows' own
   per-file path: on the SmartScreen dialog click **More info**, then **Run anyway**.

Pluma's documentation and CI must never recommend disabling Microsoft Defender globally,
disabling SmartScreen globally, weakening system policy, bypassing enterprise controls, or
disabling security services. Only the per-file user override above is used.

## CI qualification

`.github/workflows/t05-04-windows-direct-distribution.yml` (`workflow_dispatch`-only, same
reservation as the Linux production-signing, macOS direct-distribution, and
provenance-attestation workflows):

1. Builds the unsigned Windows installer (`.exe`, NSIS, `currentUser` install mode) and CLI
   archive on a GitHub-hosted `windows-latest` runner from pinned source (same build `T05-03`
   already qualifies), plus the CycloneDX SBOMs.
2. Records pre-qualification SHA-256, exact source commit, workflow/run, architecture
   (`x86_64`), version, file sizes, and SBOM linkage.
3. Runs a real Windows signature inspection
   (`scripts/release/inspect_windows_signature.sh`, `signtool verify /pa /v` plus
   `Get-AuthenticodeSignature`); the expected result is `WINDOWS_AUTHENTICODE_STATUS=NOT_SIGNED`
   with `WINDOWS_AUTHENTICODE_TRUST=NOT_AVAILABLE` and
   `WINDOWS_AUTHENTICODE_TRUST_CLAIMED=NO` throughout — never a signing PASS.
4. Signs the installer, the CLI archive, and their SHA-256 manifests with Pluma's project GPG
   release-signing key (`scripts/release/sign_linux.sh`, which is artifact-agnostic despite its
   name — it wraps `gpg --detach-sign` around any file) and independently re-verifies every
   detached signature in a clean, disposable keyring seeded only from the published public
   key — never trusting the signing step's own exit code alone. Before touching the real
   secret, the same production path is confirmed to still fail closed with no credentials set.
5. Generates a GitHub build-provenance attestation for the installer (and CLI archive where
   technically available).
6. Runs `scripts/release/install_test.sh`'s clean-install/launch/reinstall/uninstall plus
   vault-retention qualification against this specific artifact: current-user install path is
   deterministic, the app starts after install, the pre-install Pluma vault is retained
   untouched, reinstall-over-existing (the stand-in for "update", since Pluma has no
   auto-updater) does not disturb the vault, and uninstall removes application files while
   retaining the vault and reporting that retention.
7. Verifies `pluma.exe` alongside the deprecated compatibility aliases (`flake.exe`,
   `fehrest.exe`, `pluma-migrate.exe`, `flake-migrate.exe`) behave identically (the alias
   parity already proven by `tests/flake_fehrest_alias_parity.rs` is re-confirmed against the
   staged binaries, not merely trusted from source).
8. Confirms the legacy bundle identifier behavior (`app.flake.desktop.phase-t`, preserved so
   existing installs do not orphan data) and checks no arbitrary developer-only artifact is
   presented as a production download.

Exact CI run evidence, once executed, is recorded in `specs/CURRENT.md` and
`docs/evidence/flake-v1/T05-04/REPORT.md`'s addenda — never claimed in this design document
alone.

## Update model (no auto-updater)

Pluma has no auto-updater and makes no network requests to check for updates
(`docs/release/USER_GUIDE.md` section 7). Updates are manually downloaded verified packages:
download the newer release, verify checksum/signature/provenance before replacing anything,
then replace the old binaries (CLI) or run the newer installer over the existing install
(desktop). Vaults are plain folders outside the installation location, so replacing binaries
never touches user data. A newer on-disk format requires an explicit migration-to-new-root
after backup — never a silent in-place upgrade. Rollback is installing/using the retained
older verified binaries, which refuse (rather than misinterpret) vaults written in a newer
format they do not understand. If a Tauri updater is ever introduced, its signatures must be
cryptographically verified with the private key never entering the repo, invalid/tampered
updates must fail closed, and rollback must be qualified where canonical requirements require
it — until then, no updater is invented.

## Verifying a downloaded Windows artifact

See [`RELEASE_VERIFICATION.md`](RELEASE_VERIFICATION.md)'s Windows section for the exact
commands. In short: verify the SHA-256 checksum, verify the GPG signature against the
published Pluma release-signing public key, optionally verify the GitHub attestation, and
inspect the Authenticode state (expected: not signed) — none of which requires or implies
Windows platform trust, which this artifact does not have.
