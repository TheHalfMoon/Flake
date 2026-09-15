# Typed project/work-record payloads (`T02-01`, extended `T02-02`)

**Status:** `T02-01` (`Project`/`Note`/`Action`/`Decision`), extended by
`T02-02` (`Source`). Defines the JSON shape `src/project.rs`/`src/capture.rs`
read and write as `revision.payload` in the format-2 canonical store
(`docs/formats/format-2-canonical-sqlite.md`). Adds **no new SQL column and
no new `CommandTarget` variant** — every typed record is committed through
the exact same `CreateObject`/`UpdateObject` commands, atomicity guarantees,
and idempotency rules `T01-03` already built and `T01-07` already proved
with 100 fault schedules. This document is about the *payload's own*
format, not a change to the surrounding transaction mechanics.

## Common shape

Every typed record is a single JSON object with two fields every kind
shares, plus kind-specific fields below:

```json
{
  "kind": "project" | "note" | "action" | "decision" | "source",
  "payload_schema_version": 1
}
```

`kind` is written only by `RecordPayload::to_json`'s one serialization
choke point — never taken from caller-supplied data (S04/S06: "type tags...
cannot bypass admission"). `payload_schema_version` is this module's own
payload-shape version, distinct from `canonical::CANONICAL_SCHEMA_VERSION`
(the surrounding SQL schema) and from `RecordOrigin` (who declared the
record). A payload declaring a version newer than `RECORD_PAYLOAD_SCHEMA_VERSION`
is refused on read, not guessed at ("record schema capability
compatibility").

Every struct additionally carries an `unknown` catch-all
(`#[serde(flatten)]`) for any field this build does not recognize — a
payload written by a newer build round-trips those extra fields unchanged
through a read-then-write by an older one, exactly like `identity::Frontmatter::unknown`
already does for format-1 markdown frontmatter (§15 "unknown fields...
retained").

## `Project`

```json
{
  "kind": "project",
  "payload_schema_version": 1,
  "name": "My Project",
  "description": "optional description",
  "active": true
}
```

No `project_id` (a project does not belong to another project). Archiving
(`active: false`) or unarchiving is committed as an ordinary `UpdateObject`
— a new revision, never rewriting the prior one, so the pre-archive state
remains in immutable history (I05, §12 "project archive is reversible").

## `Note`, `Action`, `Decision` (work records)

Every work record carries `project_id` — the owning project's `object_id`,
validated at commit time to reference an existing `Project` object (I03/I05/I07:
"one owning project per work record; no orphaned canonical references").
Archived projects remain valid references (archiving hides a project from a
default view; it does not retroactively sever the identity its own work
records still carry).

```json
{
  "kind": "note",
  "payload_schema_version": 1,
  "project_id": "<uuid7>",
  "title": "optional title",
  "body": "Markdown body text",
  "tombstoned": false
}
```

```json
{
  "kind": "action",
  "payload_schema_version": 1,
  "project_id": "<uuid7>",
  "title": "Do the thing",
  "body": "optional detail",
  "state": "open" | "doing" | "blocked" | "done" | "cancelled"
}
```

Every action starts `"open"`. Reaching a later state, "any reopening
requires an event," and ordered dependency-cycle rejection are `T02-03`'s
objective ("record decisions and complete actions with visible history"),
not implemented here.

```json
{
  "kind": "decision",
  "payload_schema_version": 1,
  "project_id": "<uuid7>",
  "decision_key": "question-1",
  "statement": "We will do X",
  "rationale": "optional rationale",
  "lifecycle": "draft" | "accepted" | "superseded" | "withdrawn"
}
```

Every decision starts `"draft"`. `decision_key` is **not** required to be
unique within a project — §12 "Conflict" and §15 explicitly allow competing
decisions to intentionally share a key ("Existing keys are selectable to
place competing decisions in the same question"); computing that conflict
is `T02-03`'s objective. Evidence linkage, owner-only acceptance, and
override/supersession semantics are likewise `T02-03`'s objective, not this
task's.

## `Source` (`T02-02`)

```json
{
  "kind": "source",
  "payload_schema_version": 1,
  "project_id": "<uuid7>",
  "label": "meeting notes from the vendor call",
  "source_kind": "file" | "manual_reference",
  "claimed_repository": null,
  "claimed_commit": null,
  "claimed_path": null,
  "active": true,
  "capture": null
}
```

`source_kind` names its field `source_kind`, not `kind` — the latter is
reserved for `RecordPayload`'s own dispatch tag, and an earlier draft that
named it `kind` produced a genuine field collision (documented in
`docs/evidence/flake-v1/T02-02/REPORT.md`, "A real, necessary widening..."
section and its preceding bug note). `claimed_repository`/`claimed_commit`/
`claimed_path` are descriptive-only metadata, never locator/read authority
(§15 "Locator is local-only metadata, never export or read authority").
Deactivating/reactivating a source (F08 "missing source... retain saved
source revision") commits an ordinary new revision with `active` flipped,
exactly like `Project.active` — the prior revision's `capture`, if any,
remains in immutable history.

When `source_kind` is `"file"`, `capture` holds the immutable "source
revision" content (§15):

```json
{
  "bytes_hex": "68656c6c6f",
  "byte_length": 5,
  "sha256": "<hex>",
  "display_filename": "notes.txt",
  "mime_label": "text/plain",
  "observed_at": "2026-09-15T12:00:00Z",
  "source_mtime": "2024-01-03T08:15:22Z",
  "origin_label": "unknown"
}
```

Bytes are hex-encoded (not base64: no new dependency is admitted for this
task, matching this crate's existing minimalism) — 2x inflation instead of
~1.33x, which `crate::limits::MAX_COMMAND_PAYLOAD_BYTES` already accounts
for. `byte_length`/`sha256` are computed from the *original* bytes, not
`bytes_hex`. `mime_label` is a best-effort filename-extension lookup,
never content-sniffed and never a reason to execute or render anything.
`observed_at` is this module's own capture wall-clock time; `source_mtime`
is the filesystem's own reported modification time when available, or
absent — never invented (I11). `origin_label` is unconditionally
`"unknown"` in this task: there is no rights/authorship verification
mechanism yet, and no public API accepts a caller-supplied origin claim.
When `source_kind` is `"manual_reference"`, `capture` is `null` — "explicitly
reference-only" (§15).

Recovering the exact original bytes of a `"file"` source is
[`crate::capture::extract_bytes`] (CLI: `source-extract`) — this task's own
"recoverable bytes" objective made concrete, not merely an internal
round-trip.

### Obvious-secret filename refusal (S07) — and its stated limit

`crate::capture::import_file` refuses a small, named, deliberately
incomplete denylist of filenames (`.env`, `id_rsa`, `credentials.json`,
`*.pem`, and similar) before ever reading their content, with an explicit
carve-out for common non-secret suffixes (`.env.example`, `*.sample`,
`*.template`, `*.dist`). **This is a best-effort footgun reducer, not a
security guarantee**: it cannot see inside a file it refuses to admit, and
cannot catch a secret saved under an unrelated name. State this limitation
to the caller in the refusal message itself, not only in documentation.

## Field limits (checked before any transaction opens — F05/F07/F20)

| Field | Limit | Source |
|---|---|---|
| `Project.name` | 1024 bytes | No dedicated §27 limit named; reuses the title bound below |
| `Note.title` / `Action.title` | 1024 bytes | §27 "Normal inputs": title ≤1 KiB |
| `Note.body` / `Action.body` / `Project.description` / `Decision.rationale` | 1 MiB | §27 "Normal inputs": text body ≤1 MiB (`crate::limits::MAX_OBJECT_BYTES`, re-exported rather than redefined) |
| `Decision.statement` | 8192 bytes | §27 "Normal inputs": decision statement ≤8 KiB |
| `Decision.decision_key` | 1–128 bytes | §15 "Decision keys are required explicit identifiers... at most 128 UTF-8 bytes" |
| `Source.label` | 4096 bytes | §27 "Normal inputs": source locator label ≤4 KiB |
| `Source.capture` original byte length (`"file"` kind) | 64 MiB | §15 "Artifact... Max 64 MiB per artifact"; enforced against actually-read bytes, never `fs::metadata().len()` alone |

A violation returns `Error::Project` (record-shape/field-limit/cross-
reference checks) or `Error::Capture` (`T02-02`'s own security-boundary
refusals: symlink, non-regular-file, secret filename, oversized artifact,
unreachable path) before `CanonicalWriter::commit` is ever called — no
transaction opens, so `canonical_vault`'s head is provably unchanged
(verified directly in `field_limits_are_rejected_before_any_transaction_opens`,
`oversized_artifact_is_refused_before_any_mutation`).

## What reading a project's work records means today

[`CanonicalStore::list_current_objects`] returns every live object's
current `(object_id, revision_id, payload)`; `project::list_project_records`
filters that full scan by `project_id` in application code, and
`capture::list_project_sources` (`T02-02`) does the same for `Source`
objects specifically (a `Source` is not counted as a "work record" by
`list_project_records` — it has its own dedicated listing). Neither is
an index — acceptable at this task's own "S/M project open/read" performance
gate. A dedicated project-scoped index is `T02-04`'s objective ("find work
with a disposable current lexical index"), not this one's.

## Markdown preview contract (`T02-02`)

`crate::markdown::preview` is the entire "Markdown parser" this codebase
has: a bounded, `char`-boundary-safe truncation of a `Note.body` for
listings (CLI: `record-show --preview N`). It does not parse Markdown
syntax and does not render HTML — there is no HTML rendering engine
anywhere in v1 (S01). A future component that does render Markdown to a
screen (P04 desktop) must preserve the same inertness: displaying a body
is never permitted to execute or trust anything found inside it. The full,
exact body remains available unmodified via `record-show` without
`--preview`; a preview is a display convenience, never a second copy of
canonical truth.
