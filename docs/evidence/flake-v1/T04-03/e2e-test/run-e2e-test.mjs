// T04-03 desktop workflow E2E test runner.
//
// Drives the real compiled flake-desktop.exe through its exact real typed
// IPC bridge (window.__TAURI_INTERNALS__.invoke) -- the same CDP-driven
// pattern T04-01/T04-02 established -- through a complete project loop
// (project -> note -> action lifecycle -> decision lifecycle -> relation),
// then independently proves the task's own named acceptance clause:
// "Scripted complete workflow produces identical canonical state to CLI."
// The exact same workflow is separately driven through the real compiled
// `fehrest` CLI against a second, independent vault, and
// `compare_canonical_state.py` (reusing T02-07's unmodified
// `sqlite_reader.py`) proves the two vaults' content is equivalent,
// ignoring only the UUIDs/timestamps that are inherently non-deterministic
// per vault.
//
// Also exercises: an empty-state UI check (before any work exists), an
// archived-project UI check, and a genuine refused-conflict failure case
// -- the task's "empty, archived and failure states offer appropriate
// next actions" acceptance clause.

import { spawn, execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import os from "node:os";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const REPO = path.resolve(__dirname, "..", "..", "..", "..", ".."); // .../Flake
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
    timeout: 15000,
  });
  if (result.exceptionDetails) {
    throw new Error("Runtime.evaluate threw: " + JSON.stringify(result.exceptionDetails));
  }
  return JSON.parse(result.result.value);
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

function cli(args) {
  return execFileSync(CLI_EXE, args, { encoding: "utf-8" }).trim();
}

function runCliWorkflow(vaultPath) {
  cli(["canonical-init", "--vault", vaultPath]);
  const projectId = cli(["project-create", "--vault", vaultPath, "--name", "Workflow Project"]).split(/\s+/)[0];
  const noteId = cli([
    "note-create",
    "--vault", vaultPath,
    "--project", projectId,
    "--title", "Meeting notes",
    "--body", "Discussed the roadmap.",
  ]).split(/\s+/)[0];
  const [actionId, actionRev1] = cli([
    "action-create",
    "--vault", vaultPath,
    "--project", projectId,
    "--title", "Ship the update",
  ]).split(/\s+/);
  const startOut = cli(["action-start", "--vault", vaultPath, "--id", actionId, "--expect", actionRev1]);
  const actionRev2 = startOut.split(/\s+/)[1];
  cli(["action-complete", "--vault", vaultPath, "--id", actionId, "--expect", actionRev2, "--summary", "Shipped v1"]);
  const decOut = cli([
    "decision-create",
    "--vault", vaultPath,
    "--project", projectId,
    "--key", "ship-strategy",
    "--statement", "Ship weekly",
    "--basis", "user-judgment",
    "--verification", "unreviewed",
  ]);
  const [decisionId, decisionRev1] = decOut.split(/\s+/);
  cli(["decision-accept", "--vault", vaultPath, "--id", decisionId, "--expect", decisionRev1]);
  cli([
    "relation-create",
    "--vault", vaultPath,
    "--project", projectId,
    "--type", "relates_to",
    "--from", noteId,
    "--to", decisionId,
    "--note", "supports the plan",
  ]);
  return { projectId, noteId, actionId, decisionId };
}

async function main() {
  log(`=== T04-03 desktop workflow E2E test run ${runId} ===`);
  record("exe-exists", fs.existsSync(EXE), { path: EXE });
  record("cli-exe-exists", fs.existsSync(CLI_EXE), { path: CLI_EXE });

  const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "flake-t04-03-e2e-"));

  // --- CLI-driven reference vault ---
  const cliVaultPath = path.join(tmpRoot, "cli-vault");
  try {
    runCliWorkflow(cliVaultPath);
    record("cli-workflow", true, { vaultPath: cliVaultPath });
  } catch (e) {
    record("cli-workflow", false, { error: String(e) });
  }

  // --- Desktop-driven vault, via the real app ---
  const vite = spawn("npm", ["run", "dev"], { cwd: DESKTOP_DIR, stdio: ["ignore", "pipe", "pipe"], shell: true });
  try {
    await waitFor(async () => (await fetch("http://127.0.0.1:1420/").catch(() => null))?.ok, 15000);
    record("vite-dev-server-ready", true, {});
  } catch (e) {
    record("vite-dev-server-ready", false, { error: String(e) });
  }

  const port = 9601;
  let root = null;
  let desktopVaultPath = null;
  try {
    const child = spawn(EXE, [], {
      env: { ...process.env, WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${port}` },
      stdio: "ignore",
    });
    root = child.pid;
    record("launch", true, { pid: root });

    const cdp = await connectCdp(port);
    record("bridge-ready", true, { url: cdp.pageUrl });

    const vaultParent = path.join(tmpRoot, "desktop-parent");
    fs.mkdirSync(vaultParent, { recursive: true });
    const vaultRes = await invokeCommand(cdp, "vault_create", { parentDir: vaultParent, name: "desktop-vault" });
    record("vault-create", vaultRes.ok, vaultRes);
    desktopVaultPath = vaultRes.result.path;

    const projRes = await invokeCommand(cdp, "create_project", {
      vaultPath: desktopVaultPath,
      name: "Workflow Project",
      description: null,
    });
    record("project-create", projRes.ok, projRes);
    const projectId = projRes.result.id;

    // --- Empty-state UI check, before anything else exists in this project ---
    const emptyNotes = await invokeCommand(cdp, "list_notes", { vaultPath: desktopVaultPath, projectId });
    const emptyActions = await invokeCommand(cdp, "list_actions", { vaultPath: desktopVaultPath, projectId });
    const emptyDecisions = await invokeCommand(cdp, "list_decisions", { vaultPath: desktopVaultPath, projectId });
    record(
      "empty-state-backend",
      emptyNotes.ok && emptyNotes.result.length === 0 &&
        emptyActions.ok && emptyActions.result.length === 0 &&
        emptyDecisions.ok && emptyDecisions.result.length === 0,
      { emptyNotes, emptyActions, emptyDecisions }
    );

    // NOTE on what "drive the UI" can and cannot mean here: reaching the
    // open-project screen at all requires clicking "Open existing vault" /
    // "Open" on a project, both of which are gated behind this app's own
    // native OS folder-picker dialog (`pick_directory`) for at least the
    // vault step -- CDP's synthetic input never reaches a native Win32
    // common dialog (the same limitation T04-01's evidence report already
    // recorded for automating this exact picker). Since this harness opens
    // the vault via a direct `vault_create` IPC call (bypassing the picker
    // on purpose, exactly like T04-01/T04-02's own harnesses), the running
    // page's React state was never told a vault is open, so clicking "Open"
    // here would be clicking a button that does not exist on the
    // vault-picker screen the app is actually still showing -- not a
    // meaningful UI assertion. The empty-state *backend* contract (above)
    // is proven live; the empty-state *copy* is proven by static source
    // inspection instead, honestly labeled as such rather than faked as a
    // live UI assertion.
    const appTsx = fs.readFileSync(path.join(DESKTOP_DIR, "src", "App.tsx"), "utf-8");
    const actionsPanelTsx = fs.readFileSync(path.join(DESKTOP_DIR, "src", "ActionsPanel.tsx"), "utf-8");
    const decisionsPanelTsx = fs.readFileSync(path.join(DESKTOP_DIR, "src", "DecisionsPanel.tsx"), "utf-8");
    record(
      "empty-state-ui-copy-present-in-source",
      appTsx.includes("No notes yet.") &&
        actionsPanelTsx.includes("No actions yet.") &&
        decisionsPanelTsx.includes("No decisions yet."),
      { checked: ["App.tsx", "ActionsPanel.tsx", "DecisionsPanel.tsx"] }
    );

    // --- Full workflow via IPC, mirroring the CLI workflow exactly ---
    const noteRes = await invokeCommand(cdp, "note_create", {
      vaultPath: desktopVaultPath,
      projectId,
      title: "Meeting notes",
      body: "Discussed the roadmap.",
    });
    record("note-create", noteRes.ok, noteRes);
    const noteId = noteRes.result.id;

    const actionCreateRes = await invokeCommand(cdp, "action_create", {
      vaultPath: desktopVaultPath,
      projectId,
      title: "Ship the update",
      body: null,
      dependsOn: [],
    });
    record("action-create", actionCreateRes.ok, actionCreateRes);
    const actionId = actionCreateRes.result.id;

    const startRes = await invokeCommand(cdp, "action_start", {
      vaultPath: desktopVaultPath,
      actionId,
      expectedRevisionId: actionCreateRes.result.revision_id,
    });
    record("action-start", startRes.ok && startRes.result.state === "doing", startRes);

    const completeRes = await invokeCommand(cdp, "action_complete", {
      vaultPath: desktopVaultPath,
      actionId,
      expectedRevisionId: startRes.result.revision_id,
      summary: "Shipped v1",
      overrideReason: null,
    });
    record("action-complete", completeRes.ok && completeRes.result.state === "done", completeRes);

    // Deliberate failure case, on a throwaway action so it never pollutes
    // the canonical-state-vs-CLI comparison below: `(Open,Cancelled)` and
    // `(Blocked,Cancelled)` are both allowed transitions (`project.rs`'s
    // own `action_allowed_transition`), so blocking an action then
    // replaying a *stale pre-block* `action_cancel` call passes Core's
    // transition-validity guard (current state `Blocked` still permits
    // `Cancelled`) and only then hits the real expected-revision conflict
    // check inside `commit_update` -- this is a cleaner conflict probe
    // than re-completing an already-`Done` action, which fails on the
    // transition guard itself (`Done -> Done` is simply never a valid
    // transition, independent of any revision) rather than on staleness.
    const conflictActionRes = await invokeCommand(cdp, "action_create", {
      vaultPath: desktopVaultPath,
      projectId,
      title: "Conflict probe action (not part of the CLI comparison)",
      body: null,
      dependsOn: [],
    });
    record("conflict-probe-action-create", conflictActionRes.ok, conflictActionRes);
    const conflictActionId = conflictActionRes.result.id;
    const preBlockRevisionId = conflictActionRes.result.revision_id;

    const blockRes = await invokeCommand(cdp, "action_block", {
      vaultPath: desktopVaultPath,
      actionId: conflictActionId,
      expectedRevisionId: preBlockRevisionId,
      reason: "blocked for conflict probe",
    });
    record("conflict-probe-block", blockRes.ok && blockRes.result.state === "blocked", blockRes);

    const conflictRes = await invokeCommand(cdp, "action_cancel", {
      vaultPath: desktopVaultPath,
      actionId: conflictActionId,
      expectedRevisionId: preBlockRevisionId, // stale on purpose: superseded by the block above
      reason: "should not land",
    });
    record(
      "action-conflict-refused",
      conflictRes.ok === false && conflictRes.error.includes("expected revision conflict"),
      conflictRes
    );
    const listAfterConflict = await invokeCommand(cdp, "list_actions", { vaultPath: desktopVaultPath, projectId });
    const conflictProbeAfter = listAfterConflict.ok
      ? listAfterConflict.result.find((a) => a.id === conflictActionId)
      : null;
    record(
      "action-conflict-did-not-overwrite",
      conflictProbeAfter?.state === "blocked",
      { conflictProbeAfter }
    );

    const decisionRes = await invokeCommand(cdp, "decision_create", {
      vaultPath: desktopVaultPath,
      projectId,
      key: "ship-strategy",
      statement: "Ship weekly",
      basis: "user_judgment",
      verification: "unreviewed",
      extra: {},
    });
    record("decision-create", decisionRes.ok, decisionRes);
    const decisionId = decisionRes.result.id;

    const acceptRes = await invokeCommand(cdp, "decision_accept", {
      vaultPath: desktopVaultPath,
      decisionId,
      expectedRevisionId: decisionRes.result.revision_id,
    });
    record("decision-accept", acceptRes.ok && acceptRes.result.lifecycle === "accepted", acceptRes);

    const relationRes = await invokeCommand(cdp, "relation_create", {
      vaultPath: desktopVaultPath,
      projectId,
      relationType: "relates_to",
      from: noteId,
      to: decisionId,
      note: "supports the plan",
    });
    record("relation-create", relationRes.ok, relationRes);

    // --- Resume view sanity check: the completed action must not appear in
    //     next_actions (terminal state), the accepted decision must appear
    //     in current_decisions. ---
    const resumeRes = await invokeCommand(cdp, "resume_view", { vaultPath: desktopVaultPath, projectId });
    const nextActionIds = resumeRes.ok ? resumeRes.result.next_actions.map(([id]) => id) : [];
    const currentDecisionKeys = resumeRes.ok
      ? resumeRes.result.current_decisions.map((d) => d.decision_key)
      : [];
    record(
      "resume-view-sane",
      resumeRes.ok && !nextActionIds.includes(actionId) && currentDecisionKeys.includes("ship-strategy"),
      { nextActionIds, currentDecisionKeys }
    );

    // --- Archived-project state: backend round-trip proven live; the UI
    //     copy for this state is proven by static source inspection, for
    //     the same native-dialog-navigation reason explained above the
    //     empty-state check. ---
    const archiveRes = await invokeCommand(cdp, "project_archive", { vaultPath: desktopVaultPath, projectId });
    record("project-archive", archiveRes.ok && archiveRes.result.active === false, archiveRes);
    const listAfterArchive = await invokeCommand(cdp, "list_projects", { vaultPath: desktopVaultPath });
    const archivedEntry = listAfterArchive.ok ? listAfterArchive.result.find((p) => p.id === projectId) : null;
    record("project-archive-visible-in-list", archivedEntry?.active === false, { archivedEntry });

    const unarchiveRes = await invokeCommand(cdp, "project_unarchive", { vaultPath: desktopVaultPath, projectId });
    record("project-unarchive", unarchiveRes.ok && unarchiveRes.result.active === true, unarchiveRes);

    record(
      "archived-state-ui-copy-present-in-source",
      appTsx.includes("Archived") && appTsx.includes("Unarchive"),
      { checked: ["App.tsx"] }
    );

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

  // --- Independent canonical-state comparison against the CLI vault ---
  if (desktopVaultPath) {
    try {
      const cliDb = path.join(cliVaultPath, ".fehrest", "canonical.sqlite");
      const desktopDb = path.join(desktopVaultPath, ".fehrest", "canonical.sqlite");
      const out = execFileSync("python", [path.join(__dirname, "compare_canonical_state.py"), cliDb, desktopDb], {
        encoding: "utf-8",
      });
      const parsed = JSON.parse(out);
      record("canonical-state-matches-cli", parsed.match === true, parsed);
    } catch (e) {
      let parsed = null;
      try {
        parsed = JSON.parse(e.stdout);
      } catch {}
      record("canonical-state-matches-cli", false, parsed ?? { error: String(e) });
    }
  }

  const overallOk = steps.every((s) => s.ok);
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, overallOk, tmpRoot, cliVaultPath, desktopVaultPath, steps }, null, 2));
  log(`=== run complete. overallOk=${overallOk}. json=${jsonPath} log=${logPath} ===`);
}

main().catch((e) => {
  log("FATAL: " + String(e?.stack || e));
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, fatal: String(e?.stack || e), steps }, null, 2));
  process.exitCode = 1;
});
