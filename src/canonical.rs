//! Format-2 canonical transaction store (T01-02).
//!
//! Canonical build plan §13/§15/§16, task `T01-02`. This module creates and
//! (re)opens a **new, independently specified** SQLite-backed canonical
//! store, isolated from the format-1 markdown-tree [`crate::vault::Vault`].
//!
//! **Scope boundary — read before extending this module.** T01-02 is
//! "Create", not "Save". It publishes an empty, verified store with a single
//! identity/head-summary row. It deliberately does **not** implement:
//!
//! - the transaction/command admission API (`T01-03`);
//! - the OS-held single-writer lease integration for this store (the task's
//!   own security row cites S06 against `T01-01..03`, not `T01-02` alone;
//!   binding mutation to the writer lease is `T01-03`'s "bind private
//!   mutators to owning vault writer" clause);
//! - `Project`/`Note`/`Action`/`Decision`/`Transaction`/... tables from plan
//!   §15 — those are populated by later `P01`/`P02` tasks as the command API
//!   lands. Creating them now with no writer would be exactly the
//!   "successor work" this task's forbidden scope excludes.
//!
//! Building only the identity/head-summary table now, and refusing to open
//! any schema this module does not recognize, is what makes "incompatible or
//! unexpected schema refuses writes" (this task's own acceptance clause)
//! meaningful later: a future task that adds real tables does so through an
//! explicit, reviewable schema-version bump here, not by silently widening
//! what "recognized" means.
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

use crate::vault::{atomic_write_file, VaultMeta, CONTROL_DIR, VAULT_META_FILE};
use crate::{Error, Result};
use rusqlite::{Connection, OpenFlags};
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
/// in-place schema addition without moving the outer format epoch. `T01-02`
/// publishes schema version `1` (identity/head-summary table only).
pub const CANONICAL_SCHEMA_VERSION: i64 = 1;

/// Minimum reader capability required to open this schema version. Bumped
/// only on a breaking change; equals `CANONICAL_SCHEMA_VERSION` until one
/// occurs. A future writer that raises this above what a given build
/// implements must be refused by that build, not guessed at (§15 "current
/// schema/min-reader capabilities").
pub const CANONICAL_MIN_READER_CAPABILITY: i64 = 1;

/// Plan §13 tie-break: "choose the smallest configuration meeting §27
/// (tie: 4096 and 8 MiB)".
const PAGE_SIZE: i64 = 4096;
/// `PRAGMA cache_size`: a negative value is interpreted by SQLite as
/// kibibytes rather than pages. `-8192` = 8192 KiB = 8 MiB.
const CACHE_SIZE_KIB_NEGATIVE: i64 = -8192;

/// Exact recognized `canonical_vault` column `(name, declared_type)` pairs,
/// in `PRAGMA table_info` order. Any published or opened database whose
/// schema differs from this — an extra table, a renamed/retyped column — is
/// refused rather than silently admitted (S04, "incompatible or unexpected
/// schema refuses writes").
fn expected_columns() -> Vec<(String, String)> {
    [
        ("singleton", "INTEGER"),
        ("vault_id", "TEXT"),
        ("schema_version", "INTEGER"),
        ("min_reader_capability", "INTEGER"),
        ("created_by_version", "TEXT"),
        ("created_at", "TEXT"),
        ("transaction_head_seq", "INTEGER"),
        ("transaction_head_hash", "TEXT"),
    ]
    .into_iter()
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

    /// `(transaction_head_seq, transaction_head_hash)`. Always `(0, None)`
    /// until `T01-03` introduces the first committed transaction.
    pub fn transaction_head(&self) -> Result<(i64, Option<String>)> {
        self.conn
            .query_row(
                "SELECT transaction_head_seq, transaction_head_hash FROM canonical_vault WHERE singleton = 1",
                [],
                |r| Ok((r.get::<_, i64>(0)?, r.get::<_, Option<String>>(1)?)),
            )
            .map_err(|e| Error::Canonical(format!("cannot read transaction head: {e}")))
    }
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
        );",
    )
    .map_err(|e| Error::Canonical(format!("cannot create canonical_vault table: {e}")))?;

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
/// database whose schema is not exactly the single `canonical_vault` table
/// this task publishes, whose identity row does not match the guard, or
/// whose declared `min_reader_capability` exceeds what this build
/// implements (S04 "incompatible or unexpected schema refuses writes";
/// §15 "guard/DB identity agrees").
fn assert_schema_recognized(conn: &Connection, expected_vault_id: &str) -> Result<()> {
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
        .map_err(|e| Error::Canonical(format!("cannot inspect schema: {e}")))?;
    let names: Vec<String> = stmt
        .query_map([], |r| r.get::<_, String>(0))
        .map_err(|e| Error::Canonical(format!("cannot list tables: {e}")))?
        .collect::<rusqlite::Result<_>>()
        .map_err(|e| Error::Canonical(format!("cannot read table list: {e}")))?;
    if names != vec!["canonical_vault".to_string()] {
        return Err(Error::Canonical(format!(
            "unrecognized canonical schema: expected exactly [canonical_vault], found {names:?}"
        )));
    }

    let mut cols_stmt = conn
        .prepare("PRAGMA table_info(canonical_vault)")
        .map_err(|e| Error::Canonical(format!("cannot inspect canonical_vault columns: {e}")))?;
    let cols: Vec<(String, String)> = cols_stmt
        .query_map([], |r| Ok((r.get::<_, String>(1)?, r.get::<_, String>(2)?)))
        .map_err(|e| Error::Canonical(format!("cannot list columns: {e}")))?
        .collect::<rusqlite::Result<_>>()
        .map_err(|e| Error::Canonical(format!("cannot read column list: {e}")))?;
    let expected = expected_columns();
    if cols != expected {
        return Err(Error::Canonical(format!(
            "unrecognized canonical_vault schema: expected columns {expected:?}, found {cols:?}"
        )));
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
        let tables: Vec<String> = generic
            .prepare("SELECT name FROM sqlite_master WHERE type = 'table'")
            .unwrap()
            .query_map([], |r| r.get::<_, String>(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert_eq!(tables, vec!["canonical_vault".to_string()]);
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
}
