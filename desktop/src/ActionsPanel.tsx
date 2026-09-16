import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ActionSummary, ActionState } from "./types";
import { useConfirm } from "./Confirm";

const STATE_LABEL: Record<ActionState, string> = {
  open: "Open",
  doing: "Doing",
  blocked: "Blocked",
  done: "Done",
  cancelled: "Cancelled",
};

export function ActionsPanel({ vaultPath, projectId }: { vaultPath: string; projectId: string }) {
  const [actions, setActions] = useState<ActionSummary[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [newTitle, setNewTitle] = useState("");
  const [newBody, setNewBody] = useState("");
  const { requestConfirm, confirmDialog } = useConfirm();

  async function refresh() {
    try {
      const list = await invoke<ActionSummary[]>("list_actions", { vaultPath, projectId });
      setActions(list);
    } catch (e) {
      setError(String(e));
    }
  }

  useEffect(() => {
    void refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [vaultPath, projectId]);

  async function handleCreate() {
    if (!newTitle.trim()) return;
    try {
      await invoke("action_create", {
        vaultPath,
        projectId,
        title: newTitle.trim(),
        body: newBody.trim() ? newBody.trim() : null,
        dependsOn: [],
      });
      setNewTitle("");
      setNewBody("");
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  async function transition(cmd: string, extra: Record<string, unknown>) {
    try {
      await invoke(cmd, extra);
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <div className="actions-panel">
      {confirmDialog}
      <h3>Actions</h3>
      {actions.length === 0 ? (
        <p>No actions yet.</p>
      ) : (
        <ul>
          {actions.map((a) => (
            <li key={a.id}>
              <strong>{a.title}</strong> -- <span className="state-label">{STATE_LABEL[a.state]}</span>
              {a.completion_summary ? <span> ({a.completion_summary})</span> : null}
              <div className="action-transitions">
                {a.state === "open" && (
                  <button onClick={() => void transition("action_start", { vaultPath, actionId: a.id, expectedRevisionId: a.revision_id })}>
                    Start
                  </button>
                )}
                {(a.state === "open" || a.state === "doing") && (
                  <button
                    onClick={() =>
                      requestConfirm(`Block action "${a.title}"?`, () =>
                        void transition("action_block", {
                          vaultPath,
                          actionId: a.id,
                          expectedRevisionId: a.revision_id,
                          reason: "blocked by owner",
                        })
                      )
                    }
                  >
                    Block
                  </button>
                )}
                {(a.state === "open" || a.state === "doing" || a.state === "blocked") && (
                  <>
                    <button
                      onClick={() =>
                        requestConfirm(`Complete action "${a.title}"?`, () =>
                          void transition("action_complete", {
                            vaultPath,
                            actionId: a.id,
                            expectedRevisionId: a.revision_id,
                            summary: "completed by owner",
                            overrideReason: null,
                          })
                        )
                      }
                    >
                      Complete
                    </button>
                    <button
                      onClick={() =>
                        requestConfirm(`Cancel action "${a.title}"? This cannot be silently undone.`, () =>
                          void transition("action_cancel", {
                            vaultPath,
                            actionId: a.id,
                            expectedRevisionId: a.revision_id,
                            reason: "cancelled by owner",
                          })
                        )
                      }
                    >
                      Cancel
                    </button>
                  </>
                )}
                {(a.state === "done" || a.state === "cancelled") && (
                  <button
                    onClick={() =>
                      requestConfirm(`Reopen action "${a.title}"?`, () =>
                        void transition("action_reopen", {
                          vaultPath,
                          actionId: a.id,
                          expectedRevisionId: a.revision_id,
                          reason: "reopened by owner",
                        })
                      )
                    }
                  >
                    Reopen
                  </button>
                )}
              </div>
            </li>
          ))}
        </ul>
      )}
      <label>
        New action title
        <input value={newTitle} onChange={(e) => setNewTitle(e.target.value)} />
      </label>
      <label>
        Body (optional)
        <input value={newBody} onChange={(e) => setNewBody(e.target.value)} />
      </label>
      <button onClick={() => void handleCreate()} disabled={!newTitle.trim()}>
        Create action
      </button>
      {error && <p className="error">{error}</p>}
    </div>
  );
}
