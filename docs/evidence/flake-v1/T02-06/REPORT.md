# T02-06 evidence report — Validate and import portable packages into a new vault

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §12/§15/§20/§22/§27-29, task `T02-06` — sixth task of `P02`
- **Baseline / tested source commit:** forked from `origin/main` `a92dce6` (PR #78, `T02-05` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy`. All checks below were executed locally on the development host recorded in `specs/002-post-r1-canonical-core-convergence/corrective-t01/reference-hardware.md`.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`, `git log --oneline --decorate -5`, `gh pr view 78 --json state,mergedAt,mergeCommit` confirmed `main` at `a92dce6` ("feat(t02-05): export complete owned state with a verified manifest (#78)"), merged, matching `specs/CURRENT.md`'s `T02-05_STATUS=COMPLETE`.
- `gh api repos/TheHalfMoon/Flake/commits/a92dce6.../check-runs` confirmed all 8 named checks completed `success` on the merge commit.
- Read the full `T02-06` task-contract row (build plan lines 923-945) plus the plan sections it names: §12, §15 (the `Export/backup manifest`/restore-and-fork paragraphs), §20 (S01-S07/S10), §22 (F05/F07/F10/F15/F18), §27, §29 (V03/V05-V10/V13).
- Read `docs/formats/portable-export-v1.md` and `src/export.rs` (`T02-05`) in full before writing any code — this task's entire input format. Read `src/canonical.rs`'s `commit_with_fault`/`CommandTarget` implementation directly to confirm exactly what `ImportObject`/`UpdateObject` do and do not let a caller preserve (object_id yes, revision_id/recorded_at no — see "Architecture" below), rather than assuming.
- Read `src/migration.rs` (`T01-06`) as the closest existing precedent for "preserve declared identity on import" (`ImportObject`'s own doc comment names `T01-06` as the one deliberate, narrowly-scoped exception to Core-assigned identity) and for its own "omit with reason, refuse ambiguous complete" admission discipline.

## Scope actually touched

One new module (`src/import.rs`), plus `src/cli.rs` (three new subcommands), `src/lib.rs` (module registration), one visibility widening in `src/export.rs` (`compute_integrity_root` from private to `pub(crate)`, reused rather than duplicated — export and import are direct counterparts for the same format, unlike the deliberate non-reuse between unrelated historical modules elsewhere in this crate), and one addendum section in `docs/formats/portable-export-v1.md`. No `Cargo.toml`/`Cargo.lock` change, no `canonical.rs` change — every canonical operation this task needs (`ImportObject`/`CreateObject`/`UpdateObject`, `read_current`) already existed.

```text
src/cli.rs                          |  147 +++
src/export.rs                        |    2 +-
src/lib.rs                           |    1 +
src/import.rs (new)                  |  991 +++++++++++++++++++++++
docs/formats/portable-export-v1.md   |   35 +
```
(`git diff --stat main -- src/`, `raw/04-diff-stat.txt`; `src/import.rs` is untracked against `main` so `git diff --stat` does not list it — its line count is recorded separately in the same file.)

## Architecture: two import modes, one shared validation path, built entirely on already-audited canonical primitives

**Full restore** (`import_full_restore`) targets a brand-new, empty canonical store. Every object's `object_id` is preserved exactly via `CommandTarget::ImportObject`'s first revision — the identical "declared identity preserved" exception `T01-06`'s legacy migration already established, deliberately reused rather than re-invented. Subsequent revisions of the same object replay as ordinary `UpdateObject` commands, chained against the destination's own freshly-minted revision IDs.

**Selected merge** (`import_selected_merge`) targets an already-open, possibly non-empty destination. Every object gets a brand-new, Core-assigned `object_id` (ordinary `CreateObject`), and every internal cross-reference inside the imported payloads — `project_id` on `Note`/`Action`/`Decision`/`Source`, `Action::dependency_ids`, a `Relation`'s `from_object_id`/`to_object_id` — is rewritten to the new destination identities before commit, per §15: "it rewrites only the newly admitted references... does not splice a foreign chain into the local transaction chain." The complete source→destination `id_map` is returned to the caller.

**A real, honestly-documented fidelity boundary: `revision_id`/`recorded_seq`/`recorded_at` are never preserved byte-for-byte.** Reading `canonical.rs`'s own `commit_with_fault` implementation directly (not assuming) confirmed `revision_id = uuid::Uuid::now_v7().to_string()` is unconditional for every `CommandTarget` variant, `ImportObject` included — there is no lower, privileged bulk-loader that could inject historical envelope metadata without inventing exactly the kind of new canonical mechanism `AGENTS.md` §6/§7 forbids. `object_id` (I03's actual load-bearing external identity) and every revision's exact payload bytes, replayed in original order, are what this task can honestly promise — and does. This is recorded explicitly in both the module doc and the format-doc addendum, not glossed over.

**Idempotent retry, reusing existing machinery, zero new schema.** Every replayed command's `command_id` is the *source* revision's own `revision_id` — already a UUIDv7, already globally distinct. A process interruption and retry of `import_full_restore` therefore reconciles through `CanonicalWriter::commit`'s own already-audited idempotency check (`T01-03`/`T01-07`), rather than needing a new import-provenance ledger. `import_selected_merge` cannot offer the same cross-run guarantee (each run mints genuinely new destination identities by design) — recorded as a deliberate, narrower scope rather than a silent gap, since a durable "have I merged this before" ledger would itself be new schema this task's own forbidden-scope clause excludes.

**Multi-pass ordering resolves forward references without inventing a second staging mechanism.** `import_selected_merge` processes kinds in a fixed order — `Project` → `Source` → `Note`/`Decision` → `Action` (two subpasses of its own) → `Relation` — so every cross-reference a later pass rewrites already has a resolved destination identity. `Action`'s own subpass handles the one genuine forward-reference case in the current data model (a sibling action, referenced by `dependency_ids`, possibly not yet imported): subpass 1 creates every action with `dependency_ids` temporarily cleared, building the complete action `id_map`; subpass 2 issues one additional `UpdateObject` restoring the rewritten dependency list. `Relation` goes last and additionally re-pins `from_revision_id`/`to_revision_id` to each endpoint's actual destination current-revision at import time (the original pinned revision cannot survive replay for the reason above).

## Security boundary — what this task actually enforces (S01-S07/S10)

- **No archive extraction, structurally.** There is no archive/zip parsing anywhere in this module — the source is read as an ordinary directory, and only the exact paths `export-manifest.json` declares are ever opened. Nothing under the source directory is walked or trusted beyond what the manifest names and this module independently re-hashes.
- **Every declared member is independently re-verified before any canonical write.** `read_and_validate_package` re-hashes every manifest-declared file against its declared length and SHA-256, recomputes the manifest's own `integrity_root` from scratch and compares it, and separately verifies every revision file's own `payload_sha256` against its own `payload_raw` — three independent layers, any one of which catches a different class of tampering. `full_restore_is_all_or_nothing_on_a_corrupt_member` and `tampered_manifest_integrity_root_is_refused` each isolate one specific layer.
- **All-or-nothing admission.** `import_full_restore` runs the entire validation pass *before* creating the destination store at all — a rejected import leaves no destination vault whatsoever (`full_restore_is_all_or_nothing_on_a_corrupt_member` asserts `dest_root` has no `.fehrest` after a refused import).
- **No imported grants can disclose content.** Nothing in the current canonical record model is a disclosure grant (§15's `Export grant` entity does not exist in this codebase yet) — vacuously but honestly satisfied, recorded explicitly rather than left implicit, matching `T02-05`'s identical treatment of the same gap.
- **Self-containment is verified for merge imports.** `validate_self_contained` refuses (before any write) a package whose declared references (`project_id`, `dependency_ids`, relation endpoints) point outside the package itself.

## Tests added (8 new; all pass)

| Test | File | What it proves |
|---|---|---|
| `full_restore_preserves_object_ids_and_all_payload_bytes` | import.rs | `object_id` and every current payload (including one referencing another unchanged `object_id` via `dependency_ids`) are byte-identical after restore |
| `full_restore_refuses_an_already_existing_destination` | import.rs | No-clobber refusal on a second restore attempt at the same destination |
| `full_restore_is_all_or_nothing_on_a_corrupt_member` | import.rs | A single tampered revision file refuses the entire import; zero destination side effects |
| `selected_merge_assigns_new_ids_and_rewrites_references` | import.rs | New destination identities are minted (never source IDs reused); `project_id` and a sibling-action `dependency_ids` reference are both correctly rewritten through the id_map; pre-existing destination content is untouched |
| `merge_relation_endpoints_are_rewritten_and_repinned` | import.rs | A `Relation`'s `from_object_id`/`to_object_id` are rewritten and its `from_revision_id`/`to_revision_id` are re-pinned to the actual destination current revisions |
| `preview_reports_scope_without_writing_anything` | import.rs | §12 "scope shown before writing" |
| `tampered_manifest_integrity_root_is_refused` | import.rs | A manifest field feeding `integrity_root` (`snapshot_head_seq`), edited without updating the root, is caught |
| `import_preview_full_restore_and_merge_work_through_the_cli` | cli.rs | End-to-end through the actual dispatcher: export, preview, full restore into a new vault (object_id preserved), and merge into an already-populated vault (source project_id never reused as the destination identity) |

## Commands executed (all re-run on this branch against `main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **267/267 pass**, 0 failed (234 lib [226 pre-existing + 8 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib import::` | `raw/05-import-module-tests-isolated.txt` | 7/7 pass |
| `cargo test --locked --lib cli::` | `raw/06-cli-module-tests-isolated.txt` | 9/9 pass |
| `cargo test --locked --lib export::` | `raw/07-export-module-tests-isolated.txt` | 8/8 pass (confirms `export.rs`'s own tests are unaffected by the one visibility widening this task made there) |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat main -- src/` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `git diff --check` | (terminal output, no artifact file — empty output = clean) | Exit 0, no whitespace errors |
| Environment capture | `raw/09-environment.txt` | Matches `reference-hardware.md`'s frozen development profile |
| Baseline head | `raw/08-baseline-head.txt` | `a92dce6...` on both `HEAD` and `origin/main` before this task's own commit |

Raw artifact manifest: `raw/00-manifest.txt` (generated before its own content existed, so does not hash itself — same pattern every prior `flake-v1` evidence report in this phase already uses).

## Failed attempts / exclusions

- **Three test bugs, all caught by this task's own first test run, not discovered later.** (1) A swapped tuple destructure — `let (_, current_revision_id) = store_current(...)` instead of `let (current_revision_id, _) = ...` — in the action-dependency-restoring subpass, since `read_current`/`store_current` return `(revision_id, payload)` in that order; the bug produced an immediate, unambiguous `expected revision conflict` error naming a JSON payload as the "expected" revision ID (the exact same class of mistake, and the exact same immediate detection, `T02-03`'s and `T02-04`'s own evidence reports already recorded independently — a real recurring footgun of this two-tuple return shape, not a one-off). (2) `full_restore_is_all_or_nothing_on_a_corrupt_member` originally tampered a file by replacing `"owner"` with the *longer* string `"attacker"`, which changed the file's length and therefore tripped this module's length-mismatch check rather than its digest-mismatch check the test's own name and assertion expected; fixed by tampering with a same-length replacement (`"owner"` → `"oWNER"`) to specifically isolate the digest check. (3) `tampered_manifest_integrity_root_is_refused` originally edited `record_count` (which is *not* one of the fields `integrity_root` hashes — `schema`/`kind`/`vault_id`/`project_id`/`snapshot_head_seq`/`members` are), so the test was actually (correctly) triggering a different, earlier consistency check rather than the integrity-root check its name promised; fixed by editing `snapshot_head_seq`, which does feed the hash. None were product defects — all three were test-construction mistakes caught immediately by the very first run, before any of them was ever believed to be a passing, correct test.
- No test was skipped, deleted, or weakened to reach green.

## Performance gate

No dedicated timing harness was run for this task. Import cost is proportional to the package's own revision count (one parse, one hash-verify, and one `commit` per revision) plus, for merge mode, the constant per-kind pass overhead — no whole-package single allocation beyond holding the parsed `ParsedPackage` structure in memory at once, which is the same S-scale-only limitation `T02-01`-`T02-05` already carry for their own full-scan/full-collection operations, deferred to `T05-02`'s hardware-qualified pass in the identical way. "No whole-package allocation" (this task's own performance-gate clause) is not separately proven at M/L scale by this evidence — recorded as unmeasured, not silently assumed.

## Durability gate

Every write this module performs is an ordinary `CanonicalWriter::commit` call — the exact same atomic primitive `T01-03`/`T01-07`'s 100-fault-schedule matrix already proved "complete pre-state or complete committed state, never a half-state" for. `import_full_restore` additionally validates the *entire* package before creating the destination store at all, so a validation failure (the far more likely failure mode for a hand-editable JSON/Markdown format than a mid-write process crash) leaves no destination artifact whatsoever — proven directly by `full_restore_is_all_or_nothing_on_a_corrupt_member`. No new fault-injection point was added: nothing in this module bypasses or wraps the canonical writer's own commit path in a way that could introduce a new half-state.

## Cross-platform gate

Native development profile only (Windows 11/NTFS, matching `reference-hardware.md`), consistent with every prior `flake-v1` task. This module performs no filesystem operations beyond ordinary reads of manifest-declared paths (all UUID/integer-safe, per `T02-05`'s own already-established naming discipline) — nothing platform-conditional. Remaining profiles are `T05-02`'s scope.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Valid package reconstruction exact | Satisfied for full restore — `full_restore_preserves_object_ids_and_all_payload_bytes` (object_id and payload bytes byte-identical; `revision_id`/`recorded_at` are a documented, honest exception — see "Architecture") |
| All malformed/ambiguous cases refuse complete success | Satisfied — `full_restore_is_all_or_nothing_on_a_corrupt_member`, `tampered_manifest_integrity_root_is_refused`, and the structural schema/chain/duplicate-ID checks in `read_and_validate_package` |
| Imported grants cannot disclose content | Vacuously satisfied — no grant entity exists in the current record model to import (recorded explicitly) |
| Read bounded directory-format members only; no arbitrary archive extraction | Satisfied by construction — no archive parsing exists anywhere in this module |
| Validate paths, lengths, digests, schema, capability, identity and references before publication | Satisfied — see "Security boundary" above; `capability` = the `payload_schema_version`/kind-recognition check already inherited from `RecordPayload::from_json` |
| Full restore preserves declared identity only into a new root | Satisfied — `object_id` preserved exactly; a fresh `vault_id` is minted (documented: this is a new vault populated with the same declared object identities, not a byte-identical resurrection of the original vault — that is `backup.rs`'s restore, a different mechanism) |
| Selected merge imports propose new admission with origin mapping | Satisfied — `import_selected_merge`'s `id_map`, `RecordOrigin::Import` |
| Colliding immutable IDs/digests reject | Satisfied — duplicate `(object_id, revision_id)` detection in `read_and_validate_package` |
| Identical repeated import reconciles idempotently | Satisfied for full restore, within one logical operation (command-ID-derived idempotent replay); explicitly scoped narrower for selected merge — see "Architecture" |
| Core portable import/validation, CLI preview, staging vault and tests | Satisfied — `src/import.rs`, `import-preview`/`import-full-restore`/`import-merge` |

## Completion condition

Every acceptance clause above is satisfied, with the two deliberate, explicitly-documented scope boundaries recorded rather than silently assumed (revision-envelope fidelity; merge-mode cross-run idempotency). Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (267/267). Three test-construction bugs were caught by this task's own first test run and fixed at the root cause — no product defect among them. No sealed evidence altered; no force-push; no historical evidence file touched; no change to `T01-02`-`T02-05`'s already-audited mutating logic beyond one purely-additive visibility widening in `export.rs`. `T02-06` is complete.

## Next frontier

`T02-07` — Prove project reconstruction without Flake. Depends on `T02-06` (this task) — the natural end-to-end proof that `docs/formats/portable-export-v1.md`'s format, plus this task's own import path, together deliver genuine, tool-independent ownership.
