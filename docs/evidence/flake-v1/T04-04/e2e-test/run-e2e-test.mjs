// T04-04 resumption/checkpoint/source-status/package/proposal review E2E
// test runner. Reuses T04-01/T04-02/T04-03's established CDP-driven
// harness pattern to drive the real compiled flake-desktop.exe through
// its exact real typed-IPC bridge.
//
// "Desktop package/proposal result matches CLI" is proven as a genuine
// interoperability property, in both directions, on one shared vault:
// a desktop-issued grant/receipt is admitted and accepted through the
// CLI, and a CLI-issued grant/receipt is admitted and accepted through
// the desktop -- each side reading back what the other produced, since
// disclosure packages and proposals are exactly the kind of artifact
// meant to cross a tool boundary (§18's own wire protocol). A same-vault,
// same-grant/request package-preview additionally proves byte-identical
// (via emitted_sha256) deterministic compilation between the CLI and
// desktop code paths.

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
    let err = "";
    child.stderr?.on("data", (d) => (err += d.toString()));
    child.on("close", (code) => resolve({ code, out, err }));
    child.on("error", () => resolve({ code: -1, out, err }));
  });
}

function cli(args) {
  const r = execFileSync(CLI_EXE, args, { encoding: "utf-8" });
  return r.trim();
}
function cliMayFail(args) {
  try {
    return { ok: true, out: cli(args) };
  } catch (e) {
    return { ok: false, out: String(e.stdout || "") + String(e.stderr || e) };
  }
}

async function main() {
  log(`=== T04-04 resume/checkpoint/source/package/proposal E2E run ${runId} ===`);
  record("exe-exists", fs.existsSync(EXE), { path: EXE });
  record("cli-exe-exists", fs.existsSync(CLI_EXE), { path: CLI_EXE });

  const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "flake-t04-04-e2e-"));

  const vite = spawn("npm", ["run", "dev"], { cwd: DESKTOP_DIR, stdio: ["ignore", "pipe", "pipe"], shell: true });
  try {
    await waitFor(async () => (await fetch("http://127.0.0.1:1420/").catch(() => null))?.ok, 15000);
    record("vite-dev-server-ready", true, {});
  } catch (e) {
    record("vite-dev-server-ready", false, { error: String(e) });
  }

  const port = 9701;
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
    const vaultRes = await invokeCommand(cdp, "vault_create", { parentDir: vaultParent, name: "shared-vault" });
    record("vault-create", vaultRes.ok, vaultRes);
    vaultPath = vaultRes.result.path;

    const projRes = await invokeCommand(cdp, "create_project", {
      vaultPath,
      name: "T04-04 Project",
      description: null,
    });
    record("project-create", projRes.ok, projRes);
    const projectId = projRes.result.id;

    const note1Res = await invokeCommand(cdp, "note_create", {
      vaultPath,
      projectId,
      title: "First note",
      body: "Content for disclosure.",
    });
    record("note1-create", note1Res.ok, note1Res);
    const note1Id = note1Res.result.id;

    // === Checkpoint round trip ===
    const cpNone = await invokeCommand(cdp, "checkpoint_current", { vaultPath, projectId });
    record("checkpoint-current-initially-null", cpNone.ok && cpNone.result === null, cpNone);

    const cpMark1 = await invokeCommand(cdp, "checkpoint_mark", {
      vaultPath,
      projectId,
      expectedRevisionId: null,
      through: null,
    });
    record("checkpoint-mark-first", cpMark1.ok, cpMark1);

    const note2Res = await invokeCommand(cdp, "note_create", {
      vaultPath,
      projectId,
      title: "Second note",
      body: "Created after first checkpoint.",
    });
    record("note2-create", note2Res.ok, note2Res);

    const cpMark2 = await invokeCommand(cdp, "checkpoint_mark", {
      vaultPath,
      projectId,
      expectedRevisionId: cpMark1.result.revision_id,
      through: null,
    });
    record(
      "checkpoint-mark-advances",
      cpMark2.ok && cpMark2.result.reviewed_through_seq > cpMark1.result.reviewed_through_seq,
      cpMark2
    );

    const cpConflict = await invokeCommand(cdp, "checkpoint_mark", {
      vaultPath,
      projectId,
      expectedRevisionId: cpMark1.result.revision_id, // stale on purpose
      through: null,
    });
    record(
      "checkpoint-mark-conflict-refused",
      cpConflict.ok === false && cpConflict.error.includes("expected revision conflict"),
      cpConflict
    );

    const cpReset = await invokeCommand(cdp, "checkpoint_reset", {
      vaultPath,
      projectId,
      expectedRevisionId: cpMark2.result.revision_id,
      through: 0,
      reason: "test reset back to the beginning",
    });
    record(
      "checkpoint-reset",
      cpReset.ok && cpReset.result.reviewed_through_seq === 0 && cpReset.result.reset_reason,
      cpReset
    );

    // === Source status: Unchecked -> Match -> Changed -> Missing ===
    const sourceFile = path.join(tmpRoot, "source.txt");
    fs.writeFileSync(sourceFile, "version 1");
    const importOut = cli(["source-import", "--vault", vaultPath, "--project", projectId, "--label", "Test Source", "--path", sourceFile]);
    const sourceId = importOut.split(/\s+/)[0];
    record("source-import-via-cli", Boolean(sourceId), { sourceId });

    const sourcesUnchecked = await invokeCommand(cdp, "list_sources", { vaultPath, projectId });
    const entryUnchecked = sourcesUnchecked.ok ? sourcesUnchecked.result.find((s) => s.id === sourceId) : null;
    record("source-initially-unchecked", entryUnchecked?.latest_check_status === null, entryUnchecked);

    const check1 = await invokeCommand(cdp, "source_check_now", { vaultPath, sourceId });
    record("source-check-match", check1.ok && check1.result.status === "match", check1);

    fs.writeFileSync(sourceFile, "version 2 -- changed");
    const check2 = await invokeCommand(cdp, "source_check_now", { vaultPath, sourceId });
    record("source-check-changed", check2.ok && check2.result.status === "changed", check2);

    fs.unlinkSync(sourceFile);
    const check3 = await invokeCommand(cdp, "source_check_now", { vaultPath, sourceId });
    record("source-check-missing", check3.ok && check3.result.status === "missing", check3);

    const sourcesFinal = await invokeCommand(cdp, "list_sources", { vaultPath, projectId });
    const entryFinal = sourcesFinal.ok ? sourcesFinal.result.find((s) => s.id === sourceId) : null;
    record("source-list-reflects-latest-check", entryFinal?.latest_check_status === "missing", entryFinal);

    // === Grant + package preview: deterministic compilation, CLI parity ===
    const grantRes = await invokeCommand(cdp, "grant_issue", {
      vaultPath,
      projectId,
      allowedKinds: ["note"],
      byteBudget: 65536,
      ttlSecs: 3600,
      options: {},
    });
    record("grant-issue-desktop", grantRes.ok, grantRes);
    const grantId = grantRes.result.id;

    const previewDesktop = await invokeCommand(cdp, "package_preview", {
      vaultPath,
      grantId,
      requestId: "req-parity-check",
      principal: "tester",
    });
    record("package-preview-desktop", previewDesktop.ok, {
      emitted_sha256: previewDesktop.result?.receipt?.emitted_sha256,
    });

    const cliPreviewOut = cli([
      "package-preview",
      "--vault", vaultPath,
      "--grant", grantId,
      "--request-id", "req-parity-check",
      "--principal", "tester",
    ]);
    const cliSha256Match = /emitted_sha256=(\S+)/.exec(cliPreviewOut)?.[1];
    record(
      "package-preview-matches-cli",
      previewDesktop.ok && previewDesktop.result.receipt.emitted_sha256 === cliSha256Match,
      { desktop: previewDesktop.result?.receipt?.emitted_sha256, cli: cliSha256Match }
    );

    // === Expired grant is refused ===
    const shortGrantRes = await invokeCommand(cdp, "grant_issue", {
      vaultPath,
      projectId,
      allowedKinds: ["note"],
      byteBudget: 65536,
      ttlSecs: 1,
      options: {},
    });
    record("grant-issue-short-ttl", shortGrantRes.ok, shortGrantRes);
    await new Promise((r) => setTimeout(r, 2200));
    const expiredPreview = await invokeCommand(cdp, "package_preview", {
      vaultPath,
      grantId: shortGrantRes.result.id,
      requestId: "req-expired",
      principal: "tester",
    });
    record(
      "expired-grant-refused",
      expiredPreview.ok === false && expiredPreview.error.includes("expired"),
      expiredPreview
    );

    // === Proposal interop, direction A: desktop-compiled receipt, CLI admits+accepts ===
    const compileA = await invokeCommand(cdp, "package_compile", {
      vaultPath,
      grantId,
      requestId: "req-a-desktop-compile",
      principal: "tester",
    });
    record("package-compile-desktop", compileA.ok, { receipt_id: compileA.result?.receipt_id });
    const receiptA = compileA.result.receipt_id;

    const proposalAJson = JSON.stringify({
      receipt_id: receiptA,
      declared_agent: "test-agent-a",
      operations: [
        { kind: "draft_decision", decision_key: "from-desktop-receipt", statement: "Decided via desktop receipt, CLI accept" },
      ],
    });
    const proposalAFile = path.join(tmpRoot, "proposal-a.json");
    fs.writeFileSync(proposalAFile, proposalAJson);
    const importAOut = cli(["propose-import", "--vault", vaultPath, "--project", projectId, "--file", proposalAFile]);
    const proposalAId = importAOut.split(/\s+/)[0];
    const proposalARev = importAOut.split(/\s+/)[1];
    record("proposal-a-admit-via-cli", Boolean(proposalAId), { proposalAId });

    const acceptAOut = cli(["propose-accept", "--vault", vaultPath, "--id", proposalAId, "--expect", proposalARev, "--select", "0"]);
    record("proposal-a-accept-via-cli", acceptAOut.includes("Accepted"), { acceptAOut });

    const decisionsAfterA = await invokeCommand(cdp, "list_decisions", { vaultPath, projectId });
    const decisionA = decisionsAfterA.ok
      ? decisionsAfterA.result.find((d) => d.decision_key === "from-desktop-receipt")
      : null;
    record(
      "proposal-a-result-visible-to-desktop",
      decisionA?.statement === "Decided via desktop receipt, CLI accept",
      decisionA
    );

    // === Proposal interop, direction B: CLI-issued grant + CLI-compiled
    //     receipt, desktop admits+accepts ===
    const grantBOut = cli(["grant-issue", "--vault", vaultPath, "--project", projectId, "--kinds", "note", "--budget", "65536", "--ttl-hours", "1"]);
    const grantBId = grantBOut.split(/\s+/)[0];
    const packageBFile = path.join(tmpRoot, "package-b.txt");
    const exportBOut = cli(["package-export", "--vault", vaultPath, "--grant", grantBId, "--request-id", "req-b-cli-compile", "--principal", "tester", "--out", packageBFile]);
    const receiptB = /receipt_id=(\S+)/.exec(exportBOut)?.[1];
    record("package-compile-via-cli", Boolean(receiptB), { grantBId, receiptB });

    const proposalBJson = JSON.stringify({
      receipt_id: receiptB,
      declared_agent: "test-agent-b",
      operations: [
        { kind: "draft_decision", decision_key: "from-cli-receipt", statement: "Decided via CLI receipt, desktop accept" },
      ],
    });
    const admitBRes = await invokeCommand(cdp, "proposal_admit", { vaultPath, projectId, rawText: proposalBJson });
    record("proposal-b-admit-via-desktop", admitBRes.ok, admitBRes);

    const acceptBRes = await invokeCommand(cdp, "proposal_accept", {
      vaultPath,
      proposalId: admitBRes.result.id,
      expectedRevisionId: admitBRes.result.revision_id,
      selectedIndices: [0],
    });
    record("proposal-b-accept-via-desktop", acceptBRes.ok && acceptBRes.result.status === "accepted", acceptBRes);

    // The decision object's own ID is not directly returned by `accept_proposal`
    // (it only echoes the *proposal's* object_id/revision); verify the
    // resulting decision via `list_decisions` instead.
    const decisionsAfterB = await invokeCommand(cdp, "list_decisions", { vaultPath, projectId });
    const decisionB = decisionsAfterB.ok
      ? decisionsAfterB.result.find((d) => d.decision_key === "from-cli-receipt")
      : null;
    record("proposal-b-result-correct", decisionB?.statement === "Decided via CLI receipt, desktop accept", decisionB);

    // Security property, confirmed rather than assumed: accepting a
    // *proposal* only admits its DraftDecision as Core's own `Draft`
    // lifecycle -- it never auto-promotes to `Accepted`. An agent's
    // content is evidence, never authority (S16); only a *separate*,
    // explicit owner `decision_accept` call can make a proposed decision
    // the project's live current answer. Confirmed from both sides:
    const cliDecisionBeforeAccept = cliMayFail(["decision-state", "--vault", vaultPath, "--project", projectId, "--key", "from-cli-receipt"]);
    record(
      "proposal-acceptance-alone-does-not-grant-decision-authority",
      cliDecisionBeforeAccept.ok &&
        cliDecisionBeforeAccept.out.includes("NoAcceptedDecision") &&
        cliDecisionBeforeAccept.out.includes("lifecycle is Draft, not Accepted"),
      cliDecisionBeforeAccept
    );

    // The owner (desktop) now explicitly accepts the drafted decision --
    // a separate, deliberate action, never implied by accepting the
    // proposal that merely drafted it.
    const decisionAcceptRes = await invokeCommand(cdp, "decision_accept", {
      vaultPath,
      decisionId: decisionB.id,
      expectedRevisionId: decisionB.revision_id,
    });
    record("proposal-drafted-decision-explicitly-accepted-by-owner", decisionAcceptRes.ok && decisionAcceptRes.result.lifecycle === "accepted", decisionAcceptRes);

    const cliDecisionAfterAccept = cliMayFail(["decision-state", "--vault", vaultPath, "--project", projectId, "--key", "from-cli-receipt"]);
    record(
      "explicit-owner-acceptance-now-visible-to-cli",
      cliDecisionAfterAccept.ok && cliDecisionAfterAccept.out.includes("CurrentSet"),
      cliDecisionAfterAccept
    );

    // === Malformed proposal is refused, cleanly ===
    const malformed1 = await invokeCommand(cdp, "proposal_admit", { vaultPath, projectId, rawText: "not json at all" });
    record("malformed-proposal-invalid-json-refused", malformed1.ok === false, malformed1);

    const malformed2Json = JSON.stringify({ receipt_id: "00000000-0000-7000-8000-000000000000", operations: [] });
    const malformed2 = await invokeCommand(cdp, "proposal_admit", { vaultPath, projectId, rawText: malformed2Json });
    record("malformed-proposal-unknown-receipt-refused", malformed2.ok === false, malformed2);

    // === Proposal accept conflict: stale expected_revision_id refused ===
    const conflictProposalJson = JSON.stringify({
      receipt_id: receiptA,
      declared_agent: "test-agent-c",
      operations: [{ kind: "draft_decision", decision_key: "conflict-probe-decision", statement: "should still work once" }],
    });
    const admitCRes = await invokeCommand(cdp, "proposal_admit", { vaultPath, projectId, rawText: conflictProposalJson });
    record("proposal-c-admit", admitCRes.ok, admitCRes);
    const staleRev = admitCRes.result.revision_id;
    // Reject once (advances revision), then try to accept with the stale pre-reject revision.
    const rejectCRes = await invokeCommand(cdp, "proposal_reject", {
      vaultPath,
      proposalId: admitCRes.result.id,
      expectedRevisionId: staleRev,
      reason: "rejecting to advance revision for the conflict probe",
    });
    record("proposal-c-reject", rejectCRes.ok && rejectCRes.result.status === "rejected", rejectCRes);
    const acceptConflictRes = await invokeCommand(cdp, "proposal_accept", {
      vaultPath,
      proposalId: admitCRes.result.id,
      expectedRevisionId: staleRev, // stale on purpose: superseded by the reject above
      selectedIndices: [0],
    });
    record(
      "proposal-accept-conflict-refused",
      acceptConflictRes.ok === false,
      acceptConflictRes
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

  const overallOk = steps.every((s) => s.ok);
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, overallOk, tmpRoot, vaultPath, steps }, null, 2));
  log(`=== run complete. overallOk=${overallOk}. json=${jsonPath} log=${logPath} ===`);
}

main().catch((e) => {
  log("FATAL: " + String(e?.stack || e));
  fs.writeFileSync(jsonPath, JSON.stringify({ runId, fatal: String(e?.stack || e), steps }, null, 2));
  process.exitCode = 1;
});
