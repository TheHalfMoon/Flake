import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { SourceEntry } from "./types";

const STATUS_LABEL: Record<string, string> = {
  match: "Match",
  changed: "Changed",
  missing: "Missing",
  denied: "Denied",
};

export function SourcesPanel({ vaultPath, projectId }: { vaultPath: string; projectId: string }) {
  const [sources, setSources] = useState<SourceEntry[]>([]);
  const [error, setError] = useState<string | null>(null);

  async function refresh() {
    try {
      const list = await invoke<SourceEntry[]>("list_sources", { vaultPath, projectId });
      setSources(list);
    } catch (e) {
      setError(String(e));
    }
  }

  useEffect(() => {
    void refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [vaultPath, projectId]);

  async function handleCheckNow(sourceId: string) {
    try {
      await invoke("source_check_now", { vaultPath, sourceId });
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <div className="sources-panel">
      <h3>Sources</h3>
      {sources.length === 0 ? (
        <p>No sources yet.</p>
      ) : (
        <ul>
          {sources.map((s) => (
            <li key={s.id}>
              <strong>{s.label}</strong>
              {!s.active && " (deactivated)"} --{" "}
              <span className="state-label">
                {s.latest_check_status ? STATUS_LABEL[s.latest_check_status] : "Unchecked"}
              </span>
              {s.latest_check_at ? ` (as of ${s.latest_check_at})` : ""}{" "}
              <button onClick={() => void handleCheckNow(s.id)}>Check now</button>
            </li>
          ))}
        </ul>
      )}
      {error && <p className="error">{error}</p>}
    </div>
  );
}
