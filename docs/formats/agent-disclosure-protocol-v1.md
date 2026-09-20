# Agent disclosure/proposal protocol (`T03-04`/`T03-05`)

**Status:** `T03-04` (outbound half) / `T03-05` (inbound half). Produced by
`crate::disclosure::compile_disclosure_package`, consumed by
`crate::proposal::admit_proposal`. **A generic JSON/byte reader and writer
can produce and consume both halves of this protocol with no Pluma
dependency at all** — this document is that independent implementer's
specification. `T03-06` exercises this document directly: two client
fixtures, each written from this document alone, without sharing Core's own
encoder/decoder (`src/disclosure.rs`/`src/proposal.rs`) or each other's code.

## What this is not

This is not a live Pluma vault, and not the portable export format
(`docs/formats/portable-export-v1.md`) — a disclosure package is a bounded,
budget-capped, grant-scoped *subset* of one project's disclosable content,
never a full or project-scoped backup. A proposal is not a canonical
mutation — it is inert, reviewed evidence until an owner explicitly accepts
some or all of it through Pluma's own CLI (`propose-accept`).

## Part 1 — the disclosure package (outbound: Pluma → external agent)

### Wire shape: one header line, then one JSON object per line

Unlike the portable export format (one big JSON document per revision
file), a disclosure package is **line-oriented**: a header JSON object,
`\n`, then zero or more item JSON objects, each followed by `\n`. Every
line is complete, independently parseable JSON. There is no wrapping array
or outer object — this is deliberate (see "Design note: why line-oriented"
below).

```text
{"request_id":"...","grant_id":"...","project_id":"...","principal_label":"...","as_of_recorded":42,"as_of_valid":"2026-09-15T12:00:00Z","compiler_version":1,"policy_version":1}
{"object_id":"<uuid7>","kind":"note","revision_id":"<uuid7>","truncation":"full","content":"..."}
{"object_id":"<uuid7>","kind":"decision","revision_id":"<uuid7>","truncation":"truncated","content":"..."}
```

### Header line (exactly one, always first)

| Field | Type | Meaning |
|---|---|---|
| `request_id` | string | The caller-supplied idempotency key for this compilation. |
| `grant_id` | string (UUIDv7) | The `ExportGrant` object this package was compiled under. |
| `project_id` | string (UUIDv7) | The project this package discloses from. |
| `principal_label` | string | A declared label for who/what requested this package (§17: a declaration, never a verified identity). |
| `as_of_recorded` | integer | The vault's transaction-sequence snapshot this package was compiled at. |
| `as_of_valid` | string (UTC RFC 3339) | The wall-clock instant this package was compiled at. |
| `compiler_version` | integer | Currently `1`. |
| `policy_version` | integer | The issuing grant's own policy version, currently always `1`. |

### Item lines (zero or more, one per disclosed record)

| Field | Type | Meaning |
|---|---|---|
| `object_id` | string (UUIDv7) | The disclosed record's canonical identity. |
| `kind` | string | One of `"note"`, `"action"`, `"decision"`, `"source"`, `"relation"`. |
| `revision_id` | string (UUIDv7) | The exact revision this content reflects — pin proposals against this, not a guess. |
| `truncation` | string | `"full"` or `"truncated"`. |
| `content` | string | Human/agent-readable rendered text — see "Content rendering" below. **Never** raw binary bytes. |

### Content rendering per `kind` (informational — clients read `content` as opaque text; this describes what produced it)

- `note`: `"<title>\n\n<body>"`.
- `action`: `"<title> [<state>]\n\n<body>"`.
- `decision`: `"<decision_key>: <statement>\n\n<rationale>"`.
- `source`: `"<label> (<kind>, <byte_length> bytes, sha256=<hex64>)"` for a captured file, or `"<label> (<kind>, reference only)"` for a manual reference. **Never the source's own raw byte content** — a 256 KiB UTF-8 text package cannot and does not carry arbitrary binary evidence.
- `relation`: `"<RelationType>: <from_object_id> -> <to_object_id> (<note>)"`.

### What is never disclosed

- No item's `kind` is ever `"project"`, `"source_check"`, `"review_checkpoint"`, `"export_grant"`, `"disclosure_receipt"`, or `"agent_proposal"` — only the five ordinary work-record kinds above.
- A `relation` item's endpoints (`from_object_id`/`to_object_id`) are always themselves also present as their own item lines in the same package — a package never names a relation pointing at something it does not otherwise disclose.
- A `decision` item only ever reflects a currently-`Accepted` decision (alone, or one of several in an unresolved conflict) — a `Draft`/solely-`Withdrawn` decision is never disclosed.

### Budget

The whole package (header line plus every item line, each including its own trailing `\n`) never exceeds the issuing grant's own `byte_budget`, which itself never exceeds 256 KiB (`limits::MAX_PACKAGE_BYTES`). An item that does not fit within the remaining budget is either truncated (`content` shortened at a UTF-8 boundary, `truncation: "truncated"`) or omitted from the wire entirely — omitted items are recorded only in the separately-persisted `DisclosureReceipt` (a Pluma-internal canonical object, not part of this wire protocol), never as a bare unlabelled gap in the package itself.

### Design note: why line-oriented, not one JSON document

An earlier draft nested every item inside one `{..., "items": [...]}` document. It was rejected during `T03-04`'s own development: the shared envelope's byte cost is not attributable to any single item, which broke exact budget accounting (an item could be individually measured as "fits," yet the whole assembled document still exceed the cap). A flat, line-oriented wire makes each line's own length exactly what it contributes to the total — see `docs/evidence/flake-v1/T03-04/REPORT.md`'s "Failed attempts" for the full account.

## Part 2 — the agent proposal (inbound: external agent → Pluma)

A proposal is a single JSON **document** (not line-oriented — it is read whole, parsed once, and stored as one immutable blob), UTF-8, at most 1 MiB (`proposal::MAX_PROPOSAL_BYTES`).

```json
{
  "receipt_id": "<uuid7>",
  "declared_agent": "my-agent-name",
  "declared_model": "my-model-name",
  "declared_tool": "my-tool-name",
  "operations": [
    {"kind": "note_edit", "note_id": "<uuid7>", "expected_revision_id": "<uuid7>", "title": "optional", "body": "new body text"},
    {"kind": "draft_decision", "decision_key": "some-key", "statement": "...", "rationale": "optional"},
    {"kind": "evidence_relation", "relation_type": "supports", "from_object_id": "<uuid7>", "to_object_id": "<uuid7>", "note": "optional"},
    {"kind": "complete_action", "action_id": "<uuid7>", "expected_revision_id": "<uuid7>", "summary": "..."}
  ]
}
```

### Top-level fields

| Field | Type | Required | Meaning |
|---|---|---|---|
| `receipt_id` | string (UUIDv7) | yes | The `DisclosureReceipt` (from Part 1's header `request_id`/`grant_id` compilation) this proposal responds to. Must name a real receipt belonging to the same project the proposal is admitted into. |
| `declared_agent` | string or absent | no | §17: a declaration, never verified. Absent means openly "Unknown" — never guess or default to something else. |
| `declared_model` | string or absent | no | Same rule. |
| `declared_tool` | string or absent | no | Same rule. |
| `operations` | array | yes | 1 to 100 operations (`proposal::MAX_PROPOSAL_OPERATIONS`). Zero operations is refused. |

### Operation kinds — exactly these four, no others

The `kind` field is a strict discriminator. Any value other than the four below (or a structurally malformed operation) fails the *entire proposal's* admission — there is no partial acceptance of a malformed batch.

- **`note_edit`**: `note_id`, `expected_revision_id`, `body` (required); `title` (optional). Proposes replacing a `Note`'s title/body.
- **`draft_decision`**: `decision_key`, `statement` (required); `rationale` (optional). Proposes a brand-new `Decision` at `Draft` lifecycle — an agent can never create or reach `Accepted` directly (§17).
- **`evidence_relation`**: `relation_type` (one of `supports`, `contradicts`, `depends_on`, `relates_to`, `supersedes`), `from_object_id`, `to_object_id` (required); `note` (optional).
- **`complete_action`**: `action_id`, `expected_revision_id`, `summary` (required). Proposes completing an `Action`.

### Receipt binding

`note_edit` and `complete_action` each target an *existing* object (`note_id`/`action_id`). That object's ID must appear in the referenced receipt's own disclosed item list (Part 1) — a proposal cannot target something that was never actually disclosed to the agent that is now proposing to edit it. `draft_decision` (wholly new content) and `evidence_relation` (may legitimately reference an object created earlier in the very same proposal's own operation list) are exempt from this specific check.

### What a proposal can never do

Structurally, by the closed four-variant operation set above — there is no field or `kind` value that could: issue or reference a grant, change which project an object belongs to, accept or supersede a decision, migrate or restore a vault, delete anything, or execute a command/tool of any kind. §18: "Agents cannot issue grants, change project scope, accept decisions, overwrite current revisions, migrate, restore, delete data or execute tools."

### Owner review, not automatic application

Admitting a proposal (Pluma's `propose-import` CLI command, or `proposal::admit_proposal`) only ever parses and validates it into a `Pending` record — no canonical content changes. Only an explicit, separate owner command (`propose-accept`, naming which operation indices to apply) ever mutates canonical state, and only for operations whose `expected_revision_id` (where present) still matches current state at the moment of acceptance — a stale reference is refused, never silently rebased.

### Unsupported/malformed proposals

A structurally invalid proposal (bad JSON, unrecognized `kind`, missing required field, oversized, too many operations, unknown or cross-project `receipt_id`, an existing-target operation naming an undisclosed object) is refused at admission time with a clear error — it never becomes a `Pending` record at all, and canonical state is never touched.

### Replay / duplicate delivery

Submitting the exact same proposal bytes twice produces two independent `Pending` proposals (admission is not deduplicated) — the owner sees both and can reject the duplicate. Replaying an `accept`/`reject`/`expire` call against a proposal that has already left `Pending` is refused (the proposal's own status guard), so an operation is never double-applied.
