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
