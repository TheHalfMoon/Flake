import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { NoteEditor, type NoteInfo } from "./NoteEditor";
import { ActionsPanel } from "./ActionsPanel";
import { DecisionsPanel } from "./DecisionsPanel";
import { RelationsPanel } from "./RelationsPanel";
import { ResumePanel } from "./ResumePanel";
import { SearchPanel } from "./SearchPanel";
import { useConfirm } from "./Confirm";
import type { ProjectSummary } from "./types";
import "./App.css";

interface VaultInfo {
  path: string;
  vault_id: string;
}

type Tab = "notes" | "actions" | "decisions" | "search" | "resume";

async function pickDirectory(): Promise<string | null> {
  return invoke<string | null>("pick_directory");
}

export default function App() {
  const [vault, setVault] = useState<VaultInfo | null>(null);
  const [projects, setProjects] = useState<ProjectSummary[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [newVaultName, setNewVaultName] = useState("");
  const [newProjectName, setNewProjectName] = useState("");
  const [newProjectDescription, setNewProjectDescription] = useState("");
  const [openProject, setOpenProject] = useState<ProjectSummary | null>(null);
  const [notes, setNotes] = useState<NoteInfo[]>([]);
  const [selectedNoteId, setSelectedNoteId] = useState<string | "new" | null>(null);
  const [tab, setTab] = useState<Tab>("notes");
  const { requestConfirm, confirmDialog } = useConfirm();

  async function refreshNotes(vaultPath: string, projectId: string) {
    try {
      const list = await invoke<NoteInfo[]>("list_notes", { vaultPath, projectId });
      setNotes(list);
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleOpenProject(p: ProjectSummary) {
    if (!vault) return;
    setOpenProject(p);
    setSelectedNoteId(null);
    setTab("notes");
    await refreshNotes(vault.path, p.id);
  }

  async function refreshProjects(v: VaultInfo) {
    try {
      const list = await invoke<ProjectSummary[]>("list_projects", { vaultPath: v.path });
      setProjects(list);
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleCreate() {
    setError(null);
    try {
      const parent = await pickDirectory();
      if (!parent) return;
      if (!newVaultName.trim()) {
        setError("Choose a vault name first.");
        return;
      }
      const info = await invoke<VaultInfo>("vault_create", { parentDir: parent, name: newVaultName.trim() });
      setVault(info);
      await refreshProjects(info);
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleOpen() {
    setError(null);
    try {
      const path = await pickDirectory();
      if (!path) return;
      const info = await invoke<VaultInfo>("vault_open", { path });
      setVault(info);
      await refreshProjects(info);
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleRestore() {
    setError(null);
    try {
      const source = await pickDirectory();
      if (!source) return;
      const parent = await pickDirectory();
      if (!parent) return;
      if (!newVaultName.trim()) {
        setError("Choose a name for the restored vault first.");
        return;
      }
      const info = await invoke<VaultInfo>("vault_restore", {
        sourcePath: source,
        parentDir: parent,
        name: newVaultName.trim(),
      });
      setVault(info);
      await refreshProjects(info);
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleCreateProject() {
    if (!vault) return;
    setError(null);
    try {
      await invoke<ProjectSummary>("create_project", {
        vaultPath: vault.path,
        name: newProjectName.trim(),
        description: newProjectDescription.trim() || null,
      });
      setNewProjectName("");
      setNewProjectDescription("");
      await refreshProjects(vault);
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleArchiveToggle(p: ProjectSummary) {
    if (!vault) return;
    try {
      if (p.active) {
        await invoke("project_archive", { vaultPath: vault.path, projectId: p.id });
      } else {
        await invoke("project_unarchive", { vaultPath: vault.path, projectId: p.id });
      }
      await refreshProjects(vault);
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleNoteTombstoneToggle(n: NoteInfo, tombstoned: boolean) {
    if (!vault || !openProject) return;
    try {
      const cmd = tombstoned ? "note_untombstone" : "note_tombstone";
      await invoke(cmd, { vaultPath: vault.path, noteId: n.id, expectedRevisionId: n.revision_id });
      setSelectedNoteId(null);
      await refreshNotes(vault.path, openProject.id);
    } catch (e) {
      setError(String(e));
    }
  }

  if (!vault) {
    return (
      <main className="shell">
        <h1>Flake</h1>
        <p className="notice">Phase T -- experimental, not a product. Everything stays local and offline.</p>
        <label>
          Vault name (for Create / Restore)
          <input
            value={newVaultName}
            onChange={(e) => setNewVaultName(e.target.value)}
            placeholder="my-project-vault"
          />
        </label>
        <div className="actions">
          <button onClick={handleCreate}>Create new vault</button>
          <button onClick={handleOpen}>Open existing vault</button>
          <button onClick={handleRestore}>Restore from backup</button>
        </div>
        {error && <p className="error">{error}</p>}
      </main>
    );
  }

  if (openProject) {
    const selectedNote =
      selectedNoteId && selectedNoteId !== "new" ? notes.find((n) => n.id === selectedNoteId) ?? null : null;
    return (
      <main className="shell">
        {confirmDialog}
        <h1>Flake</h1>
        <button onClick={() => setOpenProject(null)}>&larr; Back to projects</button>
        <p className="notice">
          Project: {openProject.name}
          {openProject.description ? ` -- ${openProject.description}` : ""}
        </p>
        <nav className="tabs">
          {(["notes", "actions", "decisions", "search", "resume"] as Tab[]).map((t) => (
            <button
              key={t}
              className={tab === t ? "tab-active" : ""}
              onClick={() => setTab(t)}
              aria-current={tab === t}
            >
              {t[0].toUpperCase() + t.slice(1)}
            </button>
          ))}
        </nav>

        {tab === "notes" && (
          <>
            <section>
              <h2>Notes</h2>
              {notes.length === 0 ? (
                <p>No notes yet.</p>
              ) : (
                <ul>
                  {notes.map((n) => (
                    <li key={n.id}>
                      <button onClick={() => setSelectedNoteId(n.id)}>{n.title || "(untitled)"}</button>
                    </li>
                  ))}
                </ul>
              )}
              <button onClick={() => setSelectedNoteId("new")}>New note</button>
            </section>
            {(selectedNoteId === "new" || selectedNote) && (
              <section>
                <NoteEditor
                  vaultPath={vault.path}
                  projectId={openProject.id}
                  note={selectedNote}
                  onSaved={(saved) => {
                    setNotes((prev) => {
                      const exists = prev.some((n) => n.id === saved.id);
                      return exists ? prev.map((n) => (n.id === saved.id ? saved : n)) : [...prev, saved];
                    });
                    setSelectedNoteId(saved.id);
                  }}
                />
                {selectedNote && (
                  <>
                    <button
                      onClick={() =>
                        requestConfirm(
                          `Tombstone note "${selectedNote.title || "(untitled)"}"? Its content is kept in history, but it will no longer be listed.`,
                          () => void handleNoteTombstoneToggle(selectedNote, false)
                        )
                      }
                    >
                      Tombstone this note
                    </button>
                    <RelationsPanel vaultPath={vault.path} projectId={openProject.id} objectId={selectedNote.id} />
                  </>
                )}
              </section>
            )}
          </>
        )}

        {tab === "actions" && <ActionsPanel vaultPath={vault.path} projectId={openProject.id} />}
        {tab === "decisions" && <DecisionsPanel vaultPath={vault.path} projectId={openProject.id} />}
        {tab === "search" && <SearchPanel vaultPath={vault.path} projectId={openProject.id} />}
        {tab === "resume" && <ResumePanel vaultPath={vault.path} projectId={openProject.id} />}

        {error && <p className="error">{error}</p>}
      </main>
    );
  }

  return (
    <main className="shell">
      {confirmDialog}
      <h1>Flake</h1>
      <p className="notice">Vault: {vault.path}</p>
      <section>
        <h2>Projects</h2>
        {projects.length === 0 ? (
          <p>No projects yet.</p>
        ) : (
          <ul>
            {projects.map((p) => (
              <li key={p.id}>
                <strong>{p.name}</strong>
                {p.description ? ` -- ${p.description}` : ""}
                {" "}
                <span className="state-label">{p.active ? "Active" : "Archived"}</span>{" "}
                <button onClick={() => void handleOpenProject(p)}>Open</button>{" "}
                <button
                  onClick={() =>
                    requestConfirm(
                      p.active
                        ? `Archive project "${p.name}"? It will be marked inactive; nothing is deleted.`
                        : `Unarchive project "${p.name}"? It will be marked active again.`,
                      () => void handleArchiveToggle(p)
                    )
                  }
                >
                  {p.active ? "Archive" : "Unarchive"}
                </button>
              </li>
            ))}
          </ul>
        )}
      </section>
      <section>
        <h2>New project</h2>
        <label>
          Name
          <input value={newProjectName} onChange={(e) => setNewProjectName(e.target.value)} />
        </label>
        <label>
          Description (optional)
          <input value={newProjectDescription} onChange={(e) => setNewProjectDescription(e.target.value)} />
        </label>
        <button onClick={handleCreateProject} disabled={!newProjectName.trim()}>
          Create project
        </button>
      </section>
      {error && <p className="error">{error}</p>}
    </main>
  );
}
