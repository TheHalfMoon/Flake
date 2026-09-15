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
ACTIVE_IMPLEMENTATION_UNIT=T02-01
NEXT_DEPENDENCY_READY_UNIT=T02-01
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
T01-07_MERGE_COMMIT=PENDING_PR_MERGE
SPEC_003_AUTO_ACTIVATION=PROHIBITED
PROJECT_COMPLETE=NO
```

## Authority

The sole implementation roadmap is `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md`.
`docs/canonical/FLAKE_MUSE_EXECUTION_HANDOFF.md` is the short execution entry point.

Historical Fehrest, Phase T, R1, and Spec 002 artifacts remain immutable evidence. They are not the active product roadmap and must not be rewritten to make their historical identifiers match later GitHub history.

The historical local-only planning SHAs `4246f6d...` and `852e44b...` are provenance references only. Execution does not require those Git objects or any OneDrive/local path; `docs/evidence/flake-v1/T00-01/REPORT.md` records exactly how a pre-existing local branch carrying those identifiers was reconciled (not adopted as-is) against live GitHub truth.

## Next action

`T02-01` — first task of `P02`. Create and manage the four project record types (`Project`/`Note`/`Action`/`Decision`) through the transaction API section 15 defines: the common envelope, field/record limits, project membership, immutable IDs, archive/tombstone history preservation, and CLI typed commands (`docs/evidence/flake-v1/T01-07/REPORT.md`, `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` T02-01 row). This is the first task to build product-visible typed records on top of the format-2 store `P01` closed.

**`P01` is closed.** `T01-07` re-audited every canonical mutator/reader introduced since `T00-02`'s original inventory against the code as actually shipped (`docs/evidence/flake-v1/T01-07/mutator-audit.md`), ran 600 genuine deterministic fault schedules across durability classes D1–D5 (100 each; D6 explicitly deferred to `T05-02`, never claimed as passed), and found and fixed two real defects along the way rather than papering over them: (1) `EventLog::open` still unconditionally created its control directory despite `T00-02`'s own inventory naming this as a required `T01-01` fix that was never actually applied (not exploitable in any current call site, but the contract now matches its documented disposition — `docs/evidence/flake-v1/T01-01/CORRECTIVE-ADDENDUM-T01-07.md`); (2) `canonical::verify_recovery_candidate` (built by `T01-04`, reused by `T01-05`'s restore path) never cross-checked a revision's stored payload against its own `payload_sha256`, so payload-content corruption was invisible to both recovery and restore verification — found directly by this task's own D4 fault-schedule matrix (25/100 refused before the fix), root-caused, and closed. `specs/002-post-r1-canonical-core-convergence/corrective-t01/checklist.md` now has every `T01-01`–`T01-07` box checked with linked evidence.

Do not activate Spec 003 automatically. Do not invent a replacement roadmap. Do not use OpenAI API or a required paid AI/model service.
