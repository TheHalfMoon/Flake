# Founder decision — remove the T04-06 human accessibility witness gate

Date: 2026-09-16
Decision class: E — product thesis / founder direction
Status: ADOPTED
Applies to: Flake v1 `T04-06` ("Qualify native usability and accessibility of the complete desktop") only

## Authority

The founder explicitly directed: T04-06 must not stop for lack of a human accessibility reviewer/tester, and the existing `docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md` decision is prospectively expanded to cover T04-06's own witnessed-human-checklist component as well.

Under `AGENTS.md`, a Class E change requires founder authorization plus architecture reconsideration. This document is that additive record for T04-06 specifically. It does not rewrite the canonical plan, this task's own original wording, or any accepted result. It supersedes only the human-witness clauses named below — every other part of T04-06's contract (three native platforms, the full deterministic scripted flow, every accessibility *property* the task names) remains in force unchanged.

## Original T04-06 wording (preserved, not rewritten)

- **Verification method:** "Native test automation where supported plus witnessed manual checklist with exact builds and expected/observed states."
- **Files/components:** "... accessibility/IME checklists, screenshots/video where consented ..."
- **Tests:** "manual checks recorded separately from automated desktop assertions."
- **Cross-platform gate:** "All three native profiles mandatory for this exit."

Read literally, this requires a live human operator physically present to run a screen reader, type real IME composition sequences, and sign off on expected-versus-observed state, on Windows, macOS, and Linux. No such operator is present in this execution context, and none is expected to become available — this is a standing structural fact about how this task is being executed, not a one-off scheduling gap.

## Founder ruling

```text
HUMAN_ACCESSIBILITY_REVIEW_REQUIRED=NO
HUMAN_ACCESSIBILITY_EVIDENCE_CLAIMED=NO
NATIVE_PLATFORM_PROFILES_REQUIRED=WINDOWS,MACOS,LINUX
ACCESSIBILITY_GATE=AUTOMATED_TECHNICAL_QUALIFICATION
```

**What this removes**: the requirement that a human operator physically run a screen reader session, type real IME composition, or sign a witnessed checklist before T04-06 can close.

**What this does not remove**: the three-native-platform requirement (Windows, macOS, Linux all remain mandatory — this decision does not reduce platform scope, and no separate founder decision has removed macOS/Linux from supported v1 scope), or the substance of any accessibility property T04-06 names. Every property the human checklist would have exercised must still be established by a reproducible, auditable, automated or native-test-automation means wherever one exists:

- Native build, launch, and IPC/Core-canonical-result parity with the CLI oracle, on each of the three platforms, using already-authorized infrastructure (GitHub-hosted native runners for each OS, not only this session's own Windows workstation).
- Keyboard-only navigation and focus order/visibility, checked by automation or static analysis of the actual shipped source (e.g., no explicit `tabIndex` disruption, every interactive control a native semantic element).
- Semantic roles/names/states, checked via automated accessibility-tree inspection where the platform/tooling supports it, or via static analysis of the shipped markup otherwise.
- 200% zoom / layout robustness, checked via automated rendering checks or static analysis for fixed-pixel-width violations where full automation is not practical.
- Unicode and IME-*relevant* behavior, checked via the same automated, non-human mechanism `T04-02`'s own manual-composition checklist already distinguished from IME's underlying *implementation correctness* (a non-interfering, uncontrolled-value-preserving native text field) — real human IME composition itself remains outside what any automated harness can drive, exactly as `T04-02`'s evidence already recorded, and is not fabricated here either.
- Reduced-motion/system-theme behavior, no network dependency, and no inaccessible custom control, all checked automatically.

**Where an exact human-only behavior genuinely cannot be technically reproduced** (a live screen-reader's actual spoken announcement; a human's own IME composition keystrokes), this is recorded as a documented, bounded limitation in the task's evidence report — never fabricated as observed, and never used to block the properties that *can* be technically established.

## Non-precedent statement

This decision resolves T04-06's own human-witness clause specifically. It does not, by itself, remove any other human-review or human-witness requirement elsewhere in the canonical plan that has not been separately named by a founder decision — each such case still requires its own explicit ruling under `AGENTS.md`'s Class C/D/E process. It also does not reduce the three-native-platform requirement; that remains fully in force and is not superseded by this document.

## Scope

Applies to `T04-06` only. Does not apply to `T03-08`/`T05-06`/`R11` (already covered by the prior decision) and does not extend automatically to any later task's own human-review language without its own explicit ruling.
