# Analyze — Spec 002 corrective addendum cross-artifact consistency (T00-02)

**Status:** T00-02 closeout check, run before `docs/evidence/flake-v1/T00-02/REPORT.md` is written.
**Method:** `docs/19-ENGINEERING-METHOD.md` §3 `analyze` stage — "consistency check against the specification... this is where drift is caught."
**Inputs:** `spec.md`, `clarify.md`, `plan.md`, `checklist.md`, `tasks.md`, `ponytail-gate.md`, `mutator-inventory.md`, `dependency-admission.md`, `reference-hardware.md` (all this directory); canonical build plan §13 (I01-I12), §20-22 (security/durability/failure), §27 (performance), §29 (verification hierarchy), §34 (T01-01..07 contracts); `FLAKE_CORRECTIVE_PLANNING_CONTRACT.md`; historical `specs/002-post-r1-canonical-core-convergence/*` (unchanged).

## Architecture alignment

Every construct named in `spec.md`/`plan.md` traces to an already-frozen decision:

- New canonical SQLite store → canonical plan §13 "Implement directly: embedded SQLite canonical format 2... rollback journal with EXTRA synchronization, one OS-held writer" — matches `spec.md` FR1-004 exactly.
- Reuse of existing `rusqlite`/`libsqlite3-sys` rather than a new dependency → no conflict with §13's "New ADR... required before work: another canonical engine" clause, because this *is* the already-selected engine, not a different one.
- No WAL, no encryption, no sync/multi-user → none introduced by this addendum; `clarify.md` records WAL as explicitly out of scope without a new ADR.
- Writer-ownership/recovery-access lock ordering (T01-01) → matches canonical plan §14's "Acquire writer ownership before requesting exclusive recovery access; all processes use this order."

No drift found between this addendum and the canonical plan's architecture section.

## Invariant coverage (I01-I12)

| Invariant | Addressed by | Gap? |
|---|---|---|
| I01 Ownership | Unaffected — no network/account introduced | None |
| I02 Authority | T01-01 (writer lease closes bypasses); FR1-002/003 | None |
| I03 Identity | T01-01 (`ensure_vault_meta` UUIDv7 identity), T01-02 (guard/DB identity agreement), T01-06 (original ID retention) | None |
| I04 Atomicity | T01-02 (staging/no-clobber), T01-03 (one transaction) | None |
| I05 History | T01-03 (immutable history), T01-06 (no rewritten legacy history) | None |
| I06 Derivation | Not touched by this addendum (derived/FTS index untouched; T02-04 owns it) | Correctly out of scope, not a gap |
| I07 Time | T01-03 (recorded sequence vs. observed time separated) | None |
| I08 Disclosure | Not touched — no agent-visible disclosure surface exists yet (P03) | Correctly out of scope |
| I09 Recovery | T01-04 (preserve-before-repair), T01-05 (verified backup/restore) | None |
| I10 Portability | T01-02 (published format), T01-05/06 (verified reconstruction) | None |
| I11 Bounds | Not explicitly detailed per-task above; every T01-0x task inherits §27's "Normal inputs" bounds by default, but this addendum does not add a new bound-enforcement requirement beyond what already exists | **Named limitation**: T01-01..07's per-task acceptance criteria (canonical plan §34) do not individually call out I11 bound enforcement for the new store; implementers must still apply the existing §27 input ceilings to any new code path. Not a blocking gap for T00-02 (no code exists yet to check), but flagged for T01-02/T01-03 implementation review. |
| I12 Claims | `dependency-admission.md` treats SHA-256 hashes as integrity evidence only, consistent with I12; no identity/authentication claim made from any hash in this addendum | None |

## Security model alignment (S03-S10, canonical plan §20/22)

`mutator-inventory.md`'s findings map directly onto the security gap table already in the canonical plan (§7, §22 S03/S06/S09 rows) rather than introducing a new threat class:

- S03 (filesystem escape/relocation) → T01-01 root/control-dir handle validation.
- S06 (unapproved state change) → the three confirmed bypasses in `mutator-inventory.md` (`open_read` metadata creation, pre-lock `open_write` repair, unbound `EventLog::append`) are exactly S06 instances; T01-01 closes them.
- S09 (supply chain/distribution) → `dependency-admission.md` license/advisory check.

No new security boundary is introduced that lacks a `V09` line in `checklist.md`.

## Old Spec 002 consistency (non-regression check)

Compared against the historical `specs/002-post-r1-canonical-core-convergence/checklist.md` and `tasks.md` (T037-T083, closed): this addendum does not reopen, re-tick, or contradict any historical checkbox. The historical record's own final state (`SPEC_002_STATUS` closed at `b03b7a8`, per `AGENTS.md`-referenced closeout commit) is treated as an input fact, not something this addendum re-derives. `FLAKE_CORRECTIVE_PLANNING_CONTRACT.md`'s table ("Existing concern → New owning task → Required result") is reproduced faithfully in `spec.md` §1 without alteration of its wording's substance.

## Verification hierarchy coverage (canonical plan §29)

Every `checklist.md` per-task line names its `V0x` gates; cross-checked against canonical plan §34's own "Tests" field per task — no `V0x` was added or dropped relative to the canonical plan's own contract. `V04` (end-to-end) and `V11`-`V15` (packaging/native/performance/manual-UX/independent-reproduction) are correctly absent from T01-01..06 (no desktop, no release packaging, no formal performance gate yet) and correctly present only where the canonical plan itself names them (`V12` at T01-04 recovery, `V10` at T01-02/T01-06 compatibility/migration).

## Findings requiring no stop

- I11 bound-enforcement callout above (documentation gap, not a code gap — nothing to remediate before `T01-01` starts; flagged for implementation-time attention).
- The open (non-blocking) build-script admission-record question in `clarify.md`/`dependency-admission.md`.

## Conclusion

No cross-artifact drift found between this addendum and the canonical build plan, the corrective planning contract, or the historical Spec 002 record. `T00-02` may close; `T01-01` is dependency-ready.
