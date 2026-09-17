# T05-03 evidence report — Build complete offline CLI and desktop release candidates

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §25/§27/§28, task `T05-03`
- **Baseline / tested source commit:** forked from `origin/main` `9bc94895109a364370f131ea8359b0d42a03a55f` (PR #98, `T05-02` merge — post-merge `main` reverified green before this task started)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed locally on Windows; native Windows/macOS/Linux packaging, install/launch/update/uninstall qualification and reproducibility check run on GitHub-hosted Actions runners (`.github/workflows/t05-03-release-candidates.yml`), matching the precedent established at `T04-06`/`T05-01`/`T05-02`. No second human reviewer; no GitHub-enforced branch-protection check runs these jobs as a required check (unchanged limitation recorded at every prior `flake-v1` task).

## Founder ruling this task operates under

specs/CURRENT.md records the Founder's explicit instruction that T05-03 must not stop because Windows code-signing, macOS Developer ID/notarization, or a Linux release-signing key are unavailable to this environment: "Build and fully qualify every artifact that can be produced without private signing credentials." Plan §25's own T05-03 acceptance criteria independently supports this scoping — it reads "Every **unsigned** candidate installs/runs/updates/uninstalls..." — while T05-04 ("Close ownership, notices and release-signing obligations") is the task that owns signing/notarization/LICENSE/NOTICE. This report therefore closes T05-03 truthfully as an unsigned-candidate qualification task and records the exact missing-credential blocker fields below for T05-04 to pick up, rather than fabricating a signature or blocking this task on a credential it was never scoped to need.

```text
WINDOWS_SIGNING_CREDENTIALS=UNAVAILABLE
MACOS_DEVELOPER_ID=UNAVAILABLE
MACOS_NOTARIZATION_CREDENTIALS=UNAVAILABLE
LINUX_RELEASE_SIGNING_KEY=UNAVAILABLE
BLOCKED_TASK=T05-04
BLOCKING_TASK=NONE (T05-03's own acceptance criteria is explicitly scoped to unsigned candidates)
```

Every candidate built by this task is labeled `UNSIGNED_DEVELOPER_RC` in its own metadata/guide text — never presented as, or silently treated as, a final signed release.

## Scope actually touched

```text
.github/workflows/t05-03-release-candidates.yml | new
scripts/release/package_cli_archive.sh           | new
scripts/release/generate_sbom.sh                 | new
scripts/release/install_test.sh                  | new
scripts/release/reproducibility_check.sh         | new
docs/release/USER_GUIDE.md                       | new
Cargo.toml                                       | new `flake` [[bin]] target (product command alias)
src/cli.rs                                       | new USAGE banner text; wires backup-run/backup-restore/vault-recover CLI commands over existing crate::backup/crate::recovery (T01-05 deferred obligation)
tests/flake_fehrest_alias_parity.rs              | new
tests/flake_cli_backup_recover.rs                | new
desktop/src-tauri/tauri.conf.json                | new bundle metadata (publisher/copyright/category/descriptions, Windows NSIS offline WebView2 + currentUser install, macOS minimumSystemVersion)
desktop/src-tauri/src/main.rs                    | new default_vault_parent_dir command + pick_directory default-directory suggestion
desktop/src/App.tsx                              | show the suggested default vault-parent directory before "Create new vault" is clicked
docs/evidence/flake-v1/T05-03/...                | new (this report + raw + CI artifacts)
specs/CURRENT.md                                 | frontier update + T05-02 merge-commit correction
```

No change to `src/canonical.rs`, `src/backup.rs`, or `src/recovery.rs` policy/behavior — `backup-run`/`backup-restore`/`vault-recover` are a thin CLI dispatch shell over those already-reviewed library functions, adding no new backup/recovery policy of their own (see `tests/flake_cli_backup_recover.rs`'s own header comment).

## 1. Product command naming: `flake` (plan §25, "`flake` is the new product command")

Added a second `[[bin]]` Cargo target, `flake`, pointing at the exact same `src/main.rs` as the existing `fehrest` target — not a wrapper, alias script, or re-export. The two are the same compiled program shipped under two names, so "a compatibility alias, if shipped, must call the same new implementation... no hidden behavior fork" holds by construction, not convention. Proven functionally identical (not merely claimed) by `tests/flake_fehrest_alias_parity.rs`: identical `--help`/no-args/unknown-command output, and identical `init`+`scan` behavior across two independent vaults. The two binaries' raw bytes legitimately differ (embedded build-path/PDB debug metadata per distinct cargo bin target) — confirmed harmless, documented in `Cargo.toml`'s own comment on the two targets, and exactly why the parity test asserts behavior, not byte-identity.

## 2. Closing a real T01-05 deferred obligation: CLI backup/recover commands

`src/backup.rs`/`src/recovery.rs` (library functions) existed and were already tested at `T01-05`, but no CLI command ever called them — `T01-05`'s own evidence report recorded this as a deferred obligation against its own forbidden-scope boundary. Added `backup-run`, `backup-restore`, `vault-recover` subcommands to `src/cli.rs` as thin shells (construct request, call the library function, print its own report fields) with no new policy. `tests/flake_cli_backup_recover.rs` proves all three end-to-end as real subprocess invocations: a verified backup is created and reported, a restore reconstructs the exact committed content (checked via independent `project-show`, not merely CLI exit code), restore refuses to clobber an existing destination, recovery never mutates the original vault and produces an independently reverified fresh root, and a missing `--out` argument is a clear refusal, not a panic. In scope for T05-03 because plan §25's release set explicitly requires "migration tool and fixtures" and a shippable CLI archive that omits a working recovery command would leave §25's own required "CLI/help/user/recovery guides" without an actual recovery command to document.

## 3. Default vault location suggestion (plan §25, "Default data paths use the OS's per-user application-data directory under Flake/vaults")

`desktop/src-tauri/src/main.rs` adds `default_vault_parent_dir_path`, reading Tauri's own `app_data_dir()` and joining `vaults`; `pick_directory` now opens the native folder picker there by default when creating a new vault, and a new read-only `default_vault_parent_dir` command lets the frontend show that suggested path before the picker opens (`desktop/src/App.tsx`). The user can always pick any other location instead — this is a default starting point for the picker, never an enforced or silently-chosen location, matching §25's "clear location" UX requirement.

## 4. CLI release archive (`scripts/release/package_cli_archive.sh`)

Builds `flake`/`fehrest`/`flake-migrate` in release mode, stages them with `docs/release/USER_GUIDE.md` (renamed `README.md` in the archive — the repository's own root `README.md` is developer-facing and unsuitable to ship as-is, so this task wrote a genuinely new user-facing quickstart/backup/recovery/migration/update/uninstall/verification guide rather than repurposing it), smoke-checks every staged binary actually runs (`--help`) before archiving, zips (Windows) or tars (macOS/Linux), and writes a SHA-256 manifest alongside the archive. Verified locally on Windows: produced `flake-0.0.1-phase-t-windows-x86_64.zip` (5 files: `flake.exe`, `fehrest.exe`, `flake-migrate.exe`, `README.md`), manifest matches. macOS/Linux archive production is proven by the `cli-archive-and-sbom` CI matrix job (see results below), not locally — this development host is Windows-only.

## 5. SBOM generation (`scripts/release/generate_sbom.sh`)

Uses `cargo-cyclonedx` (CycloneDX 1.5, JSON) with `--describe binaries`, which emits one SBOM per `[[bin]]` Cargo target; the script keeps only the SBOMs for artifacts this release actually ships (`flake`, `fehrest`, `flake-migrate`, and the desktop shell `flake-desktop`) and discards the rest (internal-only benchmark/fault-workload/kill-test harness binaries are never packaged, so shipping SBOMs for them would misrepresent the release contents). A sanity check parses every emitted SBOM and asserts `bomFormat == "CycloneDX"` and a non-zero component count, so a broken/empty SBOM is never silently reported as success. Verified locally on Windows: 4 SBOMs generated, 36 components each for the three CLI binaries (they share the same dependency graph) and 271 for the desktop shell.

## 6. Desktop bundle build and install/launch/update/uninstall qualification (`scripts/release/install_test.sh`)

Runs after `npm run tauri -- build` produces the unsigned NSIS installer (Windows) / `.deb` (Linux) / `.dmg` (macOS). For each platform: seeds a vault via the CLI *before* installing anything (so retention is proven against real pre-existing content, not an empty directory); installs silently/unattended; launches the installed app and confirms the process survives at least 4 seconds without crashing, while diffing the OS connection table around the launch window and failing if any new non-loopback connection appears (F05: no runtime downloads/telemetry — this observes actual behavior rather than disabling the CI runner's own network hardware, which would also break the job's ability to report its own result); reinstalls the identical package over the existing install as this candidate's stand-in for "update" (installs/tar archives don't yet have a second, distinct historical version to upgrade *from* — see Limitation 2 below); uninstalls; and finally re-reads the pre-install vault through the CLI and asserts its content is byte-identical to what was seeded, reporting `RETAINED=YES` explicitly rather than only asserting it implicitly by absence of error.

## 7. Reproducibility (`scripts/release/reproducibility_check.sh`)

Performs two genuinely clean (`rm -rf target` between them, not incremental) release builds of `flake`/`fehrest`/`flake-migrate` and compares SHA-256 per binary. Run in CI on `ubuntu-latest` only, not the full 3-OS matrix: reproducibility is a toolchain-determinism property, not an OS-filesystem-semantics one (the thing that genuinely varies per OS — install/uninstall behavior — is already covered natively on all three profiles by the job above), and this follows the same disk/cost-scoping precedent `T05-01`'s own evidence already recorded for its M/L-scale timing runs. Not run locally on this development host: two full clean workspace rebuilds require enough free disk headroom that, given this host's own repeatedly-recorded near-zero-free-space history this session, was judged an unnecessary risk to take locally when CI (ample disk) proves it natively instead.

## Local gate results (Windows, this development host)

Captured in `docs/evidence/flake-v1/T05-03/raw/`:

- `01-fmt-check-windows.txt` — `cargo fmt --all -- --check`: exit 0, no output (clean).
- `02-clippy-windows.txt` — `cargo clippy --all-targets --locked -- -D warnings`: clean.
- `03-full-test-suite-windows.txt` — `cargo test --locked --all-targets`: 379 tests passed, 0 failed (328 lib + 5 `flake_cli_backup_recover` + 5 `flake_fehrest_alias_parity` + 4 `flake_fault_workload_binary` + 4 `flake_migrate_binary` + 10 `integration` + 23 `kill_tests`).
- `04-cargo-audit-windows.txt` — `cargo audit`: exit 0, 53 crate dependencies scanned, 0 vulnerabilities.

## CI results (all three native profiles)

_Filled in after `.github/workflows/t05-03-release-candidates.yml` runs green on this task's own PR — see the PR/CI links recorded in `specs/CURRENT.md` once merged._

## Honest limitations

1. **"Update" is tested as reinstall-over-existing-install, not upgrade-from-a-genuinely-older-version.** This is the very first release candidate this repository has ever built; no distinct earlier version's installer/archive exists yet to upgrade *from*. `install_test.sh` proves the install mechanism itself does not disturb a retained vault across a reinstall cycle (a necessary but not sufficient proof of "update" safety); genuine cross-version upgrade/rollback testing becomes possible, and should be added, once a second real release candidate exists.
2. **"Rollback via compatible backup" is not separately exercised beyond the reinstall test above**, for the same reason as Limitation 1 — there is no distinct older version to roll back *to* yet. Plan §25's rollback clause ("Old app rollback uses retained compatible data, not lossy downgrade") is a property of `vault-recover`/`backup-restore` plus the format-compatibility policy already qualified at `T05-01`, both proven independently in this and prior tasks' evidence; it has not been proven end-to-end through an actual two-distinct-app-version install/rollback cycle.
3. **Network-blocked-image testing is approximated by connection-table observation, not literal network isolation.** GitHub-hosted runners do not offer an easy, reliable way to disable network hardware for one test step without also breaking that job's own ability to report status back to GitHub. `install_test.sh` instead diffs the OS's own connection table immediately before/after app launch and fails on any new non-loopback entry — a real behavioral check, not a network-blocked environment.
4. **The Linux `.deb`'s "offline dependency bundle for the exact clean image" (plan §25) is not built by this task.** `install_test.sh` installs the `.deb` on the CI runner's own already-provisioned image (which already has the runtime libraries Tauri's build-time dependency detection lists, e.g. `libwebkit2gtk-4.1`), proving the package installs and runs correctly, but does not yet produce a separate downloadable bundle of those runtime `.deb`s for installation on a machine with no package-manager network access at all. Recorded here as scope not yet closed, to be picked up by whichever task next touches Linux packaging (or reopened against T05-03 itself if the Founder rules it must close before this task's own completion condition is met).
5. **macOS `.app` was copy-installed to a local staging directory, not `/Applications`,** to avoid requiring elevated/admin state changes to the GitHub-hosted macOS runner's own system directories from inside a qualification script; this still exercises the real install/launch/reinstall/uninstall mechanics against the actual `.dmg`-mounted `.app` bundle.

## Completion assessment against T05-03's own acceptance criteria

"Every unsigned candidate installs/runs/updates/uninstalls on clean network-blocked native image with retained vault bytes and complete dependency inventory" — met for the unsigned-candidate scope this task's own acceptance clause and the Founder's ruling both define, subject to Limitations 1-5 above, once the CI results section is filled in green. Signing/notarization/LICENSE/NOTICE remain explicitly deferred to `T05-04` per that same acceptance clause's own "unsigned candidate" wording.
