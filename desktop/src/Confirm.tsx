import { useState } from "react";

// S06: "confirmations name concrete changes." Every call site passes the
// exact object/change being confirmed into `message` -- never a generic
// "Are you sure?" -- and Core still separately revalidates the expected
// revision/ownership on the actual mutation regardless of what this dialog
// shows (this is a UI courtesy, never itself an authority boundary).
export function useConfirm() {
  const [pending, setPending] = useState<{ message: string; onConfirm: () => void } | null>(null);

  function requestConfirm(message: string, onConfirm: () => void) {
    setPending({ message, onConfirm });
  }

  const dialog = pending ? (
    <div className="confirm-overlay" role="alertdialog" aria-modal="true">
      <div className="confirm-box">
        <p>{pending.message}</p>
        <div className="confirm-actions">
          <button
            autoFocus
            onClick={() => {
              const action = pending.onConfirm;
              setPending(null);
              action();
            }}
          >
            Confirm
          </button>
          <button onClick={() => setPending(null)}>Cancel</button>
        </div>
      </div>
    </div>
  ) : null;

  return { requestConfirm, confirmDialog: dialog };
}
