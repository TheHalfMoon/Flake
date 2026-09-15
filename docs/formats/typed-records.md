# Typed project/work-record payloads (`T02-01`)

**Status:** `T02-01`. Defines the JSON shape `src/project.rs` reads and
writes as `revision.payload` in the format-2 canonical store
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
  "kind": "project" | "note" | "action" | "decision",
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

## Field limits (checked before any transaction opens — F05/F07/F20)

| Field | Limit | Source |
|---|---|---|
| `Project.name` | 1024 bytes | No dedicated §27 limit named; reuses the title bound below |
| `Note.title` / `Action.title` | 1024 bytes | §27 "Normal inputs": title ≤1 KiB |
| `Note.body` / `Action.body` / `Project.description` / `Decision.rationale` | 1 MiB | §27 "Normal inputs": text body ≤1 MiB (`crate::limits::MAX_OBJECT_BYTES`, re-exported rather than redefined) |
| `Decision.statement` | 8192 bytes | §27 "Normal inputs": decision statement ≤8 KiB |
| `Decision.decision_key` | 1–128 bytes | §15 "Decision keys are required explicit identifiers... at most 128 UTF-8 bytes" |

A violation returns `Error::Project` before `CanonicalWriter::commit` is
ever called — no transaction opens, so `canonical_vault`'s head is
provably unchanged (verified directly in
`field_limits_are_rejected_before_any_transaction_opens`).

## What reading a project's work records means today

[`CanonicalStore::list_current_objects`] returns every live object's
current `(object_id, revision_id, payload)`; `project::list_project_records`
filters that full scan by `project_id` in application code. This is **not**
an index — acceptable at this task's own "S/M project open/read" performance
gate. A dedicated project-scoped index is `T02-04`'s objective ("find work
with a disposable current lexical index"), not this one's.
