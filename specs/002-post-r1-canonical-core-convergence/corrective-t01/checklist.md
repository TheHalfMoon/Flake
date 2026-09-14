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

## T01-02 — isolated format-2 SQLite store

- [ ] Fresh `canonical.sqlite` created in a staging root; publish is no-clobber and only after verification.
- [ ] `journal_mode=DELETE`, `synchronous=EXTRA`, `foreign_keys=ON`, `trusted_schema=OFF` all asserted, not merely configured.
- [ ] A format-1-only binary refuses to open the new store; the new store refuses an unexpected/tampered schema.
- [ ] Interrupted creation leaves prior format-1 paths untouched.
- [ ] Format tables/payload encoding published in `docs/formats/` with a generic-reader example.
- [ ] V01-V03, V05, V06, V09, V10 present and passing.

## T01-03 — atomic save

- [ ] Every storage boundary yields complete-pre-state or complete-committed-state; no observed half-state under fault injection.
- [ ] Duplicate request (same command ID + digest) returns the original result; changed digest under the same ID is rejected.
- [ ] Full history reconstructs current state and head pointer independently of any cached/derived state.
- [ ] No fabricated timestamp; recorded sequence and observed wall-clock time are stored separately.
- [ ] V01-V07, V09 present and passing, including a deterministic storage-fault adapter and child-process termination schedule.

## T01-04 — forensic recovery

- [ ] All corrupt/unclean fixtures retain their original digests after a recovery attempt (preserve-before-repair proven, not asserted).
- [ ] Recoverable fixtures restore exact committed history to a new root.
- [ ] Irrecoverable fixtures refuse complete publication (no "complete" claim on ambiguous state).
- [ ] Second crash injected during recovery itself does not corrupt or lose the preserved original.
- [ ] V03, V05-V07, V09, V12 present and passing.

## T01-05 — backup/restore

- [ ] Concurrent legitimate save cannot produce a mixed/inconsistent snapshot in a backup.
- [ ] Restored head/content matches the backup manifest exactly (independent SQL reconstruction, not a re-serialize-and-compare).
- [ ] Cancelled/partial backup destination is never reported `Verified`.
- [ ] Disk-full and destination-collision cases leave the active vault and prior backups untouched.
- [ ] V03-V09 present and passing.

## T01-06 — legacy import

- [ ] Gold-standard legacy fixtures preserve exact payload bytes and identity mapping after import.
- [ ] Corrupt/duplicate-identity fixtures cannot report a complete migration.
- [ ] Legacy root is byte-identical (hash-verified) before and after both successful and failed migration attempts.
- [ ] Partial/ambiguous import is explicit unconfirmed evidence in a new root, never a silent skip.
- [ ] V03, V05-V10 present and passing, including CRLF/Unicode/unknown-field fixtures.

## T01-07 — native gate close

- [ ] ≥100 deterministic process-fault schedules executed per mutating operation on the development native profile (this Windows/NTFS/NVMe host — see `reference-hardware.md`).
- [ ] Every named boundary case from T01-01..06 re-audited against the actual shipped code, not only the pre-implementation inventory in `mutator-inventory.md`.
- [ ] All D1-D5 durability classes exercised; D6 (all native profiles) explicitly recorded as deferred to `T05-02`, not silently treated as passed.
- [ ] Every failure preserved as evidence — no rerun-to-green that discards an original failing run.
- [ ] `specs/CURRENT.md` updated to reflect P01 closed only after every item above is checked with linked evidence.

## Shared contract gate (every task)

- [ ] `SC` lifecycle followed: SPEC → CLARIFY → PLAN → CHECKLIST → TASKS → ANALYZE → PONYTAIL → IMPLEMENT → TEST → BENCHMARK (where applicable) → SECURITY → REVIEW → CONVERGE.
- [ ] `docs/evidence/flake-v1/<task-id>/REPORT.md` exists with a raw-artifact manifest before the task is marked complete.
- [ ] I01-I12 invariants named in `spec.md` FR1-00x for that task are not weakened.
- [ ] No sealed evidence altered; no force-push; no history rewrite of `4246f6d`/`852e44b` (preserved local-only provenance) or any published commit.
