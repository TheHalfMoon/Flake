import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { RelationEntry, RelationType } from "./types";

const TYPE_LABEL: Record<RelationType, string> = {
  supports: "Supports",
  contradicts: "Contradicts",
  depends_on: "Depends on",
  relates_to: "Relates to",
  supersedes: "Supersedes",
};

// Evidence linking (§15: "Evidence is a source revision/artifact plus a
// relation, not a second copy") -- this panel never stores a second copy
// of anything; it only creates/lists `Relation` objects pointing at
// already-existing object IDs.
export function RelationsPanel({
  vaultPath,
  projectId,
  objectId,
}: {
  vaultPath: string;
  projectId: string;
  objectId: string;
}) {
  const [relations, setRelations] = useState<RelationEntry[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [otherId, setOtherId] = useState("");
  const [direction, setDirection] = useState<"from" | "to">("from");
  const [relationType, setRelationType] = useState<RelationType>("relates_to");
  const [note, setNote] = useState("");

  async function refresh() {
    try {
      const list = await invoke<RelationEntry[]>("list_relations_for_object", { vaultPath, objectId });
      setRelations(list);
    } catch (e) {
      setError(String(e));
    }
  }

  useEffect(() => {
    void refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [vaultPath, objectId]);

  async function handleCreate() {
    if (!otherId.trim()) return;
    const from = direction === "from" ? objectId : otherId.trim();
    const to = direction === "from" ? otherId.trim() : objectId;
    try {
      await invoke("relation_create", {
        vaultPath,
        projectId,
        relationType,
        from,
        to,
        note: note.trim() ? note.trim() : null,
      });
      setOtherId("");
      setNote("");
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <div className="relations-panel">
      <h4>Evidence &amp; relations</h4>
      {relations.length === 0 ? (
        <p>No relations linked to this item yet.</p>
      ) : (
        <ul>
          {relations.map((r) => (
            <li key={r.id}>
              {r.from_object_id === objectId ? (
                <span>
                  This <strong>{TYPE_LABEL[r.relation_type]}</strong> {r.to_object_id}
                </span>
              ) : (
                <span>
                  {r.from_object_id} <strong>{TYPE_LABEL[r.relation_type]}</strong> this
                </span>
              )}
              {r.note ? <span> -- {r.note}</span> : null}
            </li>
          ))}
        </ul>
      )}
      <label>
        Other object's ID
        <input value={otherId} onChange={(e) => setOtherId(e.target.value)} placeholder="uuid" />
      </label>
      <label>
        Direction
        <select value={direction} onChange={(e) => setDirection(e.target.value as "from" | "to")}>
          <option value="from">This item -&gt; other</option>
          <option value="to">Other -&gt; this item</option>
        </select>
      </label>
      <label>
        Relation type
        <select value={relationType} onChange={(e) => setRelationType(e.target.value as RelationType)}>
          {Object.entries(TYPE_LABEL).map(([value, label]) => (
            <option key={value} value={value}>
              {label}
            </option>
          ))}
        </select>
      </label>
      <label>
        Note (optional)
        <input value={note} onChange={(e) => setNote(e.target.value)} />
      </label>
      <button onClick={() => void handleCreate()} disabled={!otherId.trim()}>
        Link
      </button>
      {error && <p className="error">{error}</p>}
    </div>
  );
}
