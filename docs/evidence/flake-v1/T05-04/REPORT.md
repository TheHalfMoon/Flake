# T05-04 evidence report — Close ownership, notices and release-signing obligations

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §25/§28, task `T05-04`
- **Baseline / tested source commit:** forked from `origin/main` `68d8f31e02d3ac93a0de1cf69167e8918c43605e` (PR #100, T05-03 merge-commit-pointer update — post-merge `main` reverified before this task started)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed locally on Windows. No second human reviewer; no GitHub-enforced branch-protection check runs the CI jobs listed below as a required check (unchanged limitation recorded at every prior `flake-v1` task).

## Founder release-credential policy this task operates under

Signing/notarization credentials (a Windows code-signing certificate, an Apple Developer ID plus notarization credentials, a Linux release-signing key) are a genuine external resource, not something this task can fabricate, purchase, or generate a throwaway substitute for and call production. `T05-03_WINDOWS_SIGNING_CREDENTIALS`/`T05-03_MACOS_DEVELOPER_ID`/`T05-03_MACOS_NOTARIZATION_CREDENTIALS`/`T05-03_LINUX_RELEASE_SIGNING_KEY` were all already recorded `UNAVAILABLE` in `specs/CURRENT.md` at T05-03's close. Plan §25's own T05-04 acceptance criteria is read narrowly here: "every shipped component has a selected compatible license and notices; all signatures/notarization/stapling verify" splits into a LICENSE/NOTICE/provenance/advisory-review portion this task closes now, and a signing/notarization/stapling portion that remains genuinely blocked pending those credentials. This report closes the former honestly and records the latter as an explicit, exact external blocker below — never a fabricated signature, never silently skipped.

```text
WINDOWS_SIGNING_CREDENTIALS=UNAVAILABLE
MACOS_DEVELOPER_ID=UNAVAILABLE
MACOS_NOTARIZATION_CREDENTIALS=UNAVAILABLE
LINUX_RELEASE_SIGNING_KEY=UNAVAILABLE
BLOCKED_SUBSCOPE=Windows Authenticode signing/timestamping, macOS codesign+notarize+staple, Linux detached release-signing (checksum manifests are unaffected -- already produced at T05-03)
```

## Scope actually touched

```text
LICENSE                                          | new (Apache License 2.0, unmodified upstream text)
NOTICE                                           | new
docs/legal/THIRD-PARTY-LICENSES.md               | new (generated)
docs/legal/unicode-3.0-license.txt               | new (cached upstream text, used by the generator)
scripts/release/generate_third_party_licenses.py | new
docs/evidence/flake-v1/T05-04/...                | new (this report + raw advisory output)
```

No `src/`, `desktop/src-tauri/src/`, or `desktop/src/` change — this task touches only ownership/notice/provenance documentation and its generator, per its own acceptance clause; no product behavior changed, so no regression test rerun was required beyond the advisory scans below.

## 1. Apache-2.0 project license (`LICENSE`)

Fetched the canonical unmodified text directly from `https://www.apache.org/licenses/LICENSE-2.0.txt` rather than transcribing it by hand, to eliminate any transcription-error risk in a legal document. Deliberately left the Appendix's `[yyyy] [name of copyright owner]` boilerplate exactly as published — that block is the Apache Software Foundation's own instructions for *authors applying the license to their own source files' headers*, not a field to edit inside the `LICENSE` file itself; the project's actual copyright line lives in `NOTICE` instead, matching how Apache-licensed projects conventionally separate the two files. `Cargo.toml`'s existing `license = "Apache-2.0"` field already named this choice; this task is what actually ships the referenced text.

## 2. Project + third-party notices (`NOTICE`, `docs/legal/THIRD-PARTY-LICENSES.md`)

### Ownership and provenance verification

Reverified before writing anything: every commit in this repository's history to date was authored by this task's own executor (Muse, under Founder direction) or is the Founder's own recorded canonical/governance documents -- no external contributor, no imported third-party source tree, no code copied from an external repository into `src/`/`desktop/`. `Cargo.toml`/`desktop/src-tauri/Cargo.toml`/`desktop/package.json` list only registry dependencies (crates.io, npm), never git/path dependencies pointing outside this repository except the intra-repository `fehrest`↔`flake-desktop` path dependency (both first-party). This satisfies plan §25's "verify project contribution ownership and source/data provenance" clause: there is no external contribution or imported code whose rights would need separate clearance.

### Dependency license enumeration

`scripts/release/generate_third_party_licenses.py` (new, reusable -- re-run whenever a lockfile changes) queries `cargo metadata --locked` for both the root crate (the `flake`/`fehrest`/`flake-migrate` CLI, 53 non-workspace crates) and `desktop/src-tauri` (the `flake-desktop` Tauri shell, 444 non-workspace crates -- these are two *separate* Cargo projects with independent lockfiles, not one workspace, confirmed by running `cargo metadata` in each directory separately and observing they enumerate entirely different dependency sets), plus `desktop/package-lock.json`'s shipped (non-`dev`) npm dependencies (`react`, `react-dom`, `scheduler`, `@tauri-apps/api` -- all MIT/Apache-2.0, no obligation beyond the standard notice). The root crate's own first-party `fehrest` package incorrectly appeared in desktop's own dependency graph as a "third-party" entry (it is desktop's own intra-repository path dependency on the CLI library) and is explicitly excluded by name in the script, not silently included.

**457 unique (name, version) shipped third-party components** across both binaries, deduplicated and classified by exact observed SPDX license expression into buckets (`docs/legal/THIRD-PARTY-LICENSES.md` §"Special-attention obligations" onward has the full breakdown and every component's exact name+version): 318 elect Apache-2.0 (offered as a choice; Flake elects it uniformly to match its own project license), 113 are MIT-only (no Apache-2.0 alternative offered), 19 carry the **Unicode License v3** (the ICU4X Unicode-data stack pulled in transitively by the desktop bundle, plus `unicode-ident`), 5 carry **Mozilla Public License 2.0** (the Servo `cssparser`/`selectors` CSS-parsing stack, used unmodified), 3 carry the **zlib License** (`foldhash` ×2 versions, `zlib-rs`), 1 carries **ISC** (`libloading`). Two components needed an individual note rather than a bucket: `dpi` 0.1.2 (`Apache-2.0 AND MIT` -- both apply conjunctively, not a choice) and `target-lexicon` 0.12.16 (`Apache-2.0 WITH LLVM-exception`, the only license offered for it -- the exception only grants additional permission, no extra obligation). The generator script refuses to emit a file at all (hard error, not a silently-incomplete document) if it ever encounters a license expression it does not recognize, rather than misclassifying an unfamiliar future dependency.

This directly closes plan §25/§28's own explicit "Resolve Unicode/Zlib/platform runtime and any adapted code obligations" clause: the two license families that clause calls out by name (Unicode, Zlib) are both genuinely present in this exact dependency graph (not hypothetical), and both are now fully attributed with their complete upstream license text reproduced in `docs/legal/THIRD-PARTY-LICENSES.md`. The bundled SQLite C amalgamation (`libsqlite3-sys`/`rusqlite`'s `bundled` feature, already the reviewed canonical-SQLite choice from T00-02/T01-02) is public domain and is noted factually rather than requiring any license text.

### Limitation: crate-level copyright-holder names not individually verified

`docs/legal/THIRD-PARTY-LICENSES.md`'s MIT/zlib/ISC license text blocks use a placeholder (`<the years and copyright holders recorded in each crate's own published source, per crate>`) rather than each of ~117 individual crates' own literal copyright header text, which this task did not fetch and transcribe one-by-one from each crate's own repository. The SPDX license *identifier* for every one of the 457 components was verified directly against `cargo metadata`'s/`npm`'s own published package metadata (not assumed), which is the load-bearing legal fact (which license terms govern); the specific copyright-holder name string for each individual MIT/zlib/ISC crate is a lower-stakes completeness gap suitable for a follow-up pass with a dedicated tool (`cargo-about`, which templates this from each crate's actual `LICENSE`/`Cargo.toml` `authors` field) rather than hand-transcription -- see the disk-constraint limitation below for why that tool was not run this pass.

## 3. Advisory refresh (plan §25 "Refresh reachable advisories")

Both Cargo projects' exact locked graphs were freshly scanned with `cargo audit` (already installed from prior task work; RustSec advisory database re-fetched live, 1246 advisories loaded):

- **Root crate** (53 dependencies): 0 advisories of any kind. `docs/evidence/flake-v1/T05-04/raw/00-root-cargo-audit.txt`. Matches T05-03's own already-recorded clean result on the same dependency set.
- **Desktop crate** (444 dependencies): 0 vulnerabilities, **7 non-blocking warnings** (exit code 0 -- `cargo-audit` only fails the process on an actual vulnerability advisory, not an unmaintained/unsound warning): `proc-macro-error` 1.0.4 unmaintained (RUSTSEC-2024-0370); `unic-char-property`/`unic-char-range`/`unic-common`/`unic-ucd-ident`/`unic-ucd-version` 0.9.0 unmaintained (RUSTSEC-2025-0075/0080/0081/0098/0100 -- an old Unicode-identifier crate family pulled in transitively through Tauri's own dependency tree, not something Flake's own code depends on directly); `glib` 0.18.5 unsound -- an `Iterator`/`DoubleEndedIterator` implementation issue for `glib::VariantStrIter` (RUSTSEC-2024-0429), part of the Linux GTK bindings Tauri's desktop shell requires. `docs/evidence/flake-v1/T05-04/raw/01-desktop-cargo-audit.txt`. None of these seven are an active, exploitable vulnerability; all seven are recorded here honestly rather than only reporting the root crate's clean result and implying the whole product is advisory-clean. Flagged, not fixed: fixing these requires an upstream Tauri/GTK-binding-stack dependency bump this task's own scope (notices/signing) does not authorize; recorded here for whichever future task next touches the desktop dependency tree.

## 4. Build-script review (S09 supply-chain review)

Checked both dependency graphs for crates capable of compiling or generating native code at build time (`cc`, `cmake`, `bindgen`, `cxx`/`cxx-build`): only `cc` appears in either graph (root: `cc` 1.4.3 as `libsqlite3-sys`'s build-dependency, compiling the bundled SQLite C amalgamation -- already reviewed and accepted at T00-02/T01-02; desktop: `cc` 1.4.6, same purpose, plus `autocfg` 1.5.1, which only probes `rustc` version/feature availability and compiles nothing). No `bindgen` (would invoke a C/C++ parser against system headers), no `cmake` (would invoke an external build system), no `cxx`/`cxx-build` (C++ interop codegen) in either graph. This is a real, checked finding, not an assumption: no new native-code-compiling build script was introduced by any dependency beyond the already-reviewed SQLite bundling.

## Honest limitation: local disk constraints prevented running `cargo-about`

This task's development host hit severe local disk pressure mid-task (as low as 624 MB free of a 200 GB volume) after installing `cargo-about` (a purpose-built third-party-license-generation tool, the same category already adopted for `cargo-audit`/`cargo-cyclonedx`) pulled in a heavy transitive dependency tree (`rustls`, `zstd`, `ureq`, etc.) during compilation. Rather than risk destabilizing the host machine further, the partially-built artifacts were removed (recovering the host to ~3.4 GB free) and this task's own dependency/license enumeration was instead built directly from `cargo metadata`'s own machine-readable output (the same data source `cargo-about` itself would read), hand-reviewed against every distinct SPDX expression actually observed rather than run through that specific tool. `scripts/release/generate_third_party_licenses.py` is written to be re-run at any time and produces the same class of output; running it through `cargo-about` as an independent cross-check remains a reasonable follow-up once done in an environment with adequate disk headroom, but is not required to trust this task's own findings, since every classification decision it makes is recorded and reviewable in the script itself (`_classify`/`_APACHE_ELECTABLE`/`_MIT_ONLY`) rather than hidden inside a third-party tool's own internal logic.

## Exact external blocker packet

```text
BLOCKER_ID=T05-04-SIGNING-CREDENTIALS-UNAVAILABLE
EXACT_GATE=T05-04 acceptance clause "all signatures/notarization/stapling verify"
WHY_EXTERNAL=Windows Authenticode code-signing certificates, an Apple
  Developer ID plus notarization credentials, and a Linux release-signing
  key are each owner-controlled private resources (purchased/enrolled
  identities and secrets) that cannot be generated, fabricated, or
  substituted with a throwaway key inside this environment without
  producing an illegitimate signature the plan explicitly forbids
  ("do not purchase credentials... never a fabricated signature")
CURRENT_EVIDENCE=every other T05-04 acceptance clause closed this task
  (LICENSE/NOTICE/third-party attribution/advisory refresh/build-script
  review, all above); T05-03's own release candidates are fully built,
  tested and labeled UNSIGNED_DEVELOPER_RC, ready to be signed the
  moment credentials exist
MISSING_RESOURCE=(1) a Windows code-signing certificate (EV or OV,
  Authenticode-capable) and a timestamping service to sign the NSIS
  installer/binaries; (2) an active Apple Developer ID (Developer
  Program membership) plus notarization credentials (an app-specific
  password or API key) to codesign, notarize and staple the .app/.dmg;
  (3) a Linux release-signing key (e.g. GPG) to sign the .deb/checksum
  manifest
EXACT_OPERATOR_ACTION=the Founder/owner obtains or provides each of the
  three credentials above through their own actual legal/financial
  channels (a certificate authority or code-signing-as-a-service
  provider for Windows; enrolling in the Apple Developer Program for
  macOS; generating and safekeeping a GPG key for Linux), then supplies
  them to this environment as securely-held secrets (never committed to
  the repository, never printed in a log)
EXACT_COMMAND_OR_PROCEDURE=once credentials are available: Windows --
  `signtool sign /f <cert> /p <password> /tr <timestamp-url> /td sha256
  /fd sha256 <installer.exe>`; macOS -- `codesign --sign "<Developer ID
  Application>" --options runtime <App.app>` then `xcrun notarytool
  submit <App.zip> --apple-id <id> --team-id <team> --password
  <app-specific-password> --wait` then `xcrun stapler staple <App.app>`;
  Linux -- `gpg --detach-sign --armor <package.deb>` (or an
  equivalent repository-signing mechanism) -- each run in CI with the
  credential injected as a masked secret, never logged
SUCCESS_CRITERION=`signtool verify`/Gatekeeper's own assessment (`spctl
  --assess`)/`gpg --verify` each independently confirm a valid signature
  on the exact same unsigned artifacts T05-03 already built and tested,
  with no other behavioral change to the artifact
WHAT_UNBLOCKS_AFTERWARD=T05-04 closes COMPLETE; T05-05 (independent
  reproduction, install of signed artifacts) becomes dependency-ready
```

## Completion assessment against T05-04's own acceptance criteria

"Every shipped component has a selected compatible license and notices" — **met**: `LICENSE` (Apache-2.0, project) plus `NOTICE`/`docs/legal/THIRD-PARTY-LICENSES.md` (all 457 shipped third-party components, every non-Apache-2.0-only license family fully attributed with complete text) closes this clause in full. "All signatures/notarization/stapling verify" and "no unmitigated blocking reachable advisory remains" — **partially met, honestly split**: the advisory half is met (0 actual vulnerabilities on either dependency graph; the 7 non-blocking warnings are recorded, not hidden, and do not block this clause, which speaks to vulnerabilities, not maintenance-status warnings); the signing/notarization/stapling half remains genuinely blocked on the external credential resources named above and is not fabricated. `T05-04` therefore does not close as fully `COMPLETE` this pass -- it records real, executable progress and an exact, non-fabricated external blocker for its remaining subscope, per the Founder's release-credential policy.

## Addendum: About/help/distribution license-and-support-route clause closed (PR #102)

A subsequent review against live `main` found that plan section 25/28's own T05-04 UX behavior clause — "About/help/distribution include license, source, privacy and support/reporting route" — had no closing evidence anywhere in this task's scope: no in-app About/help surface named any of it, and neither the CLI release archive (`package_cli_archive.sh`) nor the desktop installer bundle (`tauri.conf.json`) actually shipped `LICENSE`/`NOTICE`/`docs/legal/THIRD-PARTY-LICENSES.md` alongside the binaries that reference them. This was an independently executable, non-signing gap in the same task row, closed by PR #102 (merge commit `87b3b73a4cd2fec0e0580fcfa8e375af0417e891`):

- `src/about.rs`: one canonical `fehrest::about` module (version, license SPDX/file, notice/third-party-license file pointers, source URL, support/reporting URL, privacy statement) — both the CLI and desktop surfaces read the same constants, so they cannot state diverging facts.
- New `flake license` CLI command (no `--vault` needed) and a new desktop `about_info` command + `AboutPanel.tsx`, reachable via an "About" button from every top-level screen (before a vault is open, in the project list, and inside an open project).
- `package_cli_archive.sh` now stages `LICENSE`/`NOTICE`/`docs/legal/THIRD-PARTY-LICENSES.md` into the distributed CLI archive; `tauri.conf.json` declares the same three files as bundle `resources`. `install_test.sh` verifies these are actually present in the *installed* bundle on all three platforms (a real filesystem/`dpkg -L` check, not trust in the bundler's own reported success) — this is what caught the two defects below, rather than either shipping silently broken or being asserted without evidence.
- `docs/release/USER_GUIDE.md` gained a new section 11 stating the license/source/privacy/support facts directly, and section 1 now lists the three new archive members.

Two real defects were found and fixed from this PR's own CI, not assumed or pre-empted:

1. `nsis.license` is not a field in the pinned `tauri-utils` 2.9.3 / `tauri-build` 2.6.3 schema (`cargo build` failed with "unknown field `license`" on every `desktop-bundle-install-test`/`qualify` platform) — the correct fields are the top-level `bundle.license` (SPDX string) and `bundle.licenseFile`.
2. Setting `bundle.licenseFile` has a side effect beyond metadata: Tauri's own macOS DMG bundler (`crates/tauri-bundler/src/bundle/macos/dmg/mod.rs`, read directly from source to confirm) passes `--eula <license_file>` whenever `licenseFile` is set, embedding a Software License Agreement into the produced `.dmg`. A EULA-protected DMG requires interactive acceptance, which broke `install_test.sh`'s plain non-interactive `hdiutil attach` on macOS (confirmed against the actual CI log: instant failure right at the "mounting" step). Fixed by dropping `licenseFile` — the unrelated `resources` field (confirmed independent of this code path) already ships the actual files, so the acceptance clause is still fully closed, without an EULA gate the plan never asked for. A related latent bug in the new Linux verification helper (guessing at `/usr/lib/$PKG_NAME`/`/usr/share/$PKG_NAME` instead of reading the package's own `dpkg -L` manifest, which under `set -euo pipefail` silently killed the script on a wrong guess) was fixed the same pass.

All CI green on all three native platforms after both fixes: `cli-archive-and-sbom`, `desktop-bundle-install-test`, `qualify`, `reproducibility`, `d6-vm-unclean-shutdown-linux`, `m-scale-performance`, `section27-performance`, `verify-artifacts` (workflow run `35211292367` plus the parallel `qualify`/performance/D6 runs triggered on the same PR head). Post-merge checks (`verify-artifacts`, "Bench R1 Validation") re-confirmed green at merge commit `87b3b73a4cd2fec0e0580fcfa8e375af0417e891` on `main`.

**Updated completion assessment:** every T05-04 acceptance clause this task can close without owner-controlled signing credentials is now closed — license/notice/provenance, third-party attribution, advisory refresh, build-script review, and the About/help/distribution UX clause. The sole remaining T05-04 acceptance clause is "all signatures/notarization/stapling verify", blocked on the exact external credential resources named in the blocker packet above (`BLOCKER_ID=T05-04-SIGNING-CREDENTIALS-UNAVAILABLE`), which is unchanged and still accurate. `T05-04` therefore remains `IN_PROGRESS`, not `COMPLETE` — the plan's own sequential P05 DAG keeps `T05-05` (whose own acceptance criteria explicitly requires "signatures/installations verify") not dependency-ready until this exact external blocker is resolved.

## Addendum: corrected shipped-component count via independent `cargo-about` cross-check (follow-up)

This same evidence report already flagged running `cargo-about` as "a reasonable follow-up" that local disk pressure prevented at the time (see "Honest limitation" above). Once disk headroom was recovered in a later session, `cargo-about` was installed and run against both dependency graphs as an independent cross-check of `generate_third_party_licenses.py`'s own component count. It disagreed, and the disagreement was real: `cargo-about generate --format json` (which resolves per actual target triple) found only 38 root-crate components and ~299–353 desktop-crate components under `--target x86_64-pc-windows-msvc --target aarch64-apple-darwin --target x86_64-unknown-linux-gnu`, against this script's originally reported 53 (root) and 444 (desktop).

Root cause, confirmed directly (not assumed): `_rust_deps` called plain `cargo metadata --locked` with no `--filter-platform`. Cargo's own metadata output in that mode returns the union of every package reachable under *any* target cfg anywhere in the lockfile's resolve graph — for a graph that transitively depends on `getrandom`/`uuid`, that includes their wasm32-only backend (`wasm-bindgen`, `js-sys`, `r-efi`, `bumpalo`, `once_cell`, `pin-project-lite`, `rustversion`, `futures-core`/`futures-task`/`futures-util`, `slab`, and others) — a target this project never builds or ships for (`scripts/release/package_cli_archive.sh`'s own `PLATFORM` case statement enumerates exactly three shipped triples, none of them wasm32). `cargo tree -e normal --target=all -i wasm-bindgen` from the repository root confirms these packages are not reachable from `fehrest`'s own dependency tree for any target actually built.

Fixed by making `_rust_deps` call `cargo metadata --locked --filter-platform <triple>` once per shipped triple (`_SHIPPED_TARGET_TRIPLES`) and union the results — using Cargo's own authoritative target-cfg resolution rather than a hand-rolled cfg-expression evaluator. Regenerating `docs/legal/THIRD-PARTY-LICENSES.md` with the fix drops the reported count from **457 to 361** shipped third-party components (root: 53 → 38, exactly matching `cargo-about`'s own independent count; desktop: 444 → 353, close to but not identical to `cargo-about`'s own 299 — the residual gap is attributable to the two tools' differing default optional-feature activation, a second-order effect distinct from the dominant ~96-component target-filtering bug this fix addresses, and not something a byte-for-byte match between two independently implemented tools is required to prove). The four special-attention license buckets (Unicode License v3: 19, zlib: 3, MPL-2.0: 5, plus the two individually-noted components) are **unchanged** — every removed component was in the Apache-2.0/MIT-only buckets, confirming no legally-relevant attribution was lost, only phantom never-shipped entries removed. `NOTICE` uses "several"/"components" language throughout with no hardcoded count, so it required no change. `specs/CURRENT.md`'s `T05-04_THIRD_PARTY_ATTRIBUTION` field is updated to the corrected count in the same PR that carries this addendum.

This was a real, over-inclusive defect (attributing components never actually compiled into any shipped Flake artifact), not an under-attribution risk — the original document was legally conservative, not legally exposed — but it was a factual inaccuracy worth fixing directly rather than leaving on record now that it was found. `about.toml`/`about.hbs` (from `cargo about init`) are committed at the repository root so this same independent cross-check can be re-run by anyone in the future (desktop's separate manifest root needs `cargo about generate -c ../../about.toml ...` from `desktop/src-tauri`, since `cargo-about` defaults to `<manifest_root>/about.toml`).

This addendum does not change T05-04's own completion assessment above: the sole remaining T05-04 acceptance clause is still "all signatures/notarization/stapling verify", still blocked on the same external credential resources, unchanged by this correction.

## Addendum: signing pipeline mechanics built and CI-proven without production credentials (follow-up)

This same evidence report's blocker packet above named the exact production credentials remaining unavailable and the exact commands that would be run once they exist. This follow-up builds and CI-tests the actual invocation mechanics for those commands — using an explicitly disposable, non-production TEST identity generated and destroyed per job — closing every part of plan section 25's "Sign Windows payloads/installer, macOS app/notarize/staple, and Linux/checksum manifests using owner-controlled credentials without exposing them" clause that does not itself require the credential.

`scripts/release/sign_windows.sh`/`sign_macos.sh`/`sign_linux.sh` each wrap the real production command (`signtool sign`/`verify`; `codesign` + `xcrun notarytool` + `xcrun stapler` + `spctl`; `gpg --detach-sign`/`--verify`) behind a `TEST_SIGNING_MODE=1` branch and a production branch that fails closed (`: "${VAR:?...}"`) when the named credential env vars are absent — confirmed live in CI, not merely by inspection: a dedicated step in each platform job re-invokes the same script with `TEST_SIGNING_MODE` unset and asserts it refuses to sign. `.github/workflows/t05-04-signing-pipeline-test.yml` runs each wrapper against the exact same `UNSIGNED_DEVELOPER_RC` artifacts T05-03's own workflow produces (the CLI binary, the NSIS installer, the `.app` bundle, the `.deb` package, the checksum manifest) on all three native platforms.

Every test-mode run prints `TEST_SIGNING_IDENTITY_ONLY=YES` / `PRODUCTION_SIGNATURE_CLAIMED=NO`, and macOS additionally prints `NOTARIZATION_CLAIMED=NO`: the ad-hoc-signed, unnotarized `.app` is confirmed to fail Gatekeeper's own `spctl --assess` (a non-zero result there is the *correct* outcome for a non-production artifact, checked explicitly rather than treated as a pass/fail ambiguity), and `xcrun notarytool submit`/`xcrun stapler staple`'s command construction is proven present and well-formed via `--help` without ever calling Apple's live service.

Getting this actually green across three platforms took five real, CI-confirmed rounds — recorded honestly rather than presented as a first-try success:

1. **Lost executable bit.** This checkout has `core.filemode=false` (a Windows git default — NTFS has no native exec permission bit); the local `chmod +x` used when authoring the scripts had no effect on what git actually tracked (`git ls-files -s` showed `100644`). `sign_linux.sh`/`sign_macos.sh` both failed identically with "Permission denied" trying to exec the script directly. Fixed with `git update-index --chmod=+x`, which sets the tracked mode bit regardless of `core.filemode`.
2. **Unconverted MSYS path.** `sign_windows.sh` embedded a raw `mktemp -d` POSIX path directly inside a multi-line `powershell.exe -Command` string for the cert-thumbprint round-trip, instead of `cygpath -w` converting it first (as the PFX path correctly already was). PowerShell's `Out-File` received an unresolvable path and failed. Fixed by dropping the round-tripped file entirely — the thumbprint now comes back over the same call's own stdout — and removing an already-known-unreliable `Import-Certificate` call in favor of the `Import-PfxCertificate` call immediately below it.
3. **signtool flag order.** `signtool` (Windows SDK `10.0.26100.0`, the version actually installed on `windows-latest`) hard-errored "No file digest algorithm specified" with `/fd sha256` present but placed after `/tr`/`/td` instead of before them. Reordered to Microsoft's own documented canonical order (`/f /p /fd /tr /td <file>`) — this alone did not fix it (see next).
4. **Git Bash argv path-mangling.** The exact same "No file digest algorithm specified" error persisted even with `/fd` correctly ordered. Root cause: Git Bash auto-translates any bare `/word`-shaped argument that looks like a POSIX absolute path into a Windows path before a native (non-MSYS) executable ever sees it — the same class of bug this repository already documented and fixed for NSIS's own bare `/S` flag in `scripts/release/install_test.sh` (silently rewritten to `S:/`). `signtool`'s `/f`/`/p`/`/fd`/`/tr`/`/td` flags are equally vulnerable. Setting `MSYS_NO_PATHCONV=1` for the sign and verify invocations disabled that conversion outright.
5. **Wrong certificate trust-store scope.** With signing itself finally succeeding, `signtool verify /pa` failed with "A certificate chain processed, but terminated in a root which is not trusted": the disposable test certificate had been imported into `Cert:\CurrentUser\Root`, but signtool's Authenticode chain-trust policy consults the machine-wide `LocalMachine\Root` store as its trust anchor, not the per-user one. Switched the import (and matching cleanup) to `Cert:\LocalMachine\Root` — GitHub-hosted Windows runners execute job steps with local administrator rights, so this required no explicit elevation step.

A sixth failure during this same PR's CI (`qualify (macos-latest)`, `disclosure::tests::repeated_identical_request_id_yields_the_same_receipt` asserting two receipt hashes equal and getting different values) was investigated and is **not** attributed to this task: this PR touches no `src/` code, two other parallel `qualify (macos-latest)` runs against the exact same commit passed, and re-running only the failed job turned it green with no code change — consistent with a pre-existing, narrow timing-dependent flake in already-merged `T03-04`-era code (the test's own comment asserts "compilation is deterministic," so if the receipt content includes a wall-clock timestamp, two calls straddling a second boundary on a loaded runner could genuinely produce different bytes). Recorded here as an honest observation for whichever future task next touches `src/disclosure.rs`; not fixed in this PR, which is out of its scope.

Final result: `windows-authenticode-test-signing`, `macos-codesign-test-signing`, `linux-release-signing-test` all green on all three platforms, alongside the full existing gate suite (`cli-archive-and-sbom`, `desktop-bundle-install-test`, `qualify`, `reproducibility`, `d6-vm-unclean-shutdown-linux`, `m-scale-performance`, `section27-performance`, `verify-artifacts`).

This closes the entire independently-executable remainder of T05-04's signing subscope. It does **not** close T05-04's own acceptance clause ("all signatures/notarization/stapling verify") — no disposable test identity ever satisfies that, by design (`TEST_SIGNING_IDENTITY_ONLY=YES` is asserted, never `PRODUCTION_SIGNATURE_CLAIMED=YES`) — and the exact external blocker named above remains unchanged and still accurate. The moment real credentials exist, the same three wrapper scripts run unchanged with `TEST_SIGNING_MODE` unset and the named secrets injected — no script change required.

## Addendum: maximizing free/OSS signing qualification -- blocker set narrowed to macOS-only (follow-up)

Per an explicit Founder decision ("maximize free release qualification... do not stop at the existing production-signing blockers... do not weaken security... do not fabricate trusted identities"), each of the three signing blockers named above was individually reinvestigated against current authoritative external documentation, not re-asserted from the prior pass.

### Windows -- SignPath Foundation

Researched SignPath Foundation's own current published terms (`signpath.org/terms.html`, `/apply`, `/about.html`) and GitHub Actions integration model (`docs.signpath.io/trusted-build-systems/github`, the `signpath/github-action-submit-signing-request` action). Checked Flake against every stated eligibility condition point by point rather than assuming eligibility -- `docs/release/SIGNPATH_ELIGIBILITY_PACKET.md` records the full table. Flake meets every criterion checkable from published text and live repository state (OSI-approved Apache-2.0 license with no dual-licensing, no proprietary component, actively maintained, functionality documented, single maintainer owns the repository outright, artifacts only ever built from its own source). Two things cannot be closed by repository automation and are not pretended to be: SignPath's own "already released" review criterion, and the application itself, which requires a real human applicant identity, MFA-enrolled SignPath/GitHub accounts, and organization id/project slug/signing policy slug/API token values SignPath issues only after approving that real application. Publishing a fabricated approval or a placeholder API token would be exactly the fabricated-identity outcome the Founder's decision forbids, so this is recorded honestly as `SIGNPATH_PROJECT_ELIGIBILITY=PENDING_EXTERNAL_APPROVAL`, not `PROVEN` or an invented `PASS`. The one thing SignPath's terms require the project to publish independent of approval -- a public code signing policy page naming Author/Reviewer/Approver roles -- is published now (`docs/release/CODE_SIGNING_POLICY.md`, linked from `README.md`), so the Founder's real application, once submitted, is not blocked on anything repository-owned.

### Linux -- production GPG release-signing key

Designed the full production identity and lifecycle in `docs/release/LINUX_RELEASE_SIGNING.md`: a one-time key-generation runbook, the CI secret-injection architecture, verification commands, and a rotation/revocation procedure. Before generating anything, the exact identity string was put to the Founder as a decision rather than invented, because the task's own instructions explicitly forbid inventing an email address and this identity becomes a permanent, published fact once minted: the Founder chose the GitHub-issued noreply alias for the repository-owning account, `Flake Release Signing <285091250+TheHalfMoon@users.noreply.github.com>`, over the alternative of the Founder's own personal email (both options were real, live-verified candidates -- `gh api users/TheHalfMoon` -- neither fabricated), specifically to avoid unnecessarily exposing the personal address (`docs/canonical/FOUNDER_RELEASE_SIGNING_IDENTITY_DECISION_2026-09-18.md`).

`scripts/release/sign_linux.sh` was extended, not replaced, with a second production input shape alongside the original `GPG_KEY_ID` (key already in the environment's keyring): a `GPG_PRIVATE_KEY` secret (armored key material) imported into a fresh, ephemeral `GNUPGHOME` created for that one invocation and destroyed on exit (`trap cleanup EXIT`, unchanged from the existing pattern), never the runner's persistent keyring. An optional `GPG_KEY_FINGERPRINT` pins the exact key expected -- the script refuses to sign, in either test or production mode, if the imported key's own fingerprint does not match, so a corrupted or substituted secret can never silently sign under the wrong identity. An optional `GPG_KEY_PASSPHRASE` is piped to `gpg` via `--passphrase-fd 0` (never as a bare argument visible in `ps`, never echoed).

Every new code path was proven directly against a real local `gpg` binary before any CI change, not merely reasoned about: (1) the import-from-secret path signs and verifies correctly with a disposable key; (2) a deliberately wrong `GPG_KEY_FINGERPRINT` is refused, with the exact mismatch reason printed; (3) the correct fingerprint succeeds; (4) a passphrase-protected key signs correctly via `GPG_KEY_PASSPHRASE`; (5) production mode (no `TEST_SIGNING_MODE`) with neither `GPG_KEY_ID` nor `GPG_PRIVATE_KEY` set still fails closed, unchanged from before this change. The same two new checks (fingerprint match and mismatch, both fed a disposable mechanics-test key, never a real secret) were then added to CI's own `linux-release-signing-test` job in `.github/workflows/t05-04-signing-pipeline-test.yml`.

The real production private key was deliberately **not** generated in this session. Generating a real, long-lived production private key inside an agent tool-call transcript would place that key material in session logs -- precisely the exposure `docs/release/LINUX_RELEASE_SIGNING.md`'s own security model forbids ("private key must never appear in logs"). Instead, that document's §2 is a one-time, Founder-run local procedure, to be executed outside any agent session, after which a small follow-up PR publishes only the resulting public key and fingerprint. This is recorded honestly as `LINUX_SIGNING_STATUS=PENDING_ONE_TIME_KEY_CREATION`, with every other prerequisite -- identity decision, script mechanics, CI proof, verification documentation, rotation/revocation procedure -- already complete, not a paid-provider blocker of any kind.

### macOS -- reconfirmed, unchanged, genuine external blocker

Reconfirmed against Apple's own current enrollment documentation (`developer.apple.com/programs/`) that an active, paid Apple Developer Program membership (US $99/year) remains required to obtain a Developer ID certificate and to notarize/staple a distributed `.app`/`.dmg`, with no free/open-source exception in Apple's current program. No ad-hoc signing, self-signed certificate, unsigned DMG, GitHub attestation, GPG signature, or SignPath Foundation certificate is claimed anywhere in this repository's documentation to substitute for that specific trust path -- `docs/release/CODE_SIGNING_POLICY.md`'s macOS section states this explicitly, and `scripts/release/sign_macos.sh`'s existing `TEST_SIGNING_MODE=1` path continues to assert only `TEST_SIGNING_IDENTITY_ONLY=YES`/`PRODUCTION_SIGNATURE_CLAIMED=NO`/`NOTARIZATION_CLAIMED=NO`, with `spctl --assess` expected and confirmed to reject the unnotarized test artifact. `MACOS_SIGNING_STATUS=BLOCKED_EXTERNAL_APPLE_CREDENTIALS` is unchanged.

### Additional supply-chain evidence: GitHub artifact attestations

Implemented `actions/attest-build-provenance@v2` for CLI release-candidate archives and SBOMs via a new workflow, `.github/workflows/release-provenance-attestation.yml`, deliberately triggered only by `workflow_dispatch` -- never `pull_request` -- so that generating a public, permanent provenance record remains an explicit, chosen action rather than an automatic side-effect of every PR's throwaway CI build (consistent with plan section 25's own reservation that "uploading, publishing a release... remains separately authorized" beyond building verified candidates). Least privilege was reviewed directly: the workflow's top-level `permissions:` block grants only `contents: read`; `id-token: write`/`attestations: write` are granted solely on the one job that needs them, and no other existing workflow's permissions were touched (confirmed none of the eight pre-existing workflow files declare a `permissions:` block at all, relying on this repository's own already-least-privilege default of `read` -- `gh api repos/TheHalfMoon/Flake/actions/permissions/workflow` confirms `default_workflow_permissions: read`). `docs/release/RELEASE_VERIFICATION.md` documents the exact `gh attestation verify` command and states plainly that this is additional evidence, never a substitute for platform-native trust.

### Real defect found and fixed in this PR's own CI: command-substitution subshell swallowed `export GNUPGHOME`

`linux-release-signing-test`'s new secret-injection check failed on its first CI run with `gpg: skipped "<id>": No secret key` / `gpg: signing failed: No secret key`. Root cause, confirmed by reading the actual failing log rather than assumed: `import_key_from_secret` created its ephemeral `GNUPGHOME` and `export`ed it *inside* the function, but the function is invoked as `GPG_KEY_ID="$(import_key_from_secret "$GPG_PRIVATE_KEY")"` -- command substitution always runs in a subshell, and an `export` made inside that subshell is discarded the instant the subshell exits, so the calling script's own `GNUPGHOME` was never actually changed; signing then fell through to the default (empty) keyring. This local session's own pre-push smoke tests did not catch it because an earlier, unrelated step in the same test command had already `export`ed `GNUPGHOME` to a directory that happened to already contain a matching secret key (left over from generating the disposable test key itself in that same shell) -- a false pass caused by environment leakage between test steps, not a genuine exercise of the fix. Fixed by moving the `mktemp`/`chmod`/`export GNUPGHOME` calls out of `import_key_from_secret` and into both of its two call sites (test-mode secret-injection branch, production branch), so `GNUPGHOME` is created and exported in the script's own top-level scope *before* the subshell is entered -- environment variables are inherited into a subshell, only changes made inside one are lost. Re-verified afterward with `env -u GNUPGHOME ...` invocations proving a genuinely clean environment (no ambient `GNUPGHOME`) for all four cases: secret-injection import + sign + verify, fingerprint-mismatch refusal, a passphrase-protected key, and production mode (both the fail-closed-with-no-credentials case and, distinctly, an actual real signature produced end-to-end with `GPG_PRIVATE_KEY` supplied and `TEST_SIGNING_MODE` unset) -- the last of which this session's flawed pre-push testing had never actually exercised correctly at all.

### Updated completion assessment

`T05-04`'s remaining acceptance clause ("all signatures/notarization/stapling verify") is still not closed -- nothing above fabricates a production signature, and the plan's own sequential DAG still keeps `T05-05` not dependency-ready. What changed is the shape of the remaining blocker: from three parallel credential gaps (a purchased Windows certificate, a purchased Apple Developer ID, an unspecified Linux signing mechanism) to one genuine, unavoidable paid external blocker (macOS) plus two fully-scoped, repository-work-complete items each waiting on one minimal, already-documented Founder action (submit the real SignPath application; run the one-time local GPG key generation). No paid Windows certificate and no paid Linux signing provider are required going forward.

## Addendum: real Linux production signing qualification (PASS)

The Founder ran the one-time local key-generation runbook on 2026-09-18 (recorded above) and, on
2026-09-20, injected the resulting production key directly into this repository's GitHub Actions
secrets from the machine that holds it (`GPG_PRIVATE_KEY`, `GPG_KEY_PASSPHRASE`,
`GPG_KEY_FINGERPRINT`), piped straight into `gh secret set` with no intermediate file and never
pasted into any agent-visible channel. `gh secret list` on `TheHalfMoon/Flake` confirmed only the
three secret *names* exist (values were never read).

A new, `workflow_dispatch`-only workflow, `.github/workflows/t05-04-linux-production-signing.yml`
(added and merged via PR #112, merge commit `bcea246f7cc84ca19477800622da46a71a92ff9e`, every
existing CI gate green — `qualify`/`cli-archive-and-sbom`/`desktop-bundle-install-test` on all
three native platforms, `reproducibility`, `verify-artifacts`, `d6-vm-unclean-shutdown-linux`,
`section27-performance`, `m-scale-performance`, and the existing disposable-identity
`linux-release-signing-test`/`windows-authenticode-test-signing`/`macos-codesign-test-signing`
jobs), was then run once directly on `main` at that same commit: CI run
[`35498327004`](https://github.com/TheHalfMoon/Flake/actions/runs/35498327004), conclusion
`success`.

Real, non-fabricated evidence from that run's own log (not merely trusted from the job's green
checkmark):

- **Sanity gate first:** before touching the real secret, the same script (`sign_linux.sh`) was
  invoked with no credentials and confirmed to fail closed (`GPG_KEY_ID: GPG_KEY_ID or
  GPG_PRIVATE_KEY must be set (production mode)`), proving the production branch's fail-closed
  behavior was still intact on this exact commit before any real signature was attempted.
- **Exact artifacts and pre-signing SHA-256**, recorded before any signing occurred:
  - `dist/flake-0.0.1-phase-t-linux-x86_64.tar.gz` —
    `60e1630deb961808b518f231e1e5f455edd9cb1925f1e48ce29c810ee0e75d55`
  - `dist/flake-0.0.1-phase-t-linux-x86_64.sha256` —
    `e1305442e3d54e933ffaecccdc5cba6dad101f6cd7720b7c43b1942c31b646f3`
  - `desktop/src-tauri/target/release/bundle/deb/Flake_0.0.1-phase-t_amd64.deb` —
    `6a2c0e256a10cf6be010ecbea0f25599b6c61ead95a1c0612cdf44d02618a679`
- **Production signing** (`TEST_SIGNING_MODE` unset throughout): each of the three artifacts was
  signed with the real key imported from the `GPG_PRIVATE_KEY` secret into a fresh, per-invocation
  ephemeral `GNUPGHOME`, destroyed immediately after. The imported key's own fingerprint was
  confirmed to match the `GPG_KEY_FINGERPRINT` secret before any signature was produced (the
  script's existing fail-closed pin — GitHub redacts the printed value in the log because it is
  also stored as a secret, but the script's own `exit 1` on mismatch never fired, and the key id
  used for every `gpg --local-user` invocation, `78F7D4B92287FE22`, is exactly the low 16 hex
  digits of the published fingerprint `F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`). Every
  in-script `gpg --verify` immediately after signing printed `gpg: Good signature from "Flake
  Release Signing <285091250+TheHalfMoon@users.noreply.github.com>" [unknown]` with `Primary key
  fingerprint: F779 807C 73F2 9F4D B1E7  DC9F 78F7 D4B9 2287 FE22` for all three artifacts. Every
  job step printed `TEST_SIGNING_IDENTITY_ONLY=NO` — never the disposable-test-identity path.
- **Bytes unchanged:** post-signing `sha256sum` of all three artifacts reproduced the exact
  pre-signing hashes above byte-for-byte (`diff` between the two recorded manifests was empty),
  proving the detached `.asc` signatures never modified the signed files.
- **Independent re-verification**, deliberately in a second, freshly created `GNUPGHOME`
  containing *only* the already-published `docs/release/flake-release-signing-public.asc` (never
  the private key, never the same environment the signing step used): the imported public key's
  fingerprint and UID (`Flake Release Signing
  <285091250+TheHalfMoon@users.noreply.github.com>`) were asserted to match exactly, then
  `gpg --status-fd 1 --verify` was run against every `.asc`/artifact pair and required to emit
  both `[GNUPG:] GOODSIG` and a `[GNUPG:] VALIDSIG` line whose fingerprint field matched the
  published fingerprint exactly, for all three artifacts (`dist/flake-0.0.1-phase-t-linux-x86_64.tar.gz`,
  its `.sha256` manifest, and the `.deb`). The job's final lines: `TEST_SIGNING_IDENTITY_ONLY=NO`,
  `PRODUCTION_SIGNATURE_VERIFIED=YES`, `LINUX_PRODUCTION_FINGERPRINT=F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`.
- **No secret material leaked:** the only values GitHub Actions redacted in the log were the three
  secret env vars themselves and any log line that happened to contain the fingerprint string
  (redacted because it was also stored as the `GPG_KEY_FINGERPRINT` secret, not because it is
  itself sensitive — it is already public in `docs/release/flake-release-signing-public.asc`). No
  private key or passphrase content appears anywhere in the log. Only detached `.asc` signatures,
  checksum manifests, and this verification log were uploaded as the workflow's evidence artifact
  (`t05-04-linux-production-signing-evidence`) — never the private key.

One genuine CI hiccup during this same pass, investigated rather than dismissed: PR #112's own
`m-scale-performance` job (T05-01's unrelated L-scale migration-timing performance gate) failed
once, missing its `gate_maximum_seconds: 1800` ceiling by 66 seconds (`import_seconds: 1866.2`) —
the first failure of that workflow across its last 15 runs, on a PR that touches no migration or
performance code. Rerunning only that job (`gh run rerun --job`) on the same commit passed cleanly
in 20m16s, confirming GitHub-hosted-runner timing variance rather than a real regression; the PR's
other 20+ checks were green on the first pass.

`T05-04_LINUX_SIGNING_STATUS=PASS`. `T05-04` itself remains `IN_PROGRESS`: Windows is still
`PENDING_SIGNPATH_EXTERNAL_APPROVAL` (repository-side work complete; the Founder has not yet
submitted the real application at `signpath.org/apply`) and macOS remains
`BLOCKED_EXTERNAL_APPLE_CREDENTIALS` (genuinely blocked on a paid Apple Developer Program
membership). `T05-05` remains not dependency-ready until both of those close.

## Addendum: zero-Apple-fee macOS direct distribution (PASS)

Per the Founder's `docs/canonical/FOUNDER_ZERO_COST_MACOS_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-20.md`
(2026-09-20), Flake will not purchase Apple Developer Program membership and will not distribute
through the Mac App Store. This supersedes, prospectively and for macOS only, T05-04's prior
"all signatures/notarization/stapling verify" reading for macOS with a zero-cost technical
qualification. It does not change the underlying research — a paid Apple Developer Program
membership remains the only way to obtain Developer ID signing and notarization — only the
Founder's product decision about which release model Flake ships under.

`scripts/release/sign_macos.sh` gained a new `DIRECT_DISTRIBUTION_MODE=1` (distinct from the
existing pipeline-mechanics-only `TEST_SIGNING_MODE=1`, which always deletes its own output) and
a new `workflow_dispatch`-only workflow, `.github/workflows/t05-04-macos-direct-distribution.yml`,
was added and merged via PR #114 (merge commit `a85f906fa579f611c4c05f6464f955805cd8fd32`, every
existing CI gate green — `qualify`/`cli-archive-and-sbom`/`desktop-bundle-install-test` on all
three native platforms, `reproducibility`, `verify-artifacts`, `d6-vm-unclean-shutdown-linux`,
`section27-performance`, `m-scale-performance`, and the existing
`linux-release-signing-test`/`windows-authenticode-test-signing`/`macos-codesign-test-signing`
jobs). It was then run once directly on `main` at that same commit: CI run
[`35514419522`](https://github.com/TheHalfMoon/Flake/actions/runs/35514419522), conclusion
`success`, `macos-direct-distribution` job in 5m32s.

Real, non-fabricated evidence from that run's own log (not merely trusted from the job's green
checkmark):

- **Sanity gate first:** before ad-hoc-signing anything, `sign_macos.sh` was invoked against a
  disposable empty bundle with no `TEST_SIGNING_MODE`/`DIRECT_DISTRIBUTION_MODE` set (default
  production path) and required to fail closed on the missing `SIGNING_IDENTITY`/`APPLE_ID`
  Developer ID variables; the step is written to fail the whole job if that gate unexpectedly
  succeeds, and the job passed, so this gate held.
- **Artifact:** `desktop/src-tauri/target/release/bundle/dmg/Flake_0.0.1-phase-t_aarch64.dmg`,
  extracted, ad-hoc-signed (`DIRECT_DISTRIBUTION_MODE=1`), and rebuilt via `hdiutil create`. Every
  signing step printed `TEST_SIGNING_IDENTITY_ONLY=NO`, `DIRECT_DISTRIBUTION_MODE=YES`,
  `MACOS_APPLE_PLATFORM_TRUST=NOT_CLAIMED`, `MACOS_GATEKEEPER_TRUST=NOT_CLAIMED`,
  `MACOS_NOTARIZATION=NOT_CLAIMED`, `MACOS_DEVELOPER_ID_SIGNATURE=NOT_CLAIMED` — never a
  production Apple claim.
- **`codesign --verify --deep --strict` passed** (`SIGNATURE_MECHANICS_VERIFIED=YES`) — a local
  tamper-evidence check on the ad-hoc-signed bundle's own contents, not a trust-chain claim.
- **`spctl --assess` correctly rejected the artifact**: `SPCTL_EXIT=3` (non-zero) — Gatekeeper
  doing its job against an unnotarized build, confirmed rather than hidden or worked around.
- **Project GPG signature** (the same identity already qualified for Linux,
  `F779807C73F29F4DB1E7DC9F78F7D4B92287FE22`): the `.dmg` and its SHA-256 manifest were each
  signed via `sign_linux.sh` (artifact-agnostic despite its name) using the `GPG_PRIVATE_KEY`
  repository secret in a fresh ephemeral `GNUPGHOME`, then independently re-verified in a second,
  freshly created `GNUPGHOME` seeded only from the already-published
  `docs/release/flake-release-signing-public.asc` — never the private key, never the signing
  step's own environment. Both artifacts printed `GOODSIG, fingerprint confirmed`;
  `MACOS_ARTIFACT_PROJECT_GPG_SIGNATURE_VERIFIED=YES`, `MACOS_APPLE_PLATFORM_TRUST=NOT_CLAIMED`.
  Before touching the real secret, the same production path was confirmed to still fail closed
  with no credentials set (mirroring the Linux sanity gate).
- **GitHub build-provenance attestation**: `actions/attest-build-provenance@v2` attested the
  signed `.dmg`, printing `Attestation created for Flake_0.0.1-phase-t_aarch64.dmg@sha256:310c035d23e6b88f67b6b6ff51e195696970f2886ca9ba43381be420f5141afc`
  — this exact digest is GitHub's own independently-computed SHA-256 of the final signed
  artifact, bound to this repository, commit, and workflow run.
- **Install/launch/uninstall reconfirmed against this specific signed artifact** (not merely the
  unsigned `T05-03` candidate): `install_test.sh` mounted the `.dmg`, copied `Flake.app`, launched
  the installed binary, confirmed the bundled `LICENSE`/`NOTICE`/`THIRD-PARTY-LICENSES.md` were
  present inside the installed bundle, and confirmed the pre-existing test vault was retained and
  unmodified (`RETAINED=YES`).
- **No secret material leaked:** only the three named `GPG_*` secret env vars were ever redacted
  in the log; no private key or passphrase content appears anywhere. Only the signed `.dmg`, its
  detached signatures, checksum manifests, and the install-test log were uploaded as the
  workflow's evidence artifact (`t05-04-macos-direct-distribution-evidence`) — never key material.

`T05-04_MACOS_SIGNING_STATUS=PASS` under the amended, zero-cost acceptance clause —
`MACOS_ZERO_COST_TECHNICAL_QUALIFICATION=PASS`, `MACOS_DEVELOPER_ID_REQUIRED=NO`,
`MACOS_NOTARIZATION_REQUIRED=NO`, `MACOS_GATEKEEPER_TRUST=NOT_CLAIMED` throughout — never a claim
of Apple platform trust, Developer ID, or notarization. `T05-04` itself remains `IN_PROGRESS`:
Windows is still `PENDING_SIGNPATH_EXTERNAL_APPROVAL` — repository-side work is complete
(`docs/release/SIGNPATH_ELIGIBILITY_PACKET.md`, `docs/release/CODE_SIGNING_POLICY.md`); the one
remaining action is the Founder submitting the real SignPath Foundation application at
`signpath.org/apply` (real applicant identity, MFA-enrolled account) and SignPath's own
review/approval, which cannot be fabricated or completed by repository automation. `T05-05`
remains not dependency-ready until that closes.

## Addendum: `UNSIGNED_DEVELOPER_RC` Windows prerelease published to satisfy SignPath's "already released" criterion (2026-09-20)

**Founder authorization, explicit and scoped:** "I explicitly authorize publishing ONE public
GitHub prerelease for Flake solely to satisfy SignPath Foundation's current 'already released in
the form that should be signed' eligibility requirement... This is NOT authorization to publish
a final production release." Plan section 25's own reservation ("uploading, publishing a
release" requires separate Founder authorization beyond building verified candidates,
`AGENTS.md` §8) is satisfied by this instruction; no broader publication was performed.

**Artifact selection — no rebuild.** Re-read this report's own T05-03 evidence above rather than
building anything new. The `t05-03-release-candidates` CI run from PR #116 (`35518382422`) was
identified as the correct source: it re-ran the full qualification battery (24 checks, all
green) on a commit whose git tree is byte-identical to `main`'s current HEAD
(`27824604e09fc3ef9ba682f54e99139bdb9d8ab3` and `8bde2ae266a02faff52c810d8a442b79fb3ea814` share
tree SHA `66bedc4f60492adacf6320cb67d3247edf06af5a`, confirmed via `gh api .../git/commits` before
selecting this run). Its `cli-archive-and-sbom-windows-latest` and `desktop-bundle-windows-latest`
artifacts (not yet expired) were downloaded, not rebuilt.

**Published:** GitHub prerelease
[`v0.0.1-phase-t-rc.1`](https://github.com/TheHalfMoon/Flake/releases/tag/v0.0.1-phase-t-rc.1),
tagged at `27824604e09fc3ef9ba682f54e99139bdb9d8ab3`, confirmed via the GitHub API to carry
`prerelease: true`, `draft: false`. Tag collision checked (empty) immediately before publishing.
Artifacts uploaded: `Flake_0.0.1-phase-t_x64-setup.exe` (unsigned NSIS installer),
`flake-0.0.1-phase-t-windows-x86_64.zip` (CLI archive: `flake.exe`/`fehrest.exe`/
`flake-migrate.exe`/`README.md`/`LICENSE`/`NOTICE`/`THIRD-PARTY-LICENSES.md`), the build's own
`.sha256` manifest for the CLI archive, and four CycloneDX SBOMs
(`flake-cli`/`fehrest-cli`/`flake-migrate`/`flake-desktop`).

**Independent verification, before and after publishing:**
- CLI archive SHA-256 (`1b8a66d789bb461a6d35333859d9c0b3a3def1c57f0b0ffd79441f5b978c8f3e`) matched
  the build's own `.sha256` manifest before upload.
- The NSIS installer has no build-generated manifest; this session computed its SHA-256
  independently (`83c077a3b4c39bf0751a193c0c56e554597339cc6f868918f29a61238d8a0b44`) and published
  it in a companion `.sha256` file, recorded as independently computed rather than
  build-generated.
- After publishing, both files were freshly re-downloaded from their public
  `browser_download_url`s (not reused from the local staging copy) and re-hashed — both matched
  exactly, confirming upload integrity.

**What this does not claim:** the release notes explicitly state Windows binaries are unsigned,
SmartScreen warnings are expected, no SignPath endorsement or approval exists, and this is not a
final production release. GitHub artifact attestation was deliberately not generated for these
files — `release-provenance-attestation.yml` would rebuild the CLI archive independently and
produce artifacts that would not byte-match what was actually published, so triggering it would
have misrepresented provenance rather than proven it; this gap is stated plainly in the release
notes instead. macOS and Linux artifacts (already independently qualified above) were excluded —
this publication is scoped to exactly what the Windows SignPath application needs, matching the
Founder's authorization.

`T05-04_WINDOWS_SIGNING_STATUS` remains `PENDING_SIGNPATH_EXTERNAL_APPROVAL` — this addendum
resolves the "already released" prerequisite the packet had flagged as an open question; it does
not and cannot substitute for SignPath's own review and approval. `T05-05` remains not
dependency-ready.

## Addendum: website-first direct distribution amendment ratified, Windows direct-web qualification implemented (2026-09-26, pending CI run)

**Founder governance:** `docs/canonical/FOUNDER_WEBSITE_FIRST_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-26.md`
(`FOUNDER_DECISION=RATIFIED`, effective 2026-09-26, Class E) supersedes the prior
external-signing-only Windows exit condition prospectively. Microsoft Store, Mac App Store, any
mandatory app store, paid Windows signing for the current release, paid Apple Developer Program
membership, and paid cloud signing are out of the v1 contract
(`WEBSITE_FIRST_DIRECT_DISTRIBUTION=YES`, `FOUNDER_ZERO_COST_DISTRIBUTION_REQUIRED=YES`).
The new Windows contract is `WINDOWS_DISTRIBUTION_MODE=DIRECT_WEB` with
`WINDOWS_AUTHENTICODE_TRUST=NOT_AVAILABLE` / `WINDOWS_AUTHENTICODE_TRUST_CLAIMED=NO` /
`WINDOWS_SMARTSCREEN_WARNING=EXPECTED_AND_DISCLOSED`, and `WINDOWS_AUTHENTICODE_STATUS=PASS`
is never set without a real verified trusted signature.

**External evidence recorded truthfully:** SignPath Foundation / Phillip Deng responded
2026-09-25 with `APPLICATION NOT APPROVED AT THIS TIME` for insufficient public trust and
visibility signals (stars, forks, contributors, articles, references, Reddit, Stack Overflow,
YouTube, institutional backing, sustained activity), invited reapplication after broader
recognition, and offered a paid subscription the Founder declined. Recorded as
`T05-04_SIGNPATH_APPLICATION_STATUS=REJECTED_INSUFFICIENT_PUBLIC_VISIBILITY`,
`T05-04_SIGNPATH_FOUNDATION_APPROVAL=NO`, `T05-04_SIGNPATH_REAPPLY_AFTER_ADOPTION=YES`,
`T05-04_SIGNPATH_PAID_ROUTE=DECLINED_BY_FOUNDER`,
`T05-04_SIGNPATH_EXTERNAL_BLOCKER=SUPERSEDED_BY_FOUNDER_DIRECT_DISTRIBUTION_DECISION`.
No rejection/project/org ID, policy slug, certificate, token, scoring, or star count is
claimed. OSSign is `OSSIGN_STATUS=DEFERRED_FUTURE_OPTION`, `OSSIGN_V1_BLOCKER=NO`,
`OSSIGN_REAPPLY_WHEN_ELIGIBLE=YES`. History (submitted, then rejected, then amended) is
preserved, not rewritten.

**Implemented in this change (no PASS claimed):**

- `docs/release/WINDOWS_DIRECT_DISTRIBUTION.md` (clauses A-H: build, install, data
  continuity, manual update model, integrity, Authenticode truth, SmartScreen UX, clean
  download reverification).
- `scripts/release/inspect_windows_signature.sh` (real `signtool verify /pa /v` plus
  `Get-AuthenticodeSignature`; reports `NOT_SIGNED`/`SIGNED_TRUSTED`/`UNTRUSTED` truthfully,
  never a trust PASS; `bash -n` clean).
- `.github/workflows/t05-04-windows-direct-distribution.yml` (`workflow_dispatch`-only:
  build, integrity metadata, signature inspection expecting `NOT_SIGNED`, GPG-sign
  installer/archive/manifest with the existing project key plus clean-keyring re-verify,
  GitHub provenance attestation, `install_test.sh` reconfirmation, byte-identity proof;
  YAML parsed clean).
- `docs/release/DOWNLOAD.md` (canonical v1 download landing surface; historical
  `v0.0.1-phase-t-rc.1` preserved with exact SHA-256 identities).
- Updated `docs/release/CODE_SIGNING_POLICY.md`,
  `docs/release/SIGNPATH_ELIGIBILITY_PACKET.md` (rejection addendum),
  `docs/release/RELEASE_VERIFICATION.md` (Windows direct-web truth),
  `docs/release/USER_GUIDE.md` (sections 7/9: manual update model, SmartScreen safe path,
  no global-disable instructions), `README.md` (download table plus DOWNLOAD pointer), and
  `specs/CURRENT.md` (amendment pointer, Windows direct-web `PENDING` fields, SignPath
  rejection states, `EXECUTABLE_REPOSITORY_WORK=NONZERO` for the pending qualification run).

**Not done here:** the new workflow has not yet run green in CI, so
`T05-04_WINDOWS_DIRECT_DISTRIBUTION_STATUS=PENDING_DIRECT_DISTRIBUTION_QUALIFICATION_CI_RUN`
and `T05-04` remains `IN_PROGRESS`. `T05-05` remains not dependency-ready until that run is
independently checked and recorded in a follow-up. No `v1.0.0` tag invented; historical
`v0.0.1-phase-t-rc.1` untouched.
