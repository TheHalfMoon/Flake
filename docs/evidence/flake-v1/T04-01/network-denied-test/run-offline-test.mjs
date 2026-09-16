// T04-01 network-denied functional test runner.
//
// Self-contained: starts its own local Vite dev server (loopback only),
// launches the real compiled flake-desktop.exe with WebView2 remote
// debugging enabled on a loopback port, and drives the exact same typed
// IPC bridge the real UI uses (`window.__TAURI_INTERNALS__.invoke`) to
// exercise vault create/open, project list/create, a simulated
// restart/reopen, and clean shutdown -- entirely through the real running
// app process, never through a mocked/duplicated code path.
//
// Designed to run unattended while the host has no external network
// connectivity: every step it performs is either a local child process
// (this exe, the local dev server) or a loopback (127.0.0.1) connection,
// which does not require a working physical network adapter. The one
// deliberate outbound attempt (the connectivity canary) is expected to
// fail when network is denied -- that failure is itself the evidence,
// not a dependency of this test succeeding.
//
// All results are appended to disk incrementally (log line by line, plus
// a final JSON summary) so a run survives this controlling session
// losing connectivity to anything external, including the Claude Code
// session that launched it.

import { spawn } from "node:child_process";
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
const netLogPath = path.join(OUT_DIR, `run-${runId}.network-monitor.log`);

const steps = [];
function log(msg) {
  const line = `[${new Date().toISOString()}] ${msg}`;
  console.log(line);
  fs.appendFileSync(logPath, line + "\n");
}
function record(name, ok, detail) {
  const entry = { name, ok, detail: detail ?? null, ts: new Date().toISOString() };
  steps.push(entry);
  log(`STEP ${ok ? "OK  " : "FAIL"} ${name}${detail !== undefined ? " :: " + JSON.stringify(detail) : ""}`);
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, steps }, null, 2));
  return entry;
}

function sh(cmd, args, opts = {}) {
  return new Promise((resolve) => {
    const child = spawn(cmd, args, { shell: false, ...opts });
    let out = "";
    let err = "";
    child.stdout?.on("data", (d) => (out += d.toString()));
    child.stderr?.on("data", (d) => (err += d.toString()));
    child.on("close", (code) => resolve({ code, out, err }));
    child.on("error", (e) => resolve({ code: -1, out, err: String(e) }));
  });
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
  await sh("taskkill.exe", ["/PID", String(pid), "/T", "/F"]);
}

async function processExists(pid) {
  const r = await sh("tasklist.exe", ["/FI", `PID eq ${pid}`, "/FO", "CSV"]);
  return r.out.split("\n").length > 2; // header + at least one row
}

async function rawTcpProbe(host, port, timeoutMs) {
  const net = await import("node:net");
  return new Promise((resolve) => {
    const sock = new net.Socket();
    let done = false;
    const finish = (reachable) => {
      if (done) return;
      done = true;
      sock.destroy();
      resolve(reachable);
    };
    sock.setTimeout(timeoutMs);
    sock.once("connect", () => finish(true));
    sock.once("timeout", () => finish(false));
    sock.once("error", () => finish(false));
    sock.connect(port, host);
  });
}

async function checkConnectivity() {
  // Two independent methods, both must fail to call it offline: an HTTP
  // fetch of a lightweight, well-known connectivity-probe endpoint, and a
  // raw TCP connect attempt to a public DNS resolver's port 443 -- this
  // way a single flaky/blocked hostname or one blocked port never produces
  // a false "offline" or a false "online" reading on its own.
  let httpReachable = false;
  try {
    const res = await fetch("http://www.msftconnecttest.com/connecttest.txt", {
      signal: AbortSignal.timeout(3000),
    });
    httpReachable = res.ok;
  } catch {
    httpReachable = false;
  }
  const tcpReachable = await rawTcpProbe("1.1.1.1", 443, 3000);
  return { httpReachable, tcpReachable, anyReachable: httpReachable || tcpReachable };
}

async function main() {
  log(`=== T04-01 network-denied test run ${runId} ===`);
  log(`repo=${REPO}`);
  log(`exe=${EXE}`);
  record("exe-exists", fs.existsSync(EXE), { path: EXE });

  // 1. Fail-closed precondition gate: this test's whole point is to prove
  //    behavior when the host genuinely has no external connectivity. Poll
  //    (bounded) for that to become true rather than trusting an operator's
  //    claim; if it never becomes true, STOP here and record a FAIL rather
  //    than silently exercising the app while still online and calling that
  //    a network-denied result.
  const precondition = await waitFor(async () => {
    const c = await checkConnectivity();
    log(`connectivity poll: http=${c.httpReachable} tcp=${c.tcpReachable}`);
    return c.anyReachable ? null : c;
  }, 240000, 3000).catch(() => null);

  if (!precondition) {
    const finalCheck = await checkConnectivity();
    record("network-denied-precondition", false, {
      reason: "external connectivity was still reachable after the full wait window; aborting rather than reporting a false network-denied result",
      finalCheck,
    });
    fs.writeFileSync(
      jsonPath,
      JSON.stringify({ runId, overallOk: false, aborted: true, reason: "precondition never met", steps }, null, 2)
    );
    log("=== ABORTED: network-denied precondition never met. No functional test was run. ===");
    return;
  }
  record("network-denied-precondition", true, precondition);

  // 2. Background network-attempt monitor (loopback-independent; records any
  //    non-loopback TCP connection attempt by any process for the duration).
  const monitorScript = `
    while ($true) {
      try {
        Get-NetTCPConnection -ErrorAction SilentlyContinue |
          Where-Object { $_.RemoteAddress -ne '127.0.0.1' -and $_.RemoteAddress -ne '::1' -and $_.RemoteAddress -ne '0.0.0.0' -and $_.RemoteAddress -ne '::' } |
          ForEach-Object {
            "$(Get-Date -Format o) PID=$($_.OwningProcess) Local=$($_.LocalAddress):$($_.LocalPort) Remote=$($_.RemoteAddress):$($_.RemotePort) State=$($_.State)"
          } | Out-File -FilePath '${netLogPath.replace(/\\/g, "\\\\")}' -Append -Encoding utf8
      } catch {}
      Start-Sleep -Milliseconds 500
    }
  `;
  const monitor = spawn("powershell.exe", ["-NoProfile", "-NonInteractive", "-Command", monitorScript], {
    stdio: "ignore",
  });
  record("network-monitor-started", true, { pid: monitor.pid, logPath: netLogPath });

  // 3. Local Vite dev server (loopback only; serves the exact same App.tsx
  //    source this app's production bundle is built from).
  const vite = spawn("npm", ["run", "dev"], {
    cwd: DESKTOP_DIR,
    stdio: ["ignore", "pipe", "pipe"],
    shell: true, // Windows: npm resolves to npm.cmd, which requires a shell (Node CVE-2024-27980 hardening)
  });
  let viteOk = false;
  try {
    await waitFor(async () => {
      const res = await fetch("http://127.0.0.1:1420/").catch(() => null);
      return res && res.ok;
    }, 15000);
    viteOk = true;
  } catch (e) {
    record("vite-dev-server-ready", false, { error: String(e) });
  }
  if (viteOk) record("vite-dev-server-ready", true, { url: "http://127.0.0.1:1420/" });

  const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "flake-offline-test-"));
  const vaultParent = path.join(tmpRoot, "parent");
  fs.mkdirSync(vaultParent, { recursive: true });
  const vaultName = "offline-test-vault";
  const expectedVaultPath = path.join(vaultParent, vaultName);
  let vaultInfo1 = null;

  // --- Launch #1: cold launch, vault create, project create ---
  let root1 = null;
  try {
    const port1 = 9401;
    const child1 = spawn(EXE, [], {
      env: { ...process.env, WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${port1}` },
      stdio: "ignore",
    });
    root1 = child1.pid;
    record("launch-1-spawned", true, { pid: root1 });

    const cdp1 = await connectCdp(port1);
    record("launch-1-page-loaded", true, { url: cdp1.pageUrl });

    const bridgeType = await waitFor(async () => {
      const r = await cdp1.call("Runtime.evaluate", {
        expression: "typeof window.__TAURI_INTERNALS__?.invoke",
        returnByValue: true,
      });
      return r.result.value === "function" ? r.result.value : null;
    }, 15000, 200).catch(() => "undefined");
    record("launch-1-bridge-present", bridgeType === "function", bridgeType);

    const createRes = await invokeCommand(cdp1, "vault_create", {
      parentDir: vaultParent,
      name: vaultName,
    });
    record("vault-create", createRes.ok === true, createRes);
    if (createRes.ok) vaultInfo1 = createRes.result;

    if (vaultInfo1) {
      const listEmpty = await invokeCommand(cdp1, "list_projects", { vaultPath: vaultInfo1.path });
      record("list-projects-empty", listEmpty.ok === true && Array.isArray(listEmpty.result) && listEmpty.result.length === 0, listEmpty);

      const createProj = await invokeCommand(cdp1, "create_project", {
        vaultPath: vaultInfo1.path,
        name: "Offline Continuity Check",
        description: "T04-01 network-denied test project",
      });
      record("create-project", createProj.ok === true, createProj);

      const listOne = await invokeCommand(cdp1, "list_projects", { vaultPath: vaultInfo1.path });
      record(
        "list-projects-one",
        listOne.ok === true && Array.isArray(listOne.result) && listOne.result.length === 1,
        listOne
      );
    }

    cdp1.ws.close();
  } catch (e) {
    record("launch-1-flow", false, { error: String(e) });
  } finally {
    if (root1) {
      const existedBefore = await processExists(root1);
      await killTree(root1);
      await new Promise((r) => setTimeout(r, 500));
      const existsAfter = await processExists(root1);
      record("launch-1-clean-shutdown", existedBefore && !existsAfter, { existedBefore, existsAfter });
    }
  }

  // --- Launch #2: simulated restart -- reopen the same vault, verify persistence ---
  let root2 = null;
  try {
    if (!vaultInfo1) throw new Error("no vault from launch 1; skipping restart verification");
    const port2 = 9402;
    const child2 = spawn(EXE, [], {
      env: { ...process.env, WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${port2}` },
      stdio: "ignore",
    });
    root2 = child2.pid;
    record("launch-2-spawned", true, { pid: root2 });

    const cdp2 = await connectCdp(port2);
    record("launch-2-page-loaded", true, { url: cdp2.pageUrl });

    const bridgeType2 = await waitFor(async () => {
      const r = await cdp2.call("Runtime.evaluate", {
        expression: "typeof window.__TAURI_INTERNALS__?.invoke",
        returnByValue: true,
      });
      return r.result.value === "function" ? r.result.value : null;
    }, 15000, 200).catch(() => "undefined");
    record("launch-2-bridge-present", bridgeType2 === "function", bridgeType2);

    const openRes = await invokeCommand(cdp2, "vault_open", { path: vaultInfo1.path });
    record(
      "vault-open-after-restart",
      openRes.ok === true && openRes.result.vault_id === vaultInfo1.vault_id,
      { openRes, expectedVaultId: vaultInfo1.vault_id }
    );

    const listAfterRestart = await invokeCommand(cdp2, "list_projects", { vaultPath: vaultInfo1.path });
    const persisted =
      listAfterRestart.ok === true &&
      Array.isArray(listAfterRestart.result) &&
      listAfterRestart.result.length === 1 &&
      listAfterRestart.result[0].name === "Offline Continuity Check";
    record("project-persisted-across-restart", persisted, listAfterRestart);

    cdp2.ws.close();
  } catch (e) {
    record("launch-2-flow", false, { error: String(e) });
  } finally {
    if (root2) {
      const existedBefore = await processExists(root2);
      await killTree(root2);
      await new Promise((r) => setTimeout(r, 500));
      const existsAfter = await processExists(root2);
      record("launch-2-clean-shutdown", existedBefore && !existsAfter, { existedBefore, existsAfter });
    }
  }

  // Teardown: dev server + monitor.
  // `vite` was spawned with shell:true (required on Windows for npm.cmd),
  // so vite.kill() would only terminate the cmd.exe wrapper, not the actual
  // node/vite process bound to the port. Find and kill the real listener
  // by port instead.
  try {
    const netstat = await sh("netstat.exe", ["-ano"]);
    const line = netstat.out.split("\n").find((l) => l.includes(":1420") && l.includes("LISTENING"));
    const pid = line?.trim().split(/\s+/).pop();
    if (pid) {
      await killTree(pid);
      record("vite-dev-server-stopped", true, { pid });
    } else {
      record("vite-dev-server-stopped", false, { reason: "listener not found on port 1420" });
    }
  } catch (e) {
    record("vite-dev-server-stopped", false, { error: String(e) });
  }
  try {
    vite.kill();
  } catch {}
  try {
    monitor.kill();
  } catch {}

  record("expected-vault-path-matches", vaultInfo1 ? vaultInfo1.path === expectedVaultPath : false, {
    actual: vaultInfo1?.path,
    expected: expectedVaultPath,
  });

  const overallOk = steps.every((s) => s.ok);
  fs.writeFileSync(
    jsonPath,
    JSON.stringify(
      {
        runId,
        overallOk,
        vaultInfo1,
        tmpRoot,
        netLogPath,
        steps,
      },
      null,
      2
    )
  );
  log(`=== run complete. overallOk=${overallOk}. json=${jsonPath} log=${logPath} netlog=${netLogPath} ===`);
}

main().catch((e) => {
  log("FATAL: " + String(e?.stack || e));
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, fatal: String(e?.stack || e), steps }, null, 2));
  process.exitCode = 1;
});
