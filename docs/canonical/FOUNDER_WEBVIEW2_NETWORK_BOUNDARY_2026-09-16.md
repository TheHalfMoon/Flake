# Founder decision — WebView2 platform network boundary

Date: 2026-09-16
Decision class: D — security/foundational invariant (founder-authorized amendment to a Class D acceptance boundary)
Status: ADOPTED
Applies to: T04-01 and every later Flake v1 task that ships a WebView2-hosted (Tauri) desktop surface

## Authority

This document is the additive, Class C/D architecture-reconsideration record `docs/evidence/flake-v1/T04-01/REPORT.md`'s "What was discovered but not resolved" section named as required before this finding could be closed one way or another. It does not rewrite that report, `specs/CURRENT.md`'s prior blocker packet, or any raw evidence file. It resolves option 1 of the three the evidence report offered the founder.

## Original T04-01 wording

The canonical build plan's T04-01 acceptance clause read, and still reads unless a later explicit decision changes the plan text itself:

> starts offline with no external requests

Read with no exception, this wording is not literally satisfiable by any product that embeds a shared OS-vendor browser runtime (WebView2 on Windows, WebKitGTK on Linux, WKWebView on macOS), because the founder does not control, and this task's contract does not ask this executor to control, that runtime's own vendor-side telemetry/service layer.

## Observed WebView2 behavior (preserved, not restated as resolved)

Exactly as recorded in `docs/evidence/flake-v1/T04-01/REPORT.md`'s "What was discovered but not resolved" section and its raw artifact `raw/09-native-launch-network-observation.txt`:

- On cold launch of the real built `flake-desktop.exe`, its spawned `msedgewebview2.exe` host process opened two persistent `Established` HTTPS (port 443) connections to a Microsoft-owned endpoint, confirmed by process-tree parentage.
- This occurred before any user interaction and regardless of what the loaded page does.
- It was observed in all three tested configurations (baseline; standard Chromium background-networking-disable flags; a broader flag set).
- Flake's own built JS bundle was independently confirmed (by static grep of the production bundle) to issue zero requests to any remote origin.

This document does not delete, rewrite, or soften that observation. `msedgewebview2.exe background HTTPS connections were observed` remains true and remains recorded in the original report and raw artifact unchanged.

## Failed mitigations (preserved)

Two rounds of `tauri.conf.json`'s `additionalBrowserArgs` (documented, real Tauri/wry config surface for Chromium/Edge command-line switches) were tried against a freshly cleared WebView2 profile directory each time:

1. `--disable-background-networking --disable-component-update --disable-breakpad` plus wry's own default `--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection`.
2. A broader set additionally disabling `OptimizationHints,MediaRouter,EdgeFeedback,msEdgeTelemetry,PrivateNetworkAccessSendPreflights` via `--disable-features`, plus `--disable-domain-reliability --disable-client-side-phishing-detection`.

Both connections persisted unchanged across every configuration tried.

## Upstream limitation

Independently corroborated: [`MicrosoftEdge/WebView2Feedback#5224`](https://github.com/MicrosoftEdge/WebView2Feedback/issues/5224), an open, unresolved upstream feature request asking Microsoft for exactly the capability of disabling WebView2's own outgoing background traffic. As of this decision, no documented application-level command-line switch or config eliminates it. The behavior originates in Microsoft's own added telemetry/service layer on top of the open-source Chromium code the standard background-networking flags govern, not in Chromium itself — which is why Chromium's own flags are necessary but not sufficient here.

## Founder ruling

```text
FOUNDER_WEBVIEW2_DECISION=OPTION_1_AMENDED
```

Tauri/WebView2 is **not** replaced. Machine-level WebView2 registry/group-policy configuration is **not** made a required Flake installation prerequisite. T04-01 is **not** left permanently blocked merely because the shared Microsoft WebView2 runtime attempts its own background service/telemetry connections that Flake neither requests nor requires.

The T04-01 acceptance clause's operative reading is prospectively clarified, narrowly, as follows. This clarification applies to this clause only, for a WebView2-hosted (or any shared-OS-webview-hosted) desktop surface; it does not touch any other acceptance clause, any other task, or the plan's general offline/no-telemetry posture.

### New exact acceptance boundary

```text
Flake-controlled application code, bundled frontend code, Rust bridge,
plugins, IPC surface, and Flake-owned runtime logic MUST make zero
external network requests and MUST require zero network connectivity
for normal supported operation.

Flake MUST remain fully functional when the host has no network access.

Network activity initiated autonomously by an operating-system/shared
WebView runtime that:

1. is not requested by Flake code;
2. is not required by Flake;
3. carries no Flake project/user content;
4. grants no authority to Flake;
5. cannot be disabled using an application-supported mechanism;
6. does not prevent operation when blocked;

is a documented platform limitation, not a failure of the Flake
application-level offline contract.
```

Distinguished terms, going forward, in evidence reports and `specs/CURRENT.md`:

```text
FLAKE_APPLICATION_NETWORK=NONE
WEBVIEW2_PLATFORM_BACKGROUND_TRAFFIC=OBSERVED
NETWORK_REQUIRED_FOR_FLAKE_OPERATION=NO
```

No report may claim `ZERO_PROCESS_TREE_NETWORK_ATTEMPTS` on Windows WebView2. The correct claim is the three-line distinction above.

## What remains forbidden

This is a narrow clarification, not a blanket network exception. It does **not** authorize, for Flake-controlled code, unless a later explicit canonical decision separately authorizes one of these:

```text
fetch()
XMLHttpRequest
WebSocket
remote fonts
remote scripts
remote images
remote frames
analytics
telemetry implemented by Flake
update checks implemented by Flake
remote APIs
network plugins
localhost servers
background network services
provider SDKs
```

Flake remains a local-first/offline product. The desktop frontend and Rust/Tauri application surface must remain network-free. This decision authorizes reading one already-existing acceptance clause narrowly around one already-observed, externally-attributed platform behavior — nothing more.

## Required offline proof

This decision does not by itself close T04-01. It replaces the impossible literal runtime-wide interpretation of "starts offline with no external requests" with two separately proven requirements, both of which must be satisfied and evidenced before T04-01's disposition changes:

**A. Application-owned network surface.** `FLAKE_CONTROLLED_EXTERNAL_REQUESTS=0`, inspected across bundled JavaScript, frontend source, Rust/Tauri code, registered plugins, capability ACLs, CSP, the dependency graph, and any runtime request attributable to Flake-owned code.

**B. Network-denied functionality.** The desktop application, launched with external connectivity blocked or unavailable at the host level, must still: launch; create a local vault; open a vault; restore/import; list projects; create a project; make normal local Core bridge calls; shut down and restart cleanly. If WebView2 attempts background connections and those attempts fail because the host is offline, Flake must continue working correctly regardless. This is the load-bearing offline-product requirement this decision exists to make provable rather than definitionally impossible.

The exact test procedure and its result are recorded in `docs/evidence/flake-v1/T04-01/REPORT.md`'s network-denied test section and `docs/evidence/flake-v1/T04-01/network-denied-test/`.

## Scope of the decision

Applies to T04-01 and any later Flake v1 task (T04-02 through T04-06 and beyond) that ships or modifies a WebView2-hosted, or any other shared-OS-webview-hosted, desktop surface. It does not apply to Flake Core (`src/`), the CLI, or any non-webview surface, none of which embed a shared browser runtime and none of which are implicated by this finding.

## Non-precedent statement

This decision resolves one specific, externally-corroborated, mitigation-exhausted platform behavior (WebView2's own background telemetry connections) under one specific acceptance clause. It is not a general license to reinterpret other acceptance criteria, other security invariants, or other "offline"/"no external requests" language elsewhere in the canonical plan merely because a future finding is inconvenient. Each such case still requires its own evidence, its own exhausted-mitigation record, its own independent corroboration where available, and its own explicit founder/architecture ruling under `AGENTS.md`'s Class C/D/E process. The general principle this ruling establishes for that future process is recorded in `specs/CURRENT.md` and `AGENTS.md`, not invented ad hoc by any executor:

```text
A platform/runtime behavior outside Flake's control is not automatically
a Flake product failure when Flake neither requests nor depends on it and
the required product property remains directly testable.
```

This principle does not excuse security defects in Flake-controlled code, and does not by itself pass any task -- the task must still separately prove requirement B above.
