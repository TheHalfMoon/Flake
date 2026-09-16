import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface VaultInfo {
  path: string;
  vault_id: string;
}

interface ProjectSummary {
  id: string;
  name: string;
  description: string | null;
  active: boolean;
}

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

  return (
    <main className="shell">
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
                {!p.active && " (archived)"}
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
