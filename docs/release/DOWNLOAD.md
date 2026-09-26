# Download Pluma

Pluma is distributed directly from the web. There is no app-store download, no account, and
no sign-in. Every platform's artifact below is independently verifiable before installation.

```text
WEBSITE_FIRST_DIRECT_DISTRIBUTION=YES
MICROSOFT_STORE_DISTRIBUTION=NO
MAC_APP_STORE_DISTRIBUTION=NO
FOUNDER_ZERO_COST_DISTRIBUTION_REQUIRED=YES
```

## Canonical distribution surface (v1)

This document plus the linked GitHub Release is the canonical web distribution surface for
v1. A future domain may point to this page without changing release semantics.

- Project source: <https://github.com/TheHalfMoon/Pluma>
- Releases (artifact backend): <https://github.com/TheHalfMoon/Pluma/releases>
- Verification instructions: [`RELEASE_VERIFICATION.md`](RELEASE_VERIFICATION.md)
- Code signing policy: [`CODE_SIGNING_POLICY.md`](CODE_SIGNING_POLICY.md)
- License: Apache License, Version 2.0 (`LICENSE`, `NOTICE`,
  `docs/legal/THIRD-PARTY-LICENSES.md` bundled inside every archive and installer)

## Current release

No final production release is published yet. Release qualification is in progress under
`T05-04` (see `specs/CURRENT.md`). The historical prerelease below is preserved as
qualification evidence, not as the final product.

| Release | State | Artifacts |
|---|---|---|
| [`v0.0.1-phase-t-rc.1`](https://github.com/TheHalfMoon/Pluma/releases/tag/v0.0.1-phase-t-rc.1) | Historical `UNSIGNED_DEVELOPER_RC` prerelease (Windows-only, built pre-rename as `Flake_...`/`flake-...`) | `Flake_0.0.1-phase-t_x64-setup.exe` SHA-256 `83c077a3b4c39bf0751a193c0c56e554597339cc6f868918f29a61238d8a0b44`; `flake-0.0.1-phase-t-windows-x86_64.zip` SHA-256 `1b8a66d789bb461a6d35333859d9c0b3a3def1c57f0b0ffd79441f5b978c8f3e` |

When a new qualified release is published (a separately Founder-authorized action), this
table gains one row per platform with, for every downloadable file: version, architecture,
file size, SHA-256, signature/provenance verification command, SBOM link, release notes,
installation instructions, uninstall instructions, and known trust limitations.

## Per-platform trust model

| Platform | Distribution | Trust model | Details |
|---|---|---|---|
| Windows (x86_64) | Direct download (NSIS installer + CLI `.zip`) | Unsigned direct distribution; project GPG signature + GitHub provenance; SmartScreen warnings expected and disclosed | `docs/release/WINDOWS_DIRECT_DISTRIBUTION.md`; `WINDOWS_AUTHENTICODE_TRUST=NOT_AVAILABLE`, `WINDOWS_AUTHENTICODE_TRUST_CLAIMED=NO` |
| macOS (aarch64) | Direct download (`.dmg`) | Zero-cost direct distribution; ad-hoc codesign + project GPG signature + GitHub provenance; Gatekeeper override required, never Apple trust | `docs/release/MACOS_DIRECT_DISTRIBUTION.md`; `MACOS_GATEKEEPER_TRUST=NOT_CLAIMED`, `MACOS_APPLE_PLATFORM_TRUST=NOT_CLAIMED` — **PASS** |
| Linux (x86_64) | Direct download (CLI `.tar.gz` + `.deb`) | Project-controlled GPG release signature + GitHub provenance | `docs/release/LINUX_RELEASE_SIGNING.md` — **PASS** |

## Verify before installing

1. Confirm the file came from this page or the linked official GitHub Release.
2. Verify its SHA-256 checksum byte-for-byte.
3. Verify the project GPG signature (and optionally the GitHub attestation).
4. Follow the platform's install instructions in [`USER_GUIDE.md`](USER_GUIDE.md).

A verification failure on any check means the artifact must not be trusted or installed. Fail
closed — do not proceed past a failed check.

Never disable Microsoft Defender globally, SmartScreen globally, Gatekeeper globally, or any
system/enterprise security control to install Pluma. The only supported paths are Windows'
per-file "More info" / "Run anyway" on a verified artifact and macOS' per-app "Open Anyway"
override.
