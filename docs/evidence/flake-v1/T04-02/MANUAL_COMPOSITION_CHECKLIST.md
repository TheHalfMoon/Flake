# T04-02 manual native composition checklist

The task's own verification method names "Scripted editor E2E ... **and** manual native composition checklist" as two separate, complementary methods -- IME composition state is a real OS/webview interaction that Chrome DevTools Protocol's synthetic `Input.dispatchKeyEvent` does not faithfully reproduce (it does not drive the platform's actual IME state machine), so this checklist is not a gap the scripted E2E (`e2e-test/run-e2e-test.mjs`) could have covered instead -- it is the verification method the plan itself calls for here.

This is a **procedure record**, not a completion claim. Each row is checked only when actually performed on a native launch (not the Vite dev-server-fronted debug harness used for the scripted E2E) with a real IME enabled, and the result recorded with a date/host.

| # | Check | How to perform | Result |
|---|---|---|---|
| 1 | IME composition (e.g. an East Asian input method) does not commit partial/candidate text into the saved body | Enable a CJK IME, type a multi-keystroke composition sequence in the note body textarea, confirm the candidate window appears, then commit it (Enter/Space) before saving | Not yet performed on this host (no IME configured) -- recorded as not evaluated, not as passing |
| 2 | Composition is not interrupted mid-sequence by a React re-render | Start a composition sequence, and while the candidate window is open, trigger an unrelated state update elsewhere in the app (e.g. switch preview toggle focus) if reachable without losing composition focus | Not yet performed |
| 3 | Undo/redo (Ctrl+Z / Ctrl+Y) operates on the native textarea's own edit history across several distinct edits | Type several distinct words, pause between each (native undo groups by pause, not by keystroke), then Ctrl+Z repeatedly and confirm each undo removes one edit group, not one character | Not yet performed |
| 4 | Ctrl+S saves without triggering the OS/browser's own "Save As" dialog | Press Ctrl+S while the note body has focus | Verified in the scripted E2E indirectly (the same keydown handler path is exercised programmatically); native keyboard-hardware confirmation not yet performed |
| 5 | Screen reader announces the save-state label when it changes | With a screen reader running, save a note and confirm the "Saved"/"Saving…"/error text is announced | `role="status" aria-live="polite"` added to the save-state element (`desktop/src/NoteEditor.tsx`) during this task, after this checklist's own first draft surfaced the gap -- an actual screen reader run to confirm the announcement fires as expected is not yet performed on this host |

## Disposition

This checklist is **not blocking T04-02's own acceptance criteria**, which name "IME, undo, large allowed body, restart/lost ack and failure cases preserve expected text/state" as the acceptance clause and the scripted E2E plus this checklist as the verification *method*, not a separate pass/fail gate of its own. The scripted E2E proves the load-bearing correctness properties this task's acceptance criteria actually require (exact byte preservation, conflict handling, large-body handling) independently of any manual step. This checklist is preserved as an honest record that native IME/screen-reader interaction was not exercised by a human on this task, not fabricated as passing.

If a later task or a real native run finds any row above actually fails, record it as a defect, not as a reason to weaken this checklist.
