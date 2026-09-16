# T04-05 evidence report — Expose backup, recovery, import and export safely

**STATUS: COMPLETE.**

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` section 26/`T04-05` task row (P04, VS09), dependency `T04-04` (COMPLETE — `docs/evidence/flake-v1/T04-04/REPORT.md`)
- **Baseline / tested source commit:** forked from `origin/main` `7eb19a2275fd85b4059a9fc7660175db08e5d1b3` (PR #94 merge, T04-04)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only (hosted review remains unusable).

## Live-truth reverification performed before this task started

- `git fetch origin --prune` confirmed local `main` at `7eb19a2...`, matching `origin/main` and PR #94's merge commit.
- Confirmed `specs/CURRENT.md`'s `ACTIVE_IMPLEMENTATION_UNIT=T04-05`, `T04-04_STATUS=COMPLETE`.
- Confirmed `backup.rs`/`recovery.rs`/`export.rs`/`import.rs` (from `T01-04`/`T01-05`/`T02-05`/`T02-06`) already exist and are already tested, but their report/preview types did **not** already derive `Serialize` (unlike every T04-04 type, which happened to already have it) — this task's own additive Core change, described below.

## Scope actually touched

**Core (`src/`) — additive `#[derive(Serialize)]` only, zero logic changes**, confirmed by the full existing Rust suite re-passing unchanged (`raw/06`, 326/326):

- `src/backup.rs`: `BackupReport`, `RestoreReport`.
- `src/recovery.rs`: `RecoveryReport`.
- `src/export.rs`: `ExportReport`, `ExportPreview`.
- `src/import.rs`: `ImportPreview`, `ImportReport` (plus adding the `Serialize` import).

Every field type these structs contain (`PathBuf`, `String`, `bool`, `usize`, `i64`, `HashMap<String,String>`, and the already-`Serialize` `BackupManifest`/`ExportManifest`) was already serializable — this is the same "make an already-fully-typed Core result transportable over IPC, no new semantics" pattern `T04-03`'s `tombstone_note` addition and `T04-04`'s `resume.rs`/`decision_state.rs` derives already established.

**Desktop (`desktop/src-tauri/src/commands.rs`, `main.rs`) — 9 new typed commands**, each calling an already-audited Core function unchanged:

- `vault_backup` (async, genuinely cancellable — see "Durability/cancellation" below) / `vault_restore_from_backup` — wrap `backup::{backup_to_new_root,restore_from_backup}`.
- `vault_recover` — wraps `recovery::recover_to_new_root`.
- `export_preview` / `vault_export` — wrap `export::{preview_export,export_to_new_root}`.
- `import_preview` / `vault_import_selected` — wrap `import::{preview_import,import_selected_merge}` (distinct from `T04-01`'s existing `vault_restore`, which wraps `import_full_restore` into a brand-new empty vault; this task's `vault_import_selected` merges into an **existing** vault, requiring the owner to have already reviewed `import_preview`'s own conflict list first — never a silent merge).
- `cancel_operation` — signals a shared `Arc<AtomicBool>` cancellation flag by operation ID; a new `CancellationRegistry` Tauri-managed state (a `Mutex<HashMap<String, Arc<AtomicBool>>>`) backs it.

**No new native dialog capability.** Every destination/source in this task's new commands is selected via `pick_directory` (T04-01's already-admitted, already-audited native folder picker) — reused, not extended. This satisfies the task's own "native destination dialog bindings" component without adding a new plugin, permission, or ACL surface.

Frontend (`desktop/src/`) — new component: `DataManagementPanel.tsx` (backup with a live Cancel button while running; restore-from-backup; recover-to-new-root; export preview-then-export with scope selection; import preview-then-merge, with merge disabled whenever the preview reports conflicts).

## Implementation requirements disposition

| Requirement (from the task contract) | How satisfied |
|---|---|
| Show last verified backup and plaintext/history scope | `BackupReport.manifest` (verified flag, snapshot head, member list) rendered directly; no separate "last backup" store was added since Core does not persist backup history — each result is shown as it completes. |
| Preview import/export destination and omissions | `export_preview`/`import_preview` are always called before `vault_export`/`vault_import_selected` in the UI flow; `ExportManifest.omissions` and `ImportPreview.conflicts` are both rendered explicitly. |
| Support cancellation/progress | `vault_backup` genuinely supports cancellation (proven live in the E2E, not simulated) via a real shared flag checked by Core's own copy loop. The other four operations (`restore_from_backup`/`recover_to_new_root`/`export_to_new_root`/`import_full_restore`/`import_selected_merge`) have no cancellation hook in Core itself — this is recorded honestly as a Core-structural fact, not papered over with a bridge-level fake "cancel" that would do nothing. |
| New-root recovery | `vault_recover` wraps `recover_to_new_root` unchanged; every operation in this task writes to a **new** destination the owner selects, never in place. |
| Never offer in-place destructive repair, silent merge, erase-on-uninstall or automatic sync | No in-place write path exists anywhere in this task's five commands (`join_child` from T04-01 is reused, refusing any destination name containing a path separator). `vault_import_selected` requires the owner to have already seen `import_preview`'s conflict list (the UI disables the merge button when conflicts are present); no automatic/background sync or uninstall hook exists in this crate at all. |
| Surface generic reconstruction instructions and precise unsupported-location behavior | `import_preview` on a non-package directory fails with Core's own precise error naming the exact missing file (`raw` evidence: `"cannot read .../.fehrest-export/export-manifest.json: ... (os error 3)"`) — rendered verbatim, not replaced with a generic message that would hide what actually went wrong. |

## Security considerations (S03/S07/S10) disposition

- **S07/S10 (no-clobber)**: proven live — a second backup to the same destination is refused with `"backup destination already published"` (`backup-no-clobber-refused`), matching the same refusal `export_to_new_root`/`import_full_restore` already enforce (unchanged, reused).
- **S03 (owner-selected destinations only)**: every destination/source argument traces to `pick_directory`'s own return value or a typed name bounded by `join_child` (path-separator/`.`/`..` refused) — confirmed by source inspection of every new command; no path in this task's new surface can originate from imported/remote content.
- **Privacy preview**: `export_preview` is always called before `vault_export` in the UI; `ImportPreview.conflicts`/`ExportManifest.omissions` are both rendered, not summarized away.

## Verification performed

**Scripted E2E, native CDP-driven, byte verification and original-vault comparison** (`docs/evidence/flake-v1/T04-05/e2e-test/`) — this task's own two named verification techniques, both applied together rather than either alone:

1. Every manifest member's claimed SHA-256 is independently recomputed from the actual on-disk bytes by Node, never trusting Core's own `verified: true` claim alone.
2. The *original* vault is read directly (via T02-07's unmodified `sqlite_reader.py`) before and after every mutating operation, proving its `transaction_head_seq`/`object_count`/`vault_id` are unchanged; the *resulting* vault (backup-restored/recovered/import-merged) is independently read the same way and compared against the original.

Qualifying run (`e2e-test/results/run-2026-09-16T05-22-17-864Z.json`/`.log`), `overallOk=true`, every step, including:

- Backup created and independently byte-verified; no-clobber refused; original vault confirmed unchanged.
- Restore-from-backup produces a vault matching the original, independently confirmed.
- **A genuinely raced cancellation**: against a ~3.2 MB inflated store (to widen the cancellation-check window), a concurrent `cancel_operation` call actually won the race on this run (`raceOutcome: "genuinely-cancelled"`), and the cancelled destination was confirmed to have no published control directory — not simulated, an actual interrupted-mid-flight result. The harness also accepts (and would still report as passing) the legitimate alternative outcome where the cancel signal loses the race on a small store, since that is not a defect either — see `e2e-test/README.md`'s own honest note on this.
- Export preview/export/independent-verification: zero dangling references, recomputed integrity root matches the manifest's own claim.
- Import preview (no conflicts) → selected-merge into a fresh, unrelated vault, receiving **new** object identities (`id_map` non-empty) with byte-exact content, confirmed by reading it back through the desktop bridge.
- An unsupported (non-package) import source fails cleanly with Core's own precise, unreplaced error text.
- Recovery to a new root matches the original; the original (plus its own forensic-preservation copy, unchanged Core behavior) remains untouched.

Full accounting, including one earlier run that found and fixed a harness-only Python argument-type bug: `e2e-test/README.md`.

## Gates

| Gate | Command | Result |
|---|---|---|
| Root Rust format | `cargo fmt -- --check` | Exit 0, no diff |
| Root Rust lint | `cargo clippy --all-targets -- -D warnings` | Exit 0, 0 warnings |
| Root Rust suite | `cargo test --locked --lib` | 326/326 pass, unchanged — `raw/06` |
| Desktop Rust format | `cargo fmt -- --check` (desktop/src-tauri) | Exit 0, no diff |
| Desktop Rust lint | `cargo clippy --all-targets -- -D warnings` | Exit 0, 0 warnings (including the new `async fn` + `tauri::State` + `spawn_blocking` cancellation pattern) |
| Rust advisories | `cargo audit` | Exit 0, 0 vulnerabilities — `raw/02` |
| Frontend build | `npm run build` (`tsc && vite build`) | Exit 0 — `raw/04` |
| Frontend advisories | `npm audit` | 0 vulnerabilities, unchanged package count — `raw/03` |
| `git diff --check` | — | Exit 0, no trailing-whitespace/CRLF issues |

Raw artifact manifest with sizes and SHA-256: `raw/00-manifest.txt`.

## Performance gate

Not separately re-measured; batch progress is not reported beyond the "running/cancel available" state, since Core's own `backup_to_new_root` reports no incremental progress signal (only a cancellation check) to surface one from.

## Durability gate

**D5 desktop publication/restore integration**: every one of this task's five write operations targets a brand-new destination the owner selects (never in place), matching `backup.rs`/`export.rs`/`recovery.rs`/`import.rs`'s own existing atomicity guarantees unchanged. **Cancellation**: proven genuinely live for `vault_backup` (the one Core operation that supports it) via a real `Arc<AtomicBool>` flag checked inside Core's own copy loop, run on a `spawn_blocking` task so a concurrent `cancel_operation` call can actually be dispatched and take effect — not a UI-only "cancel" that does nothing underneath.

## Cross-platform gate

Native dialogs and permissions across all supported profiles remain `T04-06`'s own gate, per this task's contract; this task added no new native dialog (every selection reuses `pick_directory`). Native launch proven on this Windows 11/x86_64 development host only.

## Acceptance criteria disposition

| Acceptance clause | Status |
|---|---|
| Non-CLI user can create backup, restore to new location and full export/import | **Proven live**, all four operations, through the real desktop bridge, independently byte/content-verified |
| Failed/cancelled paths clearly incomplete with originals unchanged | **Proven live**: no-clobber refusal, a genuinely-cancelled backup leaving no published destination, and every original vault confirmed unchanged after every operation via independent re-reads |

## Completion condition

**Met.** Every acceptance clause above is satisfied by live, independently-verified evidence. Predecessor/phase gate (`T04-04` COMPLETE) was already satisfied. The only `src/` change is additive `#[derive(Serialize)]` with zero logic change, confirmed by the unchanged 326/326 Rust suite. No gate weakened, no test skipped, no evidence hidden.

## Next frontier

Proceed to `T04-06`.
