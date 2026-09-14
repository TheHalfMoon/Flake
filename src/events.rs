//! Append-only hash-chained canonical event log.
//!
//! **F-CORE-12 — what this provides, stated precisely.**
//!
//! | Detected | Not detected |
//! |---|---|
//! | single-record edit, truncation, reordering, splice, deletion, inconsistent restore | a **complete, internally consistent rewrite** of the whole chain |
//!
//! The chain is **unkeyed**. Under the declared root of trust (C §3.1) an attacker
//! holding the OS account can recompute every dependent hash and the result
//! verifies. This is partial-tamper evidence, **never authentication**. No MAC is
//! used, because key custody would be the same account being defended against.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// The six event types Phase T needs. Not the full architecture vocabulary —
/// event tiering stays unfrozen pending B-0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    VaultCreated,
    ObjectRegistered,
    ObjectConflict,
    MemoryRecorded,
    MemorySuperseded,
    ContextCompiled,
}

fn default_schema_version() -> u32 {
    1
}

pub const SCHEMA_VERSION_V1: u32 = 1;
pub const SCHEMA_VERSION_V2: u32 = 2;
pub const CURRENT_SCHEMA_VERSION: u32 = SCHEMA_VERSION_V2;

/// Typed payload for v2 events (FR2-012). Only Phase 1 needed variants; not the full future vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventPayload {
    VaultCreated {
        vault_id: String,
    },
    ObjectRegistered {
        object_id: String,
        path: String,
    },
    ObjectConflict {
        object_id: String,
        paths: Vec<String>,
    },
    MemoryRecorded {
        memory_id: String,
        memory_type: String,
    },
    MemorySuperseded {
        superseded_id: String,
        by_id: String,
    },
    ContextCompiled {
        context_id: String,
        digest: String,
        omitted: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    pub seq: u64,
    pub kind: EventKind,
    pub subject: String,
    pub detail: String,
    #[serde(default)]
    pub payload: Option<EventPayload>,
    pub prev_hash: String,
    pub hash: String,
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

const GENESIS: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn compute_hash(seq: u64, kind: EventKind, subject: &str, detail: &str, prev: &str) -> String {
    // Canonical serialization for hashing: field order is fixed here, not inherited
    // from a serializer whose output could change between versions.
    // V1 rule: no payload.
    let payload = format!("{seq}|{kind:?}|{subject}|{detail}|{prev}");
    hash_bytes(payload.as_bytes())
}

fn compute_hash_v2(
    seq: u64,
    kind: EventKind,
    subject: &str,
    detail: &str,
    payload: &Option<EventPayload>,
    prev: &str,
) -> String {
    // Frozen per-version order (T062): seq|kind|subject|detail|payload_json|prev
    // payload_json is serde_json deterministic for this enum (tag+content).
    let payload_json = match payload {
        Some(p) => serde_json::to_string(p).unwrap_or_else(|_| "null".into()),
        None => "null".into(),
    };
    let s = format!("{seq}|{kind:?}|{subject}|{detail}|{payload_json}|{prev}");
    hash_bytes(s.as_bytes())
}

fn compute_hash_for_event(ev: &Event) -> String {
    if ev.schema_version >= 2 {
        compute_hash_v2(
            ev.seq,
            ev.kind,
            &ev.subject,
            &ev.detail,
            &ev.payload,
            &ev.prev_hash,
        )
    } else {
        compute_hash(ev.seq, ev.kind, &ev.subject, &ev.detail, &ev.prev_hash)
    }
}

fn payload_for_kind(kind: EventKind, subject: &str, detail: &str) -> Option<EventPayload> {
    match kind {
        EventKind::VaultCreated => Some(EventPayload::VaultCreated {
            vault_id: subject.to_string(),
        }),
        EventKind::ObjectRegistered => Some(EventPayload::ObjectRegistered {
            object_id: subject.to_string(),
            path: detail.to_string(),
        }),
        EventKind::ObjectConflict => Some(EventPayload::ObjectConflict {
            object_id: subject.to_string(),
            paths: if detail.is_empty() {
                vec![]
            } else {
                vec![detail.to_string()]
            },
        }),
        EventKind::MemoryRecorded => Some(EventPayload::MemoryRecorded {
            memory_id: subject.to_string(),
            memory_type: if detail.is_empty() {
                "Fact".into()
            } else {
                detail.to_string()
            },
        }),
        EventKind::MemorySuperseded => Some(EventPayload::MemorySuperseded {
            superseded_id: subject.to_string(),
            by_id: detail.to_string(),
        }),
        EventKind::ContextCompiled => Some(EventPayload::ContextCompiled {
            context_id: subject.to_string(),
            digest: detail
                .split_whitespace()
                .next()
                .unwrap_or(detail)
                .to_string(),
            omitted: 0,
        }),
    }
}

/// The append-only event log.
#[derive(Debug)]
pub struct EventLog {
    path: PathBuf,
}

/// Result of verifying the chain.
#[derive(Debug, PartialEq, Eq)]
pub enum ChainStatus {
    Intact {
        events: usize,
    },
    /// A record's `prev_hash` does not match its predecessor's `hash`, or a
    /// record's own hash does not recompute. Reported with the exact sequence.
    Broken {
        at_seq: u64,
        reason: String,
    },
    /// `seq` is contiguous by construction; a gap means removal or a partial
    /// restore. N §3.3: not normal crash damage, and never silently continued.
    Gap {
        from_seq: u64,
        to_seq: u64,
    },
}

impl EventLog {
    pub fn open(control_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(control_dir)
            .map_err(|e| Error::Event(format!("cannot create control dir: {e}")))?;
        Ok(EventLog {
            path: control_dir.join("events.jsonl"),
        })
    }

    /// Type-proven append that requires a `VaultWriter` token.
    ///
    /// Per FR2-008 the API SHOULD structurally require writer ownership.
    /// The writer's vault control_dir must match this log's directory;
    /// this prevents a writer for vault A from appending to vault B's log
    /// via a forged path.
    pub fn append_for_writer(
        &self,
        writer: &crate::vault::VaultWriter<'_>,
        kind: EventKind,
        subject: &str,
        detail: &str,
    ) -> Result<Event> {
        // Verify control_dir identity — prevents cross-vault forgery
        let writer_control = writer.vault().control_dir();
        let self_parent = self.path.parent().unwrap_or(self.path.as_path());
        // Canonicalize both where possible, else string compare; we want strict equality.
        // Use string compare for portability (no extra syscall on read-only path that may not exist yet)
        if writer_control != self_parent {
            return Err(Error::Event(format!(
                "writer vault mismatch: writer holds {}, log at {}",
                writer_control.display(),
                self.path.display()
            )));
        }
        self.append(kind, subject, detail)
    }

    /// Unchecked append — no writer-ownership proof required by this
    /// function itself.
    ///
    /// **T01-01: this is deliberately `pub(crate)`, not `pub`.** It is the
    /// mechanism `append_for_writer` delegates to after verifying a bound
    /// `VaultWriter`, and it remains directly reachable from this crate's own
    /// tests (which legitimately need to fabricate specific chain states
    /// without a full `Vault`/writer scaffold) and from `verify`/recovery
    /// code paths that are themselves already gated elsewhere. It must
    /// **never** be reachable from `cli.rs` or any future product surface —
    /// two call sites in `cli.rs` did exactly that before this task (`init`,
    /// `compile`); both now go through `append_for_writer` instead. A new
    /// caller outside this file should be treated as a regression of the
    /// bypass this task closes, not a reason to widen this back to `pub`.
    pub(crate) fn append(&self, kind: EventKind, subject: &str, detail: &str) -> Result<Event> {
        if detail.len() > crate::limits::MAX_EVENT_BYTES {
            return Err(Error::LimitExceeded {
                what: "event detail",
                limit: crate::limits::MAX_EVENT_BYTES,
                actual: detail.len(),
            });
        }
        let events = self.read_all()?;
        let (seq, prev) = match events.last() {
            Some(e) => (e.seq + 1, e.hash.clone()),
            None => (1, GENESIS.to_string()),
        };
        let payload = payload_for_kind(kind, subject, detail);
        let schema_version = CURRENT_SCHEMA_VERSION;
        // Compute hash per frozen per-version rule (T062)
        let hash = compute_hash_v2(seq, kind, subject, detail, &payload, &prev);
        let ev = Event {
            schema_version,
            seq,
            kind,
            subject: subject.to_string(),
            detail: detail.to_string(),
            payload,
            prev_hash: prev,
            hash,
        };
        let line = serde_json::to_string(&ev)
            .map_err(|e| Error::Event(format!("cannot serialize event: {e}")))?;
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| Error::Event(format!("cannot open event log: {e}")))?;
        writeln!(f, "{line}").map_err(|e| Error::Event(format!("cannot append event: {e}")))?;
        // Durability boundary (T063): flush + sync_all on file (best-effort); parent dir sync not needed for append
        let _ = f.flush();
        let _ = f.sync_all();
        Ok(ev)
    }

    pub fn read_all(&self) -> Result<Vec<Event>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let f = std::fs::File::open(&self.path)
            .map_err(|e| Error::Event(format!("cannot open event log: {e}")))?;
        let mut out = Vec::new();
        for (i, line) in BufReader::new(f).lines().enumerate() {
            let line = line.map_err(|e| Error::Event(format!("cannot read line {i}: {e}")))?;
            if line.trim().is_empty() {
                continue;
            }
            let ev: Event = serde_json::from_str(&line)
                .map_err(|e| Error::Event(format!("malformed event at line {i}: {e}")))?;
            out.push(ev);
        }
        Ok(out)
    }

    /// Verify chain integrity. Detects what an unkeyed chain can detect, and the
    /// return type deliberately says nothing about authenticity.
    /// Supports both v1 (detail-only) and v2 (typed payload) via frozen per-version hash.
    pub fn verify(&self) -> Result<ChainStatus> {
        let events = self.read_all()?;
        let mut prev_hash = GENESIS.to_string();

        for (expected_seq, ev) in (1u64..).zip(events.iter()) {
            if ev.seq != expected_seq {
                return Ok(ChainStatus::Gap {
                    from_seq: expected_seq,
                    to_seq: ev.seq,
                });
            }
            if ev.prev_hash != prev_hash {
                return Ok(ChainStatus::Broken {
                    at_seq: ev.seq,
                    reason: "prev_hash does not match predecessor".into(),
                });
            }
            let recomputed = compute_hash_for_event(ev);
            if recomputed != ev.hash {
                return Ok(ChainStatus::Broken {
                    at_seq: ev.seq,
                    reason: "record hash does not recompute".into(),
                });
            }
            prev_hash = ev.hash.clone();
        }
        Ok(ChainStatus::Intact {
            events: events.len(),
        })
    }

    /// Expose the path for startup integrity checks (T066).
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Detect torn final record (T067).
    ///
    /// Returns `Some(torn_bytes)` if the last non-empty line fails JSON parse.
    /// Returns `None` if log is empty or last line parses as Event.
    /// If a middle line fails, that is **not** torn tail — it is gap/broken and
    /// will be reported via `verify()` failing closed (T069/T070).
    pub fn detect_torn_tail(&self) -> Result<Option<String>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let raw = std::fs::read_to_string(&self.path)
            .map_err(|e| Error::Event(format!("cannot read event log for torn detection: {e}")))?;
        if raw.trim().is_empty() {
            return Ok(None);
        }
        // Collect non-empty lines with their order
        let lines: Vec<&str> = raw.lines().filter(|l| !l.trim().is_empty()).collect();
        if lines.is_empty() {
            return Ok(None);
        }
        // Try parse each; find first failure index
        for (idx, line) in lines.iter().enumerate() {
            let parse: std::result::Result<Event, _> = serde_json::from_str(line);
            if parse.is_err() {
                let is_last = idx == lines.len() - 1;
                if is_last {
                    return Ok(Some((*line).to_string()));
                } else {
                    // Middle malformed — not torn tail, will be treated as gap/broken elsewhere
                    return Ok(None);
                }
            }
        }
        Ok(None)
    }

    /// Quarantine torn bytes and truncate log to last valid record (T068).
    ///
    /// Preservation before repair (FR2-021). Quarantine file is
    /// `<path>.torn.<seq_expected>.<uuid>.quarantine` containing exact torn bytes.
    /// Returns `Some(quarantine_path)` if repair was performed.
    pub fn quarantine_and_repair_torn_tail(&self) -> Result<Option<PathBuf>> {
        let torn = match self.detect_torn_tail()? {
            Some(t) => t,
            None => return Ok(None),
        };
        // Determine expected seq for quarantine naming: read_all of good prefix tells us count
        let mut good_count = 0usize;
        let raw = std::fs::read_to_string(&self.path)
            .map_err(|e| Error::Event(format!("cannot read for quarantine: {e}")))?;
        let lines: Vec<&str> = raw.lines().filter(|l| !l.trim().is_empty()).collect();
        for line in &lines {
            if serde_json::from_str::<Event>(line).is_ok() {
                good_count += 1;
            } else {
                break; // torn is last, so break here
            }
        }
        let seq_expected = good_count as u64 + 1;
        let qname = format!(
            "{}.torn.{}.{:?}.quarantine",
            self.path.file_name().unwrap().to_string_lossy(),
            seq_expected,
            uuid::Uuid::now_v7()
        );
        let qpath = self.path.parent().unwrap().join(qname);
        std::fs::write(&qpath, torn.as_bytes())
            .map_err(|e| Error::Event(format!("cannot write quarantine: {e}")))?;
        // Best-effort sync quarantine file
        let _ = std::fs::File::open(&qpath).and_then(|f| f.sync_all());

        // Truncate original log to last good content
        let good_lines = &lines[..good_count];
        let new_content = if good_lines.is_empty() {
            String::new()
        } else {
            good_lines.join("\n") + "\n"
        };
        // Atomic-like truncate: write to temp then rename? Simpler: truncate file in place and sync.
        // We do write + sync to preserve forensic guarantee (original truncated after quarantine).
        std::fs::write(&self.path, new_content.as_bytes())
            .map_err(|e| Error::Event(format!("cannot truncate torn tail: {e}")))?;
        if let Ok(f) = std::fs::File::open(&self.path) {
            let _ = f.sync_all();
        }
        if let Some(parent) = self.path.parent() {
            if let Ok(df) = std::fs::File::open(parent) {
                let _ = df.sync_all();
            }
        }
        Ok(Some(qpath))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp() -> PathBuf {
        let d = std::env::temp_dir().join(format!("fehrest-ev-{}", uuid::Uuid::now_v7()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn chain_is_intact_after_appends() {
        let d = tmp();
        let log = EventLog::open(&d).unwrap();
        log.append(EventKind::VaultCreated, "vault", "").unwrap();
        log.append(EventKind::ObjectRegistered, "obj-1", "a.md")
            .unwrap();
        log.append(EventKind::MemoryRecorded, "mem-1", "fact")
            .unwrap();
        assert_eq!(log.verify().unwrap(), ChainStatus::Intact { events: 3 });
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn single_record_edit_is_detected() {
        let d = tmp();
        let log = EventLog::open(&d).unwrap();
        log.append(EventKind::VaultCreated, "vault", "").unwrap();
        log.append(EventKind::ObjectRegistered, "obj-1", "a.md")
            .unwrap();
        log.append(EventKind::ObjectRegistered, "obj-2", "b.md")
            .unwrap();

        // Tamper with the middle record's detail, leaving hashes untouched.
        let p = d.join("events.jsonl");
        let text = std::fs::read_to_string(&p).unwrap();
        let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
        lines[1] = lines[1].replace("a.md", "evil.md");
        std::fs::write(&p, lines.join("\n") + "\n").unwrap();

        match log.verify().unwrap() {
            ChainStatus::Broken { at_seq, .. } => assert_eq!(at_seq, 2),
            s => panic!("edit must be detected, got {s:?}"),
        }
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn truncation_and_removal_are_detected() {
        let d = tmp();
        let log = EventLog::open(&d).unwrap();
        for i in 0..4 {
            log.append(EventKind::ObjectRegistered, &format!("o{i}"), "x")
                .unwrap();
        }
        let p = d.join("events.jsonl");
        let text = std::fs::read_to_string(&p).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        // Remove the second record: seq jumps 1 -> 3.
        let kept = format!("{}\n{}\n{}\n", lines[0], lines[2], lines[3]);
        std::fs::write(&p, kept).unwrap();

        match log.verify().unwrap() {
            ChainStatus::Gap { from_seq, to_seq } => {
                assert_eq!((from_seq, to_seq), (2, 3));
            }
            s => panic!("removal must be detected, got {s:?}"),
        }
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn consistent_full_rewrite_is_not_detected_and_we_say_so() {
        // This test exists to make the LIMIT executable rather than merely written
        // down. An unkeyed chain cannot detect a complete consistent rewrite; C 6.1
        // states that plainly, and this asserts the stated behaviour rather than
        // pretending otherwise.
        let d = tmp();
        let log = EventLog::open(&d).unwrap();
        log.append(EventKind::VaultCreated, "vault", "").unwrap();
        log.append(EventKind::ObjectRegistered, "real", "a.md")
            .unwrap();

        // Attacker rewrites the entire history consistently.
        let p = d.join("events.jsonl");
        std::fs::remove_file(&p).unwrap();
        let log2 = EventLog::open(&d).unwrap();
        log2.append(EventKind::VaultCreated, "vault", "").unwrap();
        log2.append(EventKind::ObjectRegistered, "forged", "evil.md")
            .unwrap();

        assert_eq!(log2.verify().unwrap(), ChainStatus::Intact { events: 2 });
        // Verification passes. That is the documented limit, not a defect.
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn oversized_event_detail_is_rejected() {
        let d = tmp();
        let log = EventLog::open(&d).unwrap();
        let big = "x".repeat(crate::limits::MAX_EVENT_BYTES + 1);
        assert!(matches!(
            log.append(EventKind::MemoryRecorded, "m", &big),
            Err(Error::LimitExceeded { .. })
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    // — Slice D T060–T065: versioned event journal —

    #[test]
    fn historical_v1_golden_fixture_upcasts_without_rewrite() {
        let d = tmp();
        let fixture = std::path::Path::new("tests/fixtures/events/history_v1.jsonl");
        let bytes_before = std::fs::read(fixture).unwrap();
        let dest = d.join("events.jsonl");
        std::fs::write(&dest, &bytes_before).unwrap();
        let log = EventLog::open(&d).unwrap();
        let events = log.read_all().unwrap();
        assert_eq!(events.len(), 6);
        for ev in &events {
            assert_eq!(
                ev.schema_version, 1,
                "v1 fixture must read as schema 1 without rewrite"
            );
            assert!(
                ev.payload.is_none(),
                "v1 payload must be None, upcast is in-memory"
            );
        }
        // Verify via frozen v1 hash must be intact
        assert_eq!(log.verify().unwrap(), ChainStatus::Intact { events: 6 });
        // File bytes unchanged after read (no rewrite)
        let bytes_after = std::fs::read(&dest).unwrap();
        assert_eq!(
            bytes_before, bytes_after,
            "historical bytes must not be rewritten to read them"
        );
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn current_v2_fixture_is_typed_and_verifies() {
        let d = tmp();
        let fixture = std::path::Path::new("tests/fixtures/events/current_v2.jsonl");
        let bytes = std::fs::read(fixture).unwrap();
        std::fs::write(d.join("events.jsonl"), &bytes).unwrap();
        let log = EventLog::open(&d).unwrap();
        let events = log.read_all().unwrap();
        assert_eq!(events.len(), 6);
        for ev in &events {
            assert_eq!(ev.schema_version, 2);
            assert!(ev.payload.is_some(), "v2 must carry typed payload");
        }
        assert_eq!(log.verify().unwrap(), ChainStatus::Intact { events: 6 });
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn typed_payload_participates_in_hash() {
        let d = tmp();
        let log = EventLog::open(&d).unwrap();
        let ev = log
            .append(EventKind::ObjectRegistered, "obj-123", "a.md")
            .unwrap();
        assert_eq!(ev.schema_version, CURRENT_SCHEMA_VERSION);
        assert!(ev.payload.is_some());
        // Tamper payload JSON without updating hash must break verification
        let p = d.join("events.jsonl");
        let text = std::fs::read_to_string(&p).unwrap();
        // Change payload path
        let tampered = text.replace("\"path\":\"a.md\"", "\"path\":\"evil.md\"");
        assert_ne!(text, tampered);
        std::fs::write(&p, tampered).unwrap();
        match log.verify().unwrap() {
            ChainStatus::Broken { at_seq, .. } => assert_eq!(at_seq, 1),
            s => panic!("payload tamper must be detected, got {s:?}"),
        }
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn append_after_historical_v1_preserves_chain_mixed_versions() {
        let d = tmp();
        let fixture = std::path::Path::new("tests/fixtures/events/history_v1.jsonl");
        let bytes = std::fs::read(fixture).unwrap();
        std::fs::write(d.join("events.jsonl"), &bytes).unwrap();
        let log = EventLog::open(&d).unwrap();
        assert_eq!(log.verify().unwrap(), ChainStatus::Intact { events: 6 });
        // Append new v2 event after v1 history
        let ev = log
            .append(EventKind::ObjectRegistered, "new-obj", "new.md")
            .unwrap();
        assert_eq!(ev.seq, 7);
        assert_eq!(ev.schema_version, 2);
        assert_eq!(log.verify().unwrap(), ChainStatus::Intact { events: 7 });
        // File still contains original v1 lines verbatim plus new v2 line
        let raw = std::fs::read_to_string(d.join("events.jsonl")).unwrap();
        let lines: Vec<&str> = raw.lines().collect();
        assert_eq!(lines.len(), 7);
        assert!(
            !lines[0].contains("schema_version"),
            "v1 line must remain without schema_version field, not rewritten"
        );
        assert!(
            lines[6].contains("\"schema_version\":2"),
            "new line must be v2"
        );
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn hash_freeze_is_versioned() {
        // Same logical event with different schema versions must hash differently (payload included)
        let h1 = compute_hash(1, EventKind::VaultCreated, "s", "d", GENESIS);
        let h2 = compute_hash_v2(
            1,
            EventKind::VaultCreated,
            "s",
            "d",
            &Some(EventPayload::VaultCreated {
                vault_id: "s".into(),
            }),
            GENESIS,
        );
        assert_ne!(
            h1, h2,
            "v1 vs v2 hash must differ because payload participates"
        );
        // But v1 event read as v1 must recompute via v1 rule
        let ev1 = Event {
            schema_version: 1,
            seq: 1,
            kind: EventKind::VaultCreated,
            subject: "s".into(),
            detail: "d".into(),
            payload: None,
            prev_hash: GENESIS.into(),
            hash: h1.clone(),
        };
        assert_eq!(compute_hash_for_event(&ev1), h1);
    }
}
