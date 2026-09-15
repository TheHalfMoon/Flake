# T03-02 evidence report — Resolve explicit temporal state and disagreement deterministically

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §15/§29, task `T03-02` — second task of `P03`, depends on `T03-01`
- **Baseline / tested source commit:** forked from `origin/main` `07694d5` (PR #81, `T03-01` merge; post-merge required checks all green — verified directly via `gh api .../commits/07694d5.../check-runs` before this task started)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent human/hosted reviewer merged this change into the historical record beyond what CI/hosted-review tooling below records. All checks were executed locally on the development host recorded in `specs/002-post-r1-canonical-core-convergence/corrective-t01/reference-hardware.md`.

## Live-truth reverification performed before this task started

- `git fetch origin --prune` confirmed `main` at `07694d572c295940e933f51fd72cfeddffb92e6d` ("feat(t03-01): track selected source revisions and relocation (#81)"), matching `specs/CURRENT.md`'s `T03-01_STATUS=COMPLETE` and `ACTIVE_IMPLEMENTATION_UNIT=T03-02`. `gh pr view 81` confirmed `state=MERGED` with merge commit `07694d5`. `gh api repos/TheHalfMoon/Flake/commits/07694d5.../check-runs` confirmed all 8 named post-merge checks (`manifest-check`, `canonical-equality`, `validate`, `test-review-binding`, `test-statistical-design`, `test-validator`, `test-scorer`, `verify-artifacts`) `completed`/`success`.
- Also corrected, in this task's own commit (per the founder-established handoff pattern — "each task's own commit corrects the previous task's pointer"), `specs/CURRENT.md`'s `T03-01_MERGE_COMMIT` from `PENDING_PR_MERGE` to the verified real SHA `07694d572c295940e933f51fd72cfeddffb92e6d`.
- Read the full `T03-02` task-contract row (build plan lines 995-1017) plus §15's `Decision` entity row (lifecycle `draft/accepted/superseded/withdrawn/tombstoned`, half-open `[valid_from, valid_to)`, "Acceptance owner-only; overrides require reason and explicit targets") and its "Semantic exactness" paragraph (half-open intervals, unknown/partial time never coerced into an unbounded assertion, decision keys are explicit per-project identifiers, "existing keys are selectable to place competing decisions in the same question").
- Read `src/project.rs` in full for the already-merged `Decision`/`create_decision`/`accept_decision`/`withdraw_decision`/`supersede_decision` code (`T02-03`) — confirmed directly (not assumed) that `accept_decision` is a wholly separate entry point from `supersede_decision`, so two decisions sharing a `decision_key` can each independently reach `Accepted` without ever being linked by an explicit `Supersedes` edge; and that `supersede_decision` performs two atomic steps (`Relation`, then the old decision's lifecycle flip) that always demote the superseded decision away from `Accepted` in the same operation that records the edge.
- Read `src/temporal.rs`/`src/memory.rs` (historical Phase T, immutable evidence per `AGENTS.md` §3) in full to understand exactly what algorithm shape they establish (a five-rung deterministic ladder — verification, basis, scope specificity, later `valid_from`, later `recorded_seq` — terminating in `Contradiction`, never a confidence number) and to confirm neither is reused or edited by this task, consistent with `relation.rs`'s and `source_check.rs`'s own prior non-reuse of historical code.
- Read `docs/evidence/flake-v1/T03-01/REPORT.md` again as the evidence-report template this report follows.

## Scope actually touched

One new module (`src/decision_state.rs`, a pure read-side resolver — no new `RecordPayload` kind, no canonical write path, no `canonical.rs` change), plus `src/cli.rs` (one new `decision-state` subcommand and one CLI-level end-to-end test) and `src/lib.rs` (module registration).

```text
src/cli.rs            | 234 +++++++++++++++
src/decision_state.rs | 800 ++++++++++++++++++++++++++++++++++++++++++++++++++
src/lib.rs             |   1 +
3 files changed, 1035 insertions(+)
```
(`git diff --stat 07694d5 -- src/`, `raw/04-diff-stat.txt`, captured after `git add -A` staged the new file.)

## Architecture: a new resolver module, not a reuse of `temporal.rs`/`memory.rs`

**Why the plan's own "`src/temporal.rs` and memory values or successor decision resolver" wording did not mean editing `temporal.rs`.** `temporal.rs`/`memory.rs` are historical Phase T evidence — immutable per `AGENTS.md` §3, exactly like `relation.rs`'s and `source_check.rs`'s own already-documented non-reuse of `temporal::validate_supersession` and `capture::capture_file` respectively for the same reason. `src/decision_state.rs` is the "successor decision resolver" the plan row names: a standalone module operating on §15's actual `Decision` entity (explicit owner-driven lifecycle and supersession), not on Phase T's `Memory` type (verification/basis-ranked, auto-extracted facts). The one thing genuinely carried forward from `temporal.rs`'s own resolver is a discipline, not code: "there is no rung 6" — ambiguity is surfaced (`Contradiction` there, `NeedsReview` here), never resolved by a manufactured confidence number.

**No ranking ladder is a deliberate simplification, forced by `Decision`'s own model, not a missing feature.** This task's acceptance criterion is narrower than Phase T's five-rung ladder: "Resolve only explicit accepted supersession; incomparable overlapping decisions with same key produce Conflict." This is also structurally forced, not merely chosen: `project::supersede_decision` (`T02-03`) always demotes the superseded decision's lifecycle away from `Accepted` in the same atomic step that creates the `Supersedes` `Relation` — so two decisions sharing a key can never simultaneously be `Accepted` *and* connected by an explicit supersession edge. By the time a supersession is recorded, the loser is already excluded from `Accepted`. Consequently, whenever more than one `Accepted` decision for the same key is simultaneously valid, that is unconditionally `NeedsReview` — this resolver never breaks that tie by recency, basis, or any other rung (I06/I08: no latest-wins, no model adjudication).

**One reconstruction path for "now" and "then".** `resolve_decision_state` always walks `CanonicalStore::all_revisions()` and, per object, keeps the latest revision with `recorded_seq <= as_of_recorded` (`as_of_recorded` defaults to the vault's current transaction head via `CanonicalStore::transaction_head()`). "Current state" is not a separate special-cased code path that could silently drift from the historical-reconstruction path — both are the exact same function call with a different cutoff.

**Negative evidence stays visible by construction.** `DecisionResolution.considered` lists every decision object sharing the requested `(project_id, decision_key)`, admitted or not — never only the winner. Each non-admitted entry's `exclusion_reason` names its lifecycle (folding in the decision's own `withdrawal_reason` when present, e.g. `"lifecycle is Withdrawn, not Accepted (withdrawal reason: \"changed our mind\")"`) or the specific half-open-interval boundary that excluded it at the requested `as_of_valid`.

**Deterministic, stable ordering.** `considered` is always sorted by `decision_id` ascending before being returned — the same order on every call/process, satisfying this task's own "stable deterministic ordering across processes" acceptance clause without depending on `HashMap` iteration order (which `decisions_as_of`'s internal reconstruction pass does use, but only as scratch state before the final sort).

## Verification method: an independently-written reference oracle (§29 V05)

This task's own "Verification method" clause requires "a pure reference oracle separate from production resolver ... record generated seed and cross-process equality." `src/decision_state.rs`'s `tests::reference_oracle` module is that oracle:

- It never calls `resolve_decision_state`, `decisions_as_of`, or `admission` — the production functions.
- It uses a structurally different data-access route: `CanonicalStore::history(object_id)` (walked in reverse, one object at a time) plus a `revision_id -> recorded_seq` map built once from `CanonicalStore::revisions_since(0)` (a different query/tuple shape than `all_revisions`'s full envelope struct the production path uses) — rather than the production path's single global fold over `all_revisions()`.
- It re-derives admission (lifecycle == `Accepted`, half-open interval containment) from scratch, written independently rather than calling `admission`.

`production_resolver_agrees_with_the_reference_oracle_across_random_scenarios` drives both through 200 generated scenarios from a seeded, dependency-free SplitMix64 PRNG (`SEED = 20260915`, this task's own evidence-capture date, chosen for reproducibility, not tuned to pass — adding `rand`/`proptest` was considered and rejected as unnecessary for one bounded generator, per the plan's "prove necessity before adding a runtime dependency" rule). Each scenario creates 1-4 decisions for a shared key with randomized `valid_from`/`valid_to` presence and randomized lifecycle (accepted / left draft / accepted-then-withdrawn), then probes both resolvers at 3 `as_of_valid` instants × every recorded-sequence cutoff the scenario passed through (up to 8 per scenario) — roughly 4,800 individual cross-checks across the full run, all agreeing (`raw/05-decision-state-module-tests-isolated.txt`).

## Tests added (13 new; all pass)

| Test | File | What it proves |
|---|---|---|
| `no_accepted_decision_when_nothing_exists_for_the_key` | decision_state.rs | `NoAcceptedDecision` with empty `considered` when the key has never been used |
| `no_accepted_decision_when_only_a_draft_exists` | decision_state.rs | A `Draft` decision is `considered` but not admitted; reason names its lifecycle |
| `current_set_when_exactly_one_decision_is_accepted` | decision_state.rs | The base `CurrentSet` case |
| `needs_review_when_two_independently_accepted_decisions_share_a_key_and_overlap` | decision_state.rs | Two independently-`accept_decision`-ed decisions, no supersession edge: `NeedsReview`, both admitted |
| `non_overlapping_valid_intervals_for_the_same_key_are_not_a_conflict` | decision_state.rs | Two accepted decisions with disjoint half-open intervals are never `NeedsReview`; the exact boundary instant resolves to the later one alone (half-open `[from,to)`) |
| `explicit_supersession_leaves_exactly_one_candidate_never_a_conflict` | decision_state.rs | `supersede_decision`'s lifecycle flip means the old decision is excluded (reason names `Superseded`), never counted toward `NeedsReview` |
| `withdrawal_reason_is_surfaced_as_negative_evidence` | decision_state.rs | A withdrawn decision's own `withdrawal_reason` text appears in `exclusion_reason` |
| `different_projects_with_the_same_key_never_collide` | decision_state.rs | Project scoping is exact; a shared key across two projects never cross-contaminates |
| `as_of_recorded_reconstructs_a_past_state_before_a_later_acceptance` | decision_state.rs | A recorded-sequence cutoff before an acceptance correctly reconstructs the pre-acceptance state |
| `as_of_recorded_out_of_range_is_refused` | decision_state.rs | A cutoff beyond the vault's actual current head is refused, not silently clamped |
| `resolution_ordering_is_stable_across_repeated_calls` | decision_state.rs | `considered` is deterministically `decision_id`-sorted, identically across repeated calls |
| `production_resolver_agrees_with_the_reference_oracle_across_random_scenarios` | decision_state.rs | 200-scenario, seeded property test cross-checking the production resolver against the independent reference oracle (see above) |
| `decision_state_reports_no_accepted_then_current_set_then_needs_review_through_the_cli` | cli.rs | End-to-end through the actual CLI dispatcher: `decision-state` reflects `NoAcceptedDecision` → `CurrentSet` → `NeedsReview` as real `decision-create`/`decision-accept` commands run |

## Commands executed (all re-run on this branch against `main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **295/295 pass**, 0 failed (262 lib [249 pre-existing + 13 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib decision_state::` | `raw/05-decision-state-module-tests-isolated.txt` | 12/12 pass (including the 200-scenario property test) |
| `cargo test --locked --lib cli::` | `raw/06-cli-module-tests-isolated.txt` | 11/11 pass |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings (one `manual_is_multiple_of` finding on the PRNG helper was fixed during development, before this final run — see "Failed attempts") |
| `git diff --stat 07694d5 -- src/` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `git diff --check` | `raw/07-diff-check.txt` | Exit 0, no whitespace errors |
| Environment capture | `raw/09-environment.txt` | Matches `reference-hardware.md`'s frozen development profile |
| Baseline head | `raw/08-baseline-head.txt` | `07694d5...` on both `HEAD` and `origin/main` before this task's own commit |

Raw artifact manifest: `raw/00-manifest.txt`.

## Failed attempts / exclusions

- An initial draft of `explicit_supersession_leaves_exactly_one_candidate_never_a_conflict` called `project::supersede_decision` through a `CanonicalWriter` (mirroring most other mutating calls in this test module), which does not compile — `supersede_decision`'s actual signature takes `&mut CanonicalStore` directly (it opens its own two separate `store.writer()` commits internally, per its own doc comment). Caught immediately by the compiler; fixed by calling it directly on `&mut store`.
- `cargo clippy` flagged the seeded PRNG helper's `self.next_u64() % 2 == 0` as `clippy::manual_is_multiple_of`; fixed to `self.next_u64().is_multiple_of(2)` before the final gate run recorded above. No test was skipped, deleted, or weakened to reach green.

## Performance gate

No dedicated timing harness was run. `resolve_decision_state` performs exactly one full-vault scan (`CanonicalStore::all_revisions()`), the same "S-scale full-scan" cost class every prior `list_project_*`/`list_current_objects`-based function in this crate already carries (`T02-01`-`T03-01`). M-scale measurement and a project-scoped index (avoiding the full-vault scan) are explicitly deferred by this task's own plan row to `T05-02`'s hardware-qualified pass, consistent with every prior `flake-v1` task's identical deferral.

## Durability gate

This task adds no new canonical write path — `resolve_decision_state` and the reference oracle are both entirely read-only against `CanonicalStore`'s already-durability-proven read APIs (`all_revisions`, `history`, `revisions_since`, `transaction_head`). No new fault-schedule regression test was needed because no new commit path exists to fault-inject.

## Cross-platform gate

Native development profile only (Windows 11/NTFS, matching `reference-hardware.md`), consistent with every prior `flake-v1` task. This module performs no filesystem operations at all beyond what `CanonicalStore`'s own already-cross-platform-scoped read APIs do internally. Remaining profiles are `T05-02`'s scope.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Apply canonical project, lifecycle, recorded cutoff and valid interval to all applicable records | Satisfied — `resolve_decision_state` filters by `project_id`, reconstructs `lifecycle`/`valid_from`/`valid_to` as of `as_of_recorded`, and checks `as_of_valid` against the half-open interval |
| Resolve only explicit accepted supersession; incomparable overlapping decisions with same key produce Conflict | Satisfied — no ranking ladder exists; `>=2` simultaneously-admitted decisions is unconditionally `NeedsReview` (`needs_review_when_two_independently_accepted_decisions_share_a_key_and_overlap`); an explicit supersession always leaves exactly one candidate (`explicit_supersession_leaves_exactly_one_candidate_never_a_conflict`) |
| Unknown/partial times cannot be invented as exact intervals | Satisfied — `None` `valid_from`/`valid_to` are treated as genuinely unbounded on that side, never coerced into a manufactured timestamp; §15's own I11 wording is quoted directly in `Decision`'s own doc comment (`T02-03`, unchanged by this task) |
| Keep user override reason and negative evidence in output | Satisfied — `withdrawal_reason` is folded into `exclusion_reason`; every non-admitted `considered` entry states why |
| Stable deterministic ordering across processes | Satisfied — `considered` is always sorted by `decision_id` before return (`resolution_ordering_is_stable_across_repeated_calls`) |
| S02/S05: index rank and external annotations cannot select truth or expand scope | Satisfied by construction — this module reads only `CanonicalStore`'s already-audited canonical read APIs, never the derived FTS index; there is no "annotation" input anywhere in its signature |
| I05-I08: no latest-wins, no model adjudication, no loss of basis/verification/lifecycle distinctions | Satisfied — no ranking ladder exists at all (see Architecture section); `ConsideredDecision.decision` carries the full reconstructed `Decision` including `basis`/`verification`, never collapsed into a single score |
| Return CurrentSet, NeedsReview or NoAcceptedDecision with reasons and source state, not misleading confidence | Satisfied — `DecisionOutcome` is exactly this three-way enum; `considered` is the "reasons and source state" |
| F09/F10/F20: invalid supersession rejected, unresolved conflict stays visible | Satisfied for this task's own scope — invalid-supersession rejection is `T02-03`'s already-proven `relation::create_relation`/`supersede_decision` behavior (unchanged here); an unresolved conflict is `NeedsReview`, never silently dropped |
| V02-V05/V09: valid/recorded time grid, future/backdated assertions, partial dates, cycles, ties, override and contradiction | Satisfied: time grid + partial dates (`non_overlapping_valid_intervals_for_the_same_key_are_not_a_conflict`'s boundary probe, and the property test's randomized partial `valid_from`/`valid_to`), override (`withdrawal_reason_is_surfaced_as_negative_evidence`), contradiction (`needs_review_...`); cycles are `T02-03`'s already-proven `relation.rs`/`supersede_decision` scope, not reintroduced here |
| Verification method: pure reference oracle separate from production resolver; record generated seed and cross-process equality | Satisfied — `tests::reference_oracle`, `SEED = 20260915` recorded here and in the module's own doc comment, 200 scenarios, exact agreement |

## Completion condition

Every acceptance clause above is satisfied. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (295/295). No sealed evidence altered; no force-push; no historical evidence file (`temporal.rs`/`memory.rs`) touched. `T03-02` is complete.

## Next frontier

`T03-03` — Build the owner resume view and explicit checkpoint. Depends on `T03-02` (this task).
