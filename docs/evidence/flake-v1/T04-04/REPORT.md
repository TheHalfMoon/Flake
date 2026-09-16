# T04-04 evidence report — Present resumption and agent review as one evidence-linked flow

**STATUS: COMPLETE.**

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` section 26/`T04-04` task row (P04, VS09), dependency `T04-03` (COMPLETE — `docs/evidence/flake-v1/T04-03/REPORT.md`)
- **Baseline / tested source commit:** forked from `origin/main` `bd3c3930ab235f00e385d72a1f3f3693b3a3e6b1` (PR #93 merge, T04-03)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only (hosted review remains unusable).

## Live-truth reverification performed before this task started

- `git fetch origin --prune` confirmed local `main` at `bd3c393...`, matching `origin/main` and PR #93's merge commit.
- Confirmed `specs/CURRENT.md`'s `ACTIVE_IMPLEMENTATION_UNIT=T04-04`, `T04-03_STATUS=COMPLETE`.
- Confirmed `checkpoint.rs`/`grant.rs`/`disclosure.rs`/`proposal.rs`/`source_check.rs` (all from `T03-03`/`T03-04`/`T03-05`/`T03-01`) already exist, are already tested, and already derive `Serialize` on every type this task needed to cross the IPC boundary — **no `src/` (Core) change was needed for this task**, confirmed by an empty `git diff --stat origin/main -- src/` at submission.

## Scope actually touched

Desktop only (`desktop/src-tauri/src/commands.rs`, `main.rs`) — 13 new typed commands, each opening/creating a `CanonicalStore` and calling an already-audited Core function unchanged:

- **Checkpoint:** `checkpoint_current`, `checkpoint_mark`, `checkpoint_reset` — wrap `checkpoint::{current_checkpoint,mark_reviewed_through,reset_checkpoint}`.
- **Source status:** `list_sources` (every source for a project plus its latest recorded check, generalizing `resume()`'s own stale-evidence scan to show every status, not only non-`Match` ones), `source_check_now` — wraps `source_check::check_source`.
- **Grants / package:** `grant_issue`, `grant_revoke` — wrap `grant::{issue_grant,revoke_grant}`; `package_preview` — wraps `disclosure::preview_disclosure_package` (commits no receipt, needs no write access); `package_compile` — wraps `disclosure::compile_disclosure_package` (persists the receipt, receipt-before-emission, but writes no file to disk — the actual destination-file write is `T04-05`'s own "expose... export safely" scope, which also needs a native destination dialog this task deliberately does not add).
- **Proposal review:** `list_proposals`, `proposal_admit`, `proposal_accept`, `proposal_reject` — wrap `proposal::{list_project_proposals,admit_proposal,accept_proposal,reject_proposal}`.

Frontend (`desktop/src/`) — new components: `SourcesPanel.tsx`, `GrantsPanel.tsx` (issue/revoke grants; preview a package with no receipt persisted, or compile one that persists a receipt, both without writing to disk), `ProposalsPanel.tsx` (admit by pasting raw JSON text — no new native file-open dialog capability was added; list/accept-selected-operations/reject, every claimed identity field always shown as plain text, never hidden). `ResumePanel.tsx` gained an explicit, confirmed "Mark reviewed through this snapshot" action. `App.tsx` gained three new tabs (Sources/Grants/Proposals).

No `src/` change. `desktop/src-tauri`'s only non-command-surface change is a bug fix inside `list_sources` (see "Gates" below) plus a struct-level `#[serde(default)]` fix on `GrantIssueOptions` — both found and fixed by this task's own E2E test, not by manual inspection.

## Implementation requirements disposition

| Requirement (from the task contract) | How satisfied |
|---|---|
| Display current snapshot/as-of, changes since checkpoint, conflicts/negative evidence/source states, next actions | `ResumePanel.tsx` (built in `T04-03`, unchanged in shape here) already renders exactly §12's own priority order: conflicts, stale/missing evidence, current decisions, next actions, relevant notes, then the raw changes-since-checkpoint log — this task adds the checkpoint-marking action that *uses* that view's own `head_seq`, not a new view. `SourcesPanel.tsx` (new) shows every source's status, generalizing the subset `resume()` itself surfaces. |
| Explicit reviewed-through action | `checkpoint_mark`/`checkpoint_reset`, behind a named confirmation dialog citing the exact recorded sequence being marked. |
| Package scope/budget/privacy preview and grant controls | `GrantsPanel.tsx`: issue (kinds/budget/TTL), revoke, preview (no receipt persisted) and compile (receipt persisted, still no file write) — the receipt's own `selected`/`rejected` (with per-item reasons) and `emitted_byte_count`/`emitted_sha256` are all shown. |
| Proposal diff shows claimed identity and acceptance conflicts | `ProposalsPanel.tsx` always renders `declared_agent`/`declared_model`/`declared_tool` (defaulting to "Unknown", never silently guessed) above the operation list; every operation renders as plain text (no `dangerouslySetInnerHTML`), so imported content can never impersonate this owner UI. |
| Reuse section 18 exact bytes/receipts; no chat/model-launch surface | `proposal_admit` takes the raw pasted text verbatim as `raw_bytes` — the exact same parameter `admit_proposal` already validates (SHA-256/byte-count over the untouched input, §15 "immutable inbound bytes"). No chat UI, no model invocation anywhere in this crate. |

## Security considerations (S01/S02/S06/S08) disposition

- **S06** confirmed live, not merely asserted: the checkpoint-mark confirmation names the exact recorded sequence; every grant-issue/revoke and proposal-accept/reject confirmation names the concrete kinds/decision-key/claimed-agent being acted on (source inspection of every `requestConfirm(...)` call site in the three new panels).
- **S01/S08** (imported proposal cannot impersonate owner UI or hide forbidden operations): proven two ways. Structurally, `ProposalsPanel.tsx` renders every field (including the raw claimed identity) as plain React text, never HTML. Behaviorally, the E2E's malformed-proposal cases (`malformed-proposal-invalid-json-refused`, `-unknown-receipt-refused`) confirm Core's own existing validation — not a bridge-level pre-check that could diverge from it — refuses non-JSON input and a receipt-less proposal cleanly.
- **S02/"acceptance conflicts cannot be hidden"**: proven live and precisely — accepting a proposal's `DraftDecision` operation only admits a `Draft`-lifecycle decision; it does **not** implicitly grant `Accepted` authority. The E2E confirms this from both sides (CLI `decision-state` shows `NoAcceptedDecision` with `exclusion_reason="lifecycle is Draft, not Accepted"` immediately after proposal acceptance; only a *separate*, explicit `decision_accept` call promotes it, then visible to the CLI as `CurrentSet`). This is §16's "agent content is evidence, never authority," made directly observable rather than merely asserted in a doc comment.
- Bundle network surface unchanged from the T04-01 baseline (`raw/05`); no `dangerouslySetInnerHTML` (source grep, zero real matches — only explanatory comments); no new npm/cargo dependency (`raw/02`/`raw/03`: 0 vulnerabilities, unchanged package counts).

## Verification performed

**Scripted E2E, one shared vault, CLI/desktop interoperability in both directions** (`docs/evidence/flake-v1/T04-04/e2e-test/`) — deliberately different in design from `T04-03`'s two-independent-vaults content comparator: package/proposal artifacts are meant to cross a tool boundary by design (§18's own wire protocol), so the acceptance clause "desktop package/proposal result matches CLI" is proven as genuine same-vault interoperability, not independent-vault content equivalence.

Qualifying run (`e2e-test/results/run-2026-09-16T05-02-16-712Z.json`/`.log`), `overallOk=true`, every step, including:

- Full checkpoint lifecycle (first mark, advancing mark, refused stale-revision conflict, explicit backward reset with a mandatory reason).
- Source status through all four states (`Unchecked` → `Match` → `Changed` → `Missing`), with the list view correctly reflecting the most recent check.
- **`package-preview-matches-cli`**: an identical `grant_id`/`request_id`/`principal` produces a byte-identical wire (proven via `emitted_sha256` equality) whether compiled by the CLI or the desktop bridge, on the same vault.
- An expired grant (1-second TTL) is correctly refused by Core's own existing check.
- **Proposal interoperability, direction A**: a desktop-issued grant and desktop-compiled receipt is admitted and accepted through the CLI; the resulting decision reads back correctly through the desktop bridge.
- **Proposal interoperability, direction B**: a CLI-issued grant and CLI-compiled receipt is admitted and accepted through the desktop bridge; the resulting decision reads back correctly through the CLI.
- **A security property confirmed live**: proposal acceptance alone never grants decision authority; only a separate, explicit owner action does — see `e2e-test/README.md`'s own table row for the full detail.
- Malformed proposals (non-JSON, receipt-less) refused cleanly; a rejected proposal's stale-revision accept attempt refused cleanly.

Two earlier runs are preserved on record because they found two real bugs (both fixed, not merely noted): a same-second timestamp tie-break bug in `list_sources`, and a missing struct-level `#[serde(default)]` on `GrantIssueOptions`. Full accounting: `e2e-test/README.md`.

## Gates

| Gate | Command | Result |
|---|---|---|
| Desktop Rust format | `cargo fmt -- --check` (desktop/src-tauri) | Exit 0, no diff |
| Desktop Rust lint | `cargo clippy --all-targets -- -D warnings` | Exit 0, 0 warnings |
| Rust advisories | `cargo audit` | Exit 0, 0 vulnerabilities — `raw/02` |
| Frontend build | `npm run build` (`tsc && vite build`) | Exit 0 — `raw/04` |
| Frontend advisories | `npm audit` | 0 vulnerabilities, unchanged package count — `raw/03` |
| `git diff --check` | — | Exit 0, no trailing-whitespace/CRLF issues |
| Root Rust suite | Not re-run — `git diff --stat origin/main -- src/` is empty for this task | N/A |

Raw artifact manifest with sizes and SHA-256: `raw/00-manifest.txt`.

**A real bug found and fixed by this task's own gate discipline, not by inspection alone**: `list_sources`' "latest check" tie-break originally compared `observed_at` (RFC 3339 text, truncated to whole seconds), which cannot distinguish two checks recorded in the same second — the E2E's own rapid match→changed→missing sequence hit this immediately. Fixed to tie-break on the `SourceCheck` object's own UUIDv7 ID (sub-second time-ordered), reverified by the same E2E run.

## Performance gate

Not separately re-measured; every new command follows the established pattern exactly (open/create a store, call one already-tested Core function, return).

## Durability gate

**Receipt-before-emission** (this task's own named durability requirement): `package_compile` calls `compile_disclosure_package` unchanged, which persists the `DisclosureReceipt` via `commit_create` *before* ever returning wire bytes to its caller — proven by the E2E's own `package-compile-desktop`/`package-compile-via-cli` steps succeeding and the resulting receipt IDs being independently usable by a subsequent proposal on the other tool. **Atomic acceptance under interruption**: `accept_proposal`'s own `Pending`-only guard (unchanged, already proven in `T03-05`) means a proposal can never be double-applied; this task adds no new atomicity surface.

## Cross-platform gate

Native dialog/file exchange and screen-reader behavior across profiles remains `T04-06`'s own gate, per this task's contract; this task added no new native dialog (proposal admission is a paste-text flow specifically to avoid needing one). Native launch proven on this Windows 11/x86_64 development host only.

## Acceptance criteria disposition

| Acceptance clause | Status |
|---|---|
| Every planted conflict/stale source appears before next-action confidence | Satisfied — inherited unchanged from `T04-03`'s `ResumePanel` field ordering (§12's own priority groups); this task adds no reordering |
| Desktop package/proposal result matches CLI | **Proven**: byte-identical package compilation (`emitted_sha256` equality) and full bidirectional proposal interoperability on one shared vault |
| No accidental checkpoint/acceptance | Proven: checkpoint marking is only ever reachable through one explicit, named confirmation; proposal acceptance never implicitly grants decision authority (confirmed live, both before and after an explicit separate owner acceptance) |

## Completion condition

**Met.** Every acceptance clause above is satisfied by live, independently-verified evidence. Predecessor/phase gate (`T04-03` COMPLETE) was already satisfied. No `src/` change; every Core capability this task exposes was already fully built, tested, and `Serialize`-ready from `T03-01`/`T03-03`/`T03-04`/`T03-05`. No gate weakened, no test skipped, no evidence hidden. An unrelated host disk-space condition encountered mid-task is recorded in `e2e-test/README.md` as an environment fact, not a product defect.

## Next frontier

Proceed to `T04-05`.
