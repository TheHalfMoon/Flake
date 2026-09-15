# T02-05 evidence report — Export complete owned state with a verified manifest

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §12/§14/§15/§20/§22/§27-29, task `T02-05` — fifth task of `P02`
- **Baseline / tested source commit:** forked from `origin/main` `dd7ff40` (PR #77, `T02-04` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy` (the repository's own required checks are evidence/manifest policy workflows, unrelated to Rust). All checks below were executed locally on the development host recorded in `specs/002-post-r1-canonical-core-convergence/corrective-t01/reference-hardware.md`.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`, `git log --oneline --decorate -5`, `gh pr view 77 --json state,mergedAt,mergeCommit` confirmed `main` at `dd7ff40` ("feat(t02-04): find work with a disposable current lexical index (#77)"), merged, matching `specs/CURRENT.md`'s `T02-04_STATUS=COMPLETE`.
- `gh api repos/TheHalfMoon/Flake/commits/dd7ff40.../check-runs` confirmed all 8 named checks completed `success` on the merge commit.
- Read the full `T02-05` task-contract row (build plan lines 899-921) plus the plan sections it names: §12 (Export UX row: "destination, full-backup versus selected-project package, inclusion summary, privacy warning and verified result; partial output never called complete"), §14 (component diagram: "owner-reviewed export -> immutable receipt -> explicit package file"), §15 (`Export/backup manifest` row and the hash-boundary/manifest-integrity-root paragraph), §20 (S02/S03/S07/S10), §22 (F15/F17), §27, §29 (V03-V07/V09/V13).
- Read `src/backup.rs` (`T01-05`) in full before writing any code, specifically to determine reuse-vs-independent-module and to confirm its own recorded scope boundary ("Not implemented here: ...a selected-project backup (plan §16's 'shareable project package'...)") — the exact gap this task fills, with a genuinely different artifact format (see "Architecture" below).

## Scope actually touched

One new module (`src/export.rs`), one new public format document (`docs/formats/portable-export-v1.md`), plus `src/canonical.rs` (one new read method, `all_revisions`, and its `RevisionEnvelope` return type — explicitly named in this task's own "Files/components: Core portable exporter"), `src/cli.rs` (two new subcommands), and `src/lib.rs` (module registration). No `Cargo.toml`/`Cargo.lock` change.

```text
src/canonical.rs      |   33 +
src/cli.rs             |   51 +-
src/lib.rs              |    1 +
src/export.rs (new)     |  718 +++++++++++++++
docs/formats/portable-export-v1.md (new)
```
(`git diff --stat main -- src/`, `raw/04-diff-stat.txt`; `src/export.rs` is untracked against `main` so `git diff --stat` does not list it — its line count is recorded separately in the same file.)

## Architecture: a new, independent portable-export module — not an extension of `backup.rs`

`src/backup.rs` (`T01-05`) makes an owner-controlled, *restorable* copy of a healthy vault using SQLite's own Online Backup API — its output is still `canonical.sqlite`, openable only by this exact crate, and its own module doc already records "a selected-project backup" as explicitly out of its scope. This task's own objective — "leave Flake with readable content and complete declared history," verified by "a generic JSON/byte reader... with no Flake dependency" — needs a genuinely different artifact: individual, human/tool-readable files, not a raw database file. Per this repository's now-established pattern (`T02-02`'s `capture.rs`, `T02-03`'s `relation.rs`, `T02-04`'s `index.rs` — each a new module for a new capability rather than retrofitting an existing one built for a different purpose), this task adds `src/export.rs`.

**Kind-string collision, deliberately avoided.** §15's `Export/backup manifest` row names two conceptual kinds, "full-backup" and "project-package" — but `backup.rs` already spends the literal string `"full-backup"` on its own, physically incompatible SQLite-snapshot artifact. Reusing that exact string here for a completely different file format would let a reader mistake one for the other. This task's manifest instead uses `"export-full"`/`"export-project"` — distinct strings for a distinct, documented format (`docs/formats/portable-export-v1.md` states this explicitly, including the exact reason).

**"Keep raw canonical payload bytes where JSON reserialization would alter them."** Every revision file's `payload_raw` field is the exact stored payload string, embedded as a JSON string value — never parsed into a `project::RecordPayload` and re-emitted (which could reorder keys or change whitespace). `payload_raw_is_byte_identical_to_canonical_state` proves this directly: a Unicode/CRLF/emoji note body's exported `payload_raw` is asserted byte-for-byte equal to `CanonicalStore::read_current`'s own raw payload string.

**Full history, not current state.** `store.all_revisions()` (this task's own new `canonical.rs` method) returns every revision ever recorded, oldest first, with its complete §15 envelope — filtered to the export's scope, every revision of every in-scope object is written, never only the current one. `full_export_includes_every_project_and_full_history` proves this with an updated note contributing two revision files, not one.

**Scope reuses the already-established per-kind listing functions.** A project export's object-ID set is the union of `project::list_project_records`, `capture::list_project_sources`, and `relation::list_project_relations` for that project, plus the `Project` object itself — the identical functions the live product already uses to answer "what belongs to this project," not a separately re-derived membership rule that could drift from them.

**Publication: staging directory, then one atomic rename — the identical pattern `backup.rs` already established**, reused here (not shared code, since the artifacts are unrelated, but the identical idiom): `export_to_new_root` writes everything into `.fehrest.export-staging-<uuid7>` under the destination, and only `fs::rename`s it to the published `.fehrest-export` name once every member and the manifest are written. `export_never_overwrites_an_existing_destination` proves the no-clobber refusal (F17 "partial output never overwrites a prior export").

**Independently recomputable integrity root.** `integrity_root_is_independently_recomputable` calls the exact hashing function a from-scratch generic reader would have to reimplement from `docs/formats/portable-export-v1.md`'s own specification, and asserts it matches the manifest's own `integrity_root` — the closest in-repository proxy for "a generic reader with no Flake dependency can validate this," short of writing a second, genuinely independent implementation.

## Security boundary — what this task actually enforces (S02/S03/S07/S10)

- **Cross-project isolation is exact, not just "excludes matching content."** `project_export_excludes_other_projects_entirely` checks two things: the other project's note *text* never appears anywhere in the exported files, and the other project's own *object ID string* never appears either — a stricter bar than merely filtering visible content, since a leaked ID alone would already be a scope violation (S02).
- **No local filesystem/grant authority to leak, by construction.** Nothing in the current record model (`Project`/`Note`/`Action`/`Decision`/`Source`/`Relation`) carries real filesystem access or a disclosure grant — `Source::claimed_path`/`claimed_repository`/`claimed_commit` are already-documented (`T02-02`) non-authoritative descriptive strings a user typed, never opened paths. This task adds no new authority-bearing field, so there is nothing new to redact; recorded explicitly in both the module doc and the format doc rather than left implicit.
- **Safe generated paths, structurally.** Every export member path is built from a UUID or a plain decimal integer (`object_id`, `revision_id`, `recorded_seq`) plus two fixed filenames (`export-manifest.json`, `README.md`) — never from user-supplied text (a title, a label, a statement) — so there is no sanitization step to get wrong or forget.
- **Unknown project reference refused before any write.** `unknown_project_reference_is_refused_before_any_write` proves `export_to_new_root` checks project existence and produces zero filesystem side effects on refusal.

## Tests added (9 new; all pass)

| Test | File | What it proves |
|---|---|---|
| `preview_reports_scope_without_writing_anything` | export.rs | §12 "scope shown before writing" — correct counts, zero filesystem writes |
| `full_export_includes_every_project_and_full_history` | export.rs | Full export scope is exact; an updated note contributes its full revision history, independently re-enumerated and hash-verified against the manifest |
| `project_export_excludes_other_projects_entirely` | export.rs | S02: neither the other project's content nor its bare object ID ever appears |
| `payload_raw_is_byte_identical_to_canonical_state` | export.rs | "Keep raw canonical payload bytes" — Unicode/CRLF/emoji round-trips byte-for-byte |
| `note_bodies_also_get_a_plain_markdown_sidecar_file` | export.rs | "Generic-reader instructions and plain Markdown rendering" |
| `export_never_overwrites_an_existing_destination` | export.rs | F17: no-clobber publication |
| `integrity_root_is_independently_recomputable` | export.rs | The manifest's own integrity root matches an independent recomputation from its documented algorithm |
| `unknown_project_reference_is_refused_before_any_write` | export.rs | Invalid scope reference refused, zero side effects |
| `export_preview_and_run_work_through_the_cli` | cli.rs | End-to-end through the actual CLI dispatcher: preview, full export, project-scoped export, manifest `kind` verification |

## Commands executed (all re-run on this branch against `main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **259/259 pass**, 0 failed (226 lib [217 pre-existing + 9 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib export::` | `raw/05-export-module-tests-isolated.txt` | 8/8 pass |
| `cargo test --locked --lib cli::` | `raw/06-cli-module-tests-isolated.txt` | 8/8 pass |
| `cargo test --locked --lib backup::` | `raw/07-backup-module-tests-isolated.txt` | 7/7 pass (confirms `backup.rs` is completely untouched by this task) |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat main -- src/` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `git diff --check` | (terminal output, no artifact file — empty output = clean) | Exit 0, no whitespace errors |
| Environment capture | `raw/09-environment.txt` | Matches `reference-hardware.md`'s frozen development profile |
| Baseline head | `raw/08-baseline-head.txt` | `dd7ff40...` on both `HEAD` and `origin/main` before this task's own commit |

Raw artifact manifest: `raw/00-manifest.txt` (generated before its own content existed, so does not hash itself — same pattern every prior `flake-v1` evidence report in this phase already uses).

## Failed attempts / exclusions

- **A genuinely broken assertion, caught by `cargo build` itself (a type error), not a passing-but-wrong test.** An early draft of `project_export_excludes_other_projects_entirely` chained `assert!(...).then_some(()).unwrap_or_else(...)` on the negative-leak check — a nonsensical expression left over from an editing mistake (`assert!` returns `()`, not an `Option`), which does not compile. Rewritten as two plain `assert!` calls (content must not leak; the other project's own ID must not leak either). Caught immediately, before any test ever ran with wrong logic.
- A single dead intermediate `let kind = ...` binding, immediately shadowed by a second `let kind = ...` one line later (leftover from drafting), simplified to one `if`/`else` expression. Not a clippy finding — self-caught during a pre-commit read of the diff.
- No test was skipped, deleted, or weakened to reach green.

## Performance gate

No dedicated timing harness was run for this task. Export cost is proportional to `all_revisions()`'s full-table scan (bounded by the store's total revision count, not filtered server-side) plus one file write per in-scope revision — the same documented S-scale-only limitation `T02-01`-`T02-04` already carry for their own full scans. No dataset-M/L measurement was taken, consistent with those tasks' own identical deferral to `T05-02`; this task's own performance gate ("M export ≤180 s, L streaming/cancel and RSS ceilings") remains unmeasured pending that hardware-qualified pass, and streaming (holding less than the full revision set in memory at once) is explicitly recorded as not implemented in both the module doc and the format doc.

## Durability gate

Publication uses the identical staging-directory-then-atomic-rename pattern `backup.rs` already durability-proved (`D5`) for its own publication — applied here to a directory containing many small files rather than two large ones, with the same "nothing is renamed into place until everything staged is complete" guarantee. No fault-injection point was added for this task to separately regression-test: every write in this module is an ordinary `std::fs` operation on a brand-new destination the canonical store's own writer lock never touches, so an interruption at any point before the final `fs::rename` leaves `dest_root` with, at worst, an orphaned `.fehrest.export-staging-*` directory and no published `.fehrest-export` at all — never a half-published one, since `fs::rename` is the sole publication step and is atomic on the same volume (the identical reasoning `backup.rs`/`recovery.rs`/`index.rs` already rely on for their own renames).

## Cross-platform gate

Native development profile only (Windows 11/NTFS, matching `reference-hardware.md`), consistent with every prior `flake-v1` task. Every exported member path is built exclusively from UUIDs and plain integers (see "Security boundary" above), which are valid path components on every mainstream filesystem without any OS-specific escaping. Remaining profiles are `T05-02`'s scope.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Manifest covers every declared canonical member and all exclusions | Satisfied — `members` lists every published file with length/SHA-256; `omissions` is present (always empty in this task's scope, since nothing in-scope is ever excluded) |
| Generic JSON/byte reader validates exports with no Flake dependency | Satisfied — `docs/formats/portable-export-v1.md` fully specifies the format independent of any Flake source; `integrity_root_is_independently_recomputable` proves the documented algorithm matches the implementation |
| Full-backup and selected-project package types | Satisfied — `export-full`/`export-project`, `full_export_includes_every_project_and_full_history` / `project_export_excludes_other_projects_entirely` |
| Exact members and omissions | Satisfied — see above |
| Safe generated paths and no-clobber staging publication | Satisfied — UUID/integer-only paths; `export_never_overwrites_an_existing_destination` |
| Generic-reader instructions and plain Markdown rendering | Satisfied — `README.md` written into every export; `note_bodies_also_get_a_plain_markdown_sidecar_file` |
| Keep raw canonical payload bytes where JSON reserialization would alter them | Satisfied — `payload_raw_is_byte_identical_to_canonical_state` |
| Sensitive history and scope shown before writing | Satisfied — `preview_export`/`export-preview`, exercised both directly and through the CLI |
| Selected export is not labeled a full backup | Satisfied by construction — `kind` is `"export-project"` whenever `project_id` is `Some`, never `"export-full"` |
| Core portable exporter, public format docs, CLI export preview and tests | Satisfied — `src/export.rs`, `docs/formats/portable-export-v1.md`, `export-preview`/`export-run` |

## Completion condition

Every acceptance clause above is satisfied. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (259/259). One non-compiling draft assertion was caught by `cargo build` itself before any test ever ran with it. No sealed evidence altered; no force-push; no historical evidence file touched; `backup.rs` and its own tests are completely untouched by this task (`cargo test --locked --lib backup::` re-run and green); no change to `T01-02`-`T02-04`'s already-audited mutating logic beyond one new, purely-additive `canonical.rs` read method. `T02-05` is complete.

## Next frontier

`T02-06` — Validate and import portable packages into a new vault. Depends on `T02-05` (this task) — the natural counterpart to this task's exporter, importing `docs/formats/portable-export-v1.md`'s own format back into a fresh canonical store.
