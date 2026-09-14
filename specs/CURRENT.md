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
ACTIVE_IMPLEMENTATION_UNIT=T01-04
NEXT_DEPENDENCY_READY_UNIT=T01-04
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
T01-03_MERGE_COMMIT=PENDING_PR_MERGE
SPEC_003_AUTO_ACTIVATION=PROHIBITED
PROJECT_COMPLETE=NO
```

## Authority

The sole implementation roadmap is `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md`.
`docs/canonical/FLAKE_MUSE_EXECUTION_HANDOFF.md` is the short execution entry point.

Historical Fehrest, Phase T, R1, and Spec 002 artifacts remain immutable evidence. They are not the active product roadmap and must not be rewritten to make their historical identifiers match later GitHub history.

The historical local-only planning SHAs `4246f6d...` and `852e44b...` are provenance references only. Execution does not require those Git objects or any OneDrive/local path; `docs/evidence/flake-v1/T00-01/REPORT.md` records exactly how a pre-existing local branch carrying those identifiers was reconciled (not adopted as-is) against live GitHub truth.

## Next action

`T01-04` — preserve forensic bytes and recover to a verified new root. Acquire recovery ownership and exclude normal connections; preserve the complete guard/DB/journal before any engine recovery; recover only a working copy, verify it, and publish a new root only after verification (`docs/evidence/flake-v1/T01-03/REPORT.md`, `docs/formats/format-2-canonical-sqlite.md`).

`T01-03` closed: `src/canonical.rs` now implements the transaction/command admission API (schema version 2: `revision`/`current_object`/`command` tables). `CanonicalStore::writer()` binds mutation to the OS-held single-writer lease (reusing `crate::vault::WriteLock`, widened to `pub(crate)`); `CanonicalWriter::commit` admits one minimal opaque-payload record (create or update) atomically per command, with idempotent replay on a duplicate `command_id`, explicit rejection of a changed digest under the same ID, and explicit expected-revision-conflict rejection on a stale update. Deliberately out of scope, recorded in the format doc and evidence report: multi-object/multi-operation commands, a tombstone/delete command, CLI wiring to this store, and literal multi-process kill testing (this repository's established methodology uses deterministic in-process fault injection instead, same as `T01-01`).

Do not activate Spec 003 automatically. Do not invent a replacement roadmap. Do not use OpenAI API or a required paid AI/model service.
