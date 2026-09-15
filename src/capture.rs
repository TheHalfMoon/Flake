//! Source capture and opaque artifact admission (`T02-02`).
//!
//! **Architecture.** Exactly like `crate::project` (`T02-01`), a `Source` is
//! an ordinary opaque-payload canonical object — no new `CommandTarget`
//! variant, no schema change. It is admitted as a fifth
//! [`crate::project::RecordPayload`] kind, sharing that enum's single
//! serialization choke point, so it inherits the identical atomic-commit,
//! idempotency and expected-revision-conflict guarantees `T01-03`/`T01-07`
//! already proved. A file import's "source revision" (exact captured bytes,
//! digest, length, observed time) is simply the `Source` object's own next
//! revision — the same "archive is a new revision, history is preserved"
//! pattern `project::archive_project` already established. This task does
//! **not** introduce a separately-referable `Artifact` entity shared by
//! multiple owners (e.g. agent proposals) — section 15 names that as a
//! distinct row, but nothing in this task's own scope needs an artifact
//! addressable independently of the `Source` that captured it. Deferred to
//! whichever later task first needs a shared artifact reference.
//!
//! **Security boundary — read before extending (S01/S03/S04/S07).**
//!
//! - Only a single, explicitly user-supplied path is ever opened. There is
//!   no directory walk anywhere in this module — `no recursive home/
//!   repository scan` is satisfied structurally, not by a runtime check,
//!   because no code path here ever calls `read_dir`.
//! - A symlink/reparse point on the exact supplied path is refused via
//!   `symlink_metadata` **before** `File::open` ever touches it (mirrors
//!   `crate::locator::open_confined`'s own ordering rationale: check first,
//!   because an opened handle cannot un-resolve a symlink after the fact).
//! - An "obviously a secret" filename is refused by a small, named,
//!   deliberately incomplete denylist (`detect_obvious_secret`). This is a
//!   best-effort footgun reducer, not a content scanner or a security
//!   guarantee — a secret can be named anything, and this cannot see inside
//!   a file it refuses to admit. The limitation is stated in the refusal
//!   message itself, not only in this comment (S07 "explain limits").
//! - The size ceiling is enforced against **actually-read bytes** captured
//!   through a bounded `Read::take`, never against `fs::metadata().len()`
//!   alone — a stat-only check would satisfy this task's own performance
//!   gate ("bounded reads after open, not metadata-only checks") for the
//!   wrong reason: it would let a file whose reported size lies (a virtual
//!   or racing filesystem entry) admit unbounded content.
//! - Nothing here executes, parses-as-executable, or renders the captured
//!   bytes. Nothing here makes a network call. Nothing here creates a
//!   `Decision` or a `Relation` from an import — nothing here decides
//!   anything is true; it only records that these exact bytes were seen.
//! - `origin_label` is unconditionally `"unknown"` in this task: there is no
//!   rights/authorship verification mechanism yet, and inventing a claimed
//!   origin would be exactly the "invalid provenance" (F10) this plan
//!   forbids. `source_mtime` is the filesystem's own reported modification
//!   time when available, or absent — never a fabricated stand-in (I11
//!   "observed time not invented source time").

use crate::canonical::{CanonicalStore, CanonicalWriter};
use crate::project::{self, RecordPayload};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Map as JsonMap;
use std::io::Read;
use std::path::Path;

/// §15 "Artifact ... Max 64 MiB per artifact." Checked against bytes this
/// module actually read, not against filesystem-reported metadata alone.
pub const MAX_ARTIFACT_BYTES: usize = 64 << 20;
/// §27 "Normal inputs: source locator label ≤4 KiB."
pub const MAX_SOURCE_LABEL_BYTES: usize = 4096;
/// A conservative bound on the sanitized display filename alone (never the
/// full supplied path, which is not stored at all — only its final
/// component). Reuses `project`'s title bound; no dedicated §27 limit is
/// named for a filename specifically.
pub const MAX_DISPLAY_FILENAME_BYTES: usize = project::MAX_TITLE_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    File,
    ManualReference,
}

/// One immutable capture of exact bytes — logically "the source revision"
/// from section 15, physically just the `Source` object's own current
/// revision payload. Present only when `SourceKind::File` has actually been
/// imported; absent for a pure `ManualReference`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Capture {
    /// Exact original bytes, hex-encoded (no new dependency is admitted for
    /// base64; hex costs 2x instead of ~1.33x, which this module's own
    /// widened transport backstop already accounts for — see
    /// `crate::limits::MAX_COMMAND_PAYLOAD_BYTES`).
    pub bytes_hex: String,
    /// The authoritative original length in bytes, independent of
    /// `bytes_hex`'s own string length.
    pub byte_length: u64,
    /// SHA-256 of the original bytes (not of `bytes_hex`).
    pub sha256: String,
    /// Sanitized final path component only — never the full supplied path.
    pub display_filename: String,
    /// Best-effort label from the filename extension. Never sniffed from
    /// content (content is never parsed), and never trusted as a reason to
    /// execute or render anything.
    pub mime_label: String,
    /// This module's own wall-clock capture time, UTC RFC3339.
    pub observed_at: String,
    /// The filesystem's own reported modification time, UTC RFC3339, when
    /// the platform/filesystem provides one. `None` is honest absence, not
    /// an invented "unknown" string mixed into a time field.
    pub source_mtime: Option<String>,
    /// Always `"unknown"` in this task — see module docs.
    pub origin_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Source {
    pub payload_schema_version: u32,
    pub project_id: String,
    pub label: String,
    pub source_kind: SourceKind,
    pub claimed_repository: Option<String>,
    pub claimed_commit: Option<String>,
    pub claimed_path: Option<String>,
    pub active: bool,
    pub capture: Option<Capture>,
    #[serde(flatten)]
    pub unknown: JsonMap<String, serde_json::Value>,
}

/// Named, admittedly incomplete filename denylist for S07's "obvious-secret
/// rejection." Matched case-insensitively against the sanitized display
/// filename only. An explicit non-secret carve-out exists for common
/// "not actually a secret" suffixes (`.example`, `.sample`, `.template`,
/// `.dist`) so `.env.example` remains importable.
const OBVIOUS_SECRET_EXACT: &[&str] = &[
    ".env",
    ".npmrc",
    ".netrc",
    ".pgpass",
    "id_rsa",
    "id_dsa",
    "id_ecdsa",
    "id_ed25519",
    "credentials",
    "credentials.json",
    "secrets.yml",
    "secrets.yaml",
    "secrets.json",
    ".htpasswd",
];
const OBVIOUS_SECRET_SUFFIXES: &[&str] = &[
    ".pem", ".pfx", ".p12", ".pk8", ".key", ".ppk", ".asc", ".gpg",
];
const NON_SECRET_CARVE_OUT_SUFFIXES: &[&str] = &[".example", ".sample", ".template", ".dist"];

fn detect_obvious_secret(display_filename: &str) -> Option<&'static str> {
    let lower = display_filename.to_ascii_lowercase();
    for carve_out in NON_SECRET_CARVE_OUT_SUFFIXES {
        if lower.ends_with(carve_out) {
            return None;
        }
    }
    for pattern in OBVIOUS_SECRET_EXACT {
        if lower == *pattern {
            return Some(pattern);
        }
    }
    OBVIOUS_SECRET_SUFFIXES
        .iter()
        .find(|suffix| lower.ends_with(*suffix))
        .copied()
}

/// Minimal, explicit, content-blind extension allowlist. Anything not
/// listed is `"application/octet-stream"` — never guessed from content,
/// never a reason to parse or render.
fn guess_mime_label(display_filename: &str) -> String {
    let lower = display_filename.to_ascii_lowercase();
    let known: &[(&str, &str)] = &[
        (".txt", "text/plain"),
        (".md", "text/markdown"),
        (".json", "application/json"),
        (".csv", "text/csv"),
        (".png", "image/png"),
        (".jpg", "image/jpeg"),
        (".jpeg", "image/jpeg"),
        (".gif", "image/gif"),
        (".pdf", "application/pdf"),
        (".zip", "application/zip"),
    ];
    for (ext, label) in known {
        if lower.ends_with(ext) {
            return (*label).to_string();
        }
    }
    "application/octet-stream".to_string()
}

fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

fn from_hex(s: &str) -> Result<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return Err(Error::Capture("hex payload has odd length".into()));
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let hi = (bytes[i] as char)
            .to_digit(16)
            .ok_or_else(|| Error::Capture("invalid hex digit".into()))?;
        let lo = (bytes[i + 1] as char)
            .to_digit(16)
            .ok_or_else(|| Error::Capture("invalid hex digit".into()))?;
        out.push(((hi << 4) | lo) as u8);
        i += 2;
    }
    Ok(out)
}

/// Correct proleptic-Gregorian UTC RFC3339 formatting from Unix epoch
/// seconds, with no `chrono`/`time` dependency admitted (same boundary
/// `vault::chrono_like_now_iso8601`/`canonical::now_iso8601_placeholder`
/// already accept for "now"-only formatting). This function must be
/// correct for an arbitrary **past** instant, not only "now": a file's real
/// modification time can be any historical date, and rendering it wrong
/// would itself be the "invented source time" I11 forbids — reusing the
/// existing fixed-fake-date placeholders here would be exactly that defect.
/// Uses Howard Hinnant's `civil_from_days` algorithm (public domain;
/// <https://howardhinnant.github.io/date_algorithms.html>), correct for the
/// full `i64` range of days.
pub(crate) fn rfc3339_utc_from_unix_seconds(total_secs: i64) -> String {
    let days = total_secs.div_euclid(86400);
    let secs_of_day = total_secs.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    let hh = secs_of_day / 3600;
    let mm = (secs_of_day % 3600) / 60;
    let ss = secs_of_day % 60;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// Inverse of [`civil_from_days`] (also Hinnant's algorithm, same source).
/// Deliberately not range/validity-checked here — arithmetic only; the
/// caller (`parse_rfc3339_utc`) proves validity by round-tripping the
/// result back through [`rfc3339_utc_from_unix_seconds`] and comparing,
/// rather than duplicating a days-in-month/leap-year table a second time.
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64; // [0, 399]
    let m = m as u64;
    let d = d as u64;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146097 + doe as i64 - 719468
}

/// Parse this module's own strict `YYYY-MM-DDTHH:MM:SSZ` UTC form back into
/// Unix epoch seconds (`T02-03`: decision valid-time interval admission).
/// Deliberately **not** a general RFC3339 parser — no fractional seconds, no
/// non-`Z` offsets, no other separators — only the exact shape this crate's
/// own `rfc3339_utc_from_unix_seconds` ever emits, matching this codebase's
/// existing "reuse the formatter as the validity oracle" approach: the
/// parsed value is re-formatted and compared byte-for-byte against the
/// input, which rejects an impossible calendar date (e.g. February 30)
/// without a separate days-in-month table (I11: no invented time — an
/// unparseable or impossible timestamp is refused, never coerced).
pub(crate) fn parse_rfc3339_utc(s: &str) -> Result<i64> {
    let b = s.as_bytes();
    let valid_shape = b.len() == 20
        && b[4] == b'-'
        && b[7] == b'-'
        && b[10] == b'T'
        && b[13] == b':'
        && b[16] == b':'
        && b[19] == b'Z';
    if !valid_shape {
        return Err(Error::Capture(format!(
            "not a valid UTC timestamp, expected YYYY-MM-DDTHH:MM:SSZ: {s}"
        )));
    }
    let field = |range: std::ops::Range<usize>| -> Result<i64> {
        s.get(range.clone())
            .and_then(|f| f.parse::<i64>().ok())
            .ok_or_else(|| Error::Capture(format!("invalid timestamp field in {s}: {range:?}")))
    };
    let year = field(0..4)?;
    let month = field(5..7)?;
    let day = field(8..10)?;
    let hour = field(11..13)?;
    let minute = field(14..16)?;
    let second = field(17..19)?;
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || !(0..24).contains(&hour)
        || !(0..60).contains(&minute)
        || !(0..60).contains(&second)
    {
        return Err(Error::Capture(format!("timestamp field out of range: {s}")));
    }
    let days = days_from_civil(year, month as u32, day as u32);
    let total_secs = days * 86400 + hour * 3600 + minute * 60 + second;
    if rfc3339_utc_from_unix_seconds(total_secs) != s {
        return Err(Error::Capture(format!(
            "not a valid calendar date/time (failed round-trip check): {s}"
        )));
    }
    Ok(total_secs)
}

pub(crate) fn now_rfc3339_utc() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    rfc3339_utc_from_unix_seconds(secs)
}

fn system_time_to_rfc3339_utc(t: std::time::SystemTime) -> Option<String> {
    use std::time::UNIX_EPOCH;
    let secs = t.duration_since(UNIX_EPOCH).ok()?.as_secs();
    Some(rfc3339_utc_from_unix_seconds(secs as i64))
}

/// Final path component only, sanitized. Refuses anything that is not a
/// plain, printable name (empty, path separators, control characters).
fn sanitize_display_filename(path: &Path) -> Result<String> {
    let name = path
        .file_name()
        .ok_or_else(|| {
            Error::Capture(format!("selected path has no filename: {}", path.display()))
        })?
        .to_string_lossy()
        .into_owned();
    if name.is_empty()
        || name.contains('/')
        || name.contains('\\')
        || name.chars().any(|c| c.is_control())
    {
        return Err(Error::Capture(format!(
            "unsafe display filename derived from {}",
            path.display()
        )));
    }
    if name.len() > MAX_DISPLAY_FILENAME_BYTES {
        return Err(Error::Capture(format!(
            "display filename exceeds limit: {} > {} bytes",
            name.len(),
            MAX_DISPLAY_FILENAME_BYTES
        )));
    }
    Ok(name)
}

/// Open, validate and capture exactly one explicitly-selected regular file.
/// No directory is ever read; no path other than `path` itself is ever
/// touched. F08 "missing source" and S03 "reject symlinks" are both
/// detected here, before any transaction opens.
///
/// `pub(crate)`, not private: `T03-01`'s `crate::source_check` reuses this
/// exact function (not a duplicate) whenever it needs to admit a *new*
/// capture of already-fully-defended bytes (`admit_changed_source`) —
/// direct counterpart reuse, the same choice `T02-06` already made for
/// `export::compute_integrity_root`. Its own plain recheck-only path
/// (`observe`) is a deliberate, separately-documented non-reuse — see
/// `source_check.rs` module docs for why.
pub(crate) fn capture_file(path: &Path) -> Result<Capture> {
    let meta = std::fs::symlink_metadata(path).map_err(|e| {
        Error::Capture(format!(
            "selected source is missing or unreachable: {}: {e}",
            path.display()
        ))
    })?;
    if meta.file_type().is_symlink() {
        return Err(Error::Capture(format!(
            "symlink not followed (S03): {}",
            path.display()
        )));
    }
    if !meta.file_type().is_file() {
        return Err(Error::Capture(format!(
            "not a regular file, no recursive scan performed: {}",
            path.display()
        )));
    }

    let display_filename = sanitize_display_filename(path)?;
    if let Some(pattern) = detect_obvious_secret(&display_filename) {
        return Err(Error::Capture(format!(
            "refusing an obviously-secret filename (matched {pattern:?}); this is a \
             best-effort name-based heuristic, not a content scan — it cannot see \
             inside files it refuses, and cannot catch a secret under an unrelated \
             name. Rename the file if this is a false positive."
        )));
    }

    let mut file = std::fs::File::open(path)
        .map_err(|e| Error::Capture(format!("cannot open {}: {e}", path.display())))?;
    // Bounded read against the actually-opened handle, never trusted from
    // `meta.len()` alone (this task's own performance gate: "bounded reads
    // after open, not metadata-only checks").
    let mut buf = Vec::new();
    file.by_ref()
        .take(MAX_ARTIFACT_BYTES as u64 + 1)
        .read_to_end(&mut buf)
        .map_err(|e| Error::Capture(format!("cannot read {}: {e}", path.display())))?;
    if buf.len() > MAX_ARTIFACT_BYTES {
        return Err(Error::Capture(format!(
            "artifact exceeds limit: > {MAX_ARTIFACT_BYTES} bytes, refusing partial admission"
        )));
    }

    let sha256 = crate::events::hash_bytes(&buf);
    let byte_length = buf.len() as u64;
    let bytes_hex = to_hex(&buf);
    let mime_label = guess_mime_label(&display_filename);
    let observed_at = now_rfc3339_utc();
    let source_mtime = meta.modified().ok().and_then(system_time_to_rfc3339_utc);

    Ok(Capture {
        bytes_hex,
        byte_length,
        sha256,
        display_filename,
        mime_label,
        observed_at,
        source_mtime,
        origin_label: "unknown".to_string(),
    })
}

/// Import one explicitly-selected regular file as a new `Source` in
/// `project_id`. Rejects before any transaction opens: invalid project
/// reference, oversized/unreadable/symlinked/secret-named path, or a label
/// over `MAX_SOURCE_LABEL_BYTES`.
pub fn import_file(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    project_id: &str,
    label: &str,
    path: &Path,
) -> Result<(crate::canonical::CommandOutcome, Source)> {
    project::require_project(writer.store(), project_id)?;
    project::check_len("source label", label, MAX_SOURCE_LABEL_BYTES)?;
    let capture = capture_file(path)?;
    let source = Source {
        payload_schema_version: project::RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: project_id.to_string(),
        label: label.to_string(),
        source_kind: SourceKind::File,
        claimed_repository: None,
        claimed_commit: None,
        // `T03-01`: the exact path the owner selected, retained as the
        // "owner-selected locator hint" (§15) so a later explicit
        // `source_check::check_source` has something to recheck against.
        // Local-only metadata, same non-authoritative status
        // `claimed_repository`/`claimed_commit` already document — never
        // filesystem access on its own, never re-followed on another
        // machine or without an explicit owner action.
        claimed_path: Some(path.to_string_lossy().into_owned()),
        active: true,
        capture: Some(capture),
        unknown: JsonMap::new(),
    };
    let (outcome, record) = project::commit_create(
        writer,
        actor,
        crate::canonical::RecordOrigin::User,
        RecordPayload::Source(source),
    )?;
    Ok((outcome, record.as_source()?.clone()))
}

/// Register a pure external reference with no captured bytes ("explicitly
/// reference-only" — section 15).
pub fn create_manual_reference(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    project_id: &str,
    label: &str,
    claimed_repository: Option<&str>,
    claimed_commit: Option<&str>,
    claimed_path: Option<&str>,
) -> Result<(crate::canonical::CommandOutcome, Source)> {
    project::require_project(writer.store(), project_id)?;
    project::check_len("source label", label, MAX_SOURCE_LABEL_BYTES)?;
    let source = Source {
        payload_schema_version: project::RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: project_id.to_string(),
        label: label.to_string(),
        source_kind: SourceKind::ManualReference,
        claimed_repository: claimed_repository.map(str::to_string),
        claimed_commit: claimed_commit.map(str::to_string),
        claimed_path: claimed_path.map(str::to_string),
        active: true,
        capture: None,
        unknown: JsonMap::new(),
    };
    let (outcome, record) = project::commit_create(
        writer,
        actor,
        crate::canonical::RecordOrigin::User,
        RecordPayload::Source(source),
    )?;
    Ok((outcome, record.as_source()?.clone()))
}

fn set_source_active(
    store: &mut CanonicalStore,
    actor: &str,
    source_id: &str,
    active: bool,
) -> Result<(crate::canonical::CommandOutcome, Source)> {
    let (revision_id, payload) = store
        .read_current(source_id)?
        .ok_or_else(|| Error::Capture(format!("no source exists with id {source_id}")))?;
    let mut source = RecordPayload::from_json(&payload)?.as_source()?.clone();
    source.active = active;
    let mut writer = store.writer()?;
    let (outcome, record) = project::commit_update(
        &mut writer,
        actor,
        crate::canonical::RecordOrigin::User,
        source_id,
        &revision_id,
        RecordPayload::Source(source),
    )?;
    Ok((outcome, record.as_source()?.clone()))
}

/// Mark a source unavailable (F08 "missing source ... retain saved source
/// revision"). The prior revision's captured bytes remain in history.
pub fn deactivate_source(
    store: &mut CanonicalStore,
    actor: &str,
    source_id: &str,
) -> Result<(crate::canonical::CommandOutcome, Source)> {
    set_source_active(store, actor, source_id, false)
}

pub fn reactivate_source(
    store: &mut CanonicalStore,
    actor: &str,
    source_id: &str,
) -> Result<(crate::canonical::CommandOutcome, Source)> {
    set_source_active(store, actor, source_id, true)
}

/// Recover the exact original bytes of a captured `Source` — this task's
/// own objective, "recoverable bytes," made concrete: a caller can always
/// get back precisely what was admitted, not merely inspect its metadata.
/// Refuses a `ManualReference` (nothing was ever captured) or a wrong-kind
/// object explicitly rather than returning empty bytes.
pub fn extract_bytes(store: &CanonicalStore, source_id: &str) -> Result<Vec<u8>> {
    let (_, payload) = store
        .read_current(source_id)?
        .ok_or_else(|| Error::Capture(format!("no source exists with id {source_id}")))?;
    let source = RecordPayload::from_json(&payload)?.as_source()?.clone();
    let capture = source.capture.ok_or_else(|| {
        Error::Capture(format!(
            "source {source_id} is a manual reference with no captured bytes to recover"
        ))
    })?;
    from_hex(&capture.bytes_hex)
}

/// Every `Source` currently belonging to `project_id` — the `Source`
/// counterpart of `project::list_project_records`. A full scan, same
/// documented limitation as that function.
pub fn list_project_sources(
    store: &CanonicalStore,
    project_id: &str,
) -> Result<Vec<(String, Source)>> {
    let mut out = Vec::new();
    for (object_id, _revision_id, payload) in store.list_current_objects()? {
        if let Ok(RecordPayload::Source(source)) = RecordPayload::from_json(&payload) {
            if source.project_id == project_id {
                out.push((object_id, source));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::CanonicalStore;
    use std::path::PathBuf;

    fn tmp() -> PathBuf {
        let d = std::env::temp_dir().join(format!("fehrest-capture-{}", uuid::Uuid::now_v7()));
        std::fs::create_dir_all(&d).unwrap();
        d
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
    fn hex_round_trips_every_byte_value() {
        let all: Vec<u8> = (0..=255).collect();
        let encoded = to_hex(&all);
        let decoded = from_hex(&encoded).unwrap();
        assert_eq!(decoded, all);
    }

    #[test]
    fn rfc3339_formats_known_epoch_fixed_points() {
        assert_eq!(rfc3339_utc_from_unix_seconds(0), "1970-01-01T00:00:00Z");
        assert_eq!(rfc3339_utc_from_unix_seconds(86399), "1970-01-01T23:59:59Z");
        assert_eq!(rfc3339_utc_from_unix_seconds(86400), "1970-01-02T00:00:00Z");
        // Widely-cited reference point ("Y2K epoch").
        assert_eq!(
            rfc3339_utc_from_unix_seconds(946_684_800),
            "2000-01-01T00:00:00Z"
        );
        // 2000 was a leap year: Jan (31) + Feb (29) days after 2000-01-01
        // lands exactly on 2000-03-01, proving Feb 29 is handled.
        assert_eq!(
            rfc3339_utc_from_unix_seconds(946_684_800 + (31 + 29) * 86400),
            "2000-03-01T00:00:00Z"
        );
        // 2000 has 366 days (leap); 2001 Feb has 28 (non-leap), confirming
        // the century/400-year leap rule is not just "every 4 years".
        assert_eq!(
            rfc3339_utc_from_unix_seconds(946_684_800 + 366 * 86400),
            "2001-01-01T00:00:00Z"
        );
        assert_eq!(
            rfc3339_utc_from_unix_seconds(946_684_800 + (366 + 31 + 28) * 86400),
            "2001-03-01T00:00:00Z"
        );
    }

    #[test]
    fn parse_rfc3339_utc_round_trips_known_fixed_points() {
        for secs in [
            0i64,
            86399,
            86400,
            946_684_800,                     // 2000-01-01T00:00:00Z
            946_684_800 + (31 + 29) * 86400, // 2000-03-01T00:00:00Z (leap Feb)
            946_684_800 + 366 * 86400,       // 2001-01-01T00:00:00Z
        ] {
            let s = rfc3339_utc_from_unix_seconds(secs);
            assert_eq!(parse_rfc3339_utc(&s).unwrap(), secs, "round trip for {s}");
        }
    }

    #[test]
    fn parse_rfc3339_utc_rejects_malformed_and_impossible_dates() {
        for bad in [
            "not-a-timestamp",
            "2024-01-01 00:00:00Z", // wrong separator
            "2024-01-01T00:00:00",  // missing Z
            "2024-13-01T00:00:00Z", // month out of range
            "2024-02-30T00:00:00Z", // impossible calendar date (round-trip mismatch)
            "2024-01-01T24:00:00Z", // hour out of range
        ] {
            assert!(
                parse_rfc3339_utc(bad).is_err(),
                "expected {bad} to be rejected"
            );
        }
    }

    #[test]
    fn imports_a_binary_fixture_with_exact_byte_fidelity() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);

        // Every byte value including NUL, high bytes and non-UTF-8
        // sequences — proves opaque fidelity, not merely "text happens to
        // survive".
        let mut fixture: Vec<u8> = (0u8..=255).collect();
        fixture.extend_from_slice(&[0xFF, 0x00, 0xFE, 0x80, 0x01]);
        let file_path = root.join("fixture.bin");
        std::fs::write(&file_path, &fixture).unwrap();

        let (outcome, source) = {
            let mut writer = store.writer().unwrap();
            import_file(
                &mut writer,
                "owner",
                &project_id,
                "a binary fixture",
                &file_path,
            )
            .unwrap()
        };
        assert_eq!(source.source_kind, SourceKind::File);
        let capture = source.capture.as_ref().unwrap();
        assert_eq!(capture.byte_length, fixture.len() as u64);
        assert_eq!(capture.sha256, crate::events::hash_bytes(&fixture));
        assert_eq!(from_hex(&capture.bytes_hex).unwrap(), fixture);
        assert_eq!(capture.display_filename, "fixture.bin");
        assert_eq!(capture.origin_label, "unknown");
        assert!(capture.source_mtime.is_some());
        // `T03-01`: the exact selected path is now retained as the source's
        // own locator hint, so a later `source_check::check_source` has
        // something to recheck against.
        assert_eq!(
            source.claimed_path.as_deref(),
            Some(file_path.to_string_lossy()).as_deref()
        );

        // Round-trips through read_current/from_json exactly.
        let (_, payload) = store.read_current(&outcome.object_id).unwrap().unwrap();
        let reread = RecordPayload::from_json(&payload).unwrap();
        assert_eq!(reread.as_source().unwrap(), &source);

        cleanup(&root);
    }

    #[test]
    fn imports_unicode_and_crlf_text_with_exact_byte_fidelity() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);

        let text = "héllo\r\n日本語\r\nemoji: \u{1F600}\r\n";
        let file_path = root.join("notes.txt");
        std::fs::write(&file_path, text.as_bytes()).unwrap();

        let (_, source) = {
            let mut writer = store.writer().unwrap();
            import_file(
                &mut writer,
                "owner",
                &project_id,
                "unicode text",
                &file_path,
            )
            .unwrap()
        };
        let capture = source.capture.unwrap();
        let recovered = from_hex(&capture.bytes_hex).unwrap();
        assert_eq!(recovered, text.as_bytes());
        assert_eq!(String::from_utf8(recovered).unwrap(), text);

        cleanup(&root);
    }

    #[test]
    fn refuses_a_symlink_without_ever_opening_the_target() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);

        let target = root.join("real-secret.txt");
        std::fs::write(&target, b"target contents").unwrap();
        let link = root.join("link.txt");

        #[cfg(unix)]
        std::os::unix::fs::symlink(&target, &link).unwrap();
        #[cfg(windows)]
        let symlink_created = std::os::windows::fs::symlink_file(&target, &link).is_ok();
        #[cfg(not(windows))]
        let symlink_created = true;

        #[cfg(windows)]
        if !symlink_created {
            // Creating a file symlink on Windows can require an elevated
            // privilege/dev-mode the CI runner may lack; skip rather than
            // fabricate a pass this environment cannot exercise.
            cleanup(&root);
            return;
        }

        let err = {
            let mut writer = store.writer().unwrap();
            import_file(&mut writer, "owner", &project_id, "l", &link).unwrap_err()
        };
        assert!(format!("{err}").contains("symlink not followed"));
        assert_eq!(store.list_current_objects().unwrap().len(), 1); // only the project
        cleanup(&root);
    }

    #[test]
    fn refuses_a_directory_and_a_missing_path() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let subdir = root.join("a-directory");
        std::fs::create_dir_all(&subdir).unwrap();

        let err = {
            let mut writer = store.writer().unwrap();
            import_file(&mut writer, "owner", &project_id, "l", &subdir).unwrap_err()
        };
        assert!(format!("{err}").contains("not a regular file"));

        let missing = root.join("does-not-exist.txt");
        let err = {
            let mut writer = store.writer().unwrap();
            import_file(&mut writer, "owner", &project_id, "l", &missing).unwrap_err()
        };
        assert!(format!("{err}").contains("missing or unreachable"));

        cleanup(&root);
    }

    #[test]
    fn refuses_obvious_secret_filenames_but_admits_their_example_carve_out() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);

        for name in [".env", "id_rsa", "credentials.json", "server.pem"] {
            let p = root.join(name);
            std::fs::write(&p, b"sensitive").unwrap();
            let err = {
                let mut writer = store.writer().unwrap();
                import_file(&mut writer, "owner", &project_id, "l", &p).unwrap_err()
            };
            assert!(
                format!("{err}").contains("obviously-secret"),
                "expected secret refusal for {name}, got {err}"
            );
        }

        // The explicit non-secret carve-out is admitted normally.
        let example = root.join(".env.example");
        std::fs::write(&example, b"EXAMPLE=1").unwrap();
        let ok = {
            let mut writer = store.writer().unwrap();
            import_file(&mut writer, "owner", &project_id, "l", &example)
        };
        assert!(ok.is_ok(), "{:?}", ok.err());

        cleanup(&root);
    }

    #[test]
    fn oversized_artifact_is_refused_before_any_mutation() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);

        let big = root.join("too-big.bin");
        // Sparse-ish allocation via set_len avoids actually writing
        // MAX_ARTIFACT_BYTES+1 real bytes to the shared test host's disk —
        // the content is never inspected once the length check refuses it.
        let file = std::fs::File::create(&big).unwrap();
        file.set_len(MAX_ARTIFACT_BYTES as u64 + 1).unwrap();
        drop(file);

        let head_before = store.transaction_head().unwrap();
        let err = {
            let mut writer = store.writer().unwrap();
            import_file(&mut writer, "owner", &project_id, "l", &big).unwrap_err()
        };
        assert!(format!("{err}").contains("exceeds limit"));
        assert_eq!(store.transaction_head().unwrap(), head_before);

        cleanup(&root);
    }

    #[test]
    fn extract_bytes_recovers_the_exact_original_content() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let fixture: Vec<u8> = (0u8..=255).cycle().take(10_000).collect();
        let file_path = root.join("data.bin");
        std::fs::write(&file_path, &fixture).unwrap();

        let (outcome, _) = {
            let mut writer = store.writer().unwrap();
            import_file(&mut writer, "owner", &project_id, "l", &file_path).unwrap()
        };
        let recovered = extract_bytes(&store, &outcome.object_id).unwrap();
        assert_eq!(recovered, fixture);

        cleanup(&root);
    }

    #[test]
    fn extract_bytes_refuses_a_manual_reference_with_no_captured_bytes() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (outcome, _) = {
            let mut writer = store.writer().unwrap();
            create_manual_reference(&mut writer, "owner", &project_id, "l", None, None, None)
                .unwrap()
        };
        let err = extract_bytes(&store, &outcome.object_id).unwrap_err();
        assert!(format!("{err}").contains("no captured bytes"));
        cleanup(&root);
    }

    #[test]
    fn manual_reference_source_carries_no_bytes() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);

        let (_, source) = {
            let mut writer = store.writer().unwrap();
            create_manual_reference(
                &mut writer,
                "owner",
                &project_id,
                "external doc",
                Some("org/repo"),
                Some("abc123"),
                Some("docs/x.md"),
            )
            .unwrap()
        };
        assert_eq!(source.source_kind, SourceKind::ManualReference);
        assert!(source.capture.is_none());
        assert_eq!(source.claimed_repository.as_deref(), Some("org/repo"));

        cleanup(&root);
    }

    #[test]
    fn deactivate_and_reactivate_preserve_history_like_project_archive() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let (outcome, _) = {
            let mut writer = store.writer().unwrap();
            create_manual_reference(&mut writer, "owner", &project_id, "l", None, None, None)
                .unwrap()
        };

        let (_, deactivated) = deactivate_source(&mut store, "owner", &outcome.object_id).unwrap();
        assert!(!deactivated.active);
        let (_, reactivated) = reactivate_source(&mut store, "owner", &outcome.object_id).unwrap();
        assert!(reactivated.active);

        let history = store.history(&outcome.object_id).unwrap();
        assert_eq!(history.len(), 3);

        cleanup(&root);
    }

    #[test]
    fn source_requires_an_existing_project() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let missing_project = uuid::Uuid::now_v7().to_string();
        let err = {
            let mut writer = store.writer().unwrap();
            create_manual_reference(
                &mut writer,
                "owner",
                &missing_project,
                "l",
                None,
                None,
                None,
            )
            .unwrap_err()
        };
        assert!(format!("{err}").contains("invalid project reference"));
        cleanup(&root);
    }

    /// A process failure mid-commit for a file import leaves either nothing
    /// or the complete admitted `Source` — the same all-or-nothing guarantee
    /// `T01-07`'s fault-schedule matrices already proved for the shared
    /// commit path, exercised here through this task's own new call site
    /// rather than a new fault point (F01/F15).
    #[test]
    fn a_fault_during_source_commit_leaves_no_partial_admission() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = new_project(&mut store);
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();

        let capture = capture_file(&file_path).unwrap();
        let source = Source {
            payload_schema_version: project::RECORD_PAYLOAD_SCHEMA_VERSION,
            project_id: project_id.clone(),
            label: "l".into(),
            source_kind: SourceKind::File,
            claimed_repository: None,
            claimed_commit: None,
            claimed_path: None,
            active: true,
            capture: Some(capture),
            unknown: JsonMap::new(),
        };
        let payload = RecordPayload::Source(source).to_json().unwrap();
        let head_before = store.transaction_head().unwrap();
        {
            let mut writer = store.writer().unwrap();
            let result = writer.commit_with_fault(
                crate::canonical::CommandInput {
                    command_id: uuid::Uuid::now_v7().to_string(),
                    actor: "owner".into(),
                    origin: crate::canonical::RecordOrigin::User,
                    target: crate::canonical::CommandTarget::CreateObject { payload },
                },
                Some(crate::canonical::CommitFaultPoint::BeforeSqlCommit),
            );
            assert!(result.is_err());
        }
        // Rolled back completely: head unchanged, no orphaned source object
        // (only the project itself remains).
        assert_eq!(store.transaction_head().unwrap(), head_before);
        assert_eq!(store.list_current_objects().unwrap().len(), 1);

        cleanup(&root);
    }
}
