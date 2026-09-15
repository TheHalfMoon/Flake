# T03-03 evidence report — Build the owner resume view and explicit checkpoint

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §12/§15/§29, task `T03-03` — third task of `P03`, depends on `T03-02`
- **Baseline / tested source commit:** forked from `origin/main` `4325e50` (PR #82, `T03-02` merge; post-merge required checks all green — verified directly via `gh api .../commits/4325e50.../check-runs` before this task started)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only. Self-review before committing directly caught and fixed a real omission (see "Failed attempts" below) rather than only relying on hosted/CI review.

## Live-truth reverification performed before this task started

- `git fetch origin --prune` confirmed `main` at `4325e505f31dc5bb5ae47282ea390b1bd623732e` ("feat(t03-02): resolve explicit temporal state and disagreement deterministically (#82)"), matching `specs/CURRENT.md`'s `T03-02_STATUS=COMPLETE` and `ACTIVE_IMPLEMENTATION_UNIT=T03-03`. `gh pr view 82` confirmed `state=MERGED` with merge commit `4325e50`. `gh api repos/TheHalfMoon/Flake/commits/4325e50.../check-runs` confirmed all 8 named post-merge checks `completed`/`success`.
- Also corrected, in this task's own commit, `specs/CURRENT.md`'s `T03-02_MERGE_COMMIT` from `PENDING_PR_MERGE` to the verified real SHA.
- Read the full `T03-03` task-contract row (build plan lines 1019-1042) plus §12's "Resume" and "Conflict" rows verbatim (quoted directly in this task's own module doc comments) and §15's "Review checkpoint" entity row.
- Read `src/decision_state.rs` (`T03-02`) and `src/source_check.rs` (`T03-01`) in full — this task composes both, unchanged, rather than duplicating their logic.
- Read `docs/evidence/flake-v1/T03-02/REPORT.md` again as the evidence-report template this report follows.

## Scope actually touched

Two new modules — `src/checkpoint.rs` (an eighth `RecordPayload` kind, `ReviewCheckpoint`) and `src/resume.rs` (the composed, read-only resume view) — plus the same class of minimal, mechanical extension points a new canonical record kind structurally requires, already established by `T03-01`'s `SourceCheck` precedent: `src/project.rs` (`RecordPayload` enum arm, `kind_str`/`project_id_of`/`to_json`/`from_json`/`as_review_checkpoint`, the `list_project_records` exclusion arm, and one new `as_note` accessor `resume.rs` needed and no prior task had added), `src/export.rs` (`project_scope_object_ids` union), `src/import.rs` (`validate_self_contained`/`rewrite_references` arms, **and** a genuinely new merge pass — see "Failed attempts"), `src/index.rs` (one non-indexable-kind arm), `src/cli.rs` (four new subcommands + one CLI-level end-to-end test), `src/lib.rs` (module registration).

```text
src/checkpoint.rs | 426 ++++++++++++++++++++++++++++++++++++++++++++++++++++
src/cli.rs        | 296 ++++++++++++++++++++++++++++++++++++
src/export.rs     |  41 +++++
src/import.rs     |  51 ++++++-
src/index.rs      |   3 +-
src/lib.rs        |   2 +
src/project.rs    |  41 ++++-
src/resume.rs     | 437 ++++++++++++++++++++++++++++++++++++++++++++++++++++++
8 files changed, 1288 insertions(+), 9 deletions(-)
```
(`git diff --stat 4325e50 -- src/`, `raw/04-diff-stat.txt`, captured after `git add -A` staged both new files.)

## Architecture: `ReviewCheckpoint` (`Relation`'s exact shape) plus a purely-composed `resume()`

**`ReviewCheckpoint` is a new canonical kind, not a mutation of anything else.** One canonical object per project, found by a full scan filtering on `project_id` (the same pattern `source_check::list_project_source_checks` already establishes), committed through the existing `CommandTarget::CreateObject`/`UpdateObject` — no new `CommandTarget`, no schema change. `mark_reviewed_through` is the only path that creates or ordinarily advances it; `reset_checkpoint` is the only path that may move it backward, and requires a mandatory, non-empty `reason` — mirroring `withdraw_decision`'s identical "a transition away from the normal path requires a stated reason" rule (`T02-03`). Both mutating functions require the caller's `expected_revision_id` after the first mark (F21/I05), matching every other lifecycle-significant transition in this crate.

**§12's own sentence is enforced structurally, not by convention.** "A checkpoint is an explicit 'Mark reviewed through here' sequence marker; opening a page never implies the user read it." `resume()` (`src/resume.rs`) never writes a `ReviewCheckpoint` anywhere in its body — it only reads `checkpoint::current_checkpoint`. `resume_never_writes_a_checkpoint` (new test) proves this directly: two `resume()` calls in a row, still `None` afterward.

**One fixed snapshot per call.** `resume()` reads `CanonicalStore::transaction_head()` exactly once and threads `head_seq` through every `decision_state::resolve_decision_state` call via `as_of_recorded` — the same "no separate re-read per section" discipline `T03-02`'s own resolver already established for itself, applied here at the composition layer. `resume_output_is_stable_across_repeated_calls_at_the_same_snapshot` (new test) proves two calls at the same snapshot produce an identical `ResumeView` (`PartialEq`-derived, field by field).

**Ordering matches §18's priority groups exactly, reused rather than reinvented.** `ResumeView`'s own field order — `conflicts`, `stale_or_missing_evidence`, `current_decisions`, `next_actions`, `relevant_notes`, `changes_since_checkpoint` — is §18's identically-worded "unresolved conflict/staleness, accepted decisions, active actions, cited evidence, supporting notes; then stable recorded sequence and ID," applied to Resume rather than to a disclosure package. Every list within each group is itself sorted deterministically (`decision_key`/`object_id`, or `(recorded_seq, object_id)` for the raw changes log).

**Composition, not new computation.** `resume()` calls only already-audited read functions: `decision_state::resolve_decision_state` (`T03-02`, once per distinct `decision_key` in the project, separated into `conflicts` for `NeedsReview` and `current_decisions` for `CurrentSet`; `NoAcceptedDecision` keys are omitted — neither §12 row names them), `source_check::list_checks_for_source` (`T03-01`, latest check per `Source`; anything not `Match` is `stale_or_missing_evidence`), `project::list_project_records` (`T02-01`, filtered to non-terminal `Action`s for `next_actions`), and `CanonicalStore::revisions_since` (`T02-04`, for both "notes changed since checkpoint" and the raw changes log — one scan, reused for both, rather than two).

## Failed attempts / exclusions — a real gap found and fixed by self-review

**`import_selected_merge`'s explicit per-kind pass list initially omitted `review_checkpoint` entirely.** Unlike `RecordPayload`'s own enum (which the compiler forces exhaustive), `import_selected_merge` walks its own hand-written list of pass kinds (`["project", "source", "note", "decision"]`, then a dedicated `source_check` pass, then `action`, then `relation`) — nothing forces this list to name every kind, so a merge-imported package would have silently dropped every `ReviewCheckpoint` object with no error, no warning, and no test catching it (my first pass of writing tests only exercised `checkpoint.rs`/`resume.rs` in isolation, never a round-trip through `import_selected_merge`). This is exactly the class of defect `T02-07`'s own independent verifier exists to catch (I06/I10: "full declared state is readable"). Caught during self-review, before committing, by rereading `import_selected_merge`'s pass structure line by line against every `RecordPayload` variant rather than trusting that `cargo build`'s clean exit meant the merge path was complete (it does not — `import_selected_merge`'s pass list is data, not a match arm). Fixed by adding `"review_checkpoint"` to the first, simplest pass (it needs only `project_id` remapping, like `Note`/`Decision` — no revision-reference re-pinning, unlike `SourceCheck`'s `checked_revision_id`), and adding two new regression tests that would have failed loudly under the original code: `merge_review_checkpoint_is_carried_through_with_its_project_id_rewritten` (`import.rs`) and `project_export_includes_a_review_checkpoint` (`export.rs`, proving the project-scoped export itself does not omit the kind either).
- No test was skipped, deleted, or weakened to reach green.

## Tests added (19 new; all pass)

| Test | File | What it proves |
|---|---|---|
| `no_checkpoint_before_any_mark` | checkpoint.rs | No automatic checkpoint exists before an explicit mark |
| `first_mark_creates_a_checkpoint_without_expect` | checkpoint.rs | The first mark needs no prior revision to conflict with |
| `second_mark_without_expect_is_refused` | checkpoint.rs | F21/I05: a second mark requires `--expect` |
| `mark_backward_is_refused_without_an_explicit_reset` | checkpoint.rs | Monotonicity is enforced outside of `reset_checkpoint` |
| `reset_explicitly_moves_the_checkpoint_backward_with_a_reason` | checkpoint.rs | `reset_checkpoint` is the sole backward-move path; reason persists; history retains both revisions |
| `reset_requires_a_non_empty_reason` | checkpoint.rs | A reset without a stated reason is refused |
| `reset_before_any_mark_is_refused` | checkpoint.rs | Resetting something that was never marked is a clear error |
| `out_of_range_through_seq_is_refused` | checkpoint.rs | A cutoff beyond the vault's actual head is refused, not clamped |
| `stale_expect_is_refused` | checkpoint.rs | F21/I05: a stale `expected_revision_id` cannot silently clobber a newer mark |
| `empty_project_has_an_empty_resume_view` | resume.rs | Baseline: no conflicts/evidence/decisions/actions; project creation itself is a "change" pre-checkpoint |
| `current_decisions_and_conflicts_are_separated` | resume.rs | One clean key -> `current_decisions`; one disputed key -> `conflicts` |
| `stale_evidence_lists_only_non_match_latest_checks` | resume.rs | A `Match` source is invisible; a `Changed` source appears |
| `next_actions_excludes_terminal_states` | resume.rs | `Done`/`Cancelled` actions are never "next" |
| `checkpoint_narrows_relevant_notes_and_changes` | resume.rs | Only post-checkpoint notes are "relevant"; every change entry is after the checkpoint |
| `resume_never_writes_a_checkpoint` | resume.rs | §12: opening/computing resume never itself marks reviewed |
| `resume_output_is_stable_across_repeated_calls_at_the_same_snapshot` | resume.rs | Deterministic, `PartialEq`-verified repeat output |
| `resume_and_checkpoint_lifecycle_works_through_the_cli` | cli.rs | End-to-end: no checkpoint -> first mark -> relevant note -> second mark needs `--expect` -> explicit reset with reason -> full history |
| `merge_review_checkpoint_is_carried_through_with_its_project_id_rewritten` | import.rs | Regression for the gap above: a checkpoint survives `import_selected_merge` with `project_id` rewritten |
| `project_export_includes_a_review_checkpoint` | export.rs | A project-scoped export does not silently omit the new kind |

## Commands executed (all re-run on this branch against `main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **314/314 pass**, 0 failed (281 lib [262 pre-existing + 19 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib checkpoint::` | `raw/05-checkpoint-module-tests-isolated.txt` | 9/9 pass |
| `cargo test --locked --lib resume::` | `raw/06-resume-module-tests-isolated.txt` | 7/7 pass |
| `cargo test --locked --lib cli::` | `raw/07-cli-module-tests-isolated.txt` | 12/12 pass |
| `cargo test --locked --lib export::` | `raw/11-export-module-tests-isolated.txt` | 10/10 pass |
| `cargo test --locked --lib import::` | `raw/12-import-module-tests-isolated.txt` | 9/9 pass |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat 4325e50 -- src/` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `git diff --check` | `raw/10-diff-check.txt` | Exit 0, no whitespace errors |
| Environment capture | `raw/09-environment.txt` | Matches `reference-hardware.md`'s frozen development profile |
| Baseline head | `raw/08-baseline-head.txt` | `4325e50...` on both `HEAD` and `origin/main` before this task's own commit |

Raw artifact manifest: `raw/00-manifest.txt`.

## Performance gate

No dedicated timing harness was run. `resume()` performs one `list_project_records` scan, one `list_project_sources` scan (plus one `list_checks_for_source` scan per source), one `revisions_since` scan, and one `resolve_decision_state` call per distinct `decision_key` (each itself a full `all_revisions` scan, per `T03-02`'s own evidence) — the same "S-scale full-scan" cost class every prior `flake-v1` composition function carries. M-scale measurement (this task's own "M resume p95" clause) is explicitly deferred to `T05-02`'s hardware-qualified pass, consistent with every prior task's identical deferral. "No full-vault scan hidden in project view" is satisfied: every scan `resume()` performs is itself already project-scoped or filtered by `project_id`/`source_id` before use, not a raw full-vault read masquerading as project-scoped.

## Durability gate

`ReviewCheckpoint`'s only mutating writes (`mark_reviewed_through`'s `CreateObject`/`UpdateObject`, `reset_checkpoint`'s `UpdateObject`) are ordinary `CanonicalWriter::commit` calls — the exact atomic primitive `T01-03`/`T01-07`'s fault-schedule matrix already proved. No new fault-schedule regression test was added for this task's own new call site (unlike `T03-01`'s `SourceCheck`), since `checkpoint.rs`'s commit path is structurally identical to `project.rs`'s own already-audited `commit_create`/`commit_update` helpers (directly reused, not reimplemented) rather than a new bespoke commit sequence. `resume()` itself performs no writes at all.

## Cross-platform gate

Native development profile only (Windows 11/NTFS, matching `reference-hardware.md`), consistent with every prior `flake-v1` task. Neither `checkpoint.rs` nor `resume.rs` performs any filesystem operation beyond what `CanonicalStore`'s own already-cross-platform-scoped read/write APIs do internally. Remaining profiles are `T05-02`'s scope.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Scripted interruption/resumption cases expose all planted changes/conflicts and correct next actions with no hidden prerequisites | Satisfied — `current_decisions_and_conflicts_are_separated`, `stale_evidence_lists_only_non_match_latest_checks`, `next_actions_excludes_terminal_states`, `checkpoint_narrows_relevant_notes_and_changes`, and the CLI end-to-end test each plant a specific change and assert it surfaces |
| Compose §12 ordering from canonical eligibility/resolution and bounded search support | Satisfied — `ResumeView`'s field order matches §18's priority groups exactly; `resume()` composes `T03-02`'s resolver and `T03-01`'s check history rather than reimplementing eligibility |
| Show changes since explicit reviewed-through sequence, current head, missing/changed evidence, unresolved conflicts and next actions | Satisfied — every one is its own `ResumeView` field |
| Mark reviewed only on explicit owner command; opening/searching is not a checkpoint | Satisfied — `resume_never_writes_a_checkpoint` |
| Stable resume output at the same snapshot | Satisfied — `resume_output_is_stable_across_repeated_calls_at_the_same_snapshot`; one fixed `head_seq` threaded through the whole call |
| S05/S06: no automatic decision acceptance, source promotion or silent checkpoint write | Satisfied by construction — `resume()` never calls any mutating function anywhere in its body |
| I05-I08: checkpoint sequence bound to project/history, conflicts precede reassuring summaries | Satisfied — `reviewed_through_seq` lives on the project-scoped `ReviewCheckpoint`; `conflicts`/`stale_or_missing_evidence` are `ResumeView`'s first two fields |
| V03-V05/V09/V13: checkpoint reset, stale index, deleted evidence, action reopen, deterministic repeat snapshot | Satisfied: checkpoint reset (`reset_explicitly_moves_the_checkpoint_backward_with_a_reason`), stale evidence (`stale_evidence_lists_only_non_match_latest_checks`), deterministic repeat (`resume_output_is_stable_across_repeated_calls_at_the_same_snapshot`); "deleted evidence"/"action reopen" are `T02-02`/`T02-03`'s own already-proven lifecycle behaviors, surfaced generically through `resume()`'s existing composition rather than re-proven here |
| Verification method: actual CLI workflow against saved/mutated external-source scenarios; oracle checks every visible state | Satisfied — `resume_and_checkpoint_lifecycle_works_through_the_cli` drives the real CLI dispatcher end to end and checks every `ResumeView`/`ReviewCheckpoint` field the workflow touches |

## Completion condition

Every acceptance clause above is satisfied. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (314/314). One real defect was found and fixed by self-review before commit (the `import_selected_merge` omission above) — preserved here rather than silently folded away, per the founder's own "preserve the failure evidence" instruction. No sealed evidence altered; no force-push; no historical evidence file touched. `T03-03` is complete.

## Next frontier

`T03-04` — Issue scoped grants and persist exact disclosure receipts. Depends on `T03-03` (this task).
