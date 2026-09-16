import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { DecisionSummary, DecisionBasis, DecisionVerification, DecisionLifecycle } from "./types";
import { useConfirm } from "./Confirm";

const LIFECYCLE_LABEL: Record<DecisionLifecycle, string> = {
  draft: "Draft",
  accepted: "Accepted",
  superseded: "Superseded",
  withdrawn: "Withdrawn",
};

export function DecisionsPanel({ vaultPath, projectId }: { vaultPath: string; projectId: string }) {
  const [decisions, setDecisions] = useState<DecisionSummary[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [newKey, setNewKey] = useState("");
  const [newStatement, setNewStatement] = useState("");
  const [newBasis, setNewBasis] = useState<DecisionBasis>("user_judgment");
  const [newVerification, setNewVerification] = useState<DecisionVerification>("unreviewed");
  const [supersedeSourceId, setSupersedeSourceId] = useState<string | null>(null);
  const { requestConfirm, confirmDialog } = useConfirm();

  async function refresh() {
    try {
      const list = await invoke<DecisionSummary[]>("list_decisions", { vaultPath, projectId });
      setDecisions(list);
    } catch (e) {
      setError(String(e));
    }
  }

  useEffect(() => {
    void refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [vaultPath, projectId]);

  async function handleCreate() {
    if (!newKey.trim() || !newStatement.trim()) return;
    try {
      await invoke("decision_create", {
        vaultPath,
        projectId,
        key: newKey.trim(),
        statement: newStatement.trim(),
        basis: newBasis,
        verification: newVerification,
        extra: {},
      });
      setNewKey("");
      setNewStatement("");
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleAccept(d: DecisionSummary) {
    try {
      await invoke("decision_accept", { vaultPath, decisionId: d.id, expectedRevisionId: d.revision_id });
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleWithdraw(d: DecisionSummary) {
    try {
      await invoke("decision_withdraw", {
        vaultPath,
        decisionId: d.id,
        expectedRevisionId: d.revision_id,
        reason: "withdrawn by owner",
      });
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleSupersede(oldDecision: DecisionSummary, newDecisionId: string) {
    try {
      await invoke("decision_supersede", {
        vaultPath,
        newDecisionId,
        oldDecisionId: oldDecision.id,
        expectedOldRevisionId: oldDecision.revision_id,
        reason: "superseded by owner",
      });
      setSupersedeSourceId(null);
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <div className="decisions-panel">
      {confirmDialog}
      <h3>Decisions</h3>
      {decisions.length === 0 ? (
        <p>No decisions yet.</p>
      ) : (
        <ul>
          {decisions.map((d) => (
            <li key={d.id}>
              <strong>{d.decision_key}</strong>: {d.statement} -- <span className="state-label">{LIFECYCLE_LABEL[d.lifecycle]}</span>
              <div className="decision-transitions">
                {d.lifecycle === "draft" && (
                  <button onClick={() => void handleAccept(d)}>Accept</button>
                )}
                {d.lifecycle === "accepted" && (
                  <>
                    <button
                      onClick={() =>
                        requestConfirm(`Withdraw decision "${d.decision_key}: ${d.statement}"?`, () =>
                          void handleWithdraw(d)
                        )
                      }
                    >
                      Withdraw
                    </button>
                    {supersedeSourceId === d.id ? (
                      <SupersedePicker
                        candidates={decisions.filter((c) => c.decision_key === d.decision_key && c.id !== d.id && c.lifecycle === "accepted")}
                        onPick={(newId) =>
                          requestConfirm(`Supersede "${d.decision_key}: ${d.statement}" with the selected decision?`, () =>
                            void handleSupersede(d, newId)
                          )
                        }
                        onCancel={() => setSupersedeSourceId(null)}
                      />
                    ) : (
                      <button onClick={() => setSupersedeSourceId(d.id)}>Supersede with...</button>
                    )}
                  </>
                )}
              </div>
            </li>
          ))}
        </ul>
      )}
      <label>
        Decision key
        <input value={newKey} onChange={(e) => setNewKey(e.target.value)} />
      </label>
      <label>
        Statement
        <input value={newStatement} onChange={(e) => setNewStatement(e.target.value)} />
      </label>
      <label>
        Basis
        <select value={newBasis} onChange={(e) => setNewBasis(e.target.value as DecisionBasis)}>
          <option value="user_judgment">User judgment</option>
          <option value="evidence">Evidence</option>
          <option value="agent_proposal">Agent proposal</option>
        </select>
      </label>
      <label>
        Verification
        <select value={newVerification} onChange={(e) => setNewVerification(e.target.value as DecisionVerification)}>
          <option value="unreviewed">Unreviewed</option>
          <option value="user_reviewed">User reviewed</option>
        </select>
      </label>
      <button onClick={() => void handleCreate()} disabled={!newKey.trim() || !newStatement.trim()}>
        Create decision
      </button>
      {error && <p className="error">{error}</p>}
    </div>
  );
}

function SupersedePicker({
  candidates,
  onPick,
  onCancel,
}: {
  candidates: DecisionSummary[];
  onPick: (id: string) => void;
  onCancel: () => void;
}) {
  const [selected, setSelected] = useState(candidates[0]?.id ?? "");
  if (candidates.length === 0) {
    return (
      <span>
        No other accepted decision shares this key yet.
        <button onClick={onCancel}>Cancel</button>
      </span>
    );
  }
  return (
    <span>
      <select
        aria-label="Decision to supersede with"
        value={selected}
        onChange={(e) => setSelected(e.target.value)}
      >
        {candidates.map((c) => (
          <option key={c.id} value={c.id}>
            {c.statement}
          </option>
        ))}
      </select>
      <button onClick={() => onPick(selected)} disabled={!selected}>
        Confirm supersession
      </button>
      <button onClick={onCancel}>Cancel</button>
    </span>
  );
}
