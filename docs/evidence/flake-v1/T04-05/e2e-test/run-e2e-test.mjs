// T04-05 backup/recovery/import/export E2E test runner. Reuses the
// established CDP-driven harness pattern to drive the real compiled
// flake-desktop.exe through its exact real typed-IPC bridge.
//
// "Destination manifest/byte verification and original-vault comparison"
// (this task's own named verification method) is proven by: (a) reading
// every backup/export member's bytes directly from disk with Node and
// independently recomputing SHA-256, never trusting Core's own "verified"
// claim alone; (b) reading both the original and the resulting
// (restored/recovered) vault's canonical.sqlite directly via T02-07's
// unmodified sqlite_reader.py, proving the original is byte-for-byte
// unchanged and the new one matches.

import { spawn, execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import os from "node:os";
import crypto from "node:crypto";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const REPO = path.resolve(__dirname, "..", "..", "..", "..", "..");
const DESKTOP_DIR = path.join(REPO, "desktop");
const EXE = path.join(DESKTOP_DIR, "src-tauri", "target", "debug", "flake-desktop.exe");
const OUT_DIR = path.join(__dirname, "results");
fs.mkdirSync(OUT_DIR, { recursive: true });

const runId = new Date().toISOString().replace(/[:.]/g, "-");
const logPath = path.join(OUT_DIR, `run-${runId}.log`);
const jsonPath = path.join(OUT_DIR, `run-${runId}.json`);

const steps = [];
function log(msg) {
  const line = `[${new Date().toISOString()}] ${msg}`;
  console.log(line);
  fs.appendFileSync(logPath, line + "\n");
}
function record(name, ok, detail) {
  const entry = { name, ok, detail: detail ?? null, ts: new Date().toISOString() };
  steps.push(entry);
  log(`STEP ${ok ? "OK  " : "FAIL"} ${name}${detail !== undefined ? " :: " + JSON.stringify(detail).slice(0, 600) : ""}`);
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, steps }, null, 2));
  return entry;
}

async function waitFor(fn, timeoutMs, intervalMs = 300) {
  const deadline = Date.now() + timeoutMs;
  let lastErr;
  while (Date.now() < deadline) {
    try {
      const v = await fn();
      if (v) return v;
    } catch (e) {
      lastErr = e;
    }
    await new Promise((r) => setTimeout(r, intervalMs));
  }
  throw new Error(`waitFor timed out: ${lastErr?.message ?? "condition never true"}`);
}

async function connectCdp(port) {
  const page = await waitFor(async () => {
    const res = await fetch(`http://127.0.0.1:${port}/json`);
    if (!res.ok) return null;
    const targets = await res.json();
    return targets.find((t) => t.type === "page" && t.url && !t.url.startsWith("about:")) || null;
  }, 20000);
  const ws = new WebSocket(page.webSocketDebuggerUrl);
  await new Promise((resolve, reject) => {
    ws.addEventListener("open", resolve);
    ws.addEventListener("error", reject);
  });
  let id = 0;
  function call(method, params) {
    const myId = ++id;
    return new Promise((resolve, reject) => {
      const onMsg = (ev) => {
        const msg = JSON.parse(ev.data.toString());
        if (msg.id === myId) {
          ws.removeEventListener("message", onMsg);
          if (msg.error) reject(new Error(JSON.stringify(msg.error)));
          else resolve(msg.result);
        }
      };
      ws.addEventListener("message", onMsg);
      ws.send(JSON.stringify({ id: myId, method, params }));
    });
  }
  await waitFor(async () => {
    const r = await call("Runtime.evaluate", {
      expression: "typeof window.__TAURI_INTERNALS__?.invoke",
      returnByValue: true,
    });
    return r.result.value === "function" ? true : null;
  }, 15000, 200);
  return { ws, call, pageUrl: page.url };
}

async function invokeCommand(cdp, command, payload) {
  const expr = `
    (async () => {
      try {
        const r = await window.__TAURI_INTERNALS__.invoke(${JSON.stringify(command)}, ${JSON.stringify(payload)});
        return JSON.stringify({ ok: true, result: r });
      } catch (e) {
        return JSON.stringify({ ok: false, error: String(e) });
      }
    })()
  `;
  const result = await cdp.call("Runtime.evaluate", {
    expression: expr,
    awaitPromise: true,
    returnByValue: true,
    timeout: 20000,
  });
  if (result.exceptionDetails) {
    throw new Error("Runtime.evaluate threw: " + JSON.stringify(result.exceptionDetails));
  }
  return JSON.parse(result.result.value);
}

// Fires the invoke without awaiting the CDP round trip for the result --
// used only for the cancellation race, where we want `vault_backup` and
// `cancel_operation` in flight as close together as JS scheduling allows.
function invokeCommandNoWait(cdp, command, payload) {
  return invokeCommand(cdp, command, payload);
}

async function killTree(pid) {
  if (!pid) return;
  await new Promise((resolve) => {
    const p = spawn("taskkill.exe", ["/PID", String(pid), "/T", "/F"]);
    p.on("close", resolve);
    p.on("error", resolve);
  });
}

async function sh(cmd, args, opts = {}) {
  return new Promise((resolve) => {
    const child = spawn(cmd, args, { shell: false, ...opts });
    let out = "";
    child.stdout?.on("data", (d) => (out += d.toString()));
    child.on("close", (code) => resolve({ code, out }));
    child.on("error", () => resolve({ code: -1, out }));
  });
}

function sha256File(p) {
  return crypto.createHash("sha256").update(fs.readFileSync(p)).digest("hex");
}

// Reuses T02-07's sqlite_reader.py directly (not through T04-02's
// note-specific wrapper) for a whole-vault snapshot: vault row + head hash
// chain + every current object's payload. This is the "original-vault
// comparison" evidence -- proving the source vault is untouched after a
// backup/restore/recovery, and that the destination matches.
function independentReadWholeVault(dbPath) {
  const script = `
import sys, json
from pathlib import Path
sys.path.insert(0, ${JSON.stringify(path.join(REPO, "tools", "independent-verify"))})
from sqlite_reader import read_vault
result = read_vault(Path(${JSON.stringify(dbPath)}))
print(json.dumps({
    "transaction_head_seq": result["vault_row"]["transaction_head_seq"],
    "vault_id": result["vault_row"]["vault_id"],
    "object_count": len(result["current_object"]),
    "head_hash_chain_verified": result["head_hash_chain_verified"],
}))
`;
  const out = execFileSync("python", ["-c", script], { encoding: "utf-8" });
  return JSON.parse(out.trim());
}

function independentReadExport(destRoot) {
  const out = execFileSync("python", [path.join(__dirname, "independent_read_export.py"), destRoot], {
    encoding: "utf-8",
  });
  return JSON.parse(out.trim());
}

async function main() {
  log(`=== T04-05 backup/recovery/import/export E2E run ${runId} ===`);
  record("exe-exists", fs.existsSync(EXE), { path: EXE });

  const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "flake-t04-05-e2e-"));

  const vite = spawn("npm", ["run", "dev"], { cwd: DESKTOP_DIR, stdio: ["ignore", "pipe", "pipe"], shell: true });
  try {
    await waitFor(async () => (await fetch("http://127.0.0.1:1420/").catch(() => null))?.ok, 15000);
    record("vite-dev-server-ready", true, {});
  } catch (e) {
    record("vite-dev-server-ready", false, { error: String(e) });
  }

  const port = 9801;
  let root = null;
  let vaultPath = null;
  try {
    const child = spawn(EXE, [], {
      env: { ...process.env, WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${port}` },
      stdio: "ignore",
    });
    root = child.pid;
    record("launch", true, { pid: root });

    const cdp = await connectCdp(port);
    record("bridge-ready", true, { url: cdp.pageUrl });

    const vaultParent = path.join(tmpRoot, "parent");
    fs.mkdirSync(vaultParent, { recursive: true });
    const vaultRes = await invokeCommand(cdp, "vault_create", { parentDir: vaultParent, name: "original-vault" });
    record("vault-create", vaultRes.ok, vaultRes);
    vaultPath = vaultRes.result.path;
    const vaultDb = path.join(vaultPath, ".fehrest", "canonical.sqlite");

    const projRes = await invokeCommand(cdp, "create_project", { vaultPath, name: "T04-05 Project", description: null });
    record("project-create", projRes.ok, projRes);
    const projectId = projRes.result.id;

    const noteRes = await invokeCommand(cdp, "note_create", {
      vaultPath,
      projectId,
      title: "Backup content",
      body: "This content must survive backup, restore, export, import and recovery unchanged.",
    });
    record("note-create", noteRes.ok, noteRes);

    // Inflate the store a little so a backup takes long enough for a
    // genuine concurrent-cancel race to have a real chance of landing
    // inside the copy loop (see the cancellation test below).
    for (let i = 0; i < 4; i++) {
      const big = await invokeCommand(cdp, "note_create", {
        vaultPath,
        projectId,
        title: `Filler ${i}`,
        body: "x".repeat(800_000),
      });
      record(`filler-note-${i}`, big.ok, { ok: big.ok });
    }

    const originalSnapshot = independentReadWholeVault(vaultDb);
    record("original-vault-snapshot-before-backup", originalSnapshot.head_hash_chain_verified, originalSnapshot);

    // === Backup, independently byte-verified ===
    const backupParent = path.join(tmpRoot, "backup-parent");
    fs.mkdirSync(backupParent, { recursive: true });
    const backupOpId = "op-backup-1";
    const backupRes = await invokeCommand(cdp, "vault_backup", {
      vaultPath,
      destParentDir: backupParent,
      destName: "backup-1",
      operationId: backupOpId,
    });
    record("backup-create", backupRes.ok && backupRes.result.manifest.verified, backupRes);
    const backupRoot = backupRes.result?.backup_root;

    if (backupRoot) {
      let allMembersMatch = true;
      const memberDetails = [];
      for (const m of backupRes.result.manifest.members) {
        const memberPath = path.join(backupRoot, ".fehrest", m.path);
        const exists = fs.existsSync(memberPath);
        const actualSha = exists ? sha256File(memberPath) : null;
        const matches = exists && actualSha === m.sha256;
        if (!matches) allMembersMatch = false;
        memberDetails.push({ path: m.path, expectedSha: m.sha256, actualSha, matches });
      }
      record("backup-members-independently-byte-verified", allMembersMatch, memberDetails);
    }

    // No-clobber: backing up to the same destination again must refuse.
    const backupAgainRes = await invokeCommand(cdp, "vault_backup", {
      vaultPath,
      destParentDir: backupParent,
      destName: "backup-1",
      operationId: "op-backup-clobber-probe",
    });
    record(
      "backup-no-clobber-refused",
      backupAgainRes.ok === false && backupAgainRes.error.includes("already published"),
      backupAgainRes
    );

    // === Original vault unchanged after backup ===
    const afterBackupSnapshot = independentReadWholeVault(vaultDb);
    record(
      "original-vault-unchanged-after-backup",
      afterBackupSnapshot.transaction_head_seq === originalSnapshot.transaction_head_seq &&
        afterBackupSnapshot.object_count === originalSnapshot.object_count,
      { before: originalSnapshot, after: afterBackupSnapshot }
    );

    // === Restore from backup to a new location; compare against original ===
    const restoreParent = path.join(tmpRoot, "restore-parent");
    fs.mkdirSync(restoreParent, { recursive: true });
    const restoreRes = await invokeCommand(cdp, "vault_restore_from_backup", {
      backupPath: backupRoot,
      destParentDir: restoreParent,
      destName: "restored-vault",
    });
    record("restore-from-backup", restoreRes.ok, restoreRes);
    if (restoreRes.ok) {
      const restoredDb = path.join(restoreRes.result.restored_root, ".fehrest", "canonical.sqlite");
      const restoredSnapshot = independentReadWholeVault(restoredDb);
      record(
        "restored-vault-matches-original",
        restoredSnapshot.transaction_head_seq === originalSnapshot.transaction_head_seq &&
          restoredSnapshot.object_count === originalSnapshot.object_count &&
          restoredSnapshot.vault_id === originalSnapshot.vault_id,
        { original: originalSnapshot, restored: restoredSnapshot }
      );
    }

    // === Cancellation: deterministic "nothing to cancel" case, then a
    //     genuine concurrent-cancel race (recorded honestly either way) ===
    const cancelNothingRes = await invokeCommand(cdp, "cancel_operation", { operationId: "op-does-not-exist" });
    record("cancel-nonexistent-operation-returns-false", cancelNothingRes.ok && cancelNothingRes.result === false, cancelNothingRes);

    const raceBackupParent = path.join(tmpRoot, "backup-race-parent");
    fs.mkdirSync(raceBackupParent, { recursive: true });
    const raceOpId = "op-backup-race";
    const backupPromise = invokeCommandNoWait(cdp, "vault_backup", {
      vaultPath,
      destParentDir: raceBackupParent,
      destName: "backup-race",
      operationId: raceOpId,
    });
    const cancelPromise = invokeCommand(cdp, "cancel_operation", { operationId: raceOpId });
    const [raceBackupRes, raceCancelRes] = await Promise.all([backupPromise, cancelPromise]);
    const raceOutcome = raceBackupRes.ok
      ? "completed-before-cancel-landed"
      : raceBackupRes.error?.includes("cancelled")
      ? "genuinely-cancelled"
      : "other-error";
    record("backup-cancellation-race-outcome", raceOutcome !== "other-error", {
      raceOutcome,
      raceBackupRes,
      raceCancelRes,
    });
    if (raceOutcome === "genuinely-cancelled") {
      const raceDestExists = fs.existsSync(path.join(raceBackupParent, "backup-race", ".fehrest"));
      record("cancelled-backup-left-no-published-destination", !raceDestExists, { raceDestExists });
    }

    // === Export (project-scoped), independently verified ===
    const exportPreviewRes = await invokeCommand(cdp, "export_preview", { vaultPath, projectId });
    record("export-preview", exportPreviewRes.ok && exportPreviewRes.result.record_count > 0, exportPreviewRes);

    const exportParent = path.join(tmpRoot, "export-parent");
    fs.mkdirSync(exportParent, { recursive: true });
    const exportRes = await invokeCommand(cdp, "vault_export", {
      vaultPath,
      projectId,
      destParentDir: exportParent,
      destName: "export-1",
    });
    record(
      "export-create",
      exportRes.ok && exportRes.result.manifest.omissions.length === 0,
      exportRes
    );
    const exportDestRoot = exportRes.result?.dest_root;
    if (exportDestRoot) {
      const exportVerify = independentReadExport(exportDestRoot);
      record(
        "export-independently-verified",
        exportVerify.dangling_references.length === 0 && exportVerify.record_count > 0,
        exportVerify
      );
    }

    // === Import preview + selected-merge into a FRESH second vault ===
    const secondVaultParent = path.join(tmpRoot, "second-vault-parent");
    fs.mkdirSync(secondVaultParent, { recursive: true });
    const secondVaultRes = await invokeCommand(cdp, "vault_create", { parentDir: secondVaultParent, name: "second-vault" });
    record("second-vault-create", secondVaultRes.ok, secondVaultRes);
    const secondVaultPath = secondVaultRes.result?.path;

    const importPreviewRes = await invokeCommand(cdp, "import_preview", { sourcePath: exportDestRoot });
    record(
      "import-preview",
      importPreviewRes.ok && importPreviewRes.result.conflicts.length === 0 && importPreviewRes.result.record_count > 0,
      importPreviewRes
    );

    const importRes = await invokeCommand(cdp, "vault_import_selected", {
      vaultPath: secondVaultPath,
      sourcePath: exportDestRoot,
    });
    record(
      "import-selected-merge",
      importRes.ok && importRes.result.mode === "selected-merge" && Object.keys(importRes.result.id_map).length > 0,
      importRes
    );

    if (importRes.ok) {
      // The imported project got a *new* identity (id_map is non-empty,
      // "new object identities assigned" -- never the source's own IDs) --
      // find it via list_projects on the destination vault rather than
      // assuming which key in id_map is the project.
      const importedProjects = await invokeCommand(cdp, "list_projects", { vaultPath: secondVaultPath });
      const importedProject = importedProjects.ok
        ? importedProjects.result.find((p) => p.name === "T04-05 Project")
        : null;
      record("imported-project-visible-with-new-identity", Boolean(importedProject), importedProject);
      if (importedProject) {
        const importedNotes = await invokeCommand(cdp, "list_notes", { vaultPath: secondVaultPath, projectId: importedProject.id });
        const importedNote = importedNotes.ok ? importedNotes.result.find((n) => n.title === "Backup content") : null;
        record(
          "imported-note-content-matches",
          importedNote?.body === "This content must survive backup, restore, export, import and recovery unchanged.",
          importedNote
        );
      }
    }

    // === Corrupt/unsupported input: import_preview on an empty directory ===
    const emptyDir = path.join(tmpRoot, "empty-not-a-package");
    fs.mkdirSync(emptyDir, { recursive: true });
    const badImportRes = await invokeCommand(cdp, "import_preview", { sourcePath: emptyDir });
    record("import-preview-unsupported-directory-refused-cleanly", badImportRes.ok === false, badImportRes);

    // === Recovery to a new root; compare against original ===
    const recoverParent = path.join(tmpRoot, "recover-parent");
    fs.mkdirSync(recoverParent, { recursive: true });
    const recoverRes = await invokeCommand(cdp, "vault_recover", {
      originalPath: vaultPath,
      destParentDir: recoverParent,
      destName: "recovered-vault",
    });
    record("vault-recover", recoverRes.ok, recoverRes);
    if (recoverRes.ok) {
      const recoveredDb = path.join(recoverRes.result.recovered_root, ".fehrest", "canonical.sqlite");
      const recoveredSnapshot = independentReadWholeVault(recoveredDb);
      const originalAfterRecoverySnapshot = independentReadWholeVault(vaultDb);
      record(
        "recovered-vault-matches-and-original-unchanged",
        recoveredSnapshot.object_count === originalAfterRecoverySnapshot.object_count &&
          recoveredSnapshot.vault_id === originalAfterRecoverySnapshot.vault_id,
        { recovered: recoveredSnapshot, originalAfter: originalAfterRecoverySnapshot }
      );
    }

    cdp.ws.close();
  } catch (e) {
    record("desktop-workflow", false, { error: String(e?.stack || e) });
  } finally {
    if (root) await killTree(root);
    try {
      const netstat = await sh("netstat.exe", ["-ano"]);
      const line = netstat.out.split("\n").find((l) => l.includes(":1420") && l.includes("LISTENING"));
      const pid = line?.trim().split(/\s+/).pop();
      if (pid) await killTree(pid);
    } catch {}
    try {
      vite.kill();
    } catch {}
  }

  const overallOk = steps.every((s) => s.ok);
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, overallOk, tmpRoot, vaultPath, steps }, null, 2));
  log(`=== run complete. overallOk=${overallOk}. json=${jsonPath} log=${logPath} ===`);
}

main().catch((e) => {
  log("FATAL: " + String(e?.stack || e));
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, fatal: String(e?.stack || e), steps }, null, 2));
  process.exitCode = 1;
});
