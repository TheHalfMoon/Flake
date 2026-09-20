# macOS direct distribution (zero-cost)

Full design for Pluma's macOS release path under
[`docs/canonical/FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md`](../canonical/FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md).
Mirrors the structure of [`LINUX_RELEASE_SIGNING.md`](LINUX_RELEASE_SIGNING.md) for the
equivalent macOS decision.

## What this is, and is not

Pluma distributes an ad-hoc-signed `.dmg` directly from its own README/docs and, once a
version is published, its GitHub Releases page. This is:

- **not** an Apple Developer ID signature;
- **not** notarized;
- **not** stapled;
- **not** distributed through the Mac App Store;
- **not** claimed to carry any Apple platform trust.

It **is**:

- ad-hoc-codesigned (`codesign --sign -`), which lets `codesign --verify` confirm the bundle's
  own internal integrity (its contents have not been altered since signing) — a weaker,
  self-referential guarantee than a Developer ID chain-of-trust signature, and documented as
  exactly that;
- checksummed (SHA-256);
- signed with Pluma's own project-controlled GPG release-signing identity (the same key
  already qualified for Linux artifacts in `T05-04`, fingerprint
  `F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`) — this proves the artifact came from the Pluma
  project's own release process, not from Apple;
- attested via GitHub's native build-provenance attestations, binding the artifact to the
  exact commit and workflow run that produced it.

## Why ad-hoc signing at all, if it can't pass Gatekeeper

Three independent reasons, none of them "to fool Gatekeeper" (it is not fooled — `spctl
--assess` correctly rejects this artifact, and that is confirmed in CI, not hidden):

1. `codesign --verify --deep --strict` gives a real (if weak) tamper-evidence check on the
   bundle's own contents, on top of the SHA-256 checksum and the project GPG signature.
2. Unsigned Mach-O binaries can behave differently under macOS's hardened runtime and
   Gatekeeper's path-based quarantine handling than ad-hoc-signed ones; ad-hoc signing keeps
   the tested artifact closer to what a real Developer ID build would behave like.
3. It reuses the exact `scripts/release/sign_macos.sh` mechanics already built and CI-proven
   for `T05-04`'s test-signing pipeline, in a new, honestly-labeled mode
   (`DIRECT_DISTRIBUTION_MODE=1`, distinct from `TEST_SIGNING_MODE=1`, which is pipeline-mechanics-only and always deletes its own output).

## Gatekeeper behavior, exactly as it will appear to a user

On first launch of a `.dmg`/`.app` downloaded from a browser (which applies the
`com.apple.quarantine` extended attribute), macOS Gatekeeper will refuse to open it with a
message resembling *"Pluma" cannot be opened because it is from an unidentified developer* or,
on current macOS versions, *Apple could not verify "Pluma" is free of malware*. This is
Gatekeeper working correctly against an unnotarized binary — not a bug in Pluma's build, and
not something this project's CI or documentation works around.

### Opening Pluma anyway (Apple's own supported per-app override)

1. Try to open Pluma normally (double-click the `.app`, or open the mounted `.dmg` and drag
   it to Applications first). macOS will refuse and show the warning above.
2. Open **System Settings → Privacy & Security**, scroll to the **Security** section, and
   click **Open Anyway** next to the message naming Pluma. (On older macOS versions: right-click
   — or Control-click — the app in Finder and choose **Open** from the context menu; a dialog
   then offers an **Open** button that a plain double-click does not.)
3. Confirm in the follow-up dialog. macOS remembers this choice for that specific app.

This is the same override path Apple documents for any unnotarized developer build; no
Terminal command, and no system-wide security change, is required or recommended. Pluma's own
documentation and CI must never recommend `sudo spctl --master-disable` or any other
Gatekeeper-wide change — only this per-app override.

## CI qualification

`.github/workflows/t05-04-macos-direct-distribution.yml` (`workflow_dispatch`-only, same
reservation as the Linux production-signing and provenance-attestation workflows):

1. Builds the unsigned `.dmg` on a GitHub-hosted `macos-latest` runner from pinned source
   (same build `T05-03` already qualifies).
2. Ad-hoc-signs the actual `.app` inside it (`DIRECT_DISTRIBUTION_MODE=1`) and rebuilds the
   `.dmg` around the signed bundle.
3. Publishes a SHA-256 manifest for the `.dmg`.
4. Signs the `.dmg` and its manifest with Pluma's project GPG release-signing key
   (`scripts/release/sign_linux.sh`, which is artifact-agnostic despite its name — it wraps
   `gpg --detach-sign` around any file) and independently re-verifies both detached
   signatures in a clean, disposable keyring seeded only from the published public key —
   never trusting the signing step's own exit code alone.
5. Generates a GitHub build-provenance attestation for the `.dmg`.
6. Reconfirms `codesign --verify` (expected to pass) and `spctl --assess` (expected, and
   required, to fail/reject) against this specific artifact.
7. Re-runs `scripts/release/install_test.sh`'s mount/copy/launch/uninstall/vault-retention
   qualification against this specific ad-hoc-signed `.dmg` (not merely the unsigned `T05-03`
   candidate).

Exact CI run evidence, once executed, is recorded in `specs/CURRENT.md` and
`docs/evidence/flake-v1/T05-04/REPORT.md`'s addenda — never claimed in this design document
alone.

## Verifying a downloaded macOS artifact

See [`RELEASE_VERIFICATION.md`](RELEASE_VERIFICATION.md)'s macOS section for the exact
commands. In short: verify the SHA-256 checksum, verify the GPG signature against the
published Pluma release-signing public key, and optionally verify the GitHub attestation —
none of which requires or implies Apple's trust, which this artifact does not have.
