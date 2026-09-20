# Pluma — quickstart and recovery guide

This guide ships inside every release archive. It covers the CLI only; the
desktop app has its own in-window guidance (folder-picker labels, notices)
and does not require reading this file first.

Pluma is local-first and offline: it never connects to a network, never
requires a sign-in, and never uploads a vault anywhere on its own. Every
command below works with no internet connection.

This is a **T05-03 unsigned developer release candidate
(`UNSIGNED_DEVELOPER_RC`)**. The binaries in this archive are not signed or
notarized; see "Verifying what you downloaded" below for what that does and
does not mean.

## 1. What's in this archive

- `pluma` (`pluma.exe` on Windows) — the CLI, and the name to use going
  forward.
- `flake` (`flake.exe` on Windows) — the exact same program under this
  product's name before its 2026-09-20 rename to Pluma. Kept as a
  deprecated compatibility alias for anyone with existing scripts or muscle
  memory; it will not be removed without a separate, explicitly governed
  breaking release.
- `fehrest` (`fehrest.exe` on Windows) — the exact same program under its
  original historical name, kept for anyone with existing scripts or muscle
  memory. All three (`pluma`/`flake`/`fehrest`) are built from identical
  source; none has behavior the others lack.
- `pluma-migrate` (`pluma-migrate.exe` on Windows) — a standalone tool for
  moving an older-format vault onto the current format. Only needed if you
  have a vault created by a pre-1.0 build. `flake-migrate` ships alongside
  it as the same deprecated-alias compatibility name.
- This guide.
- `LICENSE`, `NOTICE`, `docs/legal/THIRD-PARTY-LICENSES.md` — this
  product's own license and every third-party component's license (see
  section 11 below).

## 2. Quickstart

Create a vault:

```
pluma canonical-init --vault /path/to/my-vault
```

Create a project and capture a note:

```
pluma project-create --vault /path/to/my-vault --name "My first project"
pluma capture --vault /path/to/my-vault --project <project-id> --body "Hello, Pluma."
```

List commands and their exact flags at any time:

```
pluma --help
```

Nothing above touches any location outside the `--vault` path you gave it.
Pluma does not scan your home directory, does not read other applications'
data, and does not write anything until you tell it to.

## 3. Where your data lives

Pluma never chooses a data location for you implicitly from a command —
every command that touches a vault takes an explicit `--vault <path>`. The
desktop app suggests (but does not require) a per-user application-data
folder the first time you create a vault; you can always pick any other
folder instead, including a portable/removable drive.

A vault is a plain folder. Copying it, backing it up with your own tooling,
or moving it to another machine of the same platform works like copying any
other folder — nothing about it depends on an installer, a registry entry,
or a running service.

## 4. Backing up and restoring

Create a verified backup:

```
pluma backup-run --vault /path/to/my-vault --out /path/to/backup-dest
```

This copies the vault to a fresh location and independently re-verifies the
copy before reporting success — a backup that fails verification is
reported as a failure, not silently accepted.

Restore a backup to a new location:

```
pluma backup-restore --backup /path/to/backup-dest --out /path/to/restored-vault
```

`--out` must not already exist; `backup-restore` refuses to overwrite an
existing folder rather than silently merge or clobber it. If you want to
replace a vault with a restored copy, restore to a fresh path and then swap
the folder yourself once you've confirmed the restored copy is what you
expect.

## 5. Recovering from a crash or suspected corruption

If a vault was open during an unclean shutdown (power loss, forced
termination, a crashed process) and you are unsure of its state, do not
delete or edit it by hand. Instead, recover it to a fresh location:

```
pluma vault-recover --vault /path/to/my-vault --out /path/to/recovered-vault
```

`vault-recover` never modifies the original vault — it reads it, repairs
what can be safely repaired in the copy, independently re-verifies the
result, and reports the outcome. The original stays exactly as it was, byte
for byte, so a recovery attempt can never make things worse. If recovery
succeeds, switch to using the recovered copy; if it reports a problem it
could not resolve, the original vault is still there, untouched, for
further inspection.

## 6. Migrating an older-format vault

If you have a vault from a build that predates the current on-disk format,
use the standalone migration tool rather than opening it directly with
`pluma`:

```
pluma-migrate preview /path/to/old-vault
pluma-migrate import /path/to/old-vault /path/to/new-vault
```

`preview` reports what would be imported without writing anything.
`import` writes only to the new, separate destination — your original
vault is never modified or deleted by migration.

## 7. Updating

Pluma has no auto-updater and makes no network requests to check for
updates. To update, download a newer release archive, verify it (see
below), and replace the old binaries with the new ones. Your vaults are
plain folders outside the installation/archive location, so replacing the
binaries never touches your data. If a new version ever changes the vault
format, it will require an explicit migration step (like section 6 above)
rather than upgrading a vault's format silently in place.

To go back to an older version, keep the old archive's binaries around and
use them instead — an older `pluma` build refuses to open a vault written
in a newer format it doesn't understand, rather than silently
misinterpreting it.

## 8. Uninstalling (desktop app)

Uninstalling the desktop app removes the installed application files only.
It does not delete your vaults, and it does not delete backups you created
with `backup-run`. The uninstaller reports this retention explicitly.

## 9. Verifying what you downloaded

This release candidate is unsigned (`UNSIGNED_DEVELOPER_RC`): the binaries
and installers are not code-signed. Your OS will likely warn you about
running or installing an unrecognized/unsigned program — that warning is
accurate for this build.

Every archive published alongside this guide ships with a SHA-256 checksum
manifest. Confirm the archive you downloaded matches the published checksum
before running anything from it. A checksum match tells you the file was
not corrupted or altered in transit; it does not by itself prove who built
it — that assurance comes from a signature. See
`docs/release/RELEASE_VERIFICATION.md` for the exact verification commands
per platform (checksum, project GPG signature, GitHub build attestation,
and, on Windows once active, Authenticode).

### macOS: opening an unnotarized build (Gatekeeper)

Pluma's macOS release is distributed directly from Pluma's own README/docs and GitHub
Releases page, not through the Mac App Store, and is not Apple-notarized — see
`docs/canonical/FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md` for why.
The first time you try to open it, macOS Gatekeeper will refuse with a message like *"Pluma"
cannot be opened because it is from an unidentified developer* — this is Gatekeeper correctly
doing its job against an unnotarized build, not a bug.

To open it anyway (Apple's own supported per-app override — this does not disable Gatekeeper
or any other macOS security feature):

1. Try to open Pluma normally; macOS will refuse and show the warning above.
2. Open **System Settings → Privacy & Security**, scroll to **Security**, and click
   **Open Anyway** next to the message naming Pluma. (Older macOS: right-click — or
   Control-click — the app and choose **Open**, then confirm in the dialog that appears.)
3. macOS remembers this choice for this app going forward.

Full design and verification details: `docs/release/MACOS_DIRECT_DISTRIBUTION.md`.

## 10. Getting help

This is a developer release candidate, not a supported product release.
Run `pluma` with no arguments (or `pluma --help`) for the full command
list; each command's own error message names its exact required flags if
you omit one. That usage text is the authoritative reference, not this
guide.

## 11. License, source, privacy and reporting a problem

Run `pluma license` for these same facts on the command line at any time
(no `--vault` needed); the desktop app has an equivalent "About" screen
reachable from every top-level view.

- **License:** Apache License, Version 2.0. The full text ships as
  `LICENSE` in this archive; `NOTICE` and `docs/legal/THIRD-PARTY-LICENSES.md`
  (also included) record every third-party component this build ships and
  its exact license.
- **Source:** <https://github.com/TheHalfMoon/Pluma>
- **Privacy:** Pluma is local-first, offline and account-free. It does not
  sign in, sync to a cloud service, or send telemetry. No data leaves this
  device unless you explicitly export or disclose it (`export-run`,
  `package-export`).
- **Report a problem / get support:** <https://github.com/TheHalfMoon/Pluma/issues>
