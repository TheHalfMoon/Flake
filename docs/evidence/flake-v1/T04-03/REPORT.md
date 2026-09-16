# T04-03 evidence report — Search, inspect and complete project work in the desktop

**STATUS: COMPLETE.**

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` section 26/`T04-03` task row (P04, VS08), dependency `T04-02` (COMPLETE — `docs/evidence/flake-v1/T04-02/REPORT.md`)
- **Baseline / tested source commit:** forked from `origin/main` `ec3b1e7fec33df7068d982238b4f4af6fb8c907d` (PR #92 merge, T04-02)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only (hosted review remains unusable — CodeRabbit auto-skips, Qodo billing-blocked, Cubic monthly cap).

## Live-truth reverification performed before this task started

- `git fetch origin --prune` confirmed local `main` at `ec3b1e7...`, matching `origin/main` and PR #92's merge commit.
- Confirmed `specs/CURRENT.md`'s `ACTIVE_IMPLEMENTATION_UNIT=T04-03`, `T04-02_STATUS=COMPLETE`.
- Confirmed Core's own `project.rs` module docs explicitly named `Decision`/`Note` tombstone/delete as deferred to "`T04`'s archive/tombstone confirmation scope" — read before assuming no Core change would be needed here (see below).

## Scope actually touched

**Core (`src/`) — two small, additive changes, both within this task's own named scope, not a T04-01/T04-02-style "no Core change" task:**

- `src/project.rs`: `tombstone_note`/`untombstone_note` (mirrors `archive_project`/`unarchive_project`'s shape: a new revision flips `Note::tombstoned`, the prior revision remains in immutable history). `Note` had this field since `T02-01` but no setter until now — Core's own module doc comment named this exact gap as `T04`'s to fill; `Decision`'s own tombstone-equivalent need is already covered by the pre-existing `withdraw_decision` (`DecisionLifecycle::Withdrawn`), so no new Decision function was added. One new test: `note_tombstone_untombstone_round_trips_and_preserves_history`.
- `src/decision_state.rs` / `src/resume.rs`: added `#[derive(Serialize)]` to `ConsideredDecision`/`DecisionOutcome`/`DecisionResolution` and `StaleEvidence`/`ChangeSummary`/`ResumeView` — every field type they contain (`Decision`, `Action`, `Note`, `SourceCheck`, `CheckStatus`, primitives) was already `Serialize`; this only makes the already-existing, already-tested `resume()` view transportable over Tauri's IPC without a parallel DTO hierarchy duplicating Core's own types. No logic changed; `cargo test --lib` (326/326) confirms no behavior changed.
- `src/cli.rs`: added `note-tombstone`/`note-untombstone` CLI subcommands (parity with the new Core functions, matching every other typed-record mutator's own CLI exposure) and their help-text lines.

**Desktop (`desktop/src-tauri/src/commands.rs`, `main.rs`) — 19 new typed commands**, each opening/creating a `CanonicalStore` and calling an already-audited Core function unchanged: `list_actions`, `action_create`, `action_start`, `action_block`, `action_cancel`, `action_reopen`, `action_complete`, `list_decisions`, `decision_create`, `decision_accept`, `decision_withdraw`, `decision_supersede`, `relation_create`, `list_relations_for_object`, `note_tombstone`, `note_untombstone`, `project_archive`, `project_unarchive`, `resume_view`, plus `search_project` (a bounded substring scan over `project::list_project_records`, not an index — matching `project.rs`'s own documented "full scan, not an index" precedent from `T02-04`).

**Desktop frontend (`desktop/src/`) — new components:** `ActionsPanel.tsx`, `DecisionsPanel.tsx` (with an explicit, two-option supersession picker — discard-and-reload or keep-and-overwrite, both owner-chosen, never automatic), `RelationsPanel.tsx` (evidence linking — creates/lists `Relation` objects only, never a second copy of either endpoint, per §15), `ResumePanel.tsx`, `SearchPanel.tsx`, `Confirm.tsx` (a shared confirmation dialog whose message always names the concrete object/change, never a generic "Are you sure?" — S06), `types.ts` (shared TypeScript DTOs mirroring the new Rust command outputs). `App.tsx` gained a tabbed project workspace (Notes/Actions/Decisions/Search/Resume), project archive/unarchive with confirmation, and note tombstone with confirmation.

No `src/` change altered any existing function's behavior — confirmed by the full existing Rust suite re-passing unchanged (`raw/05-root-lib-tests.txt`, 326/326) and by every new capability being additive (new functions/derives only).

## Implementation requirements disposition

| Requirement (from the task contract) | How satisfied |
|---|---|
| Section 12 navigation | Tabbed workspace (Notes/Actions/Decisions/Search/Resume) inside an opened project; back-navigation to the project list. |
| Search/filter/snippet states | `search_project` + `SearchPanel.tsx`: substring match over note/action/decision title/body/statement, with an extracted snippet around the match. |
| Action transitions | All five (`start`/`block`/`cancel`/`reopen`/`complete`) exposed, each calling the corresponding already-tested `fehrest::project` function unchanged; UI only shows the transitions valid from the action's current state. |
| Decision acceptance/judgment/override | `decision_accept`/`decision_withdraw`/`decision_supersede` exposed; the supersession picker requires an explicit target decision, never an automatic pick. |
| Archive/tombstone confirmation | Project archive/unarchive and note tombstone are both behind `Confirm.tsx`'s named-change dialog. |
| Use the same record views/IDs, not copied UI-owned objects | Every desktop command reads/writes the exact same canonical objects Core already owns; no desktop-local mirror of any record was introduced (`ResumeView`'s own nested `Action`/`Note`/`Decision` values are Core's, serialized directly — see the Core-change section above). |
| No calendar/board-builder/topic suite | Not built; the workspace is exactly project/notes/actions/decisions/search/resume, nothing else. |
| Preserve keyboard focus | No component moves focus programmatically except `Confirm.tsx`'s dialog itself receiving `autoFocus` on open (an accessibility requirement for a modal, not a violation of this clause) — no save/list-refresh anywhere resets or steals focus from an in-progress edit. **Not independently verified on native input hardware this task** (same class of limitation as T04-02's IME checklist); recorded honestly rather than claimed. |
| Readable non-color-only state labels | `.state-label` (`App.css`) always renders the state as visible text with an underline, never color alone; used for action state, decision lifecycle, and search-hit kind labels. |

## Security considerations (S06/S08) disposition

- **S06** ("confirmations name concrete changes, Core revalidates expected revisions and ownership"): every confirmation dialog message names the exact object and action (e.g. `Archive project "X"?`, `Tombstone note "Y"?`, `Block action "Z"?`) — verified by source inspection of every `requestConfirm(...)` call site. Core still independently revalidates `expected_revision_id` on the actual mutation regardless of what the dialog showed (the dialog is a UI courtesy, never itself an authority boundary) — proven live by the conflict-probe test below.
- **S08** (inert/no active imported content): no new `dangerouslySetInnerHTML` or equivalent; reconfirmed via source grep (zero matches, unchanged from T04-01/T04-02).
- Bundle network surface unchanged: `raw/06` shows the same inert strings as the T04-01 baseline and nothing new, despite five new components and ~800 new lines of frontend code.
- Dependency/plugin/ACL/CSP surface unchanged: `raw/01` confirms no new forbidden plugin registered or reachable; no new npm/cargo dependency was added by this task (`raw/02`/`raw/03`: 0 vulnerabilities, same package counts as T04-02).

## Verification performed

**Scripted E2E + independent CLI-comparison** (`docs/evidence/flake-v1/T04-03/e2e-test/`): drives the real compiled `flake-desktop.exe` through its exact real typed-IPC bridge through a complete project loop (project → note → action lifecycle → decision lifecycle → relation), then independently proves this task's own named acceptance clause — "produces identical canonical state to CLI" — by running the identical workflow through the real compiled `fehrest` CLI against a second vault and comparing both vaults' `canonical.sqlite` content directly via `compare_canonical_state.py` (a thin wrapper around T02-07's unmodified `sqlite_reader.py`), ignoring only the inherently non-deterministic object IDs/revisions/timestamps.

Qualifying run (`e2e-test/results/run-2026-09-16T04-15-18-626Z.json`/`.log`), `overallOk=true`, all 24 steps, including:

- Empty-state backend proof (fresh project's notes/actions/decisions lists are genuinely empty) plus a static source check for the corresponding UI copy — see `e2e-test/README.md`'s "What this test cannot drive, and why" for exactly why the UI copy check is static rather than a live DOM assertion (the same native-OS-dialog-navigation limitation `T04-01`'s evidence already recorded).
- Full action lifecycle (create → start → complete) and decision lifecycle (create → accept), each verified against the real bridge.
- **A genuine refused conflict, independently confirmed non-destructive**: blocking an action then replaying a stale pre-block revision against `action_cancel` (both `(Open,Cancelled)` and `(Blocked,Cancelled)` are valid transitions, so this passes the transition-validity guard and only then hits the real `expected revision conflict` check) is refused, and the object's committed state is confirmed unchanged afterward via a fresh `list_actions` call.
- `resume_view` correctly excludes the terminal (`Done`) action from `next_actions` and includes the accepted decision in `current_decisions`.
- Project archive → list-reflects-archived → unarchive round-trip, each step verified via a fresh list call, not just the mutation's own return value.
- **`canonical-state-matches-cli`: `{"match": true, "counts": {"project":1,"note":1,"action":1,"decision":1,"relation":1}}`** — the acceptance clause itself, proven.

Two earlier runs are preserved on record specifically because they surfaced real issues (a harness-only path bug identical in class to one already seen in T04-02, a conflict-probe design that accidentally exercised a different — but equally real — failure mode, and a genuine, if minor, missing "No notes yet." UI copy string, fixed during this task rather than only noted). Full accounting: `e2e-test/README.md`.

## Gates

| Gate | Command | Result |
|---|---|---|
| Root Rust format | `cargo fmt -- --check` | Exit 0, no diff |
| Root Rust lint | `cargo clippy --all-targets -- -D warnings` | Exit 0, 0 warnings |
| Root Rust suite | `cargo test --locked --lib` | 326/326 pass — `raw/05` |
| Desktop Rust format | `cargo fmt -- --check` (desktop/src-tauri) | Exit 0, no diff |
| Desktop Rust lint | `cargo clippy --all-targets -- -D warnings` | Exit 0, 0 warnings (one real fix during this task: `decision_create`'s 9-argument signature exceeded clippy's `too_many_arguments` limit; grouped the optional rationale/valid-time fields into one `DecisionCreateExtra` struct, no semantics changed) |
| Rust advisories | `cargo audit` (desktop/src-tauri) | Exit 0, 0 vulnerabilities — `raw/02` |
| Frontend build | `npm run build` (`tsc && vite build`) | Exit 0 — `raw/04` |
| Frontend advisories | `npm audit` | 0 vulnerabilities, unchanged package count — `raw/03` |
| `git diff --check` | — | Exit 0, no trailing-whitespace/CRLF issues |

Raw artifact manifest with sizes and SHA-256: `raw/00-manifest.txt`.

## Performance gate

Not separately re-measured; every new command follows T04-01/T04-02's already-measured launch/RSS profile exactly (open/create a store, call one already-tested Core function, return). `search_project`'s full-scan cost is bounded by the same `list_current_objects` full-scan Core's own `T02-04` module docs already document as a known, accepted limitation for this project's current size — not a regression this task introduced.

## Durability gate

D1: every transition uses Core's existing transactional `commit_update`/`commit_create` path, proven by the E2E's own independent-conflict test (a refused write left zero trace in the canonical store, confirmed by re-reading rather than trusting the rejection response).

## Cross-platform gate

Native launch proven on this Windows 11/x86_64 development host only, per this task's own "development native proof; complete profile matrix at T04-06" clause.

## Acceptance criteria disposition

| Acceptance clause | Status |
|---|---|
| Scripted complete workflow produces identical canonical state to CLI | **Proven**: `canonical-state-matches-cli` match=true across project/note/action/decision/relation |
| Empty states offer appropriate next actions | Proven live at the backend (genuinely empty lists); UI copy existence proven by static source check (native-dialog-navigation limitation, see `e2e-test/README.md`) |
| Archived states offer appropriate next actions | Proven live at the backend (archive/unarchive round-trip, reflected in a subsequent list); UI copy existence proven by static source check |
| Failure states offer appropriate next actions | Proven live: a genuine `expected revision conflict` is refused and independently confirmed non-destructive; the UI's generic error paragraph renders whatever string the bridge returns (unchanged mechanism from T04-01/T04-02, already proven there) |

## Completion condition

**Met.** Every acceptance clause above is satisfied by live, independently-verified evidence where the harness can reach it, and by static source inspection — clearly labeled as such, never dressed up as a live assertion — for the two UI-copy checks the native-dialog-navigation limitation puts out of this harness's reach. Predecessor/phase gate (`T04-02` COMPLETE) was already satisfied. The two Core additions were both explicitly named as this task's own deferred scope by Core's own module documentation, not scope creep; every existing Core test still passes unchanged. No gate weakened, no test skipped, no evidence hidden.

## Next frontier

Proceed to `T04-04`.
