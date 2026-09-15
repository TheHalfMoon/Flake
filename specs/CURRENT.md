# CURRENT — Flake v1 Execution Frontier

**Purpose:** one authoritative pointer for what repository work may happen now.

> This file is operational state, not historical evidence. Live GitHub truth and the canonical build plan control execution.

## Current frontier

```text
PRODUCT_IDENTITY=FLAKE
ASTRO_PLAN_COMPLETE=YES
CANONICAL_BUILD_PLAN=docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md
CANONICAL_BUILD_PLAN_SHA256=b555f83ff12882ae6f55f90bbeaa411de52b84661a7f3953350cbfc6bd789fb2
CANONICAL_PLAN_REMOTE_STATUS=MIGRATED_TO_GITHUB
CANONICAL_PLAN_LOCAL_DEPENDENCY=NONE
ACTIVE_IMPLEMENTATION_UNIT=T02-04
NEXT_DEPENDENCY_READY_UNIT=T02-04
P01_STATUS=CLOSED
T00-01_STATUS=COMPLETE
T00-01_EVIDENCE=docs/evidence/flake-v1/T00-01/REPORT.md
T00-01_MERGE_COMMIT=6389f6512ea0feb90fd2da7dcdbde6f442e7eb29
T00-02_STATUS=COMPLETE
T00-02_EVIDENCE=docs/evidence/flake-v1/T00-02/REPORT.md
T00-02_SPEC_KIT=specs/002-post-r1-canonical-core-convergence/corrective-t01/
T00-02_MERGE_COMMIT=7c4a5a6521676effa1d5151bec96c271daa22aae
T01-01_STATUS=COMPLETE
T01-01_EVIDENCE=docs/evidence/flake-v1/T01-01/REPORT.md
T01-01_MERGE_COMMIT=0ca5408a324da075f3a868928296bca910644cb5
T01-02_STATUS=COMPLETE
T01-02_EVIDENCE=docs/evidence/flake-v1/T01-02/REPORT.md
T01-02_FORMAT_DOC=docs/formats/format-2-canonical-sqlite.md
T01-02_MERGE_COMMIT=fdc15c42f1fde706336d1f9b0d7b5faadd83fa92
T01-03_STATUS=COMPLETE
T01-03_EVIDENCE=docs/evidence/flake-v1/T01-03/REPORT.md
T01-03_FORMAT_DOC=docs/formats/format-2-canonical-sqlite.md
T01-03_MERGE_COMMIT=b59d39edf92c3214f68903f5af4977a03c1346fa
T01-04_STATUS=COMPLETE
T01-04_EVIDENCE=docs/evidence/flake-v1/T01-04/REPORT.md
T01-04_FORMAT_DOC=docs/formats/format-2-canonical-sqlite.md
T01-04_MERGE_COMMIT=b9aaf88f89db96d54ecfcdffb763d925fed40c50
T01-05_STATUS=COMPLETE
T01-05_EVIDENCE=docs/evidence/flake-v1/T01-05/REPORT.md
T01-05_MERGE_COMMIT=3e1e30cba341e3a1301e0af646e37f771debb2c7
T01-06_STATUS=COMPLETE
T01-06_EVIDENCE=docs/evidence/flake-v1/T01-06/REPORT.md
T01-06_FORMAT_DOC=docs/formats/legacy-migration.md
T01-06_MERGE_COMMIT=7804f202921c3bb0f7b1dcce8b8db971c6e55ea6
T01-07_STATUS=COMPLETE
T01-07_EVIDENCE=docs/evidence/flake-v1/T01-07/REPORT.md
T01-07_MUTATOR_AUDIT=docs/evidence/flake-v1/T01-07/mutator-audit.md
T01-07_MERGE_COMMIT=57de9bbd77bfec64a73c8c654a6d9d1459802d0c
T02-01_STATUS=COMPLETE
T02-01_EVIDENCE=docs/evidence/flake-v1/T02-01/REPORT.md
T02-01_FORMAT_DOC=docs/formats/typed-records.md
T02-01_MERGE_COMMIT=06818675b6bc4b5ab2904e1f5e827c028da3586b
T02-02_STATUS=COMPLETE
T02-02_EVIDENCE=docs/evidence/flake-v1/T02-02/REPORT.md
T02-02_FORMAT_DOC=docs/formats/typed-records.md
T02-02_MERGE_COMMIT=bfff06c96917726ffff00881afc09c93dcd22599
T02-03_STATUS=COMPLETE
T02-03_EVIDENCE=docs/evidence/flake-v1/T02-03/REPORT.md
T02-03_MERGE_COMMIT=PENDING_PR_MERGE
SPEC_003_AUTO_ACTIVATION=PROHIBITED
PROJECT_COMPLETE=NO
```

## Authority

The sole implementation roadmap is `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md`.
`docs/canonical/FLAKE_MUSE_EXECUTION_HANDOFF.md` is the short execution entry point.

Historical Fehrest, Phase T, R1, and Spec 002 artifacts remain immutable evidence. They are not the active product roadmap and must not be rewritten to make their historical identifiers match later GitHub history.

The historical local-only planning SHAs `4246f6d...` and `852e44b...` are provenance references only. Execution does not require those Git objects or any OneDrive/local path; `docs/evidence/flake-v1/T00-01/REPORT.md` records exactly how a pre-existing local branch carrying those identifiers was reconciled (not adopted as-is) against live GitHub truth.

## Next action

`T02-04` — Find work with a disposable current lexical index (`docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` T02-04 row). Every `list_project_*`/dependency-graph/cycle-check function `T02-01`-`T02-03` added is a documented full scan (`CanonicalStore::list_current_objects`); this task builds the first real project-scoped index (§30/§15 F04 "derived corrupt/stale... discard candidate authority, keep canonical readable"; a replaceable, disposable FTS5 generation, never authoritative).

`T02-03` closed: `src/relation.rs` adds `Relation` as a sixth `RecordPayload` kind, entirely on the same already-audited transaction API — no new `CommandTarget` variant, no schema change. `Action` gained a real state machine (`action_allowed_transition`'s table covering `Open`/`Doing`/`Blocked`/`Done`/`Cancelled`), ordered cycle-checked `dependency_ids`, and a self-describing `last_transition` on every revision (the mechanism that makes "any reopening requires an event" and full history reconstruction hold without a new event store). `Decision` gained `basis`/`verification`, a half-open `[valid_from, valid_to)` interval (parsed/validated via new `capture::parse_rfc3339_utc`), owner-only `accept_decision` (the sole path to `Accepted`), `withdraw_decision`, and `supersede_decision` (two sequential atomic commands — a `Supersedes` `Relation` plus the old decision's lifecycle flip — cycle-checked, same-`decision_key`-required, preserving the prior `Accepted` revision in history). Evidence linkage (`Decision`/`Action` → `Source`) and supersession rationale both live solely on the `Relation` itself, never duplicated onto the record they describe (§15: "not a second copy"). Full details, including the two intentional non-product test-authoring fixes this task's own first test run caught: `docs/evidence/flake-v1/T02-03/REPORT.md`. Deliberately deferred: `Decision` tombstone/delete (`T04`'s UI-confirmation scope) and a floating (vs. pinned) `Relation` endpoint-revision policy.

Do not activate Spec 003 automatically. Do not invent a replacement roadmap. Do not use OpenAI API or a required paid AI/model service.
