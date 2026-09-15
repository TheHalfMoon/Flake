# T02-03 evidence report — Record decisions and complete actions with visible history

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §12/§13/§15/§20/§22/§27-29, task `T02-03` — third task of `P02`
- **Baseline / tested source commit:** forked from `origin/main` `bfff06c9` (PR #75, `T02-02` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy`. All checks below were executed locally on the development host recorded in `specs/002-post-r1-canonical-core-convergence/corrective-t01/reference-hardware.md`.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`, `git status --short --branch`, `git log --oneline --decorate -20` confirmed `main` at `bfff06c9` ("feat(t02-02): capture notes and selected local evidence with recoverable bytes (#75)"), matching `specs/CURRENT.md`'s `T02-02_STATUS=COMPLETE` and PR #75 merged (`gh pr view 75` → `state: MERGED`, `mergeCommit.oid: bfff06c9...`).
- Confirmed the 8 GitHub Actions checks on `bfff06c9` all `success` via `gh api repos/TheHalfMoon/Flake/commits/bfff06c9.../check-runs`.
- Confirmed only two long-lived unrelated open PRs exist (#2, #40 — historical Fehrest planning docs, not touched here).
- Read `AGENTS.md`, `specs/CURRENT.md`, and the full `T02-03` task-contract row (build plan lines 851-873), plus the plan sections it names: §12 (Action/Decision/Evidence/Override UX rows), §13 (I01-I12), §15 (`Action`/`Decision`/`Relation` entity rows and the "half-open `[from,to)`", "decision keys" and "Evidence is a source revision/artifact plus a relation, not a second copy" paragraphs), §20 (S05/S06), §22 (F09/F10/F20/F21), §27 (event metadata ≤16 KiB), §29 (V02-V05/V07/V09).
- Read `docs/evidence/flake-v1/T02-02/REPORT.md` in full (its own "Next frontier" section names this task's scope precisely) and `src/project.rs`'s existing `T02-01` doc comments, which already recorded exactly what `T02-01` deliberately deferred to this task — used as the authoritative scope boundary rather than re-deriving it from the plan prose alone.

## Scope actually touched

One new module (`src/relation.rs`), plus `src/project.rs` (the bulk of this task: `Action`/`Decision` struct extensions and the full lifecycle), `src/capture.rs` (exposed/added RFC3339 helpers, reused rather than duplicated), `src/cli.rs` (new subcommands), and `src/lib.rs` (module registration). No `Cargo.toml`/`Cargo.lock` change — no new dependency was needed.

```text
src/capture.rs |  105 +-
src/cli.rs     |  620 +++++++++++++++--
src/lib.rs     |    1 +
src/project.rs | 1658 +++++++++++++++++++++++++++++++++++++++++++++++++++++---
src/relation.rs (new) | 622 ++++++++++++++++++++
```
(`git diff --stat main -- src/`, `raw/04-diff-stat.txt`; `src/relation.rs` is untracked against `main` so `git diff --stat` does not list it — its line count is recorded separately in the same file.)

## Architecture: a sixth `RecordPayload` kind, and richer existing ones — not a new mechanism

Exactly like `T02-02`'s `Source`, `Relation` (`src/relation.rs`) is an ordinary opaque-payload canonical object committed through the existing `CommandTarget::CreateObject`/`CanonicalWriter::commit` — no new `CommandTarget` variant, no schema change, no event store, no graph engine. `Action`'s and `Decision`'s new fields are ordinary struct fields on the same two `RecordPayload` variants `T02-01` already defined; every lifecycle transition is an ordinary `commit_update`, inheriting the identical atomic-commit, idempotency and expected-revision-conflict guarantees `T01-03`/`T01-07`/`T02-01` already proved.

**"Visible history" without a new event store.** Every transition already produces a new immutable revision through the exact mechanism `project::archive_project` proved back in `T02-01` ("history preserved"). This task's actual contribution is making each revision *self-describing*: `Action::last_transition` (`{from, to, reason, actor, at}`) records exactly which transition produced *that* revision. Reading `CanonicalStore::history(action_id)` and parsing each revision's payload directly reconstructs the complete transition history — current state, every previous state, and the reason for each reopen or dependency-override — without a parallel canonical mechanism. `history_reconstructs_the_full_transition_sequence` (folded into `action_lifecycle_table_allows_every_documented_transition`) proves this by walking a full 7-revision history and asserting the recorded transitions at specific indices, including that the initial `create` revision carries no transition (`None`) and a reopen revision carries its reason.

**Evidence linkage without a second copy.** §15's closing paragraph is explicit: "Evidence is a source revision/artifact plus a relation, not a second copy." Neither `Action` nor `Decision` gained an embedded evidence-IDs field; a `Relation{type: Supports, from: decision_or_action, to: source}` is the sole record, discovered via `relation::list_relations_for_object`. The same reasoning applies to a decision's override rationale: it lives once, on the `Supersedes` `Relation`'s own `note` field — not duplicated onto the superseded `Decision`.

**Two atomic commands, not one invented multi-object transaction.** `crate::canonical::CommandTarget` admits exactly one object per command (its own doc comment, unchanged by this task). `supersede_decision` therefore performs two sequential, independently atomic commands — create the `Supersedes` `Relation`, then update the old `Decision`'s `lifecycle` — documented explicitly in `supersede_decision`'s own doc comment, including the honest partial-state behavior if the process dies between the two (the `Relation` alone remains a complete, valid, inspectable record; nothing is corrupted, and step 2 alone can be retried). No new multi-object primitive was invented to avoid this — `AGENTS.md` §6/§7 (no new canonical mechanism) was treated as binding here.

**Reused, not duplicated, RFC3339 handling.** `T02-02`'s `capture.rs` already implemented correct-for-any-past-instant UTC formatting (Hinnant's `civil_from_days`). This task needed the *inverse* (parsing a user-supplied `valid_from`/`valid_to` string) for `Decision`'s half-open valid-time interval. `capture::days_from_civil` (Hinnant's algorithm's counterpart) plus `capture::parse_rfc3339_utc` were added there — not in `project.rs` — and `now_rfc3339_utc`/`rfc3339_utc_from_unix_seconds` were widened from private to `pub(crate)` so `project.rs` reuses the same formatter for `Action`/`Decision` timestamps rather than a second implementation. `parse_rfc3339_utc` validates an impossible calendar date (e.g. 2024-02-30) by round-tripping the parsed value back through the existing formatter and comparing byte-for-byte, rather than adding a second days-in-month table.

## What was built, and why — grounded in the task contract

1. **`Action` state machine** (`project.rs`): `ActionState` (already defined by `T02-01`) now has a real transition table (`action_allowed_transition`) covering exactly `Open→{Doing,Blocked,Done,Cancelled}`, `Doing→{Blocked,Done,Cancelled}`, `Blocked→{Doing,Done,Cancelled}`, and reopening (`{Done,Cancelled}→Doing`) — every other pair, including every self-transition, is rejected. `dependency_ids` is ordered and cycle-checked (`validate_action_dependency_graph`, a from-scratch depth-first reachability walk over the project's live actions — deliberately not reusing `temporal.rs`'s `Memory`-coupled `reaches`, for the same reason `relation.rs` doesn't: see its own module doc). `complete_action` requires a nonempty summary and, when any dependency is not `Done`, an explicit nonempty override reason ("blocked-dependency completion requires a reasoned override"). `reopen_action` requires an explicit nonempty reason ("any reopening requires an event").
2. **`Decision` evidence linkage, acceptance, override/supersession, valid-time** (`project.rs`): `basis`/`verification` fields; `accept_decision` (`Draft`→`Accepted` only, owner-only — no other code path ever sets `Accepted`); `withdraw_decision` (`Draft`/`Accepted`→`Withdrawn`, mandatory reason); `supersede_decision` (both decisions must already be `Accepted`, must share `decision_key`, cycle-checked via the `Relation` layer, preserves the old decision's prior `Accepted` revision in history); half-open `[valid_from, valid_to)` validated by `validate_valid_interval` (parses each bound, rejects an equal-or-inverted pair).
3. **`Relation`** (new `relation.rs`): the sixth `RecordPayload` kind. `create_relation` validates both endpoints exist, belong to the same project, are not `Project`/`Relation` themselves, rejects a self-loop, and — for `Supersedes` only — requires both endpoints to be `Decision`s and rejects a cycle (direct or transitive) via a dedicated graph walk over already-admitted `Supersedes` relations. Endpoints are pinned to the exact revision current at creation time (documented "endpoint revision policy").
4. **CLI wiring** (`cli.rs`): `action-start`/`action-block`/`action-complete`/`action-cancel`/`action-reopen`/`action-set-dependencies`, `decision-accept`/`decision-withdraw`/`decision-supersede`, `relation-create`/`object-relations`/`project-relations`; `action-create`/`decision-create`/`capture` extended with the new optional fields (`--depends-on`, `--basis`, `--verification`, `--valid-from`/`--valid-to`), all defaulting so every pre-existing CLI invocation in `T02-01`/`T02-02`'s own tests continues to work unchanged.
5. **`specs/CURRENT.md` pointer correction**: `T02-02_MERGE_COMMIT=PENDING_PR_MERGE` corrected to the actual merged SHA `bfff06c96917726ffff00881afc09c93dcd22599`, folded into this task's own commit per the handoff's instruction not to spend a standalone cosmetic PR on it.

## Security boundary — what this task actually enforces

Per §20 S05/S06, restated against the literal implementation:

- **Owner-only acceptance, structurally.** `accept_decision` is the *only* function anywhere in this crate that sets `DecisionLifecycle::Accepted`. `create_decision` always starts `Draft` regardless of `basis` — an `AgentProposal`-basis decision cannot self-promote to `Accepted` by construction, not by a runtime role check (`only_a_draft_decision_can_be_accepted` proves re-acceptance from `Accepted` is refused, not silently reapplied).
- **No content-derived authority.** Nothing in `relation.rs`/`project.rs`'s new code reads a `Decision`/`Note`/`Source` body to decide anything; every admission decision (existence, project membership, kind, cycle) is computed from Core-owned envelope fields (`object_id`, `project_id`, `kind`, the current revision), never from payload text.
- **Fail-closed conflict detection.** Every transition function (`transition_action`, `accept_decision`, `withdraw_decision`, `set_action_dependencies`, and `supersede_decision`'s second step) takes an explicit `expected_revision_id` and relies on `CanonicalWriter::commit`'s existing conflict check — deliberately *not* the simpler "read current then write" pattern `set_project_active`/`set_source_active` use for a harmless boolean toggle. `action_transition_conflicts_on_a_stale_expected_revision` proves a stale caller view is rejected, not silently overwritten (F21).
- **Fail-closed dependency evaluation.** `unmet_dependencies` treats a missing or unreadable dependency ID as unmet (never silently ignored as "must be fine"), so a completion gate cannot be defeated by a dangling reference.

## Tests added (28 new; all pass)

| Test | File | What it proves |
|---|---|---|
| `parse_rfc3339_utc_round_trips_known_fixed_points` | capture.rs | The new parser inverts the existing formatter exactly at known fixed points |
| `parse_rfc3339_utc_rejects_malformed_and_impossible_dates` | capture.rs | Malformed shape and impossible calendar dates (Feb 30) are refused, not coerced |
| `action_lifecycle_table_allows_every_documented_transition` | project.rs | Full `Open→Doing→Blocked→Doing→Done→(reopen)Doing→Cancelled` path; asserts completion fields set, reopen clears them, and full 7-revision history is reconstructible with the correct `last_transition` on each revision |
| `forbidden_action_transitions_are_rejected` | project.rs | A self-transition and `Done→Blocked` are both rejected (not in the table) |
| `reopening_without_a_reason_is_rejected` | project.rs | "Any reopening requires an event" — a blank reason is refused, head/history unchanged |
| `completing_without_a_summary_is_rejected` | project.rs | "Completion requires summary" |
| `completion_with_unmet_dependency_requires_an_override_reason` | project.rs | Refused without a reason, succeeds and records the reason once given |
| `completion_succeeds_without_override_once_dependency_is_done` | project.rs | The gate only fires when actually unmet |
| `action_dependency_must_exist_and_be_in_the_same_project` | project.rs | Missing and cross-project dependency IDs are both refused before any transaction opens |
| `action_dependency_direct_and_transitive_cycles_are_rejected` | project.rs | Direct 2-cycle and self-reference both rejected |
| `action_transition_conflicts_on_a_stale_expected_revision` | project.rs | F21: stale expected revision on a transition is refused, not overwritten |
| `valid_interval_rejects_malformed_and_inverted_bounds` | project.rs | Malformed timestamp, inverted bounds, and equal (empty) bounds all rejected |
| `valid_interval_accepts_a_well_formed_half_open_range` | project.rs | A valid `[from,to)` round-trips through creation |
| `only_a_draft_decision_can_be_accepted` | project.rs | Owner-only acceptance; re-acceptance from `Accepted` refused |
| `withdraw_requires_a_reason_and_preserves_prior_history` | project.rs | Mandatory reason; prior `Draft` revision remains in history |
| `supersede_requires_both_decisions_accepted_and_same_key` | project.rs | Three negative cases: neither accepted, only one accepted, different key |
| `supersede_preserves_prior_accepted_history_and_links_a_relation` | project.rs | Prior `Accepted` revision preserved; `Supersedes` relation discoverable from either endpoint carrying the reason, no second copy on the `Decision` |
| `supersede_rejects_a_cycle_between_two_decisions` | project.rs | The relation-layer cycle check independently proven against a real two-decision supersession |
| `links_a_decision_to_a_source_as_supporting_evidence` | relation.rs | The intended `T02-02`→`T02-03` linkage: `Decision`→`Source` |
| `rejects_a_self_loop` | relation.rs | A relation cannot name the same object at both ends |
| `rejects_a_cross_project_endpoint` | relation.rs | "Relations stay within one project" |
| `rejects_a_project_or_relation_as_an_endpoint` | relation.rs | `Project`/`Relation` themselves cannot be an endpoint |
| `supersedes_requires_decision_endpoints` | relation.rs | The deliberate `Supersedes`-is-`Decision`-only scope limit, enforced |
| `supersedes_rejects_a_direct_cycle` | relation.rs | A 2-node cycle is rejected |
| `supersedes_rejects_a_transitive_cycle` | relation.rs | A 3-node transitive cycle is rejected |
| `missing_endpoint_is_refused_before_any_mutation` | relation.rs | Transaction head unchanged on refusal |
| `endpoints_are_pinned_to_the_revision_current_at_creation` | relation.rs | The documented "endpoint revision policy": a later mutation of the endpoint does not retroactively change what the relation cites |
| `action_decision_relation_lifecycle_works_through_the_cli` | cli.rs | End-to-end through the actual CLI dispatcher: action create/start/complete, reopen-without-reason refusal, two competing decisions, accept both, link evidence via `relation-create`, supersede via `decision-supersede`, verify final state and relation count |

## Commands executed (all re-run on this branch against `main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **239/239 pass**, 0 failed (206 lib [178 pre-existing + 28 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib project::` | `raw/05-project-module-tests-isolated.txt` | 24/24 pass |
| `cargo test --locked --lib relation::` | `raw/06-relation-module-tests-isolated.txt` | 9/9 pass |
| `cargo test --locked --lib cli::` | `raw/07-cli-module-tests-isolated.txt` | 6/6 pass |
| `cargo test --locked --lib capture::` | `raw/08-capture-module-tests-isolated.txt` | 16/16 pass |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat main -- src/` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `git diff --check` | (terminal output, no artifact file — empty output = clean) | Exit 0, no whitespace errors |
| Environment capture | `raw/10-environment.txt` | Matches `reference-hardware.md`'s frozen development profile |
| Baseline head | `raw/09-baseline-head.txt` | `bfff06c9...` on both `HEAD` and `origin/main` before this task's own commit |

Raw artifact manifest (lengths and SHA-256 for every file above): `raw/00-manifest.txt`, generated before its own content existed and so does not hash itself (§15's own "manifest does not hash itself" pattern, applied here for the same reason).

## Failed attempts / exclusions

- **A tuple-destructuring bug in this task's own new CLI test, caught immediately by the test run, not discovered later.** `CanonicalStore::read_current` returns `(revision_id, payload)` in that order; the first draft of `action_decision_relation_lifecycle_works_through_the_cli` destructured it as `let (_, rev) = store.read_current(...)`, silently binding `rev` to the JSON *payload* instead of the revision ID, in three places. The resulting `expected revision conflict` error (attempting to use a JSON blob as an expected revision ID) was immediate and unambiguous on the first test run. Fixed by correcting the destructuring order at all three call sites — a test-authoring fix, not a product defect (no product code ever made this mistake; `commit_update`'s own conflict-check path is exactly what caught it).
- One test originally passed a literal empty string (`--reason ""`) through the CLI to probe `reopen_action`'s reason validation; this crate's existing hand-rolled CLI arg parser (`Args::get`, unchanged by this task) treats an empty flag value as absent (`missing required --reason`) one layer above where `reopen_action`'s own "non-empty reason" check would fire. Changed the test to pass a whitespace-only reason (`"   "`), which does reach `reopen_action`'s validation, and documented why in the test itself. Not a product defect: this is `T02-01`-era CLI-parser behavior, unrelated to this task's own scope, and the test now exercises the intended layer.
- No test was skipped, deleted, or weakened to reach green.

## Performance gate

No dedicated timing harness was run for this task. §27's relevant budgets here (`Save/write acknowledgement ≤64 KiB` p95 150 ms; event metadata ≤16 KiB) are unchanged from `T02-01`/`T02-02`'s own already-measured commit path — every `Action`/`Decision`/`Relation` mutation in this task is, structurally, the exact same `CanonicalWriter::commit` call those tasks already timed (`T02-01`: p95 6.2 ms; `T02-02`: p95 5.84 ms for a comparable small object). This task adds no new I/O shape (no new artifact encoding, no new file access) — only additional in-memory validation (a full-project scan for dependency/supersession cycle checks) ahead of the identical commit path. That scan's own cost at dataset M/L scale is `T02-04`'s indexing objective to bound properly (this task's dependency/cycle graphs are built from `list_current_objects`'s full scan, the same documented S-scale-only limitation every other `list_project_*` function in this crate already carries — not a new one). No dataset-M/L measurement was taken, consistent with `T02-01`/`T02-02`'s own equivalent deferral to `T05-02`.

## Durability gate

D1 regression for every new transition family: every `Action`/`Decision`/`Relation` mutation in this task is, structurally, the exact same `CanonicalWriter::commit`/`commit_update` call `T01-07`'s 100-fault-schedule matrix and `T02-02`'s `a_fault_during_source_commit_leaves_no_partial_admission` already regression-proved — no new transaction path exists for this task to separately fault-inject against. `supersede_decision`'s own two-step, two-command design is exactly what keeps this true: each step is independently the same already-proven atomic primitive, never a new joint-transaction mechanism that would need its own fault matrix.

## Cross-platform gate

Native development profile only (Windows 11/NTFS, matching `reference-hardware.md`), consistent with every prior `flake-v1` task. Nothing in this task's own new code is platform-conditional (no filesystem paths, no symlinks, no platform-specific API) — the RFC3339 date-math additions are pure integer arithmetic. Remaining profiles are `T05-02`'s scope.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| All allowed transitions produce exact immutable history | Satisfied — `action_lifecycle_table_allows_every_documented_transition` walks a full 7-revision history and asserts the recorded `last_transition` at specific points |
| Forbidden transitions/cycles reject | Satisfied — `forbidden_action_transitions_are_rejected`, `action_dependency_direct_and_transitive_cycles_are_rejected`, `supersedes_rejects_a_direct_cycle`, `supersedes_rejects_a_transitive_cycle`, `supersede_rejects_a_cycle_between_two_decisions` |
| Explicit judgment and override remain visible | Satisfied — dependency-override reason is recorded on the `Action`'s own transition; supersession reason is recorded on the `Relation`'s `note`; withdrawal reason on `Decision::withdrawal_reason` |
| Dependency validation and acyclic supersession | Satisfied — see above; `Supersedes` is additionally restricted to `Decision` endpoints and requires the same `decision_key` |
| Completion requires summary; blocked-dependency completion requires a reasoned override | Satisfied — `completing_without_a_summary_is_rejected`, `completion_with_unmet_dependency_requires_an_override_reason` |
| User overrides preserve conflicting/negative evidence | Satisfied — superseding never deletes the old `Decision`'s prior `Accepted` revision (`supersede_preserves_prior_accepted_history_and_links_a_relation`) |
| Agent-origin input remains draft until owner acceptance | Satisfied by construction — `create_decision` always starts `Draft` regardless of `basis`; only `accept_decision` reaches `Accepted` |
| Owner-only acceptance | Satisfied — `accept_decision` is the sole path to `Accepted`; `only_a_draft_decision_can_be_accepted` |
| Conflicting expected revision fails visibly rather than overwriting | Satisfied — `action_transition_conflicts_on_a_stale_expected_revision` (F21) |
| Relation connecting a Decision to a Source (`T02-02`'s deferral) | Satisfied — `links_a_decision_to_a_source_as_supporting_evidence` |
| CLI/product surface required by the task | Satisfied — `action_decision_relation_lifecycle_works_through_the_cli` |

## Completion condition

Every acceptance clause above is satisfied. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (239/239). One test-authoring bug (the swapped tuple destructuring) was caught by this task's own first test run and fixed at the root cause — no product code defect. No sealed evidence altered; no force-push; no historical evidence file touched; no change to `T01-02`-`T01-07`'s, `T02-01`'s, or `T02-02`'s already-audited mutating logic beyond exposing two existing private `capture.rs` helper functions as `pub(crate)` and adding two new ones in the same file. `T02-03` is complete.

## Next frontier

`T02-04` — Find work with a disposable current lexical index. Depends on `T02-03` (this task). Every `list_project_*`/dependency-graph/cycle-check function this task added is a documented full scan, exactly the limitation `T02-04`'s own objective exists to bound.
