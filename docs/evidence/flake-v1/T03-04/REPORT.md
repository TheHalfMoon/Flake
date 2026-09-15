# T03-04 evidence report — Issue scoped grants and persist exact disclosure receipts

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §15/§16/§18/§20, task `T03-04` — fourth task of `P03`, depends on `T03-03`
- **Baseline / tested source commit:** forked from `origin/main` `615e5f7` (PR #83, `T03-03` merge; post-merge required checks all green — verified directly via `gh api .../commits/615e5f7.../check-runs` before this task started)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only. Self-review before committing caught and fixed two real defects (a budget-accounting bug and a CRLF line-ending corruption) — both documented below rather than silently folded away.

## Live-truth reverification performed before this task started

- `git fetch origin --prune` confirmed `main` at `615e5f75e19437e49cc7883000fa4825ff6badc1` ("feat(t03-03): build the owner resume view and explicit checkpoint (#83)"), matching `specs/CURRENT.md`'s `T03-03_STATUS=COMPLETE` and `ACTIVE_IMPLEMENTATION_UNIT=T03-04`. `gh pr view 83` confirmed `state=MERGED`. `gh api repos/TheHalfMoon/Flake/commits/615e5f7.../check-runs` confirmed all 8 named post-merge checks `completed`/`success`.
- Also corrected, in this task's own commit, `specs/CURRENT.md`'s `T03-03_MERGE_COMMIT` from `PENDING_PR_MERGE` to the verified real SHA.
- Read the full `T03-04` task-contract row (build plan lines 1043-1067) plus §15's "Export grant"/"Disclosure receipt" entity rows, §16's shareable-package exclusion rule, and §18's full "Owner flow" and package-compilation paragraphs (priority groups, 256 KiB cap, truncation/omission rules).
- Read `src/context.rs` (historical Phase T "Context Compiler", immutable evidence per `AGENTS.md` §3) in full to understand the exact algorithm shape it establishes (budget atomicity; the manifest is built from what was actually emitted) — confirmed it is not reused or edited, consistent with this crate's established non-reuse precedent.
- Read `src/decision_state.rs` (`T03-02`) and `src/source_check.rs`/`src/relation.rs` (`T03-01`/`T02-03`) in full — this task composes all three unchanged rather than duplicating their logic.
- Read `docs/evidence/flake-v1/T03-03/REPORT.md` again as the evidence-report template this report follows.

## Scope actually touched

Two new modules — `src/grant.rs` (a ninth `RecordPayload` kind, `ExportGrant`) and `src/disclosure.rs` (a tenth kind, `DisclosureReceipt`, plus the disclosure compiler) — plus the same class of minimal, mechanical extension points every new canonical record kind structurally requires: `src/project.rs` (two `RecordPayload` enum arms, `kind_str`/`project_id_of`/`to_json`/`from_json`/`as_export_grant`/`as_disclosure_receipt`, the `list_project_records` exclusion arms), `src/export.rs` (`project_scope_object_ids` deliberately does **not** gain a union for either new kind — see "Architecture" below), `src/import.rs` (`validate_self_contained` explicitly refuses either kind; `rewrite_references` gains exhaustive-but-unreachable arms), `src/index.rs` (two non-indexable-kind arms), `src/cli.rs` (four new subcommands + one CLI-level end-to-end test), `src/lib.rs` (module registration).

```text
src/cli.rs        |  287 +++++++++++++++
src/disclosure.rs | 1045 +++++++++++++++++++++++++++++++++++++++++++++++++++++
src/export.rs     |   62 ++++
src/grant.rs      |  411 +++++++++++++++++++++
src/import.rs     |  138 ++++++-
src/index.rs      |    4 +-
src/lib.rs        |    2 +
src/project.rs    |   55 ++-
8 files changed, 1991 insertions(+), 13 deletions(-)
```
(`git diff --stat 615e5f7 -- src/`, `raw/04-diff-stat.txt`, captured after `git add -A` staged both new files.)

## Architecture

**`ExportGrant` (ninth kind, `Relation`'s shape).** One canonical object per grant, committed through the existing `CommandTarget::CreateObject`/`UpdateObject`. `issue_grant` is create-only — a grant's scope is fixed at issuance (§18: "any change before commit requires a new preview/confirmation; it cannot silently expand the package," applied here as "a grant cannot silently expand either; issue a new one"). `revoke_grant` is the sole mutating transition, and refuses to re-revoke an already-revoked grant. `byte_budget` is capped at `limits::MAX_PACKAGE_BYTES` (256 KiB, an existing crate-wide constant, reused rather than redefined); TTL is capped at seven days (§18), with no floor enforced at issuance (a `byte_budget` too small even for an empty package is instead a clear, honest *compile-time* error — see below — rather than a heuristic issuance-time guess).

**`DisclosureReceipt` (tenth kind, genuinely immutable).** No function anywhere in this crate ever issues an `UpdateObject` against one — `compile_disclosure_package` only ever `CreateObject`s it, once, with `command_id = request_id` (the caller's own idempotency key), directly reusing `CanonicalWriter::commit`'s already-proven duplicate-command-id reconciliation (`T01-03`) rather than inventing a second one.

**Receipt-before-emission (§18, D1/D5).** `compile_disclosure_package` commits the receipt *before* returning wire bytes to its caller. There is no code path that returns bytes without a successfully committed receipt immediately before it — "failed receipt persistence emits no package" holds by construction, not by a separate check.

**Wire shape: a header line plus one JSON object per item, never one wrapped document.** See "Failed attempts" below for why this replaced an initial nested-document design. `rejected` items are never written to the wire at all — they live only on the receipt (unbounded, separately persisted), mirroring `context.rs`'s own already-established separation between its budget-capped `wire: String` and its unbounded `Manifest`/`omissions`.

**§16 enforced at two independent layers, not just by omission.** `export.rs`'s `project_scope_object_ids` deliberately gains no union arm for either new kind (§16: "shareable project packages omit local-only locators, active grants and excluded sensitive fields" — a grant/receipt is local disclosure-*governance* state, not project content). `import_selected_merge`'s `validate_self_contained` additionally, explicitly refuses any package containing either kind outright — defense in depth against a hand-crafted or corrupted package, not merely trusting that the export side never produces one. A full (vault-wide) export and `import_full_restore` are both unaffected: a full restore is a same-owner backup, not a shareable disclosure, so grants/receipts survive it unchanged like every other object.

**Decision disclosure reuses `T03-02` unchanged.** For each distinct `decision_key` in the project, `decision_state::resolve_decision_state` determines whether its candidates belong to the `conflicts` group (`NeedsReview`) or the `current_decisions` group (`CurrentSet`); a `NoAcceptedDecision` key is never offered to the compiler at all, not even as a rejection (a `Draft`/solely-`Withdrawn` decision is not yet an owner assertion an external reader should be told about — §17).

**`Source` disclosure is metadata only, never `capture.bytes_hex`.** A `Source`'s canonical payload embeds its exact captured bytes inline; this module's rendered content for a `Source` is its label, kind, digest and byte length only. A 256 KiB UTF-8-text package was never going to carry arbitrary binary evidence, and keeping raw bytes out is a load-bearing choice, documented in the module's own doc comment, not an oversight.

**`Relation`s are disclosed last, and only when both endpoints were themselves disclosed.** Computed as a distinct pass after every non-`Relation` item is already decided — never reveal a relation naming an object the recipient cannot otherwise see, even if the relation object itself is nominally in the grant's scope.

## Failed attempts / exclusions — two real defects found and fixed by self-review

**1. Nested-document wire format broke exact budget accounting.** The first implementation serialized one `DisclosureWire { header fields..., items: Vec<WireItem>, rejections: Vec<WireRejection> }` as a single JSON document, accumulating `used += <length of each WireItem serialized alone>` as items were added. This undercounts by exactly the wrapping envelope's own byte cost (request_id/grant_id/project_id/principal_label/timestamps/version fields, ~250 bytes typical) — three tests (`tiny_budget_omits_items_with_a_budget_reason_never_a_partial_leak`, `truncation_never_exceeds_the_byte_budget`, and indirectly `repeated_identical_request_id_yields_the_same_receipt`) failed immediately on first run with `"internal error: compiled package (382 bytes) exceeds its own budget (32 bytes)"` — the very safety check this task exists to guarantee catching itself, honestly, before any bytes left the function. Fixed by switching to a flat, line-oriented wire (header line + one JSON object per item line, each measured exactly including its own trailing newline) and moving `rejected` items out of the wire entirely (onto the receipt only) — eliminating the shared-envelope distortion structurally rather than patching the accounting. A new test, `a_budget_too_small_for_even_the_header_is_a_clear_top_level_error`, was added to cover the now-explicit "budget cannot fit even an empty package" case.
**2. A Windows-Python text-mode write silently converted `src/project.rs` from LF to CRLF.** A `python3` heredoc script used earlier in this task (to add the `ExportGrant`/`DisclosureReceipt` `RecordPayload` match arms across several locations at once) opened the file in default text mode for both read and write; on this Windows development host, Python's text-mode write translates every `\n` to `\r\n`. `git diff --check`, run as part of this task's own pre-commit gate, flagged the entire file (2520 insertions / 2471 deletions for what was actually a ~55-line change) as "trailing whitespace" on every line. Caught before staging by running `git diff --check` and noticing the diff size was wildly disproportionate to the actual edit; fixed by a binary-safe `\r\n` → `\n` pass restricted to that one file, re-verified with `git diff --stat` (55 lines, matching the real change) and `git diff --check` (clean) before proceeding.
- No test was skipped, deleted, or weakened to reach green in either case.

## Tests added (23 new; all pass)

| Test | File | What it proves |
|---|---|---|
| `issue_grant_defaults_to_full_disclosable_scope` | grant.rs | Empty `allowed_kinds` means every disclosable kind is covered |
| `covers_respects_kind_allowlist_id_allowlist_and_exclusions` | grant.rs | Privacy exclusion wins over an otherwise-matching kind/ID allowlist |
| `unrecognized_kind_is_refused` | grant.rs | A grant cannot name a non-disclosable kind (e.g. `project`) |
| `byte_budget_out_of_range_is_refused` | grant.rs | Zero and above-`MAX_PACKAGE_BYTES` are both refused at issuance |
| `ttl_beyond_seven_days_is_refused` | grant.rs | §18's hard TTL ceiling |
| `revoke_makes_the_grant_unusable_and_cannot_be_repeated` | grant.rs | Revocation is one-way; a stale re-revoke is refused, not a no-op |
| `expired_grant_is_unusable` | grant.rs | Expiry is checked independently of revocation |
| `a_clean_note_and_decision_are_fully_disclosed` | disclosure.rs | Baseline: both selected, none rejected |
| `draft_only_decision_key_is_never_a_candidate` | disclosure.rs | §17: a draft is never even offered, let alone disclosed |
| `privacy_exclusion_is_rejected_with_a_clear_reason_never_disclosed` | disclosure.rs | Excluded content's actual text never reaches the wire |
| `mixed_project_fixture_never_discloses_the_other_project` | disclosure.rs | S02: a second project's content never even enters the candidate pool |
| `kind_restricted_grant_excludes_other_kinds` | disclosure.rs | A kind not in `allowed_kinds` is rejected, its content absent from the wire |
| `tiny_budget_omits_items_with_a_budget_reason_never_a_partial_leak` | disclosure.rs | An item whose envelope cannot fit is omitted whole, never partially |
| `truncation_never_exceeds_the_byte_budget` | disclosure.rs | A long note is truncated, never exceeding the exact byte cap |
| `relation_with_an_undisclosed_endpoint_is_rejected_not_leaked` | disclosure.rs | A relation naming an undisclosed object is rejected, not emitted |
| `a_budget_too_small_for_even_the_header_is_a_clear_top_level_error` | disclosure.rs | Regression for defect 1 above |
| `revoked_grant_refuses_compilation` | disclosure.rs | A revoked grant cannot compile a package |
| `repeated_identical_request_id_yields_the_same_receipt` | disclosure.rs | Deterministic compilation at an unchanged snapshot |
| `full_receipt_replay_verifies_exact_bytes` | disclosure.rs | The persisted receipt's digest/byte-count match the actual emitted wire, reread independently |
| `grant_and_package_lifecycle_works_through_the_cli` | cli.rs | End-to-end: issue → preview → export → stage/verify → refuse-overwrite → revoke → refuse-after-revoke, through the real CLI dispatcher |
| `selected_merge_refuses_a_package_containing_a_grant` | import.rs | §16 defense-in-depth: a hand-crafted/full-vault package containing a grant is refused by merge-import |
| `full_restore_preserves_grants_unlike_selected_merge` | import.rs | A full-restore (same-owner backup) is unaffected by the merge-only exclusion |
| `project_export_never_includes_a_grant_or_receipt` | export.rs | §16 at the export layer: neither kind ever appears in a project-scoped package, even though both exist in the vault |

## Commands executed (all re-run on this branch against `main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **337/337 pass**, 0 failed (304 lib [281 pre-existing + 23 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib grant::` | `raw/05-grant-module-tests-isolated.txt` | 7/7 pass |
| `cargo test --locked --lib disclosure::` | `raw/06-disclosure-module-tests-isolated.txt` | 12/12 pass |
| `cargo test --locked --lib cli::` | `raw/07-cli-module-tests-isolated.txt` | 13/13 pass |
| `cargo test --locked --lib export::` | `raw/11-export-module-tests-isolated.txt` | 11/11 pass |
| `cargo test --locked --lib import::` | `raw/12-import-module-tests-isolated.txt` | 11/11 pass |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat 615e5f7 -- src/` | `raw/04-diff-stat.txt` | Scope confirmed as listed above (post-LF-fix) |
| `git diff --check` | `raw/10-diff-check.txt` | Exit 0, no whitespace errors (post-LF-fix) |
| Environment capture | `raw/09-environment.txt` | Matches `reference-hardware.md`'s frozen development profile |
| Baseline head | `raw/08-baseline-head.txt` | `615e5f7...` on both `HEAD` and `origin/main` before this task's own commit |

Raw artifact manifest: `raw/00-manifest.txt`.

## Performance gate

No dedicated timing harness was run. `compile_disclosure_package` performs one `gather_candidates` pass (itself one `list_project_records` scan plus one `resolve_decision_state` call per distinct decision key, one `list_project_sources`+`list_checks_for_source`-per-source pass, one `list_project_relations` scan) — the same "S-scale full-scan" cost class every prior composition function in this crate carries (`T03-02`/`T03-03`'s own identical deferral). M-scale measurement and RSS bounds (this task's own "256 KiB cap and M resume/export latency/RSS" clause) are explicitly deferred to `T05-02`'s hardware-qualified pass. The 256 KiB cap itself is enforced exactly, not approximately — every line's actual rendered byte length (including its own trailing newline) is measured before being counted against the budget, and a final `wire.len() > budget` check exists as a last-resort structural assertion (see "Failed attempts" for why it is not merely decorative).

## Durability gate

The receipt's only write (`CommandTarget::CreateObject`) is an ordinary `CanonicalWriter::commit` call — the exact atomic primitive `T01-03`/`T01-07`'s fault-schedule matrix already proved. `issue_grant`/`revoke_grant` reuse `project::commit_create`/`commit_update` directly (the same already-audited helpers every other kind uses), not a new bespoke commit sequence, so no new fault-schedule regression test was added for this task's own new call sites, consistent with `T03-03`'s identical reasoning for `checkpoint.rs`. `preview_disclosure_package` performs no writes at all.

## Cross-platform gate

Native development profile only (Windows 11/NTFS, matching `reference-hardware.md`), consistent with every prior `flake-v1` task. Neither `grant.rs` nor `disclosure.rs` performs any filesystem operation beyond what `CanonicalStore`'s own already-cross-platform-scoped read/write APIs do internally; `package-export`'s staged-file write/reread-verify uses ordinary `std::fs::write`/`std::fs::read`, the same primitives `backup.rs`/`export.rs` already use. Remaining profiles are `T05-02`'s scope.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Mixed-project fixtures never disclose excluded material | Satisfied — `mixed_project_fixture_never_discloses_the_other_project`, `privacy_exclusion_is_rejected_with_a_clear_reason_never_disclosed`, `kind_restricted_grant_excludes_other_kinds` |
| Repeated identical request bytes match | Satisfied — `repeated_identical_request_id_yields_the_same_receipt` (via `preview_disclosure_package`, which does not itself advance vault state — see the test's own doc comment for why compiling twice through the committing path is not the right way to test this) |
| Revoked/expired grant fails | Satisfied — `revoked_grant_refuses_compilation`, `grant::expired_grant_is_unusable` |
| Full receipt replay verifies exact bytes | Satisfied — `full_receipt_replay_verifies_exact_bytes` |
| §18 owner-issued grants, expiry/revocation, type/ID restrictions and privacy exclusions | Satisfied — `grant.rs` in full |
| All item types/referenced metadata use canonical checks | Satisfied — every candidate is read through already-audited canonical read functions (`list_project_records`/`list_project_sources`/`list_project_relations`/`resolve_decision_state`/`list_checks_for_source`), never a raw/unchecked read |
| Persist receipt and exact wire bytes before emission; deterministic byte accounting includes envelope metadata | Satisfied — receipt-before-emission by construction; the header line's own bytes are reserved before any item is considered (see "Failed attempts") |
| Stage/verify package output | Satisfied — `package-export`'s CLI handler writes then rereads the file, refusing success if the reread bytes differ |
| Reconcile lost result through command ID | Satisfied — `request_id` is used directly as `CommandInput::command_id`, reusing `T01-03`'s already-proven mechanism |
| No raw agent read endpoint or imported live grant | Satisfied — the only compilation entry points are `compile_disclosure_package`/`preview_disclosure_package`, both requiring a live, owner-issued, currently-usable grant already present in *this* vault; `import.rs` never re-derives grant authority from an imported package (`selected_merge_refuses_a_package_containing_a_grant`) |
| S01/S02/S07: zero disallowed body/ID/path bytes, no source content as authority; do not expose grant bearer material | Satisfied — `Source` content is metadata-only (never `bytes_hex`); a grant is never itself disclosed as content; S02 by construction (project-scoped reads only) |
| I04/I05/I08: exact historical receipt binding to snapshot/policy/grant/revisions, no last-manifest overwrite | Satisfied — the receipt pins `grant_id`+`grant_revision_id`+`as_of_recorded`+`policy_version`+`compiler_version`; it is never updated after creation (no "last manifest" concept exists to overwrite) |
| V02-V07/V09/V13: all record types, metadata-only leaks, envelope budget boundaries, expiry/revocation, crash-before/after receipt and emission | Satisfied for all record types (`a_clean_note_and_decision_are_fully_disclosed` plus per-kind rejection tests), metadata-only (`Source` content tests), envelope budget boundaries (`tiny_budget_...`, `truncation_...`, `a_budget_too_small_...`), expiry/revocation (`grant.rs`); crash-before/after receipt is `CanonicalWriter::commit`'s own already-proven D1 guarantee, reused not re-tested |

## Completion condition

Every acceptance clause above is satisfied. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (337/337). Two real defects were found and fixed by self-review before commit (both documented above under "Failed attempts") — preserved here rather than silently folded away. No sealed evidence altered; no force-push; no historical evidence file (`context.rs`) touched. `T03-04` is complete.

## Next frontier

`T03-05` — Review and admit bounded agent proposals. Depends on `T03-04` (this task).
