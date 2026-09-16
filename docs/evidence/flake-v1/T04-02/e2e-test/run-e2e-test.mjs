// T04-02 note editor E2E test runner.
//
// Drives the real compiled flake-desktop.exe through its exact real typed
// IPC bridge (window.__TAURI_INTERNALS__.invoke, the same transport
// @tauri-apps/api's invoke() uses internally) over a loopback-only WebView2
// CDP connection -- the same mechanism T04-01's network-denied test harness
// established and validated.
//
// Every claimed save is independently cross-checked by reading
// canonical.sqlite directly (via tools/independent-verify/sqlite_reader.py,
// unchanged, through the thin independent_read_note.py wrapper in this
// directory) -- never by trusting the IPC response alone. This is the
// "exact before/after canonical payload comparison" the T04-02 task
// contract's own verification method names.

import { spawn, execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import os from "node:os";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const REPO = path.resolve(__dirname, "..", "..", "..", "..", ".."); // .../Flake
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

function independentReadNote(dbPath, objectId) {
  const out = execFileSync("python", [path.join(__dirname, "independent_read_note.py"), dbPath, objectId], {
    encoding: "utf-8",
  });
  return JSON.parse(out);
}

async function killTree(pid) {
  if (!pid) return;
  await new Promise((resolve) => {
    const p = spawn("taskkill.exe", ["/PID", String(pid), "/T", "/F"]);
    p.on("close", resolve);
    p.on("error", resolve);
  });
}

// Deliberately not Arabic script (repository-wide instruction for this
// session): CJK, an emoji, and a combining diacritical mark, to exercise
// multi-byte UTF-8 and combining-character byte-exact preservation.
const UNICODE_PROBE = "添加注释 ✨ émigré café";

async function sh(cmd, args, opts = {}) {
  return new Promise((resolve) => {
    const child = spawn(cmd, args, { shell: false, ...opts });
    let out = "";
    child.stdout?.on("data", (d) => (out += d.toString()));
    child.on("close", (code) => resolve({ code, out }));
    child.on("error", () => resolve({ code: -1, out }));
  });
}

async function main() {
  log(`=== T04-02 note editor E2E test run ${runId} ===`);
  record("exe-exists", fs.existsSync(EXE), { path: EXE });

  const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "flake-t04-02-e2e-"));
  const vaultParent = path.join(tmpRoot, "parent");
  fs.mkdirSync(vaultParent, { recursive: true });

  const vite = spawn("npm", ["run", "dev"], {
    cwd: DESKTOP_DIR,
    stdio: ["ignore", "pipe", "pipe"],
    shell: true,
  });
  try {
    await waitFor(async () => {
      const res = await fetch("http://127.0.0.1:1420/").catch(() => null);
      return res && res.ok;
    }, 15000);
    record("vite-dev-server-ready", true, {});
  } catch (e) {
    record("vite-dev-server-ready", false, { error: String(e) });
  }

  const port = 9501;
  let root = null;
  try {
    const child = spawn(EXE, [], {
      env: { ...process.env, WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${port}` },
      stdio: "ignore",
    });
    root = child.pid;
    record("launch", true, { pid: root });

    const cdp = await connectCdp(port);
    record("bridge-ready", true, { url: cdp.pageUrl });

    const vaultRes = await invokeCommand(cdp, "vault_create", { parentDir: vaultParent, name: "t04-02-vault" });
    record("vault-create", vaultRes.ok, vaultRes);
    const vaultPath = vaultRes.result.path;
    const dbPath = path.join(vaultPath, ".fehrest", "canonical.sqlite");

    const projRes = await invokeCommand(cdp, "create_project", {
      vaultPath,
      name: "T04-02 E2E Project",
      description: null,
    });
    record("project-create", projRes.ok, projRes);
    const projectId = projRes.result.id;

    // --- 1. Create note, independently verify exact saved bytes ---
    const initialBody = `# Heading\n\n**bold** and *italic* and \`code\`\n\n- item one\n- item two\n\n[a link](https://example.invalid/x) ![an image](https://example.invalid/y.png)\n\n${UNICODE_PROBE}`;
    const createRes = await invokeCommand(cdp, "note_create", {
      vaultPath,
      projectId,
      title: "E2E note",
      body: initialBody,
    });
    record("note-create", createRes.ok && createRes.result.body === initialBody, createRes);
    const noteId = createRes.result.id;
    const rev1 = createRes.result.revision_id;

    const indep1 = independentReadNote(dbPath, noteId);
    record(
      "note-create-independent-verify",
      indep1.payload?.body === initialBody && indep1.revision_id === rev1,
      { expectedBody: initialBody, actualBody: indep1.payload?.body, revisionMatch: indep1.revision_id === rev1 }
    );

    // --- 2. Update note (simulating an owner edit + save), independently verify ---
    const updatedBody = initialBody + "\n\nAppended after first save.";
    const updateRes = await invokeCommand(cdp, "note_update", {
      vaultPath,
      noteId,
      expectedRevisionId: rev1,
      title: "E2E note (edited)",
      body: updatedBody,
    });
    record("note-update", updateRes.ok && updateRes.result.body === updatedBody, updateRes);
    const rev2 = updateRes.result.revision_id;
    record("note-update-revision-advanced", rev2 !== rev1, { rev1, rev2 });

    const indep2 = independentReadNote(dbPath, noteId);
    record(
      "note-update-independent-verify",
      indep2.payload?.body === updatedBody && indep2.revision_id === rev2,
      { expectedBody: updatedBody, actualBody: indep2.payload?.body, revisionMatch: indep2.revision_id === rev2 }
    );

    // --- 3. Conflict: save against the now-stale rev1 must be refused, and
    //        must NOT silently overwrite the already-committed rev2 state ---
    const conflictAttemptBody = "an attacker/stale-editor write that must never land";
    const conflictRes = await invokeCommand(cdp, "note_update", {
      vaultPath,
      noteId,
      expectedRevisionId: rev1, // stale on purpose
      title: "should not land",
      body: conflictAttemptBody,
    });
    record(
      "conflict-refused",
      conflictRes.ok === false && conflictRes.error.includes("expected revision conflict"),
      conflictRes
    );

    const indep3 = independentReadNote(dbPath, noteId);
    record(
      "conflict-did-not-overwrite",
      indep3.payload?.body === updatedBody && indep3.revision_id === rev2,
      { expectedBody: updatedBody, actualBody: indep3.payload?.body }
    );

    // --- 4. Large body (close to Core's 1 MiB MAX_OBJECT_BYTES limit) saved
    //        and preserved byte-exact ---
    const largeBody = "x".repeat(900_000) + UNICODE_PROBE;
    const largeRes = await invokeCommand(cdp, "note_update", {
      vaultPath,
      noteId,
      expectedRevisionId: rev2,
      title: "large body",
      body: largeBody,
    });
    record("large-body-save", largeRes.ok === true, { ok: largeRes.ok, bodyLength: largeRes.result?.body?.length });
    const rev3 = largeRes.result?.revision_id;

    const indep4 = independentReadNote(dbPath, noteId);
    const largeBodyBytes = Buffer.byteLength(largeBody, "utf-8");
    const actualBytes = indep4.payload ? Buffer.byteLength(indep4.payload.body, "utf-8") : -1;
    record("large-body-independent-verify", indep4.payload?.body === largeBody && actualBytes === largeBodyBytes, {
      expectedBytes: largeBodyBytes,
      actualBytes,
    });

    // --- 5. Over Core's own 1 MiB limit is refused by Core, not silently
    //        accepted or truncated by this bridge ---
    const overLimitBody = "y".repeat(1_100_000);
    const overLimitRes = await invokeCommand(cdp, "note_update", {
      vaultPath,
      noteId,
      expectedRevisionId: rev3,
      title: "over limit",
      body: overLimitBody,
    });
    record("over-limit-refused", overLimitRes.ok === false, overLimitRes);
    const indep5 = independentReadNote(dbPath, noteId);
    record("over-limit-did-not-land", indep5.payload?.body === largeBody, {
      stillLargeBody: indep5.payload?.body === largeBody,
    });
  } catch (e) {
    record("e2e-flow", false, { error: String(e?.stack || e) });
  } finally {
    if (root) {
      await killTree(root);
    }
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
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, overallOk, tmpRoot, steps }, null, 2));
  log(`=== run complete. overallOk=${overallOk}. json=${jsonPath} log=${logPath} ===`);
}

main().catch((e) => {
  log("FATAL: " + String(e?.stack || e));
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, fatal: String(e?.stack || e), steps }, null, 2));
  process.exitCode = 1;
});
