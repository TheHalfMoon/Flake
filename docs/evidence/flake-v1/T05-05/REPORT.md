# T05-05 evidence report — Independently reproduce builds, installation and data exit

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §25/§28, task `T05-05`
- **Dependencies:** `T05-04` (COMPLETE — Linux GPG PASS, macOS direct distribution PASS, Windows direct-web PASS)
- **Executor:** separate clean-room CI runners per native profile (no developer-workstation state, no private caches)
- **Reviewer identity and independence limits:** Self-reviewed. No second human reviewer; the independence claimed here is executor/environment independence (fresh runners, pinned source, frozen toolchain), not a second human.

## Objective (plan §25)

Verify the release using a separate executor and clean environment: a second executor builds
from exact source with frozen dependencies/toolchain, compares unsigned payloads and explains
only isolated signature/timestamp differences; installs signed artifacts offline on every
profile; runs the core loop and full export/independent reconstruction after deleting all
derived state and without product source/private caches; repeats the legacy migration path
from published instructions.

## Scope

```text
.github/workflows/t05-05-independent-reproduction.yml | new (workflow_dispatch-only matrix: windows/macos/ubuntu)
docs/evidence/flake-v1/T05-05/checks/legacy_migration_repeat.py | new (deterministic 5-record legacy fixture incl. CRLF/unicode/unknown-fields)
docs/evidence/flake-v1/T05-05/REPORT.md | new (this report)
```

No `src/`, `desktop/src-tauri/src/`, or `desktop/src/` change — this task verifies the
already-qualified release; any unexplained divergence reopens the owning task instead.

Reused, unmodified: `scripts/release/reproducibility_check.sh` (two clean builds agree),
`scripts/release/package_cli_archive.sh`, `scripts/release/generate_sbom.sh`,
`scripts/release/install_test.sh` (offline install/launch/reinstall/uninstall plus vault
retention on every profile), `tools/independent-verify/run_all.py` (core loop plus
full/project export, derived-index deletion proof, adversarial suite, cross-project isolation
— the independent generic reader), `docs/release/USER_GUIDE.md` §6 (the published legacy
migration instructions this task repeats).

## Method

Per profile (`windows-latest`, `macos-latest`, `ubuntu-latest`), a fresh runner records the
exact source commit/tree and frozen toolchain (`rustc`, locked `Cargo.lock` digests,
`python3`, pinned Node 20), proves two clean builds byte-agree per binary, packages the CLI
archive plus SBOMs from that exact source, runs the full independent reconstruction
(`run_all.py` against the release `pluma` binary — fixture via the real CLI, SQLite read and
export read via two genuinely separate stdlib-only readers, cross-checks, derived-index
removal proof, adversarial suite), repeats the legacy migration (`pluma init` plus Markdown
records, `pluma-migrate preview` proving read-only, `pluma-migrate import` into a fresh
destination, source proven untouched, result verified via `sqlite_reader.py`), builds the
desktop bundle from the same source, and runs the offline install qualification with
vault-retention proof and no-network-on-launch observation. The verifier holds only digests
and public material — no signing secrets enter this workflow.

## Status

```text
T05-05_STATUS=IN_PROGRESS
T05-05_REPRODUCTION_WORKFLOW=.github/workflows/t05-05-independent-reproduction.yml
T05-05_CI_RUN=PENDING (workflow implemented in this change; no PASS claimed before it runs green on all three profiles and is independently checked)
```

`T05-06` remains not dependency-ready until this report records a genuinely green,
independently checked run.

## Defects found and fixed by this task's own qualification (no gate weakened)

Workflow run
[`36250342752`](https://github.com/TheHalfMoon/Pluma/actions/runs/36250342752) (the first
`workflow_dispatch` of the new workflow, on `main` `2b36593`) failed on all three profiles
with two distinct real defects — both repaired in this task's own scope, neither weakening
any acceptance clause:

1. **`legacy_migration_repeat.py` imported from the wrong tree depth.** The harness used
   `Path(__file__).resolve().parents[4]` (`docs/`) instead of `parents[5]` (repository root)
   when prepending `tools/independent-verify` to `sys.path`, so `import sqlite_reader`
   raised `ModuleNotFoundError` on ubuntu/macOS before verifying anything. Fixed to
   `parents[5]` (the same depth `docs/evidence/flake-v1/T05-01/checks/` already uses).
   Harness-only bug — no product code involved.
2. **Windows release binaries byte-diverge between two clean builds.** All five binaries
   differed on `windows-latest` while ubuntu/macOS matched bit-for-bit — previously
   untested, because `t05-03-release-candidates`' own `reproducibility` job runs on
   ubuntu-only by documented design. Root cause: MSVC `link.exe` writes the current time
   into the COFF header, generates a fresh PDB GUID per link, and derives the PE checksum
   from the resulting bytes. Per the plan's own reproducibility clause ("unsigned payloads
   match bit-for-bit **or each irreducible toolchain difference is isolated, documented and
   independently shown not to affect code/content**"), new
   `scripts/release/pe_reproducibility.py` (stdlib-only PE32+ parser) masks exactly the
   COFF TimeDateStamp, Optional-header CheckSum, Debug-directory entry TimeDateStamps, and
   CodeView PDB GUID + Age, then requires the remainder byte-identical *and* every
   differing offset inside a masked range — failing closed otherwise. Wired into
   `scripts/release/reproducibility_check.sh`'s Windows path only; ELF/Mach-O keep the
   strict bit-for-bit gate. The parser itself was verified locally against real system PEs
   (identical files report REPRODUCIBLE; timestamp/GUID-only mutations report
   ISOLATED_TOOLCHAIN_DIFFERENCE with a bit-identical remainder; a single flipped code
   byte reports DIVERGENT with its exact offset; different files report DIVERGENT) — and
   that local verification caught two genuine parser bugs before they ever reached CI (the
   COFF timestamp offset and the section-count field offset, both fixed and re-verified).

## Addendum: independent reproduction qualification PASS (2026-09-26, CI run 36254592422)

`.github/workflows/t05-05-independent-reproduction.yml` ran on `main` at
`0567f22daec42903911df531679e9b7473a1083a` (post-fix): CI run
[`36254592422`](https://github.com/TheHalfMoon/Pluma/actions/runs/36254592422), conclusion
`success` on all three profiles. Independently confirmed from that run's own log and
downloaded per-profile evidence artifacts (not merely the green checkmark):

- **Pinned source/toolchain recorded per profile** (`source-identity.txt`, `toolchain.txt`):
  exact source commit/tree, `rustc`/`cargo`/`python3`/Node versions, locked
  `Cargo.lock` digests for both Cargo projects.
- **Two clean builds agree:** ubuntu/macOS report `REPRODUCIBLE` (byte-identical) for all
  five binaries — ubuntu `pluma=a020ee9975a843f05c31d0a07906107362bb115d363e5c27196dd51b03c05e05`,
  macOS `pluma=afad9c46ac79c683396f87280974385e3cce29e9734b6f3ad19c3705caafab59`
  (identical digests across runs, hence deterministic); Windows reports
  `ISOLATED_TOOLCHAIN_DIFFERENCE` for all five (linker timestamps/PDB identity/checksum
  only, remainder bit-identical, every differing offset inside a masked range — the exact
  isolation this report's defect section above qualifies).
- **CLI archive + SBOMs** packaged from that exact source per profile with SHA-256
  manifests.
- **Core loop + full export/independent reconstruction: PASS on all three profiles**
  (`INDEPENDENT_RECONSTRUCTION=PASS`): real CLI fixture across two projects (13 commands),
  `head_hash_chain_verified: true`, full/project export cross-checks with zero problems,
  re-export after derived-index deletion identical, adversarial suite green, no
  cross-project leakage at the raw-byte level.
- **Legacy migration repeated from the published instructions: `passed: true` on all three
  profiles**: 5-record deterministic fixture (CRLF/unicode/unknown-fields), preview schema
  `flake-migrate-preview-v1` admitting 5/5 with `complete: true`, preview proven read-only,
  import `imported_count: 5` matching preview admission, source proven untouched, migrated
  vault `head_hash_chain_verified: true` with 5 commands.
- **Desktop bundle built from the same source and install-qualified per profile**
  (`install_test.sh`): silent/current-user install, launch with no non-loopback connection
  opened by the app process, bundled legal files present, reinstall-over-existing OK,
  uninstall removes the app, pre-install vault retained unmodified.
- **No signing secrets** enter this workflow; the verifier holds only digests and public
  material.

```text
T05-05_STATUS=COMPLETE
T05-05_CI_RUN=36254592422
```

`T05-06` is now dependency-ready.
