# Tasks — Spec 002 corrective addendum (T01-01..07)

**Status:** `T00-02` COMPLETE → `T01-01` COMPLETE → `T01-02` READY.
**Frontier:** `specs/CURRENT.md` `ACTIVE_IMPLEMENTATION_UNIT=T01-02` as of this update.
**Policy:** `PAID_MODEL_INFERENCE=NOT_AUTHORIZED`, `REQUIRED_OPENAI_OR_PAID_MODEL_DEPENDENCY=NO` — unchanged; nothing in this addendum uses a model API. Tick tasks only after evidence exists (`AGENTS.md` §11).

## Gate — T00 intake (closed)

- [x] **T00-01** Reverify live GitHub truth and activate the implementation frontier. (`docs/evidence/flake-v1/T00-01/REPORT.md`, PR #65, merge commit `6389f651...`)
- [x] **T00-02** Publish this Spec 002 corrective addendum and admission record. (`docs/evidence/flake-v1/T00-02/REPORT.md`, PR #66)

## P01 — Reliable save and recovery (T01-01 COMPLETE; T01-02 READY)

- [x] **T01-01** Make legacy inspection nonmutating and establish ownership. Depends on T00-02. Files: `src/vault.rs`, `src/events.rs`, `src/locator.rs` (unchanged — no fix needed there), `src/cli.rs`, `src/lib.rs` (new error variant). Evidence: `docs/evidence/flake-v1/T01-01/REPORT.md`.
- [ ] **T01-02** Create an independently specified format-2 transaction store. Depends on T01-01. Files: storage/vault modules, `docs/formats/`.
- [ ] **T01-03** Commit save, full history and command result atomically. Depends on T01-02. Files: Core transaction/admission API, CLI save path.
- [ ] **T01-04** Preserve forensic bytes and recover to a verified new root. Depends on T01-03. Files: Core recovery/inspection modules, CLI recover/verify.
- [ ] **T01-05** Create and restore consistent verified backups. Depends on T01-04. Files: Core backup/restore modules, CLI.
- [ ] **T01-06** Import legacy vaults without rewriting accepted history. Depends on T01-05. Files: legacy readonly parser, migration/import module.
- [ ] **T01-07** Close the corrective save/recovery gate on a native host. Depends on T01-06. Files: fault/child-process harness, disposable fixtures.

Each task's exact objective, allowed/forbidden scope, acceptance criteria, tests, verification method, performance/durability/cross-platform gate, and completion condition are normatively defined in the canonical build plan §34 (`T01-01` through `T01-07`) and are not restated here to avoid drift between two copies of the same contract; `checklist.md` gives the per-task checkable summary and `plan.md` gives the per-task build notes.

## Explicit non-tasks (do not create these without a new ADR)

- No task numbered `T01-08` or higher exists; the next task after `T01-07` is `T02-01` (P02, portable project work), out of this addendum's scope.
- No task in this list touches `derived.sqlite`/FTS5 generation (that is `T02-04`).
- No task in this list performs a `cargo install`, adds a dependency, or edits `Cargo.toml` beyond what T01-02 explicitly enumerates in the canonical plan (pinning already-present `rusqlite`/`libsqlite3-sys` versions, not adding a new crate).
