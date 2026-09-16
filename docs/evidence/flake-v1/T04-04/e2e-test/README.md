# T04-04 resume/checkpoint/source-status/package/proposal E2E test

Proves this task's acceptance clauses: "Every planted conflict/stale source appears before next-action confidence; desktop package/proposal result matches CLI; no accidental checkpoint/acceptance."

## Mechanism

`run-e2e-test.mjs` reuses the established CDP-driven harness (loopback WebView2 remote debugging, `window.__TAURI_INTERNALS__.invoke`) to drive the real compiled `flake-desktop.exe` through its exact real typed-IPC bridge, on **one shared vault** that both the desktop app and the real compiled `fehrest` CLI operate on directly (`--vault <the same path>`). This is a deliberately different design from T04-03's own comparator (two independent vaults compared for content-equivalence): package/proposal artifacts are meant to cross a tool boundary by design (§18's own wire protocol), so proving CLI/desktop parity here means proving genuine interoperability on the same canonical state, in both directions -- not just "produces the same shape independently."

## What each check proves

| Area | Checks | What they prove |
|---|---|---|
| Checkpoint | `checkpoint-current-initially-null`, `-mark-first`, `-mark-advances`, `-mark-conflict-refused`, `-reset` | The full checkpoint lifecycle (first mark with no prior revision, advancing mark with `expected_revision_id`, a genuine refused conflict on a stale mark, and an explicit backward reset with a mandatory reason) all work through the real bridge. Never happens except from this task's own explicit, confirmed "Mark reviewed through this snapshot" action (`ResumePanel.tsx`) -- the "no accidental checkpoint" acceptance clause. |
| Source status | `source-check-match/-changed/-missing`, `source-list-reflects-latest-check` | A source's live status (via re-reading its claimed local path) transitions correctly as the underlying file changes, and the source-status list reflects the *most recent* check -- a real bug (ties on second-precision `observed_at` timestamps for checks issued within the same second) was found and fixed here: tie-breaking now uses the `SourceCheck` object's own UUIDv7 ID (sub-second time-ordered), not the timestamp text. |
| Grant / package | `package-preview-matches-cli`, `expired-grant-refused`, `package-compile-desktop` | Package compilation is proven deterministic *and* CLI/desktop-interoperable on the same vault: an identical `grant_id`/`request_id`/`principal` produces byte-identical output between the CLI and the desktop bridge, confirmed via `emitted_sha256` equality (a SHA-256 match is a byte-identical-wire proof, not merely "looks similar"). An expired grant (1-second TTL, awaited past expiry) is correctly refused by Core's own existing check, not a bridge-level shortcut. |
| Proposal interop, direction A | `package-compile-desktop`, `proposal-a-admit-via-cli`, `-accept-via-cli`, `-result-visible-to-desktop` | A **desktop-issued** grant and **desktop-compiled** (persisted) receipt is admitted and accepted through the **CLI**, and the resulting decision is read back correctly through the **desktop** bridge. |
| Proposal interop, direction B | `package-compile-via-cli`, `proposal-b-admit-via-desktop`, `-accept-via-desktop`, `-result-correct` | A **CLI-issued** grant and **CLI-compiled** receipt is admitted and accepted through the **desktop** bridge, and the resulting decision is correct. |
| Security property confirmed, not assumed | `proposal-acceptance-alone-does-not-grant-decision-authority`, `proposal-drafted-decision-explicitly-accepted-by-owner`, `explicit-owner-acceptance-now-visible-to-cli` | Accepting a proposal's `DraftDecision` operation only admits it as Core's own `Draft` lifecycle -- it does **not** auto-promote to `Accepted`. `decision-state` via the **CLI** confirms `NoAcceptedDecision` with `exclusion_reason="lifecycle is Draft, not Accepted"` immediately after the desktop-side proposal acceptance; only a *separate*, explicit `decision_accept` call (an owner action, never implied) promotes it, confirmed visible to the CLI afterward as `CurrentSet`. Agent content is evidence, never authority (S16) -- this is the acceptance criterion's own "proposal diff shows claimed identity and acceptance conflicts" made concrete, not a bug this task introduced or had to work around. |
| Malformed proposal | `malformed-proposal-invalid-json-refused`, `malformed-proposal-unknown-receipt-refused` | Non-JSON input and a well-formed-but-empty-operations proposal are both refused cleanly by Core's own existing validation, not a bridge-level pre-check that could diverge from it. |
| Expected-revision conflict on proposals | `proposal-c-admit`, `-reject`, `proposal-accept-conflict-refused` | Rejecting a proposal then attempting to accept it with the pre-reject revision is refused (the proposal is `Rejected`, not `Pending` -- Core's own status guard, the same class of protection T04-03's action-transition conflict probe already established). |

## Runs on record

| Run | Result |
|---|---|
| `run-2026-09-16T04-44-33-747Z` | Found two real bugs, both fixed: (1) `list_sources`' tie-break on same-second `observed_at` timestamps picked the wrong "latest" check; (2) `grant_issue`'s bundled options struct lacked struct-level `#[serde(default)]`, so passing `{}` from the frontend (as `DecisionCreateExtra` already did safely, since all of *its* fields are `Option<T>`, which serde defaults implicitly) failed with a missing-field error for the non-`Option` `privacy_exclusions: Vec<String>` field. |
| `run-2026-09-16T04-54-31-193Z` | Both bugs fixed and reverified; found that the test's own assumption about proposal acceptance implying decision acceptance was wrong (see the security-property row above) -- not a product defect, a test-design correction. |
| `run-2026-09-16T05-02-16-712Z` | **Qualifying run.** `overallOk=true`, all steps pass, including the corrected security-property verification in both its "not yet accepted" and "explicitly accepted" states. |

Full step-by-step detail: `results/run-2026-09-16T05-02-16-712Z.json` / `.log`.

## An unrelated host condition encountered mid-task

Partway through this task, the host's `C:` drive free space dropped from ~1.5 GB to ~25 MB within roughly 15 minutes -- verified not to originate from this session's own work (Flake's build artifacts and test files totaled a few MB over that window). This caused one build to fail with a linker error (`LNK1318`, an out-of-space PDB-write failure) requiring a `cargo clean` + full rebuild of the desktop crate to recover, mirroring T04-01's own previously-documented disk-space constraint on this host. The user freed space on their end during this task; recorded here as a host-environment fact, not a Flake defect.
