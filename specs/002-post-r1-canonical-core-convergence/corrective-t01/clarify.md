# Clarify — Spec 002 corrective addendum (T01-01..07)

Per `docs/19-ENGINEERING-METHOD.md` §3: "Unresolved ambiguity is recorded, not guessed." This is a documentation-only T00-02 output; no product code was read beyond public-function signatures (`grep`) needed to size the mutator inventory and confirm the dependency-reuse claim in `spec.md` FR1-004/NFR-DEP-1.

## Questions resolved by the canonical plan (not reopened here)

| Question | Resolution | Source |
|---|---|---|
| Which storage engine for the new canonical store? | Embedded SQLite, rollback journal, `DELETE`/`EXTRA` synchronous, no WAL | Canonical plan §13 "Implement directly"; §6 final challenge summary rejects a custom multi-file transaction protocol |
| Does this require a new dependency? | No — `rusqlite`/`libsqlite3-sys` are already locked in `Cargo.lock` and used by `src/derived.rs` for the FTS5 derived index. T01-02 is the first task to also use them for the *canonical* store. | `Cargo.lock` inspection, `grep -rl rusqlite src/` → only `src/derived.rs` (see `dependency-admission.md`) |
| Cache size / page size for the new store? | Deferred measurement decision within a bounded option set (8/16/32 MiB cache; 4096/8192 page size), not chosen here. Tie-break is 4096 bytes / 8 MiB per canonical plan §13. | Canonical plan §13 "Measure within bounds" |
| Is WAL mode permitted? | No — explicitly requires a new ADR per canonical plan §13 "New ADR ... required before work: another canonical engine/layout, WAL...". Not authorized by this addendum. | Canonical plan §13 |
| Does T01-02 replace the derived/FTS index engine choice? | No — `derived.sqlite` (FTS5) is untouched; this addendum only adds a *second*, canonical use of the same already-admitted engine, in a separate database file (`canonical.sqlite`), per the component diagram in canonical plan §14. | Canonical plan §14 |
| Is Spec 003 activated by any of T01-01..07? | No. Explicitly forbidden by the canonical plan (§1, §30) and the handoff document. | Canonical plan, handoff, `specs/CURRENT.md` |
| Does this addendum authorize touching sealed R1 evidence or the historical Spec 002 files? | No. `docs/canonical/R1_V3_TERMINAL_VERDICT_2026-09-09.md` and the parent directory's `spec.md`/`plan.md`/`checklist.md`/`tasks.md`/`analyze.md`/`ponytail-gate.md`/`verification.md`/`dependencies.md` are historical and untouched. | Canonical plan §4; `FLAKE_CORRECTIVE_PLANNING_CONTRACT.md` |
| What counts as a "canonical mutator" for the T01-01 inventory? | Any function reachable from `src/vault.rs`, `src/events.rs`, `src/locator.rs`, or `src/cli.rs` that can change bytes on disk under the vault's control directory, including metadata-ensure and lock-file creation, not only the obviously named `add_object`/`append` functions. | `mutator-inventory.md`, built from a full `pub fn`/`pub(crate) fn` grep across the four listed files |

## Questions genuinely open — recorded, not guessed

| Question | Why it's open | Who resolves it | Blocking? |
|---|---|---|---|
| Exact dataset-M/L generator, seed, and digest for the section 27 performance gate | No generator exists yet; typed-record shapes needed to build dataset M don't exist until P02 | T01-07 records a *bounded, no-regression* observation only; the formal generator is a P02+ deliverable per canonical plan §27 | No — NFR-PERF-1 already scopes this out of T01-01..07's completion condition |
| Whether `libsqlite3-sys`'s `bundled` build script (compiles SQLite from C source via the `cc` crate) needs a separate S09 build-script admission record beyond what's recorded here | The build script already runs today (feature is already enabled in `Cargo.toml`); T01-02 does not add a new build script, but T00-02 has not located a prior explicit admission record for it in `dependencies.md` | T00-02 records what exists today (`dependency-admission.md`); if a prior explicit admission record turns out to be genuinely required and missing, that is named as a limitation, not fabricated | No — the build script is pre-existing and unchanged; not a new admission surface introduced by this addendum |
| Exact reference hardware for later native profiles (Linux/ext4, macOS/APFS) required by `D6`/T05-02 | Those hosts are not available in the current environment | `T05-02`, a much later task; explicitly deferred by the plan itself | No — out of T01-01..07 scope entirely |

No question above blocks activation of `T01-01`. The plan's own text (§13, §31) already resolves every architecture/product question T01-01..07 depends on; nothing here required escalation back to Astro.
