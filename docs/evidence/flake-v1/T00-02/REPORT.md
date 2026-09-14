# T00-02 evidence report — Publish the Spec 002 corrective addendum and admission record

## Identification

- **Task ID / phase / slice:** T00-02 / P00 / VS00
- **Plan version tested against:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md`, SHA-256 `b555f83ff12882ae6f55f90bbeaa411de52b84661a7f3953350cbfc6bd789fb2`, as merged to `main` by PR #64.
- **Baseline / tested source commit:** forked from `origin/main` at `6389f6512ea0feb90fd2da7dcdbde6f442e7eb29` (PR #65, `T00-01` evidence, merged after this task's predecessor closed).
- **Predecessor:** `T00-01` — COMPLETE, evidence at `docs/evidence/flake-v1/T00-01/REPORT.md`, merged via PR #65.
- **Executor:** Muse (Claude Code implementation agent).
- **Reviewer identity and independence limits:** Self-reviewed only; no second independent human reviewer. Web-sourced license/advisory findings were fetched and summarized by an automated tool (WebFetch), not independently cross-verified by a second source. Repository-configured hosted review (CodeRabbit) runs in comment-only/manual mode for this OSS repository and does not block merge.

## Provenance note

The substantive analysis in this task (source inventory, dependency admission, reference hardware, and the ten `corrective-t01/` Spec Kit documents) was originally produced the same day on local branch `codex/flake-product-review` at commit `6c234c2`, against source commit `852e44b`'s tree. `docs/evidence/flake-v1/T00-01/REPORT.md` establishes that `852e44b`'s source tree is identical to `origin/main`'s (PR #64 changed no Rust source), and this task independently re-confirmed the two load-bearing facts below directly against the current `origin/main` checkout before reusing that content:

```text
$ grep -rln "rusqlite" src/
src/derived.rs
$ grep -n "pub fn\|pub(crate) fn" src/vault.rs | head -6
35:    pub fn new(
63:pub fn is_supported(path: &Path) -> bool {
111:    pub fn create(root: impl AsRef<Path>) -> Result<Self> {
123:    pub fn open_write(root: impl AsRef<Path>) -> Result<Self> {
140:    pub fn open_read(root: impl AsRef<Path>) -> Result<Self> {
161:    pub fn vault_meta(&self) -> Result<VaultMeta> {
$ sed -n '190,200p;250,262p' Cargo.lock   # unchanged: libsqlite3-sys 0.35.0, rusqlite 0.37.0
```

Both match the original analysis exactly (line numbers, function set, locked versions). The content below is therefore reused verbatim from the original analysis rather than re-derived from scratch, with only source-commit citations corrected to the real `origin/main` lineage; this report records that reuse rather than silently re-dating the original work.

## Baseline and tested source

| Item | Value |
|---|---|
| Repository root | `C:/Users/Shehr/OneDrive/Documents/ChatGPT/Flake` |
| Branch | `flake/t00-02-spec002-corrective-addendum` |
| Forked from | `origin/main` `6389f6512ea0feb90fd2da7dcdbde6f442e7eb29` |
| Dirty diff digest | Not applicable — worktree was clean before this task's new files were added |
| Dependency (T00-01) | COMPLETE, evidence reviewed |

## Environment and tool versions

Same as `docs/evidence/flake-v1/T00-01/REPORT.md`; additionally used `WebFetch` (network tool, in the original same-day session) for the license/advisory admission check. See `raw/05-reference-hardware-capture.txt` for the exact toolchain/hardware capture.

## Work performed

1. Read the `T01-01`..`T01-07` task contracts from the canonical plan (§34) not yet read during `T00-01`.
2. Read `docs/19-ENGINEERING-METHOD.md` for the exact Spec Kit lifecycle vocabulary and the Ponytail necessity gate, to keep the new documents' structure and terminology consistent with the repository's own convention rather than inventing a new one.
3. Read the historical `specs/002-post-r1-canonical-core-convergence/{spec,plan,checklist,tasks,analyze,ponytail-gate,dependencies}.md` (heads only) to confirm this addendum's file-naming and section conventions match precedent, and to confirm none of those historical files needed to change.
4. Enumerated every `pub fn`/`pub(crate) fn` in `src/vault.rs`, `src/events.rs`, `src/locator.rs`, `src/cli.rs` (`raw/02-pub-fn-inventory.txt`) and read the body of every function whose read/write classification was not obvious from its name (`raw/03-mutator-source-excerpts.txt`), producing `mutator-inventory.md`. This independently confirmed, against actual source, all three defect clauses in the canonical plan's stated T01-01 rationale (metadata creation on read, pre-lock startup mutation, unbound event append).
5. Confirmed via `grep -rln rusqlite src/` that `rusqlite`/`libsqlite3-sys` are used today only by `src/derived.rs`, and read their exact locked versions from `Cargo.lock` (`raw/01-rusqlite-usage-and-lock.txt`), establishing that `T01-02` requires no new dependency.
6. Checked license and RustSec advisory status for both crates at their exact locked versions via `WebFetch` against `crates.io`, `rustsec.org`, and the crates' own published `Cargo.toml` files (`raw/04-license-advisory-webfetch-results.txt`), producing `dependency-admission.md`.
7. Captured exact reference-measurement hardware (CPU, RAM, storage, OS, toolchain) for the development profile (`raw/05-reference-hardware-capture.txt`), producing `reference-hardware.md`, per canonical plan §27's requirement that T00-02 freeze this before any later benchmark task runs.
8. Wrote `spec.md`, `clarify.md`, `plan.md`, `checklist.md`, `tasks.md`, `ponytail-gate.md` for `T01-01`..`T01-07`, all under a new `specs/002-post-r1-canonical-core-convergence/corrective-t01/` subdirectory that does not modify any historical file in the parent directory.
9. Wrote `analyze.md`, cross-checking every new document above against the canonical plan's I01-I12 invariants, security model, verification hierarchy, and the historical Spec 002 record, to catch drift before closing this task.
10. This task's own re-verification: re-ran the `rusqlite` usage grep and `pub fn` inventory head directly against the current `origin/main` checkout (provenance note above) to confirm no source drift since the original same-day analysis.

## Commands executed (exit status implicit — all completed without error; raw output in the named files)

| # | Command | Raw artifact |
|---|---|---|
| 1 | `grep -rln "rusqlite" src/`; `grep -A3 '^name = "rusqlite"' Cargo.lock`; same for `libsqlite3-sys` | `raw/01-rusqlite-usage-and-lock.txt` |
| 2 | `grep -n "pub fn\|pub(crate) fn"` across the four T01-01 target files | `raw/02-pub-fn-inventory.txt` |
| 3 | `sed -n` excerpts of `open_read`, `open_write`, `ensure_vault_meta`, `startup_integrity_check`, `EventLog::open`, `append_for_writer`/`append` | `raw/03-mutator-source-excerpts.txt` |
| 4 | `WebFetch` x 8 against crates.io API, rustsec.org package/advisory pages, and raw GitHub `Cargo.toml` files | `raw/04-license-advisory-webfetch-results.txt` |
| 5 | `Get-CimInstance Win32_Processor`/`Win32_LogicalDisk`, `Get-PhysicalDisk`, `systeminfo`, `rustc/cargo/git --version` | `raw/05-reference-hardware-capture.txt` |

## Results

- **Mutator inventory:** 6 mutation-capable paths confirmed reachable without proven writer ownership; 3 correctly writer-bound mutators identified as the pattern to extend; 1 runtime-checked (not type-checked) legacy mutator flagged for narrowing. Full table in `mutator-inventory.md`.
- **Dependency admission:** No new dependency required for `T01-02`. `rusqlite` 0.37.0 / `libsqlite3-sys` 0.35.0, both MIT-licensed, neither affected by any located RustSec advisory (all three checked advisories patched at versions below what is locked).
- **Reference hardware:** Development profile exceeds the canonical plan's stated minimum (8 physical cores vs. 4 required, ~15.7 GiB RAM vs. 8 GiB required, NVMe SSD). One real constraint noted: limited free space on the system drive, insufficient for a future 10 GiB dataset-L fixture without freeing space first — recorded, not hidden.
- **Spec Kit package:** `spec.md`, `clarify.md`, `plan.md`, `checklist.md`, `tasks.md`, `ponytail-gate.md`, `analyze.md`, plus supporting `mutator-inventory.md`, `dependency-admission.md`, `reference-hardware.md` — 10 files, all under `specs/002-post-r1-canonical-core-convergence/corrective-t01/`.

## Failed attempts / exclusions

None. Every planned check succeeded. No `cargo audit` was run (would require a tool install, forbidden in this task's scope); the public RustSec website was used instead and this substitution is recorded as a limitation, not concealed.

## Limitations

- License/advisory findings rest on `WebFetch`'s tool-mediated summarization of public pages, not a raw diff of the license text or a `cargo audit`/`cargo license` run. This is adequate for a documentation-stage admission record but should be re-verified with `cargo audit`/`cargo deny` once those tools are legitimately introduced under a later task (S09, `T04-01`/`T05-03..05`).
- No product code was changed by this task; the mutator inventory reflects source as of `origin/main` at the commit named above and must be re-audited against the code as actually shipped once `T01-01` is implemented (this is explicitly required by `T01-07`'s own checklist item, not skipped).
- The historical-file naming precedent check (item 3 above) was a head-only read of seven files, not a full read of every historical Spec 002 document; sufficient to confirm naming/section conventions, not a full audit of historical content.
- No installs, builds, or test runs occurred in this task, consistent with its forbidden scope; no runtime evidence of any kind is claimed.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Every P01 task is specified with negative cases | Satisfied — `checklist.md` lists failure/negative fixtures per task (fault injection, corruption, concurrent-writer, cancellation, duplicate-identity, etc.), matching each task's canonical-plan "Tests" field |
| All source findings have an owner | Satisfied — `mutator-inventory.md`'s "fix" rows are each assigned to a specific T01-0x task |
| Exact dependency/platform qualification work and evidence paths are recorded | Satisfied — `dependency-admission.md`, `reference-hardware.md`, and this report's evidence paths |

## Completion condition

All acceptance clauses above, the shared contract (SC), and the P00 phase exit condition ("Authority, historical preservation and bounded Spec 002 addendum ready; no product mutation yet") pass. No product/source/schema/migration/workflow/CI/package file was edited in this task. `T00-02` is complete.

## Next frontier

`T01-01` — Make legacy inspection nonmutating and establish ownership. This is the first task in this addendum that touches product source (`src/vault.rs`, `src/events.rs`, `src/locator.rs`, `src/cli.rs`) and therefore the first task with real Rust implementation, test, and native-verification requirements.
