# Ponytail Gate — Spec 002 corrective addendum (T00-02 necessity check)

**Status:** PASS — 2026-09-14
**Spec:** `002-post-r1-canonical-core-convergence/corrective-t01`
**Method:** `docs/19-ENGINEERING-METHOD.md` §2 — requirement → reuse check → rights/security/benchmark/authorization.
**Cost:** `COST=ZERO` — no model API call, no paid service, no new heavy dependency.

## Necessity verdict

**KEEP** the T01-01..07 corrective slice as scoped. It protects the only truly irreplaceable state in the product: canonical history. `docs/19-ENGINEERING-METHOD.md` §2.1 places "canonical-data integrity," "recovery correctness," and "data-loss prevention" on the list Ponytail may never argue away — question 1 ("does this capability need to exist") is fixed at yes by the constitution for this class of work. The canonical plan's own gap-closure table (§7) independently derives the same three defects from source inspection, not from this addendum's own preference.

## Walking the gate for each new construct this addendum introduces

| New construct | Q1 need? | Q2 Fehrest already has it? | Q3 std/platform primitive? | Q4 already-approved dependency? | Q5 smaller alternative? | Verdict |
|---|---|---|---|---|---|---|
| Canonical SQLite store (`canonical.sqlite`) | Yes — fixed (data integrity) | No — current canonical layer is raw file/journal writes | No — no std primitive gives ACID multi-table transactions | **Yes** — `rusqlite`/`libsqlite3-sys` already locked and compiled in (used by `derived.rs`) | N/A — Q4 satisfied | Use the already-approved dependency; no new crate |
| OS-held writer lease enforcement | Yes — fixed (authorization boundary) | Partially — `Vault::has_write_lock`/`writer()` exist; the gap is bypass closure, not new construction | No | N/A | Tighten existing construct rather than replace it | Extend existing `VaultWriter`, do not rewrite |
| Forensic preserve-before-repair copy | Yes — fixed (recovery correctness) | No — current recovery normalizes in place | Yes — plain byte copy via std `fs`/platform copy primitives | N/A | Smallest correct form is a straight byte-identical copy, no new framework | Std-only, no dependency |
| Fault-injection/child-process harness (T01-07) | Yes — fixed (invariant tests are explicitly excluded from minimization) | No prior harness exists in this exact form | Partially — Rust `std::process` is sufficient | N/A | Build the smallest harness that reaches ≥100 schedules per operation, no generic fuzzing framework needed for this bounded scope | New, minimal, std-based |

No construct above required a new external dependency, a new architecture decision, or an ADR beyond the ones the canonical plan already recorded (ADR-0017). WAL mode, a different engine, or a custom write-ahead protocol were all explicitly considered and rejected at the Astro planning stage (canonical plan §6 final challenge summary) — this addendum does not reopen that decision, consistent with `docs/19-ENGINEERING-METHOD.md`'s framing that Ponytail governs *how*, never *whether*, on data-integrity paths.

## What Ponytail does not get to minimize here

Per `docs/19-ENGINEERING-METHOD.md` §2.1, this addendum explicitly refuses to treat the following as candidates for reduction, regardless of any future argument that they add code for a rare case:

- The ≥100-schedule fault-injection requirement in T01-07 (an invariant test class).
- The preserve-before-repair forensic copy in T01-04 (data-loss prevention).
- The OS-held writer/recovery-access lock ordering in T01-01 (an authorization boundary).
- The no-clobber, verify-before-publish discipline repeated in T01-02/T01-05/T01-06 (recovery correctness / data-loss prevention).

A future implementer proposing to skip or shrink any of these must stop the affected task and record the conflict, not reinterpret this gate.
