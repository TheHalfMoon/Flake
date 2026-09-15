//! Disposable, project-scoped full-text search index over the format-2
//! canonical store (`T02-04`).
//!
//! **I06 "derivation": disposable, never authoritative.** Every row in this
//! module's own database is a *candidate hint*, exactly like
//! `crate::derived`'s Phase T precedent for the format-1 object model.
//! [`search`] never trusts a cached `project_id`/`kind`/`title` from the
//! index as the reason to return or admit a hit — it re-reads each
//! candidate's **current** canonical state before including it, and drops
//! (never fabricates) a candidate that canonical state can no longer
//! confirm. An index that is missing, corrupt, or simply never built cannot
//! silently produce a false "no results" — [`search`] falls back to a
//! bounded direct canonical scan instead (see "Canonical fallback" below).
//!
//! **A new, independent module — not an extension of `crate::derived`.**
//! `derived.rs` is Phase T's own FTS5 index, coupled end-to-end to the
//! historical `ObjectId`/`ObjectRecord`/`Vault` format-1 model still used by
//! this crate's original `init`/`add`/`scan`/`rebuild`/`search` CLI
//! commands — untouched by this task. This module indexes the *format-2*
//! canonical store's typed records instead, in its own database file
//! (`derived-fts.sqlite`, deliberately not reusing the literal name
//! `derived.sqlite` the plan's §14 diagram uses generically — that exact
//! filename is already `derived.rs`'s own format-1 file, and reusing it
//! here for an unrelated schema in even a different directory would be
//! needlessly confusing to a reader), so neither index can corrupt or block
//! the other.
//!
//! **Indexable "work"**: exactly `Note`/`Action`/`Decision` — the same set
//! `project::list_project_records` already calls "work records" (`Source`
//! carries opaque bytes, not indexable text beyond a label; `Relation` and
//! `Project` carry no free-text body of their own). `Note.tombstoned`ing is
//! deliberately **not** a reason to drop a record from the index — this
//! module indexes exactly what `list_project_records` scopes, the same
//! already-established boundary, and tombstone-aware filtering (if wanted)
//! is UI-layer scope, not this task's.
//!
//! **Full rebuild: crash-safe by construction, not by a special case.** A
//! rebuild is written entirely into a fresh staging file
//! (`.fehrest.index-staging-<uuid7>.sqlite`) and only `fs::rename`d over the
//! published path once completely built — the exact "staging file/dir, then
//! one atomic rename, never partial publish" pattern `backup.rs`/
//! `recovery.rs` already use. An interruption at any point before the
//! rename leaves the previously-published index (if any) completely
//! untouched and fully queryable — F04 "interrupted rebuild retains
//! previous usable generation."
//!
//! **Incremental update: one SQLite transaction, checkpointed by
//! transaction sequence.** [`incremental_update`] reads
//! `CanonicalStore::revisions_since` the index's own last-recorded
//! `built_through_seq`, re-derives the FTS row for exactly the objects that
//! changed, and advances `built_through_seq` to the highest sequence number
//! it actually processed — all inside one SQLite transaction on the index
//! database, so an interruption mid-update rolls back to the previous
//! checkpoint (still fully valid; nothing published mid-way) rather than
//! leaving a partially-updated, uncheckpointed index. Parity between a
//! from-scratch rebuild and an equivalent sequence of incremental updates
//! is `T02-04`'s own named oracle, proven by
//! `full_rebuild_and_incremental_updates_produce_identical_search_results`.
//!
//! **Canonical fallback.** When the index cannot be opened at all (missing
//! file, corrupt schema), [`search`] does not return an empty result set
//! disguised as "nothing matches" — it performs a bounded, literal,
//! case-insensitive substring scan directly over
//! `CanonicalStore::list_current_objects`, labeled
//! [`SearchStatus::CanonicalFallback`] so a caller can show the user this
//! was not the accelerated path.

use crate::canonical::CanonicalStore;
use crate::project::RecordPayload;
use crate::{limits, Error, Result};
use rusqlite::{Connection, OpenFlags};
use std::path::{Path, PathBuf};

/// Deliberately distinct from `derived::Derived`'s own `derived.sqlite` —
/// see module docs.
const INDEX_DB_FILE: &str = "derived-fts.sqlite";

#[derive(Debug, Clone, PartialEq)]
pub struct IndexStatus {
    pub built_through_seq: i64,
    pub built_at: String,
    pub indexed_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchStatus {
    /// The index has been built/updated through the canonical store's
    /// current transaction head — no known lag.
    Fresh { built_through_seq: i64 },
    /// The index exists and was used, but canonical state has advanced
    /// past it. Hits are still returned (§12 "index lag is visible", not a
    /// blocking condition) — the caller decides whether to also offer an
    /// explicit rebuild/update.
    Stale {
        built_through_seq: i64,
        current_seq: i64,
    },
    /// The index could not be opened at all; results (if any) came from a
    /// bounded direct canonical scan instead.
    CanonicalFallback { reason: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hit {
    pub object_id: String,
    pub kind: &'static str,
    pub project_id: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchOutcome {
    pub status: SearchStatus,
    pub hits: Vec<Hit>,
}

fn index_db_path(control_dir: &Path) -> PathBuf {
    control_dir.join(INDEX_DB_FILE)
}

fn open_hardened(path: &Path, flags: OpenFlags) -> Result<Connection> {
    let conn = Connection::open_with_flags(path, flags)
        .map_err(|e| Error::Derived(format!("cannot open index db {}: {e}", path.display())))?;
    conn.pragma_update(None, "trusted_schema", false)
        .map_err(|e| Error::Derived(format!("cannot set trusted_schema: {e}")))?;
    conn.pragma_update(None, "foreign_keys", true)
        .map_err(|e| Error::Derived(format!("cannot set foreign_keys: {e}")))?;
    Ok(conn)
}

fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS index_meta (
           singleton         INTEGER PRIMARY KEY CHECK (singleton = 1),
           built_through_seq INTEGER NOT NULL,
           built_at          TEXT NOT NULL
         );
         CREATE TABLE IF NOT EXISTS record_index (
           object_id   TEXT PRIMARY KEY,
           project_id  TEXT NOT NULL,
           kind        TEXT NOT NULL,
           revision_id TEXT NOT NULL,
           fts_rowid   INTEGER NOT NULL
         );
         CREATE VIRTUAL TABLE IF NOT EXISTS record_fts USING fts5(
           title, body,
           tokenize = 'unicode61 remove_diacritics 2'
         );",
    )
    .map_err(|e| Error::Derived(format!("cannot init index schema: {e}")))
}

/// Extract `(title, body)` searchable text for an indexable record, or
/// `None` for a kind this module does not index (`Project`/`Source`/
/// `Relation`) — see module docs, "Indexable work".
fn searchable_text(record: &RecordPayload) -> Option<(&'static str, String, String)> {
    match record {
        RecordPayload::Note(n) => {
            Some(("note", n.title.clone().unwrap_or_default(), n.body.clone()))
        }
        RecordPayload::Action(a) => Some((
            "action",
            a.title.clone(),
            a.body.clone().unwrap_or_default(),
        )),
        RecordPayload::Decision(d) => Some((
            "decision",
            d.decision_key.clone(),
            format!(
                "{} {}",
                d.statement,
                d.rationale.clone().unwrap_or_default()
            ),
        )),
        RecordPayload::Project(_)
        | RecordPayload::Source(_)
        | RecordPayload::Relation(_)
        | RecordPayload::SourceCheck(_)
        | RecordPayload::ReviewCheckpoint(_)
        | RecordPayload::ExportGrant(_)
        | RecordPayload::DisclosureReceipt(_) => None,
    }
}

fn project_id_of_indexable(record: &RecordPayload) -> Option<&str> {
    match record {
        RecordPayload::Note(n) => Some(&n.project_id),
        RecordPayload::Action(a) => Some(&a.project_id),
        RecordPayload::Decision(d) => Some(&d.project_id),
        _ => None,
    }
}

/// Insert one record's current text into `conn`'s FTS/index tables,
/// replacing any prior row for the same `object_id` first (upsert, not a
/// duplicate insert — FTS5 has no natural primary key to conflict on, so
/// `record_index.fts_rowid` is the explicit link this module maintains
/// itself).
fn upsert_record(
    conn: &Connection,
    object_id: &str,
    revision_id: &str,
    record: &RecordPayload,
) -> Result<()> {
    let Some((kind, title, body)) = searchable_text(record) else {
        return Ok(());
    };
    let project_id =
        project_id_of_indexable(record).expect("indexable kinds always carry a project_id");

    if let Ok(old_rowid) = conn.query_row(
        "SELECT fts_rowid FROM record_index WHERE object_id = ?1",
        rusqlite::params![object_id],
        |r| r.get::<_, i64>(0),
    ) {
        conn.execute(
            "DELETE FROM record_fts WHERE rowid = ?1",
            rusqlite::params![old_rowid],
        )
        .map_err(|e| Error::Derived(format!("cannot delete stale fts row: {e}")))?;
    }
    conn.execute(
        "INSERT INTO record_fts (title, body) VALUES (?1, ?2)",
        rusqlite::params![title, body],
    )
    .map_err(|e| Error::Derived(format!("cannot insert fts row: {e}")))?;
    let new_rowid = conn.last_insert_rowid();
    conn.execute(
        "INSERT INTO record_index (object_id, project_id, kind, revision_id, fts_rowid)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(object_id) DO UPDATE SET
           project_id = excluded.project_id,
           kind = excluded.kind,
           revision_id = excluded.revision_id,
           fts_rowid = excluded.fts_rowid",
        rusqlite::params![object_id, project_id, kind, revision_id, new_rowid],
    )
    .map_err(|e| Error::Derived(format!("cannot upsert record_index row: {e}")))?;
    Ok(())
}

/// Full rebuild from canonical state. Snapshots `built_through_seq`
/// **before** scanning (conservative: any commit landing during the scan
/// makes the published checkpoint understate freshness, never overstate
/// it — the unsafe direction would be claiming a commit is indexed when it
/// was not).
pub fn rebuild_index(store: &CanonicalStore, control_dir: &Path) -> Result<IndexStatus> {
    let (before_seq, _) = store.transaction_head()?;
    let objects = store.list_current_objects()?;

    std::fs::create_dir_all(control_dir)
        .map_err(|e| Error::Derived(format!("cannot create control dir: {e}")))?;
    let staging = control_dir.join(format!(
        ".fehrest.index-staging-{}.sqlite",
        uuid::Uuid::now_v7()
    ));
    let cleanup = |p: &Path| {
        let _ = std::fs::remove_file(p);
    };

    let result = (|| -> Result<usize> {
        let conn = open_hardened(
            &staging,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        init_schema(&conn)?;
        let mut indexed = 0usize;
        for (object_id, revision_id, payload) in &objects {
            let record = match RecordPayload::from_json(payload) {
                Ok(r) => r,
                Err(_) => continue,
            };
            if searchable_text(&record).is_some() {
                upsert_record(&conn, object_id, revision_id, &record)?;
                indexed += 1;
            }
        }
        let built_at = crate::capture::now_rfc3339_utc();
        conn.execute(
            "INSERT INTO index_meta (singleton, built_through_seq, built_at) VALUES (1, ?1, ?2)
             ON CONFLICT(singleton) DO UPDATE SET built_through_seq = excluded.built_through_seq, built_at = excluded.built_at",
            rusqlite::params![before_seq, built_at],
        )
        .map_err(|e| Error::Derived(format!("cannot write index_meta: {e}")))?;
        drop(conn);
        Ok(indexed)
    })();

    let indexed_count = match result {
        Ok(n) => n,
        Err(e) => {
            cleanup(&staging);
            return Err(e);
        }
    };

    std::fs::rename(&staging, index_db_path(control_dir)).map_err(|e| {
        cleanup(&staging);
        Error::Derived(format!("cannot publish rebuilt index: {e}"))
    })?;

    Ok(IndexStatus {
        built_through_seq: before_seq,
        built_at: crate::capture::now_rfc3339_utc(),
        indexed_count,
    })
}

/// Incrementally update an already-published index. Refuses (rather than
/// silently rebuilding) if no index has ever been published — callers that
/// want "update, or build if absent" should try this first and fall back to
/// [`rebuild_index`] on that specific refusal (this is exactly what the
/// `fts-update` CLI command does).
pub fn incremental_update(store: &CanonicalStore, control_dir: &Path) -> Result<IndexStatus> {
    let path = index_db_path(control_dir);
    if !path.exists() {
        return Err(Error::Derived(
            "no published index exists to update; rebuild first".into(),
        ));
    }
    let mut conn = open_hardened(
        &path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;
    init_schema(&conn)?; // no-op if already present; tolerates a pre-`index_meta` legacy file defensively

    let (built_through_seq, built_at): (i64, String) = conn
        .query_row(
            "SELECT built_through_seq, built_at FROM index_meta WHERE singleton = 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|e| {
            Error::Derived(format!(
                "index is missing its checkpoint row (corrupt?): {e}"
            ))
        })?;

    let changes = store.revisions_since(built_through_seq)?;
    if changes.is_empty() {
        let indexed_count = conn
            .query_row("SELECT COUNT(*) FROM record_index", [], |r| {
                r.get::<_, i64>(0)
            })
            .map_err(|e| Error::Derived(format!("cannot count index rows: {e}")))?;
        return Ok(IndexStatus {
            built_through_seq,
            built_at,
            indexed_count: indexed_count as usize,
        });
    }

    let mut new_checkpoint = built_through_seq;
    {
        let tx = conn
            .transaction()
            .map_err(|e| Error::Derived(format!("cannot open index transaction: {e}")))?;
        for (seq, object_id, revision_id, payload) in &changes {
            let record = match RecordPayload::from_json(payload) {
                Ok(r) => r,
                Err(_) => {
                    new_checkpoint = *seq;
                    continue;
                }
            };
            if searchable_text(&record).is_some() {
                upsert_record(&tx, object_id, revision_id, &record)?;
            }
            new_checkpoint = *seq;
        }
        let built_at_now = crate::capture::now_rfc3339_utc();
        tx.execute(
            "UPDATE index_meta SET built_through_seq = ?1, built_at = ?2 WHERE singleton = 1",
            rusqlite::params![new_checkpoint, built_at_now],
        )
        .map_err(|e| Error::Derived(format!("cannot advance index checkpoint: {e}")))?;
        tx.commit()
            .map_err(|e| Error::Derived(format!("cannot commit incremental index update: {e}")))?;
    }

    let (built_through_seq, built_at): (i64, String) = conn
        .query_row(
            "SELECT built_through_seq, built_at FROM index_meta WHERE singleton = 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|e| Error::Derived(format!("cannot re-read index checkpoint: {e}")))?;
    let indexed_count = conn
        .query_row("SELECT COUNT(*) FROM record_index", [], |r| {
            r.get::<_, i64>(0)
        })
        .map_err(|e| Error::Derived(format!("cannot count index rows: {e}")))?;
    Ok(IndexStatus {
        built_through_seq,
        built_at,
        indexed_count: indexed_count as usize,
    })
}

/// [`incremental_update`], falling back to [`rebuild_index`] only when no
/// index has ever been published yet (the one refusal `incremental_update`
/// raises for a genuinely missing index) — never silently discards and
/// rebuilds over a *corrupt* one, which would hide a real problem the
/// caller should see.
pub fn ensure_index_current(store: &CanonicalStore, control_dir: &Path) -> Result<IndexStatus> {
    if !index_db_path(control_dir).exists() {
        return rebuild_index(store, control_dir);
    }
    incremental_update(store, control_dir)
}

/// Build an FTS5 `MATCH` expression that treats input as literal text — the
/// identical escaping technique `derived::literal_match_expression` already
/// uses for the format-1 index (each whitespace-separated token quoted,
/// internal quotes doubled), reimplemented here rather than shared because
/// the two modules are otherwise fully independent (see module docs).
fn literal_match_expression(query: &str) -> String {
    query
        .split_whitespace()
        .filter(|t| !t.is_empty())
        .map(|t| format!("\"{}\"", t.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(" ")
}

fn canonical_fallback_search(
    store: &CanonicalStore,
    project_filter: Option<&str>,
    query: &str,
    limit: usize,
    reason: String,
) -> Result<SearchOutcome> {
    let needles: Vec<String> = query
        .split_whitespace()
        .map(|t| t.to_lowercase())
        .filter(|t| !t.is_empty())
        .collect();
    let mut hits = Vec::new();
    if !needles.is_empty() {
        for (object_id, _revision_id, payload) in store.list_current_objects()? {
            if hits.len() >= limit {
                break;
            }
            let Ok(record) = RecordPayload::from_json(&payload) else {
                continue;
            };
            let Some((kind, title, body)) = searchable_text(&record) else {
                continue;
            };
            let project_id = project_id_of_indexable(&record).unwrap().to_string();
            if let Some(pf) = project_filter {
                if project_id != pf {
                    continue;
                }
            }
            let haystack = format!("{title} {body}").to_lowercase();
            if needles.iter().all(|n| haystack.contains(n.as_str())) {
                hits.push(Hit {
                    object_id,
                    kind,
                    project_id,
                    title: Some(title).filter(|t| !t.is_empty()),
                });
            }
        }
    }
    Ok(SearchOutcome {
        status: SearchStatus::CanonicalFallback { reason },
        hits,
    })
}

/// Search the index for `query` (optionally scoped to `project_filter`),
/// returning at most `limit` hits (bounded by
/// `crate::limits::MAX_SEARCH_RESULTS`). Every returned [`Hit`] reflects
/// **current** canonical state, re-read fresh for each raw index match — an
/// index row naming a project/kind that no longer matches canonical state
/// is dropped, never trusted (S02/S04, I06; see module docs).
pub fn search(
    store: &CanonicalStore,
    control_dir: &Path,
    project_filter: Option<&str>,
    query: &str,
    limit: usize,
) -> Result<SearchOutcome> {
    if query.len() > limits::MAX_QUERY_BYTES {
        return Err(Error::LimitExceeded {
            what: "search query",
            limit: limits::MAX_QUERY_BYTES,
            actual: query.len(),
        });
    }
    let limit = limit.min(limits::MAX_SEARCH_RESULTS);

    let path = index_db_path(control_dir);
    let conn = if !path.exists() {
        None
    } else {
        open_hardened(
            &path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .ok()
    };
    let Some(conn) = conn else {
        return canonical_fallback_search(
            store,
            project_filter,
            query,
            limit,
            "no index has been built yet, or the index file could not be opened".into(),
        );
    };

    let built_through_seq: Option<i64> = conn
        .query_row(
            "SELECT built_through_seq FROM index_meta WHERE singleton = 1",
            [],
            |r| r.get(0),
        )
        .ok();
    let Some(built_through_seq) = built_through_seq else {
        return canonical_fallback_search(
            store,
            project_filter,
            query,
            limit,
            "index database exists but has no checkpoint row (corrupt)".into(),
        );
    };

    let match_expr = literal_match_expression(query);
    let mut hits = Vec::new();
    if !match_expr.is_empty() {
        // Deliberately no `project_id` predicate here, even though
        // `record_index.project_id` is available — that column is a
        // *cached* hint, and letting it drive SQL-level inclusion/exclusion
        // would let a poisoned or stale index row either hide a record that
        // canonically does belong to the requested project, or (worse)
        // decide relevance for one that does not. Project scoping happens
        // exactly once, below, against freshly re-read canonical state.
        let mut stmt = conn
            .prepare(
                "SELECT ri.object_id FROM record_fts f
                 JOIN record_index ri ON ri.fts_rowid = f.rowid
                 WHERE record_fts MATCH ?1
                 ORDER BY rank, ri.object_id LIMIT ?2",
            )
            .map_err(|e| Error::Derived(format!("cannot prepare index search: {e}")))?;
        // A generous over-fetch bound: post-candidate eligibility can drop
        // rows (poisoned/stale entries, or non-matching-project rows when
        // `project_filter` is set), so the raw query asks for more than
        // `limit` before that filtering — substantially more when scoping
        // to one project out of a possibly multi-project index — capped
        // well below `MAX_SEARCH_RESULTS` scale.
        let overfetch = if project_filter.is_some() { 20 } else { 4 };
        let raw_limit = (limit.saturating_mul(overfetch))
            .max(limit)
            .min(limits::MAX_SEARCH_RESULTS * overfetch);
        let rows = stmt
            .query_map(rusqlite::params![match_expr, raw_limit as i64], |r| {
                r.get::<_, String>(0)
            })
            .map_err(|e| Error::Derived(format!("cannot run index search: {e}")))?;
        let candidate_ids: Vec<String> = rows
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|e| Error::Derived(format!("cannot read index search rows: {e}")))?;

        for object_id in candidate_ids {
            if hits.len() >= limit {
                break;
            }
            // Post-candidate eligibility: re-read fresh canonical state.
            // Never trust the index's own cached kind/project/title.
            let Some((_, payload)) = store.read_current(&object_id)? else {
                continue; // stale index entry for a since-vanished object
            };
            let Ok(record) = RecordPayload::from_json(&payload) else {
                continue;
            };
            let Some((kind, title, _)) = searchable_text(&record) else {
                continue; // no longer (or never really) an indexable kind
            };
            let project_id = project_id_of_indexable(&record).unwrap().to_string();
            if let Some(pf) = project_filter {
                if project_id != pf {
                    continue; // poisoned/stale project hint — drop, never trust
                }
            }
            hits.push(Hit {
                object_id,
                kind,
                project_id,
                title: Some(title).filter(|t| !t.is_empty()),
            });
        }
    }

    let (current_seq, _) = store.transaction_head()?;
    let status = if current_seq > built_through_seq {
        SearchStatus::Stale {
            built_through_seq,
            current_seq,
        }
    } else {
        SearchStatus::Fresh { built_through_seq }
    };
    Ok(SearchOutcome { status, hits })
}

/// Read the published index's own checkpoint, without searching —
/// `fts-status`'s CLI backing.
pub fn status(control_dir: &Path) -> Result<Option<IndexStatus>> {
    let path = index_db_path(control_dir);
    if !path.exists() {
        return Ok(None);
    }
    let conn = match open_hardened(
        &path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    ) {
        Ok(c) => c,
        Err(_) => return Ok(None),
    };
    let row: Option<(i64, String)> = conn
        .query_row(
            "SELECT built_through_seq, built_at FROM index_meta WHERE singleton = 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .ok();
    let Some((built_through_seq, built_at)) = row else {
        return Ok(None);
    };
    let indexed_count = conn
        .query_row("SELECT COUNT(*) FROM record_index", [], |r| {
            r.get::<_, i64>(0)
        })
        .unwrap_or(0);
    Ok(Some(IndexStatus {
        built_through_seq,
        built_at,
        indexed_count: indexed_count as usize,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::CanonicalStore;
    use crate::project;
    use std::path::PathBuf;

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-index-{}", uuid::Uuid::now_v7()))
    }
    fn cleanup(p: &Path) {
        let _ = std::fs::remove_dir_all(p);
    }

    fn new_project(store: &mut CanonicalStore) -> String {
        let mut writer = store.writer().unwrap();
        project::create_project(&mut writer, "owner", "P", None)
            .unwrap()
            .0
            .object_id
    }

    #[test]
    fn rebuild_indexes_notes_actions_and_decisions_but_not_sources_or_relations() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(
                &mut writer,
                "owner",
                &project_id,
                Some("Alpha"),
                "alpha body",
            )
            .unwrap();
            project::create_action(&mut writer, "owner", &project_id, "Beta action", None, &[])
                .unwrap();
            project::create_decision(
                &mut writer,
                "owner",
                &project_id,
                "gamma-key",
                "Gamma statement",
                None,
                project::DecisionBasis::UserJudgment,
                project::DecisionVerification::Unreviewed,
                None,
                None,
            )
            .unwrap();
            crate::capture::create_manual_reference(
                &mut writer,
                "owner",
                &project_id,
                "delta source",
                None,
                None,
                None,
            )
            .unwrap();
        }

        let status = rebuild_index(&store, &store.control_dir()).unwrap();
        assert_eq!(status.indexed_count, 3); // note, action, decision — not the source

        for (term, expect_kind) in [("alpha", "note"), ("beta", "action"), ("gamma", "decision")] {
            let outcome = search(&store, &store.control_dir(), None, term, 10).unwrap();
            assert_eq!(outcome.hits.len(), 1, "expected one hit for {term}");
            assert_eq!(outcome.hits[0].kind, expect_kind);
        }
        // The source's own label never entered the index.
        let outcome = search(&store, &store.control_dir(), None, "delta", 10).unwrap();
        assert!(outcome.hits.is_empty());

        cleanup(&root);
    }

    #[test]
    fn full_rebuild_and_incremental_updates_produce_identical_search_results() {
        // One canonical store, two independently-located index generations
        // built two different ways, both reading the identical underlying
        // object IDs — so their hit sets are directly comparable (UUIDv7
        // object IDs are never expected to match across separate stores,
        // so this deliberately uses one store rather than two).
        let root = tmp();
        let index_a = root.join("index-a"); // built by incremental updates, one per mutation
        let index_b = root.join("index-b"); // built by a single full rebuild after all mutations
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);

        for i in 0..5 {
            let title = format!("Task number {i}");
            {
                let mut writer = store.writer().unwrap();
                project::create_action(&mut writer, "owner", &project_id, &title, None, &[])
                    .unwrap();
            }
            // Incrementally maintained from the start (falls back to a full
            // rebuild only on its very first call, when index_a does not
            // exist yet).
            ensure_index_current(&store, &index_a).unwrap();
        }
        rebuild_index(&store, &index_b).unwrap();

        let hits_a = search(&store, &index_a, None, "Task", 20).unwrap();
        let hits_b = search(&store, &index_b, None, "Task", 20).unwrap();
        let mut ids_a: Vec<String> = hits_a.hits.iter().map(|h| h.object_id.clone()).collect();
        let mut ids_b: Vec<String> = hits_b.hits.iter().map(|h| h.object_id.clone()).collect();
        ids_a.sort();
        ids_b.sort();
        assert_eq!(
            ids_a, ids_b,
            "full rebuild and incremental updates must converge to the same result set"
        );
        assert_eq!(ids_a.len(), 5);

        cleanup(&root);
    }

    #[test]
    fn stale_index_is_labeled_but_still_returns_results() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, None, "findable text").unwrap();
        }
        rebuild_index(&store, &store.control_dir()).unwrap();

        // A canonical mutation happens after the index was built, without
        // an incremental update — the index is now behind.
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, None, "unrelated text")
                .unwrap();
        }

        let outcome = search(&store, &store.control_dir(), None, "findable", 10).unwrap();
        assert!(matches!(outcome.status, SearchStatus::Stale { .. }));
        assert_eq!(
            outcome.hits.len(),
            1,
            "a stale index still returns its own known-good hits"
        );

        cleanup(&root);
    }

    #[test]
    fn missing_index_falls_back_to_a_bounded_canonical_scan_not_a_false_empty_result() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, None, "never indexed yet")
                .unwrap();
        }
        // No rebuild_index call at all.
        let outcome = search(&store, &store.control_dir(), None, "never", 10).unwrap();
        assert!(matches!(
            outcome.status,
            SearchStatus::CanonicalFallback { .. }
        ));
        assert_eq!(
            outcome.hits.len(),
            1,
            "fallback must actually find the real record, not just report a status"
        );

        cleanup(&root);
    }

    #[test]
    fn corrupt_index_database_falls_back_rather_than_erroring_the_whole_search() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(
                &mut writer,
                "owner",
                &project_id,
                None,
                "findable despite corruption",
            )
            .unwrap();
        }
        std::fs::create_dir_all(store.control_dir()).unwrap();
        std::fs::write(index_db_path(&store.control_dir()), b"not a sqlite file").unwrap();

        let outcome = search(&store, &store.control_dir(), None, "findable", 10).unwrap();
        assert!(matches!(
            outcome.status,
            SearchStatus::CanonicalFallback { .. }
        ));
        assert_eq!(outcome.hits.len(), 1);

        cleanup(&root);
    }

    #[test]
    fn a_poisoned_project_hint_in_the_index_cannot_leak_into_another_projects_scoped_search() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let p1 = new_project(&mut store);
        let p2 = new_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &p1, None, "p1 secret content").unwrap();
        }
        rebuild_index(&store, &store.control_dir()).unwrap();

        // Poison the index directly: claim the p1 note actually belongs to p2.
        let conn = open_hardened(
            &index_db_path(&store.control_dir()),
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .unwrap();
        conn.execute(
            "UPDATE record_index SET project_id = ?1",
            rusqlite::params![p2],
        )
        .unwrap();
        drop(conn);

        // A search scoped to p2 must NOT return the p1 record just because
        // the index says so — canonical re-verification drops it.
        let outcome = search(&store, &store.control_dir(), Some(&p2), "secret", 10).unwrap();
        assert!(
            outcome.hits.is_empty(),
            "a poisoned index project hint must not leak a record into a scoped search it does not canonically belong to"
        );

        // The record is still findable, correctly, scoped to its real project.
        let outcome = search(&store, &store.control_dir(), Some(&p1), "secret", 10).unwrap();
        assert_eq!(outcome.hits.len(), 1);

        cleanup(&root);
    }

    #[test]
    fn project_scoped_search_excludes_other_projects_results() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let p1 = new_project(&mut store);
        let p2 = new_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &p1, None, "shared term one").unwrap();
            project::create_note(&mut writer, "owner", &p2, None, "shared term two").unwrap();
        }
        rebuild_index(&store, &store.control_dir()).unwrap();

        let outcome = search(&store, &store.control_dir(), Some(&p1), "shared", 10).unwrap();
        assert_eq!(outcome.hits.len(), 1);
        assert_eq!(outcome.hits[0].project_id, p1);

        cleanup(&root);
    }

    #[test]
    fn literal_query_cannot_activate_fts_syntax() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, None, "alpha beta").unwrap();
        }
        rebuild_index(&store, &store.control_dir()).unwrap();

        // As literal text, "alpha OR beta" must not broaden the match to an
        // OR of the two terms — nothing contains the literal token "OR".
        let outcome = search(&store, &store.control_dir(), None, "alpha OR beta", 10).unwrap();
        assert!(outcome.hits.is_empty());
        let outcome = search(&store, &store.control_dir(), None, "alpha", 10).unwrap();
        assert_eq!(outcome.hits.len(), 1);

        cleanup(&root);
    }

    #[test]
    fn query_and_result_bounds_are_enforced() {
        let root = tmp();
        let store = CanonicalStore::create(&root).unwrap();
        let big = "x".repeat(limits::MAX_QUERY_BYTES + 1);
        assert!(matches!(
            search(&store, &store.control_dir(), None, &big, 10),
            Err(Error::LimitExceeded { .. })
        ));
        cleanup(&root);
    }

    #[test]
    fn interrupted_rebuild_leaves_the_previous_generation_fully_usable() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(
                &mut writer,
                "owner",
                &project_id,
                None,
                "original generation",
            )
            .unwrap();
        }
        let first = rebuild_index(&store, &store.control_dir()).unwrap();

        // Simulate an interrupted rebuild: a leftover, incomplete staging
        // file exists (as if the process died mid-build), but the
        // published index itself was never touched by it.
        let stray_staging = store.control_dir().join(format!(
            ".fehrest.index-staging-{}.sqlite",
            uuid::Uuid::now_v7()
        ));
        std::fs::write(&stray_staging, b"incomplete").unwrap();

        let outcome = search(&store, &store.control_dir(), None, "original", 10).unwrap();
        assert_eq!(
            outcome.hits.len(),
            1,
            "the previously published generation remains fully queryable"
        );
        assert!(
            matches!(outcome.status, SearchStatus::Fresh { built_through_seq } if built_through_seq == first.built_through_seq)
        );

        let _ = std::fs::remove_file(&stray_staging);
        cleanup(&root);
    }
}
