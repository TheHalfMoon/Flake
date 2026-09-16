# Founder decision — remove the T05-02 physical-device power-loss trial gate

Date: 2026-09-16
Decision class: E — product thesis / founder direction
Status: ADOPTED
Applies to: Flake v1 `T05-02` ("Qualify durability, confinement and performance on every platform") only — specifically the D6 physical-hardware sub-clause

## Authority

The founder explicitly directed: T05-02 must not stop for lack of physical hardware to run controlled power-interruption trials, and the existing `docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md` / `docs/canonical/FOUNDER_T04-06_ACCESSIBILITY_WITNESS_AMENDMENT_2026-09-16.md` precedent is prospectively expanded to cover this task's own physical-device requirement as well.

Under `AGENTS.md`, a Class E change requires founder authorization plus architecture reconsideration. This document is that additive record for T05-02 specifically. It does not rewrite the canonical plan, this task's own original wording, or any accepted result. It supersedes only the physical-hardware clause named below — every other part of T05-02's contract (D1-D5, the 30-cycle native-VM-unclean-shutdown requirement, all three native platforms, section 27 performance, confinement) remains in force unchanged.

## Original T05-02 wording (preserved, not rewritten)

- **Durability evidence matrix (section 15):** "D6 native unclean shutdown and power-interruption trials on all three profiles... Physical device power interruption additionally requires at least 10 controlled disposable-data trials/profile or an externally obtained qualified lab report for that exact stack. VM shutdown and process kill are labeled separately; neither substitutes for physical power-loss evidence."
- **Implementation requirements:** "...30 native VM unclean shutdowns/profile, and 10 physical-device controlled trials/profile or an exact-stack qualified lab report."
- **Failure behavior:** "Missing hardware/report or failing maximum blocks qualification; no process-kill proxy for power-loss evidence."

Read literally, this requires either ten controlled trials of cutting real power to physical storage hardware per platform, or an externally obtained lab report qualifying that exact stack. No such hardware, lab access, or report is available in this execution context, and none is expected to become available — this is a standing structural fact about how this task is being executed, not a one-off scheduling gap.

## Founder ruling

```text
PHYSICAL_POWER_LOSS_TRIALS_REQUIRED=NO
PHYSICAL_POWER_LOSS_EVIDENCE_CLAIMED=NO
D6_GATE=AUTOMATED_UNCLEAN_SHUTDOWN_AND_STORAGE_FAULT_QUALIFICATION
RESIDUAL_PHYSICAL_HARDWARE_RISK=DOCUMENTED_NON_BLOCKING
```

**What this removes**: the requirement that qualification be blocked on ten controlled real physical-device power-interruption trials per platform, or an externally obtained qualified lab report for that exact stack.

**What this does not remove**: the 30-cycle native-VM-unclean-shutdown requirement (genuine forced/unclean VM termination, not a graceful shutdown and not an in-process kill), the full D1-D5 deterministic process-fault-schedule matrix (100+ schedules/operation), the three-native-platform requirement, section 27 performance qualification, or confinement/security qualification. No other T05-02 durability, recovery, integrity, or zero-false-success requirement is weakened by this decision.

This decision does **not** assert that VM/process/OS-crash simulation is equivalent to pulling power from real hardware. The assurance gap between "OS observed an unclean termination of the process/VM" and "storage media itself lost power mid-write, including firmware-level write-cache and controller behavior that no software-level fault injection can reach" is real and is recorded explicitly in this task's own evidence report, not glossed over.

**Required substitute evidence** (the strongest reproducible automated durability qualification available on already-authorized infrastructure):

- Process-kill fault injection at every meaningful commit/fsync boundary (D1-D5, already built and passing: `d1_commit_atomicity_fault_schedule_matrix`, `d2_short_write_fault_schedule_matrix`, `d3_competing_opens_fault_schedule_matrix`, `d4_bit_corruption_fault_schedule_matrix`, `d5_backup_cancellation_fault_schedule_matrix`, `d5_migration_interruption_fault_schedule_matrix`), run natively on all three platforms.
- Genuine forced VM unclean shutdown (not a graceful `shutdown`/`poweroff`, not an in-process kill) — at minimum 30 cycles per platform, using already-authorized infrastructure (this workstation's own Hyper-V, and GitHub-hosted-runner nested virtualization where the runner exposes it), covering interruption during canonical writes, SQLite commit/fsync boundaries, backup, restore, migration, and export/import.
- Torn/truncated/corrupted storage fixtures (D4) and stale/partial publication cases, exercised as deterministic byte-level fixtures independent of any live fault-injection timing.
- Post-restart verification proving zero acknowledged canonical loss and zero false-success recovery across the tested matrix, forensic preservation of the pre-recovery state, and exact, deterministic, repeatable fault schedules (fixed seeds), exactly as D1-D5 already establish.

**Recorded, not silently dropped**, in `docs/evidence/flake-v1/T05-02/REPORT.md`:

```text
WHAT_WAS_NOT_TESTED=10 controlled physical-device power-interruption trials/profile (real power cut to physical storage hardware mid-write)
WHY_IT_REQUIRES_PHYSICAL_HARDWARE=No controlled physical storage device with a switched power supply, and no externally obtained qualified lab report for this exact stack, is available in this execution context
WHAT_AUTOMATED_EVIDENCE_SUBSTITUTES_FOR_IT=D1-D5 deterministic process-fault-schedule matrices (100+ schedules/operation) on all three native platforms, plus >=30 genuine forced-unclean-shutdown native VM cycles/profile covering every named write boundary
WHAT_RESIDUAL_RISK_REMAINS=Storage-controller/firmware-level behavior under an actual mid-write power cut (write-cache reordering, torn sectors below the OS/filesystem's own visibility) is not exercised by any software-level fault injection, however faithful; this residual risk is explicitly non-blocking under this founder decision, not claimed as covered
```

## Non-precedent statement

This decision resolves T05-02's own physical-hardware clause specifically. It does not, by itself, remove any other human-review, human-witness, or physical-hardware requirement elsewhere in the canonical plan that has not been separately named by a founder decision — each such case still requires its own explicit ruling under `AGENTS.md`'s Class C/D/E process. It also does not reduce the three-native-platform requirement or the 30-cycle native-VM-unclean-shutdown requirement; both remain fully in force and are not superseded by this document.

## Scope

Applies to `T05-02` only. Does not apply to `T03-08`/`T04-06`/`T05-06`/`R11` (already covered by the prior decisions cited above) and does not extend automatically to any later task's own physical-hardware or human-review language without its own explicit ruling.
