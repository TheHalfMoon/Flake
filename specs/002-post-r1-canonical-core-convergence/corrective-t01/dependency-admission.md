# Dependency admission record — T00-02

Scope per canonical plan §34 (T00-02): "Pin reference hardware/dataset definitions and proposed dependency versions with license, advisory and build-script admission records; perform no installs or product edits in this unit." No `cargo install`, `cargo add`, `cargo update`, or edit to `Cargo.toml`/`Cargo.lock` was performed to produce this record — every version below is what is already locked in the working tree (re-confirmed against `origin/main` `6389f651...`; see docs/evidence/flake-v1/T00-02/REPORT.md provenance note).

## Proposed dependency for T01-02: none new

`src/derived.rs` already depends on `rusqlite` (feature `bundled`, which vendors and compiles SQLite from C source via `libsqlite3-sys`). `T01-02` extends usage of this already-admitted crate to a second database (`canonical.sqlite`); it does not add a new entry to `Cargo.toml`. Confirmed by `grep -rln rusqlite src/` → only `src/derived.rs` today; `Cargo.lock` already contains both crates at the versions below.

| Crate | Locked version | Source |
|---|---|---|
| `rusqlite` | `0.37.0` | `Cargo.lock`, checksum `165ca6e57b20e1351573e3729b958bc62f0e48025386970b6e4d29e7a7e71f3f` |
| `libsqlite3-sys` | `0.35.0` | `Cargo.lock`, checksum `133c182a6a2c87864fe97778797e46c7e999672690dc9fa3ee8e241aa4a9c13f` |

## License

Checked directly against each crate's own `Cargo.toml` at its published tag (`raw.githubusercontent.com/rusqlite/rusqlite/v0.37.0/{Cargo.toml,libsqlite3-sys/Cargo.toml}`), not inferred from memory:

| Crate | `license` field |
|---|---|
| `rusqlite` 0.37.0 | `MIT` |
| `libsqlite3-sys` 0.35.0 | `MIT` |

Both are compatible with the founder-selected Apache-2.0 project intent (permissive, no copyleft, notice-only obligation). SQLite itself, vendored via the `bundled` feature, is placed in the public domain by its authors (sqlite.org's own declaration; not independently re-verified in this session, but this is long-standing, widely-relied-upon public information, not a novel claim). No `LICENSE`/`NOTICE` file yet exists at the repository root (noted already in the canonical plan §2 live-truth snapshot: "root LICENSE absent"); creating one is out of scope for T00-02 and belongs to whichever later task the plan assigns it (S09 row references `T00-02, T01-02, T04-01, T05-03..05` for supply-chain/distribution — the actual LICENSE file publication is a distribution-phase (P05) concern per §25, not a T01-xx blocker).

## Advisory check

Checked against the public RustSec advisory database (`rustsec.org`), not via a local `cargo audit` run — running `cargo audit` would require installing a new tool, which this task's forbidden scope excludes ("no installs ... in this unit"). This is recorded as a limitation, not silently omitted.

| Crate | Advisory | Patched versions | Locked version | Affected? |
|---|---|---|---|---|
| `rusqlite` | RUSTSEC-2021-0128 (incorrect lifetime bounds on closures) | `>=0.26.2, ^0.25.4` | `0.37.0` | No — well above patched range |
| `rusqlite` | RUSTSEC-2020-0014 (various memory-safety issues) | `>=0.23.0` | `0.37.0` | No |
| `libsqlite3-sys` | RUSTSEC-2022-0090 (CVE-2022-35737, upstream SQLite) | `>=0.25.1` | `0.35.0` | No |

No open/unpatched advisory applies to the locked versions. This check covers only the two crates T01-02 will additionally rely on; it is not a full-tree `cargo audit` and does not certify every transitive dependency in `Cargo.lock`. A full-tree audit, if required, belongs to the S09 supply-chain gate at `T04-01`/`T05-03..05` per canonical plan §22 row S09.

## Build-script note

`libsqlite3-sys`'s `bundled` feature (already enabled in `Cargo.toml`) compiles the vendored SQLite C amalgamation via a build script using the `cc` crate (already present in `Cargo.lock`). This build script is **pre-existing and already runs on every build today** — T01-02 does not add a new build script or change this feature flag. `clarify.md` records as a genuinely open (non-blocking) question whether a prior explicit S09 admission record for this build script exists elsewhere in the repository's history; none was located during this session's search, and this is named as a limitation rather than assumed either way.

## Reference dataset definitions

Deferred in full to `plan.md`/`clarify.md`: dataset S/M/L generators (canonical plan §27) require typed-record shapes that do not exist before P02. T01-01..06 use small, task-specific disposable fixtures (described per-task in `checklist.md`); no dataset-M/L generator is pinned by this record. This is explicitly non-blocking per `spec.md` NFR-PERF-1.
