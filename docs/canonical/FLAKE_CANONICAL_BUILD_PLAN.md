# Flake canonical build plan (project renamed to Pluma, 2026-09-20)

Date: 2026-09-13. Owner: Astro, final planning authority delegated by the founder. Audience: Muse, future implementation agent. Version: 1.0. This is a specification and execution contract; it is not evidence that the specified product exists.

## 1. Document status and authority

**ADOPTED FOR PLANNING. This is the sole implementation roadmap.** The founder's final-planning assignment authorizes the decisions and governance reconciliation recorded here. The assignment explicitly prohibits implementation. [CURRENT](../../specs/CURRENT.md) therefore records planning completion with no active implementation unit. A subsequent instruction to Muse to implement this plan activates T00-01; it need not ask the founder to select a roadmap, storage engine, editor, license, or routine next task again. The instruction's actual scope still controls.

Authority order: current explicit founder instruction and verified repository state; CURRENT for the active unit; this plan and its [architecture decision](FLAKE_V1_ARCHITECTURE_DECISION.md); retained historical invariants as mapped here; the active unit's conforming Spec Kit; historical proposals as evidence only. The [corpus disposition annex](FLAKE_PLANNING_CORPUS_DISPOSITIONS.md) is an incorporated inventory, not a second roadmap. The [security review](../reviews/FLAKE_FINAL_PLANNING_SECURITY_REVIEW.md) records review evidence; section 20 owns the product security contract.

The freeze applies to the **Flake v1 continuity product**, with a new **vault format 2**. Product version and format version are different. Historical R1 and Phase T remain immutable. No benchmark is rerun, rescored, reinterpreted as proven product advantage, or silently renamed. New evidence lives in new namespaces. `ASTRO_PLAN_COMPLETE=YES` means planning obligations are closed. `PROJECT_COMPLETE` remains NO until section 37 is satisfied by implementation evidence.

## 2. Live-truth snapshot

The snapshot was independently refreshed on 2026-09-13 before authoring, using local Git, fetched origin, GitHub API metadata, exact Git blobs, source inspection, CI definitions and Cargo metadata. It is a baseline, not permission to ignore later changes.

| Item | Verified value |
|---|---|
| Repository / root | `TheHalfMoon/Flake`; `C:/Users/Shehr/OneDrive/Documents/ChatGPT/Flake` |
| Branch / initial worktree | `codex/flake-product-review`; clean tracked and untracked status, ignored build/review scratch excluded |
| Planning baseline HEAD | `4246f6dd62ce21797f07a2ff169a4c1ba8249419` |
| Parent / tree | `b03b7a8a1507cccef9f7230e17b1c2ff9f08ae84` / `6f0da92c55afc04a21652394923edc7d25d5e45e` |
| Default branch / fetched origin/main | `main`; `b03b7a8a1507cccef9f7230e17b1c2ff9f08ae84` |
| Remote tree / parent | `8f936e9ff37c3a597f7778350a0a9b76b55b4729` / `7c23c9de0d03faf51ded853d425d5513e6dec362` |
| Remotes / local branches | origin fetch and push `https://github.com/TheHalfMoon/Flake.git`; one local branch |
| Local-only history | Exactly the preserved review commit at intake |
| Draft PR 2 | OPEN, `a99413db9e6540fef967ac9c88549db66fc13c99`; broad V2 proposal, not adopted |
| Draft PR 40 | OPEN, `e16e521db509e9d0c0023d25ceb689d780d4e0f9`; donor/deep-review proposal, not adopted |
| Tags / releases / open issues | None returned at intake; task authority is in repository documents |
| Relevant merged work | PR 56 R1 terminal, 57 Spec 002 activation, 58-63 its implementation/closeout and evidence; historical review-policy change preserved separately |
| Remote CI | Artifact verification run `34307022808` and R1 validation `34307022809` successful; neither proves native product durability or desktop behavior |
| Product source | Experimental Rust `fehrest` 0.0.1-phase-t, 12 modules; 10 CLI verbs; no desktop, gateway, sync, model runtime or release packaging |
| License | Cargo declares Apache-2.0; root LICENSE absent, GitHub licenseInfo null. Founder explicitly selected Apache-2.0 during this assignment |

The old remote CURRENT and navigation roadmap contain stale frontier statements alongside later closeout. They are preserved as prior states; the new local CURRENT and this plan replace their operational authority. The follow-up planning commit must descend from 4246f6d, not rewrite it. Its own SHA is reported by the closeout and obtained with Git; no document contains a fabricated self-referential commit SHA.

## 3. Planning corpus inspected

The exact-source inventory contains 179 Markdown documents: 87 at the planning baseline, 24 changed proposal documents at PR 2, eight at PR 40, and 60 historical documents absent from the working tree but readable from the reconciled historical Git object. Of these, 156 are material planning/governance/evidence documents. Eight V0 corpus files are experiment inputs, and 15 historical skill/template files are tooling rather than independent plans. All are individually recorded in the annex with revision and SHA-256. Existing review evidence archives are preserved as an additional evidence set, not classified as new competing plans.

Inspection combined full reads of load-bearing decisions/contracts with targeted sections, inventories and source checks. It is not a claim that every archive line was independently scientifically validated. Historical missing files were read from `ed79d8ecee08e4ce4dd384edaffc4a27cfd6d37c`, not reconstructed from memory. PR text was read at exact heads, not assumed merged. Spec 003 exists as a named future plan, not a checked-in active Spec Kit. Main and PR 2 use conflicting later numeric spec identities; the namespace mapping in section 30 resolves that collision.

Source and operational inspection also covered Cargo manifests/lockfile, all production module boundaries, critical writer/recovery/compiler paths, tests, benchmark harnesses, artifact manifests, recovery tooling and both GitHub workflows. No runtime dependency, external source code or model was installed for this planning assignment.

## 4. Preservation of 4246f6d

The commit exists, has the exact parent/tree above, and contains the completed 15-report review, its retained evidence and operational documentation reconciliation. It was local-only at intake. It remains a valid historical review with explicit limits. Preserve its entire tree and all evidence bytes. The plan changes conclusions prospectively through an additive follow-up commit. Do not amend, reset, squash, rebase away or replace this review. Source findings do not retroactively change the prior acceptance records.

Historical sealed R1 v1.1 remains commit `ed79d8ecee08e4ce4dd384edaffc4a27cfd6d37c`, tree `f7ea7e0f57019c8061a4019ac614730f68750f19`; the historical bundle SHA-256 remains `a36639da9731cd4778777e14b980ca04784f9a00890a57d0a3fc10591f54f5f9`, as recorded in the preserved recovery manifest. Never substitute a bootstrap SHA. R1 v3 terminal wording remains `THESIS_SUPPORTED_ON_COST_CAVEAT`, with the review's qualification attached to product inference.

## 5. Fifteen-report preservation map

All paths in this table are under `docs/reviews/`, are retained byte-for-byte, and are not direct execution instructions. The single disposition here matches the annex.

| Report | Primary disposition | Retained conclusion | Superseded or narrowed conclusion / destination |
|---|---|---|---|
| FLAKE_EXECUTIVE_PRODUCT_REVIEW.md | AMEND | Continuity hypothesis and foundation-first sequence | Proposed direction becomes bounded adopted contract, sections 8-13 |
| FLAKE_CURRENT_STATE_TRUTH.md | KEEP | Source/evidence findings at its exact baseline | New work must refresh facts, not rewrite this record, sections 2/28 |
| FLAKE_COMPETITIVE_LANDSCAPE.md | KEEP | Documented comparison, no measured superiority | Broad team/topic opportunity is outside v1, sections 9/10 |
| FLAKE_EXTERNAL_SOURCE_QUALIFICATION.md | KEEP | Study/reuse distinction and donor provenance | New narrow source decisions in section 24 do not authorize all donors |
| FLAKE_GAP_MATRIX.md | MERGE | Source-backed failures and missing product proof | Every applicable gap assigned to tasks and failure routes |
| FLAKE_PRODUCT_THESIS.md | AMEND | Shared evidence-linked decisions/actions across replacements | Solo owner and offline interchange; no team platform, section 8 |
| FLAKE_PRODUCT_PRINCIPLES.md | KEEP | Ownership, bounded authority, negative evidence, simplicity | Applied as section 13 invariants |
| FLAKE_INFORMATION_ARCHITECTURE.md | SPLIT | Project context, familiar objects, low setup cost | Four user types retained; topics/inbox platform deferred, sections 12/15 |
| FLAKE_AGENT_NATIVE_ARCHITECTURE.md | SPLIT | Core admission, provenance, grants and receipts | File/CLI proposal exchange now; broker/server/provider runtime deferred |
| FLAKE_DATA_OWNERSHIP_AND_LOCAL_FIRST_REVIEW.md | AMEND | Complete owned history and independently usable exit | Canonical SQLite format 2 explicitly selected, sections 15/21 |
| FLAKE_SECURITY_AND_TRUST_REVIEW.md | MERGE | Honest OS-user root, no authority from content | One product security contract, section 20 |
| FLAKE_RENAME_MIGRATION_PLAN.md | AMEND | Preserve old evidence, explicit user-visible rename | New product command/package naming at T05-03; `.fehrest` compatibility guard retained |
| FLAKE_ROADMAP_RECONVERGENCE.md | SUPERSEDE | U1 save, U2 portable work, U3 resumption | One 38-task graph replaces proposed sequence, sections 31-35 |
| FLAKE_NORTH_STAR_METRICS.md | AMEND | Successful continuation and total maintenance cost | Fixed bounded engineering trials, section 26; no decorative multipliers |
| FLAKE_30_60_90_EXECUTION_PLAN.md | SUPERSEDE | Stop conditions and staged value | Dependency/evidence gates replace calendar promises |

## 6. Final challenge summary

Flake has not established demand or superiority. Notes plus a maintained project index are a strong simpler substitute. The defensible experiment is whether explicit current decisions, changed evidence and next actions reduce recurring restart work enough to repay capture and maintenance cost. Local operation is useful because the complete loop and exit remain available without a vendor, account, network or model; it does not itself prove demand.

The final challenge rejected four premature commitments: a custom multi-file canonical transaction protocol, a mandatory graph/model platform, a rich collaborative editor, and an execution broker. It also rejected an unqualified assertion that hashes authenticate provenance or that unit tests prove power-loss survival. The resulting design uses an established transactional engine, small typed work records, deterministic resolution and user-reviewed package interchange. Rust remains justified by a single existing correctness boundary shared by CLI and desktop; React cannot own an independent data model.

These are prospective Class C/D and product-scope Class E decisions under delegated final planning authority, recorded in [ADR-0017](FLAKE_V1_ARCHITECTURE_DECISION.md). The evidence does **not** show that graph capabilities never help, that the old editor candidates are unmaintained, or that SQLite alone guarantees durability. The new scope removes those dependencies without making those empirical claims.

## 7. Remaining-gap closure summary

| Gap | Frozen resolution / owner |
|---|---|
| Startup/read mutation before ownership; raw journal mutators | Genuine nonmutating reader; OS-held writer ownership and private store writes, T01-01..03 |
| Separate object/log commit, ignored sync errors, destructive repair | One canonical transaction; ambiguous outcome protocol; forensic recovery, T01-02..04 |
| Canonical portability versus new database | Published schema, complete revision bytes, verified full export and independent reader, T02-05..07 |
| Existing Spec 002 accepted despite source concerns | Preserve old closeout, activate a bounded corrective addendum, T00-02 and P01 |
| Duplicate identity, stale/partial index and expensive full scans | Canonical constraints, transaction-sequence checkpoint and replaceable FTS generation, T02-04 |
| Source drift, conflict, override and temporal semantics | Immutable source snapshots and explicit unresolved/override states, T03-01..03 |
| Ordinary records bypass scope; latest manifest overwrite | One canonical eligibility path for all record types; immutable disclosure receipts, T03-04 |
| Agent authority and repeat delivery | Owner-issued export only; proposal ingestion and expected-revision acceptance, T03-04..06 |
| R1 raw evidence missing, harness/fallback/budget concerns | Preserve uncertainty; fresh separately registered local proof, T03-07..08; no historical rerun |
| Product demand and no-model proof | Strong manual baseline, cold successor, all maintenance counted; desktop adoption gate, T05-06 |
| Native durability / distribution / license | Three named profiles, fault and recovery records, verified offline release candidates, P05 |

No unresolved product-choice question remains. Runtime performance, user benefit, dependency admission, native behavior and release credentials are **bounded execution gates**, not assumed successes. A failed gate can stop implementation; a complete plan cannot promise a successful product.

## 8. Final product thesis

Flake is a local work-continuity application for one person to capture project work, preserve the evidence behind decisions, and resume after an interruption with visible changes and next actions. A replaceable external agent can receive a bounded evidence package and return reviewable proposals. The same project remains understandable and usable when that agent disappears.

The v1 advantage to test is **less total effort to resume correctly than well-maintained Markdown notes, an index and a task list**. Flake does not claim better general intelligence, automatic truth, universal memory, or parity with work-management suites. Failure to clear the defined value gate stops expansion and invokes the scope reconsideration rule in section 39.

## 9. Users and jobs-to-be-done

Primary user: an individual software developer or technical project owner, including a person working inside a team whose shared systems remain elsewhere. One local OS account owns each vault; other people are sources or external collaborators, not authenticated tenants. The recurring scenario is returning after one day to several weeks, or handing work to another agent, when notes, source revisions and previous conclusions disagree.

Jobs: capture a thought without schema setup; identify the next action; explain why a decision was accepted; notice changed or missing evidence; correct a previous conclusion without erasing it; resume with a fresh reader; leave with complete owned data. A task manager organizes work, a note tool captures prose, IDE history records code and an agent store preserves its own state. Flake must prove useful continuity across those boundaries rather than replace those tools. This differentiation is a hypothesis, not an exclusive capability claim.

## 10. Product boundaries and non-goals

Complete scope: local vault/project creation, four user-facing record types, explicit source snapshots, local literal/ranked search, intentional links/dependencies, decisions and action transitions, deterministic resumption, portable full export/import/backup/restore, scoped agent package/proposal exchange, CLI and minimal desktop, three qualified native release profiles, complete documentation and license compliance.

No network listener, account, hosted service, paid model, required local model, chat UI, agent execution, provider credentials, global graph, vector index, automatic memory, plugin host, marketplace, organization tenancy, sync, collaborative editing, mobile, calendar, messenger, PR/CI replacement or general workflow language is in the completion boundary. A manually imported repository file and supplied commit reference are sufficient repository integration for v1. Flake does not execute Git, hooks, scripts, build tools, extensions or linked applications. Source URL/commit metadata may be copied and displayed as text; Flake does not fetch it.

## 11. Irreducible Flake experience

First launch offers **Create local vault**, **Open vault**, and **Restore/import**. Creation explains local ownership, location and backup responsibility, then asks only for a project name. The first project shows one capture field with type choices Note, Action, Decision; Note is the default. A decision can be recorded without evidence only through an explicit “Record my judgment” action that leaves evidence status Unlinked. An action needs no artificial evidence requirement.

The value moment is leaving one action and a decision linked to a file snapshot, closing Flake, changing or removing the external file, and returning to a Resume view that still explains the prior choice, identifies what was checked and what changed, and shows the next action. Repeated use is capture → inspect changes → choose/update an action → save → later resume. No streaks, notifications, mandatory analytics or automated importance scores drive retention.

## 12. Canonical UX model

Navigation has Projects and Search. A project has **Resume**, **Work**, **Notes**, **Decisions**, **Evidence**, and **History**. These are filtered views of the same records. No separate session entity or conversation product is created. A checkpoint is an explicit “Mark reviewed through here” sequence marker; opening a page never implies the user read it.

| Concept / state | User-visible meaning and behavior |
|---|---|
| Project | Named boundary for one endeavor; active or archived; durable ID independent of folder/path |
| Action | Work item with Open, Doing, Blocked, Done or Cancelled state; completion has a summary, optional evidence, timestamp and actor |
| Note | Editable Markdown text, immutable saved revisions; links reference IDs |
| Decision | A stated choice with rationale, evidence and optional effective interval; Draft, Accepted, Superseded or Withdrawn; agent output never self-accepts |
| Evidence | An exact imported snapshot with source label, content digest, observation time and provenance; no instruction authority |
| Resume | Current accepted decisions, next actions, relevant notes, and changes since the user's explicit checkpoint; conflicts and stale evidence precede reassuring summaries |
| Trust | Separate labels for Source provenance, User acceptance, and Evidence freshness. No one-dimensional confidence percentage |
| Current / changed | “Matches when checked at …” versus “Changed since saved revision”; never “current” without a check or a defined snapshot/as-of label |
| Missing / unchecked | “Source unavailable; saved copy retained” / “Not checked”; absent network or file is not negative evidence |
| Negative evidence | An explicit user assertion or failed verification linked to its source/observation; never silently omitted from a summary |
| Conflict | Incompatible accepted decisions with the same decision key and overlapping time remain visible together; no latest-wins resolution |
| Override | User sees competing evidence, enters a reason and accepts an explicit superseding revision/decision. Evidence remains; the UI labels the override |
| Capture | Plain text area; Ctrl/Cmd+Enter saves, Ctrl/Cmd+S saves editor; no success label before durable acknowledgement; unsaved buffer remains on errors |
| Search | Project/all-vault selector, literal terms by default, optional title/type/state filters; snippets with revision/as-of state; index lag is visible |
| Complete / reopen | Explicit state transition; blocked dependencies require a reasoned override before Done; reopen preserves previous completion |
| Export | Destination, full-backup versus selected-project package, inclusion summary, privacy warning and verified result; partial output never called complete |
| Recovery | Read-only error with reason, preserved evidence location, verified backup choices and explicit recover-to-new-location action |

Empty states have a single relevant action: create first project, capture first note, add next action, select a source, or clear search filters. Archived/deleted items remain inspectable through History; delete means tombstone and requires confirmation naming the item, while project archive is reversible. There is no purge UI in v1. No color-only trust/state signals. Keyboard focus, screen-reader names, IME composition, Unicode and 200% zoom are required; destructive controls are not default-focused. Offline is normal and creates no blocking banner. Import/repair/proposal progress can be cancelled before commit; cancellation after commit reports the committed result.

Markdown preview disables embedded HTML, scripts, remote resources and active links. It renders text and safe formatting only. External navigation is copy-address only in v1. Attachments are downloadable/exportable bytes with safe names and MIME labels; Flake does not execute or render active attachment formats. The editor is a native HTML textarea with a separate safe preview; rich blocks, CRDT and editor extensions are deferred. These limitations are visible product scope, not hidden fidelity loss.

## 13. Canonical architecture and decision freeze

The irreducible invariant set, named **I01-I12**, applies to every task:

1. **I01 Ownership:** one local owner; all core workflows offline, account-free and model-free.
2. **I02 Authority:** Rust Core alone admits canonical changes; content, index rows and model assertions never mint authority.
3. **I03 Identity:** UUIDv7 record identity is opaque and stable; vault identity plus record ID forms the external identity; paths never identify a record.
4. **I04 Atomicity:** one acknowledged command has one committed transaction containing its full changes, history and idempotent result; never a half-state.
5. **I05 History:** accepted revisions, decisions, provenance, receipts and tombstones are immutable; no silent overwrites or purge.
6. **I06 Derivation:** indexes, rankings, snippets and computed conflicts are disposable; canonical reads decide identity, scope, revision and lifecycle.
7. **I07 Time:** recorded sequence and valid time are separate; partial/unknown source times retain their precision; no manufactured dates or clock-based last-writer-wins.
8. **I08 Disclosure:** every agent-visible item passes canonical eligibility and retains its full machine-owned envelope within the complete byte budget.
9. **I09 Recovery:** corruption is visible; original bytes and verified backups survive recovery attempts; absence of data is never repaired by invention.
10. **I10 Portability:** full owned state can be interpreted and reconstructed with published formats and generic tooling independent of Flake and derived state.
11. **I11 Bounds:** every input, parser, query, operation and output has an enforced resource ceiling and explicit overflow outcome.
12. **I12 Claims:** hashes are integrity evidence, not human identity or adversarial authentication; measured platform/benchmark limits accompany claims.

**Implement directly:** embedded SQLite canonical format 2, separate FTS5, rollback journal with EXTRA synchronization, one OS-held writer, full revision history, explicit imported snapshots, four user types, deterministic temporal resolution, offline proposal interchange, plain Markdown textarea, Tauri 2/React presentation, Apache-2.0 intended license, three profiles and manual updates.

**Measure within bounds:** SQLite cache 8/16/32 MiB and page size 4096/8192 before first format-2 release; select the smallest configuration meeting section 27 (tie: 4096 and 8 MiB). Search batch size 100/500/1000; choose smallest meeting the same gate. These values may change only as nonsemantic tuning with evidence. Exact maintained patch versions, compiler patch and OS image digests are pinned at admission, not guessed today. UI spacing, icon choice, text wrapping and component factoring may change within the frozen UX/accessibility contract. No architecture bake-off is delegated.

**New ADR and appropriate C/D/E review required before work:** another canonical engine/layout, WAL, external canonical blob store, reducing sync, direct external file writeback, canonical compaction/purge, encryption/key management, scope/identity semantics, sync/multi-user, execution/network/provider runtime, rich editor, a fourth platform, weakening final gates or replacing the thesis. Stop the affected unit and record the concrete conflict; a failed benchmark is not permission to change its threshold.

**Deferred:** all non-goals in sections 10/38. A future dependency proof requires a named failed retained requirement, simpler alternatives and cost/security evidence. It does not authorize immediate implementation.

## 14. Component and process boundaries

```text
Owner CLI or local desktop UI
             |
      typed bounded commands
             v
 Rust Core: admission / scope / state / transactions / recovery
       | canonical snapshot        | derived candidate IDs
       v                           v
 canonical.sqlite              derived.sqlite (FTS5)
       |
 owner-reviewed export -> immutable receipt -> explicit package file
                                                   |
                                         replaceable external client
                                                   |
                                  untrusted proposal file -> Core review
```

One process owns a writer at a time. CLI and desktop use the same Rust command implementation; no daemon or hidden background server. A second writer returns Busy with owner diagnostics. The desktop may stay open as writer; CLI reads remain bounded read snapshots, and CLI writes must wait for closure rather than bypass ownership. There is no IPC writer-bypass feature. Long read snapshots can block rollback-journal commit; readers have a 2-second normal deadline, paginated history/export use bounded batches or a dedicated snapshot, and writers return Busy after 2 seconds without partial success.

Coordination uses two stable OS-held locks: an exclusive writer-ownership lock for normal writer/recovery processes, and an access lock held shared by every normal open connection and exclusively by recovery/snapshot-preservation. Acquire writer ownership before requesting exclusive recovery access; all processes use this order. Recovery waits for normal connections to close or returns Busy. A new reader must acquire shared access before opening SQLite, so it cannot start during preservation. Lock files are created only during explicit vault initialization and never removed to steal ownership; missing coordination metadata requires owner-directed inspection, not readonly repair. These locks coordinate cooperating processes, not malicious same-user filesystem access.

Core modules may be split for maintenance without new process boundaries. UI carries text, typed IDs and bounded input but never SQL, filesystem paths for arbitrary operations or authority assertions. Owner-selected path access is mediated by native dialogs/CLI arguments in trusted Core. The agent parser is an untrusted-data parser, not a shell. Indexing runs as a bounded Core operation and reads an immutable canonical snapshot. It cannot mutate canonical content. Import/export/backup are Core-owned operations with explicit destination authority and staging rules.

## 15. Canonical data model

This section specifies logical records; Muse may choose private SQL names/indexes without changing semantics. Public format documentation must expose all tables/columns and reconstruction rules. No application schema file is created by this planning assignment.

Common canonical envelope: vault_id, object_id (UUIDv7), kind, revision_id (UUIDv7), parent_revision_id or null, recorded_seq (transaction-monotonic integer), recorded_at (actual UTC timestamp with precision), actor (owner or declared external principal), origin (user, import, agent-proposal, migration, system), schema_version, lifecycle, exact UTF-8 payload bytes and SHA-256. Timestamps never order conflicting writes. Unknown fields/raw payload bytes are retained; unsupported required capabilities cause refusal. All references include vault ID and object/revision ID where historical meaning is intended. User-facing names are labels, not identifiers.

Wire determinism: versioned machine-owned JSON envelopes use RFC 8785 JCS for digest input, rejecting duplicate keys, invalid Unicode and non-finite/unsupported numeric representations. Transaction sequences/counters are bounded to signed 64-bit nonnegative storage values and encoded as decimal strings on the public JSON/UI boundary, avoiding JavaScript precision loss. UUIDs use lowercase canonical spelling; binary payload references use lowercase SHA-256 hex. Original user/source bytes are hashed directly and stored separately from canonicalized metadata; JCS never normalizes their text. Normalized command digest covers version, command ID, target scope, expected revisions and ordered requested operations, excluding transport formatting and later response fields. Publish cross-language golden vectors before first accepted write.

Hash boundaries avoid recursion: transaction hash covers a domain/version tag, previous transaction hash and canonical mutation/result data excluding its own hash and the result's resulting-head-hash field; fill those derived hash fields after calculation. A receipt stores exact package bytes and their digest outside those bytes. A manifest integrity root hashes the sorted member path/length/digest list and declared snapshot/scope/version, excluding the root field itself. These are consistency checks against expected artifacts, not signatures or authority.

| Entity | Required content / ownership / lifecycle / invariants |
|---|---|
| Vault | Immutable ID, format epoch, creation metadata; current schema/min-reader capabilities and transaction head. Owner-owned; no tenant/member table. Guard metadata must agree with the DB or open fails |
| Project | Name, optional description, active/archived; owner-owned. Exactly one project for each work record; moving between projects is an explicit revision, never a path rename |
| Note | Project, title, Markdown body; saved revisions immutable. Active/tombstoned. No block IDs or independently mutable capture object |
| Action | Project, title/body, state, ordered explicit dependency IDs, optional completion summary/evidence. Open→Doing/Blocked/Done/Cancelled; any reopening requires an event. Dependency cycles rejected; no autonomous transitions |
| Decision | Project, stable user-supplied decision key, statement, rationale, basis, verification, lifecycle and valid interval. Basis = evidence/user-judgment/agent-proposal; verification = unreviewed/user-reviewed; lifecycle = draft/accepted/superseded/withdrawn/tombstoned. Acceptance owner-only; overrides require reason and explicit targets |
| Source | Project, label, kind (file/manual-reference), owner-selected locator hint, optional claimed repository/commit/path metadata; active/unavailable. Locator is local-only metadata, never export or read authority |
| Source revision | Source ID, exact captured bytes or explicitly reference-only, digest/length, observed_at, claimed source time plus precision, origin and rights note when known. Immutable. A URI or commit label alone is not verified content; missing bytes label evidence incomplete |
| Artifact | Immutable opaque bytes, digest, safe display filename, MIME label, source/proposal reference and user admission. Max 64 MiB per artifact; all canonical bytes reside in the same transactional DB, never an untracked external blob |
| Relation | Project, typed endpoints, relation type (supports, contradicts, depends-on, relates-to, supersedes), endpoint revision policy. Intentional links are canonical. A supersession must be owner-accepted, acyclic and within project; links do not imply current truth |
| Source check | Source/revision, observed digest or Missing/Denied/Unchecked, time, checking actor and exact algorithm/version. Append-only observation, not a rewritten revision. Check result says what was observed, not that the source remains unchanged forever |
| Review checkpoint | Project, owner, reviewed-through transaction sequence. Explicit user transition, monotonically advances unless user explicitly resets with reason; not an automatic session record |
| Export grant | Owner-issued ID, project, allowed record types/IDs, expiry, byte budget, privacy exclusions, active/revoked state and policy version. Local canonical state only; no authority survives export/import |
| Disclosure receipt | Immutable request ID, grant ID/version, canonical snapshot head, as-of times, selected/rejected revision IDs and reason codes, compiler/policy version, exact emitted package bytes and digest, byte count, principal label and creation time. Persists before emission; no self-hashing recursion |
| Agent proposal | Immutable inbound bytes/digest, package receipt reference, declared agent/model/tool identities (unknown permitted and labeled), proposed bounded operations, expected target revisions and rationale/evidence. Pending/accepted/rejected/expired status is a new revision; never an accepted decision by ingestion alone |
| Transaction | Command UUID, normalized input digest, actor/operation, previous head/hash, ordered full mutations including payload bytes or immutable blob digests, result and resulting head. Command result and all rows commit together. Operation-specific permission checked before transaction |
| Export/backup manifest | Version, kind (full-backup/project-package), vault snapshot head, member paths/lengths/digests, record/revision counts, omissions with reasons, capability versions and integrity root. Stored with the exported set; its creation may have a local receipt, but the manifest does not hash itself |

Capture is an operation producing a Note/Action/Decision, not another durable type. Evidence is a source revision/artifact plus a relation, not a second copy. Completion is an Action transition; a run/session is not required. Trust assertion fields live on Decision/provenance envelopes. Conflict is computed from canonical assertions and explicit supersession; it cannot be the only record of an unresolved disagreement.

Semantic exactness: valid intervals are half-open `[from,to)` in UTC, with null endpoints representing unbounded intervals only when explicitly stated by the user. Unknown/partial source time is separate metadata and never coerced into an unbounded decision assertion. Recorded cutoffs are inclusive transaction sequence numbers. Decision keys are required explicit identifiers within a project: Unicode NFC, case-sensitive, leading/trailing whitespace removed with a preview of the stored key, nonempty, at most 128 UTF-8 bytes and no control characters. Existing keys are selectable to place competing decisions in the same question; changing a key is an explicit revision. Display text/body bytes are never normalized silently. User-created relations stay within one project; cross-project references in v1 are descriptive labels without automatic traversal. Missing evidence bytes or tombstoned targets remain visible unresolved references, not cascade deletion. A moved work record must revalidate/rebind its project-scoped relations and grants in the same transaction or reject the move.

Deletion is a new tombstone revision retaining old content and references. A project archive hides default lists, not history. Revocation prevents future disclosures but cannot recall prior packages. There is no privacy erasure claim; permanent purge requires a separate retention/backup ADR. Re-import of identical IDs/revisions is idempotent; same identity with different immutable bytes is a conflict, never last-writer-wins.

Restore/fork authority is explicit. A full restore validates and retains the original immutable history prefix, then appends a local restore transaction that advances the grant-issuance epoch; every pre-restore grant becomes unusable without changing its historical bytes. New grants bind the current epoch. A fork/import into a different vault creates new admitted record IDs and a complete old-vault/old-ID→new-ID origin mapping; it rewrites only the newly admitted references and preserves original package members/receipt bytes as inert imported evidence. It does not splice a foreign chain into the local transaction chain or claim the two histories are identical. A selected-project merge follows this new-admission rule, not full-restore identity semantics. These modes must have separate previews and acceptance tests.

## 16. Canonical versus derived state and reconstruction

Canonical format 2 consists of the immutable compatibility guard plus the SQLite database and, while a transaction is in progress, its SQLite-managed rollback journal. The guard `.fehrest/vault.json` announces format_version=2 before a new vault is published; an old format-1 binary must refuse it. New initialization occurs in a new sibling staging directory and becomes selectable only after verification and durable publication. Never overwrite an existing path. The guard cannot be recreated by a read path when missing.

`canonical.sqlite` contains all state in section 15, including exact original payload/attachment/receipt bytes. Tables for current revisions and transaction head are canonical transactionally maintained summaries, cross-checked against immutable revisions and mutation history. Only Core writes them; a discrepancy blocks mutation. `.fehrest/derived.sqlite` contains FTS and projection checkpoints only. Removing it with all relevant connections closed loses no canonical data. It is never included as authoritative backup material.

A full portable export has a versioned manifest, immutable record/revision JSONL, transaction JSONL, content-addressed byte files, receipt bytes and a plain Markdown project rendering. JSONL is a portable export, not the live transaction engine. Unknown raw payload bytes are separately retained where parsing would normalize them. Member paths are generated solely from validated IDs/digests; user labels do not become paths. A selected-project package explicitly lists scope exclusions and is not a full vault backup.

The portable container is a directory tree in v1, not an arbitrary archive extractor. A human may transfer that tree using another tool. Import accepts only enumerated manifest members, rejects unexpected members/symlinks/devices and validates actual opened bytes. Full private backups retain local locator/grant history as inert records; shareable project packages omit local-only locators, active grants and excluded sensitive fields, with explicit omission markers. A shareable package is therefore a complete export of its declared permitted scope, not a lossless backup of omitted private state. Receipt history included in a project package must itself pass disclosure scope checks; do not include another project's old package bytes through a receipt reference.

Independent reconstruction uses generic SQLite or JSON/UTF-8 readers, SHA-256 and the published schema: validate member lengths/digests; load immutable records and full mutations in recorded order; reproduce current pointers, lifecycle and relation state; verify the declared head and each blob; regenerate derived search/conflicts; compare canonical logical digests and original bytes. No Flake binary, agent, network, vendor service or old cached index is needed. A backup is complete only if all canonical members required by its declared scope verify. A consistent prefix is not proof of completeness without an independently retained expected head/manifest.

Fidelity comparisons distinguish the exported snapshot from deliberate post-restore events. All original snapshot payloads/history/receipts must match exactly at its declared head. The only permitted added restore state is the recorded local restore transaction and grant-epoch invalidation; a fork instead has its explicit complete origin map and new-admission transactions. Verify those deltas independently. Do not demand byte-identical SQLite files, conceal the added events, or accept unexplained content changes as “migration normalization.”

## 17. Provenance, evidence and trust

Source content is evidence with `authority=none`. Machine-owned fields are assigned by Core admission, never copied as authority from inbound JSON/frontmatter. Preserve imported provenance claims as claims under an imported namespace; Core records who admitted them and from which exact bytes. User acceptance means this local owner accepted a statement, not a physical human was cryptographically authenticated. An agent/model/tool name is a declaration, not an identity certificate.

Decision evidence links pin source revisions. “User judgment, no linked evidence” is permitted only as an explicit visible choice. A contradictory observation does not erase an accepted decision; it adds a visible unresolved condition. A user override requires rationale and named targets, preserves disconfirming evidence, and appears in resume/history. Source-check freshness never changes valid-time truth by ranking.

SHA-256 checks detect mismatches relative to retained expected digests. The transaction chain detects inconsistent local edits relative to its expected head; a same-user attacker can rewrite a whole vault and local manifest. Do not claim authentication, non-repudiation, tamper-proof storage or proof of complete history from an unanchored chain. Immutable here means enforced by admitted Core operations, not invulnerability to an OS owner editing files.

## 18. Strict replaceable-agent boundary

The v1 adapter is an **offline versioned package/proposal protocol**, documented independently of any model. Core never starts an agent, supplies a shell, calls a model or opens a network socket. Two independently written local clients must interoperate without sharing the product compiler implementation. Clients may be deterministic programs; that proves protocol interoperability, not AI outcome advantage.

Owner flow: select project and scope → inspect disclosure preview and privacy exclusions → issue or select an unexpired grant → request package → Core checks canonical scope/types/lifecycle/as-of and budget → persist immutable receipt and exact wire bytes → write a staged package file → verify and publish it. Every record type, including ordinary notes and source metadata, passes the same eligibility function. A project label, derived candidate or model instruction never grants access. A grant is not a bearer credential embedded in the package and cannot be imported as live authority.

The preview is bound to the exact canonical snapshot, selection, grant version and bytes/digest the owner reviews. Any change before commit requires a new preview/confirmation; it cannot silently expand the package. Grant default expiry is 24 hours, maximum seven days, owner-revocable immediately for future disclosures. Time is actual UTC; a detected rollback behind the last persisted grant-check time blocks grant use until owner clock review/reissue. This is an operational fail-closed check, not protection from an OS owner changing both clock and data.

Package cap is 256 KiB of complete UTF-8 wire bytes. Metadata counts. Include deterministic priority groups: unresolved conflict/staleness, accepted decisions, active actions, cited evidence, supporting notes; then stable recorded sequence and ID. If mandatory envelopes do not fit, omit the entire item and record why; truncate body only at a UTF-8 boundary with original digest/length and a truncation label. Never emit a bare unwrapped chunk to meet a budget. An as-of request pins recorded sequence and valid time. Output contains request/grant version labels and policy versions but no local paths, detected secrets or usable capabilities. Undetected secrets in user-authored prose remain a disclosed DLP limitation; the owner reviews the concrete package before release.

External agent flow: receive package outside Flake → return a bounded proposal file → owner chooses import → Core verifies version/size/digest/receipt reference and expected revisions → show proposed changes, evidence and claimed identity → owner accepts selected operations atomically or rejects. Agent operations are limited to proposed note/action edits, draft decisions, evidence relations and completion suggestions. Agents cannot issue grants, change project scope, accept decisions, overwrite current revisions, migrate, restore, delete data or execute tools. An accepted proposal records the owner transition and agent origin separately.

Expiry/revocation is rechecked before export and before proposal acceptance. Re-exporting saved receipt bytes is another disclosure: it requires current owner confirmation and an active matching grant; “replay” alone never authorizes a new file emission. The owner can inspect historical receipt metadata locally without renewing a grant. Acceptance also requires current expected revisions; stale proposals return Conflict and require a newly reviewed proposal, not automatic rebase. Repeated command/proposal IDs with identical digest yield the same committed result; a changed digest under the same ID is rejected. A lost response is reconciled through the immutable command result. Packages are replayable historical evidence; replay never reissues authority. No callback, lease heartbeat, provider retry, agent timeout engine or cancellation broker is needed because Flake does not run the agent. Pending proposals can expire or be manually rejected without changing accepted state.

## 19. Local-first strategy

No network, model, account or remote repository access is needed for create/open/capture/search/decision/action/resume/history/export/import/backup/restore. All canonical state is local and user-owned. External files are explicitly imported snapshots; a moved/deleted repository cannot erase the saved evidence. Optional source rechecking is a local user-triggered operation over the previously authorized selected file; it never expands to its containing repository. A new path requires owner reselection and digest comparison.

The default vault lives in the per-user application-data location, outside OneDrive/iCloud/Dropbox synchronized folders and the application installation directory. The owner can select a supported local volume. Detect known network/sync/removable/reparse locations and refuse a durability-qualified writable open; detection is best effort, and unsupported location disclosure remains required. Never silently relocate a vault. The development checkout's OneDrive location is not a qualified product vault.

No app encryption is supplied in v1. Recommend OS account controls/full-disk encryption and protected backup destinations; those do not isolate same-user processes. Backups and packages contain plaintext and retained history unless their inclusion summary says otherwise. Sync is not provided by copying an open database. Copying a verified closed backup to another machine is transfer, not collaboration. Two independent copies can diverge; opening duplicate vault identities requires an explicit read-only choice or a fork import with a new vault identity and retained origin mapping. No automatic merge of divergent histories.

## 20. Canonical security model

Assets: canonical content/history, local source paths, grants, exact disclosure receipts, backup/export data and release signing material. Threat actors: malicious imported content or package author, faulty external agent, corrupted derived input, untrusted dependency, accidental concurrent process, and storage failure. The OS account and trusted application binary are the root. No remote service, tenant, provider credential vault or sandbox is claimed.

| Threat category | Boundary / default control | Mandatory gate |
|---|---|---|
| S01 Content-to-authority confusion | Parse descriptive input only; assign trust fields in Core; no executable Markdown/HTML or imported grants | T02-02, T03-04..06, T04-01; all crafted policy-like content remains inert |
| S02 Cross-project disclosure | Canonical scope and revision check on every item and referenced metadata; default no agent disclosure | T03-04; zero disallowed bytes/IDs/paths across mixed-project fixtures |
| S03 Filesystem escape / relocation | Owner selects root/destination; reject symlinks, junctions, reparse points, device names, traversal, UNC and ambiguous names; opened-handle identity/containment at admission | T01-01, T02-05..06, T03-01; native path and replacement tests |
| S04 Malformed input / exhaustion | Bounded streaming parsing, checked lengths/nesting, no archives as executable programs, no dynamic SQL/SQLite extensions | T01-02, T02-06, T03-05; negative corpus/property/fuzz tests within controlled defensive fixtures |
| S05 Integrity / misleading provenance | Immutable revisions and command results, expected-head checks, separate claimed versus observed origin; hash limits disclosed | T01-03..04, T02-07, T03-05 |
| S06 Unapproved state change | OS-held one-writer lease and Rust admission; expected revisions; owner confirmation for accept/override/import/recover/export/delete | T01-01..03, T03-05; no public alternate canonical mutation path |
| S07 Local privacy / secrets | No telemetry, no environment/credential capture, explicit selected-source import, private file permissions where available, redacted diagnostics | All phases; secret sentinel and packet-capture checks |
| S08 UI privilege / active content | Local signed assets, strict CSP, allowlisted typed commands, no fs/shell/http/process/SQL plugins, no remote asset requests | T04-01..06; inspect built bundle and actual network behavior |
| S09 Supply chain / distribution | Exact pins, source/license/notice records, locked builds, review build scripts, vulnerability triage, signed verified packages | T00-02, T01-02, T04-01, T05-03..05 |
| S10 Destructive recovery / counterfeit completeness | No repair before preservation; recovery-to-new-root, full scope manifest, independent reconstruction, no partial-success promotion | T01-04..06, T02-05..07, T05-01..02 |

Filesystem APIs must use validated handles or platform adapters that retain equivalent object identity through the operation. Checking a string path and later reopening it is not a confinement proof. SQLite's own files require a trusted stable app-owned directory; custom source paths never become database filenames. Advisory locks coordinate cooperating Flake processes; they are not an adversarial same-user access-control boundary. Prefer the standard Rust OS file-lock API, retain a stable lock file while in use, never unlink it to “steal” ownership, and qualify native semantics. Process IDs are diagnostics only.

Secrets policy: Flake never enumerates environment variables, credential stores or repository credential files. Import allowlists text/opaque user-selected attachments and rejects known secret-file patterns by default. A bounded local detector rejects obvious credential material pending manual editing; this is not a DLP guarantee. Arbitrary user prose can contain an undetected secret, so disclosure preview and user exclusion remain essential. Sensitive records default to no agent export. A redacted export is a distinct derived view with explicit omission markers; it never silently rewrites canonical evidence. Diagnostic logs contain error codes, object/operation IDs and counts, not source bodies, package contents, absolute paths or secrets. Full disclosure bytes remain only in the private canonical receipt, not routine logs.

No shell/process authority exists in product flows. File associations, markdown links and attachment names cannot trigger execution. Model/provider and plugin interfaces remain deferred; adding either is a new trust boundary and ADR. Each phase must review newly introduced boundaries before its exit; no “security at the end” substitute.

Vulnerability response: publish a private reporting route controlled by the maintainer before release; do not invent an email or turn on a remote feature now. Maintainer triages reproducible reports within seven calendar days as an operational target, records affected versions and preserves evidence. Confirmed canonical loss, disclosure or signature failure blocks affected releases, prompts remediation and advisory assessment. Acknowledging a report is not proof of a fix. Release evidence includes the dependency advisory snapshot date and disposition; no known unmitigated high-impact issue in a shipped reachable path can be waived by a green test suite.

## 21. Durability, recovery and compatibility

**Live persistence choice:** SQLite through rusqlite, one canonical DB, rollback journal `DELETE`, `synchronous=EXTRA`, foreign keys on, trusted schema off, extension loading disabled, no untrusted URI/ATTACH, no application-supplied SQL from imports. Validate effective settings on each writer. Do not silently fall back to WAL/NORMAL/OFF. Pin a currently maintained SQLite release after advisory review; the inspected lockfile embeds 3.50.2, which is not a release approval. SQLite atomicity depends on its documented filesystem/flush assumptions; qualification must test the actual binary/filesystem/device stack.

On the macOS profile enable and verify SQLite `fullfsync=ON` for its stronger supported flush path, and qualify actual device behavior; on other profiles record the actual native flush mechanism. A successful PRAGMA statement is insufficient because unknown pragmas may be ignored: read back expected values and verify compiled features. Never claim physical persistence solely from a flag setting.

An external SQLite file is not automatically a trusted live vault. Inspect it readonly with engine resource limits, expected format/schema fingerprint and explicit permitted table/index definitions; unexpected triggers/views/virtual-table modules or write-affecting schema refuse writable admission. Foreign content enters through the published bounded portable format into a newly constructed trusted schema, never by executing imported DDL or attaching a caller-supplied database. Readonly integrity inspection is an untrusted parser boundary, so dependency qualification and limits still apply. No claim that `trusted_schema=OFF` alone makes arbitrary SQLite safe.

**Transaction:** acquire process writer ownership before any canonical mutation; validate admission and expected revisions; `BEGIN IMMEDIATE`; insert exact input/result and full immutable changes; advance head/current pointers; commit; only then acknowledge Saved. A committed command result is the retry source of truth. Error before commit returns NotSaved; error/termination around commit where outcome cannot be established returns OutcomeUnknown. On restart, reconcile command ID/digest before retry. Never translate Unknown into success or generate a new command ID automatically.

**Read:** open existing validated metadata and database without CREATE or schema migration. Do not repair a hot journal or create an index in a read-only route. Report RecoveryRequired when SQLite cannot safely serve a readonly snapshot. A process holding a writer can provide read operations through its own Core connection; independent reader processes remain readonly and use bounded snapshots. Index creation/rebuild is an explicitly classified derived mutation, never disguised as canonical repair.

**Recovery:** block canonical writers and acquire the dedicated recovery lease. Close all normal connections; preserve the complete original DB, journal, guard and incident metadata into a new destination, with digests and actual preservation result. If preservation cannot finish because of disk/permission trouble, do not modify the original. Use SQLite-managed recovery on a working copy in a new root; never manually trim journals or normalize old logs. Validate SQLite integrity, foreign keys, full transaction chain/head, current-pointer reconstruction and content digests. Publish a recovered vault only after all checks. Otherwise keep evidence and restore the latest independently verified backup to another new root. Salvage can produce a clearly partial export; it cannot become an accepted complete vault or fabricate missing records.

**Backup:** explicit command plus pre-migration/restore safeguard. Use a consistent SQLite online backup snapshot or closed snapshot, then logical/head verification; never copy an open live DB file alone. Use a sibling staging directory and no-clobber final name; flush members and publication metadata using platform-qualified operations. A result is Verified only when reopened from the destination and independently checked. Backup success records snapshot head and destination identifier; it cannot retroactively include its own later backup event. The UI shows last verified backup time/head and warns that one local disk is not an independent failure domain. No background cloud backup service.

**Legacy migration:** nonmutating format-1 inspection, exact originals and a reconciliation report, then import to an empty format-2 root. Do not replay unverified historical logs as authenticated full history. Preserve raw legacy bytes, missing history/unknown provenance and mapping. Any duplicate ID, corrupt journal or ambiguous state prevents “complete migration”; the owner may import a clearly selected partial subset as new unconfirmed evidence, with omissions, without destroying originals. No in-place upgrade. This is the bounded Spec 002 corrective replacement, not a retroactive repair of experimental evidence.

**Format evolution:** current major epoch and its immediately previous epoch may be supported by the live reader where exact capability compatibility is declared. Unknown required field/capability or newer major refuses writes and states the required reader. Unknown optional payload bytes survive read/export/import unchanged. Major migration is always copy-to-new-root with a verified backup and explicit user confirmation. Publish permanent format specifications, golden fixtures and versioned standalone offline migration tools; old owned data must remain readable without keeping an obsolete main app. No perpetual unbounded upcaster stack in Core. Downgrade opens a retained old backup, not a lossy reverse mutation. Derived state is rebuilt, never migrated as authority.

**Durability evidence matrix:** D1 every transaction/ack boundary under process termination; D2 short writes/disk-full/permission/flush errors at each storage adapter boundary; D3 competing opens, relocation and lock release after crash; D4 bit corruption/missing journal/inconsistent head with safe refusal and backup recovery; D5 interrupted backup/import/migration/export publication; D6 native unclean shutdown and power-interruption trials on all three profiles. At least 100 deterministic process-fault schedules per operation/profile, all named boundary cases, and 30 unclean native VM shutdown cycles/profile are required. Physical device power interruption additionally requires at least 10 controlled disposable-data trials/profile or an externally obtained qualified lab report for that exact stack. VM shutdown and process kill are labeled separately; neither substitutes for physical power-loss evidence. Never test using the owner's real vault. Zero acknowledged canonical loss and zero false-success recovery in the tested matrix are hard gates, not universal mathematical guarantees.

## 22. Failure-route model

Every route is Failure → Detection → Containment → User signal → Recovery → Evidence → Retry/terminal state. These reason codes are a shared CLI/UI vocabulary; wrappers may improve wording but not outcome semantics.

| Code / failure | Detection | Containment | User signal | Recovery | Evidence | Retry / terminal |
|---|---|---|---|---|---|---|
| F01 write/partial write/flush | Storage error or absent committed command result | Abort transaction; retain buffer | Not saved, or outcome unknown | Reconcile command ID after reopen | Input digest, result/error and boundary | Same ID retry only after reconciliation |
| F02 crash/power interruption | Startup journal/command state | No automatic destructive repair | Recovery required or recovered known result | Section 21 copy-and-verify | Full forensic set, native trace | Verified recovery or readonly terminal |
| F03 canonical corruption | Integrity/digest/head/reconstruction mismatch | Freeze writes, preserve all originals | Data needs recovery; no fabricated count | Restore verified backup to new root | Expected/observed digests, omissions | Complete verified restore or partial salvage only |
| F04 derived corrupt/stale | Integrity failure or checkpoint behind canonical head | Discard candidate authority; keep canonical readable | Search updating/unavailable, no false empty result | Explicit rebuild; canonical bounded fallback | Generation/head/equivalence results | Retry search after current generation |
| F05 unsupported schema | Guard/capability mismatch | Refuse mutation/migration guess | Required format/tool version | Published reader/migrator or old backup | Version and refusal result | Compatible tool or terminal readonly |
| F06 failed migration | Checkpoint/postcondition/manifest failure | Original intact; target stays staging | Migration incomplete with reason | Restart/reconcile from original into new target | Mapping, counts, digests, failure | Verified target only; otherwise terminal |
| F07 malformed/ambiguous import | Limits, schema, IDs, reference or digest validation | No active-vault writes; quarantine input unchanged | Import rejected with exact categories | Correct input or explicitly select partial new evidence | Validation and omission report | New review required; no silent skip |
| F08 missing source | Selected handle missing/unreachable | Retain saved source revision | Source unavailable; saved copy retained | Owner reselects path or accepts snapshot-only | Check result and time | Recheck or remain unavailable |
| F09 stale/revision mismatch | Observed digest differs from pinned revision | No auto-replacement of cited evidence | Changed since saved evidence | Import new revision; review affected decisions | Old/new digests and links | Explicit supersession/reconfirmation |
| F10 invalid provenance | Unknown/malformed origin or unverifiable claims | Retain as unconfirmed claim, not authority | Provenance unverified | Owner supplies evidence or rejects | Raw claim and admission result | Explicit acceptance of judgment or rejection |
| F11 unauthorized agent operation | Grant/scope/operation/expiry/revision check | No canonical change/disclosure | Request rejected with safe reason | Owner creates a new valid request | IDs and reason, no sensitive content | New reviewed request; repeat abuse remains rejected |
| F12 malformed/duplicate agent output | Protocol/size/digest/idempotency checks | Preserve bounded inbound artifact privately | Proposal invalid/conflicting/previously processed | Correct proposal or inspect prior result | Digest and validation/receipt binding | Same-ID exact replay or new reviewed proposal |
| F13 unavailable model | No model process exists in v1 | Core unaffected | External client unavailable if owner reports it | Continue manually or choose another external client | No invented model result | Normal core loop continues |
| F14 unavailable network/remote commit | No fetch attempted; reference cannot be locally verified | Use saved evidence with limits | Reference unverified; offline normal | Optional later explicit local snapshot import | Claimed locator and observed status | Normal core loop; no false verification |
| F15 full disk / denied permission | Preflight plus actual storage errors | Stop publication, preserve original and unsaved input | Space/access needed, actual outcome shown | Owner changes space/destination/permissions | Error, stage, command ID | F01 reconciliation before retry |
| F16 relocation / duplicate open | Root handle/identity mismatch or lock contention | Refuse competing writer; no lock theft | Vault moved / already open / duplicate identity | Close/reopen verified root, or explicit fork import | Root identity/lease and mapping | One writer or readonly terminal |
| F17 export partial/cancelled | Member write/digest/publication failure | Staging stays incomplete; no overwrite | Export incomplete, destination and reason | Retry new verified target | Manifest draft and failed stage | Only verified publication is success |
| F18 restore failure | Manifest/postrestore mismatch | Active original and backup intact | Restore incomplete | New empty destination and verified backup | Before/after head and validation | Verified root or terminal |
| F19 unsupported filesystem/platform | Profile/location/capability checks or qualification failure | No durability-qualified writable mode | Unsupported storage/platform | Copy verified backup to qualified local profile | Exact unsupported capability | Readonly inspection or terminal |
| F20 decision conflict/override | Same key/time overlap, dependency contradiction | Do not auto-resolve or silently complete | Needs review, all competing evidence visible | Owner explicit reasoned transition | Supersession/override history | Resolved explicitly or remains unresolved |
| F21 stale proposal / lost acknowledgement | Expected revision or command-result lookup | No rebase or duplicate change | Changed since proposal / result recovered | New owner review or exact idempotent result | Proposal and transaction IDs | Review retry or exact prior result |
| F22 release/license/signature failure | Artifact/license/advisory/signature checks | Withhold affected release candidate | Release blocked, named obligation | Repair/rebuild/reverify without changing gate | SBOM, license and verification report | No final completion until resolved |

## 23. Build, integrate, adapt, defer or reject

| Capability | Decision | Smallest justified scope |
|---|---|---|
| Local persistence | INTEGRATE | Existing SQLite/rusqlite, qualified canonical transaction profile; do not build a database |
| Indexing / full-text retrieval | INTEGRATE | Separate SQLite FTS5, transaction-head checkpoint and rebuild |
| Search | BUILD | Canonical eligibility, simple filters and stable result/snippet presentation around FTS5 |
| Graph / vector retrieval | DEFER | Explicit user relations retained; extraction/embeddings need a later failed requirement and experiment |
| Agent adapters | BUILD | Open package/proposal contract plus two independent test clients; no provider runtime |
| Model runtime | DEFER | No model weights, inference library, provider credential or paid API in core |
| Project import / export | BUILD | Versioned full-fidelity format, selected Markdown/file inputs and explicit loss report |
| Repository integration | ADAPT | Snapshot bytes and user-declared commit/path labels; no Git subprocess or service API |
| Filesystem watching | DEFER | Explicit source recheck; canonical DB transactions already provide deterministic index invalidation |
| Task / decision tracking | BUILD | Four familiar record types and explicit lifecycle; no user-programmable workflow engine |
| Provenance / receipts / history | ADAPT | Minimal entity/activity/actor vocabulary and full immutable payload history; no RDF/event platform |
| Sync / collaboration | DEFER | Transfer verified backups, explicit duplicate-identity handling; no merge protocol |
| Backup | INTEGRATE | SQLite consistent snapshot mechanism wrapped by verification and no-clobber publication |
| Encryption | DEFER | No app crypto/key lifecycle; OS protection and truthful plaintext export disclosure |
| Desktop shell | INTEGRATE | Tauri 2, React/TypeScript, narrow Rust commands, local assets |
| Editor / CLI | BUILD | Plain textarea/safe preview and shared Core command interface; no generic editor platform |
| Telemetry | REJECT | No mandatory or automatic analytics; optional user-owned local study reports only |
| Auto-update | DEFER | Manual signature-verified replacement and explicit migration |
| Plugins / marketplace / Hub / cloud | DEFER | No execution or hosting surface needed for the continuity loop |
| Self-evolving skills / arbitrary execution | REJECT | No v1 requirement; future proposal needs a new authority/security design |

## 24. External sources and licensing

Additional research stopped after resolving storage, locking, desktop isolation and offline distribution questions. Existing competitor/donor research remains attributed in the 15-report review; it was not restarted. Sources below were read on 2026-09-13. Documentation URLs can move; T00-02/T04-01 must record exact retrieved versions/hashes for implementation admission. Documentation study is not code reuse authorization.

| Primary source / unresolved question | Evidence extracted / influence | License and reuse boundary |
|---|---|---|
| [SQLite atomic commit](https://www.sqlite.org/atomiccommit.html), [synchronization](https://www.sqlite.org/pragma.html#pragma_synchronous) | ADOPT transactional storage with explicit flush assumptions; EXTRA selected for rollback mode | SQLite core public domain, wrapper separately licensed; no copied implementation text |
| [SQLite backup](https://www.sqlite.org/backup.html), [file format](https://www.sqlite.org/fileformat.html) | ADAPT consistent snapshot and independently documented format; raw open-file copy is insufficient | REFERENCE docs; use admitted engine API, preserve format-specific tests |
| [SQLite corruption guidance](https://www.sqlite.org/howtocorrupt.html), [WAL](https://www.sqlite.org/wal.html) | REFERENCE locking/platform risks and current WAL maintenance history; no WAL needed for one writer | No vulnerability reproduction. Refresh advisories before dependency admission; page describes fixes absent from old bundled version |
| [SQLite copyright](https://www.sqlite.org/copyright.html) | REFERENCE public-domain core versus separately copyrighted tooling/tests | Verify exact redistributed source units, not the project name alone |
| [Rust File locking](https://doc.rust-lang.org/std/fs/struct.File.html#method.try_lock) | ADOPT OS-held standard-library lock where qualified; lifetime/handle behavior matters | Rust distribution notices at exact toolchain; documentation licenses do not automatically apply to copied code |
| [RFC 8785 JCS](https://www.rfc-editor.org/rfc/rfc8785) | ADOPT deterministic machine-envelope digest encoding with bounded numeric/string rules; fixes cross-language retry/receipt ambiguity | Informational RFC, not an Internet Standards Track claim; reference specification only, implementation package requires exact rights review |
| [Tauri security](https://v2.tauri.app/security/), [permissions](https://v2.tauri.app/security/permissions/), [CSP](https://v2.tauri.app/security/csp/) | ADAPT minimal local webview-to-Core command boundary; explicitly configure CSP and permissions | Tauri intended MIT/Apache-2.0 components require exact-package admission, transitive notices and build-script review |
| [Tauri Windows installer](https://v2.tauri.app/distribute/windows-installer/) | ADOPT offline runtime installer mode; default bootstrap path needs network | WebView2 redistribution terms separately reviewed; no assumption that its runtime is Apache-2.0 |
| [Tauri Debian](https://v2.tauri.app/distribute/debian/), [macOS signing](https://v2.tauri.app/distribute/sign/macos/) | REFERENCE native package dependencies and signing/notarization requirements | Platform redistribution/signing rights and credentials are separate release gates |

The founder selected **Apache-2.0**. T05-04 adds the actual license/notice files only after ownership and prior contribution/provenance review. Cargo metadata alone does not license all historical evidence or copied third-party content. No root LICENSE was added during planning.

Read-only locked Cargo metadata contains 53 packages including the root, 52 third-party packages. Runtime direct dependencies are rusqlite 0.37.0 (MIT), uuid 1.24.1, sha2 0.10.9, serde 1.0.229 and serde_json 1.0.151 (MIT OR Apache-2.0). libsqlite3-sys 0.35.0 wraps SQLite 3.50.2 in the inspected cache. Distinguish wrapper license and engine public-domain status. Most transitive packages are MIT/Apache choices; special obligations include foldhash Zlib, unicode-ident `(MIT OR Apache-2.0) AND Unicode-3.0`, and memchr's MIT option. r-efi offers MIT/Apache/LGPL choices and did not expose a root license filename in the simple inventory; prove whether it ships on selected targets and retain the applicable text before distribution. This was a metadata/license-file-presence inspection, not legal clearance or a vulnerability audit.

Admission record for every shipped component: exact version/source digest, selected license expression, license/notice texts, transitive and embedded units, build scripts/features, modifications, provenance and redistribution scope, reachable advisory status, necessity and owning task. Unknown ownership/rights blocks shipping that component. Use permissive options only where actually offered; do not discard mandatory Unicode/Zlib/platform notices. Data/test corpora need redistributable or explicit consented rights. Optional future models need separate code, weight, tokenizer and dataset terms; no weights are admitted now. PR 40 source-use permission is retained evidence, not a substitute for exact distributed-code rights. No donor implementation is copied by this plan.

## 25. Distribution and release model

Supported v1 profiles are **Windows 11 x86-64 on local NTFS**, **Ubuntu 24.04 LTS x86-64 on local ext4**, and **macOS 14 or later on Apple Silicon/local APFS**, each qualified on an exact supported OS build at release. A passing macOS build does not imply every later OS is tested; publish the tested-version table. No Windows ARM, Intel macOS, mobile, arbitrary Linux distribution, network filesystem or active consumer-sync directory claim. Platform expansion requires a new qualified profile.

Ship CLI archives for all three: Windows ZIP, Linux/macOS tar archive; `flake` is the new product command. Preserve the historical `fehrest` source/evidence namespace and do not rewrite old commands in sealed artifacts. A compatibility alias, if shipped, must call the same new implementation and refuse unsafe old-format writes; no hidden behavior fork. Desktop: Windows per-user signed NSIS installer with offline WebView2 installer, Ubuntu `.deb` plus an offline dependency bundle for the exact clean image, and signed/notarized/stapled macOS `.dmg` containing the app. No mandatory developer toolchain on an end-user machine.

Default data paths use the OS's per-user application-data directory under Flake/vaults. Installer assets live separately; uninstall removes application files and optional preferences only, retains vaults/backups by default, and clearly reports that retention. First run requires no sign-in and does not scan the user's home/repositories. Updates are manually downloaded/transferred verified packages, signature/checksum checked before replacement; no auto-updater or silent migration. A newer format requires explicit migration-to-new-root after backup. Old app rollback uses retained compatible data, not lossy downgrade.

Release set: source archive at exact commit, platform binaries/installers, full offline dependencies required on the declared clean image, SHA-256 manifest, signatures, SBOM and notices, versioned format/protocol specs, CLI/help/user/recovery guides, migration tool and fixtures, verification report and known limitations. Reproducibility requires two clean builds with pinned compiler/dependencies/assets: unsigned payloads match bit-for-bit or each irreducible toolchain difference is isolated, documented and independently shown not to affect code/content. Signatures/timestamps are compared separately. “Reproducible build” is claimed only for the subset actually byte-reproduced; unexplained executable divergence blocks release.

Windows signing identity and macOS Developer ID/notarization credentials require owner-controlled release infrastructure. No credentials go into content/logs; no purchase or publication is authorized here. Linux package and checksum manifests require an owner-controlled release signing key. Missing credentials block the release task, not product design. Creating verified signed local release candidates is the completion gate; uploading, publishing a release, pushing or merging remains separately authorized. An unsigned developer build is not the final release candidate.

## 26. Metrics and value gates

Every metric is a future acceptance target, not an observed result. Evidence vocabulary: VERIFIED = inspectable exact artifact supports the specific claim; LOCALLY VERIFIED = supports only the named environment; REPRODUCIBLE = another executor repeated the exact method; INFERRED = conclusion depends on stated assumptions; PARTIALLY SUPPORTED = some prerequisites/observations missing; UNPROVEN = no adequate experiment; UNKNOWN = necessary fact unavailable; DEFERRED = outside scope.

| Category | Definition / gate |
|---|---|
| Atomic persistence | Acknowledged commands whose complete state/history/result survives recovery / acknowledged commands. 100% in D1-D6; any observed loss blocks release |
| Recovery and corruption | Recoverable fixtures restored exactly / recoverable fixtures; 100%. Every seeded corruption is detected or declared outside the tested detection model; no silent acceptance |
| Reconstruction / portability | Exact immutable payload/history/receipt bytes and logical state after full export/import/independent reconstruction. 100% across supported format/profile fixtures |
| Provenance | Every persisted/imported item has observed origin or explicit Unknown; all accepted evidence links resolve to exact stored revisions or visible Missing. 100% labeling, not 100% factual certainty |
| State quality | Zero impossible action transitions, supersession cycles or silently hidden conflicts in generated/curated suites; explicit unlinked judgments counted separately |
| Resume correctness | Correct next action plus correct current decision, conflict and source-status interpretation on preregistered tasks; no unsafe action counted as success |
| Source awareness | Correct classification of matching/changed/missing/unchecked sources and visible unresolved conflicts; 100% on held-out deterministic cases |
| Maintenance cost | Capture, organization, source checking, correction, export and resume time all counted; no free Flake preparation versus charged baseline preparation |
| Adoption | Voluntary local repeat use during the bounded trial below; no compulsory cloud telemetry or silent tracking |

**P03 value gate before desktop:** preregister six participants from the target group, eight matched interruption/resumption pairs each, 96 total attempts. Use disjoint but difficulty-matched cases, randomize order and counterbalance conditions. Baseline is plain Markdown, an explicitly maintained project index/task list and ordinary local file search; both conditions receive the same raw sources, setup/training time, human review, maximum time and byte-access budget. Use product Rust paths; do not substitute a scripted resolver. Hold answer keys and changed-source/conflict cases outside the implementation fixtures until evaluation. Record all attempts and reasons for exclusion before seeing results; failed/timeout attempts remain failures. No model/API is required.

Qualify only if Flake achieves at least 90% successful resumes, no more than five percentage points below baseline success, at least 20% lower median paired **total maintenance-plus-resume time**, and zero missed high-consequence planted conflicts or unintended disclosures. Target benefit is 25%; 20% is the minimum investment gate, chosen as a meaningful reduction that repays switching/maintenance, not a literature-derived performance claim. Report participant-clustered descriptive intervals and all task-class results; this small study does not prove equivalence or population superiority. Any class with zero successful Flake cases blocks qualification. Two independent offline clients must separately pass the interchange contract; no claim of AI outcome superiority follows from them.

**P05 adoption gate:** eight new consenting target users, ten workdays, with their own disposable/nonsecret or privately retained project data. At least six users must voluntarily use Flake on six distinct days and each record at least three successful later resumptions. At least six must complete capture→evidence→decision/action→resume→export without facilitator rescue by the last session. Record maintenance time and reasons for abandoning. Observation is opt-in, local and exportable by the participant; publish aggregates and consented synthetic reproductions, never private content. Missing participants/evidence is Inconclusive, not Pass. These are small product investment gates, not retention claims for a market.

One bounded defect-repair repeat per proof gate is permitted after recording the failed run, exact fixes and a newly preregistered disjoint case set. A second failure or inconclusive result stops expansion and requires product reconsideration; no threshold lowering or cherry-picked reruns. Historical R1 remains separate and retains its uncertainty about raw observations, saved-state behavior, fallback equality, budgets and inference.

## 27. Performance budgets

Budgets are interaction and resource design constraints, not claims of measured speed. Target is desirable; maximum is release-blocking. The gap between them permits ordinary optimization without moving the acceptance threshold. T00-02 freezes exact measurement hardware; T05-02 repeats on each supported native profile. Reference minimum: four physical CPU cores, 8 GiB RAM, local SSD, no concurrent benchmark load. Record actual CPU/RAM/SSD/OS/filesystem, power settings, toolchain and build mode. Release builds only; report at least 30 cold launches and 100 warm operations, p50/p95/max, errors and memory. Cold means new process and explicitly reported cache state; do not call a warm OS cache cold storage.

Dataset S: 100 records/10 MiB; M: 10,000 current records, 100,000 revisions/transactions and 1 GiB canonical payload; L: 100,000 current records, 1,000,000 revisions/transactions and 10 GiB payload. Mix projects/types/Unicode/near-limit records; fixed seed and generator digest. M is the interactive release gate; L must remain bounded, cancellable and truthful but may use slower reported batch times. Per-operation budget excludes explicitly reported unsupported external-source I/O, never hidden background canonical work.

| Operation / condition | Target | Maximum acceptable | Gate / size |
|---|---|---|---|
| CLI help/version cold process | p95 100 ms | 500 ms | Hard, no vault I/O |
| CLI project open readonly, M | p95 250 ms warm / 1 s cold | 1 s warm / 3 s cold | Hard; no full-history scan disguised as startup |
| Desktop first usable project list, M | p95 1 s warm / 2 s cold | 3 s warm / 5 s cold | Hard; background checks visibly incomplete |
| Project detail / canonical read ≤1 MiB | p95 100 ms warm / 250 ms cold | 500 ms / 1 s | Hard |
| Capture UI feedback | p95 50 ms | 100 ms | Hard; feedback means unsaved buffer, not durability |
| Save/write acknowledgement ≤64 KiB | p95 150 ms | 750 ms; any >2 s shows progress | Hard, full durable commit included |
| Search M, 50 results | p95 150 ms warm / 500 ms cold | 500 ms / 1.5 s | Hard; stale status included |
| Resume M, ≤256 KiB | p95 250 ms warm / 750 ms cold | 1 s / 2 s | Hard, scope + canonical revision checks + receipt when exported |
| Full FTS rebuild M | 30 s | 120 s | Hard; progress every ≤1 s, cancel response ≤1 s |
| Full export/import M | 60 s each | 180 s each | Hard; verification included, no partial success |
| Clean startup integrity quick checks M | 250 ms | 1 s | Hard; full verify is explicit operation |
| Full verify / recovery working copy M | 60 s | 180 s | Hard; forensic copy bytes/time reported separately |
| L rebuild/export/import/full verification | 10 min each | 30 min each | Hard bounded batch qualification; cancel ≤1 s outside atomic commit |
| Core RSS / desktop RSS on M | 128 / 384 MiB | 256 / 768 MiB | Hard, peak incl import/rebuild measured; no entire-history reads |
| Storage growth / amplification | Report retained bytes by type | ≤3× uncompressed unique payload + transaction metadata for fixed corpus; derived ≤2× searchable current text + 64 MiB | Hard on fixed corpus; count full history, receipts and backup separately |

Normal inputs: text body ≤1 MiB, title ≤1 KiB, source locator label ≤4 KiB, query ≤1 KiB, decision statement ≤8 KiB, event metadata ≤16 KiB excluding referenced full payloads, JSON depth ≤32, proposal ≤1 MiB and ≤100 operations, package ≤256 KiB, artifact ≤64 MiB. Imports stream with per-member and total limits; default whole import cap 10 GiB/1,000,000 records, owner can select smaller batches, never bypass per-record bounds. Full exports of larger vaults stream and may split by deterministic member groups with a parent manifest; no silent truncation. Bounds are safety ceilings, not pricing tiers. Exceeding them returns explicit size/count information and no partial active-state admission.

If the specified minimum hardware cannot meet a maximum after one measured bounded optimization pass, stop the affected unit for ADR/scope reconsideration. Do not silently weaken durability, remove provenance or make a hosted service mandatory to pass.

## 28. Evidence artifact model

Each task records `docs/evidence/flake-v1/<task-id>/REPORT.md` plus exact raw artifacts under a matching nonhistorical evidence directory. A report identifies: task/spec/plan version; baseline and tested source commit/tree; dirty diff digest if applicable; environment and tool versions; inputs/generator/seed/digests; exact commands and exit codes; stdout/stderr/raw timings; expected/observed results; failed attempts; exclusions; limitations/unsupported profiles; reviewer identity and independence limits; acceptance decision and next frontier. A raw artifact manifest gives lengths and SHA-256 for all files. Never hash only a narrative or silently omit the executed harness.

Avoid self-reference: test an immutable implementation commit, then commit its evidence separately if necessary, identifying both. A final completion report may live in a subsequent documentation commit whose parent is the tested release commit; no code may change after the release evidence without affected checks rerunning. Input, output, comparison scripts and analysis must be available enough for independent repetition. Protect user study content and credentials; consented synthetic datasets reproduce contracts, private content does not go into public Git history.

Tests are checks; evidence is the inspectable result of a specific run. Prior passing tests do not prove a changed binary. A Linux build on Windows is not a Linux-native test. A hash-valid archive is not a scientifically valid benchmark. A passing fixture proxy is not an independent agent outcome. No `PASS`, `CLOSED`, `MERGED`, `DURABLE`, `SAFE` or `PROJECT_COMPLETE` without the exact supporting artifacts and scope.

## 29. Verification hierarchy

| ID | Gate | What it can establish |
|---|---|---|
| V01 | Static/schema/format/Clippy | Declared code/format properties; no runtime guarantee |
| V02 | Unit | Local function behavior for named cases |
| V03 | Integration | Real components and persistence interactions |
| V04 | End-to-end | User/CLI workflow through actual product paths |
| V05 | Property/invariant | Generated cases preserve named invariants under bounded generation |
| V06 | Fault injection | Selected storage/error boundary responses, not physical device behavior |
| V07 | Crash/recovery | Process interruption and restart with exact persisted observations |
| V08 | Migration | Specified old/new fixtures, fidelity and interrupted transitions |
| V09 | Security | Named trust-boundary negative cases, source review and dependency admission |
| V10 | Compatibility | Version/capability refusal and declared supported transitions |
| V11 | Packaging | Clean install/update/uninstall/signature/offline behavior |
| V12 | Platform-native | Actual named OS/filesystem/architecture and D6 observations |
| V13 | Performance | Section 27 method and thresholds on exact data/hardware |
| V14 | Manual UX | Keyboard/IME/accessibility and user observation with scripted criteria |
| V15 | Independent reproduction | Another executor/tool repeats exact claims without shared hidden implementation |

Common commands once implementation exists: `cargo fmt --all -- --check`, `cargo test --locked --all-targets`, `cargo clippy --locked --all-targets -- -D warnings`; use RTK wrapper where repository rules require it. These do not activate model benchmarks. Muse must add named local defensive/format/fault/CLI tests and scripts in their owning task and record exact invocations; this plan uses method descriptions where commands do not yet exist. Desktop checks include the admitted package manager's locked install, typecheck, unit/build commands and native manual/automation script. No lower gate substitutes for a higher one.

## 30. Old-spec and roadmap convergence

The incorporated [disposition annex](FLAKE_PLANNING_CORPUS_DISPOSITIONS.md) gives every material document exactly one primary KEEP/AMEND/MERGE/SPLIT/REORDER/DEFER/DELETE/SUPERSEDE classification, apparent authority, reason, surviving contract, change, destination, affected dependencies and old-direct-execution flag. All flags are NO: even retained historical invariants are implemented through this plan and its active unit. DELETE would mean remove execution scope, not erase evidence; no historical artifact is deleted.

| Prior logical unit / namespace | Primary disposition | Survives / changes / owning destination |
|---|---|---|
| Historical Spec 001 / Phase T | KEEP | Exact experiment and source history; no reactivation |
| Main/PR2 Spec 002 | AMEND | Writer/identity/durability purpose; format-2 corrective addendum, P01 |
| Main/PR2 Spec 003 | SPLIT | Necessary lexical generation/rebuild at T02-04; watcher/general registry deferred; no automatic activation |
| Main/PR2 004 GI capability | DEFER | Falsifiable comparator after a retained requirement fails; outside v1 |
| Main/PR2 005 graph integration | DEFER | No production graph dependency |
| Main/PR2 006 temporal memory | SPLIT | Explicit decisions, time, conflicts at T02-03/T03-02; automatic memory/extraction deferred |
| Main/PR2 007 compiler/gateway, PR40 007A/007B | SPLIT | Eligibility/receipts/proposals at T03-04..06; server and execution broker deferred |
| Main 008 full vertical proof | REORDER | P03 pilot before desktop, P05 independent/adoption proof; no paid inference gate |
| Main 009 desktop | AMEND | Minimal Tauri/textarea P04, after P03 proof |
| PR2 008 GitHub/IDE integration | SPLIT | Local snapshot and supplied commit labels T03-01; remote API/IDE plugin deferred |
| PR2 009 vertical proof | MERGE | Same new P03/P05 proof, not a second numbered frontier |
| PR2 010 open objects/workspace | SPLIT | Four typed records and portable model P02; universal schema/view engine deferred |
| PR2 011 personal workspace | SPLIT | Project capture/resume P02/P04; inbox/task-suite breadth deferred |
| PR2 012 advanced search/graph views | SPLIT | Literal/filter search T02-04/P04; graph/AI search deferred |
| PR2 013 model runtime | DEFER | No provider/model prerequisite |
| PR2 014 web acquisition | DEFER | Explicit local source import only |
| PR2 015 migration/adoption | REORDER | Portability at P01/P02, compatibility and adoption P05 |
| PR2 016 sync experiment | DEFER | No v1 sync authority or runtime |
| PR2 017 sync production | DEFER | Requires a later authority/merge/key decision |
| PR2 018 organization auth | DEFER | Solo OS owner, no tenancy claim |
| PR2 019 team communication | DEFER | No messenger/topics platform |
| PR2 020 mobile | DEFER | No mobile package in release gate |
| PR2 021 / PR40 021A/021B extension/skill lifecycle | DEFER | No plugin host, install/update/optimization machinery |
| PR2 022 public Hub | DEFER | No public service or marketplace |
| PR2 Linear L-PX0..5 / L-GA | SUPERSEDE | Simple work objects/keyboard workflow retained P02/P04; parity track removed |
| PR40 P-TS-01..07 | MERGE | Time/truth/provenance/promotion/residency distinctions in I01-I12 and P03 |
| PR40 P-TS-08 | DEFER | Extraction pipeline outside scope |
| PR40 P-TS-09..15 | SPLIT | Read/proposal admission and artifact integrity in P03; execution liveness/fencing/install/network controls deferred with broker |
| PR40 P-TS-16..18 | MERGE | Fair budgets, reproducibility and optimization-before-claim rules in sections 26-29 |
| PR40 P-TS-19 | KEEP | Rights/provenance gate retained in section 24 |

Historical ADR 0001 and 0006 are SUPERSEDE for Flake v1 physical storage only; 0002 and 0011 SUPERSEDE with plain editor/Tauri; 0003 DEFER; 0004/0005/0007/0008/0009/0010/0014 KEEP for retained identity, pattern-only reuse, lexical/time/Rust/method invariants; 0012 DEFER; 0013 SUPERSEDE for format-2 layout; 0015 AMEND with bounded live readers and permanent standalone migration specifications; 0016 AMEND with transaction-sequence projection checkpoints. The original ADR text stays historical. Main F-1's agent-only powered thesis gate is AMEND prospectively to the explicitly human-and-agent-independent continuity investment gate, without changing any historical statistical result. F-2/F-6/F-9/F-12 safety intent survives. Graph/editor-specific failures are DEFER with their removed capabilities, not declared empirically disproven.

## 31. Definitive dependency DAG

There are **6 phases, 12 slices and 38 executable Muse tasks** (two intake/specification tasks and 36 delivery/verification tasks). The graph is an intentionally serial topological chain because one active frontier and small reviewable units are more valuable than scheduling concurrency here. Every task depends on its immediate predecessor; the first has no task predecessor but requires a subsequent implementation instruction. Transitive dependencies are normative. No task is independent of the preceding phase exit.

```text
P00: T00-01 -> T00-02
  -> P01: T01-01 -> T01-02 -> T01-03 -> T01-04 -> T01-05 -> T01-06 -> T01-07
  -> P02: T02-01 -> T02-02 -> T02-03 -> T02-04 -> T02-05 -> T02-06 -> T02-07
  -> P03: T03-01 -> T03-02 -> T03-03 -> T03-04 -> T03-05 -> T03-06 -> T03-07 -> T03-08
  -> P04: T04-01 -> T04-02 -> T04-03 -> T04-04 -> T04-05 -> T04-06
  -> P05: T05-01 -> T05-02 -> T05-03 -> T05-04 -> T05-05 -> T05-06 -> T05-07 -> T05-08
```

The contract metadata below supplies exact dependencies and next task. A task can have internal substeps, but those do not create parallel active frontiers or authorize a successor early. Evidence checks from earlier units stay required; a regression reopens the owning task and blocks descendants.

## 32. Phase definitions

| Phase | Outcome / entry | Exit |
|---|---|---|
| P00 Intake and corrective specification | Future implementation instruction; exact live recheck | Authority, historical preservation and bounded Spec 002 addendum ready; no product mutation yet |
| P01 Reliable save and recovery | P00 closed | New isolated format-2 save/history/backup/migration works under first native fault gate |
| P02 Portable project work | P01 closed | Create, capture, find, decide, complete and fully reconstruct a project through CLI |
| P03 Scoped resumption and proof | P02 closed | Current/stale/conflicting evidence, receipts and proposals work; local value gate passes before UI |
| P04 Minimal desktop continuity | P03 closed | Same Core loop usable with required keyboard/accessibility/failure behavior |
| P05 Qualified release | P04 closed | Compatibility, all native durability, offline signed packages, independent reproduction, adoption and final audit pass |

## 33. Vertical slices

| Slice / tasks | New user outcome | Assumption / boundary / durability / evidence / unlock |
|---|---|---|
| VS00 intake: T00-01..02 | Implementer can start one precise authorized correction | Enabling exception, no product value claimed; exact source/spec evidence unlocks P01 |
| VS01 save: T01-01..03 | Inspect old data safely and save a new record with an honest result | One canonical transaction/owner boundary; interruption evidence unlocks recovery |
| VS02 recover: T01-04..07 | Recover/backup/import legacy data without destroying it | Copy-and-verify recovery; native fault/legacy reports unlock project workflow |
| VS03 work: T02-01..04 | Create a project, capture, decide, finish and find work | Canonical types/lifecycle versus index; CLI state tests unlock portability |
| VS04 leave: T02-05..07 | Export and independently reconstruct all owned project data | Exit fidelity/import boundary; independent generic-reader proof unlocks resume |
| VS05 resume: T03-01..03 | See what changed and choose a defensible next action | Time/source truth not rank; source loss retains evidence; held-out cases unlock packages |
| VS06 exchange: T03-04..06 | Give an external client scoped context and review proposals | Disclosure/admission boundary; permanent exact receipts; interoperability unlocks value trial |
| VS07 prove: T03-07..08 | Observe whether the CLI loop repays its maintenance cost | Product hypothesis, fair local trial; raw negative results preserved; pass unlocks desktop |
| VS08 desktop work: T04-01..03 | Capture/search/complete through a small accessible app | Thin UI cannot own data semantics; restart/edit evidence unlocks desktop resume |
| VS09 desktop resume: T04-04..06 | Review changes, agents, recovery and export without CLI knowledge | Same grants/failure semantics; native UX evidence unlocks release qualification |
| VS10 install: T05-01..04 | Install, update and retain data offline on supported systems | Version/installer/supply chain boundary; native/rights/signature evidence unlocks independent use |
| VS11 complete: T05-05..08 | Depend on a verified bounded release and complete exit documentation | Independent reconstruction, voluntary use and exact release audit; unlocks completion only |

## 34. Executable task contracts

**Shared contract SC applies to every task in addition to its individual fields.** Before implementation follow SPEC → CLARIFY → PLAN → CHECKLIST → TASKS → ANALYZE → PONYTAIL → IMPLEMENT → TEST → required BENCHMARK → SECURITY → REVIEW → CONVERGE. Clarification records already frozen answers; it is not an invitation to ask Astro to redesign. T00-02 creates the first corrective Spec Kit; each later unit records a bounded addendum with task-specific implementation details and links to this specification before code. No new product/architecture choices are delegated. Ponytail asks whether each new abstraction/dependency is necessary to the task's stated outcome; eliminate unused generality.

All tasks preserve I01-I12, prior evidence, forbidden capabilities and exact authority. Allowed files are likely components, not permission to touch unrelated code. A task may update its tests/documentation/evidence only as required. Static/code tasks use V01 and appropriate V02/V03 plus named additional gates. Every new trust boundary requires V09. `Not applicable` fields below mean the task introduces no such behavior; they cannot waive an inherited unresolved gate. Complete only when all individual acceptance conditions, shared contract and predecessor/phase gates pass with section 28 evidence. Record failure and remain at the task otherwise. Update CURRENT and exact checkboxes only then. Prefer one narrowly coherent verified commit; split evidence commits when needed to bind immutable source. No paid inference, publication or history rewriting.

### T00-01 — Reverify and activate the bounded implementation frontier

- **Task ID / phase / slice:** `T00-01` / `P00` / `VS00`
- **Objective:** Establish exact current authority and a clean, preserved implementation baseline.
- **Why it exists:** The planning commit is local and later GitHub work can invalidate stale task assumptions.
- **Dependencies:** None in the DAG; requires a subsequent implementation instruction and section 1 authority.
- **Files/components:** AGENTS.md, specs/CURRENT.md, canonical handoff/plan, Git metadata and task evidence.
- **Allowed scope:** Planning/governance/evidence Markdown and read-only inspection only.
- **Forbidden scope:** Runtime/source/schema/migration/workflow/CI/configuration/package edits; all remote mutations.
- **Implementation requirements:** Read the future implementation instruction; record its scope. Fetch and inspect exact remote/PR state, 4246f6d ancestry and all 15 report hashes. Compare live changes with this plan. Record T00-01 as the sole active unit only if the instruction authorizes implementation of this graph.
- **Security considerations:** Do not infer remote publication, spending, model execution or broader capability authority.
- **Data invariants:** Preserve prior commits, evidence and unrelated work; no source/runtime changes in this task.
- **UX behavior:** Implementer sees one active unit and an explicit conflict/blocker report if live truth differs.
- **Failure behavior:** If authority or a load-bearing source is missing, remain at intake and name it; no speculative code.
- **Acceptance criteria:** Exact root/HEAD/tree/branch/remotes/PRs/worktree and authorization are recorded; review commit and files preserved; no competing frontier.
- **Tests:** V01 document links, ancestry and hash checks; no application tests needed for this documentation task.
- **Verification method:** Git status/show/log/merge-base, fetched refs, provider PR metadata and SHA-256 comparisons; record exact invocations.
- **Evidence artifact:** `docs/evidence/flake-v1/T00-01/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Not applicable: no product flow.
- **Durability gate:** Preservation hashes only; no durability claim.
- **Cross-platform gate:** Record available native hosts; absence is an execution prerequisite, not a false pass.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T00-02`

### T00-02 — Publish the Spec 002 corrective addendum and admission record

- **Task ID / phase / slice:** `T00-02` / `P00` / `VS00`
- **Objective:** Make the first correction executable without reopening product strategy.
- **Why it exists:** Existing Spec 002 closeout is evidence but its writer/recovery contract needs prospective correction.
- **Dependencies:** `T00-01`
- **Files/components:** specs/002-post-r1-canonical-core-convergence/ corrective Markdown, docs/evidence/flake-v1, dependency/platform inventory.
- **Allowed scope:** Planning/governance/evidence Markdown and read-only inspection only.
- **Forbidden scope:** Runtime/source/schema/migration/workflow/CI/configuration/package edits; all remote mutations.
- **Implementation requirements:** Instantiate SPEC/CLARIFY/PLAN/CHECKLIST/TASKS/ANALYZE/PONYTAIL for T01-01..07 using this plan. Enumerate every old canonical mutator and source finding. Map retained old tests to new guarantees and intentional format differences. Pin reference hardware/dataset definitions and proposed dependency versions with license, advisory and build-script admission records; perform no installs or product edits in this unit.
- **Security considerations:** Class C/D/E decisions reference ADR-0017 and the dedicated security review; no invented approval requirement beyond actual governance.
- **Data invariants:** Keep old closeout and sealed fixtures byte-exact; put format-2 fixtures in a new namespace.
- **UX behavior:** Corrective scope is explicit: safe legacy inspection, new atomic save/history/recovery and migration, not Spec 003.
- **Failure behavior:** Unclear rights, unsupported required native API or a contradictory frozen contract blocks the owning implementation task with evidence.
- **Acceptance criteria:** All source findings have an owner; every P01 task is specified with negative cases; exact dependency/platform qualification work and evidence paths are recorded.
- **Tests:** V01 contract completeness and dependency audit; static source inventory; no feature tests yet.
- **Verification method:** Inspect Rust call sites and public mutation APIs; locked Cargo metadata and primary advisory/license sources; compare checklist with I01-I12 and D1-D6.
- **Evidence artifact:** `docs/evidence/flake-v1/T00-02/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Freeze section 27 hardware and generator specification; no measured PASS claimed.
- **Durability gate:** Define boundary schedule and external native test prerequisites.
- **Cross-platform gate:** Plan W11/NTFS, Ubuntu/ext4 and macOS/APFS evidence separately.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T01-01`

### T01-01 — Make legacy inspection nonmutating and establish ownership

- **Task ID / phase / slice:** `T01-01` / `P01` / `VS01`
- **Objective:** Open existing format-1 data safely and establish the new Core ownership boundary.
- **Why it exists:** Current open_read may create metadata, startup writes precede ownership, and raw event methods bypass writer binding.
- **Dependencies:** `T00-02`
- **Files/components:** src/vault.rs, src/events.rs, src/locator.rs, src/cli.rs; new storage/ownership module and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Separate readonly inspection from creation/recovery. Prevent all product format-1 writes after the new route is introduced; retain historical experimental code only where required without using it for v1 writes. Implement stable OS-held writer and recovery coordination with validated root handles; inventory and close public bypasses. Missing/unknown guard, stale diagnostic PID and path aliases must never authorize creation or lock theft.
- **Security considerations:** S03/S06; validate root/control-directory handles and native reparse behavior; same-user malicious process remains outside isolation claim.
- **Data invariants:** I02/I03/I09: every readonly invocation leaves canonical bytes unchanged; no creation, repair or metadata migration.
- **UX behavior:** Readonly inspection works or returns MissingMetadata/UnsupportedFormat/Busy/RecoveryRequired with a useful next action.
- **Failure behavior:** F02/F05/F16/F19; no automatic stale-lock deletion or path repair.
- **Acceptance criteria:** All nominal readers preserve a before/after byte inventory, missing guard is not created, two writers cannot coexist, crash releases the OS lease and recovery cannot race a reader.
- **Tests:** V01-V03, V05/V07/V09; public API audit, two-process contention, missing metadata, alias/reparse and reader-during-recovery fixtures.
- **Verification method:** Common Rust checks plus named filesystem snapshot and multiprocess integration tests on disposable roots.
- **Evidence artifact:** `docs/evidence/flake-v1/T01-01/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Readonly open and CLI startup budgets; no full journal allocation.
- **Durability gate:** D3 ownership/crash behavior and no mutation under failed open.
- **Cross-platform gate:** Native development profile mandatory now; all remaining profiles retained for T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T01-02`

### T01-02 — Create an independently specified format-2 transaction store

- **Task ID / phase / slice:** `T01-02` / `P01` / `VS01`
- **Objective:** Initialize a new isolated canonical store with the selected engine and published logical format.
- **Why it exists:** A single transaction removes the current object/event split without a custom write-ahead protocol.
- **Dependencies:** `T01-01`
- **Files/components:** src storage/vault modules, Cargo.toml/Cargo.lock only admitted changes, docs/formats/ and new format fixtures.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Admit and pin maintained rusqlite/SQLite versions. Create guard and database in a new staging root; publish no-clobber only after verification. Enforce DELETE/EXTRA, foreign keys, trusted schema off, defensive schema/SQL restrictions and bounded engine settings. Publish exact format tables/payload encoding and generic-reader examples. Store artifacts/receipts within canonical DB, not an external blob side channel.
- **Security considerations:** S04/S09; reject unexpected executable schema, extensions, ATTACH and caller SQL; inspect build features/scripts and effective SQLite version/settings.
- **Data invariants:** I03/I04/I10: guard/DB identity agrees, old binary refuses format 2, no existing directory is overwritten.
- **UX behavior:** Create reports a selected local location and verified created state; interrupted staging is labeled incomplete.
- **Failure behavior:** F01/F05/F15/F19; never fall back to weaker sync or old writer.
- **Acceptance criteria:** New vault reopens with generic SQLite inspection; required settings are asserted; incompatible or unexpected schema refuses writes; interrupted creation leaves old paths intact.
- **Tests:** V01-V03/V05/V06/V09/V10: fresh creation, every initialization failure stage, guard mismatch, old-reader refusal and malformed DB admission.
- **Verification method:** Common Rust checks; SQLite version/pragma capture; generic schema/content inspection and byte inventory.
- **Evidence artifact:** `docs/evidence/flake-v1/T01-02/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** S/M startup and bounds; choose page/cache only within section 13 options with recorded measurement.
- **Durability gate:** D2/D5 initialization publication and persisted guard/database agreement.
- **Cross-platform gate:** Native development profile; SQLite/native adapter qualification on all profiles at T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T01-03`

### T01-03 — Commit save, full history and command result atomically

- **Task ID / phase / slice:** `T01-03` / `P01` / `VS01`
- **Objective:** Save a minimal record with one durable, reconcilable outcome.
- **Why it exists:** Current replacement and journal append are separate and acknowledgement can ignore errors.
- **Dependencies:** `T01-02`
- **Files/components:** Core transaction/admission API, CLI save path, revision/history/command tables and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Bind private mutators to owning vault writer. Validate expected revisions and command UUID/digest; atomically persist full new payload/history/current pointers/event/head/result. Allocate real UTC observation timestamps and monotonic recorded sequence. Duplicate exact request returns the original result; changed digest under same ID rejects. Expose saved/not-saved/outcome-unknown distinctions.
- **Security considerations:** S05/S06; actor/provenance assigned by Core, no public raw append escape.
- **Data invariants:** I04/I05/I07: no new ID overwriting an existing object; all accepted payload versions recoverable; no fake timestamp or path-as-vault-ID.
- **UX behavior:** CLI shows command ID/result; UI-ready response contract never labels an unacknowledged buffer Saved.
- **Failure behavior:** F01/F15/F21; reconcile unknown commit outcomes before retry, never issue a replacement ID automatically.
- **Acceptance criteria:** Every storage boundary yields either complete pre-state or complete committed state; repeated request changes state exactly once; full history reconstructs current state and head.
- **Tests:** V01-V07/V09: state-machine properties, expected-revision conflicts, lost acknowledgement, flush/commit errors and process interruption.
- **Verification method:** Common Rust checks plus deterministic storage fault adapter and child-process termination schedule; inspect reopened DB independently.
- **Evidence artifact:** `docs/evidence/flake-v1/T01-03/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Save/write acknowledgement and memory ceilings at M; no O(history) append.
- **Durability gate:** D1/D2 all commit/ack boundaries, zero false Saved outcomes.
- **Cross-platform gate:** Development native gate now; exact same matrix on all profiles T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T01-04`

### T01-04 — Preserve forensic bytes and recover to a verified new root

- **Task ID / phase / slice:** `T01-04` / `P01` / `VS02`
- **Objective:** Recover an interrupted or damaged vault without destroying the original evidence.
- **Why it exists:** Current torn-tail repair normalizes/replaces original log bytes and ignores sync failures.
- **Dependencies:** `T01-03`
- **Files/components:** Core recovery/inspection modules, CLI recover/verify interfaces, incident manifest and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Acquire recovery ownership and exclude normal connections. Preserve complete guard/DB/journal before any engine recovery. Recover only a working copy; verify integrity, references, history/head and reconstructed current state. Publish a new root only after verification. On failed preservation or unverifiable state, leave original untouched and offer backup restore/partial salvage labeling.
- **Security considerations:** S05/S10; incident paths are owner-selected and handle-validated; diagnostics exclude content.
- **Data invariants:** I09/I12: preserve raw bytes/terminators, no invented records, valid prefix is not complete-history proof.
- **UX behavior:** Recovery report names what was preserved, verified, lost/unknown and the new destination; no ambiguous success banner.
- **Failure behavior:** F02/F03/F15/F18; partial salvage never becomes a complete writable vault.
- **Acceptance criteria:** All corrupt/unclean fixtures retain original digests; recoverable fixtures restore exact committed history; irrecoverable fixtures refuse complete publication.
- **Tests:** V03/V05-V07/V09/V12: corruption, missing journal, failing forensic copy, concurrent read, second crash during recovery and bad head fixtures.
- **Verification method:** Named recovery matrix over disposable copies with before/after manifests and independent logical comparison.
- **Evidence artifact:** `docs/evidence/flake-v1/T01-04/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Recovery/full verify budgets plus separately reported forensic copy duration; bounded streaming.
- **Durability gate:** D2/D4/D5 and interrupted recovery, zero overwrite of original evidence.
- **Cross-platform gate:** First native development report; profile-specific recovery results mandatory at T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T01-05`

### T01-05 — Create and restore consistent verified backups

- **Task ID / phase / slice:** `T01-05` / `P01` / `VS02`
- **Objective:** Make an owner-controlled backup that can restore complete saved work.
- **Why it exists:** Copying a live database file is not a consistent backup and same-disk success is not independent resilience.
- **Dependencies:** `T01-04`
- **Files/components:** Core backup/restore modules, CLI, snapshot and publication tests, recovery guide.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Use admitted SQLite snapshot/backup API; write staging members and full manifest, verify reopened destination, then no-clobber publish. Record snapshot head and later backup result separately. Restore to a new root, invalidate derived state and verify all canonical bytes/history. Cancellation before publication leaves explicit incomplete output.
- **Security considerations:** S03/S07/S10; export history/plaintext disclosure and destination ownership checks.
- **Data invariants:** I09/I10: backup includes all canonical revisions/artifacts/receipts; no derived authority or recursive self-inclusion.
- **UX behavior:** Show last verified backup head/time, destination and plaintext/history scope; distinguish backup from same-disk convenience copy.
- **Failure behavior:** F15/F17/F18; active vault and prior backups remain untouched.
- **Acceptance criteria:** Concurrent legitimate save cannot create mixed snapshot state; restored head/content matches manifest; cancelled/partial destination never reports Verified.
- **Tests:** V03-V08/V09: concurrent snapshot, disk-full, cancellation, destination collision, backup-after-recovery and restore failure.
- **Verification method:** End-to-end backup/restore into empty directories, member hashes and independent SQL reconstruction.
- **Evidence artifact:** `docs/evidence/flake-v1/T01-05/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** M backup/export/restore timing and memory limits; long operations report progress.
- **Durability gate:** D5 every backup/publication/restore stage.
- **Cross-platform gate:** Development native profile; clean cross-profile transfer tested later.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T01-06`

### T01-06 — Import legacy vaults without rewriting accepted history

- **Task ID / phase / slice:** `T01-06` / `P01` / `VS02`
- **Objective:** Move valid format-1 work into a separate format-2 vault with honest provenance.
- **Why it exists:** A storage transition must not manufacture lost revisions or mutate sealed experimental bytes.
- **Dependencies:** `T01-05`
- **Files/components:** Legacy readonly parser, migration/import module, CLI migration preview, docs/formats and fixtures.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Read nonmutating snapshots; validate IDs/frontmatter/log consistency and preserve exact raw legacy bytes. Produce complete mapping/omissions report before admission. Import valid selected records with migration origin; do not authenticate legacy log claims or pretend to recover absent history. Original root unchanged; ambiguous complete migration refuses. Partial selected import is explicit unconfirmed evidence in a new root.
- **Security considerations:** S03/S04/S05/S10; hostile or duplicate identity data cannot overwrite a live vault or import authority.
- **Data invariants:** I03/I05/I09/I10: original IDs retained only when unambiguous; unknown fields and original bytes preserved; provenance gaps visible.
- **UX behavior:** Preview counts, collisions, unknown provenance and destination; no automatic in-place migration.
- **Failure behavior:** F05/F06/F07/F10; complete or explicitly partial result, never silent skip.
- **Acceptance criteria:** Valid gold fixtures preserve exact payloads and identity mappings; corrupt/duplicate cases cannot report complete migration; originals hash-identical after success/failure.
- **Tests:** V03/V05-V10: exact historical-format fixtures in new namespace, CRLF/Unicode/unknown fields, partial history, interrupted migration.
- **Verification method:** Migration dry-run and confirmed new-root import; compare manifests and reconstruct with generic reader.
- **Evidence artifact:** `docs/evidence/flake-v1/T01-06/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** M import/memory bounds; stream legacy logs with full-record limits.
- **Durability gate:** D5 migration interruption and original preservation.
- **Cross-platform gate:** Development profile; cross-platform path/line-ending fixtures and later native run.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T01-07`

### T01-07 — Close the corrective save/recovery gate on a native host

- **Task ID / phase / slice:** `T01-07` / `P01` / `VS02`
- **Objective:** Demonstrate the first interruption-safe slice before adding product breadth.
- **Why it exists:** Unit tests cannot substitute for ownership, persistence and recovery observations.
- **Dependencies:** `T01-06`
- **Files/components:** New fault/child-process harness, disposable fixtures, Spec 002 corrective verification and evidence.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Run D1-D5 at the development native profile with exact tested commit, engine and filesystem. Audit every canonical mutator and reader against T00-02 inventory. Run at least 100 deterministic process-fault schedules per operation and all named boundary cases. Preserve failures. Record remaining D6/all-profile prerequisites explicitly for T05-02.
- **Security considerations:** S03-S06/S10 source review with zero alternate writer route and no recovery-before-preservation.
- **Data invariants:** I04/I09/I12; scope claims to tested native profile, no universal power-loss statement.
- **UX behavior:** Produce a concise supported/unsupported recovery report suitable for later user documentation.
- **Failure behavior:** Any lost acknowledged state or false complete result reopens its owning task; no P02 advance.
- **Acceptance criteria:** All P01 contracts and first native fault matrix pass; source inventory closed; Spec 002 corrective acceptance recorded prospectively, old acceptance unchanged.
- **Tests:** V01-V10/V12/V13 appropriate to P01; no physical power-loss claim without D6.
- **Verification method:** Full new deterministic suite and exact native process/fault commands in manifest-backed evidence.
- **Evidence artifact:** `docs/evidence/flake-v1/T01-07/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** M save/open/verify/recovery and memory ceilings pass; raw distributions retained.
- **Durability gate:** D1-D5 first native proof; D6 remains a named release dependency.
- **Cross-platform gate:** One actual native development profile required; other native profiles not represented as tested.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T02-01`

### T02-01 — Create and manage the four project record types

- **Task ID / phase / slice:** `T02-01` / `P02` / `VS03`
- **Objective:** Create/open/archive a project with stable typed work records.
- **Why it exists:** A small common model is needed for the first user loop without a universal schema platform.
- **Dependencies:** `T01-07`
- **Files/components:** Core project/note/action/decision model, CLI typed commands, format specification and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Implement section 15 common envelope and four types through the transaction API. Validate field/record limits, project membership and immutable IDs. Project archive and tombstones preserve history; paths remain locator labels. Record schema capability compatibility and unknown optional payload preservation.
- **Security considerations:** S04/S06; type tags and supplied trust fields cannot bypass admission.
- **Data invariants:** I03/I05/I07; one owning project per work record; no orphaned canonical references.
- **UX behavior:** One project name starts useful work; no workspace schema designer or session setup.
- **Failure behavior:** F05/F07/F20; invalid state rejected before transaction, existing state unchanged.
- **Acceptance criteria:** Create/open/archive/unarchive and typed record CRUD-as-revision work through CLI; invalid cross-project references and ID collisions reject.
- **Tests:** V01-V05/V09/V10: generated valid/invalid record combinations, unknown fields, lifecycle and cross-project references.
- **Verification method:** Common Rust checks plus CLI round-trip fixtures and generic format inspection.
- **Evidence artifact:** `docs/evidence/flake-v1/T02-01/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** S/M project open/read and bounds.
- **Durability gate:** All mutations use proven command transaction; D1 regression sample for each new type.
- **Cross-platform gate:** Portable model tests on available host; all profiles at T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T02-02`

### T02-02 — Capture exact notes and selected local evidence

- **Task ID / phase / slice:** `T02-02` / `P02` / `VS03`
- **Objective:** Save a note or selected file snapshot with clear origin and recoverable bytes.
- **Why it exists:** Fast capture and lossless evidence are the first value prerequisites.
- **Dependencies:** `T02-01`
- **Files/components:** Core capture/source/artifact admission, CLI file selection, Markdown parser/preview contract and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Note default, explicit Action/Decision selection, title/body bounds. Import only selected regular files through validated handles; no recursive home/repository scan, symlinks or secret-file defaults. Store original bytes/digest/time and unknown origin label; no HTML execution, network fetch or auto-promotion. Binary artifacts remain opaque and bounded.
- **Security considerations:** S01/S03/S04/S07; implement obvious-secret rejection and explain limits; no source-body logging.
- **Data invariants:** I05/I07/I11; exact payload bytes survive saves/import, observed time not invented source time.
- **UX behavior:** Capture needs no evidence; imported evidence clearly labeled. Failed save retains unsaved text and reports outcome.
- **Failure behavior:** F01/F07/F08/F10/F15; invalid content cannot silently truncate or become accepted truth.
- **Acceptance criteria:** Text/Unicode/CRLF/opaque artifact fidelity exact; rejected inputs leave no partial record; no unseen filesystem/network access.
- **Tests:** V02-V07/V09: boundary sizes, IME-relevant text, binary fixtures, prohibited selected paths, secret sentinels and process failure.
- **Verification method:** CLI capture/import E2E, before/after manifests, controlled filesystem access observations.
- **Evidence artifact:** `docs/evidence/flake-v1/T02-02/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Capture/save and per-input limits; bounded reads after open, not metadata-only checks.
- **Durability gate:** D1/D2 per capture and artifact transaction.
- **Cross-platform gate:** Native path handling on development profile; remaining profiles T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T02-03`

### T02-03 — Record decisions and complete actions with visible history

- **Task ID / phase / slice:** `T02-03` / `P02` / `VS03`
- **Objective:** Accept an evidence-linked choice and finish/reopen work without hiding disagreement.
- **Why it exists:** A note list alone does not explain why work changed or whether a completion was intentional.
- **Dependencies:** `T02-02`
- **Files/components:** Core decision/action/relation commands, CLI detail/history, temporal value validation and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Implement exact lifecycle transitions, decision key/valid interval, optional explicit unlinked user judgment, dependency validation and acyclic supersession. Completion requires summary; blocked-dependency completion requires a reasoned override. User overrides preserve conflicting/negative evidence. Agent-origin input remains draft until owner acceptance.
- **Security considerations:** S05/S06; owner-only acceptance and Core-owned recorded sequence.
- **Data invariants:** I05/I07; basis/verification/lifecycle/resolution remain separate; no rank-derived supersession.
- **UX behavior:** Show accepted/draft/withdrawn state, evidence links, negative evidence, prior completion and override rationale.
- **Failure behavior:** F09/F10/F20/F21; conflicting expected revision fails visibly rather than overwriting.
- **Acceptance criteria:** All allowed transitions produce exact immutable history; forbidden transitions/cycles reject; explicit judgment and override remain visible.
- **Tests:** V02-V05/V07/V09: lifecycle table, overlap/dependency cases, stale expected revision and lost ack.
- **Verification method:** CLI create/accept/complete/reopen/history scenario and state-machine property tests.
- **Evidence artifact:** `docs/evidence/flake-v1/T02-03/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** M state-change/read budgets, bounded dependency traversal.
- **Durability gate:** D1 regression for every new transition family.
- **Cross-platform gate:** Semantic tests portable; native transaction behavior inherited.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T02-04`

### T02-04 — Find work with a disposable current lexical index

- **Task ID / phase / slice:** `T02-04` / `P02` / `VS03`
- **Objective:** Search captured work without making index contents authoritative.
- **Why it exists:** Current rebuild deletes/inserts without an atomic generation and can present stale data as current.
- **Dependencies:** `T02-03`
- **Files/components:** src/derived.rs or replacement FTS module, canonical query interface, CLI search/rebuild and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Use FTS5 separate DB, literal query escaping, canonical filters and post-candidate eligibility. Increment from canonical transaction sequence; publish index generation/checkpoint atomically only after success. Full rebuild and incremental results must match. Detect missing/corrupt/lagging index; provide visible bounded canonical fallback or SearchUpdating, never false NoResults. No filesystem watcher or graph registry.
- **Security considerations:** S02/S04; index project/path/revision hints cannot authorize reads or infer absence.
- **Data invariants:** I03/I06/I11; derived deletion cannot remove content, user relations or accepted state.
- **UX behavior:** Search type/state/project filters, snippets and source revision; visibly label lag and permit explicit rebuild.
- **Failure behavior:** F04/F15; interrupted rebuild retains previous usable generation with lag label.
- **Acceptance criteria:** Clean/incremental results equivalent for all generated mutation sequences; index corruption never leaks another project or changes canonical state.
- **Tests:** V02-V07/V09/V13: index deletion/corruption, stale checkpoints, generation interruption and literal-query cases.
- **Verification method:** Named rebuild-vs-incremental oracle comparing IDs/revisions/rank tie policy; canonical byte comparison.
- **Evidence artifact:** `docs/evidence/flake-v1/T02-04/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** M search/rebuild/RSS and L bounded batch ceilings.
- **Durability gate:** Derived-generation crash publication, canonical bytes unchanged.
- **Cross-platform gate:** Available-host evidence now; all native profiles at T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T02-05`

### T02-05 — Export complete owned state with a verified manifest

- **Task ID / phase / slice:** `T02-05` / `P02` / `VS04`
- **Objective:** Leave Flake with readable content and complete declared history.
- **Why it exists:** An export that omits source revisions, receipts or unknown payloads is not ownership.
- **Dependencies:** `T02-04`
- **Files/components:** Core portable exporter, public format docs, CLI export preview and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Implement full-backup and selected-project package types, exact members and omissions, safe generated paths and no-clobber staging publication. Include generic-reader instructions and plain Markdown rendering. Keep raw canonical payload bytes where JSON reserialization would alter them. Sensitive history and scope shown before writing; selected export is not labeled a full backup.
- **Security considerations:** S02/S03/S07/S10; export only authorized selection, no local locator/grant authority leakage in shareable package.
- **Data invariants:** I05/I10; manifest snapshot head excludes its own later export event; bytes/counts/digests complete.
- **UX behavior:** Owner reviews full-history/plaintext inclusion, chooses new destination, sees Verified only after reopened validation.
- **Failure behavior:** F15/F17; partial output remains staging and never overwrites a prior export.
- **Acceptance criteria:** Manifest covers every declared canonical member and all exclusions; generic JSON/byte reader validates exports with no Flake dependency.
- **Tests:** V03-V07/V09/V13: Unicode/unsafe labels, partial/cancelled export, destination collision, selected-scope privacy and unknown fields.
- **Verification method:** E2E full and project exports, independently enumerate/hash members and compare canonical logical state.
- **Evidence artifact:** `docs/evidence/flake-v1/T02-05/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** M export ≤180 s, L streaming/cancel and RSS ceilings.
- **Durability gate:** D5 all publication stages; original vault unchanged.
- **Cross-platform gate:** Cross-OS-safe member names now; native publication profiles T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T02-06`

### T02-06 — Validate and import portable packages into a new vault

- **Task ID / phase / slice:** `T02-06` / `P02` / `VS04`
- **Objective:** Recreate work from an export while treating imported authority claims as untrusted.
- **Why it exists:** Portability is incomplete without safe ingress and explicit conflict semantics.
- **Dependencies:** `T02-05`
- **Files/components:** Core portable import/validation, CLI preview, staging vault and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Read bounded directory-format members only; no arbitrary archive extraction in v1. Validate paths, lengths, digests, schema, capability, identity and references before publication. Full restore preserves declared identity only into a new root; selected merge imports propose new admission with origin mapping. Imported grants are inert history, all new grants require owner issuance. Colliding immutable IDs/digests reject; identical repeated import reconciles idempotently.
- **Security considerations:** S01-S07/S10; no imported SQL, executable content, credentials or live authorization.
- **Data invariants:** I03/I05/I09/I10; exact raw content and unknown optional fields preserved; partial source is labeled partial.
- **UX behavior:** Preview scope/counts/conflicts/unknowns and ask explicit owner confirmation for the concrete import.
- **Failure behavior:** F05/F07/F10/F15/F18; active vault untouched until fully validated transaction/new-root publication.
- **Acceptance criteria:** Valid package reconstruction exact, all malformed/ambiguous cases refuse complete success, imported grants cannot disclose content.
- **Tests:** V03/V05-V10/V13: limits/traversal/duplicate IDs/unknown required fields/missing members/failed publication.
- **Verification method:** Round-trip exports into empty roots and partial/new-origin cases; independent manifest and state comparisons.
- **Evidence artifact:** `docs/evidence/flake-v1/T02-06/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** M/L import and parser/RSS ceilings; no whole-package allocation.
- **Durability gate:** D5 interruption at validation/staging/commit/publication.
- **Cross-platform gate:** Development host and cross-platform fixture names; native transfer T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T02-07`

### T02-07 — Prove project reconstruction without Flake

- **Task ID / phase / slice:** `T02-07` / `P02` / `VS04`
- **Objective:** Demonstrate the complete create/capture/find/complete/export loop and independent exit.
- **Why it exists:** A product-owned export/import pair can share the same omission bug.
- **Dependencies:** `T02-06`
- **Files/components:** Standalone generic-reader verification tool, public format documentation, disposable project fixtures and evidence.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Have an independent executor/tool reconstruct from exported JSON/bytes and separately inspect canonical SQLite without linking Core or using its importer. Recompute history/current state/relations, verify artifact/receipt bytes and unknown fields. Run the whole CLI user loop with derived DB removed. Document how owners recover without Flake.
- **Security considerations:** S05/S10; compare against retained expected manifest/head, not an unanchored self-consistent prefix.
- **Data invariants:** I06/I10/I12: full declared state is readable without vendor/runtime, index or agent.
- **UX behavior:** A user can follow published instructions to inspect and extract all declared owned state.
- **Failure behavior:** Any missing member/unsupported invariant reopens export/schema work; no P03 advance.
- **Acceptance criteria:** Independent logical and byte comparison is exact on all full fixtures; intentional selected-scope omissions are explicit; CLI loop succeeds offline.
- **Tests:** V04/V08/V09/V15; independent code path and exact original-data oracle, not copied product serializer.
- **Verification method:** Record independent executor/tool source, inputs and commands; compare canonical digests and complete counts.
- **Evidence artifact:** `docs/evidence/flake-v1/T02-07/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Report independent-reader time and memory; product M export/import hard gates still apply.
- **Durability gate:** Restore and derived-deletion proof with retained originals.
- **Cross-platform gate:** At least one actual host now; cross-platform independence repeated T05-05.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T03-01`

### T03-01 — Track selected source revisions and relocation

- **Task ID / phase / slice:** `T03-01` / `P03` / `VS05`
- **Objective:** Show whether saved evidence still matches a user-selected local source.
- **Why it exists:** Repository moves, file deletion and unreachable commits must not erase why a decision was made.
- **Dependencies:** `T02-07`
- **Files/components:** Source-check/revision/locator Core modules, CLI evidence commands and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Explicitly recheck previously authorized selected regular files; bind observed bytes through the opened handle. Record Match/Changed/Missing/Denied/Unchecked with time/digest; import changed bytes as a new revision only with owner admission. Reselection after move verifies digest and preserves source identity/history. Commit/URL labels are declared references unless exact local bytes independently establish the claim. No Git subprocess, remote fetch or watcher.
- **Security considerations:** S03/S07; one selected source path does not authorize its repository, parent directory or credentials.
- **Data invariants:** I03/I05/I07: source snapshot survives disappearance; source time precision retained; path is not identity.
- **UX behavior:** Matches when checked, Changed, Missing and Unchecked labels; show affected evidence-linked decisions.
- **Failure behavior:** F08/F09/F14/F16; no automatic locator remap or currentness assertion.
- **Acceptance criteria:** All four freshness states and moved/deleted-source scenarios show correct immutable history; no out-of-selection reads.
- **Tests:** V02-V05/V09: same/different bytes, rename/replacement races, partial source timestamps and absent commit source.
- **Verification method:** Selected-file E2E observations with retained digest oracle and filesystem access record.
- **Evidence artifact:** `docs/evidence/flake-v1/T03-01/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Canonical/resume budgets; external file check progress/cancel; bounded selected-file reads.
- **Durability gate:** New checks/revisions use atomic transaction and survive source loss.
- **Cross-platform gate:** Native selected-handle behavior on each profile by T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T03-02`

### T03-02 — Resolve explicit temporal state and disagreement deterministically

- **Task ID / phase / slice:** `T03-02` / `P03` / `VS05`
- **Objective:** Compute current accepted project state without hiding conflict or negative evidence.
- **Why it exists:** Temporal rank, user acceptance and evidence freshness are different facts.
- **Dependencies:** `T03-01`
- **Files/components:** src/temporal.rs and memory values or successor decision resolver, Core state queries and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Apply canonical project, lifecycle, recorded cutoff and valid interval to all applicable records. Resolve only explicit accepted supersession; incomparable overlapping decisions with same key produce Conflict. Unknown/partial times cannot be invented as exact intervals. Keep user override reason and negative evidence in output. Stable deterministic ordering across processes.
- **Security considerations:** S02/S05; index rank and external annotations cannot select truth or expand scope.
- **Data invariants:** I05-I08; no latest-wins, no model adjudication, no loss of basis/verification/lifecycle distinctions.
- **UX behavior:** Return CurrentSet, NeedsReview or NoAcceptedDecision with reasons and source state, not misleading confidence.
- **Failure behavior:** F09/F10/F20; invalid supersession rejected, unresolved conflict stays visible.
- **Acceptance criteria:** 100% exact results on held-out temporal/conflict cases and deterministic repeat runs; cross-project/time-excluded records absent.
- **Tests:** V02-V05/V09: valid/recorded time grid, future/backdated assertions, partial dates, cycles, ties, override and contradiction.
- **Verification method:** Pure reference oracle separate from production resolver; record generated seed and cross-process equality.
- **Evidence artifact:** `docs/evidence/flake-v1/T03-02/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** M resume/read latency, bounded resolution per project.
- **Durability gate:** Resolution derived only; canonical bytes remain unchanged.
- **Cross-platform gate:** Cross-platform deterministic semantic output comparison at T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T03-03`

### T03-03 — Build the owner resume view and explicit checkpoint

- **Task ID / phase / slice:** `T03-03` / `P03` / `VS05`
- **Objective:** Let a returning owner see changed evidence, accepted decisions and next actions.
- **Why it exists:** The primary continuity job needs a composed end-to-end view before UI investment.
- **Dependencies:** `T03-02`
- **Files/components:** Core resume query, CLI resume/checkpoint/history and golden user scenarios.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Compose section 12 ordering from canonical eligibility/resolution and bounded search support. Show changes since explicit reviewed-through sequence, current head, missing/changed evidence, unresolved conflicts and next actions. Mark reviewed only on explicit owner command; opening/searching is not a checkpoint. Stable resume output at the same snapshot.
- **Security considerations:** S05/S06; no automatic decision acceptance, source promotion or silent checkpoint write.
- **Data invariants:** I05-I08; checkpoint sequence bound to project/history, conflicts precede reassuring summary.
- **UX behavior:** Returning user can answer what changed, why this decision and what next without reading raw storage.
- **Failure behavior:** F04/F08-F10/F20; unavailable index/source produces honest degraded view, not false certainty.
- **Acceptance criteria:** Scripted interruption/resumption cases expose all planted changes/conflicts and correct next actions with no hidden prerequisites.
- **Tests:** V03-V05/V09/V13: checkpoint reset, stale index, deleted evidence, action reopen, deterministic repeat snapshot.
- **Verification method:** Actual CLI workflow against saved/mutated external-source scenarios; oracle checks every visible state.
- **Evidence artifact:** `docs/evidence/flake-v1/T03-03/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** M resume p95 and wire bounds; no full-vault scan hidden in project view.
- **Durability gate:** Checkpoint is a normal atomic owner command; resume query itself readonly.
- **Cross-platform gate:** Available-host proof now; same output across profiles later.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T03-04`

### T03-04 — Issue scoped grants and persist exact disclosure receipts

- **Task ID / phase / slice:** `T03-04` / `P03` / `VS06`
- **Objective:** Export a bounded package whose every byte has explicit scope and provenance.
- **Why it exists:** Current ordinary records bypass project filtering and the latest-manifest file is not durable replay evidence.
- **Dependencies:** `T03-03`
- **Files/components:** Core grants/eligibility/compiler/receipt store, CLI package preview/export and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Implement section 18 owner-issued grants, expiry/revocation, type/ID restrictions and privacy exclusions. All item types/referenced metadata use canonical checks. Persist receipt and exact wire bytes before emission; deterministic byte accounting includes envelope metadata. Stage/verify package output; reconcile lost result through command ID. No raw agent read endpoint or imported live grant.
- **Security considerations:** S01/S02/S07; zero disallowed body/ID/path bytes, no source content as authority; do not expose grant bearer material.
- **Data invariants:** I04/I05/I08; exact historical receipt binding to snapshot/policy/grant/revisions, no last-manifest overwrite.
- **UX behavior:** Owner sees disclosure scope, budget, exclusions and plaintext warning before export; truncation/omission explicitly labeled.
- **Failure behavior:** F11/F15/F17/F21; failed receipt persistence emits no package; lost package output may be re-exported from the same receipt only after owner confirmation and current matching grant revalidation. Receipt replay alone grants no authority.
- **Acceptance criteria:** Mixed-project fixtures never disclose excluded material; repeated identical request bytes match; revoked/expired grant fails; full receipt replay verifies exact bytes.
- **Tests:** V02-V07/V09/V13: all record types, metadata-only leaks, envelope budget boundaries, expiry/revocation, crash-before/after receipt and emission.
- **Verification method:** Byte-level disclosure oracle against canonical allowlist and persisted receipt; compare exact UTF-8 wire length/digest.
- **Evidence artifact:** `docs/evidence/flake-v1/T03-04/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** 256 KiB cap and M resume/export latency/RSS; overflow reason counts required.
- **Durability gate:** D1 receipt-before-emission and D5 package publication.
- **Cross-platform gate:** Native output publication/encoding on all profiles by T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T03-05`

### T03-05 — Review and admit bounded agent proposals

- **Task ID / phase / slice:** `T03-05` / `P03` / `VS06`
- **Objective:** Accept selected external suggestions only through explicit owner review.
- **Why it exists:** Agent independence is useful only if returned state cannot overwrite authority or stale work.
- **Dependencies:** `T03-04`
- **Files/components:** Core proposal parser/admission/review state, CLI diff/accept/reject and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Validate ≤1 MiB/100 operations, protocol/receipt binding, declared identity, operation allowlist, expected revisions and evidence references. Keep raw bounded proposal immutable and pending; owner-selected acceptance is one normal atomic command. Recheck grant validity/current scope and revisions at acceptance; reject stale operations, do not auto-rebase. Exact replay returns original result; digest collision under same ID rejects.
- **Security considerations:** S01/S04/S06/S07; proposals cannot issue grants, accept decisions, execute tools, migrate, restore or delete.
- **Data invariants:** I02/I04/I05/I08; owner transition and agent origin remain separate, unknown model/tool identity stays declared Unknown.
- **UX behavior:** Show exact proposed changes, claimed identity, evidence, conflicts and reasons; partial owner selection produces explicit selected-operation receipt.
- **Failure behavior:** F10-F12/F20/F21; malformed or stale proposals leave accepted state untouched.
- **Acceptance criteria:** Only permitted owner-reviewed operations commit; replay changes once; conflict leaves pending/rejected history and original state; arbitrary instruction text is inert.
- **Tests:** V02-V07/V09: protocol negative corpus, unexpected fields, stale references, mixed allowed/forbidden batch and duplicate delivery.
- **Verification method:** CLI package→external fixture proposal→review→accept/reject E2E, independent resulting-state oracle.
- **Evidence artifact:** `docs/evidence/flake-v1/T03-05/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Proposal limits and M transaction/read budgets; bounded parser nesting.
- **Durability gate:** D1 atomic selected-operation acceptance and lost-ack reconciliation.
- **Cross-platform gate:** Protocol portable; native persistence coverage inherited and repeated.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T03-06`

### T03-06 — Verify interchange with two independent offline clients

- **Task ID / phase / slice:** `T03-06` / `P03` / `VS06`
- **Objective:** Prove continuity survives replacing the external client.
- **Why it exists:** A single product-owned adapter can hide undocumented provider or serialization assumptions.
- **Dependencies:** `T03-05`
- **Files/components:** Versioned protocol docs, two independent fixture clients outside Core, interchange conformance evidence.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Client A reads package and proposes a note/action update; client B reads the same package/accepted history and continues with a compatible different proposal. Implement clients from public protocol only, without sharing Core encoder/decoder. Run both offline with no model/account. Verify unknown declared identity, replay and unsupported protocol behavior. Keep executable fixtures minimal, not an orchestration framework.
- **Security considerations:** S01/S02/S06; clients receive no filesystem/network/command authority from Flake.
- **Data invariants:** I08/I10/I12; protocol interoperability does not prove AI outcome superiority.
- **UX behavior:** Owner can swap clients while keeping understandable source/decision/proposal history.
- **Failure behavior:** F11/F12/F13/F21; incompatible client fails with version guidance, manual workflow continues.
- **Acceptance criteria:** Both independent clients pass byte/provenance/acceptance/replay contract and replacement continuation; no product/provider private state required.
- **Tests:** V04/V09/V10/V15; independent implementations, negative protocol cases and zero-network observation.
- **Verification method:** Record client sources/digests, offline execution commands, packages/proposals and exact accepted-state comparison.
- **Evidence artifact:** `docs/evidence/flake-v1/T03-06/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Package/proposal ceilings apply; client latency reported separately, never part of Core performance claim.
- **Durability gate:** Receipts and accepted proposals survive client deletion and restart.
- **Cross-platform gate:** At least one host now; protocol cross-platform paths and bytes rechecked later.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T03-07`

### T03-07 — Preregister the local continuity proof

- **Task ID / phase / slice:** `T03-07` / `P03` / `VS07`
- **Objective:** Freeze fair cases, allocation, metrics and routing before observing product outcomes.
- **Why it exists:** Historical state/budget/fallback problems must not recur in a new namespace.
- **Dependencies:** `T03-06`
- **Files/components:** New bench/flake-v1 protocol/case manifests, consent/privacy plan, measurement and analysis specifications.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Implement section 26 six-participant/eight-pair design, held-out gold, counterbalancing, equal source/setup/access budgets and all maintenance costs. Seal product/harness/case/analysis hashes before execution. Define exact success/timeout/exclusion rules, class-loss route and one allowed disjoint repair repeat. Baseline uses maintained Markdown/index/task list, not an intentionally weak raw dump. No model execution or private data publication.
- **Security considerations:** S07; opt-in data minimization and consented evidence; source content never enters public logs by default.
- **Data invariants:** I12; raw run manifests include actual harness and previous state, no score-based authority.
- **UX behavior:** Participants receive equal training and understand optional participation and local data control.
- **Failure behavior:** Missing participants/gold/budget enforcement prevents trial start; protocol drift invalidates affected run visibly.
- **Acceptance criteria:** Every outcome can be routed mechanically to Pass/Fail/Inconclusive; independent reviewer can reconstruct pairing and total-time calculation before results.
- **Tests:** V01/V05/V09/V15 protocol/dry-run validation using synthetic non-evaluation cases only.
- **Verification method:** Dry-run collection/analysis scripts and independent arithmetic checks; seal exact artifacts and no-API execution path.
- **Evidence artifact:** `docs/evidence/flake-v1/T03-07/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Measurement overhead recorded; timing budgets and participant time limits fixed before observations.
- **Durability gate:** Raw records append with resumable state manifest and no silent lost attempts.
- **Cross-platform gate:** Record study host profiles; claims limited to measured environment.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T03-08`

### T03-08 — Run the CLI continuity proof and decide desktop entry

- **Task ID / phase / slice:** `T03-08` / `P03` / `VS07`
- **Objective:** Determine whether the bounded continuity loop earns further investment.
- **Why it exists:** A polished interface should not conceal an unproven core workflow.
- **Dependencies:** `T03-07`
- **Files/components:** Preregistered local study runner/data/analysis and P03 acceptance report.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Execute exact registered cases through real CLI/Core. Preserve every attempt, failure and raw timing; validate actual budgets and state continuity before analysis. Apply all section 26 thresholds and report per-class/participant outcomes and limitations. If needed, one recorded defect-repair repeat uses a new seal and disjoint cases; a second failure/inconclusive result stops for reconsideration.
- **Security considerations:** S07 and all disclosure gates; no paid models or unconsented data export.
- **Data invariants:** I12; no equivalence claim from nonsignificance, no model outcome claim from humans/fixture clients.
- **UX behavior:** Report actual successful resumes and total maintenance cost, including participants who could not finish.
- **Failure behavior:** Failure/inconclusive means P04 blocked; preserve negative results and request only the required reconsideration.
- **Acceptance criteria:** ≥90% resume success, ≤5 percentage point deficit versus baseline, ≥20% lower paired total-time median, zero high-consequence missed conflict/disclosure and no all-failed class; exact data valid.
- **Tests:** V04/V09/V13-V15 and preregistration integrity verification.
- **Verification method:** Run registered commands; independent raw-to-summary calculation and all threshold checks with exact denominators.
- **Evidence artifact:** `docs/evidence/flake-v1/T03-08/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Section 26 value gate and section 27 Core maxima both pass; never trade correctness for time.
- **Durability gate:** Study interruption/state persistence inspected; prior storage gates remain green.
- **Cross-platform gate:** Claims explicitly scoped to measured native profiles; full release profiles still pending.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T04-01`

### T04-01 — Create a thin local desktop shell

- **Task ID / phase / slice:** `T04-01` / `P04` / `VS08`
- **Objective:** Open the proven project loop in a narrowly privileged desktop process.
- **Why it exists:** The desktop should improve access without creating a second correctness or authority model.
- **Dependencies:** `T03-08`
- **Files/components:** New desktop/ or src-tauri/ presentation package, React/TypeScript assets, typed Rust bridge, admitted package locks and security tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Admit Tauri 2/React and only necessary dependencies at exact maintained versions with rights/advisory/build-script review. Render local assets with explicit restrictive CSP and narrow typed commands. No filesystem/shell/http/process/SQL/opener plugins, remote resources, model SDK, localhost server or auto-update. Reuse Core, enforce input bounds and writer ownership. Add native dialog-mediated selection as a bounded Core capability.
- **Security considerations:** S01/S08/S09; imported content cannot mint bridge capabilities; preview and dialog paths separately reviewed.
- **Data invariants:** I02/I06; UI state/cache is derived and cannot mutate DB directly.
- **UX behavior:** First run Create/Open/Restore, local ownership/location explanation and project list; offline normal.
- **Failure behavior:** F05/F16/F19; useful Busy/RecoveryRequired/UnsupportedFormat states without bypass.
- **Acceptance criteria:** Built bundle exposes only named Core commands, starts offline with no external requests, and cannot render active imported content or execute arbitrary paths.
- **Tests:** V01-V04/V09/V11/V14: bridge validation, built-capability/CSP audit, clean offline launch and malicious-content fixtures.
- **Verification method:** Locked frontend install/typecheck/build, Rust checks, built asset/permission inspection and native network observation.
- **Evidence artifact:** `docs/evidence/flake-v1/T04-01/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Desktop cold/warm startup and RSS ceilings.
- **Durability gate:** No new persistence engine; first-run/close tests retain Core ownership semantics.
- **Cross-platform gate:** Native desktop launch on development host now; all profiles required at T04-06/T05-02.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T04-02`

### T04-02 — Capture and edit notes with honest save state

- **Task ID / phase / slice:** `T04-02` / `P04` / `VS08`
- **Objective:** Let owners write comfortably while preserving exact saved text.
- **Why it exists:** A minimal editor must still handle composition, undo and failure correctly.
- **Dependencies:** `T04-01`
- **Files/components:** Desktop capture/editor/preview components and typed save binding, UI tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Plain textarea with safe separate Markdown preview, type selector, keyboard save, undo/redo and IME composition. Keep unsaved buffer distinct from last durable revision. Save through shared command UUID/expected revision; error retains buffer, restart recovers committed result only. No rich block sidecar, autosave truth promotion or external link execution.
- **Security considerations:** S01/S08; HTML/remote media inert, clipboard paste treated as text, no unrequested clipboard reads.
- **Data invariants:** I04/I05; exact text bytes/Unicode semantics and saved revisions preserved; normalization policy explicit.
- **UX behavior:** Visible Unsaved/Saving/Saved/NotSaved/OutcomeUnknown, focus retained on failure and no false success.
- **Failure behavior:** F01/F15/F21; conflicting editor revision shows review options, never silent overwrite.
- **Acceptance criteria:** IME, undo, large allowed body, restart/lost ack and failure cases preserve expected text/state; preview causes zero network/process activity.
- **Tests:** V03/V04/V07/V09/V14: native keyboard/IME/Unicode/accessibility and save-boundary tests.
- **Verification method:** Scripted editor E2E with exact before/after canonical payload comparison and manual native composition checklist.
- **Evidence artifact:** `docs/evidence/flake-v1/T04-02/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Capture feedback, save acknowledgement and desktop RSS budgets.
- **Durability gate:** D1 save/restart integration and unsaved-buffer labeling; no unproven crash recovery of unsaved text.
- **Cross-platform gate:** Native composition/keyboard behavior across three profiles by T04-06.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T04-03`

### T04-03 — Search, inspect and complete project work in the desktop

- **Task ID / phase / slice:** `T04-03` / `P04` / `VS08`
- **Objective:** Complete the portable project loop without CLI knowledge.
- **Why it exists:** A shell alone is not a usable continuity product.
- **Dependencies:** `T04-02`
- **Files/components:** Desktop project/work/note/decision/evidence/history views, typed Core queries/actions and UI tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Implement section 12 navigation, search/filter/snippet states, action transitions, decision acceptance/judgment/override and archive/tombstone confirmation. Use the same record views/IDs, not copied UI-owned objects. No calendar/board-builder/topic suite. Preserve keyboard focus and readable non-color-only state labels.
- **Security considerations:** S06/S08; confirmations name concrete changes, Core revalidates expected revisions and ownership.
- **Data invariants:** I03/I05/I06; views do not change scope/state merely by opening.
- **UX behavior:** A user creates project, captures note, links evidence, accepts choice, completes/reopens action, searches and inspects history.
- **Failure behavior:** F04/F09/F20/F21; stale search and conflicting acceptance shown without hidden mutation.
- **Acceptance criteria:** Scripted complete workflow produces identical canonical state to CLI; empty, archived and failure states offer appropriate next actions.
- **Tests:** V03/V04/V09/V14: shared CLI/UI state oracle, keyboard-only flow, stale-index/conflict cases.
- **Verification method:** Desktop E2E commands plus canonical state export comparison against corresponding CLI scenario.
- **Evidence artifact:** `docs/evidence/flake-v1/T04-03/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Search/project detail/read budgets and accessible progress behavior.
- **Durability gate:** All transitions use existing transactional Core; D1 sample from UI.
- **Cross-platform gate:** Development native proof; complete profile matrix at T04-06.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T04-04`

### T04-04 — Present resumption and agent review as one evidence-linked flow

- **Task ID / phase / slice:** `T04-04` / `P04` / `VS09`
- **Objective:** Show what changed and let an owner safely exchange and review external contributions.
- **Why it exists:** Continuity value depends on understandable uncertainty, not just a compiler output.
- **Dependencies:** `T04-03`
- **Files/components:** Desktop Resume/checkpoint/source-status/package/proposal review views, Core bindings and tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Display current snapshot/as-of, changes since checkpoint, conflicts/negative evidence/source states, next actions and explicit reviewed-through action. Provide package scope/budget/privacy preview and grant controls; proposal diff shows claimed identity and acceptance conflicts. Reuse section 18 exact bytes/receipts; no chat/model-launch surface.
- **Security considerations:** S01/S02/S06/S08; imported proposal cannot impersonate owner UI or hide forbidden operations.
- **Data invariants:** I05/I07/I08; explicit override and stale evidence retained in every relevant view.
- **UX behavior:** User can resume, inspect why, mark reviewed, export a package and accept/reject a proposal with clear consequences.
- **Failure behavior:** F08-F14/F20/F21; missing client/network leaves manual workflow fully usable.
- **Acceptance criteria:** Every planted conflict/stale source appears before next-action confidence; desktop package/proposal result matches CLI; no accidental checkpoint/acceptance.
- **Tests:** V03/V04/V09/V14: stale/change/negative evidence, expired grant, malformed proposal and expected-revision conflict flows.
- **Verification method:** Scripted desktop/CLI parity and manual comprehension checks with predefined correct interpretations.
- **Evidence artifact:** `docs/evidence/flake-v1/T04-04/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Resume and package budgets, no UI truncation hiding required safety labels.
- **Durability gate:** Receipt-before-emission and atomic acceptance from desktop under interruption.
- **Cross-platform gate:** Native dialog/file exchange and screen-reader behavior across profiles T04-06.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T04-05`

### T04-05 — Expose backup, recovery, import and export safely

- **Task ID / phase / slice:** `T04-05` / `P04` / `VS09`
- **Objective:** Allow an owner to recover or leave using the desktop alone.
- **Why it exists:** Data ownership fails in practice if recovery requires undocumented command-line expertise.
- **Dependencies:** `T04-04`
- **Files/components:** Desktop data-management/recovery screens, native destination dialog bindings and user guide.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Show last verified backup and plaintext/history scope; preview import/export destination and omissions; support cancellation/progress and new-root recovery. Never offer in-place destructive repair, silent merge, erase-on-uninstall or automatic sync. Surface generic reconstruction instructions and precise unsupported-location behavior.
- **Security considerations:** S03/S07/S10; owner-selected destinations only, no-clobber and privacy preview.
- **Data invariants:** I09/I10; incomplete output never shown as verified, old vault/backups intact.
- **UX behavior:** Empty/corrupt/incompatible vault produces actionable readonly recovery screen; export/restore results name verified head and omissions.
- **Failure behavior:** F03/F05-F07/F15-F19; retain failed staging/evidence and offer concrete retry route.
- **Acceptance criteria:** Non-CLI user can create backup, restore to new location and full export/import; failed/cancelled paths clearly incomplete with originals unchanged.
- **Tests:** V04/V07-V11/V14: disk-full, permission denied, corrupt input, unsupported directory and interrupted operations.
- **Verification method:** Native scripted UI flows plus destination manifest/byte verification and original-vault comparison.
- **Evidence artifact:** `docs/evidence/flake-v1/T04-05/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Batch progress/cancel and M export/import/recovery maxima.
- **Durability gate:** D5 desktop publication/restore integration.
- **Cross-platform gate:** Native dialogs and permissions on all supported profiles at T04-06.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T04-06`

### T04-06 — Qualify native usability and accessibility of the complete desktop

- **Task ID / phase / slice:** `T04-06` / `P04` / `VS09`
- **Objective:** Verify the minimal app is usable across the promised desktop profiles.
- **Why it exists:** Web-only tests cannot prove platform keyboard, IME, accessibility or installation behavior.
- **Dependencies:** `T04-05`
- **Files/components:** Native UI scenario scripts, accessibility/IME checklists, screenshots/video where consented and evidence.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Run create→capture→evidence→decision/action→interrupt→resume→proposal→export→restore on all three native profiles. Validate keyboard-only operation, screen-reader names/focus, 200% zoom, Unicode/IME, error progress and non-color-only states. Use deterministic expected results; record manual operator and unresolved issues. Fix bounded UI defects without widening scope.
- **Security considerations:** S08 built-bundle audit and actual no-network/no-active-content observations on each platform.
- **Data invariants:** I02/I12; UI evidence cannot substitute for storage qualification or user adoption.
- **UX behavior:** Every required flow completes with visible save/trust/failure state and no facilitator-only hidden step.
- **Failure behavior:** Native failure blocks P05 entry; unsupported platform cannot be quietly dropped.
- **Acceptance criteria:** All scripted tasks and accessibility criteria pass on the three named profiles; canonical results match CLI oracle.
- **Tests:** V04/V09/V12-V14; manual checks recorded separately from automated desktop assertions.
- **Verification method:** Native test automation where supported plus witnessed manual checklist with exact builds and expected/observed states.
- **Evidence artifact:** `docs/evidence/flake-v1/T04-06/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Desktop startup, interaction and RSS ceilings on each profile.
- **Durability gate:** Restart/close and recovery UX checks; D6 remains T05-02.
- **Cross-platform gate:** All three native profiles mandatory for this exit.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T05-01`

### T05-01 — Freeze compatibility and qualify migration tooling

- **Task ID / phase / slice:** `T05-01` / `P05` / `VS10`
- **Objective:** Make the first release format survivable across versions.
- **Why it exists:** An open format is incomplete if upgrades silently abandon unknown fields or old data.
- **Dependencies:** `T04-06`
- **Files/components:** Versioned standalone offline migrator, public format/protocol specs, golden fixtures and compatibility tests.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Implement section 21 bounded live capability policy and permanently documented old-format reader/migration path. Preserve unknown optional bytes, refuse unsupported required fields/new majors, never lossy downgrade. Qualify format-1→2 and future-unsupported synthetic fixtures; no invented future production format. Pre-migration verified backup, new-root publication and resumable/reconcilable stages.
- **Security considerations:** S04/S05/S10; standalone tool has the same input/authority bounds and no network requirement.
- **Data invariants:** I05/I09/I10; old data remains readable without keeping obsolete main app; unknown bytes exact.
- **UX behavior:** Version refusal names compatible tool and recovery route; migration preview states irreversible aspects and retained old root.
- **Failure behavior:** F05/F06/F18; no automatic compatibility coercion or partial migration success.
- **Acceptance criteria:** All supported paths preserve exact content/history; unsupported fixtures refuse safely; standalone migration succeeds offline from published instructions.
- **Tests:** V05-V10/V15: long-chain applicable fixtures, unknown capabilities, interrupted migration and independent output comparison.
- **Verification method:** Run standalone migrator on preserved synthetic/golden copies and compare manifests with independent reader.
- **Evidence artifact:** `docs/evidence/flake-v1/T05-01/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** M/L migration and memory/cancel ceilings.
- **Durability gate:** D5 full migration failure schedule.
- **Cross-platform gate:** Migration and refusal behavior native on all profiles.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T05-02`

### T05-02 — Qualify durability, confinement and performance on every platform

- **Task ID / phase / slice:** `T05-02` / `P05` / `VS10`
- **Objective:** Produce the complete native evidence required for supported-platform claims.
- **Why it exists:** Existing evidence is Windows-local and does not prove cross-platform or physical power-loss behavior.
- **Dependencies:** `T05-01`
- **Files/components:** D1-D6 native harness/adapters, profile manifests, benchmark generators and immutable raw evidence.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Run all D1-D6 and section 27 hard gates at exact release-candidate source on all profiles. Use disposable vaults, 100 deterministic process schedules/operation, 30 native VM unclean shutdowns/profile, and 10 physical-device controlled trials/profile or an exact-stack qualified lab report. Record device flush/cache assumptions and unsupported cases. Recheck root-handle/path, locks and readonly recovery exclusions.
- **Security considerations:** S03-S06/S10, actual defensive boundary behavior; no offensive exploitation or owner-data fault tests.
- **Data invariants:** I04/I09/I11/I12; zero acknowledged loss/false success in stated matrix, no universal claim.
- **UX behavior:** Publish precise supported profiles and recovery limitations, not a generic cross-platform badge.
- **Failure behavior:** Missing hardware/report or failing maximum blocks qualification; no process-kill proxy for power-loss evidence.
- **Acceptance criteria:** All required native/fault/security/performance cases pass and raw observations are complete; every unsupported capability is explicit.
- **Tests:** V01-V13 as applicable, particularly V06/V07/V09/V12/V13; no lower-gate substitution.
- **Verification method:** Exact native commands, randomized schedules/seeds, device observations, time distributions and independent manifest checks.
- **Evidence artifact:** `docs/evidence/flake-v1/T05-02/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** All section 27 hard maxima at M/L and process RSS/storage growth pass.
- **Durability gate:** D1-D6 complete with profile/device assumptions.
- **Cross-platform gate:** All three profiles and their actual filesystems/devices; no cross-compile-only evidence.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T05-03`

### T05-03 — Build complete offline CLI and desktop release candidates

- **Task ID / phase / slice:** `T05-03` / `P05` / `VS10`
- **Objective:** Install and update Flake without developer tools, network or a model.
- **Why it exists:** Distribution and data retention are part of the product, not a post-project detail.
- **Dependencies:** `T05-02`
- **Files/components:** Product package/binary naming, native package definitions, locked build pipelines and release documentation.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Build section 25 archives/NSIS/deb+offline dependencies/dmg and source bundle. Use flake product naming while preserving historical evidence and safe old-format refusal. Default local data separate from installation/sync folders. Test clean install, manual update, rollback via compatible backup and uninstall retaining vaults. Produce exact source/toolchain/dependency/asset manifests and SBOM.
- **Security considerations:** S07-S09; no secret in build logs, no runtime downloads/core telemetry, package dependencies reviewed.
- **Data invariants:** I01/I09/I10; installer cannot erase or silently migrate owned data.
- **UX behavior:** Offline first run works with Create/Open/Restore, clear location and uninstall retention.
- **Failure behavior:** F05/F19/F22; missing runtime dependency is packaging failure, not a requirement to sign in or connect.
- **Acceptance criteria:** Every unsigned candidate installs/runs/updates/uninstalls on clean network-blocked native image with retained vault bytes and complete dependency inventory.
- **Tests:** V10-V13; fresh user account/clean image installation, upgrade/downgrade refusal and offline packet observation.
- **Verification method:** Exact native packaging commands plus clean-image install scripts and package content manifests.
- **Evidence artifact:** `docs/evidence/flake-v1/T05-03/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Installed startup/RSS and artifact sizes measured; section 27 ceilings apply.
- **Durability gate:** Installer/update/uninstall original-vault preservation and interruption checks.
- **Cross-platform gate:** All three native package profiles; no developer-runtime assumption.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T05-04`

### T05-04 — Close ownership, notices and release-signing obligations

- **Task ID / phase / slice:** `T05-04` / `P05` / `VS10`
- **Objective:** Produce redistributable, verifiable release artifacts under the chosen Apache-2.0 license.
- **Why it exists:** Cargo metadata and public source availability do not prove redistribution rights or artifact authenticity.
- **Dependencies:** `T05-03`
- **Files/components:** LICENSE/NOTICE/third-party notices, SBOM, provenance records, owner-controlled signing configuration and release artifacts.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Verify project contribution ownership and source/data provenance; add Apache-2.0 project license and exact component notices. Resolve Unicode/Zlib/platform runtime and any adapted code obligations. Refresh reachable advisories and build-script review. Sign Windows payloads/installer, macOS app/notarize/staple, and Linux/checksum manifests using owner-controlled credentials without exposing them. Do not purchase credentials or publish remotely without actual authority.
- **Security considerations:** S09; keys remain outside repository/content/logs; signature verification independent of build process.
- **Data invariants:** I12; no legal clearance claim beyond recorded exact rights evidence; unsigned preview not final release.
- **UX behavior:** About/help/distribution include license, source, privacy and support/reporting route.
- **Failure behavior:** F22; absent rights/key/credentials blocks this task with exact obligation, never a fabricated signature.
- **Acceptance criteria:** Every shipped component has a selected compatible license and notices; all signatures/notarization/stapling verify; no unmitigated blocking reachable advisory remains.
- **Tests:** V09/V11/V15: SBOM-to-package reconciliation, independent signature checks, notice completeness and reporting-route validation.
- **Verification method:** Exact native sign/verify/notary commands with secret-free logs, license/provenance checklist and artifact digests.
- **Evidence artifact:** `docs/evidence/flake-v1/T05-04/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Signed installed binaries retain performance gates; no behavioral post-sign mutation.
- **Durability gate:** Signing touches packages, not vaults; package/source digests preserved.
- **Cross-platform gate:** Windows and macOS native signing verification plus Linux manifest signature verification.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T05-05`

### T05-05 — Independently reproduce builds, installation and data exit

- **Task ID / phase / slice:** `T05-05` / `P05` / `VS11`
- **Objective:** Verify the release using a separate executor and clean environment.
- **Why it exists:** One builder can reproduce its own mistakes and undocumented setup.
- **Dependencies:** `T05-04`
- **Files/components:** Pinned source/dependency bundle, reproducibility scripts, independent generic reader and evidence.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Second executor builds from exact source with frozen dependencies/toolchain, compares unsigned payloads and explains only isolated signature/timestamp differences. Install signed artifacts offline on every profile. Run core loop and full export/independent reconstruction after deleting all derived state and without product source/private caches. Repeat legacy migration path from published instructions.
- **Security considerations:** S05/S09/S10; verifier has expected artifact digests and public verification material, no signing secrets.
- **Data invariants:** I01/I06/I10/I12; no network/model/vendor dependency or unexplained executable divergence.
- **UX behavior:** A new user/executor can follow published setup and exit documentation unaided.
- **Failure behavior:** Any unexplained divergence, missing offline dependency or reconstruction omission reopens owning task.
- **Acceptance criteria:** Two clean builds agree on declared reproducible payloads, signatures/installations verify and full canonical reconstruction is exact on all profiles.
- **Tests:** V04/V08-V13/V15, with independence limits and environment differences explicitly recorded.
- **Verification method:** Independent commands, source/toolchain hashes, binary comparison and generic-reader state/byte oracle.
- **Evidence artifact:** `docs/evidence/flake-v1/T05-05/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Installed performance maxima confirmed; no full rerun unless changes/failures require it.
- **Durability gate:** Independent restore and interruption sample plus complete T05-02 evidence verification.
- **Cross-platform gate:** All supported native profiles; independent executor cannot substitute cross-compilation.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T05-06`

### T05-06 — Run the private voluntary repeat-use gate

- **Task ID / phase / slice:** `T05-06` / `P05` / `VS11`
- **Objective:** Check that target users return to the complete product and can leave it.
- **Why it exists:** Correct software may still fail the recurring continuity job.
- **Dependencies:** `T05-05`
- **Files/components:** Consent/privacy and observation protocol, local participant reports and aggregate acceptance evidence.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Execute section 26 eight-new-user/ten-workday study with opt-in local observation, predefined successful resumption and no mandatory telemetry. Count voluntary use, maintenance cost and unaided full-loop/export completion. Preserve abandonment/failures. One recorded bounded repair repeat only; second failure/inconclusive stops product reconsideration.
- **Security considerations:** S07; participant content stays private unless separately consented; aggregates cannot expose paths/secrets.
- **Data invariants:** I12; no fabricated participants, retention extrapolation or substituted internal demos.
- **UX behavior:** Observe users solving their actual bounded project job and using recovery/export independently.
- **Failure behavior:** Missing consent/participants/observations yields Inconclusive; unmet thresholds Fail; no silent completion.
- **Acceptance criteria:** At least six of eight use on six distinct days with ≥3 successful later resumes each; at least six finish the full loop/export without rescue; all required observations legitimate.
- **Tests:** V04/V14/V15 user observation with protocol integrity and independently checked aggregate denominators.
- **Verification method:** Presealed study protocol, opt-in local logs/diaries, consented session checklists and independent aggregate calculation.
- **Evidence artifact:** `docs/evidence/flake-v1/T05-06/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Report total maintenance burden and observed waits; no mandatory data collection.
- **Durability gate:** Any reported lost acknowledged work reopens durability gate immediately.
- **Cross-platform gate:** Record actual participant platforms; native support claims remain grounded in technical matrix.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T05-07`

### T05-07 — Audit the exact release against R01-R12

- **Task ID / phase / slice:** `T05-07` / `P05` / `VS11`
- **Objective:** Determine release readiness from complete evidence rather than completion checkboxes.
- **Why it exists:** Late drift, stale results or missing documentation can invalidate a nearly finished project.
- **Dependencies:** `T05-06`
- **Files/components:** Release manifest, all task evidence, user/CLI/format/protocol/recovery/security docs and final audit.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Map every R01-R12 requirement to exact tested revision/artifact, verify all raw manifests and failures/limitations, inspect final source/security/dependency diff since qualification, and rerun affected checks for any change. Confirm no forbidden capabilities or paid-model/core-network dependency. Verify reporting route, notices, support profiles and recovery instructions. Do not upload or publish.
- **Security considerations:** All S01-S10 reviewed; no unresolved release-blocking issue hidden by severity relabeling.
- **Data invariants:** I01-I12 and historical evidence preservation verified at final candidate.
- **UX behavior:** Documentation explains normal/offline/failure/recovery/exit flows and limitations accurately.
- **Failure behavior:** Any missing/stale/inconsistent artifact blocks R gate; repair exact owner then re-evaluate descendants.
- **Acceptance criteria:** R01-R12 each PASS with exact linked evidence and no unresolved blocking unknown; local candidate set complete.
- **Tests:** V01/V09/V15 audit, targeted regression/packaging/native tests only where changed or unresolved.
- **Verification method:** Manifest rehash, revision-to-evidence traceability, final source/capability review and independent readiness checklist.
- **Evidence artifact:** `docs/evidence/flake-v1/T05-07/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** All maxima tied to final binary; unexplained later change forces affected rerun.
- **Durability gate:** All D1-D6 results bound to current storage implementation.
- **Cross-platform gate:** Final supported-profile table matches actual package/native evidence.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** `T05-08`

### T05-08 — Record verified project completion and preserve the release baseline

- **Task ID / phase / slice:** `T05-08` / `P05` / `VS11`
- **Objective:** Close this bounded product graph with an auditable completion state.
- **Why it exists:** A final declaration must refer to the implemented release, not the planning baseline.
- **Dependencies:** `T05-07`
- **Files/components:** specs/CURRENT.md, completed task ledger, release acceptance report and exact local artifact manifest.
- **Allowed scope:** Only the objective, listed components and necessary tests/documentation/evidence; no unrelated refactoring or successor work.
- **Forbidden scope:** All sections 10/38 non-goals, weakening gates, altering sealed evidence, new authority/process boundaries, successor tasks and unauthorized remote actions.
- **Implementation requirements:** Verify predecessor and all phase gates; freeze tested source/release digests; write evidence-only completion record. Set PROJECT_COMPLETE=YES only under section 37. Record unsupported systems, residual honest limits and deferred scope. No source change after qualification inside this task, no automatic post-completion feature or publication.
- **Security considerations:** Completion does not grant upload/merge/spending authority or weaken security limits.
- **Data invariants:** Preserve 4246f6d, sealed R1 and every failed implementation experiment; no rewritten success story.
- **UX behavior:** Owner receives exact usable local release artifacts, documentation and known limits.
- **Failure behavior:** If any gate is missing, set no success state; return to named owning gate with evidence.
- **Acceptance criteria:** All 38 tasks/R01-R12/phase gates verified; exact tested source and release artifacts linked; clean controlled worktree or explicitly separated unrelated work; truthful final report.
- **Tests:** V01/V15 final evidence/ancestry/diff checks; no new feature tests for documentation-only closeout.
- **Verification method:** Git ancestry/status/diff, artifact rehash and complete gate ledger verification.
- **Evidence artifact:** `docs/evidence/flake-v1/T05-08/REPORT.md`, complete raw-artifact manifest and section 28 environment/commands/results/failures/limitations; no success without actual evidence.
- **Performance gate:** Inherited verified final-binary gates; no new measurement required absent change.
- **Durability gate:** Inherited complete D1-D6; no mutation of real vaults.
- **Cross-platform gate:** All three qualified profiles explicitly identified in completion record.
- **Completion condition:** Every acceptance clause above plus SC and predecessor/phase gates passes; exact evidence reviewed and CURRENT/checkboxes updated. Otherwise remain at this task.
- **Next tasks unlocked:** None. Completion creates no automatic post-v1 frontier.


## 35. Per-phase exit gates

P00: live source/remote/corpus conflicts reconciled, this plan's exact authority recorded, corrective Spec 002 boundaries and admission matrix published. P01: every identified writer/startup/recovery bypass addressed by the new format-2 path or made legacy-readonly, no old evidence rewritten, save/history/backup/migration proofs and T01-07 first native qualification pass. P02: the full CLI project loop and independent portable reconstruction pass, including all omissions/conflicts/unknown fields. P03: all item types enforce scope; exact receipts replay; stale/conflicting evidence stays visible; both clients pass; registered value gate passes. P04: thin desktop passes the same Core state and failure tests, native accessibility/IME and complete recovery/export flow. P05: every section 36 gate passes at the exact release revision.

No generic “mostly done” state crosses a phase boundary. A missing platform or inconclusive value trial is not a pass. Newly discovered failures reopen the owning gate. Evidence that requires an external operator is explicitly pending until supplied; Muse may not replace it with invented logs or a weaker proxy.

## 36. Release-readiness gates

R01 canonical atomicity/history/idempotency and D1-D6 at all supported profiles; R02 source/time/conflict/override behavior with zero hidden disallowed state; R03 complete export/import/legacy migration and independent recovery; R04 default-deny disclosure/proposal/security and current supply-chain review; R05 model/account/network-free end-to-end loop; R06 all hard performance/bounds gates; R07 supported version refusal and migration fixtures; R08 clean native offline install/update/uninstall and retained data; R09 actual licenses/notices/rights/SBOM/signatures and reproducibility evidence; R10 CLI/desktop/protocol/format/recovery/security reporting documentation; R11 product value and voluntary adoption gates; R12 independent reproduction and no unresolved release-blocking regression. Each release gate maps to task evidence in the final report; binary checksums identify exactly what was installed.

## 37. Objective project-complete definition

Muse may set `PROJECT_COMPLETE=YES` only when all 38 tasks and six phase gates have evidence-backed completion, R01-R12 are PASS, the qualified local release-candidate set is present and verified, and the final tested source/tree is immutable and identified. All retained workflows must work on every supported profile with no network/account/model/remote repository. No unresolved acknowledged data loss, false recovery success, disallowed disclosure, missing provenance labeling, incompatible silent mutation, missing redistributable rights or unexplained executable build divergence may remain.

A person must be able to install Flake offline, create/open a project, capture a note/action/decision, link evidence, complete/reopen work, interrupt and resume with honest current/stale/conflict signals, exchange an agent package/proposal, export all owned state, and recover/import it without Flake infrastructure. Required native durability evidence and independent reconstruction must exist. The final report explicitly lists unsupported systems and unproven broader market/agent-performance claims. Actual public publication is separately authorized and is not required to make a verified local release candidate complete. Future features remain deferred; they are not hidden completion tasks.

## 38. Definitive post-completion deferrals

Generalized cloud sync, encrypted collaboration, organization auth/SSO, enterprise administration, messenger/calendar/code-review/CRM parity, mobile, rich collaborative editors/canvas, graph-first navigation/extraction, embeddings/vector defaults, hosted or local model runtimes, AI search/web acquisition, IDE/service integrations, plugin marketplace/Hub, self-optimizing skills, remote execution/orchestration and auto-update are outside this graph. They have no implicit next task after T05-08. Reopening requires a new requirement and authorization with simpler-baseline, rights, security, cost and outcome evidence. No obligation to complete every old roadmap survives this deferral.

## 39. Open questions and bounded stops

**Blocking planning questions: zero.** Apache-2.0 intent is answered by the founder. No storage/editor/platform/agent strategy choice remains for Muse. Implementation-time gates remain: exact dependency/security/rights admission; measured budgets; native device behavior; access to test hardware and consenting users; release signing credentials. These have specified owners/tasks and pass/failure routes. Lack of those resources can block execution; it does not justify declaring success or improvising another product.

If a frozen contract proves infeasible or the product proof fails twice, stop the affected slice, preserve all evidence, propose the smallest ADR or product reconsideration identifying affected contracts/tasks, and obtain the required decision. New C changes require ADR+review, D changes dedicated adversarial/security review, E changes founder direction. Routine task implementation, reversible defect repairs inside the contract and measured bounded options need no new product permission. Remote actions and spending still require their actual authorization.

## 40. Muse execution rules

Start with [the short handoff](FLAKE_MUSE_EXECUTION_HANDOFF.md), then this plan and the active unit. Reverify root/branch/HEAD/worktree/remotes/fetched canonical head/PR state/relevant source before each unit. Preserve unrelated work and all historical evidence. Reconcile new live conflicts explicitly; do not silently cherry-pick a draft PR or resurrect an old roadmap. Execute one dependency-ready unit at a time; no autoactivation of Spec 003. Follow its exact admission, failure, acceptance, verification and phase-exit contract. Record exact source/evidence, retain failed attempts, and update CURRENT only after gates close.

The first unit after a future implementation instruction is **T00-01**. It is an intake/activation task, not a feature. Its immediate successor is **T00-02**, which activates the bounded Spec 002 corrective contract for P01. This assignment has not started either unit. Muse's terminal target is section 37; Astro's planning completion is distinct. No OpenAI API, required paid-model service, force push, destructive history rewriting or unauthorized remote mutation is permitted.
