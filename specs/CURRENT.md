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
ACTIVE_IMPLEMENTATION_UNIT=T02-07
NEXT_DEPENDENCY_READY_UNIT=T02-07
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
T02-03_MERGE_COMMIT=e8a285d2672fb6093125739f4307675ebc08195a
T02-04_STATUS=COMPLETE
T02-04_EVIDENCE=docs/evidence/flake-v1/T02-04/REPORT.md
T02-04_MERGE_COMMIT=dd7ff40bec97d14d966ec1c6e9bd993e351bd366
T02-05_STATUS=COMPLETE
T02-05_EVIDENCE=docs/evidence/flake-v1/T02-05/REPORT.md
T02-05_FORMAT_DOC=docs/formats/portable-export-v1.md
T02-05_MERGE_COMMIT=a92dce62eca038cd643620bf158175f916acf394
T02-06_STATUS=COMPLETE
T02-06_EVIDENCE=docs/evidence/flake-v1/T02-06/REPORT.md
T02-06_MERGE_COMMIT=PENDING_PR_MERGE
SPEC_003_AUTO_ACTIVATION=PROHIBITED
PROJECT_COMPLETE=NO
```

## Authority

The sole implementation roadmap is `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md`.
`docs/canonical/FLAKE_MUSE_EXECUTION_HANDOFF.md` is the short execution entry point.

Historical Fehrest, Phase T, R1, and Spec 002 artifacts remain immutable evidence. They are not the active product roadmap and must not be rewritten to make their historical identifiers match later GitHub history.

The historical local-only planning SHAs `4246f6d...` and `852e44b...` are provenance references only. Execution does not require those Git objects or any OneDrive/local path; `docs/evidence/flake-v1/T00-01/REPORT.md` records exactly how a pre-existing local branch carrying those identifiers was reconciled (not adopted as-is) against live GitHub truth.

## Next action

`T02-07` — Prove project reconstruction without Flake (`docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` T02-07 row). Not a product-code task: "Have an independent executor/tool reconstruct from exported JSON/bytes and separately inspect canonical SQLite **without linking Core or using its importer**" — a standalone generic-reader verification tool (its own, deliberately not `crate::import`/`crate::export`), disposable project fixtures, and evidence that the whole create/capture/find/complete/export loop and an independent exit both hold, including running the CLI loop with the derived DB removed. "Why it exists: a product-owned export/import pair can share the same omission bug" — this task exists specifically to catch a bug `T02-05`/`T02-06`'s own tests structurally cannot, by construction, catch (both are this crate's own code, testing itself).

`T02-06` closed: `src/import.rs` adds `import_full_restore` (brand-new empty vault, `object_id` preserved exactly via the same `ImportObject` exception `T01-06`'s migration established) and `import_selected_merge` (already-open destination, new Core-assigned identities, every internal cross-reference — `project_id`, `Action::dependency_ids`, `Relation` endpoints — rewritten via a returned `id_map`, in a fixed pass order that resolves forward references without a second staging mechanism). Every write is an ordinary, already-audited `CanonicalWriter::commit`; validation (member digests, manifest `integrity_root`, revision-chain consistency, duplicate-ID detection) always runs to completion *before* any canonical write, so a rejected import leaves zero destination side effects. Honestly documented fidelity boundary: `object_id` and payload bytes are preserved exactly; `revision_id`/`recorded_seq`/`recorded_at` are not (the destination store always mints these fresh — verified directly against `canonical.rs`'s own commit code, not assumed). New CLI: `import-preview`/`import-full-restore`/`import-merge`. Full details, including three test-construction bugs (not product defects) this task's own first test run caught: `docs/evidence/flake-v1/T02-06/REPORT.md`.

Do not activate Spec 003 automatically. Do not invent a replacement roadmap. Do not use OpenAI API or a required paid AI/model service.
