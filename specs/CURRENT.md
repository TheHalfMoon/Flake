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
ACTIVE_IMPLEMENTATION_UNIT=T03-05
NEXT_DEPENDENCY_READY_UNIT=T03-05
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
T02-06_MERGE_COMMIT=dc2f1c16f8331be873937b6b6bc39888eedda9c1
T02-07_STATUS=COMPLETE
T02-07_EVIDENCE=docs/evidence/flake-v1/T02-07/REPORT.md
T02-07_MERGE_COMMIT=e7d9445b87f48c99ad2ed0f59b32194255520af3
T03-01_STATUS=COMPLETE
T03-01_EVIDENCE=docs/evidence/flake-v1/T03-01/REPORT.md
T03-01_MERGE_COMMIT=07694d572c295940e933f51fd72cfeddffb92e6d
T03-02_STATUS=COMPLETE
T03-02_EVIDENCE=docs/evidence/flake-v1/T03-02/REPORT.md
T03-02_MERGE_COMMIT=4325e505f31dc5bb5ae47282ea390b1bd623732e
T03-03_STATUS=COMPLETE
T03-03_EVIDENCE=docs/evidence/flake-v1/T03-03/REPORT.md
T03-03_MERGE_COMMIT=615e5f75e19437e49cc7883000fa4825ff6badc1
T03-04_STATUS=COMPLETE
T03-04_EVIDENCE=docs/evidence/flake-v1/T03-04/REPORT.md
T03-04_MERGE_COMMIT=PENDING_PR_MERGE
SPEC_003_AUTO_ACTIVATION=PROHIBITED
PROJECT_COMPLETE=NO
```

## Authority

The sole implementation roadmap is `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md`.
`docs/canonical/FLAKE_MUSE_EXECUTION_HANDOFF.md` is the short execution entry point.

Historical Fehrest, Phase T, R1, and Spec 002 artifacts remain immutable evidence. They are not the active product roadmap and must not be rewritten to make their historical identifiers match later GitHub history.

The historical local-only planning SHAs `4246f6d...` and `852e44b...` are provenance references only. Execution does not require those Git objects or any OneDrive/local path; `docs/evidence/flake-v1/T00-01/REPORT.md` records exactly how a pre-existing local branch carrying those identifiers was reconciled (not adopted as-is) against live GitHub truth.

## Next action

`T03-05` — Review and admit bounded agent proposals (`docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` T03-05 row, `P03`/`VS06`). Objective: accept selected external suggestions only through explicit owner review. Validate ≤1 MiB/100 operations, protocol/receipt binding, declared identity, operation allowlist, expected revisions and evidence references; keep the raw bounded proposal immutable and pending; owner-selected acceptance is one normal atomic command; recheck grant validity/current scope and revisions at acceptance (reject stale operations, never auto-rebase); exact replay returns the original result, a digest collision under the same ID is rejected. Proposals cannot issue grants, accept decisions, execute tools, migrate, restore or delete. Files/components: Core proposal parser/admission/review state, CLI diff/accept/reject and tests. Depends on `T03-04` (complete).

`T03-04` closed: two new `RecordPayload` kinds — `ExportGrant` (`src/grant.rs`, ninth kind, `Relation`'s shape: owner-issued, project-scoped, kind/ID-allowlisted, privacy-excludable, byte-budget-capped, TTL-capped at seven days) and `DisclosureReceipt` (`src/disclosure.rs`, tenth kind, immutable — never updated after creation). `disclosure::compile_disclosure_package` persists the receipt via `CommandTarget::CreateObject` *before* ever returning wire bytes to its caller (receipt-before-emission, D1/D5); the wire format is a header line plus one JSON object per disclosed item (never one big wrapped document — an earlier draft that nested items inside a shared envelope broke exact budget accounting, since the envelope's own byte cost wasn't attributable to any single item; fixed during self-review, see the evidence report's "Failed attempts"). Every candidate is grant-scope-checked first (kind allowlist, ID allowlist, privacy exclusions, each with its own reason) then budget-fit (full/truncated/omitted, each with its own reason) — `rejected` entries live only on the receipt, never on the budget-capped wire itself. `Relation`s are disclosed last and only when both endpoints were themselves disclosed, never revealing a relation naming an object the recipient can't otherwise see. §16 enforced at two independent layers: `export.rs`'s project-scoped path never emits a grant or receipt (proven negatively), and `import_selected_merge` explicitly refuses a package containing either (defense in depth; full-restore is unaffected — a same-owner backup, not a shareable disclosure). `Decision` disclosure reuses `T03-02`'s resolver unchanged (only `NeedsReview`/`CurrentSet` decisions are ever candidates; a solely-`Draft`/`Withdrawn` key is never offered at all, not even as a rejection). CLI: `grant-issue`, `grant-revoke`, `package-preview`, `package-export` (stage-then-reread-verify before reporting success). 23 new tests (7 `grant.rs` + 12 `disclosure.rs` + 1 `cli.rs` end-to-end + 2 `import.rs` + 1 `export.rs`, the last three proving the §16 boundary), all passing (337/337 full suite); `fmt`/`clippy -D warnings`/`git diff --check` all clean (a stray CRLF conversion in `project.rs` from a Windows-Python text-mode write, caught during self-review before commit, was reverted to LF to avoid a spurious whole-file diff). Full details: `docs/evidence/flake-v1/T03-04/REPORT.md`.

`T03-03` closed: an eighth `RecordPayload` kind, `ReviewCheckpoint` (`src/checkpoint.rs`), mirroring `Relation`'s shape — one canonical object per project recording an explicit, owner-driven "reviewed through this recorded sequence" marker, monotonic unless explicitly reset with a reason (`mark_reviewed_through`/`reset_checkpoint`). `src/resume.rs`'s `resume()` composes the owner resume view purely from already-canonical reads (never writes a checkpoint, so opening/computing resume can never itself mark reviewed — §12): conflicts (`NeedsReview` decision keys) and stale/missing evidence (non-`Match` latest source checks) precede current accepted decisions, next actions (non-terminal), relevant notes (changed since checkpoint) and the raw changes-since-checkpoint log, exactly §18's own priority-group ordering; the whole view is pinned to one fixed `head_seq` snapshot per call. CLI: `resume`, `checkpoint-mark`, `checkpoint-reset`, `checkpoint-history`. Self-review before commit caught a real gap: `import_selected_merge`'s explicit per-kind pass list initially omitted `review_checkpoint`, which would have silently dropped every checkpoint object during a merge-import — fixed (checkpoints now ride the same simple project_id-only pass as `Note`/`Decision`) and regression-tested (`merge_review_checkpoint_is_carried_through_with_its_project_id_rewritten`, `project_export_includes_a_review_checkpoint`). 19 new tests, all passing (314/314 full suite); `fmt`/`clippy -D warnings`/`git diff --check` all clean. Full details: `docs/evidence/flake-v1/T03-03/REPORT.md`.

`T03-02` closed: a standalone module, `src/decision_state.rs` (`resolve_decision_state`), deterministically resolves "what decision is currently accepted" for one `(project, decision_key)` as of a caller-chosen valid-time instant and recorded-sequence cutoff — `CurrentSet`/`NeedsReview`/`NoAcceptedDecision`, with `considered` always listing every candidate decision and, for each excluded one, why (lifecycle, out-of-interval, or its own `withdrawal_reason`). Not a reuse of the historical Phase T `temporal.rs`/`memory.rs` (immutable evidence, `AGENTS.md` §3): `Decision`'s already-explicit lifecycle/supersession model needs none of Phase T's five-rung confidence ladder, so the only rule this resolver applies is "two independently-accepted decisions sharing a key with overlapping valid time are always `NeedsReview`, never silently ranked" — structurally forced by `supersede_decision` always demoting the loser's lifecycle in the same step that records the supersession edge. Verified against a from-scratch, independently-written reference oracle (`tests::reference_oracle`, using `CanonicalStore::history`/`revisions_since` rather than the production path's `all_revisions` fold) across 200 seeded random scenarios (seed `20260915`); all agree. CLI: `decision-state --project <uuid> --key K [--as-of-valid TS] [--as-of-recorded N]`. 13 new tests, all passing (295/295 full suite); `fmt`/`clippy -D warnings`/`git diff --check` all clean. Full details: `docs/evidence/flake-v1/T03-02/REPORT.md`.

`T03-01` closed: a seventh `RecordPayload` kind, `SourceCheck` (`src/source_check.rs`), mirroring `Relation`'s architectural shape — an append-only observation of whether a previously admitted `Source`'s selected local file still matches, changed, went missing or became unreadable. `capture::import_file` now populates the previously-always-`None` `Source::claimed_path` locator hint. Three functions, deliberately separated: `check_source` (pure observation, never mutates `Source`), `reselect_source` (relocation, refused unless the new location's bytes are digest-identical to the last saved capture), `admit_changed_source` (explicit owner admission of changed bytes, always re-opening and re-hashing fresh). Both mutating functions require the caller's `expected_revision_id` (F21/I05). A symlink or any non-regular-file swapped in at the checked path is refused as `Denied`, never followed (S03). `export.rs`'s project-scope union, `import.rs`'s merge/rewrite passes and `index.rs` each got the minimal, mechanical extension a new `RecordPayload` kind structurally requires, matching every prior new-kind task's own precedent. `Unchecked` is a derived read-path label (zero rows in history), not a stored `CheckStatus` variant — recorded as a deliberate reading of §15, not an omission. 15 new tests, all passing (282/282 full suite); `fmt`/`clippy -D warnings`/`git diff --check` all clean. Full details: `docs/evidence/flake-v1/T03-01/REPORT.md`.

`T02-07` closed: a standalone Python (stdlib-only, no Flake crate dependency) independent verifier under `tools/independent-verify/` proves Flake's canonical/exported state is reconstructable without Flake. Two genuinely separate raw readers — `sqlite_reader.py` (direct `canonical.sqlite` inspection via `sqlite3`, independently re-deriving `payload_sha256` and the full `resulting_head_hash` chain from the format document's own published algorithm) and `export_reader.py` (a `.fehrest-export/` package reader, independently recomputing `integrity_root` and every member/payload digest) — each build a semantic report (record counts, current state per object, relations, action dependencies, source byte digests, project membership) from a disposable fixture built through the real `fehrest` CLI's full create/capture/find/complete/export loop across two projects. The two independently-derived reports agree exactly on the full-store export and on the project-scoped subset (`crosscheck.py`), including after the derived FTS index (`derived-fts.sqlite`) is deleted and the vault is re-exported — proving canonical export does not depend on derived state. An 11-case adversarial suite (`adversarial.py`) hand-corrupts copies of a valid export (missing member, length/digest mismatch, tampered `integrity_root`, duplicate `(object_id, revision_id)`, broken relation endpoint, absent action dependency, truncated JSON, a fully-removed referenced object, an unrecognized manifest schema, and a source `capture.bytes_hex`/`capture.sha256` mismatch) and confirms the verifier detects and clearly reports every one — no product defect was found in `T02-05`/`T02-06` by this task. Full details: `docs/evidence/flake-v1/T02-07/REPORT.md`.

Do not activate Spec 003 automatically. Do not invent a replacement roadmap. Do not use OpenAI API or a required paid AI/model service.
