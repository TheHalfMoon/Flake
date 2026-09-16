import { useEffect, useState } from "react";
import type { KeyboardEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { MarkdownPreview } from "./markdown";

export interface NoteInfo {
  id: string;
  project_id: string;
  title: string | null;
  body: string;
  revision_id: string;
}

type SaveState = "saved" | "unsaved" | "saving" | "not-saved-conflict" | "not-saved-error" | "outcome-unknown";

const CONFLICT_MARKER = "expected revision conflict";

function saveStateLabel(state: SaveState): string {
  switch (state) {
    case "saved":
      return "Saved";
    case "unsaved":
      return "Unsaved";
    case "saving":
      return "Saving…";
    case "not-saved-conflict":
      return "Not saved — someone else changed this note first";
    case "not-saved-error":
      return "Not saved — save failed";
    case "outcome-unknown":
      return "Outcome unknown — check before retrying";
  }
}

export function NoteEditor({
  vaultPath,
  projectId,
  note,
  onSaved,
}: {
  vaultPath: string;
  projectId: string;
  note: NoteInfo | null;
  onSaved: (note: NoteInfo) => void;
}) {
  // `committed` is the last known-durable state (what Core actually has).
  // `title`/`body` are the live, possibly-unsaved editing buffer -- T04-02's
  // "keep unsaved buffer distinct from last durable revision" requirement.
  // On a failed save the buffer is never overwritten from `committed` or
  // from any stale server response -- it is only ever set by the owner's
  // own typing or by a successful save's own echoed result.
  const [committed, setCommitted] = useState<NoteInfo | null>(note);
  const [title, setTitle] = useState(note?.title ?? "");
  const [body, setBody] = useState(note?.body ?? "");
  const [saveState, setSaveState] = useState<SaveState>("saved");
  const [errorDetail, setErrorDetail] = useState<string | null>(null);
  const [showPreview, setShowPreview] = useState(false);

  useEffect(() => {
    setCommitted(note);
    setTitle(note?.title ?? "");
    setBody(note?.body ?? "");
    setSaveState("saved");
    setErrorDetail(null);
  }, [note?.id]);

  function markDirty(nextTitle: string, nextBody: string) {
    setTitle(nextTitle);
    setBody(nextBody);
    if (saveState !== "saving") setSaveState("unsaved");
  }

  // `against` names exactly which committed state (id + expected revision)
  // to save over. Never read from the `committed` state variable directly
  // inside this function -- a caller that just adopted a newer revision
  // (see `overwriteWithMyBuffer`) would otherwise race React's asynchronous
  // state update and save against the stale revision it just replaced.
  async function performSave(against: NoteInfo | null) {
    if (saveState === "saving") return; // no concurrent double-submit
    setSaveState("saving");
    setErrorDetail(null);
    try {
      let result: NoteInfo;
      if (against) {
        result = await invoke<NoteInfo>("note_update", {
          noteId: against.id,
          expectedRevisionId: against.revision_id,
          title: title.trim() ? title : null,
          body,
        });
      } else {
        result = await invoke<NoteInfo>("note_create", {
          projectId,
          title: title.trim() ? title : null,
          body,
        });
      }
      setCommitted(result);
      setSaveState("saved");
      onSaved(result);
    } catch (e) {
      const message = String(e);
      if (message.includes(CONFLICT_MARKER)) {
        setSaveState("not-saved-conflict");
      } else {
        setSaveState("not-saved-error");
      }
      setErrorDetail(message);
      // Buffer (title/body) is deliberately left exactly as the owner typed
      // it -- never reset, never overwritten with the rejected request or
      // any other party's data. Focus is never moved programmatically here,
      // so the textarea keeps focus on failure.
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "s") {
      e.preventDefault();
      void performSave(committed);
    }
  }

  async function fetchLatestRevisionId(): Promise<string | null> {
    const notes = await invoke<NoteInfo[]>("list_notes", { vaultPath, projectId });
    return notes.find((n) => n.id === committed?.id)?.revision_id ?? null;
  }

  async function reloadCommittedAndDiscardBuffer() {
    // Explicit "review options" action for a conflict, option A: adopt the
    // latest durable revision as-is, discarding the local unsaved buffer.
    // Never happens automatically or silently.
    if (!committed) return;
    const notes = await invoke<NoteInfo[]>("list_notes", { vaultPath, projectId });
    const latest = notes.find((n) => n.id === committed.id);
    if (!latest) return;
    setCommitted(latest);
    setTitle(latest.title ?? "");
    setBody(latest.body);
    setSaveState("saved");
    setErrorDetail(null);
  }

  async function overwriteWithMyBuffer() {
    // Explicit "review options" action for a conflict, option B: keep my
    // own unsaved buffer exactly as typed, adopt only the latest revision
    // id, and save over the other party's revision. An explicit,
    // owner-chosen override -- never a silent last-writer-wins.
    if (saveState === "saving") return;
    const latestRevisionId = await fetchLatestRevisionId().catch(() => null);
    if (!latestRevisionId || !committed) {
      setSaveState("not-saved-error");
      setErrorDetail("Could not read the latest revision to save over. Nothing was overwritten.");
      return;
    }
    await performSave({ ...committed, revision_id: latestRevisionId });
  }

  return (
    <div className="note-editor" onKeyDown={handleKeyDown}>
      <div className="note-editor-toolbar">
        <span className={`save-state save-state-${saveState}`} role="status" aria-live="polite">
          {saveStateLabel(saveState)}
        </span>
        <button onClick={() => void performSave(committed)} disabled={saveState === "saving"}>
          Save
        </button>
        <button onClick={() => setShowPreview((v) => !v)}>{showPreview ? "Edit" : "Preview"}</button>
      </div>
      <label>
        Title
        <input value={title} onChange={(e) => markDirty(e.target.value, body)} />
      </label>
      {showPreview ? (
        <MarkdownPreview text={body} />
      ) : (
        <textarea
          className="note-body"
          aria-label="Note body"
          value={body}
          onChange={(e) => markDirty(title, e.target.value)}
          rows={16}
        />
      )}
      {(saveState === "not-saved-conflict" || saveState === "not-saved-error") && (
        <div className="save-error" role="alert">
          <p>{errorDetail}</p>
          {saveState === "not-saved-conflict" && (
            <div className="conflict-review">
              <p>Someone else saved a newer revision of this note first. Your edits above are preserved, unsaved.</p>
              <button onClick={() => void reloadCommittedAndDiscardBuffer()}>
                Discard my edits and load the latest saved version
              </button>
              <button onClick={() => void overwriteWithMyBuffer()}>Keep my edits and save over theirs</button>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
