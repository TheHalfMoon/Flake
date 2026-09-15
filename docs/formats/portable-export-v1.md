# Portable export format (`T02-05`)

**Status:** `T02-05`. Produced by `crate::export::export_to_new_root`, published
at `<dest_root>/.fehrest-export/`. A **generic JSON/byte reader can validate
and read this format with no Flake dependency at all** — this document is
that generic reader's specification.

## What this is not

This is **not** a live Flake vault. You cannot point
`crate::canonical::CanonicalStore::open` at an export directory — it carries
no writer lock, no live transaction log, and no `canonical.sqlite`. It is a
read-only, independently verifiable snapshot. See `docs/formats/format-2-canonical-sqlite.md`
for the live-vault format, and `T01-05`'s `crate::backup` module for a
*restorable* SQLite-level backup — a different, unrelated artifact that
happens to also use the phrase "full-backup" in `backup-manifest.json`'s own
`kind` field (this export format's own `kind` values,
`export-full`/`export-project`, are deliberately spelled differently so the
two are never confused).

## Layout

```text
.fehrest-export/
  export-manifest.json
  README.md
  revisions/<object_id>/<recorded_seq>-<revision_id>.json
  revisions/<object_id>/<recorded_seq>-<revision_id>.md      (Note revisions only)
```

Every path segment other than the two fixed filenames is either a UUIDv7
(`object_id`, `revision_id`) or a plain decimal integer (`recorded_seq`) —
never user-supplied text — so every member path is safe on every target
filesystem by construction.

## `export-manifest.json`

```json
{
  "schema": "flake-export-manifest-v1",
  "kind": "export-full",
  "vault_id": "<uuid7>",
  "project_id": null,
  "snapshot_head_seq": 42,
  "snapshot_head_hash": "<hex64-or-null>",
  "created_at": "<UTC RFC3339>",
  "record_count": 7,
  "revision_count": 11,
  "omissions": [],
  "members": [
    {"path": "README.md", "length": 1234, "sha256": "<hex64>"},
    {"path": "revisions/<object_id>/<seq>-<revision_id>.json", "length": 512, "sha256": "<hex64>"}
  ],
  "integrity_root": "<hex64>"
}
```

- `kind`: `"export-full"` (every object in the store) or `"export-project"`
  (exactly one project's `Project` record plus its `Note`/`Action`/
  `Decision`/`Source`/`Relation` objects — see "Scope" below). A
  project-scoped export is never labeled `export-full`.
- `snapshot_head_seq`/`snapshot_head_hash`: the canonical store's own
  transaction head captured **before** any revision was read — this
  snapshot boundary excludes the export operation's own (nonexistent, since
  export never writes to the canonical store) later event, per §15's "manifest
  snapshot head excludes its own later export event."
- `omissions`: always empty for this format version — every in-scope
  revision is always included. Present in the schema (not omitted) so a
  reader never has to guess whether omission-tracking exists.
- `members`: every published file except `export-manifest.json` itself,
  each with its exact byte length and SHA-256.
- `integrity_root`: SHA-256 of the JSON object
  `{schema, kind, vault_id, project_id, snapshot_head_seq, members}`, where
  `members` is first sorted by `path` — the manifest never hashes itself.
  Recompute it with any JSON/SHA-256 tool to independently verify nothing in
  the export was altered after publication.

## Revision files (`revisions/<object_id>/<seq>-<revision_id>.json`)

```json
{
  "schema": "flake-export-revision-v1",
  "object_id": "<uuid7>",
  "revision_id": "<uuid7>",
  "parent_revision_id": "<uuid7-or-null>",
  "recorded_seq": 3,
  "recorded_at": "<UTC RFC3339>",
  "actor": "owner",
  "origin": "user",
  "kind": "note",
  "payload_sha256": "<hex64>",
  "payload_raw": "<the exact original stored payload string>"
}
```

`payload_raw` is the **exact** byte-for-byte string `canonical.sqlite`'s own
`revision.payload` column holds for this revision — never re-parsed into a
typed record and re-emitted, which could silently reorder JSON keys or
change whitespace. Verify it against `payload_sha256` (and, transitively,
against `export-manifest.json`'s own `sha256` for this file) with any
SHA-256 tool. `kind` is a read-only convenience (Core-derived from the
payload's own dispatch tag at export time, per `crate::project::RecordPayload`)
for a human skimming the export — never authoritative, and this format never
feeds it back into anything.

Every revision of an in-scope object is included — not only its current
state — because "an export that omits source revisions... is not
ownership" (this task's own stated rationale). A `Note` revision additionally
gets a `.md` sibling file containing just its plain-text Markdown body, for
easy human reading; it is plain text, never HTML, and nothing in this format
executes or interprets it.

## Scope

- `export-full`: every revision of every object currently in the canonical
  store, of any kind (`Project`/`Note`/`Action`/`Decision`/`Source`/
  `Relation`).
- `export-project`: the named project's own `Project` object, plus every
  `Note`/`Action`/`Decision` currently belonging to it
  (`crate::project::list_project_records`), every `Source`
  (`crate::capture::list_project_sources`), and every `Relation`
  (`crate::relation::list_project_relations`) — the identical per-kind
  listing functions the live product already uses, not a separately
  re-derived membership rule. A project-scoped export never contains any
  byte belonging to another project — not the other project's content, and
  not even its bare object IDs.

## What this format deliberately omits

- Streaming export for a store too large to hold entirely in memory at
  once — `T05-02`'s hardware-qualified pass, the identical deferral every
  prior `flake-v1` task with a performance gate already records.
- Any redaction/encryption mechanism: nothing in the current canonical
  record model carries local filesystem or agent-grant authority to redact
  in the first place (`Source::claimed_path`/`claimed_repository`/
  `claimed_commit` are already-documented non-authoritative descriptive
  strings the user typed, never real filesystem access — see
  `crate::capture` module docs).

## Importing this format (`T02-06`, `crate::import`)

`crate::import::read_and_validate_package` re-verifies everything this
document describes before trusting any of it: every declared member's
existence, length and SHA-256; the manifest's own `integrity_root`,
independently recomputed; every revision file's own `schema` and
`payload_sha256`; that every revision parses as a recognized typed record;
that no `(object_id, revision_id)` pair repeats; and that each object's
revision chain (`parent_revision_id`) is internally consistent. Any
violation refuses the **entire** import — there is no partial admission.

Two import modes exist, both described in full in `crate::import`'s own
module docs:

- **Full restore** (`import_full_restore`) into a brand-new, empty vault:
  every `object_id` is preserved exactly.
- **Selected merge** (`import_selected_merge`) into an already-open,
  possibly non-empty vault: every object gets a **new** destination
  identity, and every internal cross-reference (`project_id`,
  `Action::dependency_ids`, a `Relation`'s endpoints) is rewritten to point
  at the new identities.

**Neither mode preserves `revision_id`/`recorded_seq`/`recorded_at`
byte-for-byte.** The destination canonical store always mints a fresh
`revision_id` and `recorded_at` for every committed command — this format's
own `revision_id`/`recorded_seq`/`recorded_at` fields exist so a reader can
reconstruct the *exact content and order* of every historical state, not so
an importer can literally replay the original envelope metadata verbatim.
`object_id` (the identity §15 actually calls load-bearing) and every
revision's exact payload bytes, replayed in their original order, are what
survive import exactly.
