# Mutator inventory — T00-02 source finding (for T01-01)

Built by enumerating every `pub fn`/`pub(crate) fn` in the four files the canonical plan names as T01-01's components (`src/vault.rs`, `src/events.rs`, `src/locator.rs`, `src/cli.rs`; `git grep -n "pub fn\|pub(crate) fn"`), then reading each body to classify it. This is read-only source inspection; **no source file was modified to produce this document**, consistent with T00-02's forbidden scope.

Line numbers below were produced against `852e44b`'s source tree and re-confirmed unchanged against `origin/main` `6389f651...` (see docs/evidence/flake-v1/T00-02/REPORT.md provenance note).

## Confirmed findings matching the canonical plan's stated T01-01 rationale

The canonical plan states T01-01 exists because: *"Current `open_read` may create metadata, startup writes precede ownership, and raw event methods bypass writer binding."* All three clauses are independently verified against the actual source:

1. **`open_read` creates metadata.** `Vault::open_read` (`src/vault.rs:140`) unconditionally calls `ensure_vault_meta(&root.join(CONTROL_DIR))` (`vault.rs:146`). `ensure_vault_meta` (`vault.rs:460`) calls `write_vault_meta_atomic` — a durable write — whenever no metadata file is found, i.e. a plain "read" open of a metadata-less legacy directory silently creates a new vault identity on disk. Confirmed, not inferred.
2. **Startup writes precede ownership.** `Vault::open_write` (`vault.rs:123`) calls `ensure_vault_meta` (possible write) and then `startup_integrity_check` (`vault.rs:629`) — which itself calls `EventLog::quarantine_and_repair_torn_tail()` (a repair/mutation, `events.rs:378`) — **before** `WriteLock::acquire(&root)` is called on the next line. Two mutating operations can run before the OS-held writer lease exists. Confirmed by direct read of `open_write`'s body.
3. **Raw event methods bypass writer binding.** `EventLog::append` (`events.rs:238`, `pub fn append(&self, ...)`) requires only an `EventLog` handle, not a `VaultWriter`. `EventLog::open` (`events.rs:202`) itself performs `std::fs::create_dir_all(control_dir)` — a mutation — and returns a handle with no writer check at all. Any code path holding an `EventLog` (obtainable without ever proving writer ownership) can call `append` directly. The safe path, `append_for_writer` (`events.rs:216`), exists and delegates to `append` after verifying the writer's `control_dir` matches — but nothing stops a caller from skipping it and calling `append` directly, since both are `pub`.

## Full classification

| Function | File:line | Class | Writer-bound today? | T01-01 disposition |
|---|---|---|---|---|
| `Vault::new` | vault.rs:35 | internal constructor, no I/O | n/a | unchanged |
| `is_supported` | vault.rs:63 | pure/read | n/a | unchanged |
| `Vault::create` | vault.rs:111 | **mutate** (creates control dir, metadata, then delegates to `open_write`) | Explicit creation path, not a bypass | keep as explicit-creation entry point |
| `Vault::open_write` | vault.rs:123 | **mutate before lock** (metadata ensure + torn-tail repair before `WriteLock::acquire`) | No — mutates ahead of ownership | **fix**: reorder so ownership is acquired before any repair-capable operation, or make the pre-lock steps genuinely read-only with repair deferred to an explicit post-lock step |
| `Vault::open_read` | vault.rs:140 | **mutate** (auto-creates metadata via `ensure_vault_meta`) | No — a "read" open can write | **fix**: split into a genuinely nonmutating inspection path; auto-upcast becomes an explicit, writer-bound migration step |
| `Vault::require_vault` | vault.rs:150 | read (existence check) | n/a | unchanged |
| `Vault::vault_meta` | vault.rs:161 | read | n/a | unchanged |
| `Vault::root` | vault.rs:170 | read (accessor) | n/a | unchanged |
| `Vault::control_dir` | vault.rs:174 | read (accessor) | n/a | unchanged |
| `Vault::has_write_lock` | vault.rs:178 | read (accessor) | n/a | unchanged |
| `Vault::scan` | vault.rs:187 | read (directory scan) | n/a | unchanged |
| `Vault::add_object` (inherent, runtime-checked) | vault.rs:290 | **mutate**, runtime `has_write_lock()` self-check | Runtime-checked, not type-checked | **narrow**: retain only where the type-checked `VaultWriter::add_object` cannot be used yet; document as the legacy path per its own doc comment |
| `Vault::writer` | vault.rs:329 | read (capability accessor; fails if no lock) | n/a | unchanged — this is the correct chokepoint entry |
| `VaultWriter::vault` | vault.rs:385 | read (accessor) | n/a | unchanged |
| `VaultWriter::add_object` | vault.rs:390 | **mutate**, type-bound to a held writer | Yes | keep as the preferred path; T01-03 builds the atomic transaction on top of this |
| `VaultWriter::append_event` | vault.rs:401 | **mutate**, type-bound to a held writer | Yes | keep |
| `ensure_vault_meta` | vault.rs:460 | **mutate** (conditional create) | No inherent check | **fix**: callable only from an already-ownership-proven or explicitly-migration path, never from a plain read |
| `atomic_write_file` / `atomic_write_file_with_fault` | vault.rs:504,521 | **mutate**, low-level primitive | Caller's responsibility | unchanged as a primitive; callers audited individually above |
| `startup_integrity_check` | vault.rs:629 | **mutate** (may quarantine/repair via `EventLog::quarantine_and_repair_torn_tail`) | No inherent ownership check of its own | **fix**: must run only after writer ownership (or exclusive recovery access) is held, never as a bare pre-lock step |
| `EventLog::hash_bytes` | events.rs:85 | pure | n/a | unchanged |
| `EventLog::open` | events.rs:202 | **mutate** (`create_dir_all`) | No | **fix**: a genuinely readonly open must not create the control directory; creation belongs to `Vault::create` only |
| `EventLog::append_for_writer` | events.rs:216 | **mutate**, writer-bound (verifies `control_dir` match) | Yes | keep as the sanctioned path |
| `EventLog::append` | events.rs:238 | **mutate**, **not writer-bound** | **No — this is the bypass** | **fix**: make private or otherwise unreachable except via `append_for_writer`; no public unbound mutator should remain |
| `EventLog::read_all` | events.rs:279 | read | n/a | unchanged |
| `EventLog::verify` | events.rs:301 | read (hash-chain check) | n/a | unchanged |
| `EventLog::path` | events.rs:333 | read (accessor) | n/a | unchanged |
| `EventLog::detect_torn_tail` | events.rs:343 | read (detection only) | n/a | unchanged |
| `EventLog::quarantine_and_repair_torn_tail` | events.rs:378 | **mutate** (repair) | No inherent ownership check | **fix**: must require the same exclusive access `startup_integrity_check` will be required to hold once fixed; T01-04 additionally requires preserve-before-repair semantics here |
| `Locator::new` / `as_str` | locator.rs:31,34 | pure data type | n/a | unchanged |
| `open_confined` | locator.rs:79 | read (confined file open) | n/a | unchanged |
| `read_verified` | locator.rs:128 | read (verified read) | n/a | unchanged |
| `cli::run` | cli.rs:78 | dispatcher — delegates to the above | n/a | unchanged; downstream calls inherit whatever fix lands in `vault.rs`/`events.rs` |

## Summary

- **6 confirmed mutation-capable paths reachable without proven writer ownership**: `Vault::open_write`'s pre-lock steps, `Vault::open_read`, `ensure_vault_meta`, `startup_integrity_check`, `EventLog::open`, `EventLog::append`.
- **3 correctly writer-bound mutators already exist** and are the pattern to extend, not replace: `VaultWriter::add_object`, `VaultWriter::append_event`, `EventLog::append_for_writer`.
- **1 runtime-checked (not type-checked) legacy mutator** flagged for narrowing: `Vault::add_object` (inherent).

This inventory is the concrete "enumerate every old canonical mutator" deliverable T00-02's contract requires. `T01-01`'s implementation must close every "fix" row above; `T01-07`'s native fault pass must re-audit this table against the code as actually shipped, per `checklist.md`.
