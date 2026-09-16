# Format-compatibility golden fixtures (`T05-01`)

Two checked-in JSON payload fixtures, each proving one clause of
`docs/formats/format-compatibility-policy.md`. Both are loaded via
`include_str!` (not regenerated at test time) by
`src/project.rs::tests`, so any independent reader can inspect the exact
bytes this build is tested against without running this repository's own
test harness.

## `future-unsupported-note-payload.json`

A `note`-kind payload declaring `payload_schema_version: 999999` — chosen
to stay newer than any real version this codebase reaches, so this
fixture keeps proving the same refusal behavior across future schema-version
bumps without needing to be regenerated. Proves: "Unknown required
field/capability or newer major refuses writes" (plan §21).

Reproduce independently: `RecordPayload::from_json` on this file's exact
bytes must return `Err`, with a message containing "newer than this build
supports" — never a partially-applied or guessed-at parse.

## `unknown-optional-fields-note-payload.json`

A `note`-kind payload at the **current** `payload_schema_version` (`1`),
carrying two fields (`future_priority_flag`, `future_reminder`) that no
field on `Note` names — simulating what a real future minor payload
addition would look like to an older build. Proves: "Unknown optional
payload bytes survive read/export/import unchanged" (plan §21).

Reproduce independently: `RecordPayload::from_json` on this file's exact
bytes must succeed, and both unrecognized fields must be present,
unchanged, in the parsed `Note::unknown` map, and still present after a
read-then-write round trip (`to_json` then `from_json` again).

## Why JSON payload fixtures, not a full binary `canonical.sqlite`

Both rules being proven live at the typed-payload layer
(`RecordPayload::from_json`/`to_json`, `src/project.rs`), independent of
the surrounding SQL schema (`docs/formats/format-2-canonical-sqlite.md`'s
own `min_reader_capability` guard already has its own dedicated test,
`src/canonical.rs::min_reader_capability_higher_than_supported_is_refused`,
proving the whole-database analogue of the same refusal rule). A
human-readable JSON fixture is directly inspectable by any reader with a
text editor; a checked-in binary SQLite file would not be, and would
require this exact `rusqlite` build to open at all — the wrong choice for
an artifact whose whole purpose is independent reproducibility.
