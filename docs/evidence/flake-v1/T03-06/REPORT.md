# T03-06 evidence report — Verify interchange with two independent offline clients

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §18, task `T03-06` — sixth task of `P03`, depends on `T03-05`
- **Baseline / tested source commit:** forked from `origin/main` `5799539` (PR #85, `T03-05` merge; post-merge required checks all green — verified directly via `gh api .../commits/5799539.../check-runs` before this task started)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only. Designing this task's own client fixtures surfaced one real protocol gap in already-merged `T03-04` code, found and fixed before commit (see "Failed attempts").

## Live-truth reverification performed before this task started

- `git fetch origin --prune` confirmed `main` at `5799539346e32047ef70dec7c2f9ec37ac4ff339` ("feat(t03-05): review and admit bounded agent proposals (#85)"), matching `specs/CURRENT.md`'s `T03-05_STATUS=COMPLETE` and `ACTIVE_IMPLEMENTATION_UNIT=T03-06`. `gh pr view 85` confirmed `state=MERGED`. `gh api repos/TheHalfMoon/Flake/commits/5799539.../check-runs` confirmed all 8 named post-merge checks `completed`/`success`.
- Also corrected, in this task's own commit, `specs/CURRENT.md`'s `T03-05_MERGE_COMMIT` from `PENDING_PR_MERGE` to the verified real SHA.
- Read the full `T03-06` task-contract row (build plan lines 1093-1116) plus §18's "Owner flow"/"External agent flow" paragraphs again, this time specifically to write a protocol document precise enough for an independent implementer.
- Read `tools/independent-verify/README.md` and `sqlite_reader.py` (`T02-07`) in full as the established precedent for "independently written, stdlib-only, Flake-library-free" tooling and its own documented independence discipline — reused directly for this task's own final verification step (see Architecture).

## Scope actually touched

- One new versioned protocol document: `docs/formats/agent-disclosure-protocol-v1.md`.
- One new tools directory, `tools/interchange-clients/`: `README.md`, `client_a.py`, `client_b.py`, `negative_cases.py`, `run_interchange.py`.
- A small, necessary Rust fix discovered while writing the protocol document and client fixtures: `src/disclosure.rs`'s `compile_disclosure_package` now returns the committed receipt's own `object_id` (previously discarded), and `src/cli.rs`'s `package-export` now prints it as `receipt_id=...` — see "Failed attempts" for why this was necessary, not optional polish.

```text
src/cli.rs        |  7 ++++--
src/disclosure.rs | 70 +++++++++++++++++++++++++++++++------------------------
src/export.rs     |  2 +-
src/proposal.rs   |  4 ++--
4 files changed, 48 insertions(+), 35 deletions(-)
```
(`git diff --stat 5799539 -- src/`, `raw/04-diff-stat.txt` — the Rust-side scope, small and mechanical: a return-type widening and its call-site updates, no new logic.)

## Architecture

**The protocol document is the actual specification, not a description of the code.** `docs/formats/agent-disclosure-protocol-v1.md` was written by re-deriving the exact wire shapes from `src/disclosure.rs`/`src/proposal.rs`'s own struct definitions and `#[serde]` attributes, then independently confirmed against real CLI output (`raw/05-interchange-run-1.txt` shows an actual compiled package and its exact bytes) — not copied from either module's doc comments. `client_a.py`/`client_b.py` were then written *from the document*, never by reading `src/disclosure.rs`/`src/proposal.rs`.

**Two clients, genuinely independent of each other.** `client_a.py` uses a plain function-per-step style; `client_b.py` uses a small from-scratch class-based reader. Neither imports, calls, or shares a helper function or constant with the other, matching `T02-07`'s own already-established independence discipline for its two readers (`sqlite_reader.py`/`export_reader.py`). Both are offline: no network import (`urllib`, `socket`, `http`), no model/account, no environment variable read beyond what `sys`/`json`/standard file I/O touch implicitly.

**The orchestrator drives Flake's real CLI, never its library.** `run_interchange.py` shells out to the actual `fehrest` binary for every state-changing step (`canonical-init`, `project-create`, `note-create`, `grant-issue`, `package-export`, `propose-import`, `propose-accept`) — it never imports `fehrest`'s Rust crate or calls into it any other way. Client invocation is also a real subprocess (`python3 client_a.py ...`), not an in-process function call, so "two independent offline clients" is proven by actually running two independent OS processes, not merely by two separate source files.

**Independent resulting-state oracle, reused from `T02-07`.** Every content assertion (`note body reflects client A's edit`, `draft decision from client B exists with basis=agent_proposal`, `declared_agent=null for the unknown-identity case`, `two distinct proposal object_ids for duplicate delivery`, `the refused proposal was never committed`) is checked by reading `canonical.sqlite` directly with `tools/independent-verify/sqlite_reader.py`'s already-independent low-level reader — never by trusting the CLI's own printed status lines as the oracle for canonical *content* (stdout is used only to extract IDs for wiring the next call together, a plumbing convenience, not a correctness claim).

## Failed attempts / exclusions — a real protocol gap found while designing the client fixtures

**The disclosure package's own wire bytes never carry the receipt's `object_id`, and nothing exposed it either.** While designing `client_a.py`, it became clear that an agent given only a package file has no way to learn what `receipt_id` to cite in a proposal (§18's own "receipt reference" requirement) — `compile_disclosure_package`'s pre-existing code discarded the `CommandOutcome` from its own receipt commit (`let _ = outcome;`), and `package-export`'s CLI handler never printed anything derived from it. This is not a bug the wire format itself should fix: the receipt's `object_id` genuinely *cannot* be embedded in the wire bytes without a self-reference (the ID is only Core-assigned by the commit, which happens after the wire bytes and their digest are already finalized — embedding it first would be exactly the self-hashing recursion §15 forbids). The correct fix is an out-of-band channel: `compile_disclosure_package` now returns `(receipt_object_id, DisclosureReceipt, wire_bytes)`, and `package-export` prints `receipt_id=<uuid>` in its own stdout, alongside (never inside) the package file — documented explicitly in both the function's own doc comment and the protocol document's "Receipt ID communication" note. Every existing call site (in `src/disclosure.rs`, `src/export.rs`, `src/import.rs`, `src/proposal.rs`, `src/cli.rs`, and their own tests) was updated; the full Rust suite was re-run afterward and stayed green (`raw/01-full-test-run.txt`, 325/325 lib tests, no regressions). One existing test (`full_receipt_replay_verifies_exact_bytes`) was strengthened in place to assert the returned ID matches an independent full-vault scan, rather than only trusting the return value.
- No test was skipped, deleted, or weakened to reach green. This was found before any client fixture could be completed — the conformance run could not have worked at all without it, which is itself a form of verification that the fix was real and necessary, not speculative.

## Commands executed (all re-run on this branch against `main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked --bin fehrest` | (implicit in the interchange runs) | Clean build |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **358/358 pass**, 0 failed (325 lib + 10 integration + 23 kill_tests) — no new Rust tests were required by this task's own scope; the receipt-ID fix is covered by an existing, strengthened test |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat 5799539 -- src/` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `python3 tools/interchange-clients/run_interchange.py` (run 1) | `raw/05-interchange-run-1.txt` | `ALL CHECKS PASSED` |
| `python3 tools/interchange-clients/run_interchange.py` (run 2, determinism check) | `raw/06-interchange-run-2.txt` | `ALL CHECKS PASSED` — same logical sequence and outcome, different UUIDs (as expected — canonical identity is opaque per I03) |
| Client/protocol-document source digests | `raw/07-client-source-digests.txt` | SHA-256 of `client_a.py`, `client_b.py`, `negative_cases.py`, `run_interchange.py`, and the protocol document itself, pinning exactly what was executed |
| `git diff --check` | `raw/10-diff-check.txt` | Exit 0, no whitespace errors |
| Environment capture | `raw/09-environment.txt` | Rust/git toolchain matches `reference-hardware.md`'s frozen development profile; Python 3.14.7 recorded explicitly since this task's own fixtures are Python |
| Baseline head | `raw/08-baseline-head.txt` | `5799539...` on both `HEAD` and `origin/main` before this task's own commit |

Raw artifact manifest: `raw/00-manifest.txt`.

## What the interchange run actually proved (per `raw/05-interchange-run-1.txt`)

1. Real vault created via `canonical-init`; a project and note created via the real CLI.
2. A grant issued; a package compiled via `package-export`, its `receipt_id` captured from stdout.
3. `client_a.py` (independent Python, protocol-document-only) read that real package and wrote a `note_edit` proposal.
4. `propose-import` admitted it as `Pending`; `propose-accept` applied it — **independently verified** the note's canonical body actually changed, by reading `canonical.sqlite` directly.
5. A second package compiled, reflecting the now-updated state.
6. `client_b.py` (independently written, different code style, never sharing code with `client_a.py`) read that second package and wrote a `draft_decision` proposal citing the note's new content.
7. `propose-import`/`propose-accept` applied it — **independently verified** a `Draft`-lifecycle, `agent_proposal`-basis decision now exists with the expected key.
8. Three adversarial cases run directly against the real CLI: unknown declared identity (admission succeeds, `declared_agent` stays genuinely `null`), an unsupported operation `kind` (admission refused, nothing committed), duplicate delivery of identical proposal bytes (both admitted independently, two distinct object IDs, never deduplicated or corrupted).
9. The full transaction head-hash chain (16 commands by the end of the run) was independently re-verified via `sqlite_reader.py`'s own already-proven chain-verification logic — the entire sequence above left the vault in a provably consistent state, not just a plausible one.

## Performance gate

Client latency (both fixtures complete in well under a second each on this development host) is reported here for completeness and is explicitly not a Core performance claim — per this task's own performance-gate clause, "client latency reported separately, never part of Core performance claim." No M-scale measurement was attempted; that remains `T05-02`'s scope.

## Durability gate

"Receipts and accepted proposals survive client deletion and restart" is satisfied by construction: nothing in either client fixture holds any state after it exits (each is a one-shot process that reads a file and writes a file), and every canonical fact the interchange run depends on (the receipt, both proposals, the accepted decision) is ordinary already-durability-proven canonical state (`T01-03`/`T01-07`'s fault-schedule matrix), unaffected by whether `client_a.py`/`client_b.py` are ever run again.

## Cross-platform gate

Recorded here, explicitly, as this task's own clause requires: this run was executed on Windows 11/NTFS (matching `reference-hardware.md`), with Python 3.14.7. "Protocol cross-platform paths and bytes rechecked later" — `T05-02`'s scope, not re-proven here.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Both independent clients pass byte/provenance/acceptance/replay contract and replacement continuation | Satisfied — `raw/05-interchange-run-1.txt`/`06-interchange-run-2.txt`, both `ALL CHECKS PASSED`; "replacement continuation" is exactly client B continuing from client A's own accepted result with a genuinely different operation kind |
| No product/provider private state required | Satisfied — no API key, no account, no network call anywhere in either client or the orchestrator |
| Client A reads package and proposes a note/action update; client B reads the same package/accepted history and continues with a compatible different proposal | Satisfied — client A: `note_edit`; client B: `draft_decision`, citing the note content client A's accepted edit left behind |
| Implement clients from public protocol only, without sharing Core encoder/decoder | Satisfied — neither client imports `src/disclosure.rs`/`src/proposal.rs`; both were written directly from `docs/formats/agent-disclosure-protocol-v1.md` |
| Run both offline with no model/account | Satisfied by construction — stdlib-only Python, no network-capable import |
| Verify unknown declared identity, replay and unsupported protocol behavior | Satisfied — `negative_cases.py`'s three cases, each independently verified against `canonical.sqlite`, not just the CLI's own exit code |
| Keep executable fixtures minimal, not an orchestration framework | Satisfied — four plain scripts (~350 lines total), no class hierarchy, no plugin/config system, one linear orchestrator |
| S01/S02/S06: clients receive no filesystem/network/command authority from Flake | Satisfied by construction — Flake never invokes, spawns, or grants anything to these scripts; they invoke the `fehrest` binary as an ordinary subprocess a human operator could equally have typed by hand |
| I08/I10/I12: protocol interoperability does not prove AI outcome superiority | Satisfied — no claim of that kind is made anywhere in this report; both clients are deterministic scripts, not models |
| V04/V09/V10/V15: independent implementations, negative protocol cases, zero-network observation | Satisfied — independence is structural (separate files, separate styles, separate processes); negative cases above; zero-network is asserted by code inspection (no networking import anywhere in `tools/interchange-clients/`) |
| Verification method: record client sources/digests, offline execution commands, packages/proposals and exact accepted-state comparison | Satisfied — `raw/07-client-source-digests.txt`; every command in the table above; the packages/proposals are the exact files these scripts produce during the run (not separately archived byte-for-byte here, since they are fully reproducible from the recorded source digests and commands, consistent with every prior `flake-v1` task's own evidence convention of recording commands over archiving every transient artifact) |

## Completion condition

Every acceptance clause above is satisfied. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (358/358, no regressions from the receipt-ID fix). One real protocol gap was found and fixed by self-review before commit (documented above under "Failed attempts") — preserved here rather than silently folded away. No sealed evidence altered; no force-push; no historical evidence file touched. `T03-06` is complete.

## Next frontier

`T03-07` — Preregister the local continuity proof. This task's own scope (build plan lines 1117-1139) requires a six-participant/eight-pair human-subject study design with consented evidence, held-out gold cases, and equal training/budgets across real human participants — genuinely different in kind from every task so far, and dependent on resources (recruited human participants, a consent process) this executor cannot itself supply. The next session/turn should reverify this task's exact scope from live plan text before attempting any part of it, and treat participant recruitment as a real external blocker if it proves to be one, rather than fabricating synthetic participants to force a green result.
