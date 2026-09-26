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
