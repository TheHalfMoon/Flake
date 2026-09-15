# Interchange conformance fixtures (`T03-06`)

Two independently-written, minimal Python (stdlib-only) client fixtures,
written from `docs/formats/agent-disclosure-protocol-v1.md` alone — neither
imports, calls, or shares code with `src/disclosure.rs`/`src/proposal.rs`
(Flake's own encoder/decoder) or with each other. Both run fully offline:
no network call, no model/account, no environment variable read beyond
what Python's own standard library touches by default.

- `client_a.py` — reads one disclosure package, picks its first `note`
  item, and writes a `note_edit` proposal replacing that note's body.
  Straightforward, linear, function-per-step style.
- `client_b.py` — reads a *second* disclosure package (compiled after
  client A's proposal was reviewed and accepted through Flake's own CLI)
  and writes a `draft_decision` proposal — a genuinely different operation
  kind, citing the now-current note content client A changed. Deliberately
  written in a different style (a small class-based reader) from
  `client_a.py`, matching this crate's own established independence
  discipline (see `tools/independent-verify/README.md`'s identical rule for
  `T02-07`'s two readers).
- `negative_cases.py` — three adversarial proposals (unknown declared
  identity, a structurally unsupported operation `kind`, and duplicate
  delivery of the same well-formed proposal) submitted directly through
  Flake's own `propose-import`/`propose-accept` CLI, independently verifying
  each is handled exactly as `docs/formats/agent-disclosure-protocol-v1.md`
  says it must be.
- `run_interchange.py` — the orchestrator. Builds the exact end-to-end
  sequence `T03-06`'s own acceptance criterion names (client A proposes,
  owner reviews/accepts, client B reads the resulting package and
  continues, owner reviews/accepts), runs the negative cases, then
  independently verifies final canonical state by reading
  `canonical.sqlite` directly with Python's `sqlite3` module (reusing
  `tools/independent-verify/sqlite_reader.py`'s already-independent
  low-level reader, not Flake's own `record-show`/`resume` output) — never
  trusting the CLI's own claims about what it did as the sole oracle.

## Running

```bash
cargo build --locked --bin fehrest
python3 tools/interchange-clients/run_interchange.py
```

Requires only a built `fehrest` binary (path autodetected under
`target/debug/` or `target/release/`, or pass `--bin <path>` explicitly)
and a Python 3 interpreter with only the standard library. Prints a
plain-text conformance report to stdout and exits non-zero on any failure.
