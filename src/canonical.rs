//! Format-2 canonical transaction store (T01-02).
//!
//! Canonical build plan §13/§15/§16, task `T01-02`. This module creates and
//! (re)opens a **new, independently specified** SQLite-backed canonical
//! store, isolated from the format-1 markdown-tree [`crate::vault::Vault`].
//!
//! **Scope boundary — read before extending this module.** `T01-02`
//! published "Create" only: an empty, verified store with a single
//! identity/head-summary row, no writer-lease integration, no mutation API.
//! `T01-03` (this module's second layer, schema version 2) adds exactly what
//! its own contract authorizes — "Core transaction/admission API... revision/
//! history/command tables" — and nothing from plan §15 beyond that:
//!
//! - [`CanonicalStore::writer`] binds mutation to the OS-held single-writer
//!   lease (`crate::vault::WriteLock`, reused rather than duplicated — see
//!   that type's doc comment), returning a [`CanonicalWriter`] capability
//!   that structurally proves ownership, mirroring
//!   `crate::vault::Vault::writer`/`VaultWriter`.
//! - [`CanonicalWriter::commit`] admits one opaque-payload "record" per
//!   command (create or update), atomically persisting the new revision,
//!   the updated current-pointer, the command's own result, and the
//!   advanced transaction head in one SQLite transaction (I04). A duplicate
//!   command (same `command_id`, same input digest) returns the original
//!   result without re-executing; a changed digest under the same
//!   `command_id` is rejected (§18, F01).
//! - **Not** implemented here: typed `Project`/`Note`/`Action`/`Decision`
//!   records, multi-object commands, or ordered multi-operation commands —
//!   `T01-03`'s own objective is "save **a minimal record**", and the
//!   typed/UX record model is `P02`'s job once the command API exists to
//!   populate it. Building those now would be exactly the "successor work"
//!   this task's forbidden scope excludes.
//!
//! Every schema addition bumps `CANONICAL_SCHEMA_VERSION`/
//! `CANONICAL_MIN_READER_CAPABILITY` and extends the exact-recognition list
//! in [`assert_schema_recognized`] — a build that does not know about a
//! table cannot silently treat it as absent-and-fine.
//!
//! ## On-disk layout (published)
//!
//! ```text
//! <vault root>/.fehrest/vault.json         guard: format_version = 2 (VaultMeta)
//! <vault root>/.fehrest/canonical.sqlite   the canonical database
//! ```
//!
//! The guard reuses [`VaultMeta`] — same file name, same JSON shape the
//! format-1 reader already parses — with `format_version = 2`. Format-1's
//! own `read_vault_meta` already refuses any `format_version` greater than
//! `SUPPORTED_FORMAT_VERSION` (`1`), so an old format-1 binary refusing a
//! format-2 guard is existing, tested behavior in `vault.rs`; this module
//! adds a direct integration test proving it against a *real* published
//! format-2 store rather than only the pre-existing synthetic fixture.
//!
//! Full published format documentation, exact table DDL and a generic
//! (non-Flake) reader example live in `docs/formats/format-2-canonical-sqlite.md`.
//!
//! ## Creation protocol (§16)
//!
//! "New initialization occurs in a new sibling staging directory and becomes
//! selectable only after verification and durable publication. Never
//! overwrite an existing path." Concretely: build the guard and database in
//! a uniquely named `.fehrest.staging-<uuid7>` sibling directory, reopen the
//! database independently (read-only) to verify the schema is exactly what
//! was just written, then publish by renaming the staging directory onto
//! `.fehrest`. Correctness of "never overwrite" rests on `fs::rename`'s own
//! failure when the destination already exists (both POSIX `rename(2)` and
//! Win32 `MoveFileExW` refuse to replace an existing directory) — the
//! upfront `exists()` checks in this module are a fast, friendly error path,
//! not the enforcement mechanism. A single crashed/failed creator always
//! leaves at most an orphaned staging directory behind and never touches
//! `.fehrest`; this module removes the staging directory on every failure
//! path so a retry sees a clean root. True concurrent-multi-process creation
//! racing on a brand-new root (as opposed to one process failing) is not
//! this task's acceptance clause and is not claimed here.

use crate::vault::{atomic_write_file, VaultMeta, WriteLock, CONTROL_DIR, VAULT_META_FILE};
use crate::{Error, Result};
use rusqlite::{Connection, OpenFlags, OptionalExtension};
use std::fs;
use std::path::{Path, PathBuf};

/// Guard `format_version` value that identifies a format-2 canonical store.
///
/// `crate::vault::SUPPORTED_FORMAT_VERSION` (`1`) is deliberately left
/// unchanged by this task: it is exactly the value that makes the existing
/// format-1 reader refuse this guard (F05, S03/S09 row, "old binary refuses
/// format 2").
pub const CANONICAL_FORMAT_VERSION: u32 = 2;

/// SQLite database file name inside the published `.fehrest` control dir.
pub const CANONICAL_DB_FILE: &str = "canonical.sqlite";

/// Logical schema version stored inside `canonical.sqlite`. Distinct from
/// the guard's `format_version`: this may advance for a compatible,
/// in-place schema addition without moving the outer format epoch.
/// `T01-02` published version `1` (identity/head-summary table only);
/// `T01-03` publishes version `2`, adding `revision`/`current_object`/
/// `command` (see [`assert_schema_recognized`]).
pub const CANONICAL_SCHEMA_VERSION: i64 = 2;

/// Minimum reader capability required to open this schema version. Bumped
/// only on a breaking change; equals `CANONICAL_SCHEMA_VERSION` until one
/// occurs. A future writer that raises this above what a given build
/// implements must be refused by that build, not guessed at (§15 "current
/// schema/min-reader capabilities"). Bumped to `2` alongside the schema
/// version in `T01-03`: a `T01-02`-only build does not understand the
/// revision/command tables and must not attempt to write through them.
pub const CANONICAL_MIN_READER_CAPABILITY: i64 = 2;

/// Plan §13 tie-break: "choose the smallest configuration meeting §27
/// (tie: 4096 and 8 MiB)".
const PAGE_SIZE: i64 = 4096;
/// `PRAGMA cache_size`: a negative value is interpreted by SQLite as
/// kibibytes rather than pages. `-8192` = 8192 KiB = 8 MiB.
const CACHE_SIZE_KIB_NEGATIVE: i64 = -8192;

/// Exact recognized table names, sorted (matches
/// `SELECT name FROM sqlite_master WHERE type='table' ORDER BY name`). Any
/// database with a different table set — one missing, one extra, however
/// innocuous-looking — is refused rather than silently admitted (S04,
/// "incompatible or unexpected schema refuses writes").
const EXPECTED_TABLES: [&str; 4] = ["canonical_vault", "command", "current_object", "revision"];

/// Exact recognized column `(name, declared_type)` pairs for one table, in
/// `PRAGMA table_info` order. A renamed or retyped column is refused just
/// like an extra table.
fn expected_columns(table: &str) -> Vec<(String, String)> {
    let raw: &[(&str, &str)] = match table {
        "canonical_vault" => &[
            ("singleton", "INTEGER"),
            ("vault_id", "TEXT"),
            ("schema_version", "INTEGER"),
            ("min_reader_capability", "INTEGER"),
            ("created_by_version", "TEXT"),
            ("created_at", "TEXT"),
            ("transaction_head_seq", "INTEGER"),
            ("transaction_head_hash", "TEXT"),
        ],
        "revision" => &[
            ("revision_id", "TEXT"),
            ("object_id", "TEXT"),
            ("parent_revision_id", "TEXT"),
            ("recorded_seq", "INTEGER"),
            ("recorded_at", "TEXT"),
            ("actor", "TEXT"),
            ("origin", "TEXT"),
            ("payload", "TEXT"),
            ("payload_sha256", "TEXT"),
        ],
        "current_object" => &[
            ("object_id", "TEXT"),
            ("current_revision_id", "TEXT"),
            ("tombstoned", "INTEGER"),
        ],
        "command" => &[
            ("command_id", "TEXT"),
            ("input_digest", "TEXT"),
            ("actor", "TEXT"),
            ("recorded_seq", "INTEGER"),
            ("recorded_at", "TEXT"),
            ("previous_head_seq", "INTEGER"),
            ("previous_head_hash", "TEXT"),
            ("object_id", "TEXT"),
            ("revision_id", "TEXT"),
            ("resulting_head_seq", "INTEGER"),
            ("resulting_head_hash", "TEXT"),
        ],
        other => unreachable!("expected_columns called for unrecognized table {other}"),
    };
    raw.iter()
        .map(|(n, t)| (n.to_string(), t.to_string()))
        .collect()
}

/// A published, open format-2 canonical store.
#[derive(Debug)]
pub struct CanonicalStore {
    conn: Connection,
    root: PathBuf,
    vault_id: String,
}

/// Injected failure points for `create`'s staged-publication protocol
/// (V06 fault injection). Every variant must leave the eventual root with no
/// published `.fehrest` and no leftover staging directory — production code
/// never passes a fault; this exists for the test in this module that
/// proves the "interrupted creation leaves old paths intact" acceptance
/// clause at each real stage rather than only the happy path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CreateFaultPoint {
    AfterStagingDirCreated,
    AfterGuardWritten,
    AfterSchemaInit,
    BeforePublish,
}

impl CanonicalStore {
    /// Create and publish a new format-2 canonical store at `root`.
    ///
    /// Refuses if `root/.fehrest` already exists (no-clobber: this may be a
    /// format-1 vault, an already-published format-2 store, or unrelated
    /// content — never guessed at or replaced).
    pub fn create(root: impl AsRef<Path>) -> Result<Self> {
        Self::create_with_fault(root, None)
    }

    pub(crate) fn create_with_fault(
        root: impl AsRef<Path>,
        fault: Option<CreateFaultPoint>,
    ) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        let published_control = root.join(CONTROL_DIR);
        if published_control.exists() {
            return Err(Error::Canonical(format!(
                "cannot create format-2 canonical store: {} already exists (no-clobber; open the existing vault instead)",
                published_control.display()
            )));
        }
        fs::create_dir_all(&root)
            .map_err(|e| Error::Canonical(format!("cannot create vault root: {e}")))?;

        let staging = root.join(format!(".fehrest.staging-{}", uuid::Uuid::now_v7()));
        fs::create_dir_all(&staging)
            .map_err(|e| Error::Canonical(format!("cannot create staging dir: {e}")))?;
        if fault == Some(CreateFaultPoint::AfterStagingDirCreated) {
            let _ = fs::remove_dir_all(&staging);
            return Err(Error::Canonical(
                "injected fault: AfterStagingDirCreated".into(),
            ));
        }

        let vault_id = uuid::Uuid::now_v7().to_string();
        let meta = VaultMeta::new(
            vault_id.clone(),
            CANONICAL_FORMAT_VERSION,
            env!("CARGO_PKG_VERSION").to_string(),
            now_iso8601_placeholder(),
        );
        if let Err(e) = write_guard_in_dir(&staging, &meta) {
            let _ = fs::remove_dir_all(&staging);
            return Err(e);
        }
        if fault == Some(CreateFaultPoint::AfterGuardWritten) {
            let _ = fs::remove_dir_all(&staging);
            return Err(Error::Canonical("injected fault: AfterGuardWritten".into()));
        }

        let db_path = staging.join(CANONICAL_DB_FILE);
        if db_path.exists() {
            let _ = fs::remove_dir_all(&staging);
            return Err(Error::Canonical(format!(
                "staging database already exists unexpectedly: {}",
                db_path.display()
            )));
        }
        if let Err(e) = init_database(&db_path, &vault_id) {
            let _ = fs::remove_dir_all(&staging);
            return Err(e);
        }
        if fault == Some(CreateFaultPoint::AfterSchemaInit) {
            let _ = fs::remove_dir_all(&staging);
            return Err(Error::Canonical("injected fault: AfterSchemaInit".into()));
        }

        // Independent re-open verification before publication: never trust
        // the write-time connection's own view of what it just wrote.
        if let Err(e) = verify_staged_database(&db_path, &vault_id) {
            let _ = fs::remove_dir_all(&staging);
            return Err(e);
        }

        if fault == Some(CreateFaultPoint::BeforePublish) {
            let _ = fs::remove_dir_all(&staging);
            return Err(Error::Canonical("injected fault: BeforePublish".into()));
        }

        if let Err(e) = fs::rename(&staging, &published_control) {
            let _ = fs::remove_dir_all(&staging);
            return Err(Error::Canonical(format!(
                "cannot publish staged canonical store to {}: {e} (no-clobber: destination exists or was created concurrently)",
                published_control.display()
            )));
        }

        Self::open(&root)
    }

    /// Open an existing published format-2 canonical store.
    ///
    /// Re-asserts every required `PRAGMA` on this connection (they are
    /// per-connection, not persisted) and refuses any schema this module
    /// does not exactly recognize, including a guard/database identity
    /// mismatch and a database that declares a `min_reader_capability`
    /// higher than this build implements.
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        let control = root.join(CONTROL_DIR);
        let meta = read_guard(&control)?.ok_or_else(|| {
            Error::Canonical(format!(
                "no canonical store guard at {}; not a format-2 vault",
                control.join(VAULT_META_FILE).display()
            ))
        })?;
        if meta.format_version != CANONICAL_FORMAT_VERSION {
            return Err(Error::Canonical(format!(
                "guard declares format_version {}, the format-2 store API only opens format_version {} (use crate::vault::Vault for format 1)",
                meta.format_version, CANONICAL_FORMAT_VERSION
            )));
        }
        let db_path = control.join(CANONICAL_DB_FILE);
        if !db_path.exists() {
            return Err(Error::Canonical(format!(
                "canonical guard present but database missing: {}",
                db_path.display()
            )));
        }
        let conn = Connection::open_with_flags(
            &db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|e| Error::Canonical(format!("cannot open canonical db: {e}")))?;
        apply_and_assert_runtime_pragmas(&conn)?;
        assert_page_size(&conn)?;
        assert_schema_recognized(&conn, &meta.vault_id)?;
        Ok(CanonicalStore {
            conn,
            root,
            vault_id: meta.vault_id,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn control_dir(&self) -> PathBuf {
        self.root.join(CONTROL_DIR)
    }

    pub fn db_path(&self) -> PathBuf {
        self.control_dir().join(CANONICAL_DB_FILE)
    }

    pub fn vault_id(&self) -> &str {
        &self.vault_id
    }

    pub fn schema_version(&self) -> Result<i64> {
        self.conn
            .query_row(
                "SELECT schema_version FROM canonical_vault WHERE singleton = 1",
                [],
                |r| r.get(0),
            )
            .map_err(|e| Error::Canonical(format!("cannot read schema_version: {e}")))
    }

    /// `(transaction_head_seq, transaction_head_hash)`. `(0, None)` until
    /// the first command commits.
    pub fn transaction_head(&self) -> Result<(i64, Option<String>)> {
        self.conn
            .query_row(
                "SELECT transaction_head_seq, transaction_head_hash FROM canonical_vault WHERE singleton = 1",
                [],
                |r| Ok((r.get::<_, i64>(0)?, r.get::<_, Option<String>>(1)?)),
            )
            .map_err(|e| Error::Canonical(format!("cannot read transaction head: {e}")))
    }

    /// Acquire the OS-held single-writer lease and return a capability that
    /// structurally proves ownership for its lifetime (mirrors
    /// `crate::vault::Vault::writer`/`VaultWriter`). At most one
    /// `CanonicalWriter` can exist for a given root at a time, both across
    /// processes (the OS-held `writer.lock` lease) and within this process
    /// (`&mut self` makes a second concurrent call a borrow-check error, not
    /// only a runtime one).
    pub fn writer(&mut self) -> Result<CanonicalWriter<'_>> {
        let lock = WriteLock::acquire(&self.root)?;
        Ok(CanonicalWriter {
            store: self,
            _lock: lock,
        })
    }

    /// `(revision_id, payload)` for `object_id`'s current revision, or
    /// `None` if no object with that ID has ever been admitted.
    pub fn read_current(&self, object_id: &str) -> Result<Option<(String, String)>> {
        self.conn
            .query_row(
                "SELECT r.revision_id, r.payload
                 FROM current_object c JOIN revision r ON r.revision_id = c.current_revision_id
                 WHERE c.object_id = ?1",
                rusqlite::params![object_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(|e| Error::Canonical(format!("cannot read current object: {e}")))
    }

    /// Full revision history for `object_id`, oldest first: `(revision_id,
    /// parent_revision_id, payload)`. Reconstructed purely from the
    /// immutable `revision` table — independent of, and cross-checkable
    /// against, `current_object`'s summary pointer (§16 "full history
    /// reconstructs current state and head").
    pub fn history(&self, object_id: &str) -> Result<Vec<(String, Option<String>, String)>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT revision_id, parent_revision_id, payload FROM revision
                 WHERE object_id = ?1 ORDER BY recorded_seq ASC",
            )
            .map_err(|e| Error::Canonical(format!("cannot prepare history query: {e}")))?;
        let rows = stmt
            .query_map(rusqlite::params![object_id], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, Option<String>>(1)?,
                    r.get::<_, String>(2)?,
                ))
            })
            .map_err(|e| Error::Canonical(format!("cannot run history query: {e}")))?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|e| Error::Canonical(format!("cannot read history rows: {e}")))
    }

    /// The stored, already-committed outcome for `command_id`, if any.
    /// Distinct from calling `commit` again with the same input: this never
    /// mutates, and is how a caller reconciles an outcome-unknown state
    /// after a lost acknowledgement without resubmitting anything (F01).
    pub fn command_outcome(&self, command_id: &str) -> Result<Option<CommandOutcome>> {
        Ok(self.read_command_row(command_id)?.map(|c| CommandOutcome {
            command_id: c.command_id,
            object_id: c.object_id,
            revision_id: c.revision_id,
            resulting_head_seq: c.resulting_head_seq,
            resulting_head_hash: c.resulting_head_hash,
            replay: true,
        }))
    }

    fn read_command_row(&self, command_id: &str) -> Result<Option<StoredCommand>> {
        self.conn
            .query_row(
                "SELECT command_id, input_digest, object_id, revision_id, resulting_head_seq, resulting_head_hash
                 FROM command WHERE command_id = ?1",
                rusqlite::params![command_id],
                |r| {
                    Ok(StoredCommand {
                        command_id: r.get(0)?,
                        input_digest: r.get(1)?,
                        object_id: r.get(2)?,
                        revision_id: r.get(3)?,
                        resulting_head_seq: r.get(4)?,
                        resulting_head_hash: r.get(5)?,
                    })
                },
            )
            .optional()
            .map_err(|e| Error::Canonical(format!("cannot read command: {e}")))
    }
}

struct StoredCommand {
    command_id: String,
    input_digest: String,
    object_id: String,
    revision_id: String,
    resulting_head_seq: i64,
    resulting_head_hash: String,
}

/// Who/what declared this record, per plan §15's `origin` envelope field.
/// A caller-declared value, never extracted from payload content (§17:
/// machine-owned fields are Core-assigned, not copied as authority from
/// inbound text).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordOrigin {
    User,
    Import,
    AgentProposal,
    Migration,
    System,
}

impl RecordOrigin {
    fn as_str(self) -> &'static str {
        match self {
            RecordOrigin::User => "user",
            RecordOrigin::Import => "import",
            RecordOrigin::AgentProposal => "agent-proposal",
            RecordOrigin::Migration => "migration",
            RecordOrigin::System => "system",
        }
    }
}

/// One command's requested mutation. `T01-03` admits exactly one opaque
/// UTF-8 payload per command against exactly one object — "a minimal
/// record" per the task's own objective. Multi-object/multi-operation
/// commands are not this task's scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandTarget {
    /// Admit a brand-new object. Its `object_id` is allocated by this call
    /// (I03: identity is Core-assigned, never caller-supplied).
    CreateObject { payload: String },
    /// Replace the current revision of an existing object.
    /// `expected_revision_id` must equal the object's actual current
    /// revision or the command is rejected as a conflict — never a silent
    /// last-writer-wins (I07).
    UpdateObject {
        object_id: String,
        expected_revision_id: String,
        payload: String,
    },
}

/// One command submission. `command_id` is the caller-supplied idempotency
/// key: resubmitting the exact same `CommandInput` under the same
/// `command_id` returns the original result; resubmitting a *different*
/// `CommandInput` under the same `command_id` is rejected (§18).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandInput {
    pub command_id: String,
    pub actor: String,
    pub origin: RecordOrigin,
    pub target: CommandTarget,
}

/// The durable result of a committed (or replayed) command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutcome {
    pub command_id: String,
    pub object_id: String,
    pub revision_id: String,
    pub resulting_head_seq: i64,
    pub resulting_head_hash: String,
    /// `true` when this call performed no new mutation: an identical
    /// `command_id` + input digest was already committed, and this is that
    /// original result returned again (idempotent replay).
    pub replay: bool,
}

/// A capability that structurally proves this process holds the OS-held
/// single-writer lease for a [`CanonicalStore`]. Obtained only via
/// [`CanonicalStore::writer`]; its private `_lock` field cannot be
/// constructed from outside this module (mirrors `vault::VaultWriter`'s
/// `_private: ()` pattern).
#[derive(Debug)]
pub struct CanonicalWriter<'a> {
    store: &'a mut CanonicalStore,
    _lock: WriteLock,
}

/// Injected failure points for `commit`'s transaction protocol (V06/V07).
/// Production code never passes a fault; this exists for this module's own
/// tests proving "complete pre-state or complete committed state" and F01's
/// outcome-unknown reconciliation, as a deterministic in-process fault
/// adapter rather than a literal cross-process kill (this repository's
/// established methodology — see `docs/evidence/flake-v1/T01-01/REPORT.md`,
/// which used the same in-process approach for its own empirical
/// regression proof).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommitFaultPoint {
    /// After every row is written inside the SQL transaction but before
    /// `COMMIT`: dropping the transaction here rolls everything in it back.
    /// Proves a failure at this point leaves zero trace (F01, "complete
    /// pre-state or complete committed state", never a half-state).
    BeforeSqlCommit,
    /// After the SQL transaction durably commits but before the in-memory
    /// `CommandOutcome` is constructed and returned: simulates a crashed or
    /// lost acknowledgement. Proves that retrying with the exact same
    /// `CommandInput`/`command_id` reconciles to the real committed outcome
    /// (`replay: true`) rather than re-executing or minting a second
    /// command for one logical request (F01 "reconcile command ID after
    /// reopen... never issue a replacement ID automatically").
    AfterSqlCommitBeforeReturn,
}

impl<'a> CanonicalWriter<'a> {
    pub fn store(&self) -> &CanonicalStore {
        self.store
    }

    /// Commit one command atomically. See module docs and `CommandTarget`/
    /// `CommandOutcome` for the exact contract.
    pub fn commit(&mut self, input: CommandInput) -> Result<CommandOutcome> {
        self.commit_with_fault(input, None)
    }

    pub(crate) fn commit_with_fault(
        &mut self,
        input: CommandInput,
        fault: Option<CommitFaultPoint>,
    ) -> Result<CommandOutcome> {
        let payload = match &input.target {
            CommandTarget::CreateObject { payload } => payload,
            CommandTarget::UpdateObject { payload, .. } => payload,
        };
        if payload.len() > crate::limits::MAX_OBJECT_BYTES {
            return Err(Error::Canonical(format!(
                "payload exceeds limit: {} > {} bytes",
                payload.len(),
                crate::limits::MAX_OBJECT_BYTES
            )));
        }
        let payload_sha256 = crate::events::hash_bytes(payload.as_bytes());
        let input_digest = compute_input_digest(&input, &payload_sha256);

        // Idempotency / conflict check — outside any new write transaction,
        // and before this command touches canonical_vault's head at all.
        if let Some(existing) = self.store.read_command_row(&input.command_id)? {
            if existing.input_digest == input_digest {
                return Ok(CommandOutcome {
                    command_id: existing.command_id,
                    object_id: existing.object_id,
                    revision_id: existing.revision_id,
                    resulting_head_seq: existing.resulting_head_seq,
                    resulting_head_hash: existing.resulting_head_hash,
                    replay: true,
                });
            }
            return Err(Error::Canonical(format!(
                "command_id {} was already committed with a different input; refusing (changed digest under the same command_id)",
                input.command_id
            )));
        }

        let tx = self
            .store
            .conn
            .transaction()
            .map_err(|e| Error::Canonical(format!("cannot begin transaction: {e}")))?;

        let (prev_head_seq, prev_head_hash): (i64, Option<String>) = tx
            .query_row(
                "SELECT transaction_head_seq, transaction_head_hash FROM canonical_vault WHERE singleton = 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .map_err(|e| Error::Canonical(format!("cannot read transaction head: {e}")))?;

        let (object_id, parent_revision_id): (String, Option<String>) = match &input.target {
            CommandTarget::CreateObject { .. } => (uuid::Uuid::now_v7().to_string(), None),
            CommandTarget::UpdateObject {
                object_id,
                expected_revision_id,
                ..
            } => {
                let current: Option<String> = tx
                    .query_row(
                        "SELECT current_revision_id FROM current_object WHERE object_id = ?1",
                        rusqlite::params![object_id],
                        |r| r.get(0),
                    )
                    .optional()
                    .map_err(|e| Error::Canonical(format!("cannot read current pointer: {e}")))?;
                let current = current.ok_or_else(|| {
                    Error::Canonical(format!("cannot update unknown object_id {object_id}"))
                })?;
                if &current != expected_revision_id {
                    return Err(Error::Canonical(format!(
                        "expected revision conflict: object {object_id} is at revision {current}, not the expected {expected_revision_id}"
                    )));
                }
                (object_id.clone(), Some(expected_revision_id.clone()))
            }
        };

        let revision_id = uuid::Uuid::now_v7().to_string();
        let recorded_seq = prev_head_seq + 1;
        let recorded_at = now_iso8601_placeholder();

        tx.execute(
            "INSERT INTO revision
                (revision_id, object_id, parent_revision_id, recorded_seq, recorded_at, actor, origin, payload, payload_sha256)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                revision_id,
                object_id,
                parent_revision_id,
                recorded_seq,
                recorded_at,
                input.actor,
                input.origin.as_str(),
                payload,
                payload_sha256
            ],
        )
        .map_err(|e| Error::Canonical(format!("cannot insert revision: {e}")))?;

        tx.execute(
            "INSERT INTO current_object (object_id, current_revision_id, tombstoned) VALUES (?1, ?2, 0)
             ON CONFLICT(object_id) DO UPDATE SET current_revision_id = excluded.current_revision_id",
            rusqlite::params![object_id, revision_id],
        )
        .map_err(|e| Error::Canonical(format!("cannot update current pointer: {e}")))?;

        let resulting_head_hash = crate::events::hash_bytes(
            format!(
                "flake-canonical-tx-v1|{}|{}|{}",
                prev_head_hash.clone().unwrap_or_default(),
                revision_id,
                input_digest
            )
            .as_bytes(),
        );

        tx.execute(
            "INSERT INTO command
                (command_id, input_digest, actor, recorded_seq, recorded_at, previous_head_seq,
                 previous_head_hash, object_id, revision_id, resulting_head_seq, resulting_head_hash)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                input.command_id,
                input_digest,
                input.actor,
                recorded_seq,
                recorded_at,
                prev_head_seq,
                prev_head_hash,
                object_id,
                revision_id,
                recorded_seq,
                resulting_head_hash
            ],
        )
        .map_err(|e| Error::Canonical(format!("cannot insert command: {e}")))?;

        tx.execute(
            "UPDATE canonical_vault SET transaction_head_seq = ?1, transaction_head_hash = ?2 WHERE singleton = 1",
            rusqlite::params![recorded_seq, resulting_head_hash],
        )
        .map_err(|e| Error::Canonical(format!("cannot advance transaction head: {e}")))?;

        if fault == Some(CommitFaultPoint::BeforeSqlCommit) {
            // Dropping `tx` without calling `.commit()` rolls back every
            // statement executed against it above.
            return Err(Error::Canonical("injected fault: BeforeSqlCommit".into()));
        }

        tx.commit()
            .map_err(|e| Error::Canonical(format!("cannot commit transaction: {e}")))?;

        if fault == Some(CommitFaultPoint::AfterSqlCommitBeforeReturn) {
            return Err(Error::Canonical(
                "injected fault: AfterSqlCommitBeforeReturn (already durably committed; retry with the same command_id to reconcile the outcome)".into(),
            ));
        }

        Ok(CommandOutcome {
            command_id: input.command_id,
            object_id,
            revision_id,
            resulting_head_seq: recorded_seq,
            resulting_head_hash,
            replay: false,
        })
    }
}

/// Deliberately narrower than full RFC 8785 JCS (§15): `serde_json::Value`'s
/// `Map` is `BTreeMap`-backed in this crate (the `preserve_order` feature is
/// not enabled — see `Cargo.toml`), so serializing through `Value` yields
/// object keys in sorted order, which is JCS's core "canonical key order"
/// property. Full JCS also mandates Unicode NFC normalization and a
/// non-finite-number rejection rule; this digest input has no float fields
/// by construction (every value here is a string, an `Option<&str>`, or a
/// fixed integer tag), so the number rule cannot bite, and no free-text
/// field is JCS-normalized here — recorded as a scope limitation in
/// `docs/formats/format-2-canonical-sqlite.md`, not silently claimed as
/// full JCS compliance. Cross-language golden vectors (§15) are not
/// produced by this task: there is no second implementation to vector
/// against yet, and this digest is used only for this store's own
/// in-process idempotency check, not yet an interop wire format.
fn compute_input_digest(input: &CommandInput, payload_sha256: &str) -> String {
    let (target_kind, object_id, expected_revision_id): (&str, Option<&str>, Option<&str>) =
        match &input.target {
            CommandTarget::CreateObject { .. } => ("create_object", None, None),
            CommandTarget::UpdateObject {
                object_id,
                expected_revision_id,
                ..
            } => (
                "update_object",
                Some(object_id.as_str()),
                Some(expected_revision_id.as_str()),
            ),
        };
    let value = serde_json::json!({
        "version": 1,
        "command_id": input.command_id,
        "actor": input.actor,
        "origin": input.origin.as_str(),
        "target_kind": target_kind,
        "object_id": object_id,
        "expected_revision_id": expected_revision_id,
        "payload_sha256": payload_sha256,
    });
    let canonical =
        serde_json::to_string(&value).expect("json! of plain strings/options cannot fail");
    crate::events::hash_bytes(canonical.as_bytes())
}

/// Build the database file at `db_path` (which must not yet exist) with the
/// required page size, then the required runtime pragmas, then the schema
/// and its single identity row — all inside the staging directory, before
/// anything is published.
fn init_database(db_path: &Path, vault_id: &str) -> Result<()> {
    let conn = Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|e| Error::Canonical(format!("cannot create canonical db: {e}")))?;

    // page_size only takes effect before the first table is created (or
    // after a VACUUM); it must run before init_schema, and only here at
    // creation time — reopening later can only observe it, never change it.
    conn.pragma_update(None, "page_size", PAGE_SIZE)
        .map_err(|e| Error::Canonical(format!("cannot set page_size: {e}")))?;

    apply_and_assert_runtime_pragmas(&conn)?;
    init_schema(&conn, vault_id)?;

    conn.close()
        .map_err(|(_, e)| Error::Canonical(format!("cannot close canonical db after init: {e}")))
}

fn init_schema(conn: &Connection, vault_id: &str) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE canonical_vault (
            singleton               INTEGER PRIMARY KEY CHECK (singleton = 1),
            vault_id                TEXT NOT NULL,
            schema_version          INTEGER NOT NULL,
            min_reader_capability   INTEGER NOT NULL,
            created_by_version      TEXT NOT NULL,
            created_at              TEXT NOT NULL,
            transaction_head_seq    INTEGER NOT NULL,
            transaction_head_hash   TEXT
        );
        CREATE TABLE revision (
            revision_id             TEXT PRIMARY KEY,
            object_id               TEXT NOT NULL,
            parent_revision_id      TEXT,
            recorded_seq            INTEGER NOT NULL,
            recorded_at             TEXT NOT NULL,
            actor                   TEXT NOT NULL,
            origin                  TEXT NOT NULL,
            payload                 TEXT NOT NULL,
            payload_sha256          TEXT NOT NULL,
            FOREIGN KEY (parent_revision_id) REFERENCES revision(revision_id)
        );
        CREATE TABLE current_object (
            object_id               TEXT PRIMARY KEY,
            current_revision_id     TEXT NOT NULL,
            tombstoned              INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (current_revision_id) REFERENCES revision(revision_id)
        );
        CREATE TABLE command (
            command_id              TEXT PRIMARY KEY,
            input_digest            TEXT NOT NULL,
            actor                   TEXT NOT NULL,
            recorded_seq            INTEGER NOT NULL,
            recorded_at             TEXT NOT NULL,
            previous_head_seq       INTEGER NOT NULL,
            previous_head_hash      TEXT,
            object_id               TEXT NOT NULL,
            revision_id             TEXT NOT NULL,
            resulting_head_seq      INTEGER NOT NULL,
            resulting_head_hash     TEXT NOT NULL,
            FOREIGN KEY (revision_id) REFERENCES revision(revision_id)
        );",
    )
    .map_err(|e| Error::Canonical(format!("cannot create canonical schema: {e}")))?;

    conn.execute(
        "INSERT INTO canonical_vault
            (singleton, vault_id, schema_version, min_reader_capability,
             created_by_version, created_at, transaction_head_seq, transaction_head_hash)
         VALUES (1, ?1, ?2, ?3, ?4, ?5, 0, NULL)",
        rusqlite::params![
            vault_id,
            CANONICAL_SCHEMA_VERSION,
            CANONICAL_MIN_READER_CAPABILITY,
            env!("CARGO_PKG_VERSION"),
            now_iso8601_placeholder(),
        ],
    )
    .map_err(|e| Error::Canonical(format!("cannot insert canonical_vault identity row: {e}")))?;
    Ok(())
}

/// Reopen the just-written staging database independently, read-only, and
/// run the exact same recognition check `open` will later run. This is what
/// "verification" means in "becomes selectable only after verification and
/// durable publication" (§16): the connection that wrote the schema is never
/// the one that gets to also certify it.
fn verify_staged_database(db_path: &Path, expected_vault_id: &str) -> Result<()> {
    let conn = Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|e| Error::Canonical(format!("post-init verification open failed: {e}")))?;
    assert_page_size(&conn)?;
    assert_schema_recognized(&conn, expected_vault_id)
}

/// Set and immediately read back every required runtime `PRAGMA`. A pragma
/// that silently failed to apply (build variance, an unexpected SQLite
/// compile-time option) is caught here rather than trusted.
fn apply_and_assert_runtime_pragmas(conn: &Connection) -> Result<()> {
    let mode: String = conn
        .pragma_update_and_check(None, "journal_mode", "DELETE", |row| row.get(0))
        .map_err(|e| Error::Canonical(format!("cannot set journal_mode: {e}")))?;
    if !mode.eq_ignore_ascii_case("delete") {
        return Err(Error::Canonical(format!(
            "journal_mode did not apply as DELETE, observed {mode}"
        )));
    }

    conn.pragma_update(None, "synchronous", "EXTRA")
        .map_err(|e| Error::Canonical(format!("cannot set synchronous: {e}")))?;
    let sync: i64 = conn
        .query_row("PRAGMA synchronous", [], |r| r.get(0))
        .map_err(|e| Error::Canonical(format!("cannot read synchronous: {e}")))?;
    if sync != 3 {
        return Err(Error::Canonical(format!(
            "synchronous did not apply as EXTRA(3), observed {sync}"
        )));
    }

    conn.pragma_update(None, "foreign_keys", true)
        .map_err(|e| Error::Canonical(format!("cannot set foreign_keys: {e}")))?;
    let fk: i64 = conn
        .query_row("PRAGMA foreign_keys", [], |r| r.get(0))
        .map_err(|e| Error::Canonical(format!("cannot read foreign_keys: {e}")))?;
    if fk != 1 {
        return Err(Error::Canonical(format!(
            "foreign_keys did not apply as ON, observed {fk}"
        )));
    }

    conn.pragma_update(None, "trusted_schema", false)
        .map_err(|e| Error::Canonical(format!("cannot set trusted_schema: {e}")))?;
    let trusted: i64 = conn
        .query_row("PRAGMA trusted_schema", [], |r| r.get(0))
        .map_err(|e| Error::Canonical(format!("cannot read trusted_schema: {e}")))?;
    if trusted != 0 {
        return Err(Error::Canonical(format!(
            "trusted_schema did not apply as OFF, observed {trusted}"
        )));
    }

    conn.pragma_update(None, "cache_size", CACHE_SIZE_KIB_NEGATIVE)
        .map_err(|e| Error::Canonical(format!("cannot set cache_size: {e}")))?;
    let cache: i64 = conn
        .query_row("PRAGMA cache_size", [], |r| r.get(0))
        .map_err(|e| Error::Canonical(format!("cannot read cache_size: {e}")))?;
    if cache != CACHE_SIZE_KIB_NEGATIVE {
        return Err(Error::Canonical(format!(
            "cache_size did not apply as {CACHE_SIZE_KIB_NEGATIVE}, observed {cache}"
        )));
    }

    Ok(())
}

/// `page_size` is a database-level property fixed at creation; assert it
/// reads back correctly on every open, including reopens where this
/// connection never set it itself.
fn assert_page_size(conn: &Connection) -> Result<()> {
    let page_size: i64 = conn
        .query_row("PRAGMA page_size", [], |r| r.get(0))
        .map_err(|e| Error::Canonical(format!("cannot read page_size: {e}")))?;
    if page_size != PAGE_SIZE {
        return Err(Error::Canonical(format!(
            "unrecognized canonical database: expected page_size {PAGE_SIZE}, found {page_size}"
        )));
    }
    Ok(())
}

/// Refuse to open (for writing or otherwise treating as authoritative) any
/// database whose schema is not exactly the [`EXPECTED_TABLES`] this build
/// recognizes, whose identity row does not match the guard, or whose
/// declared `min_reader_capability` exceeds what this build implements
/// (S04 "incompatible or unexpected schema refuses writes"; §15 "guard/DB
/// identity agrees").
fn assert_schema_recognized(conn: &Connection, expected_vault_id: &str) -> Result<()> {
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
        .map_err(|e| Error::Canonical(format!("cannot inspect schema: {e}")))?;
    let names: Vec<String> = stmt
        .query_map([], |r| r.get::<_, String>(0))
        .map_err(|e| Error::Canonical(format!("cannot list tables: {e}")))?
        .collect::<rusqlite::Result<_>>()
        .map_err(|e| Error::Canonical(format!("cannot read table list: {e}")))?;
    if names != EXPECTED_TABLES {
        return Err(Error::Canonical(format!(
            "unrecognized canonical schema: expected exactly {EXPECTED_TABLES:?}, found {names:?}"
        )));
    }

    for table in EXPECTED_TABLES {
        let mut cols_stmt = conn
            .prepare(&format!("PRAGMA table_info({table})"))
            .map_err(|e| Error::Canonical(format!("cannot inspect {table} columns: {e}")))?;
        let cols: Vec<(String, String)> = cols_stmt
            .query_map([], |r| Ok((r.get::<_, String>(1)?, r.get::<_, String>(2)?)))
            .map_err(|e| Error::Canonical(format!("cannot list {table} columns: {e}")))?
            .collect::<rusqlite::Result<_>>()
            .map_err(|e| Error::Canonical(format!("cannot read {table} column list: {e}")))?;
        let expected = expected_columns(table);
        if cols != expected {
            return Err(Error::Canonical(format!(
                "unrecognized {table} schema: expected columns {expected:?}, found {cols:?}"
            )));
        }
    }

    let (db_vault_id, min_reader_capability): (String, i64) = conn
        .query_row(
            "SELECT vault_id, min_reader_capability FROM canonical_vault WHERE singleton = 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|e| Error::Canonical(format!("cannot read canonical_vault identity row: {e}")))?;
    if db_vault_id != expected_vault_id {
        return Err(Error::Canonical(format!(
            "guard/database identity mismatch: guard vault_id {expected_vault_id}, database vault_id {db_vault_id}"
        )));
    }
    if min_reader_capability > CANONICAL_MIN_READER_CAPABILITY {
        return Err(Error::Canonical(format!(
            "canonical database requires min_reader_capability {min_reader_capability}, this build supports {CANONICAL_MIN_READER_CAPABILITY}; refusing rather than guessing compatibility"
        )));
    }
    Ok(())
}

fn write_guard_in_dir(dir: &Path, meta: &VaultMeta) -> Result<()> {
    let content = serde_json::to_string_pretty(meta)
        .map_err(|e| Error::Canonical(format!("cannot serialize guard: {e}")))?;
    atomic_write_file(&dir.join(VAULT_META_FILE), content.as_bytes())
}

fn read_guard(control_dir: &Path) -> Result<Option<VaultMeta>> {
    let p = control_dir.join(VAULT_META_FILE);
    if !p.exists() {
        return Ok(None);
    }
    let data =
        fs::read_to_string(&p).map_err(|e| Error::Canonical(format!("guard unreadable: {e}")))?;
    let meta: VaultMeta =
        serde_json::from_str(&data).map_err(|e| Error::Canonical(format!("guard corrupt: {e}")))?;
    if uuid::Uuid::parse_str(&meta.vault_id).is_err() {
        return Err(Error::Canonical(format!(
            "guard corrupt: vault_id not a UUID: {}",
            meta.vault_id
        )));
    }
    Ok(Some(meta))
}

/// Same documented limitation as `vault.rs::chrono_like_now_iso8601`: a real
/// `chrono`/`time` dependency is not admitted for this minimal task, so this
/// emits a distinct, sortable, RFC3339-shaped placeholder rather than a
/// calendar-correct timestamp. Recorded, not hidden.
fn now_iso8601_placeholder() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("2026-09-14T{:05}Z", secs % 86400)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp() -> PathBuf {
        let d = std::env::temp_dir().join(format!("fehrest-canonical-{}", uuid::Uuid::now_v7()));
        d
    }

    fn cleanup(root: &Path) {
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn fresh_creation_publishes_guard_and_database_with_matching_identity() {
        let root = tmp();
        let store = CanonicalStore::create(&root).unwrap();
        assert!(root.join(CONTROL_DIR).join(VAULT_META_FILE).exists());
        assert!(root.join(CONTROL_DIR).join(CANONICAL_DB_FILE).exists());
        assert!(uuid::Uuid::parse_str(store.vault_id()).is_ok());
        assert_eq!(store.schema_version().unwrap(), CANONICAL_SCHEMA_VERSION);
        assert_eq!(store.transaction_head().unwrap(), (0, None));
        cleanup(&root);
    }

    #[test]
    fn reopen_after_create_is_stable_and_pragmas_hold() {
        let root = tmp();
        let created = CanonicalStore::create(&root).unwrap();
        let id = created.vault_id().to_string();
        drop(created);

        let reopened = CanonicalStore::open(&root).unwrap();
        assert_eq!(reopened.vault_id(), id);
        assert_eq!(reopened.schema_version().unwrap(), CANONICAL_SCHEMA_VERSION);

        let page_size: i64 = reopened
            .conn
            .query_row("PRAGMA page_size", [], |r| r.get(0))
            .unwrap();
        assert_eq!(page_size, PAGE_SIZE);
        let journal_mode: String = reopened
            .conn
            .query_row("PRAGMA journal_mode", [], |r| r.get(0))
            .unwrap();
        assert!(journal_mode.eq_ignore_ascii_case("delete"));
        let synchronous: i64 = reopened
            .conn
            .query_row("PRAGMA synchronous", [], |r| r.get(0))
            .unwrap();
        assert_eq!(synchronous, 3, "synchronous must be EXTRA");
        let foreign_keys: i64 = reopened
            .conn
            .query_row("PRAGMA foreign_keys", [], |r| r.get(0))
            .unwrap();
        assert_eq!(foreign_keys, 1);
        let trusted_schema: i64 = reopened
            .conn
            .query_row("PRAGMA trusted_schema", [], |r| r.get(0))
            .unwrap();
        assert_eq!(trusted_schema, 0);
        let cache_size: i64 = reopened
            .conn
            .query_row("PRAGMA cache_size", [], |r| r.get(0))
            .unwrap();
        assert_eq!(cache_size, CACHE_SIZE_KIB_NEGATIVE);
        cleanup(&root);
    }

    #[test]
    fn generic_sqlite_connection_can_inspect_published_schema() {
        // I10 portability: a connection opened through this module's own API
        // is not the proof; a bare, independently opened rusqlite connection
        // (standing in for any generic SQLite tool) must see the same thing.
        let root = tmp();
        let store = CanonicalStore::create(&root).unwrap();
        let db_path = store.db_path();
        let vault_id = store.vault_id().to_string();
        drop(store);

        let generic = Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .expect("a generic reader must be able to open the published database");
        let mut tables: Vec<String> = generic
            .prepare("SELECT name FROM sqlite_master WHERE type = 'table'")
            .unwrap()
            .query_map([], |r| r.get::<_, String>(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        tables.sort();
        assert_eq!(tables, EXPECTED_TABLES.to_vec());
        let read_id: String = generic
            .query_row(
                "SELECT vault_id FROM canonical_vault WHERE singleton = 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(read_id, vault_id);
        cleanup(&root);
    }

    #[test]
    fn create_refuses_to_clobber_existing_control_dir() {
        let root = tmp();
        fs::create_dir_all(root.join(CONTROL_DIR)).unwrap();
        fs::write(root.join(CONTROL_DIR).join("sentinel.txt"), b"pre-existing").unwrap();

        let err = CanonicalStore::create(&root).unwrap_err();
        assert!(matches!(err, Error::Canonical(_)));
        // The pre-existing control dir and its content are completely untouched.
        assert_eq!(
            fs::read(root.join(CONTROL_DIR).join("sentinel.txt")).unwrap(),
            b"pre-existing"
        );
        assert!(!root.join(CONTROL_DIR).join(CANONICAL_DB_FILE).exists());
        cleanup(&root);
    }

    #[test]
    fn second_creation_on_a_published_root_is_refused_and_first_is_untouched() {
        let root = tmp();
        let first = CanonicalStore::create(&root).unwrap();
        let first_id = first.vault_id().to_string();
        drop(first);

        let err = CanonicalStore::create(&root).unwrap_err();
        assert!(matches!(err, Error::Canonical(_)));

        let reopened = CanonicalStore::open(&root).unwrap();
        assert_eq!(
            reopened.vault_id(),
            first_id,
            "first store must be unchanged"
        );
        cleanup(&root);
    }

    #[test]
    fn legacy_format1_reader_refuses_format2_store() {
        // S03/S09, F05: the existing format-1 code path (unchanged by this
        // task) must refuse a *real* published format-2 guard, not only the
        // synthetic fixture `unsupported_newer_v2.json` used by vault.rs's
        // own pre-existing test.
        let root = tmp();
        let _store = CanonicalStore::create(&root).unwrap();

        let err_write = crate::vault::Vault::open_write(&root).unwrap_err();
        let msg = format!("{err_write}");
        assert!(
            msg.contains("unsupported vault format_version 2"),
            "got {msg}"
        );

        let err_read = crate::vault::Vault::open_read(&root).unwrap_err();
        let msg = format!("{err_read}");
        assert!(
            msg.contains("unsupported vault format_version 2"),
            "got {msg}"
        );
        cleanup(&root);
    }

    #[test]
    fn guard_database_identity_mismatch_is_refused() {
        let root = tmp();
        let store = CanonicalStore::create(&root).unwrap();
        drop(store);

        // Hand-corrupt the guard's vault_id, simulating divergence between
        // the guard and the database (disk tamper, partial restore, etc.).
        let guard_path = root.join(CONTROL_DIR).join(VAULT_META_FILE);
        let mut meta: VaultMeta =
            serde_json::from_str(&fs::read_to_string(&guard_path).unwrap()).unwrap();
        meta.vault_id = uuid::Uuid::now_v7().to_string();
        fs::write(&guard_path, serde_json::to_string_pretty(&meta).unwrap()).unwrap();

        let err = CanonicalStore::open(&root).unwrap_err();
        let msg = format!("{err}");
        assert!(
            msg.contains("guard/database identity mismatch"),
            "got {msg}"
        );
        cleanup(&root);
    }

    #[test]
    fn malformed_database_file_is_refused_not_panicking() {
        let root = tmp();
        let store = CanonicalStore::create(&root).unwrap();
        let db_path = store.db_path();
        drop(store);

        let fixture = Path::new("tests/fixtures/canonical/malformed_not_sqlite.bin");
        fs::copy(fixture, &db_path).unwrap();

        let err = CanonicalStore::open(&root).unwrap_err();
        assert!(matches!(err, Error::Canonical(_)));
        cleanup(&root);
    }

    #[test]
    fn unrecognized_extra_table_in_schema_refuses_open() {
        let root = tmp();
        let store = CanonicalStore::create(&root).unwrap();
        let db_path = store.db_path();
        drop(store);

        {
            let raw = Connection::open(&db_path).unwrap();
            raw.execute_batch("CREATE TABLE evil (x TEXT);").unwrap();
        }

        let err = CanonicalStore::open(&root).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("unrecognized canonical schema"), "got {msg}");
        cleanup(&root);
    }

    #[test]
    fn min_reader_capability_higher_than_supported_is_refused() {
        let root = tmp();
        let store = CanonicalStore::create(&root).unwrap();
        let db_path = store.db_path();
        drop(store);

        {
            let raw = Connection::open(&db_path).unwrap();
            raw.execute(
                "UPDATE canonical_vault SET min_reader_capability = ?1 WHERE singleton = 1",
                rusqlite::params![CANONICAL_MIN_READER_CAPABILITY + 1],
            )
            .unwrap();
        }

        let err = CanonicalStore::open(&root).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("min_reader_capability"), "got {msg}");
        cleanup(&root);
    }

    #[test]
    fn interrupted_creation_at_each_fault_point_leaves_root_without_published_control_dir() {
        for fault in [
            CreateFaultPoint::AfterStagingDirCreated,
            CreateFaultPoint::AfterGuardWritten,
            CreateFaultPoint::AfterSchemaInit,
            CreateFaultPoint::BeforePublish,
        ] {
            let root = tmp();
            let err = CanonicalStore::create_with_fault(&root, Some(fault)).unwrap_err();
            assert!(matches!(err, Error::Canonical(_)), "{fault:?}: {err}");
            assert!(
                !root.join(CONTROL_DIR).exists(),
                "{fault:?}: .fehrest must not be published on failure"
            );
            // No orphaned staging directory left behind either.
            if root.exists() {
                let leftovers: Vec<_> = fs::read_dir(&root)
                    .unwrap()
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name())
                    .collect();
                assert!(
                    leftovers.is_empty(),
                    "{fault:?}: leftover entries in root: {leftovers:?}"
                );
            }
            cleanup(&root);
        }
    }

    #[test]
    fn repeated_creations_have_unique_vault_ids_and_all_reopen_correctly() {
        let mut roots = Vec::new();
        let mut ids = std::collections::HashSet::new();
        for _ in 0..20 {
            let root = tmp();
            let store = CanonicalStore::create(&root).unwrap();
            assert!(
                ids.insert(store.vault_id().to_string()),
                "vault_id collision"
            );
            drop(store);
            let reopened = CanonicalStore::open(&root).unwrap();
            assert!(ids.contains(reopened.vault_id()));
            roots.push(root);
        }
        for root in roots {
            cleanup(&root);
        }
    }

    // -----------------------------------------------------------------
    // T01-03 — transaction/command admission API
    // -----------------------------------------------------------------

    fn create_input(payload: &str) -> CommandInput {
        CommandInput {
            command_id: uuid::Uuid::now_v7().to_string(),
            actor: "owner".into(),
            origin: RecordOrigin::User,
            target: CommandTarget::CreateObject {
                payload: payload.into(),
            },
        }
    }

    #[test]
    fn create_object_commits_atomically_and_advances_head() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let (before_seq, before_hash) = store.transaction_head().unwrap();
        assert_eq!((before_seq, before_hash), (0, None));

        let outcome = {
            let mut writer = store.writer().unwrap();
            writer.commit(create_input("hello")).unwrap()
        };
        assert!(!outcome.replay);
        assert_eq!(outcome.resulting_head_seq, 1);

        let (after_seq, after_hash) = store.transaction_head().unwrap();
        assert_eq!(after_seq, 1);
        assert_eq!(after_hash.unwrap(), outcome.resulting_head_hash);

        let (rev_id, payload) = store.read_current(&outcome.object_id).unwrap().unwrap();
        assert_eq!(rev_id, outcome.revision_id);
        assert_eq!(payload, "hello");
        cleanup(&root);
    }

    #[test]
    fn update_object_requires_matching_expected_revision() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let created = {
            let mut writer = store.writer().unwrap();
            writer.commit(create_input("v1")).unwrap()
        };

        // Correct expected_revision_id succeeds.
        let updated = {
            let mut writer = store.writer().unwrap();
            writer
                .commit(CommandInput {
                    command_id: uuid::Uuid::now_v7().to_string(),
                    actor: "owner".into(),
                    origin: RecordOrigin::User,
                    target: CommandTarget::UpdateObject {
                        object_id: created.object_id.clone(),
                        expected_revision_id: created.revision_id.clone(),
                        payload: "v2".into(),
                    },
                })
                .unwrap()
        };
        assert_eq!(updated.object_id, created.object_id);
        assert_ne!(updated.revision_id, created.revision_id);
        let (current_rev, payload) = store.read_current(&created.object_id).unwrap().unwrap();
        assert_eq!(current_rev, updated.revision_id);
        assert_eq!(payload, "v2");

        // Stale expected_revision_id (the now-superseded first revision) is
        // an explicit conflict, never a silent last-writer-wins.
        let stale_err = {
            let mut writer = store.writer().unwrap();
            writer
                .commit(CommandInput {
                    command_id: uuid::Uuid::now_v7().to_string(),
                    actor: "owner".into(),
                    origin: RecordOrigin::User,
                    target: CommandTarget::UpdateObject {
                        object_id: created.object_id.clone(),
                        expected_revision_id: created.revision_id.clone(),
                        payload: "v3-conflicting".into(),
                    },
                })
                .unwrap_err()
        };
        let msg = format!("{stale_err}");
        assert!(msg.contains("expected revision conflict"), "got {msg}");
        // The conflicting attempt changed nothing.
        let (current_rev, payload) = store.read_current(&created.object_id).unwrap().unwrap();
        assert_eq!(current_rev, updated.revision_id);
        assert_eq!(payload, "v2");
        cleanup(&root);
    }

    #[test]
    fn update_of_unknown_object_id_is_refused() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let mut writer = store.writer().unwrap();
        let err = writer
            .commit(CommandInput {
                command_id: uuid::Uuid::now_v7().to_string(),
                actor: "owner".into(),
                origin: RecordOrigin::User,
                target: CommandTarget::UpdateObject {
                    object_id: uuid::Uuid::now_v7().to_string(),
                    expected_revision_id: uuid::Uuid::now_v7().to_string(),
                    payload: "x".into(),
                },
            })
            .unwrap_err();
        assert!(format!("{err}").contains("unknown object_id"));
        drop(writer);
        cleanup(&root);
    }

    #[test]
    fn duplicate_command_id_with_identical_input_replays_the_original_result() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let input = create_input("once");
        let command_id = input.command_id.clone();

        let first = {
            let mut writer = store.writer().unwrap();
            writer.commit(input.clone()).unwrap()
        };
        assert!(!first.replay);
        assert_eq!(first.command_id, command_id);

        let second = {
            let mut writer = store.writer().unwrap();
            writer.commit(input).unwrap()
        };
        assert!(second.replay);
        assert_eq!(second.command_id, command_id);
        assert_eq!(
            second,
            CommandOutcome {
                replay: true,
                ..first.clone()
            }
        );

        // The replay executed no new mutation: the head did not advance
        // past the single real commit.
        assert_eq!(store.transaction_head().unwrap().0, 1);
        cleanup(&root);
    }

    #[test]
    fn duplicate_command_id_with_different_input_is_rejected() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let mut input = create_input("original");
        let command_id = input.command_id.clone();
        {
            let mut writer = store.writer().unwrap();
            writer.commit(input.clone()).unwrap();
        }

        input.target = CommandTarget::CreateObject {
            payload: "different-payload-same-id".into(),
        };
        let err = {
            let mut writer = store.writer().unwrap();
            writer.commit(input).unwrap_err()
        };
        let msg = format!("{err}");
        assert!(
            msg.contains("already committed with a different input"),
            "got {msg}"
        );
        assert!(msg.contains(&command_id));
        // Still exactly one committed transaction.
        assert_eq!(store.transaction_head().unwrap().0, 1);
        cleanup(&root);
    }

    #[test]
    fn oversized_payload_is_refused_before_any_mutation() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let huge = "x".repeat(crate::limits::MAX_OBJECT_BYTES + 1);
        let mut writer = store.writer().unwrap();
        let err = writer.commit(create_input(&huge)).unwrap_err();
        assert!(matches!(err, Error::Canonical(_)));
        drop(writer);
        assert_eq!(store.transaction_head().unwrap(), (0, None));
        cleanup(&root);
    }

    #[test]
    fn full_history_reconstructs_current_state_independent_of_pointer() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let created = {
            let mut writer = store.writer().unwrap();
            writer.commit(create_input("r1")).unwrap()
        };
        let updated = {
            let mut writer = store.writer().unwrap();
            writer
                .commit(CommandInput {
                    command_id: uuid::Uuid::now_v7().to_string(),
                    actor: "owner".into(),
                    origin: RecordOrigin::User,
                    target: CommandTarget::UpdateObject {
                        object_id: created.object_id.clone(),
                        expected_revision_id: created.revision_id.clone(),
                        payload: "r2".into(),
                    },
                })
                .unwrap()
        };

        let history = store.history(&created.object_id).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(
            history[0],
            (created.revision_id.clone(), None, "r1".to_string())
        );
        assert_eq!(
            history[1],
            (
                updated.revision_id.clone(),
                Some(created.revision_id.clone()),
                "r2".to_string()
            )
        );

        // Walking the immutable history to find the revision nothing else
        // names as a parent must agree with the maintained current pointer
        // — the two are cross-checkable, not the same code path.
        let parents: std::collections::HashSet<_> =
            history.iter().filter_map(|(_, p, _)| p.clone()).collect();
        let head_from_history = history
            .iter()
            .map(|(rev, _, _)| rev.clone())
            .find(|rev| !parents.contains(rev))
            .unwrap();
        let (current_rev, _) = store.read_current(&created.object_id).unwrap().unwrap();
        assert_eq!(head_from_history, current_rev);
        cleanup(&root);
    }

    #[test]
    fn fault_before_sql_commit_leaves_zero_trace() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let input = create_input("never-committed");
        let command_id = input.command_id.clone();
        {
            let mut writer = store.writer().unwrap();
            let err = writer
                .commit_with_fault(input, Some(CommitFaultPoint::BeforeSqlCommit))
                .unwrap_err();
            assert!(matches!(err, Error::Canonical(_)));
        }
        // Complete pre-state: head unchanged, no command row, nothing to
        // replay under this command_id.
        assert_eq!(store.transaction_head().unwrap(), (0, None));
        assert!(store.command_outcome(&command_id).unwrap().is_none());
        cleanup(&root);
    }

    #[test]
    fn fault_after_sql_commit_reconciles_on_retry_instead_of_double_committing() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let input = create_input("durably-committed-but-unacknowledged");
        let command_id = input.command_id.clone();

        let lost_ack_err = {
            let mut writer = store.writer().unwrap();
            writer
                .commit_with_fault(
                    input.clone(),
                    Some(CommitFaultPoint::AfterSqlCommitBeforeReturn),
                )
                .unwrap_err()
        };
        assert!(matches!(lost_ack_err, Error::Canonical(_)));

        // The transaction actually committed before the fault fired: the
        // head already advanced, even though the caller received an error.
        assert_eq!(store.transaction_head().unwrap().0, 1);

        // Reconciliation without resubmitting anything: the real outcome is
        // already readable.
        let reconciled = store.command_outcome(&command_id).unwrap().unwrap();
        assert_eq!(reconciled.command_id, command_id);
        assert!(reconciled.replay);

        // Retrying with the exact same input also reconciles to the same
        // result rather than executing a second commit or minting a new ID.
        let retried = {
            let mut writer = store.writer().unwrap();
            writer.commit(input).unwrap()
        };
        assert!(retried.replay);
        assert_eq!(retried.object_id, reconciled.object_id);
        assert_eq!(retried.revision_id, reconciled.revision_id);
        assert_eq!(
            store.transaction_head().unwrap().0,
            1,
            "no second commit occurred"
        );
        cleanup(&root);
    }

    #[test]
    fn only_one_writer_capability_can_exist_at_a_time() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let _writer = store.writer().unwrap();
        // A second `CanonicalStore` handle on the same root cannot obtain a
        // writer while the OS-held lease is held by the first.
        let mut second_handle = CanonicalStore::open(&root).unwrap();
        let err = second_handle.writer().unwrap_err();
        assert!(matches!(err, Error::WriterLocked { .. }));
        cleanup(&root);
    }

    #[test]
    fn reopened_store_independently_shows_the_committed_command_and_revision() {
        // Inspect a freshly reopened, independent handle — not the one that
        // performed the commit — matching this task's "inspect reopened DB
        // independently" verification method.
        let root = tmp();
        let outcome = {
            let mut store = CanonicalStore::create(&root).unwrap();
            let mut writer = store.writer().unwrap();
            writer
                .commit(create_input("payload-for-reopen-check"))
                .unwrap()
        };

        let reopened = CanonicalStore::open(&root).unwrap();
        let (rev, payload) = reopened.read_current(&outcome.object_id).unwrap().unwrap();
        assert_eq!(rev, outcome.revision_id);
        assert_eq!(payload, "payload-for-reopen-check");
        let stored = reopened
            .command_outcome(&outcome.command_id)
            .unwrap()
            .unwrap();
        assert_eq!(stored.resulting_head_hash, outcome.resulting_head_hash);
        assert_eq!(
            reopened.transaction_head().unwrap(),
            (1, Some(outcome.resulting_head_hash))
        );
        cleanup(&root);
    }
}
