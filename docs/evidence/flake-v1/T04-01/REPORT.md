# T04-01 evidence report — Create a thin local desktop shell

**STATUS: COMPLETE.** Updated 2026-09-16 after the founder ruling in `docs/canonical/FOUNDER_WEBVIEW2_NETWORK_BOUNDARY_2026-09-16.md`. Everything below the "Founder decision applied" section is additive evidence recorded after that ruling; nothing above it (the original `BLOCKED_PENDING_ARCHITECTURE_DECISION` finding) has been deleted, rewritten, or softened -- the WebView2 background-connection observation remains exactly as originally recorded.

---

**Original status at first submission (preserved below, unchanged): BLOCKED_PENDING_ARCHITECTURE_DECISION.** This was not a completion report at that time. The desktop shell was built, secured at the application level, and functionally exercised the project loop, but one acceptance clause ("starts offline with no external requests") was not fully satisfied for a documented, externally-verified reason.

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` section 26/`T04-01` task row (P04, VS08), dependency `T03-08` (COMPLETE, PASS — `docs/evidence/flake-v1/T03-08/REPORT.md`)
- **Baseline / tested source commit:** forked from `origin/main` `c5f1f1ad3ca5a90129660ede814291b9e7e909b2` (PR #90, T03-08 merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only.

## Live-truth reverification performed before this task started

- `git fetch origin --prune` / `git pull --ff-only` confirmed local `main` at `c5f1f1ad3ca5a90129660ede814291b9e7e909b2`, matching `origin/main` and PR #90's `mergeCommit.oid`.
- `gh api .../commits/c5f1f1a.../check-runs` (performed as part of T03-08's own post-merge verification) confirmed all 8 required checks green on this exact SHA.
- Confirmed `specs/CURRENT.md`'s `ACTIVE_IMPLEMENTATION_UNIT=T04-01`, `T03-08_STATUS=COMPLETE`.
- Confirmed no prior `desktop/` directory or T04-01 evidence existed.

## Scope actually touched

- New `desktop/` package: `desktop/src-tauri/` (Rust Tauri 2 backend crate, its own independent `Cargo.toml`/`Cargo.lock`, not part of the root crate's workspace), `desktop/src/` (React/TypeScript frontend), `desktop/package.json`/`package-lock.json`, build/tooling config (`vite.config.ts`, `tsconfig.json`, `index.html`).
- No `src/` (Core) changes. The desktop crate depends on the root `fehrest` crate via a path dependency and calls only already-audited, already-tested public `fehrest::{canonical,project,import}` functions.

## Dependency admission review (S09)

Every new dependency was checked for necessity, exact-version pin, license, and known advisories before admission. None was added "for convenience" — each is either Tauri itself, its build-time codegen crate, the one plugin the task contract explicitly authorizes (native dialog-mediated selection), or a standard React/Vite/TypeScript toolchain package.

| Package | Exact version | License | Why needed |
|---|---|---|---|
| `tauri` (crate) | `=2.11.5` | Apache-2.0 OR MIT | The desktop runtime itself |
| `tauri-build` (crate) | `=2.6.3` | Apache-2.0 OR MIT | Build-time codegen the task contract requires |
| `tauri-plugin-dialog` (crate) | `=2.7.3` | Apache-2.0 OR MIT | The task's own required "native dialog-mediated selection as a bounded Core capability" -- nothing else provides this |
| `@tauri-apps/api` (npm) | `2.11.1` | Apache-2.0 OR MIT | Frontend `invoke()` bridge -- the only way the UI calls typed commands |
| `@tauri-apps/cli` (npm, dev) | `2.11.4` | MIT | Local dev/build tooling only, not shipped in the bundle |
| `react` / `react-dom` (npm) | `19.3.0` | MIT | The task contract names React explicitly |
| `vite` (npm, dev) | `8.3.0` | MIT | Standard Tauri+React bundler |
| `@vitejs/plugin-react` (npm, dev) | `6.1.1` | MIT | Vite's own React JSX transform plugin |
| `typescript` (npm, dev) | `7.0.2` | Apache-2.0 | TypeScript per the task's own "React/TypeScript assets" |
| `esbuild` (npm, dev) | `0.28.2` | MIT | Required transitively by this Vite version for CSS/asset transforms (build fails without it; not previously bundled) |
| `@types/react`, `@types/react-dom` (npm, dev) | `19.3.0` | MIT | Type declarations only, no runtime code |

**Explicitly not admitted anywhere in this crate's dependency tree** (verified: `raw/01-cargo-tree.txt`, `grep`-checked for every forbidden prefix): `tauri-plugin-shell`, `tauri-plugin-http`, `tauri-plugin-process`, `tauri-plugin-opener`, `tauri-plugin-sql`, `tauri-plugin-fs` (this last one *is* present as a **transitive, unregistered** dependency of `tauri-plugin-dialog` itself -- see "Security boundary" below; it is never registered with `Builder::plugin()` and never granted any `fs:*` permission in `capabilities/default.json`, so its commands are unreachable from the webview).

**Advisory scan**: `cargo audit` (`raw/02-cargo-audit.txt`) against the full 444-crate lockfile: **0 vulnerabilities**. 7 informational "unmaintained crate" / 1 "unsound" warnings on transitive dependencies several layers deep in Tauri's own tree (`proc-macro-error`, four `unic-*` Unicode-identifier crates pulled in via HTML parsing, `glib` -- Linux GTK bindings, inert on this Windows target); none reachable from any code this task wrote, none a known exploitable vulnerability (cargo-audit's own PASS/FAIL distinction: exit code 0, no `error:` lines). `npm audit` (`raw/... captured inline during npm install`): **0 vulnerabilities** across 30 packages.

## Architecture

**No new authority model.** `desktop/src-tauri/src/commands.rs` is the entire typed bridge: five `#[tauri::command]` functions plus one native-dialog wrapper (`pick_directory` in `main.rs`). Every command opens/creates a `fehrest::canonical::CanonicalStore` at an owner-chosen path and calls an already-audited `fehrest::project`/`fehrest::import` function unchanged -- there is no new mutation path, no new validation logic beyond simple name-length/path-segment bounds (`bounded_name`/`join_child`), and no bypass of `fehrest`'s own `check_len`/writer-ownership/expected-revision machinery.

**Security boundary, verified four independent ways:**
1. *Dependency graph* (`raw/01-cargo-tree.txt`): no filesystem/shell/http/process/opener/SQL plugin crate is anywhere in the tree except `tauri-plugin-fs` as `tauri-plugin-dialog`'s own unregistered transitive dependency (confirmed via `cargo tree -i tauri-plugin-fs`, single edge from `tauri-plugin-dialog`).
2. *Runtime registration* (`main.rs`): only `tauri_plugin_dialog::init()` is passed to `Builder::plugin()`. `tauri-plugin-fs`'s own `init()` is never called, so its commands never enter Tauri's IPC command registry regardless of what's compiled into the binary.
3. *ACL capability grant* (`desktop/src-tauri/capabilities/default.json`, `raw/07`): the main window's only permissions are `core:default` and `dialog:default` -- grepped for every forbidden prefix (`fs:`, `shell:`, `http:`, `process:`, `opener:`, `sql:`), zero matches. Tauri 2's security model requires *both* plugin registration *and* an explicit capability grant before any command is invocable from the webview; this crate satisfies neither for filesystem/shell/http/process access.
4. *Content Security Policy* (`tauri.conf.json`, `raw/08`): `default-src 'self'; script-src 'self'; connect-src 'self' ipc: http://ipc.localhost; object-src 'none'; frame-src 'none'; base-uri 'none'; form-action 'none'`. No remote origin is ever permitted for script, connect, or frame; `object-src 'none'` and no `dangerouslySetInnerHTML`-equivalent anywhere in `App.tsx` means imported/remote content can never become active/executable content (S01/S08).

**Path arguments are owner-mediated only.** Every path string a command receives originates from `pick_directory` (a native OS folder picker the owner explicitly drives) or the owner's own typed vault/project name (bounded to 200 bytes, rejected if it contains a path separator or is `.`/`..`) -- never from imported or remote content.

## Failed attempts / exclusions

**Disk exhaustion mid-build.** The development host's `C:` drive reached 0 bytes free partway through the first `cargo build` (Tauri's dependency tree is large: ~440 crates). Deleting this task's own partial/corrupted `target/` directory recovered ~2.6 GB; a `[profile.dev] debug = 0, incremental = false` override in `desktop/src-tauri/Cargo.toml` then kept the full build (`-j 2`, to bound peak concurrent temp-file usage) inside the remaining ~1.3 GB. This is a documented local-host constraint affecting only local dev-profile artifact size, not the shipped product.

**Native WebView2 background network traffic -- the actual blocking finding.** See "What was discovered but not resolved" below; this is reported as a genuine limitation, not silently worked around.

## Commands executed

| Command | Raw artifact | Result |
|---|---|---|
| `npm install` (in `desktop/`) | (inline; `raw/05` captures the subsequent build) | 30 packages, 0 vulnerabilities |
| `npm run build` (`tsc && vite build`) | `raw/05-frontend-build.txt` | Exit 0; `dist/assets/index-*.js` 226 KB (71.5 KB gzip) |
| `cargo build -j 2` (in `desktop/src-tauri/`) | (build log, not separately archived; see "Failed attempts") | Exit 0 after the disk-space/profile fix, ~9.5 min cold |
| `cargo fmt -- --check` | `raw/03-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --all-targets -- -D warnings` | `raw/04-clippy.txt` | Exit 0, 0 warnings (one real defect found and fixed during this task: `needless_as_bytes` on `commands.rs`'s own bound-check) |
| `cargo audit` | `raw/02-cargo-audit.txt` | Exit 0, 0 vulnerabilities, 7 informational unmaintained/unsound warnings on transitive deps |
| `cargo tree` | `raw/01-cargo-tree.txt` | 444 crates; no forbidden plugin crate registered/reachable |
| `grep` for forbidden ACL prefixes in `capabilities/default.json` | `raw/07-capabilities-default.json` | Zero matches |
| `grep` for `fetch(`/`XMLHttpRequest`/`WebSocket(`/literal URLs in the built JS bundle | `raw/06-bundle-network-surface-grep.txt` | Only Vite's own same-origin modulepreload polyfill and inert XML-namespace/doc-link strings |
| Native cold launch + `Get-CimInstance`/`Get-NetTCPConnection` process-tree network observation, 3 times (baseline, +`--disable-background-networking` etc., +broader flag set) | `raw/09-native-launch-network-observation.txt` | App itself: 0 external requests. WebView2 runtime process: 2 persistent Established HTTPS connections to a Microsoft-owned endpoint, present in all 3 configurations |
| Cargo full Rust suite (unaffected, no `src/` change) | not re-run this task (no `src/` diff) | N/A -- `git diff --stat origin/main -- src/` is empty for this task |

Raw artifact manifest with sizes and SHA-256: `raw/00-manifest.txt`.

## What was discovered but not resolved: WebView2's own background network traffic

**Finding.** On a cold launch of the built `flake-desktop.exe`, its own spawned `msedgewebview2.exe` host process (confirmed by process-tree parentage and by its command line naming `--webview-exe-name=flake-desktop.exe`) opens two persistent `Established` HTTPS (port 443) connections to a Microsoft-owned IPv6 address (`2603:1046:c0b:*::2`, Microsoft's own announced range) within seconds of launch -- before any user interaction, and regardless of what the loaded page does.

**Mitigation attempted.** Two rounds of `tauri.conf.json`'s `app.windows[0].additionalBrowserArgs` (a real, verified-present Tauri/wry config field for passing Chromium/Edge command-line switches): first the standard `--disable-background-networking --disable-component-update --disable-breakpad` (plus wry's own default `--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection`, which setting this field requires re-specifying), then a broader set adding `OptimizationHints,MediaRouter,EdgeFeedback,msEdgeTelemetry,PrivateNetworkAccessSendPreflights` to `--disable-features` plus `--disable-domain-reliability --disable-client-side-phishing-detection`. Each change was rebuilt and relaunched with a freshly cleared WebView2 profile directory (`%LOCALAPPDATA%\app.flake.desktop.phase-t`) to rule out cached state. **The same two connections persisted in all three configurations.**

**Independent corroboration.** A web search plus direct read of `github.com/MicrosoftEdge/WebView2Feedback#5224` ("[Feature]: Disable outgoing traffic from WebView2") confirms this is a currently open, unresolved upstream feature request against the WebView2 runtime itself -- i.e., as of this writing there is no documented application-level command-line switch or config that fully eliminates WebView2's own background network activity; the standard Chromium background-networking flags this task applied are necessary but not sufficient, because this behavior appears to originate in Microsoft's own added telemetry/service layer on top of the open-source Chromium code those flags govern, not in Chromium itself.

**What this is not.** It is not a defect in this crate's own code: `raw/06` confirms Flake's own JS bundle issues zero requests to any remote origin (the sole `fetch(` call is Vite's own same-origin modulepreload polyfill, itself further bounded by this app's `connect-src 'self'` CSP), and the CSP/ACL/plugin-registration boundary (see "Architecture" above) is independently sound regardless of this finding.

**Why this blocks a clean PASS on this task's own acceptance criterion.** The task contract's exact wording is "starts offline with no external requests" -- a plain reading that does not carve out an exception for the WebView2 host process's own platform-level telemetry. Silently reinterpreting "no external requests" to mean "no requests from this crate's own code" would be exactly the kind of narrative reinterpretation the founder's evidence discipline forbids ("never... lower thresholds just to pass," "preserve negative evidence"). This is reported instead as a genuine, externally-verified platform limitation of the admitted Tauri 2 + WebView2 stack on Windows, requiring an explicit founder/architecture decision (Class C/D under `AGENTS.md`) on one of:

1. Accept "no external requests from Flake's own code" as the operative reading of this acceptance clause for a WebView2-hosted shell, formally amending/annotating the task contract to say so; or
2. Require machine-level WebView2 policy configuration (`HKLM\SOFTWARE\Policies\Microsoft\Edge\WebView2\*`, e.g. `MetricsReportingEnabled=0`) as an explicit, documented **deployment/packaging** requirement for a qualified Flake desktop build (outside any single application's own runtime control, and outside this task's own scope to set unilaterally on a shared development host); or
3. Reconsider whether WebView2 is admissible at all for a strict-offline product, which would be a far larger P04 architecture reconsideration than this task's own scope.

This executor does not have standing to make that call unilaterally -- it is exactly the kind of product-thesis/security-posture tradeoff `AGENTS.md`'s Class C/D process exists for.

## What has explicitly NOT been claimed

(Original list, at first submission, before the founder decision -- preserved unchanged:)

- **Not claiming T04-01 is COMPLETE.** `specs/CURRENT.md`'s `T04-01_STATUS=BLOCKED_PENDING_ARCHITECTURE_DECISION`, not `COMPLETE`.
- **Not claiming this crate's own code is responsible for, or capable of independently eliminating, the WebView2 platform traffic.**
- **Not claiming this finding invalidates the CSP/ACL/dependency-admission work already done** -- that work is independently verified correct (see "Architecture").
- **Not fabricating a passing "native network observation" acceptance result** to close the task -- the raw observation (`raw/09`) is preserved exactly as captured, including the two persistent connections, across all three configurations tried.

**Addendum, after the founder decision (2026-09-16):** T04-01 is now claimed COMPLETE (see "Founder decision applied" and "Completion condition (updated)" below), but the other three bullets above still hold exactly as written: this crate's own code is still not claimed responsible for or capable of eliminating WebView2's platform traffic; the finding still does not invalidate the independently-verified CSP/ACL/dependency-admission work; and no "zero process-tree network attempts" result is or was ever fabricated -- `raw/09`'s original observation stands unedited, and the network-denied test's own machine-wide monitor log (`network-denied-test/results/*.network-monitor.log`) is preserved as captured, not curated to remove inconvenient entries.

## Performance gate

Cold-launch RSS: 29-30 MB (`tasklist`, `raw/09`), well inside any reasonable desktop-shell ceiling. Not further gated pending the architecture decision above (no point tightening a performance budget on a build whose network-offline status is itself unresolved).

## Durability gate

No new persistence engine. Every command opens/creates a `CanonicalStore` via already-proven (`T01-03`/`T01-07`) paths; no desktop-owned mutable state exists outside what Core already durably commits.

## Cross-platform gate

Native launch proven on this Windows 11/x86_64 development host only, per this task's own "native desktop launch on development host now; all profiles required at T04-06" clause. Not evaluated on macOS/Linux this task.

## Acceptance criteria disposition

| Acceptance clause | Status |
|---|---|
| Built bundle exposes only named Core commands | Satisfied -- verified four independent ways (dependency graph, plugin registration, ACL grant, CSP) |
| Starts offline with no external requests | **Not satisfied as literally worded** -- app-level: satisfied (verified); WebView2-runtime-level: 2 persistent connections observed across 3 mitigation attempts, externally corroborated as a currently-unresolved platform limitation. Blocking finding; founder/architecture decision requested (see above) |
| Cannot render active imported content or execute arbitrary paths | Satisfied -- strict CSP (`object-src 'none'`, no remote `script-src`), no `dangerouslySetInnerHTML`-equivalent, every path argument owner-mediated only |

## Completion condition (original, at first submission)

**Not met at that time.** Per this task's own "Completion condition: Every acceptance clause above plus SC and predecessor/phase gates passes... Otherwise remain at this task," one acceptance clause was not satisfied. This was recorded as `T04-01_STATUS=BLOCKED_PENDING_ARCHITECTURE_DECISION` in `specs/CURRENT.md`, with the exact blocker packet there. The scaffold, dependency admission, and security-boundary work in this report was preserved as real, independently-verified progress -- not discarded -- so a founder decision could unblock continuation without re-deriving it.

---

## Founder decision applied (2026-09-16)

The founder reviewed this finding and ruled `FOUNDER_WEBVIEW2_DECISION=OPTION_1_AMENDED` -- full ruling, exact new acceptance-boundary wording, preserved negative evidence, and non-precedent statement recorded in `docs/canonical/FOUNDER_WEBVIEW2_NETWORK_BOUNDARY_2026-09-16.md`. Summary: the "starts offline with no external requests" clause is read, narrowly and only for a WebView2-hosted (or any shared-OS-webview-hosted) surface, as two separately provable requirements rather than one impossible literal one:

```text
FLAKE_APPLICATION_NETWORK=NONE
WEBVIEW2_PLATFORM_BACKGROUND_TRAFFIC=OBSERVED
NETWORK_REQUIRED_FOR_FLAKE_OPERATION=NO
```

This did not by itself close the task. Both requirements below were separately reverified/proven before this report's status was changed to `COMPLETE`.

### Requirement A -- application-owned network surface reverified

Independently re-run on a fresh checkout of this same branch (not assumed from the original report): `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo audit`, `npm install`/`npm run build`, `npm audit`, static bundle grep for `fetch(`/`XMLHttpRequest`/`WebSocket(`/literal URLs, `grep` for `dangerouslySetInnerHTML`, `cargo tree -i tauri-plugin-fs`, plugin-registration and ACL/CSP file inspection. All results matched the original report exactly (0 vulnerabilities both `cargo audit` and `npm audit`; identical bundle size 226.30 KB / 71.53 KB gzip; identical dependency graph; only `tauri_plugin_dialog::init()` registered; ACL grant still exactly `core:default`+`dialog:default`; CSP unchanged; no `dangerouslySetInnerHTML`). Full transcript: `raw/10-founder-decision-reverification.txt`.

### Requirement B -- network-denied functionality, proven live

Full procedure, harness source, and every run on record (including two runs correctly *not* claimed as passing because connectivity was not genuinely down yet): `docs/evidence/flake-v1/T04-01/network-denied-test/README.md`.

The qualifying run (`network-denied-test/results/run-2026-09-16T02-47-39-926Z.json`/`.log`) used a self-contained Node harness that (1) fails closed by polling two independent connectivity checks (an HTTP fetch and a raw TCP connect) until both genuinely read unreachable -- never trusting an operator's claim -- then (2) launches the real compiled `flake-desktop.exe` with WebView2 remote debugging on loopback only, (3) drives the exact real `window.__TAURI_INTERNALS__.invoke` IPC transport (the same transport `@tauri-apps/api`'s `invoke()` uses internally) to call the real Rust command handlers and real, already-audited `fehrest` Core functions, and (4) exercises vault create, project list (empty), project create, project list (one), clean process-tree shutdown, a second independent launch simulating a restart, vault re-open (vault_id verified matching), project list (persisted), and clean shutdown again.

Both independent connectivity checks read unreachable from 2026-09-16T02:48:22Z onward (confirmed against `Get-NetAdapter` showing the host's only physical adapter, Wi-Fi, disabled). Every one of the 15 functional steps after the precondition gate passed. Two earlier attempts in the same session are preserved on record specifically because they were *not* yet genuinely offline (a canary and `Get-NetAdapter` both still showed live connectivity) and were correctly not reported as a network-denied result -- see the run table in the linked README for the full accounting, including the harness bugs (a Windows `spawn EINVAL` on `npm.cmd`, and an IPC-bridge-readiness race) found and fixed along the way.

`FLAKE_NETWORK_DEPENDENCY=NONE` and `OFFLINE_FUNCTIONAL_TEST=PASS` are both now proven, not merely asserted.

## Completion condition (updated)

**Met.** All three acceptance clauses now pass under the founder-amended reading of the offline clause:

| Acceptance clause | Status |
|---|---|
| Built bundle exposes only named Core commands | Satisfied (unchanged; reverified in `raw/10`) |
| Starts offline with no external requests | Satisfied under `docs/canonical/FOUNDER_WEBVIEW2_NETWORK_BOUNDARY_2026-09-16.md`'s amended reading: `FLAKE_APPLICATION_NETWORK=NONE` (reverified, `raw/10`) and `NETWORK_REQUIRED_FOR_FLAKE_OPERATION=NO` (proven live, `network-denied-test/`). `WEBVIEW2_PLATFORM_BACKGROUND_TRAFFIC=OBSERVED` remains true and is preserved as a documented platform limitation, not a Flake product failure |
| Cannot render active imported content or execute arbitrary paths | Satisfied (unchanged; reverified in `raw/10`) |

Predecessor/phase gates (`T03-08` COMPLETE/PASS) were already satisfied before this task began. Performance (29-30 MB RSS), durability (no new persistence engine), and cross-platform (Windows-only, per this task's own "all profiles required at T04-06" clause) gates are unchanged from the original submission.

## Next frontier

T04-01 is COMPLETE. Proceed to `T04-02`.
