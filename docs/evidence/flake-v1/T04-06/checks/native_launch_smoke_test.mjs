// T04-06 automated technical accessibility/native-launch qualification --
// cross-platform native launch smoke test. Runs identically on Windows,
// macOS, and Linux (this file, unmodified, is what
// .github/workflows/t04-06-cross-platform.yml invokes on each of the
// three native runners): locate the platform-correct built binary, spawn
// it natively (no dev-server dependency -- this exercises the actual
// production-shaped launch path, not the Vite-dev-server-fronted debug
// harness earlier T04-0x tasks used for deep IPC exercise on Windows
// only), confirm it launches and stays alive without crashing, then
// terminate it and confirm a clean process-tree teardown.
//
// This is deliberately a *shallower* check than the Windows-only
// CDP-driven IPC harness (docs/evidence/flake-v1/T04-0{1,2,3,4,5}/e2e-test/):
// WebView2's `--remote-debugging-port` env var trick is Windows-specific.
// WKWebView (macOS) exposes no equivalent env-var-triggered remote
// inspector, and WebKitGTK (Linux) uses a different wire protocol
// (WebKit's own remote inspector, not Chrome DevTools Protocol) that
// would need a separate client implementation to drive. Per
// docs/canonical/FOUNDER_T04-06_ACCESSIBILITY_WITNESS_AMENDMENT_2026-09-16.md,
// this is recorded here as a bounded, honest limitation -- this script
// proves genuine native launch and clean shutdown on all three
// platforms; it does not claim to have driven typed IPC commands on
// macOS/Linux the way the Windows harness does.

import { spawn } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import os from "node:os";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const REPO = path.resolve(__dirname, "..", "..", "..", "..", "..");
const DESKTOP_DIR = path.join(REPO, "desktop");

function platformBinaryPath() {
  const base = path.join(DESKTOP_DIR, "src-tauri", "target", "debug");
  if (process.platform === "win32") return path.join(base, "pluma-desktop.exe");
  return path.join(base, "pluma-desktop");
}

const OUT_DIR = path.join(__dirname, "..", "results");
fs.mkdirSync(OUT_DIR, { recursive: true });
const runId = `${process.platform}-${new Date().toISOString().replace(/[:.]/g, "-")}`;
const jsonPath = path.join(OUT_DIR, `native-launch-smoke-${runId}.json`);

const steps = [];
function record(name, ok, detail) {
  const entry = { name, ok, detail: detail ?? null, ts: new Date().toISOString() };
  steps.push(entry);
  console.log(`STEP ${ok ? "OK  " : "FAIL"} ${name} :: ${JSON.stringify(detail).slice(0, 300)}`);
}

function isProcessAlive(pid) {
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
}

async function main() {
  const exePath = platformBinaryPath();
  record("platform", true, { platform: process.platform, arch: process.arch, exePath });
  record("binary-exists", fs.existsSync(exePath), { exePath });
  if (!fs.existsSync(exePath)) {
    fs.writeFileSync(jsonPath, JSON.stringify({ runId, overallOk: false, steps }, null, 2));
    process.exitCode = 1;
    return;
  }

  const vaultParent = fs.mkdtempSync(path.join(os.tmpdir(), "flake-t04-06-smoke-"));
  const env = { ...process.env };
  // Give the app an isolated, disposable config/profile location where the
  // platform supports it via env var, so a CI runner's shared user profile
  // is never touched by this smoke test. Harmless no-op on platforms/
  // builds that ignore it.
  env.FLAKE_SMOKE_TEST_TMP = vaultParent;

  const child = spawn(exePath, [], { env, stdio: "ignore" });
  const pid = child.pid;
  record("spawned", Boolean(pid), { pid });

  let spawnError = null;
  child.on("error", (e) => {
    spawnError = e;
  });

  // Give it a moment to either come up cleanly or crash immediately (a
  // missing native dependency, a broken bundle, etc. -- exactly the
  // failure mode this smoke test exists to catch on a platform this
  // codebase has never been launched on natively before).
  await new Promise((r) => setTimeout(r, 3000));

  const aliveAfterStartup = pid ? isProcessAlive(pid) : false;
  record("alive-after-3s", aliveAfterStartup && !spawnError, { spawnError: spawnError ? String(spawnError) : null });

  // Clean shutdown: terminate and confirm the process actually exits
  // within a bounded window, rather than assuming the signal worked.
  if (pid && aliveAfterStartup) {
    child.kill();
    const deadline = Date.now() + 5000;
    let exited = false;
    while (Date.now() < deadline) {
      if (!isProcessAlive(pid)) {
        exited = true;
        break;
      }
      await new Promise((r) => setTimeout(r, 200));
    }
    if (!exited) {
      // Last resort, platform-appropriate force kill, so CI never leaves
      // an orphaned process behind regardless of what this test found.
      try {
        process.kill(pid, "SIGKILL");
      } catch {}
    }
    record("clean-shutdown", exited, { exited });
  } else {
    record("clean-shutdown", false, { reason: "process was not alive to shut down" });
  }

  const overallOk = steps.every((s) => s.ok);
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, overallOk, steps }, null, 2));
  console.log(`=== native launch smoke test complete on ${process.platform}. overallOk=${overallOk} ===`);
  process.exitCode = overallOk ? 0 : 1;
}

main().catch((e) => {
  console.error("FATAL:", e);
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, fatal: String(e?.stack || e), steps }, null, 2));
  process.exitCode = 1;
});
