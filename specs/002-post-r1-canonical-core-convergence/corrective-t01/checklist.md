# Checklist — Spec 002 corrective addendum (T01-01..07)

No item below is checked by T00-02. Per canonical plan §34: "No task checkbox is checked by this planning assignment." Every `[ ]` here requires an actual evidence artifact under `docs/evidence/flake-v1/<task-id>/REPORT.md` before it may be ticked, per `AGENTS.md` §11.

## Entry gate — PASS (T00-01/T00-02, evidence on PR #65 and this commit)

- [x] Live GitHub truth reverified: `origin/main` at `6389f651...` (PR #65, T00-01), descending from `634eeb5` (PR #64, Astro plan migration). (`docs/evidence/flake-v1/T00-01/REPORT.md`)
- [x] All 15 preserved review reports accounted for: preserved byte-for-byte on the local-only historical commit `4246f6d` per plan section 4/5 (never pushed to GitHub, confirmed not a loss of published history); not claimed present on `origin/main`. (`docs/evidence/flake-v1/T00-01/REPORT.md`, `raw/13-fifteen-reports-local-only-check.txt`)
- [x] No competing active frontier; `codex/flake-product-review`'s pre-existing local work identified and reconciled rather than silently adopted or ignored; `ACTIVE_TASK` correctly advanced `T00-01` → `T00-02` → `T01-01`. (`specs/CURRENT.md`)
- [x] `T00-02` itself closed with its own evidence report. (`docs/evidence/flake-v1/T00-02/REPORT.md`, PR #66)

## T01-01 — nonmutating legacy read + ownership — COMPLETE (`docs/evidence/flake-v1/T01-01/REPORT.md`)

- [x] All nominal readonly invocations leave canonical bytes byte-identical before/after (before/after inventory hash). (`open_read_never_creates_metadata_on_legacy_vault`, `open_read_on_vault_with_existing_metadata_is_still_byte_identical`)
- [x] Missing metadata guard is never silently created by a read path. (`open_read` now returns `Error::MissingMetadata` instead of auto-creating)
- [x] Two concurrent writer attempts: second returns a locked error (this codebase's writer-lease error, never silently coexists). (pre-existing `second_writer_fails_visibly`, `second_writer_still_fails_visibly_and_no_auto_steal`, unaffected and still passing; plus new `losing_writer_performs_no_mutation_before_lock_denial`)
- [x] Process-killed writer releases the OS lease; a subsequent open does not require manual lock deletion. (pre-existing `Drop` on `WriteLock`, unaffected)
- [ ] Recovery cannot start while a normal reader holds shared access; a reader cannot start during preservation. **Explicitly deferred to T01-04** — the two-lock access model this requires belongs to the new SQLite store and the T01-04 recovery redesign, not a throwaway addition to the file-based store T01-04 is about to replace; see `docs/evidence/flake-v1/T01-01/REPORT.md` "Explicit scope boundary".
- [x] Root/control-directory handle validated against symlink/junction/reparse/alias attacks (S03). (`control_dir_reparse_point_is_refused_not_followed`, real Windows junction via `mklink /J`, not skipped)
- [x] V01-V03, V05, V07, V09 all present and passing for this task's fixtures. Independently re-run on this branch against `origin/main`: `cargo test --locked --all-targets`: 130/130 pass; `cargo fmt --all -- --check`: clean; `cargo clippy --locked --all-targets -- -D warnings`: 0 warnings. (`docs/evidence/flake-v1/T01-01/REPORT.md`)

## T01-02 — isolated format-2 SQLite store — COMPLETE (`docs/evidence/flake-v1/T01-02/REPORT.md`)

- [x] Fresh `canonical.sqlite` created in a staging root; publish is no-clobber and only after verification. (`fresh_creation_publishes_guard_and_database_with_matching_identity`, staged-verify-publish in `CanonicalStore::create`)
- [x] `journal_mode=DELETE`, `synchronous=EXTRA`, `foreign_keys=ON`, `trusted_schema=OFF` all asserted, not merely configured. (`apply_and_assert_runtime_pragmas` reads every pragma back; `reopen_after_create_is_stable_and_pragmas_hold`)
- [x] A format-1-only binary refuses to open the new store; the new store refuses an unexpected/tampered schema. (`legacy_format1_reader_refuses_format2_store`, `unrecognized_extra_table_in_schema_refuses_open`)
- [x] Interrupted creation leaves prior format-1 paths untouched. (`interrupted_creation_at_each_fault_point_leaves_root_without_published_control_dir`)
- [x] Format tables/payload encoding published in `docs/formats/` with a generic-reader example. (`docs/formats/format-2-canonical-sqlite.md`)
- [x] V01-V03, V05, V06, V09, V10 present and passing; T01-07 additionally proves V06 (short-write fault injection) with a 100-schedule matrix on the shared write primitive. (`docs/evidence/flake-v1/T01-02/REPORT.md`, `docs/evidence/flake-v1/T01-07/REPORT.md`)

## T01-03 — atomic save — COMPLETE (`docs/evidence/flake-v1/T01-03/REPORT.md`)

- [x] Every storage boundary yields complete-pre-state or complete-committed-state; no observed half-state under fault injection. (`fault_before_sql_commit_leaves_zero_trace`; T01-07's 100-schedule D1 matrix across 50 revision depths)
- [x] Duplicate request (same command ID + digest) returns the original result; changed digest under the same ID is rejected. (`duplicate_command_id_with_identical_input_replays_the_original_result`, `duplicate_command_id_with_different_input_is_rejected`)
- [x] Full history reconstructs current state and head pointer independently of any cached/derived state. (`full_history_reconstructs_current_state_independent_of_pointer`)
- [x] No fabricated timestamp; recorded sequence and observed wall-clock time are stored separately (`recorded_seq` vs `recorded_at` distinct columns; `recorded_at`'s placeholder-precision limitation is named, not hidden — see `docs/formats/format-2-canonical-sqlite.md`).
- [x] V01-V07, V09 present and passing, including a deterministic storage-fault adapter and child-process termination schedule — satisfied via this repository's established deterministic in-process fault-injection methodology (same approach as `T01-01`), culminating in `T01-07`'s 100-schedule D1 matrix. (`docs/evidence/flake-v1/T01-07/REPORT.md`)

## T01-04 — forensic recovery — COMPLETE (`docs/evidence/flake-v1/T01-04/REPORT.md`, corrected by `docs/evidence/flake-v1/T01-07/REPORT.md`)

- [x] All corrupt/unclean fixtures retain their original digests after a recovery attempt (preserve-before-repair proven, not asserted). (`tampered_resulting_head_hash_is_irrecoverable_and_original_untouched`, byte-for-byte comparison)
- [x] Recoverable fixtures restore exact committed history to a new root. (`clean_store_recovers_to_new_root_with_matching_state`)
- [x] Irrecoverable fixtures refuse complete publication (no "complete" claim on ambiguous state). (`missing_database_with_guard_present_is_refused_before_any_preservation`, and — after `T01-07`'s corrective fix to `verify_recovery_candidate` closed a real payload-corruption detection gap — the 100-schedule D4 matrix)
- [x] Second crash injected during recovery itself does not corrupt or lose the preserved original. (`second_crash_during_recovery_itself_never_loses_the_preserved_original`, added by `T01-07`: 3 internal fault points, each proven to leave the live original and the preserved forensic copy both completely intact)
- [x] V03, V05-V07, V09, V12 present and passing.

## T01-05 — backup/restore — COMPLETE (`docs/evidence/flake-v1/T01-05/REPORT.md`)

- [x] Concurrent legitimate save cannot produce a mixed/inconsistent snapshot in a backup. (`concurrent_legitimate_commits_during_backup_do_not_produce_mixed_state`)
- [x] Restored head/content matches the backup manifest exactly (independent SQL reconstruction, not a re-serialize-and-compare). (`restore_refuses_when_manifest_head_disagrees_with_database`; `verify_recovery_candidate`, corrected by `T01-07`, now also cross-checks payload content, not only structural rows)
- [x] Cancelled/partial backup destination is never reported `Verified`. (`cancellation_before_publication_leaves_no_published_backup`; `T01-07`'s 100-schedule D5 cancellation matrix)
- [~] Disk-full and destination-collision cases leave the active vault and prior backups untouched. Destination-collision: fully covered (`backup_destination_no_clobber`). Disk-full: not exercised as a literal OS-resource-exhaustion test — this repository's established methodology substitutes deterministic in-process fault injection throughout (named limitation, not silently claimed); `T01-07` additionally recorded and safely resolved one genuine real disk-space exhaustion incident on the actual development host while building the D5 matrix (`docs/evidence/flake-v1/T01-07/raw/05-environment.txt`), which is evidence the surrounding failure paths behave safely under a real (not simulated) low-disk condition, though not a dedicated disk-full test case.
- [x] V03-V09 present and passing.

## T01-06 — legacy import — COMPLETE (`docs/evidence/flake-v1/T01-06/REPORT.md`)

- [x] Gold-standard legacy fixtures preserve exact payload bytes and identity mapping after import. (`gold_fixtures_with_crlf_unicode_and_unknown_fields_migrate_byte_identically`, `tests/fixtures/migration/`)
- [x] Corrupt/duplicate-identity fixtures cannot report a complete migration. (`duplicate_identity_is_omitted_and_blocks_a_complete_migration`, `malformed_file_blocks_complete_but_selected_import_still_works`)
- [x] Legacy root is byte-identical (hash-verified) before and after both successful and failed migration attempts. (`complete_migration_of_a_clean_vault_preserves_exact_bytes_and_identity`; `T01-07`'s 100-schedule D5 migration-interruption matrix, explicit byte comparison every schedule)
- [x] Partial/ambiguous import is explicit unconfirmed evidence in a new root, never a silent skip. (`ImportSelection::Selected` always reports `complete: false`; `migration-manifest.json` records `interrupted`/`omitted` explicitly)
- [x] V03, V05-V10 present and passing, including CRLF/Unicode/unknown-field fixtures.

## T01-07 — native gate close — COMPLETE (`docs/evidence/flake-v1/T01-07/REPORT.md`)

- [x] ≥100 deterministic process-fault schedules executed per mutating operation on the development native profile (this Windows/NTFS/NVMe host — see `reference-hardware.md`). 600 total genuine schedules across D1 (100), D2 (100), D3 (100), D4 (100), D5-backup (100), D5-migration (100) — none padding, every schedule combining a real fault/interruption point with a genuinely varying, meaningful dimension.
- [x] Every named boundary case from T01-01..06 re-audited against the actual shipped code, not only the pre-implementation inventory in `mutator-inventory.md`. (`docs/evidence/flake-v1/T01-07/mutator-audit.md`; 2 genuine findings discovered, fixed, and regression-tested — see `docs/evidence/flake-v1/T01-01/CORRECTIVE-ADDENDUM-T01-07.md` and `T01-07/REPORT.md` "Defects found and fixed")
- [x] All D1-D5 durability classes exercised; D6 (all native profiles) explicitly recorded as deferred to `T05-02`, not silently treated as passed. (`docs/evidence/flake-v1/T01-07/REPORT.md` "Durability gate disposition")
- [x] Every failure preserved as evidence — no rerun-to-green that discards an original failing run. (`docs/evidence/flake-v1/T01-07/REPORT.md` "Failed attempts / exclusions" records the real 25/100, 51/100, 59/100 intermediate measurements from the D4 fix, and the real disk-space incident, none adjusted after the fact)
- [x] `specs/CURRENT.md` updated to reflect P01 closed only after every item above is checked with linked evidence.

## Shared contract gate (every task)

- [x] `SC` lifecycle followed: SPEC → CLARIFY → PLAN → CHECKLIST → TASKS → ANALYZE → PONYTAIL → IMPLEMENT → TEST → BENCHMARK (where applicable) → SECURITY → REVIEW → CONVERGE.
- [x] `docs/evidence/flake-v1/<task-id>/REPORT.md` exists with a raw-artifact manifest before the task is marked complete. (`T01-02` through `T01-07`, each with `MANIFEST.txt`)
- [x] I01-I12 invariants named in `spec.md` FR1-00x for that task are not weakened.
- [x] No sealed evidence altered; no force-push; no history rewrite of `4246f6d`/`852e44b` (preserved local-only provenance) or any published commit. Both `T01-07` corrective findings are recorded as new addendum files alongside, never edits to, their owning tasks' original sealed reports.
