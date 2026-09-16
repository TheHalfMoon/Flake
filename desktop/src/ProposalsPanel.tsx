import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ProposalEntry, ProposedOperation } from "./types";
import { useConfirm } from "./Confirm";

const STATUS_LABEL: Record<string, string> = {
  pending: "Pending",
  accepted: "Accepted",
  rejected: "Rejected",
  expired: "Expired",
};

function operationSummary(op: ProposedOperation): string {
  switch (op.kind) {
    case "note_edit":
      return `Edit note ${op.note_id}: "${op.body.slice(0, 60)}"`;
    case "draft_decision":
      return `Draft decision "${op.decision_key}": ${op.statement}`;
    case "evidence_relation":
      return `Relation (${op.relation_type}) ${op.from_object_id} -> ${op.to_object_id}`;
    case "complete_action":
      return `Complete action ${op.action_id}: ${op.summary}`;
  }
}

// S01/S02/S06/S08: this panel renders every field as plain text (no
// dangerouslySetInnerHTML, same as the note Markdown preview) so an
// imported proposal's own content can never impersonate this owner UI or
// execute as anything but inert text -- and the claimed identity
// (declared_agent/model/tool, §17: "a declaration, never verified") is
// always shown, never hidden or silently trusted.
export function ProposalsPanel({ vaultPath, projectId }: { vaultPath: string; projectId: string }) {
  const [proposals, setProposals] = useState<ProposalEntry[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [rawText, setRawText] = useState("");
  const [selectedByProposal, setSelectedByProposal] = useState<Record<string, Set<number>>>({});
  const { requestConfirm, confirmDialog } = useConfirm();

  async function refresh() {
    try {
      const list = await invoke<ProposalEntry[]>("list_proposals", { vaultPath, projectId });
      setProposals(list);
    } catch (e) {
      setError(String(e));
    }
  }

  useEffect(() => {
    void refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [vaultPath, projectId]);

  async function handleAdmit() {
    if (!rawText.trim()) return;
    setError(null);
    try {
      await invoke("proposal_admit", { vaultPath, projectId, rawText });
      setRawText("");
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  function toggleSelected(proposalId: string, index: number) {
    setSelectedByProposal((prev) => {
      const current = new Set(prev[proposalId] ?? []);
      if (current.has(index)) current.delete(index);
      else current.add(index);
      return { ...prev, [proposalId]: current };
    });
  }

  async function handleAccept(p: ProposalEntry) {
    const indices = Array.from(selectedByProposal[p.id] ?? []).sort((a, b) => a - b);
    if (indices.length === 0) return;
    try {
      await invoke("proposal_accept", {
        vaultPath,
        proposalId: p.id,
        expectedRevisionId: p.revision_id,
        selectedIndices: indices,
      });
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleReject(p: ProposalEntry) {
    try {
      await invoke("proposal_reject", {
        vaultPath,
        proposalId: p.id,
        expectedRevisionId: p.revision_id,
        reason: "rejected by owner",
      });
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <div className="proposals-panel">
      {confirmDialog}
      <h3>Agent proposal review</h3>
      {proposals.length === 0 ? (
        <p>No proposals yet.</p>
      ) : (
        <ul>
          {proposals.map((p) => (
            <li key={p.id}>
              <p>
                <span className="state-label">{STATUS_LABEL[p.status]}</span> claimed identity: agent=
                {p.declared_agent ?? "Unknown"}, model={p.declared_model ?? "Unknown"}, tool=
                {p.declared_tool ?? "Unknown"}
              </p>
              <ul>
                {p.operations.map((op, i) => (
                  <li key={i}>
                    {p.status === "pending" && (
                      <input
                        type="checkbox"
                        checked={selectedByProposal[p.id]?.has(i) ?? false}
                        onChange={() => toggleSelected(p.id, i)}
                        aria-label={`select operation ${i}`}
                      />
                    )}
                    {operationSummary(op)}
                  </li>
                ))}
              </ul>
              {p.status === "pending" && (
                <div className="proposal-transitions">
                  <button
                    onClick={() =>
                      requestConfirm(
                        `Accept ${selectedByProposal[p.id]?.size ?? 0} selected operation(s) from this proposal (claimed agent: ${p.declared_agent ?? "Unknown"})?`,
                        () => void handleAccept(p)
                      )
                    }
                    disabled={(selectedByProposal[p.id]?.size ?? 0) === 0}
                  >
                    Accept selected
                  </button>
                  <button
                    onClick={() =>
                      requestConfirm("Reject this entire proposal?", () => void handleReject(p))
                    }
                  >
                    Reject
                  </button>
                </div>
              )}
            </li>
          ))}
        </ul>
      )}

      <h4>Admit a new proposal</h4>
      <label>
        Paste the proposal's raw JSON text (received out of band)
        <textarea className="note-body" value={rawText} onChange={(e) => setRawText(e.target.value)} rows={8} />
      </label>
      <button onClick={() => void handleAdmit()} disabled={!rawText.trim()}>
        Admit proposal
      </button>

      {error && <p className="error">{error}</p>}
    </div>
  );
}
