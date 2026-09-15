# T02-04 evidence report — Find work with a disposable current lexical index

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §14/§15/§16/§20/§22/§27-29, task `T02-04` — fourth task of `P02`
- **Baseline / tested source commit:** forked from `origin/main` `e8a285d` (PR #76, `T02-03` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy` (the repository's actual required checks are evidence/manifest policy workflows, unrelated to Rust — see `raw/`). All checks below were executed locally on the development host recorded in `specs/002-post-r1-canonical-core-convergence/corrective-t01/reference-hardware.md`.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`, `git log --oneline --decorate -5`, `gh pr view 76 --json state,mergedAt,mergeCommit` confirmed `main` at `e8a285d` ("feat(t02-03): record decisions and complete actions with visible history (#76)"), merged, matching `specs/CURRENT.md`'s `T02-03_STATUS=COMPLETE`.
- `gh api repos/TheHalfMoon/Flake/commits/e8a285d.../check-runs` confirmed all 8 named checks (`manifest-check`, `canonical-equality`, `validate`, `test-review-binding`, `test-statistical-design`, `test-validator`, `test-scorer`, `verify-artifacts`) completed `success` on the merge commit.
- Read the full `T02-04` task-contract row (build plan lines 875-897) plus the plan sections it names: §14 (component/process boundary diagram, "canonical.sqlite | derived.sqlite (FTS5)"), §15 ("indexes... disposable; canonical reads decide identity, scope, revision and lifecycle"), §16, §20 (S02/S04), §22 (F04/F15), §27 (search/rebuild performance budgets), §29 (V02-V07/V09/V13).
- Read `src/derived.rs` in full — Phase T's own existing FTS5 index — to determine reuse-vs-replace before writing any code (see "Architecture" below for why it was not extended).

## Scope actually touched

One new module (`src/index.rs`), plus `src/canonical.rs` (one new read method, `revisions_since` — explicitly named in this task's own "Files/components: ...canonical query interface"), `src/cli.rs` (four new subcommands), and `src/lib.rs` (module registration). No `Cargo.toml`/`Cargo.lock` change — FTS5 was already available through the existing `rusqlite` `bundled` feature (proven by `derived.rs`'s own already-passing FTS5 tests; no new dependency or feature flag was needed).

```text
src/canonical.rs      |   26 +
src/cli.rs             |  106 +++-
src/lib.rs              |    1 +
src/index.rs (new)      |  981 +++++++++++++++
```
(`git diff --stat main -- src/`, `raw/04-diff-stat.txt`; `src/index.rs` is untracked against `main` so `git diff --stat` does not list it — its line count is recorded separately in the same file.)

## Architecture: a new, independent index module — not an extension of `derived.rs`

`src/derived.rs` is Phase T's own FTS5 index, coupled end-to-end to the historical `ObjectId`/`ObjectRecord`/`Vault` format-1 model still used by this crate's original `init`/`add`/`scan`/`rebuild`/`search` CLI commands. It has no notion of a canonical transaction sequence, no incremental maintenance (its own doc comment says "Full rebuild only... `INCREMENTAL_REINDEX = YAGNI_DEFERRED`"), and no generation/checkpoint concept — none of which this task's acceptance criteria could be satisfied by extending. Per this repository's own established pattern (`T02-02`'s `capture.rs`, `T02-03`'s `relation.rs` — both new modules rather than retrofitting Phase T code onto the format-2 model), this task adds `src/index.rs`: a fully independent index over the format-2 canonical store's typed records, in its own database file (`derived-fts.sqlite`, deliberately distinct from `derived.rs`'s own literal `derived.sqlite` filename — reusing that exact name for an unrelated schema, even in a different directory, would be needlessly confusing). Neither index can corrupt or block the other; `derived.rs` and its CLI commands are completely untouched by this task.

**Indexable "work"** is exactly `Note`/`Action`/`Decision` — the identical set `project::list_project_records` already calls "work records". `Source` (opaque bytes) and `Relation`/`Project` (no free-text body) are not indexed.

**Full rebuild is crash-safe by construction**, using the exact "staging file, then one atomic `fs::rename`, never partial publish" pattern `backup.rs`/`recovery.rs` already established: a rebuild is written entirely into a fresh `.fehrest.index-staging-<uuid7>.sqlite` file and only published by renaming it over the live path once completely built. An interruption at any point before that rename leaves the previously-published index (if any) completely untouched and fully queryable — `interrupted_rebuild_leaves_the_previous_generation_fully_usable` proves this directly (F04 "interrupted rebuild retains previous usable generation").

**Incremental update is checkpointed by transaction sequence, in one SQLite transaction.** `incremental_update` reads `CanonicalStore::revisions_since` the index's own last `built_through_seq` (a new `canonical.rs` read method added by this task, mirroring `history`'s existing query shape but filtered by sequence rather than by object), re-derives the FTS row for exactly the objects that changed, and advances `built_through_seq` to the highest sequence it actually processed — all inside one SQLite transaction on the index database, so an interruption mid-update rolls back to the previous checkpoint rather than leaving a partial, uncheckpointed update. `full_rebuild_and_incremental_updates_produce_identical_search_results` is this task's own named "rebuild-vs-incremental oracle": one canonical store, five mutations, indexed two different ways (index A incrementally after every mutation; index B by a single full rebuild after all five) — both converge to the identical hit-ID set.

**Never trust the index — always re-verify against fresh canonical state.** [`search`]'s SQL query deliberately does **not** filter by the index's own cached `project_id` column, even though that column exists and would make project-scoped queries cheaper — a poisoned or merely stale `project_id` hint must never decide inclusion *or* exclusion at the SQL layer. Every raw FTS match is re-read from `CanonicalStore::read_current` fresh; a candidate whose live canonical project/kind no longer matches what the caller asked for is dropped, never trusted from the cache (I06, S02/S04). `a_poisoned_project_hint_in_the_index_cannot_leak_into_another_projects_scoped_search` proves this directly: the index's `project_id` column is corrupted by a raw SQL `UPDATE` outside this module's own API, and a scoped search neither leaks the record into the wrong project nor loses it from its real one.

**Canonical fallback, not false `NoResults`.** When the index cannot be opened at all — missing file, or a corrupt/foreign file at the expected path — `search` does not return an empty result set disguised as "nothing matches". It performs a bounded, literal, case-insensitive substring scan directly over `CanonicalStore::list_current_objects`, labeled `SearchStatus::CanonicalFallback` so a caller can show the user this was the slower unindexed path. `missing_index_falls_back_to_a_bounded_canonical_scan_not_a_false_empty_result` and `corrupt_index_database_falls_back_rather_than_erroring_the_whole_search` both prove the fallback actually finds the real record, not merely reports a status.

## Security boundary — what this task actually enforces (S02/S04)

- **No SQL injection, no FTS5 syntax injection.** `literal_match_expression` reimplements the identical escaping technique `derived::literal_match_expression` already uses (each whitespace-separated token individually double-quoted, internal quotes doubled) — a fresh, independent copy rather than a shared call, consistent with this task's own "new module, not an extension" architecture. `literal_query_cannot_activate_fts_syntax` proves `"alpha OR beta"` matches nothing broader than the literal tokens.
- **Index hints cannot authorize or infer absence.** Every returned `Hit`'s `kind`/`project_id`/`title` comes from a fresh canonical re-read, never from the index's own cached row — proven by the poisoning test above. A missing/vanished object referenced by a stale index row is silently dropped from results, not fabricated.
- **Bounded resources.** `search`'s query-length and result-count bounds reuse the identical `crate::limits::MAX_QUERY_BYTES`/`MAX_SEARCH_RESULTS` constants `derived.rs` already established — `query_and_result_bounds_are_enforced` proves the query bound; the result bound is structural (`limit.min(MAX_SEARCH_RESULTS)`).

## Tests added (11 new; all pass)

| Test | File | What it proves |
|---|---|---|
| `rebuild_indexes_notes_actions_and_decisions_but_not_sources_or_relations` | index.rs | Exactly the "work record" set is indexed; a `Source`'s label never enters the index |
| `full_rebuild_and_incremental_updates_produce_identical_search_results` | index.rs | The named rebuild-vs-incremental oracle: identical hit-ID sets from two different index-building strategies against the same canonical state |
| `stale_index_is_labeled_but_still_returns_results` | index.rs | Lag is visible (`SearchStatus::Stale`) without blocking already-known-good results |
| `missing_index_falls_back_to_a_bounded_canonical_scan_not_a_false_empty_result` | index.rs | No index yet → real fallback results, not a misleading empty set |
| `corrupt_index_database_falls_back_rather_than_erroring_the_whole_search` | index.rs | A foreign/corrupt file at the index path degrades to fallback, not a hard error |
| `a_poisoned_project_hint_in_the_index_cannot_leak_into_another_projects_scoped_search` | index.rs | S02: a directly-corrupted `project_id` column cannot leak a record into a project it does not canonically belong to |
| `project_scoped_search_excludes_other_projects_results` | index.rs | Normal (unpoisoned) project scoping actually excludes the other project's matching record |
| `literal_query_cannot_activate_fts_syntax` | index.rs | S04: FTS5 operator syntax in user input stays literal |
| `query_and_result_bounds_are_enforced` | index.rs | Oversized query refused with `Error::LimitExceeded` |
| `interrupted_rebuild_leaves_the_previous_generation_fully_usable` | index.rs | F04: a stray incomplete staging file never affects the already-published generation |
| `fts_index_rebuild_search_and_status_work_through_the_cli` | cli.rs | End-to-end through the actual CLI dispatcher: pre-index fallback search, `fts-rebuild`, `fts-status`, a new mutation, `fts-update`, and a fresh scoped search |

## Commands executed (all re-run on this branch against `main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **250/250 pass**, 0 failed (217 lib [206 pre-existing + 11 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib index::` | `raw/05-index-module-tests-isolated.txt` | 10/10 pass |
| `cargo test --locked --lib cli::` | `raw/06-cli-module-tests-isolated.txt` | 7/7 pass |
| `cargo test --locked --lib canonical::` | `raw/07-canonical-module-tests-isolated.txt` | 24/24 pass (confirms `revisions_since` did not disturb existing canonical-store tests) |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat main -- src/` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `git diff --check` | (terminal output, no artifact file — empty output = clean) | Exit 0, no whitespace errors |
| Environment capture | `raw/09-environment.txt` | Matches `reference-hardware.md`'s frozen development profile |
| Baseline head | `raw/08-baseline-head.txt` | `e8a285d...` on both `HEAD` and `origin/main` before this task's own commit |

Raw artifact manifest: `raw/00-manifest.txt` (generated before its own content existed, so does not hash itself — same pattern `T02-03`'s evidence report already used and explained).

## Failed attempts / exclusions

- **Two design bugs, both caught by this task's own first test run, not discovered later.** (1) An early draft filtered `project_id` in the SQL `WHERE` clause when a project scope was given. `a_poisoned_project_hint_in_the_index_cannot_leak_into_another_projects_scoped_search`'s second assertion (searching the record's *real* project after poisoning *every* index row's `project_id` to a different one) failed: the poisoned index row no longer matched the SQL predicate for its own real project, so a legitimate scoped search silently lost a result it should have found. Root cause: letting *any* index-cached field drive SQL-level inclusion/exclusion — even for the "correct" project — means index corruption can degrade availability, not just (safely) fail to leak. Fixed by removing the `project_id` predicate from the SQL query entirely; project scoping now happens exactly once, during post-candidate re-verification against fresh canonical state, for both the block and the allow direction. (2) The same test's oracle sibling (`full_rebuild_and_incremental_updates_produce_identical_search_results`) originally created two *separate* canonical stores and asserted their first-created project IDs were equal — a wrong assumption, since `T01-03`'s UUIDv7 object-ID allocation is time-ordered and random, never deterministic or content-addressed, so two independently-created stores can never be expected to allocate matching IDs. Fixed by redesigning the test to use one canonical store with two independently-located index directories (one built by full rebuild, one by interleaved incremental updates), so the compared hit-ID sets are meaningfully comparable. Neither was a product defect — both were caught immediately by the test itself.
- Two `clippy::match_result_ok`/`clippy::manual_ok_err` lints on first `clippy` run (an `if let Some(..) = ...ok()` and a `match ... { Ok(c) => Some(c), Err(_) => None }`, each idiomatically simplified per clippy's own suggestion); both fixed to the suggested idiom, not suppressed.
- No test was skipped, deleted, or weakened to reach green.

## Performance gate

No dedicated timing harness was run for this task. Full-rebuild cost is proportional to `list_current_objects`'s existing full-scan cost (already the documented S-scale-only limitation `T02-01`–`T02-03` carry) plus one FTS5 insert per indexable record — no new I/O shape beyond what those tasks' own commit paths already exercise. Incremental-update cost is proportional to the number of objects changed since the last checkpoint via `revisions_since` (a single indexed-by-`recorded_seq` query), not the full corpus — the intended improvement this task's own "why it exists" clause names ("current rebuild deletes/inserts without an atomic generation"). No dataset-M/L measurement was taken, consistent with `T02-01`–`T02-03`'s own equivalent deferral to `T05-02`.

## Durability gate

Full-rebuild publication uses the identical staging-file-then-atomic-rename pattern this repository's own `backup.rs`/`recovery.rs` already durability-proved for directory publication, applied here to a single file — `interrupted_rebuild_leaves_the_previous_generation_fully_usable` is this task's own regression sample. Incremental updates are wrapped in one `rusqlite` `Connection::transaction`, so SQLite's own transaction atomicity (the same guarantee `canonical.rs`'s own commit path already relies on) is what makes an interrupted incremental update roll back to the prior checkpoint rather than leaving a partial one — no new fault-injection point was introduced for this task to separately regression-test with `CommitFaultPoint`, since that mechanism belongs to the *canonical* writer, not this derived index's own SQLite connection.

## Cross-platform gate

Native development profile only (Windows 11/NTFS, matching `reference-hardware.md`), consistent with every prior `flake-v1` task. `fs::rename`'s cross-platform atomic-overwrite behavior (Rust's std implementation uses `MOVEFILE_REPLACE_EXISTING` on Windows) is relied upon here exactly as `backup.rs`/`recovery.rs` already rely on it for directory renames. Remaining profiles are `T05-02`'s scope.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Clean/incremental results equivalent for all generated mutation sequences | Satisfied — `full_rebuild_and_incremental_updates_produce_identical_search_results` |
| Index corruption never leaks another project | Satisfied — `a_poisoned_project_hint_in_the_index_cannot_leak_into_another_projects_scoped_search` |
| Index corruption never changes canonical state | Satisfied by construction — `search`/`rebuild_index`/`incremental_update` only ever read from `CanonicalStore` (`&CanonicalStore`, never `&mut`) except through the existing `CanonicalWriter` commit path this task does not touch at all |
| Use FTS5 separate DB | Satisfied — `derived-fts.sqlite`, its own file, its own schema |
| Literal query escaping | Satisfied — `literal_query_cannot_activate_fts_syntax` |
| Canonical filters and post-candidate eligibility | Satisfied — every hit is re-verified against fresh canonical state before being returned |
| Increment from canonical transaction sequence; publish generation/checkpoint atomically only after success | Satisfied — `revisions_since`-driven incremental update, checkpoint advanced only inside the same committed SQLite transaction |
| Detect missing/corrupt/lagging index; visible bounded canonical fallback or SearchUpdating, never false NoResults | Satisfied — `SearchStatus::{Fresh,Stale,CanonicalFallback}`, all three exercised by tests |
| No filesystem watcher or graph registry | Satisfied by construction — this module is invoked only explicitly (`rebuild_index`/`incremental_update`/`search`), never by a background process |
| CLI search/rebuild | Satisfied — `fts-rebuild`/`fts-update`/`fts-status`/`fts-search`, exercised end-to-end in `fts_index_rebuild_search_and_status_work_through_the_cli` |

## Completion condition

Every acceptance clause above is satisfied. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (250/250). Two design/test-authoring issues (SQL-level project filtering allowing index poisoning to *hide* legitimate results; a wrong cross-store UUID-equality assumption in the oracle test) were caught by this task's own first test run and fixed at the root cause. No sealed evidence altered; no force-push; no historical evidence file touched; `derived.rs` and its own CLI commands are completely untouched; no change to `T01-02`–`T02-03`'s already-audited mutating logic beyond the one new, purely-additive `canonical.rs` read method. `T02-04` is complete.

## Next frontier

`T02-05` — Export complete owned state with a verified manifest. Depends on `T02-04` (this task).
