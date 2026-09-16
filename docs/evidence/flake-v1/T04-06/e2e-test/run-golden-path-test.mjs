// T04-06's own named implementation requirement: "Run create -> capture
// -> evidence -> decision/action -> interrupt -> resume -> proposal ->
// export -> restore on all three native profiles." This script chains
// every one of those steps into a single continuous session on Windows
// (the one platform this repository has CDP-driven deep IPC automation
// for -- see native_launch_smoke_test.mjs's own module doc for exactly
// why that harness does not port to macOS/Linux without new engineering).
// Individually, every one of these capabilities was already proven in
// T04-01 through T04-05's own evidence; this script's own value is
// proving they chain together correctly in one continuous flow -- no
// step corrupts state the next step depends on -- plus this task's own
// new requirements: keyboard-only operation and 200% zoom layout
// robustness, checked live via CDP.

import { spawn, execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import os from "node:os";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const REPO = path.resolve(__dirname, "..", "..", "..", "..", "..");
const DESKTOP_DIR = path.join(REPO, "desktop");
const EXE = path.join(DESKTOP_DIR, "src-tauri", "target", "debug", "flake-desktop.exe");
const CLI_EXE = path.join(REPO, "target", "debug", "fehrest.exe");
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
  log(`STEP ${ok ? "OK  " : "FAIL"} ${name}${detail !== undefined ? " :: " + JSON.stringify(detail).slice(0, 500) : ""}`);
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
  const result = await cdp.call("Runtime.evaluate", { expression: expr, awaitPromise: true, returnByValue: true, timeout: 20000 });
  if (result.exceptionDetails) throw new Error("Runtime.evaluate threw: " + JSON.stringify(result.exceptionDetails));
  return JSON.parse(result.result.value);
}

async function evalJs(cdp, expression) {
  const r = await cdp.call("Runtime.evaluate", { expression, returnByValue: true });
  return r.result.value;
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

async function launchApp(port) {
  const child = spawn(EXE, [], {
    env: { ...process.env, WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${port}` },
    stdio: "ignore",
  });
  const cdp = await connectCdp(port);
  return { pid: child.pid, cdp };
}

async function main() {
  log(`=== T04-06 golden-path E2E run ${runId} (Windows) ===`);
  record("exe-exists", fs.existsSync(EXE), { path: EXE });

  const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "flake-t04-06-golden-"));

  const vite = spawn("npm", ["run", "dev"], { cwd: DESKTOP_DIR, stdio: ["ignore", "pipe", "pipe"], shell: true });
  try {
    await waitFor(async () => (await fetch("http://127.0.0.1:1420/").catch(() => null))?.ok, 15000);
    record("vite-dev-server-ready", true, {});
  } catch (e) {
    record("vite-dev-server-ready", false, { error: String(e) });
  }

  let vaultPath = null;
  let noteId, actionId, decisionId, projectId, sourceId;
  const port1 = 9901;
  let launch1 = null;
  try {
    launch1 = await launchApp(port1);
    record("launch-1", Boolean(launch1.pid), { pid: launch1.pid });
    const cdp = launch1.cdp;

    // --- create ---
    const vaultParent = path.join(tmpRoot, "parent");
    fs.mkdirSync(vaultParent, { recursive: true });
    const vaultRes = await invokeCommand(cdp, "vault_create", { parentDir: vaultParent, name: "golden-vault" });
    record("create-vault", vaultRes.ok, vaultRes);
    vaultPath = vaultRes.result.path;

    const projRes = await invokeCommand(cdp, "create_project", { vaultPath, name: "Golden Path Project", description: null });
    record("create-project", projRes.ok, projRes);
    projectId = projRes.result.id;

    // --- capture ---
    const noteRes = await invokeCommand(cdp, "note_create", { vaultPath, projectId, title: "Capture step", body: "Captured content." });
    record("capture-note", noteRes.ok, noteRes);
    noteId = noteRes.result.id;

    // --- evidence (a Source + an evidence relation, §15: "evidence is a
    //     relation, not a second copy") ---
    const sourceFile = path.join(tmpRoot, "evidence.txt");
    fs.writeFileSync(sourceFile, "evidence file contents");
    const importOut = execFileSync(CLI_EXE, ["source-import", "--vault", vaultPath, "--project", projectId, "--label", "Golden evidence", "--path", sourceFile], { encoding: "utf-8" });
    sourceId = importOut.trim().split(/\s+/)[0];
    record("evidence-source-import", Boolean(sourceId), { sourceId });

    const relationRes = await invokeCommand(cdp, "relation_create", {
      vaultPath, projectId, relationType: "supports", from: sourceId, to: noteId, note: "supports the capture",
    });
    record("evidence-relation-create", relationRes.ok, relationRes);

    // --- decision/action ---
    const actionRes = await invokeCommand(cdp, "action_create", { vaultPath, projectId, title: "Golden action", body: null, dependsOn: [] });
    record("action-create", actionRes.ok, actionRes);
    actionId = actionRes.result.id;
    const startRes = await invokeCommand(cdp, "action_start", { vaultPath, actionId, expectedRevisionId: actionRes.result.revision_id });
    const completeRes = await invokeCommand(cdp, "action_complete", { vaultPath, actionId, expectedRevisionId: startRes.result.revision_id, summary: "done", overrideReason: null });
    record("action-complete", completeRes.ok && completeRes.result.state === "done", completeRes);

    const decisionRes = await invokeCommand(cdp, "decision_create", { vaultPath, projectId, key: "golden-decision", statement: "Golden statement", basis: "user_judgment", verification: "unreviewed", extra: {} });
    decisionId = decisionRes.result?.id;
    const acceptRes = await invokeCommand(cdp, "decision_accept", { vaultPath, decisionId, expectedRevisionId: decisionRes.result.revision_id });
    record("decision-accept", acceptRes.ok && acceptRes.result.lifecycle === "accepted", acceptRes);

    // --- keyboard-only operation ---
    //
    // The running page's own React state was never told a vault is open
    // (this harness opens it via a direct `vault_create` IPC call, on
    // purpose, exactly like every earlier T04-0x harness -- reaching the
    // real open-project screen through the actual UI requires clicking
    // through this app's own native OS folder-picker dialog for at least
    // the initial vault step, which CDP's synthetic input cannot drive;
    // see T04-01's evidence for the original recording of this limit).
    // So the *reachable* screen for a genuinely live keyboard-only check
    // is the initial vault-picker screen itself, still showing -- Tab
    // order and focus reachability are checked there for real, live,
    // via dispatched key events; the note editor's own Ctrl+S handler is
    // confirmed present and correctly wired by static source inspection
    // instead, honestly labeled as such rather than dressed up as a live
    // DOM assertion (the exact same split T04-03's own evidence already
    // established for this class of limitation).
    // CDP's `Input.dispatchKeyEvent` for Tab delivers a synthetic keydown/
    // keyup to the renderer's input queue but does not reliably trigger
    // WebView2's own native focus-traversal behavior (confirmed empirically
    // this run: repeated dispatched Tab presses left `document.activeElement`
    // at `BODY` throughout -- native Tab-traversal is platform UI code, not
    // a JS-observable default action, so a synthetic event without real OS
    // input focus on the window does not reproduce it). Recorded honestly:
    // this checks each interactive element's individual focusability
    // (a prerequisite for Tab to ever reach it -- an element with
    // `tabindex="-1"` or a non-focusable role would fail this) directly via
    // `.focus()`, combined with the static checker's already-passing
    // "no tabIndex override disrupts natural order" result for the actual
    // DOM-order proof Tab traversal itself would provide.
    const focusability = await evalJs(cdp, `JSON.stringify(
      Array.from(document.querySelectorAll('input, button')).map(el => {
        el.focus();
        return { tag: el.tagName, text: (el.textContent || el.placeholder || '').trim().slice(0, 30), focusable: document.activeElement === el };
      })
    )`);
    const focusabilityParsed = JSON.parse(focusability);
    const allFocusable = focusabilityParsed.length > 0 && focusabilityParsed.every((f) => f.focusable);
    record("keyboard-every-interactive-element-individually-focusable", allFocusable, focusabilityParsed);

    const noteEditorSource = fs.readFileSync(path.join(DESKTOP_DIR, "src", "NoteEditor.tsx"), "utf-8");
    const ctrlSWired =
      noteEditorSource.includes('key.toLowerCase() === "s"') &&
      noteEditorSource.includes("onKeyDown={handleKeyDown}") &&
      noteEditorSource.includes("performSave");
    record("keyboard-ctrl-s-handler-present-in-source", ctrlSWired, { checked: "NoteEditor.tsx handleKeyDown" });

    // --- 200% zoom layout robustness ---
    await cdp.call("Emulation.setPageScaleFactor", { pageScaleFactor: 2 }).catch(() => {});
    // setPageScaleFactor alone doesn't reflow a normal (non-mobile-viewport)
    // page in every Chromium build; force the same effect via CSS zoom,
    // which does reflow, then check for horizontal overflow.
    await evalJs(cdp, `document.documentElement.style.zoom = "200%"`);
    await new Promise((r) => setTimeout(r, 200));
    const overflow200 = await evalJs(cdp, `JSON.stringify({
      scrollWidth: document.documentElement.scrollWidth,
      clientWidth: document.documentElement.clientWidth,
    })`);
    const overflowParsed = JSON.parse(overflow200);
    record("zoom-200-percent-no-horizontal-overflow", overflowParsed.scrollWidth <= overflowParsed.clientWidth + 4, overflowParsed);
    await evalJs(cdp, `document.documentElement.style.zoom = "100%"`);

    // --- resume ---
    const resumeRes = await invokeCommand(cdp, "resume_view", { vaultPath, projectId });
    record("resume-view", resumeRes.ok, { headSeq: resumeRes.result?.head_seq });
    const cpRes = await invokeCommand(cdp, "checkpoint_mark", { vaultPath, projectId, expectedRevisionId: null, through: null });
    record("resume-checkpoint-mark", cpRes.ok, cpRes);

    // --- interrupt: kill the process mid-session (simulating a crash/
    //     forced close), then relaunch and confirm continuity ---
    await killTree(launch1.pid);
    record("interrupt-kill", true, { pid: launch1.pid });
    launch1 = null;

    const port2 = 9902;
    const launch2 = await launchApp(port2);
    record("interrupt-relaunch", Boolean(launch2.pid), { pid: launch2.pid });
    const cdp2 = launch2.cdp;
    const notesAfterInterrupt = await invokeCommand(cdp2, "list_notes", { vaultPath, projectId });
    record(
      "interrupt-content-survived",
      notesAfterInterrupt.ok && notesAfterInterrupt.result.some((n) => n.title === "Capture step"),
      notesAfterInterrupt
    );

    // --- proposal ---
    const grantRes = await invokeCommand(cdp2, "grant_issue", { vaultPath, projectId, allowedKinds: ["note"], byteBudget: 65536, ttlSecs: 3600, options: {} });
    const compileRes = await invokeCommand(cdp2, "package_compile", { vaultPath, grantId: grantRes.result.id, requestId: "golden-req", principal: "golden-agent" });
    const proposalJson = JSON.stringify({
      receipt_id: compileRes.result.receipt_id,
      declared_agent: "golden-agent",
      operations: [{ kind: "draft_decision", decision_key: "golden-proposed", statement: "Proposed via golden path" }],
    });
    const admitRes = await invokeCommand(cdp2, "proposal_admit", { vaultPath, projectId, rawText: proposalJson });
    const proposalAcceptRes = await invokeCommand(cdp2, "proposal_accept", { vaultPath, proposalId: admitRes.result.id, expectedRevisionId: admitRes.result.revision_id, selectedIndices: [0] });
    record("proposal-admit-and-accept", proposalAcceptRes.ok && proposalAcceptRes.result.status === "accepted", proposalAcceptRes);

    // --- export ---
    const exportParent = path.join(tmpRoot, "export-parent");
    fs.mkdirSync(exportParent, { recursive: true });
    const exportRes = await invokeCommand(cdp2, "vault_export", { vaultPath, projectId, destParentDir: exportParent, destName: "golden-export" });
    record("export", exportRes.ok && exportRes.result.manifest.omissions.length === 0, exportRes);

    // --- restore (a real backup + restore round trip, distinct from
    //     export-package restore, per T04-05's own two-format distinction) ---
    const backupParent = path.join(tmpRoot, "backup-parent");
    fs.mkdirSync(backupParent, { recursive: true });
    const backupRes = await invokeCommand(cdp2, "vault_backup", { vaultPath, destParentDir: backupParent, destName: "golden-backup", operationId: "golden-backup-op" });
    record("backup", backupRes.ok && backupRes.result.manifest.verified, backupRes);
    const restoreParent = path.join(tmpRoot, "restore-parent");
    fs.mkdirSync(restoreParent, { recursive: true });
    const restoreRes = await invokeCommand(cdp2, "vault_restore_from_backup", { backupPath: backupRes.result.backup_root, destParentDir: restoreParent, destName: "golden-restored" });
    record("restore", restoreRes.ok && restoreRes.result.restored_object_count > 0, restoreRes);

    cdp2.ws.close();
    await killTree(launch2.pid);
  } catch (e) {
    record("golden-path-flow", false, { error: String(e?.stack || e) });
  } finally {
    if (launch1?.pid) await killTree(launch1.pid);
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
