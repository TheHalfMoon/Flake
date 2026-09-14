# Legacy format-1-to-format-2 migration (`T01-06`)

**Status:** `T01-06`. Imports a format-1 markdown-tree vault
(`crate::vault::Vault`) into a fresh format-2 canonical store
(`crate::canonical::CanonicalStore`) as a one-shot, non-mutating-on-the-source
operation. Does not import the source's event-log content as format-2
history — see "What is not migrated" below.

## What gets imported

Each admittable legacy object (a markdown file with valid, unambiguous
frontmatter identity) becomes one `CommandTarget::ImportObject` command
against the new vault:

- `object_id` = the legacy file's own UUIDv7, preserved exactly — the one
  deliberate exception to normal command admission's "identity is
  Core-assigned" rule (I03), used only because the legacy identity is
  unambiguous (see "Admission rules" below).
- `payload` = the **exact original file bytes**, read directly from disk
  and never passed through `identity::parse`/`serialize`. CRLF line
  endings, exact whitespace, non-ASCII/Unicode content, and any
  unrecognized frontmatter lines all survive untouched, because the parser
  used to *discover* an object's ID (`crate::vault::Vault::scan`) is never
  used to *reconstruct* its content.
- `actor` = `"migration"`, `origin` = `RecordOrigin::Migration`.

## Admission rules (what preview computes, before anything is imported)

`crate::migration::preview_migration` runs a nonmutating
`Vault::scan()` over the source and classifies every discovered path:

| Outcome | Condition |
|---|---|
| Admitted | Valid, parseable frontmatter with a UUIDv7 `id` observed at exactly one path |
| Omitted: `ambiguous identity ...` | The same `id` observed at more than one path (`scan()`'s own conflict detection) — retaining either copy would mean guessing which is authoritative, which this module never does |
| Omitted: `malformed: ...` | The file failed to parse (no frontmatter, corrupt fields, etc.) |
| Omitted: `skipped: reserved directory or unsupported extension` | `.fehrest`/`.git` content, or a non-`.md`/`.markdown` file |

The source vault's own event-log chain status (if a log is present) is
recorded on the preview as informational text only — **never** authenticated
or treated as proof that anything it claims actually happened, per this
task's own "do not authenticate legacy log claims" instruction.

## What "complete" means

- `ImportSelection::AllAdmittedOnly`: refuses immediately, before creating
  anything, if the preview shows **any** omission at all. "Ambiguous
  complete migration refuses" is enforced as a hard precondition, not a
  best-effort warning.
- `ImportSelection::Selected(ids)`: imports exactly the named
  `legacy_object_id`s. Always produces `complete: false`, even if the
  selection happens to cover every admittable record — the label states
  intent (an explicit, reviewed subset), not just outcome.
- Requesting an `id` that is not admittable at all (typo, or itself
  ambiguous) is refused outright, not silently dropped from the selection.

## Migration manifest

Written as `.fehrest/migration-manifest.json` inside the **new** vault,
whether the import completed or was interrupted partway:

```json
{
  "schema": "flake-migration-manifest-v1",
  "source_root": "<path>",
  "imported": [
    {"legacy_object_id": "<uuid7>", "rel_path": "a.md", "raw_bytes": "...", "content_sha256": "<hex64>"}
  ],
  "omitted": [
    {"rel_path": "b.md", "reason": "malformed: ..."}
  ],
  "complete": true,
  "interrupted": false,
  "failure_reason": null
}
```

An interrupted import (a real error, or one of this module's own injected
test faults) still writes this manifest with `interrupted: true` and the
exact `failure_reason`, and `imported` lists exactly the records that
actually committed before the failure — the new vault's own state
(`CanonicalStore::open`, `read_current`) is trustworthy for exactly those
records; the manifest is what tells a caller not to mistake that state for
a finished migration.

## What is not migrated

- The source event log's own event records (`VaultCreated`,
  `ObjectRegistered`, ...) are not replayed as format-2 commands. Format-2's
  own `command`/`revision` history for each imported object begins fresh,
  at one `ImportObject` command — it does not manufacture a multi-revision
  history the legacy format never actually had (this task's own "why it
  exists": "must not manufacture lost revisions").
- No CLI migration-preview command exists (no CLI wiring exists for the
  format-2 store at all yet).
- Streaming import for a legacy vault too large to `scan()` into memory at
  once is not implemented — `Vault::scan()` itself is not streaming, a
  pre-existing property of the format-1 reader this task does not change.
