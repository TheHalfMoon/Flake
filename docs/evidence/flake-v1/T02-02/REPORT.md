# T02-02 evidence report — Capture exact notes and selected local evidence

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §12/§15/§20/§22/§27-29, task `T02-02` — second task of `P02`
- **Baseline / tested source commit:** forked from `origin/main` `0681867` (PR #74, `T02-01` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5 / Claude Fable 5.1 session)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy`. All checks below were executed locally on the development host recorded in `specs/002-post-r1-canonical-core-convergence/corrective-t01/reference-hardware.md`.

## Live-truth reverification performed before this task started

- `git status`/`git log --oneline -3` confirmed `main` at `0681867` ("feat(t02-01): create and manage the four project record types (#74)"), matching `specs/CURRENT.md`'s `T02-01_STATUS=COMPLETE`.
- Re-read the full `T02-02` task-contract row before writing any code, plus the plan sections it names: §15 (`Source`/`Source revision`/`Artifact` rows), §20 (S01/S03/S04/S07), §22 (F01/F07/F08/F10/F15), §27 (source label ≤4 KiB, artifact ≤64 MiB, performance budgets), §29 (V02-V09).
- Confirmed `src/locator.rs`'s existing containment/symlink-refusal pattern (used for vault-internal reads) before designing this task's own, differently-scoped, external-path admission — the two are deliberately not unified (see "Architecture").

## Scope actually touched

Two new modules (`src/capture.rs`, `src/markdown.rs`), plus the minimum necessary extension of `src/project.rs`'s existing `RecordPayload` dispatch to admit a fifth kind, `src/cli.rs` (new subcommands), `src/lib.rs` (module registration, one new `Error::Capture` variant, one new transport-level size constant), and `src/canonical.rs` (widening one existing backstop constant — see "A real, necessary widening of an existing constant" below). No `Cargo.toml`/`Cargo.lock` change: hex encoding and UTC-from-epoch formatting are both hand-written rather than pulling in `base64`/`chrono`, matching this crate's stated minimalism (`Ponytail DELETE: clap` precedent) and the exact same "no new dependency for clock formatting" boundary `vault::chrono_like_now_iso8601`/`canonical::now_iso8601_placeholder` already accept.

```text
src/canonical.rs |  20 +-
src/capture.rs   | 914 +++++++++++++++++++++++++++++++++++++++++++++++++++++++
src/cli.rs       | 340 ++++++++++++++++++++-
src/lib.rs       |  19 ++
src/markdown.rs  |  75 +++++
src/project.rs   |  35 ++-
6 files changed, 1393 insertions(+), 10 deletions(-)
```
(`git diff --stat main -- src/`, `raw/04-diff-stat.txt`)

## Architecture: a fifth `RecordPayload` kind, not a new entity family

Exactly like `T02-01`'s own four record kinds, `Source` is an ordinary opaque-payload canonical object committed through the existing `CommandTarget::CreateObject`/`UpdateObject` and `CanonicalWriter::commit` — no new `CommandTarget` variant, no schema change beyond nothing at all (not even a new read method this time; `capture::list_project_sources` reuses `list_current_objects` exactly like `project::list_project_records` does). A file import's "source revision" (plan §15: exact captured bytes, digest, length, observed time) is simply the `Source` object's own next revision, reusing the identical "archive is a new revision, history preserved" pattern `T02-01`'s `archive_project` established — proven again here by `deactivate_and_reactivate_preserve_history_like_project_archive`.

This task's own plan row separately names `Source`, `Source revision`, and `Artifact` as three distinct §15 rows. This implementation deliberately does **not** introduce `Artifact` as a separately-referable entity shared by multiple owners (e.g. a future agent proposal) — nothing in `T02-02`'s own scope needs an artifact addressable independently of the `Source` that captured it, and inventing that sharing mechanism now would be exactly the kind of successor-work scope creep the task's own forbidden-scope clause rules out. It is recorded here as a deliberate deferral, to whichever later task first needs a shared artifact reference (most likely alongside `T02-06`'s import/export or `T03-04..06`'s agent proposals).

`Source` is dispatched through `project::RecordPayload`'s existing single serialization choke point (`to_json`/`from_json`), not a parallel one — `project.rs` gained one new match arm per function plus three visibility widenings (`check_len`, `require_project`, `commit_create`/`commit_update` from private to `pub(crate)`) so `capture.rs` could reuse them instead of duplicating field-limit/cross-reference logic.

## Security boundary — what this task actually enforces, and its named limits

Per §20 S01/S03/S04/S07, restated here against the literal implementation, not just the module docs:

- **No recursive scan, structurally.** `capture_file` never calls `read_dir` anywhere in this module. Only the single path a caller supplies is ever touched.
- **Symlinks refused before opening.** `std::fs::symlink_metadata` is checked and refused *before* `File::open`, mirroring `locator::open_confined`'s own stated ordering rationale (an opened handle cannot un-resolve a symlink after the fact). `refuses_a_symlink_without_ever_opening_the_target` proves the refusal and that zero objects are admitted.
- **Obvious-secret filenames refused, with the limitation stated in the refusal message itself** (S07 "explain limits"), not only in a doc comment: `detect_obvious_secret` is a small, named, admittedly incomplete denylist (`.env`, `id_rsa`, `credentials.json`, `*.pem`, etc.) with an explicit non-secret carve-out (`.env.example` and similar `.example`/`.sample`/`.template`/`.dist` suffixes remain importable). `refuses_obvious_secret_filenames_but_admits_their_example_carve_out` proves both halves.
- **Size ceiling enforced against actually-read bytes, not `stat()` metadata alone** — this task's own performance-gate clause, "bounded reads after open, not metadata-only checks," read literally: `capture_file` opens the file and reads via `Read::take(MAX_ARTIFACT_BYTES + 1)`, then checks the *actual* buffer length. A file whose reported size lies would still be caught. `oversized_artifact_is_refused_before_any_mutation` uses `File::set_len` to construct an over-limit file cheaply (no real bytes written to the shared host's disk) and proves the transaction head is unchanged after refusal.
- **No network fetch, no HTML/Markdown execution, no auto-promotion.** Structural, not merely asserted: no code path in `capture.rs` or `markdown.rs` makes a network call, parses Markdown syntax, or creates a `Decision`/`Relation` as a side effect of an import.
- **`origin_label` is unconditionally `"unknown"`**, and **`source_mtime` is the filesystem's own reported value or `None`**, never fabricated — there is no parameter anywhere in this module's public API that lets a caller assert a claimed origin or time (I11 "observed time not invented source time"; F10 "invalid provenance... retain as unconfirmed claim, not authority" — this task does not even offer the claim path yet).

## A real, necessary widening of an existing constant

`canonical::commit_with_fault`'s existing payload-size backstop checked `payload.len() > crate::limits::MAX_OBJECT_BYTES` (1 MiB) unconditionally for *every* `CommandTarget`, including this task's new `Source` kind. Since §15 fixes the artifact ceiling at 64 MiB and this task's own architecture hex-encodes captured bytes directly into the JSON payload (2x inflation, ~128 MiB), that 1 MiB backstop made storing any artifact over roughly 750 KiB *structurally impossible* — not a product limit, an accidental one, discovered while sizing this task's own binary-fixture tests. Fixed by introducing a distinct, explicitly-named `crate::limits::MAX_COMMAND_PAYLOAD_BYTES` (150 MiB, sized with margin above the 128 MiB worst case) as the transport-level backstop, leaving `MAX_OBJECT_BYTES` (1 MiB) unchanged and still exclusively governing text-body product limits (`project::MAX_BODY_BYTES` still equals it, unchanged). This is a widening of a defense-in-depth ceiling to admit a newly-implemented, plan-specified capability — not a weakening of any existing, currently-enforced product limit: no existing record type's own field bound moved. `canonical::tests::oversized_payload_is_refused_before_any_mutation` was updated in place (not deleted) to assert against the new constant, with an inline comment explaining why, and still proves the same property: an oversized payload is refused before any transaction opens.

## Correct arbitrary-past-time formatting (a genuine first for this codebase)

Every existing "now" timestamp in this codebase (`vault::chrono_like_now_iso8601`, `canonical::now_iso8601_placeholder`) is a documented, already-accepted placeholder that only ever formats the *current* instant, using a fixed fake calendar date — acceptable there because nothing previously needed to render an arbitrary *past* instant correctly. `Source::capture::source_mtime` is the first field in this codebase that must: a captured file's real modification time can be any historical date, and reusing the existing placeholder would render a wrong calendar date for it — precisely the "invented source time" this task's own I11 invariant forbids. `capture.rs` implements `rfc3339_utc_from_unix_seconds` using Howard Hinnant's public-domain `civil_from_days` algorithm (no new dependency), tested against the widely-cited 2000-01-01 UTC epoch milestone plus derived leap-year/non-leap-year boundary checks (`rfc3339_formats_known_epoch_fixed_points`).

## What was built, and why — grounded in the task contract

1. **`capture` CLI command, defaulting to `Note`, explicit `Action`/`Decision` selection** (`--kind note|action|decision`, default `note`) — the literal "Note default, explicit Action/Decision selection" requirement. An unrecognized `--kind` is rejected with exit code 64, not silently defaulted (proven in `capture_and_source_lifecycle_works_through_the_cli`).
2. **`Source` file import** (`capture::import_file`): validates the project reference and label length, refuses symlinks/non-regular-files/missing paths/secret filenames/oversized content, then captures exact bytes, SHA-256, sanitized display filename, best-effort MIME label (filename-extension allowlist only, never content-sniffed), observed-at and source-mtime timestamps, and the unconditional `"unknown"` origin label.
3. **`Source` manual reference** (`capture::create_manual_reference`): the "explicitly reference-only" half of §15's `Source revision` row — no bytes, only descriptive claimed-repository/commit/path metadata.
4. **Source lifecycle** (`deactivate_source`/`reactivate_source`): F08 "missing source... retain saved source revision" — marking a source unavailable is a new revision, never a deletion; the captured bytes remain in history.
5. **Byte recovery** (`capture::extract_bytes` + CLI `source-extract`): this task's own objective phrase, "recoverable bytes," made concrete and testable rather than merely internal — a caller can always get back the exact original bytes, proven end-to-end through the CLI in `capture_and_source_lifecycle_works_through_the_cli` (write a fixture, import it, extract it to a different path, compare bytes).
6. **Markdown preview contract** (`markdown::preview`, wired to `record-show --preview N`): a bounded, `char`-boundary-safe truncation for listings, proven never to split a multi-byte sequence (CJK/emoji fixture) and proven that hostile-looking embedded content (`<script>...</script>`, a fake YAML-frontmatter-shaped block) survives as an exact literal substring — i.e. that nothing in this module interprets or executes it (S01).

## Tests added (19 new; all pass)

| Test | File | What it proves |
|---|---|---|
| `hex_round_trips_every_byte_value` | capture.rs | The hand-written hex codec is lossless for all 256 byte values |
| `rfc3339_formats_known_epoch_fixed_points` | capture.rs | Correct arbitrary-past UTC formatting, incl. leap/non-leap year boundaries |
| `imports_a_binary_fixture_with_exact_byte_fidelity` | capture.rs | Opaque byte-for-byte fidelity for non-UTF-8 binary content (V02/V09 binary fixture) |
| `imports_unicode_and_crlf_text_with_exact_byte_fidelity` | capture.rs | Exact byte survival for Unicode + CRLF text (V02 boundary/IME-adjacent case) |
| `refuses_a_symlink_without_ever_opening_the_target` | capture.rs | S03 symlink refusal before any open; zero objects admitted (V09) |
| `refuses_a_directory_and_a_missing_path` | capture.rs | "Prohibited selected paths" — non-regular-file and missing-source refusals (V09/F08) |
| `refuses_obvious_secret_filenames_but_admits_their_example_carve_out` | capture.rs | S07 secret-sentinel refusal plus its documented non-secret carve-out (V09) |
| `oversized_artifact_is_refused_before_any_mutation` | capture.rs | §27 64 MiB ceiling enforced pre-transaction; head unchanged |
| `manual_reference_source_carries_no_bytes` | capture.rs | Reference-only sources never carry a `capture` |
| `deactivate_and_reactivate_preserve_history_like_project_archive` | capture.rs | F08 unavailable-marking preserves history (3 revisions) |
| `source_requires_an_existing_project` | capture.rs | Cross-reference validation reused from `project.rs` |
| `extract_bytes_recovers_the_exact_original_content` | capture.rs | This task's own "recoverable bytes" objective |
| `extract_bytes_refuses_a_manual_reference_with_no_captured_bytes` | capture.rs | No fabricated empty-bytes result for a reference-only source |
| `a_fault_during_source_commit_leaves_no_partial_admission` | capture.rs | "Process failure" (F01/F15): reuses `T01-07`'s existing `CommitFaultPoint`, no new fault point needed |
| `short_body_is_returned_unchanged_with_no_marker` | markdown.rs | Preview never alters content that already fits |
| `truncates_on_a_char_boundary_never_splitting_multibyte_sequences` | markdown.rs | CJK/emoji safety (V02 IME-relevant text) |
| `embedded_html_and_script_like_content_stays_literal_and_unexecuted` | markdown.rs | S01: no Markdown/HTML interpretation exists to bypass |
| `full_body_remains_available_unmodified_regardless_of_preview` | markdown.rs | Preview never mutates or truncates canonical truth |
| `capture_and_source_lifecycle_works_through_the_cli` | cli.rs | End-to-end: default-Note capture, explicit-kind capture, file import, extraction, manual reference, deactivate/reactivate, preview, and an unrecognized-kind rejection — all through the actual CLI dispatcher |
| `oversized_payload_is_refused_before_any_mutation` (updated) | canonical.rs | The widened transport backstop still refuses before any mutation |

## Commands executed (all re-run on this branch against `main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked` | (folded into the test run) | Clean build, 0 warnings |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | **211/211 pass**, 0 failed (178 lib [159 pre-existing + 19 new] + 10 integration + 23 kill_tests) |
| `cargo test --locked --lib capture::` | `raw/07-capture-module-tests-isolated.txt` | 14/14 pass |
| `cargo test --locked --lib cli::` | `raw/08-cli-module-tests-isolated.txt` | 5/5 pass (4 pre-existing + 1 new) |
| `cargo test --locked --lib markdown::` | `raw/09-markdown-module-tests-isolated.txt` | 4/4 pass |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --stat main -- src/` | `raw/04-diff-stat.txt` | Scope confirmed as listed above |
| `cargo run --release --example capture_timing` (throwaway harness, deleted before this commit) | `raw/05-capture-and-source-timing.txt` | text import p95 5.84ms (n=20); 1 MiB binary import p95 136.62ms (n=5); 1 MiB extract p95 8.09ms (n=5) |
| Environment capture | `raw/06-environment.txt` | Matches `reference-hardware.md`'s frozen development profile |

## Failed attempts / exclusions

- **A field-name collision, caught by the test suite on the first run, not discovered later.** `Source`'s own `kind: SourceKind` field (file/manual-reference) collided with `RecordPayload`'s dispatch-tag key, also named `"kind"`. `to_json`'s tag insertion silently overwrote the struct's own `kind` value in the serialized JSON; `from_json`'s `remove("kind")` then removed what it thought was only the dispatch tag, leaving the concrete `Source` struct's deserialization missing its own required field (`missing field \`kind\``), failing 4 of the new tests immediately. Fixed by renaming the struct field to `source_kind` — a naming fix, not a change to the dispatch architecture itself (the exact same architecture `T02-01` already established and this task deliberately reused).
- Two `clippy::manual_find`/`clippy::manual_is_multiple_of` lints on first `clippy` run (a hand-rolled loop-and-return instead of `Iterator::find`, and `% 2 != 0` instead of `.is_multiple_of(2)`); both fixed to the suggested idiom, not suppressed.
- No test was skipped, deleted, or weakened to reach green.

## Performance gate

Closest applicable measurement, at S-scale (`raw/05-capture-and-source-timing.txt`); see that file's own "Interpretation" for the full caveat against `T02-01`'s comparable commit-path timing:

| Operation | Comparison | Observed |
|---|---|---|
| Source import, 1 KiB text | `T02-01`'s own typed-record create (p95 6.2ms) | p95 5.84ms — consistent |
| Source import, 1 MiB binary | This task's own byte-proportional hex-encoding cost | p95 136.62ms — dominated by hex encoding + SQLite TEXT write of the ~2 MiB result, proportional to input size |
| Extraction, 1 MiB binary | Hex decode only | p95 8.09ms |

No dataset-M/L measurement, and no measurement at the full 64 MiB `MAX_ARTIFACT_BYTES` ceiling: both remain `T05-02`'s full hardware-qualified pass, exactly like `T02-01`'s own equivalent deferral for record counts at M/L scale.

## Durability gate

D1/D2 per capture and artifact transaction: every `Source` mutation is, structurally, the exact same `CanonicalWriter::commit` call `T01-07`'s D1 matrix already ran 100 fault schedules against — no new transaction path exists for this task to separately regression-test. `a_fault_during_source_commit_leaves_no_partial_admission` is this task's own regression sample, reusing the existing `CommitFaultPoint::BeforeSqlCommit` (no new fault point was needed), proving a source-import commit still exhibits "complete pre-state or complete committed state, never a half-state" for this task's own new call site.

## Cross-platform gate

Native development profile only (Windows 11/NTFS, matching `reference-hardware.md`), consistent with every prior `flake-v1` task. The symlink-refusal test uses `#[cfg(unix)]`/`#[cfg(windows)]` conditional construction and skips (rather than fabricates a pass) if Windows file-symlink creation is unavailable in this environment (it was available and exercised here). All remaining profiles are `T05-02`'s scope.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Text/Unicode/CRLF/opaque artifact fidelity exact | Satisfied — `imports_unicode_and_crlf_text_with_exact_byte_fidelity`, `imports_a_binary_fixture_with_exact_byte_fidelity` |
| Rejected inputs leave no partial record | Satisfied — `oversized_artifact_is_refused_before_any_mutation`, `refuses_a_symlink_without_ever_opening_the_target` (both assert transaction head / object count unchanged) |
| No unseen filesystem/network access | Satisfied by construction — no `read_dir`, no network call anywhere in `capture.rs`/`markdown.rs`; only the single caller-supplied path is ever touched |
| Note default, explicit Action/Decision selection | Satisfied — CLI `capture` command, `capture_and_source_lifecycle_works_through_the_cli` |
| No recursive scan, symlinks or secret-file defaults | Satisfied — see "Security boundary" above |
| Original bytes/digest/time and unknown origin label preserved | Satisfied — `Capture` struct fields, `imports_a_binary_fixture_with_exact_byte_fidelity` asserts `origin_label == "unknown"` and a present `source_mtime` |
| No HTML execution, network fetch or auto-promotion | Satisfied structurally — see "Security boundary" and "Markdown preview contract" |
| Binary artifacts remain opaque and bounded | Satisfied — §15's 64 MiB ceiling enforced against actually-read bytes |

## Completion condition

Every acceptance clause above is satisfied. `Decision`/`Action` evidence linkage via a `Relation` to a `Source` remains `T02-03`'s objective (this task only admits sources; it does not yet let a decision cite one). Project-scoped indexing of sources remains `T02-04`'s objective. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (211/211). One real bug (the `kind`-field collision) was caught by this task's own tests and fixed at the root cause; one necessary, explicitly-justified widening of an existing transport-level constant was made and documented, with no change to any product-facing field limit. No sealed evidence altered; no force-push; no historical evidence file touched; no change to `T01-02`–`T01-07`'s or `T02-01`'s already-audited mutating logic beyond the one documented constant widening. `T02-02` is complete.

## Next frontier

`T02-03` — Record decisions and complete actions with visible history. Depends on `T02-02` (this task) and `T02-01`. Builds the full `Action` state machine, `Decision` evidence linkage/override/supersession, and (per this task's own deferral) the `Relation` type connecting a `Decision` to a `Source` this task admitted.
