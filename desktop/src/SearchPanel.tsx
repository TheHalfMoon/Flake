import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { SearchHit } from "./types";

const KIND_LABEL: Record<SearchHit["kind"], string> = {
  note: "Note",
  action: "Action",
  decision: "Decision",
};

export function SearchPanel({ vaultPath, projectId }: { vaultPath: string; projectId: string }) {
  const [query, setQuery] = useState("");
  const [hits, setHits] = useState<SearchHit[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function runSearch() {
    setError(null);
    try {
      const result = await invoke<SearchHit[]>("search_project", { vaultPath, projectId, query });
      setHits(result);
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <div className="search-panel">
      <h3>Search</h3>
      <label>
        Query
        <input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") void runSearch();
          }}
        />
      </label>
      <button onClick={() => void runSearch()} disabled={!query.trim()}>
        Search
      </button>
      {error && <p className="error">{error}</p>}
      {hits !== null && (
        hits.length === 0 ? (
          <p>No matches.</p>
        ) : (
          <ul>
            {hits.map((h) => (
              <li key={`${h.kind}-${h.id}`}>
                <span className="state-label">{KIND_LABEL[h.kind]}</span>{" "}
                <strong>{h.title || "(untitled)"}</strong>: {h.snippet}
              </li>
            ))}
          </ul>
        )
      )}
    </div>
  );
}
