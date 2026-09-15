# Independent T02-07 verifier

**Independence boundary (read this first):** everything in this directory
is plain Python 3, using only the standard library (`json`, `hashlib`,
`sqlite3`, `pathlib`, `re`, `struct`). Nothing here imports, links, builds,
or shells out to the `fehrest` crate's own parsing/export/import code
(`crate::export`, `crate::import`, `crate::project::RecordPayload`, or any
other Rust module). This is a different language, a different runtime, and
a hand-written re-implementation of the two format specifications
(`docs/formats/format-2-canonical-sqlite.md`,
`docs/formats/portable-export-v1.md`) from their own published text, not
from reading the Rust source that implements them. That is the actual
independence claim T02-07 requires: an outside tool, built only from the
public documentation, can validate and reconstruct Flake's declared state.

Two genuinely separate raw-artifact readers exist:

- `sqlite_reader.py` reads `canonical.sqlite` directly with Python's
  built-in `sqlite3` module -- no Flake binary, no Flake library.
- `export_reader.py` reads a published `.fehrest-export/` directory as
  plain JSON/filesystem, independently recomputing every hash the format
  document specifies.

Both raw readers produce a list of "revision envelopes" (plain dicts:
`object_id`, `revision_id`, `parent_revision_id`, `recorded_seq`, `kind`,
`payload` (parsed generically with `json.loads`, never through a typed
Rust struct)). `semantic_model.py` is *shared* by both readers -- sharing
this file is not a violation of independence: it contains none of the raw
parsing/hashing logic (that lives separately in each reader, per format),
only the pure, format-agnostic step of turning "a list of revision
envelopes" into semantic facts (current state per object, record counts,
relation edges, dependency edges, project membership). Reusing this last
step avoids two copies of ordinary dict-manipulation code; it reuses none
of Flake's own code and could not, by itself, hide a shared omission bug in
*either* raw reader, because each reader independently decides what
envelopes exist in the first place.

## Files

| File | Role |
|---|---|
| `sqlite_reader.py` | Raw reader: `canonical.sqlite` -> revision envelopes, using documented schema/pragmas only |
| `export_reader.py` | Raw reader: `.fehrest-export/` -> revision envelopes, re-verifying every manifest/member/payload hash first |
| `semantic_model.py` | Shared, format-agnostic: revision envelopes -> semantic report (counts, current state, relations, dependencies, project membership) |
| `crosscheck.py` | Compares a SQLite-derived semantic report against an export-derived one (full-store equality, or project-scope subset) |
| `adversarial.py` | Hand-corrupts copies of a valid export package and asserts `export_reader.py` refuses each one with a specific, correct diagnosis |
| `build_fixture.py` | NOT part of the verifier -- drives the real `fehrest` CLI to build disposable fixture data, exactly as an ordinary user would |
| `run_all.py` | Orchestrates: builds a fixture, runs both readers, cross-checks, runs the adversarial suite, prints a machine-readable JSON report |

## Running it

```sh
cargo build --locked --bin fehrest
python3 tools/independent-verify/run_all.py \
    --binary target/debug/fehrest \
    --workdir /tmp/t0207-run
```

Exits non-zero and prints which check failed if anything does not verify.
