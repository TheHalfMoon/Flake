# Implementation Plan — Spec 002 corrective addendum (T01-01..07)

**Status:** PLANNED / NOT STARTED — no task in this addendum has an implementation commit yet.
**Depends on:** `spec.md`, `clarify.md`, canonical build plan §13-22/27-34, `FLAKE_CORRECTIVE_PLANNING_CONTRACT.md`, `mutator-inventory.md`, `dependency-admission.md`, `reference-hardware.md`.

## 1. Strategy

Implement in the seven task-sized slices the canonical plan already fixes as `T01-01` → `T01-07` (strictly serial; canonical plan §31 — "Every task depends on its immediate predecessor"). No slice reordering, merging, or splitting is authorized by this addendum.

```text
T01-01  nonmutating legacy read + OS-held writer/recovery ownership
   -> T01-02  isolated format-2 SQLite canonical store (staging root, no-clobber publish)
   -> T01-03  one atomic save transaction (payload + history + result, idempotent)
   -> T01-04  forensic-preserving recovery to a verified new root
   -> T01-05  SQLite-native backup/restore, verified before publish
   -> T01-06  legacy (format-1) import into a new format-2 root, honest provenance
   -> T01-07  D1-D5 native fault qualification on the development profile, gate close
```

## 2. Reuse-first approach (Ponytail question 2-4)

Before any new code: the canonical store need not add a dependency. `rusqlite` 0.37.0 / `libsqlite3-sys` 0.35.0 are already locked and already compiled into every build (`src/derived.rs` uses them for the FTS5 derived index today). T01-02 is scoped to open a *second* SQLite database (`canonical.sqlite`, alongside the existing `derived.sqlite`) using the same crate, same version, same `bundled` build already admitted into the tree. See `dependency-admission.md` and `ponytail-gate.md` for the full necessity argument.

The existing writer-ownership primitives in `src/vault.rs` (`has_write_lock`, `writer()` → `VaultWriter`) are the foundation T01-01 tightens, not a construct to replace. `VaultWriter::add_object`/`append_event` already exist as writer-bound mutators; T01-01's job is closing the *bypasses* — the module-level `Vault::add_object` and `EventLog::append` that do not require a bound writer — not rewriting the writer concept itself. See `mutator-inventory.md` for the exact function-by-function classification this claim rests on.

## 3. Per-task build notes

| Task | Primary files (from canonical plan §34) | What changes | What must NOT change |
|---|---|---|---|
| T01-01 | `src/vault.rs`, `src/events.rs`, `src/locator.rs`, `src/cli.rs` | Split `open_read` into a genuinely nonmutating path; require a bound writer for every mutating call; validate root/control-directory handles against reparse/alias attacks (S03) | Historical Phase T experimental code paths retained read-only where still required; no new process/IPC boundary |
| T01-02 | storage/vault modules, `Cargo.toml`/`Cargo.lock` (no new deps expected), `docs/formats/` | New `canonical.sqlite` schema, staging-root creation, no-clobber publish, pragma enforcement (`DELETE`/`EXTRA`, foreign_keys on, trusted_schema off) | `derived.sqlite`/FTS5 schema; existing format-1 on-disk layout |
| T01-03 | Core transaction/admission API, CLI save path | One transaction = payload + history + command result + head pointer; idempotency by command ID + digest | Any success/failure reporting that could show "Saved" before durable commit |
| T01-04 | Core recovery/inspection modules, CLI recover/verify | Preserve-before-repair forensic copy; working-copy-only recovery; new-root publish only after verification | Original guard/DB/journal bytes — never modified in place |
| T01-05 | Core backup/restore modules, CLI | SQLite online backup API; verified reopen before publish; restore-to-new-root | Live vault during backup (no live-file copy); prior backups |
| T01-06 | Legacy readonly parser, migration/import module, CLI migration preview | New-root import with mapping/omissions report; explicit migration provenance field | Legacy root bytes (read-only always); no in-place upgrade |
| T01-07 | New fault/child-process harness, disposable fixtures | ≥100 deterministic fault schedules per mutating op; D1-D5 evidence; audit every mutator in `mutator-inventory.md` against final code | Sealed R1/Phase T evidence; no rerun-to-green hiding an original failure |

## 4. Ponytail gate result (summary; full argument in `ponytail-gate.md`)

**KEEP** — this is not deferrable polish. The canonical plan's own gap-closure table (§7) traces three source-backed defects (unenforced read/write boundary, non-atomic save, destructive torn-tail recovery) directly to `T01-01`-`T01-04`. Deferring would leave the *only* irreplaceable state in the product (canonical history) without crash-safety, which `docs/19-ENGINEERING-METHOD.md` §2.1 places on the list Ponytail may never minimize ("canonical-data integrity," "recovery correctness," "data-loss prevention"). No smaller implementation satisfies FR1-001..015 at lower cost: the reuse-first approach above already removes the one place a smaller-but-worse alternative (a custom write-ahead log) was rejected by Astro's final challenge (canonical plan §6).

## 5. What this plan does not authorize

- No implementation commit. `plan.md` describes intended per-task changes at the file/module level established by the canonical plan; it is not permission to start `T01-01` from this document — `T01-01`'s own contract (canonical plan §34) and this addendum's `checklist.md`/`tasks.md` govern activation.
- No new dependency, no new architecture, no new UX, no new security boundary beyond what `spec.md` FR1-001..015 already traces to the canonical plan.
- No installs, no `cargo build`/`cargo test` runs beyond what was already used to produce `mutator-inventory.md`'s static `grep` evidence (T00-02's forbidden scope explicitly excludes runtime/source edits and "no installs or product edits in this unit").
