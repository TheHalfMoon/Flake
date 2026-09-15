# Corrective addendum to T01-01 — discovered during T01-07's re-audit

**This does not alter `docs/evidence/flake-v1/T01-01/REPORT.md`.** That report remains the sealed, historical record of what T01-01's original implementation session did and found. This addendum records a defect T01-07's re-audit of `docs/evidence/flake-v1/T00-02/mutator-inventory.md` against the *actual shipped code* (not only the pre-implementation inventory) found in the source T01-01 owns, the root-cause fix, and the regression coverage added — per the canonical plan's explicit defect policy for T01-07: reopen the owning task, fix the root cause, add regression coverage, re-run the owning task's gates, do not paper over it.

## What T01-07's re-audit found

`docs/evidence/flake-v1/T00-02/mutator-inventory.md`'s own table names three confirmed findings requiring a T01-01 fix:

1. `Vault::open_read` auto-creates metadata — **closed** by T01-01 (`Error::MissingMetadata`, verified unchanged in current `src/vault.rs`).
2. `Vault::open_write` mutates before acquiring the writer lease — **closed** by T01-01 (lease acquired first, verified unchanged in current `src/vault.rs`).
3. `EventLog::open` (`events.rs:202` at the time of the original inventory) unconditionally calls `std::fs::create_dir_all(control_dir)` — the inventory's own disposition column says: **"fix: a genuinely readonly open must not create the control directory; creation belongs to `Vault::create` only."**

Re-reading the actual shipped `src/events.rs` as part of T01-07's audit (not merely trusting the inventory's pre-implementation description) showed finding (3) was **not** actually fixed: `EventLog::open` still unconditionally called `create_dir_all` in every commit from T01-01 through `T01-06`'s merge (`7804f20`). T01-01's own evidence report's "Scope actually touched" correctly lists `src/events.rs` as touched, but the actual diff only changed `EventLog::append`'s visibility (`pub` → `pub(crate)`, finding out of the same inventory) and added its doc comment — finding (3) was never implemented, and neither T01-01's evidence nor any later task's evidence claimed otherwise. This is a genuine gap between the inventory's stated disposition and the shipped code, not a fabricated or exaggerated one: confirmed by direct inspection of `EventLog::open`'s function body before this fix.

## Was this exploitable in the current system?

Every call site of `EventLog::open` across the entire codebase (`grep -rn "EventLog::open("`, `src/vault.rs`, `src/cli.rs`, `src/migration.rs`, and every test file) passes a control directory obtained from an already-opened or already-created `Vault` (`v.control_dir()`), which is only reachable after `Vault::open_read`/`open_write`/`create` has already required `.fehrest` to exist. So `create_dir_all` was a no-op (directory already present) in every currently-reachable call, and no observed byte-level mutation from this specific path was ever caught by `open_read_never_creates_metadata_on_legacy_vault`'s existing byte-for-byte-unchanged assertion (that test never routes through `EventLog::open` at all).

This does not change the disposition: the inventory's own bar for this row is "a genuinely readonly open must not create the control directory" — a property of the function's own contract, not a claim conditioned on today's caller discipline. A `pub fn` with latent mutation capability is exactly the shape of defect this whole corrective slice (`T01-01` onward) exists to close everywhere else (`vault.json`, `access.lock`) — leaving one instance open because it happens not to be reachable today is the "papering over" the plan's defect policy explicitly prohibits.

## Root-cause fix

`src/events.rs::EventLog::open` now requires the control directory to already exist, returning `Error::Event` if it does not — mirroring the identical, already-established pattern for `vault.json` (`Error::MissingMetadata`) and `access.lock` (`Error::Vault`, "missing coordination metadata"). No caller needed to change: every real call site already only ever calls this on a directory a `Vault` has already guaranteed exists.

```rust
pub fn open(control_dir: &Path) -> Result<Self> {
    if !control_dir.is_dir() {
        return Err(Error::Event(format!(
            "cannot open event log: control directory does not exist: {}",
            control_dir.display()
        )));
    }
    Ok(EventLog { path: control_dir.join("events.jsonl") })
}
```

## Regression coverage added

`src/events.rs::tests::open_never_creates_a_missing_control_directory`: calls `EventLog::open` on a path that provably does not exist, asserts it returns `Error::Event` (not a panic, not success), and asserts the path still does not exist afterward — the genuinely-readonly-open property the original inventory demanded, now directly tested rather than only true by caller-discipline coincidence.

## Re-run of the owning task's gates (T01-01, and every task built on it)

Re-run in full on this branch, after the fix:

- `cargo test --locked --all-targets`: **175/175 pass** (174 pre-existing + 1 new regression test) — includes every `T01-01` test unchanged and passing (`open_read_never_creates_metadata_on_legacy_vault`, `open_write_acquires_lock_before_any_startup_mutation_no_split_brain_identity`, `losing_writer_performs_no_mutation_before_lock_denial`, and all others), plus every `T01-02`–`T01-06` test unchanged and passing, confirming this fix introduced no regression anywhere in the corrective slice built on top of `EventLog`.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --locked --all-targets -- -D warnings`: PASS, 0 warnings.

No test was skipped, deleted, or weakened to reach this result. Raw output preserved under `docs/evidence/flake-v1/T01-07/raw/` (this fix is committed as part of the `T01-07` branch/PR, since it was discovered during `T01-07`'s own audit — see `docs/evidence/flake-v1/T01-07/REPORT.md` "Defects found and fixed during this task").

## Disposition

`T01-01`'s finding (3) is now genuinely closed. `T01-01`'s original acceptance disposition (`docs/evidence/flake-v1/T01-01/REPORT.md`) is not retroactively rewritten — it accurately reported what that session verified at the time, under the limitations it recorded. This addendum is the honest record of what was actually still open, found by the next task the canonical plan assigned exactly this auditing responsibility to.
