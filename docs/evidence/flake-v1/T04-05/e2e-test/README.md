# T04-05 backup/recovery/import/export E2E test

Proves this task's own named verification method -- "Native scripted UI flows plus destination manifest/byte verification and original-vault comparison" -- and its acceptance clause: "Non-CLI user can create backup, restore to new location and full export/import; failed/cancelled paths clearly incomplete with originals unchanged."

## Mechanism

`run-e2e-test.mjs` reuses the established CDP-driven harness to drive the real compiled `flake-desktop.exe` through its exact real typed-IPC bridge. Two independent-verification techniques are combined:

1. **Byte verification, not trust**: after a backup, every manifest member's claimed SHA-256 is recomputed directly from the actual bytes on disk by Node (`crypto.createHash("sha256")`), never trusting the `verified: true` flag Core's own manifest already asserts.
2. **Original-vault comparison**: `independent_read_export.py` (a thin wrapper around T02-07's unmodified `export_reader.py`) and an inline Python snippet using T02-07's unmodified `sqlite_reader.py` read the *source* vault, the *backup/export*, and the *restored/recovered/imported* result independently of Flake's own binaries, before and after every mutating operation -- proving the original is byte-for-byte unchanged (same `transaction_head_seq`, `object_count`, `vault_id`) and the new copy matches.

## What each check proves

| Check | What it proves |
|---|---|
| `backup-create`, `backup-members-independently-byte-verified` | A backup is created and its manifest's claimed digests match the actual on-disk bytes, independently recomputed. |
| `backup-no-clobber-refused` | Backing up to an already-published destination is refused (S07/S10 "no-clobber"), not silently overwritten. |
| `original-vault-unchanged-after-backup` | The source vault's own head sequence and object count are identical before and after the backup, independently read. |
| `restore-from-backup`, `restored-vault-matches-original` | Restoring a backup to a new location produces a vault matching the original's head/object-count/vault-id, independently confirmed -- never assumed from the restore call's own return value alone. |
| `cancel-nonexistent-operation-returns-false` | The cancellation mechanism itself is well-defined: cancelling an operation that isn't running returns `false`, not an error. |
| `backup-cancellation-race-outcome`, `cancelled-backup-left-no-published-destination` | A genuine concurrent cancel (fired as close to the backup's own start as JS scheduling allows, against an inflated ~3.2 MB store so the copy loop's cancellation check has a real window to land in) actually interrupted the backup mid-flight on this run (`raceOutcome: "genuinely-cancelled"`) -- and the destination was confirmed to have **no** published `.fehrest` control directory afterward: a cancelled backup is never shown as complete, and the original vault was untouched throughout (see `no-clobber` and `unchanged-after-backup` above, which this operation shares a vault with). |
| `export-preview`, `export-create`, `export-independently-verified` | A project-scoped export preview matches the actual export's own record/revision counts; the exported package is independently readable with zero dangling references and a recomputed integrity root matching the manifest's own claim. |
| `second-vault-create`, `import-preview`, `import-selected-merge`, `imported-project-visible-with-new-identity`, `imported-note-content-matches` | A fresh, unrelated vault previews the exported package (no conflicts), merges it in, receives **new** object identities (`id_map` is non-empty -- never the source's own IDs), and the merged content is byte-exact -- proven by reading it back through the desktop bridge, not assumed from the import call's own report. |
| `import-preview-unsupported-directory-refused-cleanly` | Previewing an arbitrary, non-package directory as an import source fails cleanly with a readable error, not a crash or a silent empty result. |
| `vault-recover`, `recovered-vault-matches-and-original-unchanged` | Recovery to a new root produces a vault matching the original, and the original (including its own forensic-preservation copy, per `recovery.rs`'s own existing guarantee) remains unchanged, independently confirmed. |

## An honest note on the cancellation test

Core's `backup_to_new_root` checks its `should_cancel` callback once before each `sqlite3_backup_step(100 pages)` call. For a small vault, the entire backup can complete in a single step, in which case a same-process concurrent cancel call might lose the race and the backup simply succeeds -- which the harness records as `raceOutcome: "completed-before-cancel-landed"` and still treats as passing (a race that a cancel-in-time signal can also legitimately lose is not a defect), never fabricated as "cancelled." This run's own store was inflated to ~3.2 MB (four ~800 KB filler notes) specifically to widen that window, and on this run the cancel signal won the race, giving direct, non-simulated evidence of the cancelled path -- including that the destination was left with no published control directory.

## Runs on record

| Run | Result |
|---|---|
| `run-2026-09-16T05-21-45-138Z` | Failed on a harness-only bug: the inline Python snapshot helper passed a raw string to `sqlite_reader.read_vault`, which requires a `pathlib.Path` (the same class of argument-type mismatch already seen and fixed in earlier tasks' independent-verify wrappers). |
| `run-2026-09-16T05-22-17-864Z` | **Qualifying run.** Fixed, `overallOk=true`, every step passes, including a genuinely-raced cancellation landing as truly cancelled. |

Full step-by-step detail: `results/run-2026-09-16T05-22-17-864Z.json` / `.log`.
