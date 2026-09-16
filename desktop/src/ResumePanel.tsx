import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ResumeView } from "./types";
import { outcomeLabel } from "./types";

function str(rec: Record<string, unknown>, key: string): string {
  const v = rec[key];
  return typeof v === "string" ? v : "";
}

// §12's own priority order, unchanged: conflicts and stale evidence
// precede reassuring summaries -- this component renders exactly the
// section order `ResumeView`'s own field order already encodes, never
// re-sorted for presentation.
export function ResumePanel({ vaultPath, projectId }: { vaultPath: string; projectId: string }) {
  const [view, setView] = useState<ResumeView | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function load() {
    setError(null);
    try {
      const result = await invoke<ResumeView>("resume_view", { vaultPath, projectId });
      setView(result);
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <div className="resume-panel">
      <h3>Resume / history</h3>
      <button onClick={() => void load()}>
        {view ? "Refresh resume view" : "Load resume view"}
      </button>
      {error && <p className="error">{error}</p>}
      {view && (
        <div>
          <p className="notice">
            Snapshot at recorded sequence {view.head_seq}
            {view.reviewed_through_seq !== null
              ? ` -- reviewed through ${view.reviewed_through_seq}`
              : " -- never marked reviewed"}
          </p>

          {view.conflicts.length > 0 && (
            <section>
              <h4>Conflicts (needs review)</h4>
              <ul>
                {view.conflicts.map((c) => (
                  <li key={c.decision_key}>
                    <strong>{c.decision_key}</strong>: {outcomeLabel(c.outcome)}
                  </li>
                ))}
              </ul>
            </section>
          )}

          {view.stale_or_missing_evidence.length > 0 && (
            <section>
              <h4>Stale or missing evidence</h4>
              <ul>
                {view.stale_or_missing_evidence.map((s) => (
                  <li key={s.source_id}>
                    {s.source_id}: {s.latest_check.status}
                  </li>
                ))}
              </ul>
            </section>
          )}

          <section>
            <h4>Current decisions</h4>
            {view.current_decisions.length === 0 ? (
              <p>None.</p>
            ) : (
              <ul>
                {view.current_decisions.map((c) => (
                  <li key={c.decision_key}>
                    <strong>{c.decision_key}</strong>: {outcomeLabel(c.outcome)}
                  </li>
                ))}
              </ul>
            )}
          </section>

          <section>
            <h4>Next actions</h4>
            {view.next_actions.length === 0 ? (
              <p>None open.</p>
            ) : (
              <ul>
                {view.next_actions.map(([id, a]) => (
                  <li key={id}>
                    {str(a, "title")} -- <span className="state-label">{str(a, "state")}</span>
                  </li>
                ))}
              </ul>
            )}
          </section>

          <section>
            <h4>Relevant notes</h4>
            {view.relevant_notes.length === 0 ? (
              <p>None changed since the last checkpoint.</p>
            ) : (
              <ul>
                {view.relevant_notes.map(([id, n]) => (
                  <li key={id}>{str(n, "title") || "(untitled)"}</li>
                ))}
              </ul>
            )}
          </section>

          <section>
            <h4>Changes since checkpoint</h4>
            {view.changes_since_checkpoint.length === 0 ? (
              <p>None.</p>
            ) : (
              <ul>
                {view.changes_since_checkpoint.map((c) => (
                  <li key={`${c.recorded_seq}-${c.object_id}`}>
                    #{c.recorded_seq} {c.kind} {c.object_id}
                  </li>
                ))}
              </ul>
            )}
          </section>
        </div>
      )}
    </div>
  );
}
