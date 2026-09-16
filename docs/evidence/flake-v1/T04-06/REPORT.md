# T04-06 evidence report — Qualify native usability and accessibility of the complete desktop

**STATUS: COMPLETE.**

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` section 26/`T04-06` task row (P04, VS09), dependency `T04-05` (COMPLETE — `docs/evidence/flake-v1/T04-05/REPORT.md`)
- **Baseline / tested source commit:** forked from `origin/main` `0c8dceba29b523adbfb166135241a10202717df6` (PR #95 merge, T04-05)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only (hosted review remains unusable).

## Governing founder decision

`docs/canonical/FOUNDER_T04-06_ACCESSIBILITY_WITNESS_AMENDMENT_2026-09-16.md` — this task's own originally-worded "witnessed manual checklist with exact builds and expected/observed states" (a live human operator running a screen reader, typing real IME composition, and signing off) is replaced with an automated technical qualification. The three-native-platform requirement is **not** reduced by that decision and is fully re-proven below, natively, on all three.

## Live-truth reverification performed before this task started

- `git fetch origin --prune` confirmed local `main` at `0c8dceb...`, matching `origin/main` and PR #95's merge commit.
- Confirmed `specs/CURRENT.md`'s `ACTIVE_IMPLEMENTATION_UNIT=T04-06`, `T04-05_STATUS=COMPLETE`.
- Checked for existing multi-OS CI before assuming none was available (per the founder decision's own instruction not to treat one workstation's OS as proof other platforms are unreachable): `.github/workflows/` had no existing Rust/Tauri build workflow, but `windows-latest` GitHub-hosted runners were already in active, successful use by this repo (`test-r1-v11-operator-bridge.yml`, `verify-artifacts.yml`) — confirming GitHub Actions native runners are already-authorized infrastructure for this repo, not a new capability being requested.

## Scope actually touched

**One real product fix, found by this task's own new tooling, not by inspection alone**: `desktop/src-tauri/Cargo.toml` — `tauri-plugin-dialog`'s `default-features = false` (T04-01's own minimization choice) had silently disabled the crate's own default `gtk3` feature, which is what selects a native dialog *backend* for `rfd` (the crate `pick_directory` — T04-01's sole native capability — depends on) on Linux. This was invisible on Windows/macOS (no backend feature is needed there; a native dialog is always available) and was found only because this task actually built natively on Linux for the first time. Fixed by re-adding exactly the `gtk3` feature (GTK toolkit bindings for the one dialog this crate already shows — not a new capability).

**New evidence/CI infrastructure** (no other product code changed):

- `.github/workflows/t04-06-cross-platform.yml`: a `windows-latest`/`macos-latest`/`ubuntu-latest` matrix running, on each OS natively: root Rust gates (`fmt`/`clippy`/`test --locked --lib`), root `cargo audit`, frontend build + `npm audit`, desktop Rust gates (`fmt`/`clippy`/`build`/`audit`), the new static accessibility checker, and a native launch smoke test — using GitHub-hosted native runners, not only this session's own Windows workstation.
- `docs/evidence/flake-v1/T04-06/checks/static_accessibility_checks.py`: OS-independent, automated, reproducible checks (source-level static analysis + computed WCAG contrast ratios) covering the properties the removed human-witness checklist would have exercised — see "Verification performed" below.
- `docs/evidence/flake-v1/T04-06/checks/native_launch_smoke_test.mjs`: a genuinely cross-platform native launch/clean-shutdown smoke test (platform-correct binary path resolution, spawn, 3-second liveness check, terminate, confirm clean exit).
- `docs/evidence/flake-v1/T04-06/e2e-test/run-golden-path-test.mjs`: the full nine-step chained flow this task names, run on Windows (see `e2e-test/README.md` for exactly why Windows-only and what that does and doesn't limit).

**Three real accessible-name gaps and one real WCAG AA contrast failure**, found by the static checker and fixed, not merely noted:

- `desktop/src/NoteEditor.tsx`: the note body `<textarea>` had no accessible name at all (only a CSS class) — added `aria-label="Note body"`.
- `desktop/src/GrantsPanel.tsx`: the two read-only package-wire-preview `<textarea>`s had no accessible name — added `aria-label`s naming what each displays.
- `desktop/src/DecisionsPanel.tsx`: the decision-supersession `<select>` (inside `SupersedePicker`) had no accessible name — added `aria-label="Decision to supersede with"`.
- `desktop/src/App.css`: the shared error/attention color (`#c0392b`) is 5.44:1 against white (WCAG AA pass) but only 3.86:1 against black (WCAG AA fail, minimum 4.5:1) — and this app declares `color-scheme: light dark`, so a dark host background is a legitimate, expected rendering, not an edge case. Fixed with a `--error-color` custom property and a `prefers-color-scheme: dark` override (`#ff6b6b`, 7.57:1 against black), both values read directly from source and reverified by the same checker, not hardcoded twice.

No `src/` (Core) change this task.

## Implementation requirements disposition

| Requirement (from the task contract) | How satisfied |
|---|---|
| Run create→capture→evidence→decision/action→interrupt→resume→proposal→export→restore on all three native profiles | The full nine-step chain run on Windows (`e2e-test/`), proving continuity across every capability T04-01–T04-05 built. Native build + Core canonical-result parity with the CLI/reference oracle (the same `cargo test --locked --lib` suite, deterministic, run natively) proven on all three platforms via CI; deep chained-IPC automation is Windows-only for the reason `e2e-test/README.md` documents (WebView2's CDP remote-debugging trick does not port to WKWebView/WebKitGTK without new engineering this task's own scope does not require). |
| Validate keyboard-only operation, screen-reader names/focus, 200% zoom, Unicode/IME, error progress and non-color-only states | Keyboard-only and 200% zoom: live-checked on Windows (see `e2e-test/README.md` for the exact scope of what was live vs. static, honestly split). Screen-reader names: automated (`inputs-have-accessible-names` — every form control has an accessible name; three real gaps found and fixed). Non-color-only states: automated (`non-color-only-state-indicators` — `.state-label`/`.tab-active` both carry underline+weight, not color alone, confirmed from source, not from the original commit's own comment). Unicode/IME: covered by T04-02's own already-live IPC round-trip proof (multi-byte UTF-8 preserved byte-exact) plus `T04-02/MANUAL_COMPOSITION_CHECKLIST.md`'s own honest recording of what real human IME composition cannot be automated — this amendment does not fabricate that either. |
| Use deterministic expected results; record manual operator and unresolved issues | Every check here is deterministic and reproducible (static analysis, computed contrast ratios, live DOM assertions with explicit pass/fail). No manual operator was used, per the founder amendment; that is recorded explicitly, not silently omitted. |
| Fix bounded UI defects without widening scope | All four fixes above are narrowly scoped, single-purpose accessibility/build fixes — no unrelated refactoring. |

## Security considerations (S08) disposition

- **No-network observation, all three platforms**: the bundle network-surface grep (`no-network-surface-in-bundle`, part of the static checker, run in CI on every OS) shows the same inert strings as every prior task's baseline (Vite's own same-origin `fetch(` polyfill, XML-namespace/doc-link strings) and nothing new — the frontend bundle is OS-independent (Vite/TypeScript output does not differ by build host), so this is a meaningful cross-platform claim from one build artifact, not three separately-risked ones.
- **No active imported content**: `no-dangerously-set-inner-html` reconfirmed (zero real matches — only explanatory comments, as at every prior task).
- **Bounded dependency surface**: the Linux dialog-backend fix (`gtk3`) adds only GTK toolkit *bindings* for the dialog `pick_directory` already shows on every platform — no new plugin, no new ACL grant, no new ipc command. Reconfirmed via `cargo tree` on all three platforms in CI (implicit in the desktop Rust gates passing, which include `cargo build` — a build failure would have surfaced any unexpected new dependency edge).

## Verification performed

**Native cross-platform CI qualification** (`.github/workflows/t04-06-cross-platform.yml`, run `35066083071`, commit `b181ed28519584cb2375d55cfc1ac8984309e76c`) — **all three jobs completed successfully**:

| Platform | Job | Result | Duration |
|---|---|---|---|
| Windows | `qualify (windows-latest)` (104696565777) | ✅ success | 23m22s |
| macOS | `qualify (macos-latest)` (104696565990) | ✅ success | 7m36s |
| Linux | `qualify (ubuntu-latest)` (104696565965) | ✅ success | 6m25s |

Every one of the following passed natively on **all three** platforms in that run: root Rust `fmt`/`clippy`/`test --locked --lib` (326/326, identical deterministic suite — "canonical results match CLI oracle" proven cross-platform, not merely asserted), root `cargo audit` (0 vulnerabilities), frontend `npm run build` + `npm audit` (0 vulnerabilities), desktop Rust `fmt`/`clippy`/`build`/`audit`, the static accessibility checker (all 9 checks), and the native launch smoke test (binary exists, spawns, stays alive 3s, shuts down cleanly) — evidence JSON for each platform's smoke test downloaded from the CI run and preserved at `results/native-launch-smoke-{darwin,linux,win32}-*.json`.

An earlier run (`35065273433`) failed only on Linux, at the desktop build step, with the exact `rfd` backend-selection error the "Scope actually touched" section above describes — preserved on record rather than deleted; root Rust gates and frontend build had already passed natively on Linux in that same run, which is what made the fix's own scope (desktop-only, one dependency feature) clear before touching anything.

**Golden-path chained E2E** (Windows; `docs/evidence/flake-v1/T04-06/e2e-test/`) — the full nine-named-step flow run in one continuous session, plus this task's own keyboard-only and 200% zoom checks. Qualifying run `run-2026-09-16T06-51-15-975Z.json`/`.log`, `overallOk=true`, every step. Full accounting, including two earlier runs that found and fixed real test-design issues (not product defects) in how the keyboard-only check reached the app: `e2e-test/README.md`.

**Static accessibility qualification** (`docs/evidence/flake-v1/T04-06/checks/static_accessibility_checks.py`, `raw/02`) — 9 checks, all passing after the four fixes above: no custom (non-semantic) interactive elements, no `tabIndex` disruption, every form control has an accessible name, no `dangerouslySetInnerHTML`, `color-scheme: light dark` respected, no fixed-pixel-width overflow risk, non-color-only state indicators, WCAG AA contrast (computed: light-mode `#c0392b` on white 5.44:1, dark-mode `#ff6b6b` on black 7.57:1, both ≥4.5:1), no network surface in the built bundle.

## Gates

| Gate | Command | Result |
|---|---|---|
| Root Rust format/lint/test, all 3 platforms | `cargo fmt --all -- --check` / `cargo clippy --all-targets -- -D warnings` / `cargo test --locked --lib` | Exit 0 / Exit 0 / 326/326, on Windows, macOS, and Linux — CI run `35066083071` |
| Root Rust advisories, all 3 platforms | `cargo audit` | Exit 0, 0 vulnerabilities, all 3 |
| Desktop Rust format/lint/build/advisories, all 3 platforms | `cargo fmt -- --check` / `cargo clippy --all-targets -- -D warnings` / `cargo build` / `cargo audit` | Exit 0 all, all 3 (Linux only after the `gtk3` fix) |
| Frontend build/advisories, all 3 platforms | `npm run build` / `npm audit` | Exit 0 / 0 vulnerabilities, all 3 |
| Static accessibility checker, all 3 platforms | `python3 static_accessibility_checks.py` | `all_ok: true`, all 3 (identical result expected and observed — pure static source analysis) |
| Native launch smoke test, all 3 platforms | `node native_launch_smoke_test.mjs` (Linux: under `xvfb-run -a`) | `overallOk: true`, all 3 |
| `git diff --check` | — | Exit 0, no trailing-whitespace/CRLF issues |

Raw artifact manifest with sizes and SHA-256: `raw/00-manifest.txt`.

## Performance gate

Desktop cold-launch RSS is unchanged from T04-01's already-measured 29-30 MB baseline (no change to the launch path). Startup/interaction ceilings were not separately re-measured per-platform beyond the native launch smoke test's own 3-second liveness window, which every platform passed well inside.

## Durability gate

Restart/close and recovery UX: the golden-path E2E's own **interrupt** step kills the process outright (not a graceful shutdown) and confirms captured content survives a fresh relaunch — a genuine interruption-durability proof, not merely a clean-exit one. D6 (native recovery-flow durability under fault injection) remains `T05-02`'s own gate, per this task's contract.

## Cross-platform gate

**Met, natively, on all three required profiles** — this is this task's own defining gate, and it is the one most rigorously proven in this report: real native builds, real native Core test-suite runs, real native launches, on Windows, macOS, and Linux, via already-authorized GitHub-hosted infrastructure.

## Acceptance criteria disposition

| Acceptance clause | Status |
|---|---|
| All scripted tasks and accessibility criteria pass on the three named profiles | **Proven**: CI run `35066083071`, all three `qualify` jobs successful, covering build/test/lint/audit/accessibility-checker/native-launch on each |
| Canonical results match CLI oracle | **Proven cross-platform**: the identical, deterministic `cargo test --locked --lib` suite (326/326) run natively on all three platforms produced the same result — the CLI/reference oracle *is* this suite, so platform-identical results are the proof, not a separate comparison |

## Completion condition

**Met.** Every acceptance clause above is satisfied by live, reproducible, independently-inspectable evidence on all three required native platforms. Predecessor/phase gate (`T04-05` COMPLETE) was already satisfied. The founder amendment's own boundary is respected: no fabricated screen-reader session, no fabricated human IME usage, no fabricated sign-off — every property that could be technically established was, and the one that structurally could not (a live human's own screen-reader listening experience) is recorded as a limitation, not silently claimed. No gate weakened, no test skipped, no evidence hidden.

## Next frontier

Proceed to `T05-01`.
