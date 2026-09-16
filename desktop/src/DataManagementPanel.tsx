import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type {
  BackupReport,
  ExportPreview,
  ExportReport,
  ImportPreview,
  ImportReport,
  RecoveryReport,
  RestoreReport,
} from "./types";
import { useConfirm } from "./Confirm";

async function pickDirectory(): Promise<string | null> {
  return invoke<string | null>("pick_directory");
}

function newOperationId(): string {
  return `op-${Date.now()}-${Math.random().toString(36).slice(2)}`;
}

// T04-05: every destination/source here comes only from the owner's own
// native folder picker (`pick_directory`, already admitted in T04-01) --
// no new native dialog capability, no silent merge, no in-place
// destructive repair. Every result names its own verified head/omissions
// (BackupManifest/ExportManifest's own fields) rather than a bare "done."
export function DataManagementPanel({ vaultPath, projectId }: { vaultPath: string; projectId: string }) {
  const [error, setError] = useState<string | null>(null);
  const { requestConfirm, confirmDialog } = useConfirm();

  // --- Backup (cancellable) ---
  const [backupRunning, setBackupRunning] = useState(false);
  const [backupOpId, setBackupOpId] = useState<string | null>(null);
  const [backupResult, setBackupResult] = useState<BackupReport | null>(null);

  async function handleBackup() {
    const parent = await pickDirectory();
    if (!parent) return;
    const name = `backup-${Date.now()}`;
    const opId = newOperationId();
    setBackupOpId(opId);
    setBackupRunning(true);
    setError(null);
    try {
      const result = await invoke<BackupReport>("vault_backup", {
        vaultPath,
        destParentDir: parent,
        destName: name,
        operationId: opId,
      });
      setBackupResult(result);
    } catch (e) {
      setError(String(e));
    } finally {
      setBackupRunning(false);
      setBackupOpId(null);
    }
  }

  async function handleCancelBackup() {
    if (!backupOpId) return;
    await invoke("cancel_operation", { operationId: backupOpId });
  }

  // --- Restore from backup ---
  const [restoreResult, setRestoreResult] = useState<RestoreReport | null>(null);
  async function handleRestoreFromBackup() {
    const backupDir = await pickDirectory();
    if (!backupDir) return;
    const parent = await pickDirectory();
    if (!parent) return;
    const name = `restored-${Date.now()}`;
    setError(null);
    try {
      const result = await invoke<RestoreReport>("vault_restore_from_backup", {
        backupPath: backupDir,
        destParentDir: parent,
        destName: name,
      });
      setRestoreResult(result);
    } catch (e) {
      setError(String(e));
    }
  }

  // --- Recovery to a new root ---
  const [recoveryResult, setRecoveryResult] = useState<RecoveryReport | null>(null);
  async function handleRecover() {
    const original = await pickDirectory();
    if (!original) return;
    const parent = await pickDirectory();
    if (!parent) return;
    const name = `recovered-${Date.now()}`;
    setError(null);
    try {
      const result = await invoke<RecoveryReport>("vault_recover", {
        originalPath: original,
        destParentDir: parent,
        destName: name,
      });
      setRecoveryResult(result);
    } catch (e) {
      setError(String(e));
    }
  }

  // --- Export (preview first, then explicit export) ---
  const [exportScope, setExportScope] = useState<"project" | "full">("project");
  const [exportPreview, setExportPreview] = useState<ExportPreview | null>(null);
  const [exportResult, setExportResult] = useState<ExportReport | null>(null);

  async function handleExportPreview() {
    setError(null);
    try {
      const result = await invoke<ExportPreview>("export_preview", {
        vaultPath,
        projectId: exportScope === "project" ? projectId : null,
      });
      setExportPreview(result);
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleExport() {
    const parent = await pickDirectory();
    if (!parent) return;
    const name = `export-${Date.now()}`;
    setError(null);
    try {
      const result = await invoke<ExportReport>("vault_export", {
        vaultPath,
        projectId: exportScope === "project" ? projectId : null,
        destParentDir: parent,
        destName: name,
      });
      setExportResult(result);
    } catch (e) {
      setError(String(e));
    }
  }

  // --- Import (preview first, then explicit selected-merge into THIS vault) ---
  const [importPreview, setImportPreview] = useState<ImportPreview | null>(null);
  const [importSourcePath, setImportSourcePath] = useState<string | null>(null);
  const [importResult, setImportResult] = useState<ImportReport | null>(null);

  async function handleImportPreview() {
    const source = await pickDirectory();
    if (!source) return;
    setImportSourcePath(source);
    setError(null);
    try {
      const result = await invoke<ImportPreview>("import_preview", { sourcePath: source });
      setImportPreview(result);
    } catch (e) {
      setError(String(e));
      setImportPreview(null);
    }
  }

  async function handleImportSelected() {
    if (!importSourcePath) return;
    setError(null);
    try {
      const result = await invoke<ImportReport>("vault_import_selected", {
        vaultPath,
        sourcePath: importSourcePath,
      });
      setImportResult(result);
      setImportPreview(null);
      setImportSourcePath(null);
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <div className="data-management-panel">
      {confirmDialog}
      <h3>Backup &amp; recovery</h3>
      <p className="notice">Backups and recovery never touch the original vault; every result names its own verified head.</p>
      <div className="action-transitions">
        <button onClick={() => void handleBackup()} disabled={backupRunning}>
          {backupRunning ? "Backing up…" : "Create backup"}
        </button>
        {backupRunning && <button onClick={() => void handleCancelBackup()}>Cancel</button>}
        <button onClick={() => void handleRestoreFromBackup()}>Restore from backup (to a new location)</button>
        <button onClick={() => void handleRecover()}>Recover to a new location</button>
      </div>
      {backupResult && (
        <p>
          Backup verified: head_seq={backupResult.manifest.snapshot_head_seq}, {backupResult.manifest.members.length}{" "}
          member(s), verified={String(backupResult.manifest.verified)} -&gt; {backupResult.backup_root}
        </p>
      )}
      {restoreResult && (
        <p>
          Restored vault {restoreResult.vault_id} at head_seq={restoreResult.restored_transaction_head_seq},{" "}
          {restoreResult.restored_object_count} objects -&gt; {restoreResult.restored_root}
        </p>
      )}
      {recoveryResult && (
        <p>
          Recovered vault {recoveryResult.vault_id}, verified at head_seq={recoveryResult.verified_transaction_head_seq}
          . Original bytes preserved at {recoveryResult.preserved_at} -&gt; {recoveryResult.recovered_root}
        </p>
      )}

      <h3>Export</h3>
      <label>
        Scope
        <select value={exportScope} onChange={(e) => setExportScope(e.target.value as "project" | "full")}>
          <option value="project">This project only</option>
          <option value="full">Full vault</option>
        </select>
      </label>
      <button onClick={() => void handleExportPreview()}>Preview export</button>
      {exportPreview && (
        <div>
          <p>
            {exportPreview.kind}: {exportPreview.record_count} records, {exportPreview.revision_count} revisions, snapshot
            head_seq={exportPreview.snapshot_head_seq}
          </p>
          <button
            onClick={() =>
              requestConfirm(
                `Export ${exportPreview.record_count} records (${exportPreview.kind}) to a new location you choose?`,
                () => void handleExport()
              )
            }
          >
            Export to a new location
          </button>
        </div>
      )}
      {exportResult && (
        <p>
          Exported {exportResult.manifest.record_count} records, {exportResult.manifest.revision_count} revisions,
          omissions=[{exportResult.manifest.omissions.join(", ") || "none"}] -&gt; {exportResult.dest_root}
        </p>
      )}

      <h3>Import</h3>
      <button onClick={() => void handleImportPreview()}>Choose a package to preview</button>
      {importPreview && (
        <div>
          <p>
            {importPreview.kind} from vault {importPreview.source_vault_id}: {importPreview.record_count} records,{" "}
            {importPreview.revision_count} revisions
          </p>
          {importPreview.conflicts.length > 0 && (
            <ul>
              {importPreview.conflicts.map((c, i) => (
                <li key={i} className="error">
                  {c}
                </li>
              ))}
            </ul>
          )}
          <button
            onClick={() =>
              requestConfirm(
                `Merge ${importPreview.record_count} records from this package into the current vault? New object identities will be assigned; nothing in the source is modified.`,
                () => void handleImportSelected()
              )
            }
            disabled={importPreview.conflicts.length > 0}
          >
            Merge into this vault
          </button>
        </div>
      )}
      {importResult && (
        <p>
          Imported {importResult.imported_object_count} objects, {importResult.imported_revision_count} revisions
          ({importResult.mode}) into {importResult.dest_root}
        </p>
      )}

      {error && <p className="error">{error}</p>}
    </div>
  );
}
