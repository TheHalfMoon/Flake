# T02-07 evidence report — Prove project reconstruction without Flake

- **Plan contract:** `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md` §12/§15/§20/§22/§27-29, task `T02-07` — seventh and final task of `P02`
- **Baseline / tested source commit:** forked from `origin/main` `dc2f1c1` (PR #79, `T02-06` merge)
- **Executor:** Muse (Claude Code implementation agent, model Sonnet 5)
- **Reviewer identity and independence limits:** Self-reviewed only. No second independent reviewer; no GitHub-enforced CI in this repository runs `cargo test`/`fmt`/`clippy`. All checks below were executed locally on the development host recorded in `specs/002-post-r1-canonical-core-convergence/corrective-t01/reference-hardware.md`.

## Live-truth reverification performed before this task started

- `git fetch origin --prune`, `git log --oneline --decorate -20`, `gh pr view 79 --json state,mergedAt,mergeCommit` confirmed `main` at `dc2f1c1` ("feat(t02-06): validate and import portable packages into a new vault (#79)"), merged at `2026-09-15T06:16:59Z`.
- `gh api repos/TheHalfMoon/Flake/commits/dc2f1c1.../check-runs` confirmed all 8 named checks (`manifest-check`, `canonical-equality`, `validate`, `test-review-binding`, `test-statistical-design`, `test-validator`, `test-scorer`, `verify-artifacts`) completed `success` on the merge commit — polled until the initially-`queued` `test-validator` run finished, rather than assumed.
- Read the full `T02-07` task-contract row (build plan lines 947-970) plus `specs/CURRENT.md`'s own "Next action" section, `docs/formats/portable-export-v1.md` (full), `docs/formats/format-2-canonical-sqlite.md` (schema, pragmas, `payload_sha256`, `resulting_head_hash` chain, generic-reader example sections), and both `docs/evidence/flake-v1/T02-05/REPORT.md` and `docs/evidence/flake-v1/T02-06/REPORT.md` in full.
- Read `src/cli.rs`'s full subcommand dispatch table and `src/project.rs`/`src/capture.rs`/`src/relation.rs`'s public struct field definitions to determine the exact CLI invocation shapes and payload field names an outside tool would need — reading this source to understand the specification the format documents describe, never to reuse its parsing/serialization code (see "Independence boundary" below).

## Scope actually touched

One new, self-contained tool directory (`tools/independent-verify/`), one evidence report + raw artifacts (this directory), and a `specs/CURRENT.md` update (the stale `T02-06_MERGE_COMMIT` pointer correction plus this task's own frontier advance, per the Founder's explicit instruction to fold both into this task rather than a separate cosmetic PR). No `src/` change — this task found no defect in `T02-05`/`T02-06` requiring one (see "Independent reconstruction results" below).

```text
docs/evidence/flake-v1/T02-07/REPORT.md (new)
docs/evidence/flake-v1/T02-07/raw/ (new, 7 files)
specs/CURRENT.md                                |    9 +-
tools/independent-verify/README.md (new)
tools/independent-verify/adversarial.py (new)
tools/independent-verify/build_fixture.py (new)
tools/independent-verify/crosscheck.py (new)
tools/independent-verify/export_reader.py (new)
tools/independent-verify/run_all.py (new)
tools/independent-verify/semantic_model.py (new)
tools/independent-verify/sqlite_reader.py (new)
```

## Independence boundary — what actually makes this tool independent

This task's own stated purpose is that a product-owned export/import round trip cannot catch a bug the exporter and importer share, because both are this crate's own code testing itself. `tools/independent-verify/` is plain Python 3, standard library only (`json`, `hashlib`, `sqlite3`, `pathlib`) — a different language and runtime from the `fehrest` crate, with zero dependency on it: no `Cargo.toml` entry, no FFI, no shelling out to any Flake-internal command that itself parses the format (the only `fehrest` binary invocations are ordinary CLI commands a real user would run: `canonical-init`, `project-create`, `note-create`, `action-create`/`-start`/`-complete`, `decision-create`/`-accept`, `source-import`, `relation-create`, `fts-rebuild`/`fts-search`, `export-run`, `project-records` — never `import-*`, and never a Rust-linked call into `crate::export`/`crate::import`).

Two genuinely separate raw-artifact readers exist, per the task's own "no self-validating oracle" requirement:

1. `sqlite_reader.py` opens `canonical.sqlite` directly, read-only, via Python's built-in `sqlite3` module, and re-derives every fact from `docs/formats/format-2-canonical-sqlite.md`'s own published schema/pragma/hash-algorithm text -- never from reading `src/canonical.rs`'s implementation as anything but a specification cross-reference during design.
2. `export_reader.py` reads a published `.fehrest-export/` directory as plain JSON/filesystem, independently recomputing `export-manifest.json`'s `integrity_root` (re-derived here as: sort `members` by `path`, build the JSON object `{schema, kind, vault_id, project_id, snapshot_head_seq, members}` with `members` re-nested as `{length, path, sha256}`, serialize compactly with keys sorted -- matching the fact that the producing Rust code builds this value through `serde_json::json!` backed by a `BTreeMap`, since this crate's `serde_json` dependency does not enable `preserve_order` -- and SHA-256 it) and every member/payload digest from scratch.

Both readers produce the same intermediate shape (a list of plain-dict "revision envelopes": `object_id`, `revision_id`, `parent_revision_id`, `recorded_seq`, `kind`, `payload` as a generic parsed dict, never a typed Rust struct). `semantic_model.py` is deliberately shared between them to turn that intermediate list into semantic facts (current state per object, record counts, relation edges, action-dependency edges, source digests, project membership) -- sharing this last, purely format-agnostic step does not reintroduce a shared-omission risk, because each reader independently decides what envelopes exist to begin with, from two different raw artifacts, using two different parsing code paths. `tools/independent-verify/README.md` states this boundary explicitly, matching this task's own "document its independence boundary explicitly" requirement.

## Fixture: the complete create/capture/find/complete/export loop, via the real CLI

`build_fixture.py` drives `target/debug/fehrest` (the actual product binary) through, per disposable temp vault:

1. `canonical-init`, `project-create` x2 (Project A fully populated; Project B a single-note isolation control).
2. Capture: `note-create` (Unicode/CRLF/emoji body), `action-create` x2 (one depending on the other via `--depends-on`), `decision-create` (`--basis evidence`), `source-import` (a real local file, producing `capture.bytes_hex`/`sha256`/`byte_length`), `relation-create` (`supports`, decision to source).
3. Complete/update: `action-start` + `action-complete` on the root action, `decision-accept`, `note-update` (a second revision, proving full-history export below).
4. Find: `fts-rebuild` + `fts-search --project <A> --query Updated` -- 1 hit, the just-updated note.
5. Export: `export-run` (full store) and `export-run --project <A>` (project-scoped).
6. Derived-index absence: delete `<vault>/.fehrest/derived-fts.sqlite` (the FTS index; distinct from `canonical.sqlite`, per `src/index.rs`'s own `INDEX_DB_FILE` constant and `CanonicalStore::control_dir`), then re-run `project-records` and `export-run --project <A>` again -- both succeed identically with the index gone (`fehrest`'s own `fts-search` was separately observed, outside this fixture, to fall back to a full canonical scan when the index file is absent rather than failing, via its own `CanonicalFallback` status -- recorded here as an observation, not re-asserted as this task's own claim).

This exercises every proof-surface-4 verb (create/capture/find/complete/export) plus proof-surface-5 (derived-index removal) using the actual product, not a synthetic fixture format.

## Independent reconstruction results

All results below are from `docs/evidence/flake-v1/T02-07/raw/05-independent-verifier-run.json` (a real, captured run -- `"passed": true`, zero entries in `"failures"`).

- SQLite raw read (`sqlite_reader.py`): schema recognition (exact 4-table set, `page_size=4096`) passed; every one of 13 revisions' `payload_sha256` independently recomputed and matched; `current_object`'s pointer for every object independently confirmed to be the highest-`recorded_seq` revision; the full 13-command `resulting_head_hash` chain (`sha256("flake-canonical-tx-v1|" + previous_head_hash + "|" + revision_id + "|" + input_digest)`) recomputed from scratch and matched the stored `canonical_vault.transaction_head_hash` exactly (`head_hash_chain_verified: true`).
- Export raw read, full store (`export_reader.py`): 9 records / 13 revisions, `integrity_root` independently recomputed and matched, every member length/SHA-256 verified, every revision's `payload_sha256` verified against `payload_raw`, zero dangling references, the one `Source` object's `capture.bytes_hex` independently re-hashed and matched its own `capture.sha256`.
- Export raw read, project-scoped: 7 records / 11 revisions (Project A's `Project`/2x`Action`/`Decision`/`Note` via `list_project_records`, plus its `Source` and `Relation` -- matching `T02-05`'s documented per-kind scope union), zero dangling references.
- Cross-check, full store vs. SQLite (`crosscheck.compare_full_export`): the semantic report independently derived from `canonical.sqlite` and the one independently derived from the full-store export are exactly equal -- same object-id set, same `kind` per object, same current-revision-id, same current payload, same relations, same action-dependency edges, same source digests, same decision states. Zero problems reported.
- Cross-check, project scope vs. SQLite subset (`crosscheck.compare_project_export`): the project-scoped export's semantic report equals the project-A subset independently re-derived from the full SQLite report itself (never from a fixture-supplied "expected" list) -- zero problems, and zero Project-B object ids present.
- Cross-check, re-export after derived-index deletion: the export taken after `derived-fts.sqlite` was deleted produces an identical semantic report to the original project export (zero problems) -- canonical export does not depend on derived state.
- Cross-project isolation, raw-byte scan: every file under the project-scoped export directory was searched, as raw text, for Project B's object ids and for a planted literal marker string (`SECRET-MARKER-PROJECT-B-CONTENT-9f3a`) never referenced anywhere in Project A's data. Zero hits -- a stricter bar than a semantic/parsed check, since it would also catch a leak hiding in an unparsed or unknown JSON field.
- Find loop: `fts-search` returned exactly 1 hit for the post-update note body, through the real index.

No defect was found in `T02-05`/`T02-06`/the format documents by this task -- every one of the above independently-computed facts agreed across both raw readers on real fixture data. This is a genuine negative result for "found a bug," not a weakened check: the adversarial suite below independently confirms the verifier is actually capable of detecting corruption, so the clean result above is not merely "the tool never triggers."

## Adversarial suite -- 11/11 detected and clearly reported

`adversarial.py` copies a valid full-store export and hand-corrupts each copy with plain file/byte edits (never via `crate::export`/`crate::import`), then asserts `export_reader.read_export` either raises a specific, correctly-diagnosed error or reports a specific dangling reference -- never silently drops the bad record. All 11 passed on the captured run:

| Case | Corruption | Detected as |
|---|---|---|
| `missing_member` | delete a declared revision file | raises: `missing declared member` |
| `member_length_mismatch` | append a byte to a member file, manifest unchanged | raises: `declared length ... != actual` |
| `member_digest_mismatch` | flip one byte, same length | raises: `declared sha256 ... != actual` |
| `tampered_integrity_root` | edit `snapshot_head_seq` (a field the root covers) without updating `integrity_root` | raises: `integrity_root mismatch` |
| `duplicate_object_revision_pair` | add a second file re-declaring an existing `(object_id, revision_id)` | raises: `duplicate (object_id, revision_id)` |
| `broken_relation_endpoint` | point a `Relation.to_object_id` at an id absent from the package | reported: dangling reference, "not in this package" |
| `action_dependency_absent` | add an `Action.dependency_ids` entry absent from the package | reported: dangling reference, "depends_on" |
| `truncated_artifact` | truncate a revision file mid-JSON, manifest hash updated to match (attacker controls the whole package) | raises: `not valid JSON` |
| `missing_referenced_object` | remove every file for an object a `Relation` still points at | reported: dangling reference naming the missing object id |
| `unrecognized_manifest_schema` | change `manifest.schema` to an unknown value | raises: `unexpected manifest schema` |
| `source_bytes_digest_mismatch` | flip one hex digit of a `Source`'s `capture.bytes_hex` | raises: `capture.sha256 ... != recomputed` |

This maps onto the build-plan's own adversarial-case list as: missing exported member, missing/corrupted Source bytes, manifest/member length-and-digest mismatch, broken relation endpoint, absent Action dependency, duplicate object ID, truncated artifact, an object required by another record but wholly missing, and an unrecognized/incompatible schema version. Not applicable to this format's actual shape, recorded honestly rather than fabricated: "inconsistent project scope" as a package-internal corruption has no meaningful distinct form beyond what `broken_relation_endpoint`/`missing_referenced_object` already cover, because this export format has no separate scope-declaration field to falsify independent of the members it actually contains -- cross-project scope correctness is instead proven positively above (the project-scoped export's object set exactly equals the independently-derived expected subset, and zero foreign content/ids appear), which is the stronger and more direct proof for this specific format.

## Security boundary -- what this task actually enforces (S05/S10)

- Compares against the retained expected manifest/head, not an unanchored self-consistent prefix. The cross-checks above compare the export's independently-derived semantic report against `canonical.sqlite`'s own independently-derived report (the retained original), never merely checking that the export is internally self-consistent in isolation -- an internally-consistent-but-wrong export would still be caught, because it would disagree with the SQLite-derived report.
- Full declared state is independently readable without vendor/runtime, index, or agent (I06/I10/I12). Proven directly: the verifier never links the `fehrest` crate, never requires the derived FTS index (proven by the post-deletion re-export cross-check), and requires no network or AI/model service (plain stdlib Python, offline).
- Fail closed on incompatible/ambiguous transitions. `unrecognized_manifest_schema` proves an unrecognized `schema` value is refused outright rather than guessed at.

## Commands executed (all re-run on this branch against `main`)

| Command | Raw artifact | Result |
|---|---|---|
| `cargo build --locked --bin fehrest` | (folded into fixture runs) | Clean build |
| `cargo test --locked --all-targets` | `raw/01-full-test-run.txt` | 267/267 pass, 0 failed (234 lib + 10 integration + 23 kill_tests -- identical counts to `T02-06`'s baseline, confirming zero `src/` change) |
| `cargo fmt --all -- --check` | `raw/02-fmt-check.txt` | Exit 0, no diff |
| `cargo clippy --locked --all-targets -- -D warnings` | `raw/03-clippy.txt` | Exit 0, 0 warnings |
| `git diff --check` | `raw/04-diff-check.txt` | Exit 0, no whitespace errors |
| `python3 tools/independent-verify/run_all.py --binary target/debug/fehrest --workdir <tmp>` | `raw/05-independent-verifier-run.json` | `"passed": true`, 0 failures, 11/11 adversarial cases correctly detected (run twice for determinism; both runs agreed on every semantic fact modulo freshly-minted UUIDs/timestamps) |
| Baseline head | `raw/06-baseline-head.txt` | `dc2f1c1...` on both `HEAD` and `origin/main` before this task's own commit |
| Environment capture | `raw/07-environment.txt` | Matches `reference-hardware.md`'s frozen development profile |

Raw artifact manifest: `raw/00-manifest.txt` (generated before its own content existed, so does not hash itself -- same pattern every prior `flake-v1` evidence report in this phase already uses).

## Failed attempts / exclusions

- An early draft of the fixture's `fts-search` assertion queried for "Unicode" (a word only present in the note's original body), but `fts-rebuild` runs after `note-update` replaces that body with "Updated body after fixture edit" -- the query correctly found 0 hits against the current indexed state, which is exactly correct FTS behavior, not a bug. Fixed by searching for "Updated" (present in the post-update body) instead of weakening or removing the assertion.
- No adversarial case, defect, or check was skipped, deleted, or weakened to reach a passing result. No product (`src/`) defect was found or needed fixing by this task.

## Performance gate

No dedicated timing harness was run. The independent verifier's own cost is proportional to the fixture's revision count (one `sqlite3` query pass, one JSON/hash pass per export member) -- negligible at the S-scale fixture used here (13 revisions). Consistent with every prior `flake-v1` task, M/L-scale independent-reader timing is deferred to `T05-02`'s hardware-qualified pass; this task's own performance-gate clause ("report independent-reader time and memory; product M export/import hard gates still apply") is therefore only partially exercised here -- recorded as unmeasured at scale, not silently assumed adequate.

## Durability gate

The derived-index-removal proof (`prove_derived_index_removal`) directly demonstrates the durability-relevant claim this task's own gate names: canonical state and its export/reconstruction path survive complete deletion of derived state (`derived-fts.sqlite`), with the original canonical bytes (`canonical.sqlite`) never touched by this task's tooling. No new fault-injection point was added by this task, since it introduces no new canonical mutation path -- it is read-only against the canonical store (only `fehrest`'s own already-audited CLI mutates anything, in the ordinary course of building the fixture).

## Cross-platform gate

Native development profile only (Windows 11/NTFS), consistent with every prior `flake-v1` task; `T05-05` repeats independent-reader cross-platform verification per the build-plan row's own cross-platform-gate clause. Nothing in the independent verifier is platform-conditional: it uses only `pathlib`/`sqlite3`/`json`/`hashlib` from the Python standard library, none of which branch on OS beyond ordinary path separator handling `pathlib` already normalizes.

## Acceptance criteria disposition

| Plan acceptance clause | Status |
|---|---|
| Independent logical and byte comparison is exact on all full fixtures | Satisfied -- `crosscheck_full`/`crosscheck_project`/`crosscheck_reexport_after_index_removed` all report zero problems |
| Intentional selected-scope omissions are explicit | Satisfied -- the project-scoped export's object set is proven to equal exactly the independently-derived expected subset, no more and no less; the raw-byte scan proves zero foreign content leaks |
| CLI loop succeeds offline | Satisfied -- every fixture command is a local `fehrest` CLI invocation against a local vault; the independent verifier itself requires no network |
| Independent executor/tool reconstructs from exported JSON/bytes and separately inspects canonical SQLite without linking Core or its importer | Satisfied -- see "Independence boundary" above; neither reader imports/links/shells into `crate::export`/`crate::import` |
| Recompute history/current state/relations, verify artifact/receipt bytes and unknown fields | Satisfied -- full revision-chain replay, current-state derivation, relation/dependency extraction, member and payload digest re-verification, plus the `resulting_head_hash` chain recomputation |
| Run the whole CLI user loop with derived DB removed | Satisfied -- `prove_derived_index_removal`, cross-checked identical to the pre-removal export |
| Document how owners recover without Flake | Satisfied -- `tools/independent-verify/README.md` plus this report describe exactly how an owner/tool reconstructs state from `canonical.sqlite` or an export package using only public documentation and standard tools |
| Record independent executor/tool source, inputs and commands; compare canonical digests and complete counts | Satisfied -- this report plus `raw/05-independent-verifier-run.json` |

## Completion condition

Every acceptance clause above is satisfied. Shared contract gates pass, independently re-run on this branch: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked --all-targets` all green (267/267, identical counts to `T02-06`'s own baseline -- zero `src/` regression risk since this task made no `src/` change). The independent verifier genuinely cross-validated two separately-implemented raw readers of two different Flake-produced artifacts against each other and found them in exact agreement, and separately proved it can detect 11 distinct classes of corruption/omission it did not find in the genuine data. No sealed evidence altered; no force-push; no historical evidence file touched; the stale `T02-06_MERGE_COMMIT` pointer is corrected in this same task's commit per the Founder's explicit instruction. `T02-07` is complete, closing `P02`.

## Next frontier

`T03-01` -- Track selected source revisions and relocation, first task of `P03`. Depends on `T02-07` (this task).
