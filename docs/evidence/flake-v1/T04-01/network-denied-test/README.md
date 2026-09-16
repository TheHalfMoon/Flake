# T04-01 network-denied functional test

Proves requirement B of `docs/canonical/FOUNDER_WEBVIEW2_NETWORK_BOUNDARY_2026-09-16.md`: the desktop application, launched with external connectivity genuinely unavailable at the host level, still launches, creates/opens a vault, lists/creates a project, survives a restart with state persisted, and shuts down cleanly.

## Mechanism

`run-offline-test.mjs` is self-contained (Node 22, no npm dependencies -- uses only built-ins: `node:child_process`, `node:net`, global `fetch`, global `WebSocket`). It:

1. **Fails closed on its own precondition.** Before touching the app, it polls (up to 4 minutes, two independent methods: an HTTP fetch of a well-known connectivity-probe endpoint, and a raw TCP connect to `1.1.1.1:443`) until external connectivity is genuinely unreachable by both methods. If connectivity never actually drops within the wait window, the script aborts and writes a FAIL result -- it never reports a network-denied result while still online.
2. Starts a local-only Vite dev server (`127.0.0.1:1420`, loopback, no external request) serving the exact same `App.tsx` source the production bundle is built from.
3. Launches the real compiled `desktop/src-tauri/target/debug/flake-desktop.exe` with `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=<N>`, which opens a Chrome DevTools Protocol (CDP) endpoint on loopback only (`127.0.0.1:<N>`) -- this requires no external network path, only the loopback interface, which functions with the physical adapter disabled.
4. Connects to that CDP endpoint over a local WebSocket and calls `window.__TAURI_INTERNALS__.invoke(command, payload)` via `Runtime.evaluate` -- the exact same low-level IPC transport `@tauri-apps/api`'s `invoke()` uses internally (this crate sets `"withGlobalTauri": false`, so the convenience `window.__TAURI__` namespace is intentionally not exposed; `__TAURI_INTERNALS__` is Tauri's own always-present transport underneath it). This drives the real Rust command handlers in `desktop/src-tauri/src/commands.rs`, which call the real, already-audited `fehrest::{canonical,project}` functions unchanged -- not a mock, not a duplicated code path.
5. Exercises: `vault_create` -> `list_projects` (expect empty) -> `create_project` -> `list_projects` (expect one) -> clean process-tree shutdown (`taskkill /T /F` on the exact spawned root PID, verified gone) -> a second, independent launch on a fresh CDP port simulating a restart -> `vault_open` on the same path (vault_id must match) -> `list_projects` (project must still be there) -> clean shutdown again.
6. Runs a background PowerShell loop for the whole test recording any non-loopback TCP connection observed machine-wide (`Get-NetTCPConnection`, 500 ms polling) to a separate log file, for raw-observation purposes.
7. Writes every step's timestamp, pass/fail, and full detail to a JSON file plus a plain-text log incrementally (append-per-step), so a run's evidence survives even if the controlling session loses connectivity mid-run.

Native OS folder-picker automation was deliberately **not** attempted -- Tauri's WebDriver/CDP surface only reaches the webview's own DOM content, not native Win32 common dialogs, and automating those would need a separate UI-Automation layer this task's scope does not require. Driving the exact same typed-command bridge directly (step 4 above) exercises the identical Rust command handlers and Core calls the UI's buttons would otherwise trigger after the owner picks a folder; the native-dialog path itself was already exercised and verified in the original (online) T04-01 evidence.

## Runs on record

| Run | `network-denied-precondition` | Result |
|---|---|---|
| `run-2026-09-16T02-15-51-816Z` | n/a (pre-fail-closed-gate script version) | Crashed before the app launched (`spawn EINVAL` on Windows npm invocation) -- fixed, not a network result. |
| `run-2026-09-16T02-19-20-044Z` | n/a | Mechanics dry run (online). App launched but the IPC-bridge-ready check raced the page load; fixed with a retry-poll. |
| `run-2026-09-16T02-20-21-998Z` | n/a (online) | Full mechanics dry run, online, all 20 steps green. Confirms the harness itself works before ever asking for a real disconnect. |
| `run-2026-09-16T02-30-56-513Z` | not yet gated | Run attempted after being told Wi-Fi was off; connectivity canary still returned `200 OK` and `Get-NetAdapter` still showed `Up/Connected` -- **not** a genuine network-denied result, correctly not claimed as one. This is why the fail-closed precondition gate (item 1 above) was added before any further attempt. |
| `run-2026-09-16T02-47-39-926Z` | **PASS** -- polled every ~3 s from 02:47:40Z; both HTTP and raw-TCP checks read reachable until 02:48:16Z, then both read unreachable from 02:48:22Z onward | **This is the qualifying run.** `overallOk=true`, all steps pass: `vault_create`, `list_projects` (empty), `create_project`, `list_projects` (one), clean shutdown, second launch, `vault_open` (vault_id matches), `list_projects` (project persisted), clean shutdown. |

Full step-by-step detail for the qualifying run: `results/run-2026-09-16T02-47-39-926Z.json` / `.log`.

## What this test does and does not establish

**Established:** `NETWORK_REQUIRED_FOR_FLAKE_OPERATION=NO`. Every Flake-owned operation exercised (vault create/open, project list/create, restart-and-reopen, persistence, clean shutdown) completed successfully with the host's only physical network adapter (Wi-Fi) confirmed down by two independent methods for the whole exercised window.

**Not re-established by this test, and not required to be:** the WebView2 background-connection-attempt finding itself. That finding (`docs/evidence/flake-v1/T04-01/REPORT.md`, `raw/09-native-launch-network-observation.txt`) was already evidenced while online, across three configurations, and is preserved unchanged. This test's own machine-wide network monitor log (`results/run-2026-09-16T02-47-39-926Z.network-monitor.log`) recorded other, unrelated processes' pre-existing connections transitioning to `FinWait`/`TimeWait` in the seconds after disconnection -- consistent with a genuine loss of connectivity -- but each app instance in this run was only open for roughly 1.5-2.4 seconds (just long enough to complete the IPC exchange), which is not a claim about what WebView2's own background connection would do over a longer window, and the monitor was not scoped to only this app's process tree. This is recorded as a scope limitation of this specific log, not as a contradiction of the original finding or as grounds to weaken it.
