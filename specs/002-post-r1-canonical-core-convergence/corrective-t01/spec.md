# Feature Specification: Spec 002 corrective addendum — T01-01..07 (format-2 save/recovery)

**Feature ID:** `002-post-r1-canonical-core-convergence/corrective-t01`
**Status:** SPECIFIED — instantiated by T00-02, not yet implementation-active
**Parent:** [FLAKE_CORRECTIVE_PLANNING_CONTRACT.md](../FLAKE_CORRECTIVE_PLANNING_CONTRACT.md) (Astro's Class C/D policy decision); [FLAKE_CANONICAL_BUILD_PLAN.md](../../../docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md) sections 13-22, 27-29, 34 (T01-01..07 contracts).
**Historical relationship:** This is an *additive* corrective slice living in a new subdirectory. It does not amend, delete, or reinterpret `spec.md`, `plan.md`, `checklist.md`, `tasks.md`, `analyze.md`, `ponytail-gate.md`, `verification.md`, or `dependencies.md` in the parent `specs/002-post-r1-canonical-core-convergence/` directory. Those remain the historical record of the original Spec 002 (T037-T083), which the closeout commit `b03b7a8` already marked complete. This addendum is the T01-xx prospective correction authorized by the canonical build plan.

## 1. Purpose

The founder's final-planning assignment (Astro) found, and the founder-authorized canonical build plan freezes as a Class C/D decision, that the current canonical write path has three source-backed defects that must be corrected before any further product breadth is added:

1. **Read/write boundary is not enforced.** `Vault::open_read` and related startup paths can create metadata or otherwise mutate before writer ownership is established (canonical plan section 7, row 1; T01-01 "why it exists").
2. **Save is not one atomic unit.** The current object write and event-journal append are separate operations; a crash between them, or a sync failure that is silently ignored, can leave a state where the acknowledged result and the persisted state disagree (section 7 row 2; T01-02/T01-03 "why it exists").
3. **Torn-tail recovery destroys forensic evidence.** The existing recovery path normalizes/rewrites the original log bytes rather than preserving them before attempting any repair (section 7 row 1 and 4; T01-04 "why it exists").

This addendum specifies the seven tasks (`T01-01` through `T01-07`) that close these three defects using a new, isolated format-2 SQLite canonical store, without touching the historical Phase T / original Spec 002 record and without reopening any already-decided architecture, UX, security, or product-scope question.

## 2. Scope

In scope — exactly `T01-01` through `T01-07` as contracted in the canonical plan section 34:

```text
T01-01  Make legacy inspection nonmutating and establish ownership
T01-02  Create an independently specified format-2 transaction store
T01-03  Commit save, full history and command result atomically
T01-04  Preserve forensic bytes and recover to a verified new root
T01-05  Create and restore consistent verified backups
T01-06  Import legacy vaults without rewriting accepted history
T01-07  Close the corrective save/recovery gate on a native host
```

Out of scope, deferred to later phases per the plan's own DAG (section 31): typed work-record CRUD (P02), search/FTS generation of the canonical store (P02/T02-04), source/temporal resolution (P03), agent disclosure/proposals (P03), desktop UI (P04), release packaging/signing/native cross-platform qualification beyond the single development-profile pass required by T01-07 (P05). None of these are touched by this addendum.

Explicitly forbidden by inheritance from canonical plan sections 10/38 (unchanged): network listener, account/auth, hosted or required local model, chat UI, agent execution, sync, collaborative editing, mobile, Spec 003 activation. This addendum grants none of these.

## 3. Functional requirements

Each requirement traces to one canonical-plan task contract (section 34) and one or more invariants `I01`-`I12` (section 13). No requirement here introduces a new architecture, UX, or security decision; each is a restatement, for spec-kit traceability, of what the canonical plan already froze.

| ID | Requirement | Owning task | Invariants |
|---|---|---|---|
| FR1-001 | Readonly legacy inspection performs zero canonical byte mutation, including no metadata creation on first open. | T01-01 | I02, I03, I09 |
| FR1-002 | Exactly one process holds writer ownership at a time via a stable OS-held lease; a second writer request returns `Busy`, never silently coexists. | T01-01 | I02, I04 |
| FR1-003 | Recovery cannot race a normal reader; recovery acquires exclusive access only after normal connections close or times out with `Busy`. | T01-01 | I02, I09 |
| FR1-004 | A new format-2 canonical store is created in an isolated staging root, using the SQLite engine already present in the dependency tree (`rusqlite`/`libsqlite3-sys`, currently used only by the derived/FTS index), with `DELETE`/`EXTRA` synchronous journal mode, foreign keys on, trusted schema off. | T01-02 | I03, I04, I10 |
| FR1-005 | An old-format (format-1) reader/writer refuses to open a format-2 store; a format-2 reader refuses an unrecognized/tampered schema. | T01-02 | I02, I10 |
| FR1-006 | One acknowledged save command produces one committed transaction containing the full new payload, its history entry, the current-state pointer, and the command result — never a partial/half state. | T01-03 | I04, I05, I07 |
| FR1-007 | A duplicate request (same command ID, same digest) returns the original result idempotently; the same command ID with a changed digest is rejected. | T01-03 | I04, I05 |
| FR1-008 | Recorded sequence and observed wall-clock time are captured separately; no fabricated or clock-derived last-writer-wins ordering. | T01-03 | I07 |
| FR1-009 | Recovery preserves the complete original guard/database/journal bytes before any engine-level repair is attempted; a working copy, not the original, is what gets recovered. | T01-04 | I09, I12 |
| FR1-010 | A recovery result is one of: verified-complete-restore-to-new-root, verified-partial-salvage (explicitly labeled, never a silent complete claim), or refuse-with-original-untouched. | T01-04 | I09 |
| FR1-011 | Backups use the SQLite online backup/snapshot API (not a raw file copy of a live database) and are verified by reopening the destination before being published (no-clobber) as complete. | T01-05 | I09, I10 |
| FR1-012 | Restore targets a new root, invalidates derived/FTS state, and verifies restored canonical bytes and history against the backup manifest. | T01-05 | I09, I10 |
| FR1-013 | Legacy (format-1) vaults are read via the existing nonmutating reader only; import writes into a *new* format-2 root with explicit migration provenance; the legacy root is never mutated. | T01-06 | I03, I05, I09, I10 |
| FR1-014 | Import produces a mapping/omissions report before admission; ambiguous or partial migration is never reported as a complete success. | T01-06 | I09, I05 |
| FR1-015 | The full corrective slice (T01-01..06) passes at least 100 deterministic process-fault schedules per mutating operation on the development native profile (D1-D5), with every failure preserved as evidence, not hidden or silently retried to green. | T01-07 | I04, I09 |

## 4. Non-functional requirements

- **NFR-PERF-1:** Section 27 budgets for "Save/write acknowledgement ≤64 KiB," "CLI project open readonly, M," and "Full verify / recovery working copy M" apply once dataset-M fixtures exist; T01-01..06 record *bounded/no-regression* observations at whatever fixture size is exercised, since dataset-M generation belongs to later CLI/typed-record work (P02). No task in this addendum claims a passing measured section-27 gate; that gate closes formally no earlier than T01-07's D1-D5 native pass, and the full M/L dataset gate remains a P02+ concern.
- **NFR-SEC-1:** Every new trust boundary introduced by T01-01..07 requires `V09` security tests per canonical plan section 29; see `checklist.md` for the enumerated boundaries (writer lease, recovery access, format-2 schema admission, migration import).
- **NFR-DUR-1:** `D1`-`D5` (development-profile durability classes; full definitions in canonical plan section 21) must be exercised by T01-07; `D6` (all native profiles) is explicitly deferred to `T05-02` per the plan's own text and is *not* a completion condition of this addendum.
- **NFR-DEP-1:** No new external dependency is introduced. `rusqlite` 0.37.0 / `libsqlite3-sys` 0.35.0 are already locked in `Cargo.lock` (used today only by `src/derived.rs`); T01-02 extends their use to the canonical store rather than admitting a new crate. See `dependency-admission.md`.

## 5. Explicitly not respecified here

Product thesis, UX model, canonical architecture (I01-I12), security model, licensing intent, and task sequencing are decided in the canonical build plan and are referenced, not restated in full, throughout this addendum. Where a task-level detail below appears to conflict with the canonical plan, the canonical plan controls and the conflict must be raised as a stop, per `AGENTS.md` §10, not resolved by silent reinterpretation here.
