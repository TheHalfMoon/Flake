//! Portable, generic-reader-readable export of owned canonical state
//! (`T02-05`).
//!
//! **Distinct from `crate::backup` (`T01-05`).** Backup exists to make an
//! owner-controlled *restorable* snapshot of a healthy vault using SQLite's
//! own Online Backup API — its output is still a `canonical.sqlite` file,
//! openable only by this exact crate. This module exists for a different
//! purpose this task's own objective states directly: "leave Flake with
//! readable content and complete declared history" — an artifact a generic
//! JSON/byte tool can validate and read with **no Flake dependency at
//! all**, per this task's own acceptance criterion. Nothing here reuses or
//! extends `backup.rs`; the two remain independent, non-interfering
//! mechanisms with genuinely different output formats. `backup.rs`'s own
//! manifest already uses `kind: "full-backup"` for its SQLite-snapshot
//! artifact — this module deliberately uses different kind strings
//! (`export-full`/`export-project`) for its own, physically incompatible
//! artifact, so a reader who encounters either manifest can never conflate
//! the two formats.
//!
//! **Layout published at `<dest_root>/.fehrest-export/`:**
//!
//! ```text
//! export-manifest.json        this export's own manifest (§15 "Export/backup manifest")
//! README.md                   generic-reader instructions, plain Markdown
//! revisions/<object_id>/<recorded_seq>-<revision_id>.json   one file per revision
//! revisions/<object_id>/<recorded_seq>-<revision_id>.md     Note bodies only, plain text
//! ```
//!
//! Every path component is a UUID or a plain decimal integer — never
//! user-supplied text — so every member path is safe on every target
//! filesystem by construction (no sanitization step needed or trusted).
//!
//! **"Keep raw canonical payload bytes where JSON reserialization would
//! alter them."** Each revision file embeds the *exact* stored payload
//! string as a JSON string value (`payload_raw`) — never parsed into a
//! `project::RecordPayload` and re-emitted, which could silently reorder
//! keys or change whitespace. JSON string-escaping arbitrary UTF-8 bytes is
//! a lossless, reversible transform of the *encoding*, not a reserialization
//! of the payload's own JSON *structure* — `payload_raw` decodes back to
//! byte-for-byte the same string `canonical.sqlite`'s `revision.payload`
//! column holds. `kind`/`title` alongside it are read-only conveniences for
//! a human skimming the export, never authoritative and never fed back into
//! anything.
//!
//! **Scope.** A project export includes exactly the `Note`/`Action`/
//! `Decision`/`Source`/`Relation` objects currently belonging to that
//! project (the same per-kind `list_project_*` functions `T02-01`-`T02-04`
//! already established), plus the `Project` object itself — full history
//! for every one of them, not just their current revision. A full export
//! includes every revision of every object in the store, of any kind.
//!
//! **Not implemented here** (recorded, not silently dropped): streaming
//! export for an L-scale store too large to hold `all_revisions()` in
//! memory at once (`T05-02`'s own hardware-qualified pass, matching every
//! prior task's identical deferral); package **import** (`T02-06`'s
//! objective); and any encryption/redaction beyond "this format carries no
//! local filesystem path or grant authority to begin with" — there is
//! nothing in the current record model to redact (`Source::claimed_path`
//! and friends are already-documented non-authoritative descriptive
//! strings, never real filesystem access).

use crate::canonical::{CanonicalStore, RevisionEnvelope};
use crate::project::{self, RecordPayload};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

const EXPORT_CONTROL_DIR: &str = ".fehrest-export";
const EXPORT_MANIFEST_FILE: &str = "export-manifest.json";
const EXPORT_README_FILE: &str = "README.md";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExportManifestMember {
    pub path: String,
    pub length: u64,
    pub sha256: String,
}

/// §15 "Export/backup manifest" applied to this module's own portable
/// format — see module docs for why `kind` is distinct from
/// `backup::BackupManifest::kind`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExportManifest {
    pub schema: String,
    /// `"export-full"` or `"export-project"`.
    pub kind: String,
    pub vault_id: String,
    /// `Some` only for `kind: "export-project"`.
    pub project_id: Option<String>,
    /// Captured before any revision is read — "manifest snapshot head
    /// excludes its own later export event" (I05/§15).
    pub snapshot_head_seq: i64,
    pub snapshot_head_hash: Option<String>,
    pub created_at: String,
    pub record_count: usize,
    pub revision_count: usize,
    /// Always empty in this task's own scope — every in-scope revision is
    /// included, never silently skipped. Present (not omitted from the
    /// schema) so a reader never has to guess whether omission-tracking
    /// exists at all.
    pub omissions: Vec<String>,
    pub members: Vec<ExportManifestMember>,
    /// Sorted-member-list hash over `(schema, kind, vault_id, project_id,
    /// snapshot_head_seq, members)`, excluding this field itself (§15
    /// "the manifest does not hash itself").
    pub integrity_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportReport {
    pub dest_root: PathBuf,
    pub manifest: ExportManifest,
}

/// A no-write dry run — "sensitive history and scope shown before writing"
/// (§12). Computes exactly what [`export_to_new_root`] would produce,
/// without touching the filesystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportPreview {
    pub kind: String,
    pub project_id: Option<String>,
    pub record_count: usize,
    pub revision_count: usize,
    pub snapshot_head_seq: i64,
}

/// Every object ID that belongs in scope for `project_id`: the project
/// itself plus every `Note`/`Action`/`Decision` (`project::
/// list_project_records`), `Source` (`capture::list_project_sources`),
/// `Relation` (`relation::list_project_relations`) and `SourceCheck`
/// (`source_check::list_project_source_checks`, `T03-01`) currently scoped
/// to it — reusing each kind's own already-established listing function
/// rather than re-deriving project membership here. Every new record kind
/// this crate ever adds must be added to this union too, or a
/// project-scoped export would silently omit it (I06/I10) — exactly the
/// class of omission `T02-07`'s independent verifier exists to catch.
fn project_scope_object_ids(store: &CanonicalStore, project_id: &str) -> Result<HashSet<String>> {
    let mut ids = HashSet::new();
    ids.insert(project_id.to_string());
    for (id, _) in project::list_project_records(store, project_id)? {
        ids.insert(id);
    }
    for (id, _) in crate::capture::list_project_sources(store, project_id)? {
        ids.insert(id);
    }
    for (id, _) in crate::relation::list_project_relations(store, project_id)? {
        ids.insert(id);
    }
    for (id, _) in crate::source_check::list_project_source_checks(store, project_id)? {
        ids.insert(id);
    }
    Ok(ids)
}

fn compute_preview(
    store: &CanonicalStore,
    project_id: Option<&str>,
) -> Result<(HashSet<String>, i64, Option<String>)> {
    let (snapshot_head_seq, snapshot_head_hash) = store.transaction_head()?;
    let scope = match project_id {
        Some(p) => {
            project::require_project(store, p)?;
            project_scope_object_ids(store, p)?
        }
        None => store
            .list_current_objects()?
            .into_iter()
            .map(|(id, _, _)| id)
            .collect(),
    };
    Ok((scope, snapshot_head_seq, snapshot_head_hash))
}

/// Preview an export without writing anything. See module docs, "Scope".
pub fn preview_export(store: &CanonicalStore, project_id: Option<&str>) -> Result<ExportPreview> {
    let (scope, snapshot_head_seq, _) = compute_preview(store, project_id)?;
    let revision_count = store
        .all_revisions()?
        .into_iter()
        .filter(|r| scope.contains(&r.object_id))
        .count();
    Ok(ExportPreview {
        kind: if project_id.is_some() {
            "export-project"
        } else {
            "export-full"
        }
        .to_string(),
        project_id: project_id.map(str::to_string),
        record_count: scope.len(),
        revision_count,
        snapshot_head_seq,
    })
}

fn sha256_hex(bytes: &[u8]) -> String {
    crate::events::hash_bytes(bytes)
}

pub(crate) fn compute_integrity_root(
    schema: &str,
    kind: &str,
    vault_id: &str,
    project_id: Option<&str>,
    snapshot_head_seq: i64,
    members: &[ExportManifestMember],
) -> String {
    let mut sorted = members.to_vec();
    sorted.sort_by(|a, b| a.path.cmp(&b.path));
    let value = serde_json::json!({
        "schema": schema,
        "kind": kind,
        "vault_id": vault_id,
        "project_id": project_id,
        "snapshot_head_seq": snapshot_head_seq,
        "members": sorted.iter().map(|m| serde_json::json!({
            "path": m.path, "length": m.length, "sha256": m.sha256,
        })).collect::<Vec<_>>(),
    });
    sha256_hex(serde_json::to_string(&value).unwrap().as_bytes())
}

fn readme_text(kind: &str) -> String {
    format!(
        "# Flake portable export ({kind})\n\n\
This directory is a **portable export** of owned Flake state. It is plain \
JSON and Markdown files; no Flake install, SQLite library, or any other \
tool-specific dependency is needed to read or verify it.\n\n\
## Layout\n\n\
- `export-manifest.json` — this export's own manifest: every member's path, \
byte length and SHA-256, plus an `integrity_root` covering the whole sorted \
member list. Recompute it yourself (sort members by `path`, hash the JSON \
object `{{schema, kind, vault_id, project_id, snapshot_head_seq, members}}` \
with SHA-256) to independently verify nothing was altered.\n\
- `revisions/<object_id>/<recorded_seq>-<revision_id>.json` — one file per \
canonical revision, oldest state and every later state both included \
(never only the current one). `payload_raw` is the *exact* original stored \
payload string, byte-for-byte — do not re-format it before comparing its \
hash against `revisions[].payload_sha256`.\n\
- `revisions/<object_id>/<recorded_seq>-<revision_id>.md` — present only for \
`Note` revisions: the note's plain-text Markdown body alone, for easy \
reading. Never HTML; never executed by anything that reads it.\n\n\
## What this is not\n\n\
This is **not** a live Flake vault — you cannot point `CanonicalStore::open` \
at this directory. It carries no writer lock, no live transaction log, and \
publishing it does not grant any local filesystem or agent authority to \
whoever receives it.\n"
    )
}

fn write_member(control: &Path, rel_path: &str, bytes: &[u8]) -> Result<ExportManifestMember> {
    let full = control.join(rel_path);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| Error::Vault(format!("cannot create export directory: {e}")))?;
    }
    fs::write(&full, bytes)
        .map_err(|e| Error::Vault(format!("cannot write export member {rel_path}: {e}")))?;
    Ok(ExportManifestMember {
        path: rel_path.replace('\\', "/"),
        length: bytes.len() as u64,
        sha256: sha256_hex(bytes),
    })
}

#[derive(Debug, Serialize)]
struct RevisionFile<'a> {
    schema: &'a str,
    object_id: &'a str,
    revision_id: &'a str,
    parent_revision_id: &'a Option<String>,
    recorded_seq: i64,
    recorded_at: &'a str,
    actor: &'a str,
    origin: &'a str,
    kind: Option<&'a str>,
    payload_sha256: &'a str,
    payload_raw: &'a str,
}

/// Export a full-vault or single-project portable package to a brand-new
/// `dest_root`. Refuses if `<dest_root>/.fehrest-export` already exists —
/// "partial output never overwrites a prior export" (F17), the identical
/// no-clobber-staging-then-rename discipline `backup.rs` already
/// established for its own publication.
pub fn export_to_new_root(
    store: &CanonicalStore,
    project_id: Option<&str>,
    dest_root: impl AsRef<Path>,
) -> Result<ExportReport> {
    let dest_root = dest_root.as_ref().to_path_buf();
    let published = dest_root.join(EXPORT_CONTROL_DIR);
    if published.exists() {
        return Err(Error::Vault(format!(
            "export destination already published: {}",
            published.display()
        )));
    }

    let (scope, snapshot_head_seq, snapshot_head_hash) = compute_preview(store, project_id)?;
    let revisions: Vec<RevisionEnvelope> = store
        .all_revisions()?
        .into_iter()
        .filter(|r| scope.contains(&r.object_id))
        .collect();

    fs::create_dir_all(&dest_root)
        .map_err(|e| Error::Vault(format!("cannot create export destination: {e}")))?;
    let staging = dest_root.join(format!(".fehrest.export-staging-{}", uuid::Uuid::now_v7()));
    fs::create_dir_all(&staging)
        .map_err(|e| Error::Vault(format!("cannot create export staging dir: {e}")))?;

    let result = (|| -> Result<Vec<ExportManifestMember>> {
        let mut members = Vec::new();
        for rev in &revisions {
            let record = RecordPayload::from_json(&rev.payload).ok();
            let kind = record.as_ref().map(|r| r.kind_str());
            let file = RevisionFile {
                schema: "flake-export-revision-v1",
                object_id: &rev.object_id,
                revision_id: &rev.revision_id,
                parent_revision_id: &rev.parent_revision_id,
                recorded_seq: rev.recorded_seq,
                recorded_at: &rev.recorded_at,
                actor: &rev.actor,
                origin: &rev.origin,
                kind,
                payload_sha256: &rev.payload_sha256,
                payload_raw: &rev.payload,
            };
            let json = serde_json::to_string_pretty(&file)
                .map_err(|e| Error::Vault(format!("cannot serialize revision file: {e}")))?;
            let rel = format!(
                "revisions/{}/{}-{}.json",
                rev.object_id, rev.recorded_seq, rev.revision_id
            );
            members.push(write_member(&staging, &rel, json.as_bytes())?);

            if let Some(RecordPayload::Note(note)) = &record {
                let md_rel = format!(
                    "revisions/{}/{}-{}.md",
                    rev.object_id, rev.recorded_seq, rev.revision_id
                );
                members.push(write_member(&staging, &md_rel, note.body.as_bytes())?);
            }
        }

        let readme = readme_text(if project_id.is_some() {
            "export-project"
        } else {
            "export-full"
        });
        members.push(write_member(
            &staging,
            EXPORT_README_FILE,
            readme.as_bytes(),
        )?);

        Ok(members)
    })();

    let mut members = match result {
        Ok(m) => m,
        Err(e) => {
            let _ = fs::remove_dir_all(&staging);
            return Err(e);
        }
    };
    members.sort_by(|a, b| a.path.cmp(&b.path));

    let kind = if project_id.is_some() {
        "export-project"
    } else {
        "export-full"
    };
    let schema = "flake-export-manifest-v1";
    let integrity_root = compute_integrity_root(
        schema,
        kind,
        store.vault_id(),
        project_id,
        snapshot_head_seq,
        &members,
    );
    let manifest = ExportManifest {
        schema: schema.to_string(),
        kind: kind.to_string(),
        vault_id: store.vault_id().to_string(),
        project_id: project_id.map(str::to_string),
        snapshot_head_seq,
        snapshot_head_hash,
        created_at: crate::capture::now_rfc3339_utc(),
        record_count: scope.len(),
        revision_count: revisions.len(),
        omissions: Vec::new(),
        members,
        integrity_root,
    };
    let manifest_json = match serde_json::to_string_pretty(&manifest) {
        Ok(j) => j,
        Err(e) => {
            let _ = fs::remove_dir_all(&staging);
            return Err(Error::Vault(format!(
                "cannot serialize export manifest: {e}"
            )));
        }
    };
    if let Err(e) = fs::write(staging.join(EXPORT_MANIFEST_FILE), manifest_json) {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Vault(format!("cannot write export manifest: {e}")));
    }

    if published.exists() {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Vault(format!(
            "export destination published concurrently: {}",
            published.display()
        )));
    }
    if let Err(e) = fs::rename(&staging, &published) {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::Vault(format!(
            "cannot publish export to {}: {e}",
            published.display()
        )));
    }

    Ok(ExportReport {
        dest_root,
        manifest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::CanonicalStore;
    use std::path::PathBuf;

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-export-{}", uuid::Uuid::now_v7()))
    }
    fn cleanup(p: &Path) {
        let _ = std::fs::remove_dir_all(p);
    }

    fn new_project(store: &mut CanonicalStore, name: &str) -> String {
        let mut writer = store.writer().unwrap();
        project::create_project(&mut writer, "owner", name, None)
            .unwrap()
            .0
            .object_id
    }

    #[test]
    fn preview_reports_scope_without_writing_anything() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let p1 = new_project(&mut store, "P1");
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &p1, None, "hello").unwrap();
        }
        let dest = tmp();
        let preview = preview_export(&store, Some(&p1)).unwrap();
        assert_eq!(preview.record_count, 2); // the project itself + the note
        assert_eq!(preview.revision_count, 2); // one create revision each
        assert!(!dest.exists(), "preview must not write anything");

        cleanup(&root);
        cleanup(&dest);
    }

    /// `T03-01`: a project-scoped export must not silently drop the new
    /// `SourceCheck` kind — the exact class of omission `T02-07`'s
    /// independent verifier exists to catch, proven directly here rather
    /// than only relying on that later, separate tool.
    #[test]
    fn project_export_includes_source_checks() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let p1 = new_project(&mut store, "P1");
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let source_id = {
            let mut writer = store.writer().unwrap();
            crate::capture::import_file(&mut writer, "owner", &p1, "l", &file_path)
                .unwrap()
                .0
                .object_id
        };
        crate::source_check::check_source(&mut store, "owner", &source_id).unwrap();

        let preview = preview_export(&store, Some(&p1)).unwrap();
        // project + source + one source_check = 3 records.
        assert_eq!(preview.record_count, 3);

        let dest = tmp();
        let report = export_to_new_root(&store, Some(&p1), &dest).unwrap();
        assert_eq!(report.manifest.record_count, 3);
        let control = dest.join(".fehrest-export");
        let mut saw_source_check = false;
        for entry in walk(&control) {
            if entry.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if entry.file_name().and_then(|n| n.to_str()) == Some("export-manifest.json") {
                continue;
            }
            let text = std::fs::read_to_string(&entry).unwrap();
            let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
            if parsed.get("kind").and_then(|k| k.as_str()) == Some("source_check") {
                saw_source_check = true;
            }
        }
        assert!(
            saw_source_check,
            "expected a source_check revision file in the project export"
        );

        cleanup(&root);
        cleanup(&dest);
    }

    #[test]
    fn full_export_includes_every_project_and_full_history() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let p1 = new_project(&mut store, "P1");
        let p2 = new_project(&mut store, "P2");
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &p1, None, "note one").unwrap();
            project::create_note(&mut writer, "owner", &p2, None, "note two").unwrap();
        }
        // Two revisions for one note (create + update) to prove full
        // history, not only current state, is exported.
        let (note_outcome, _) = {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &p1, None, "v1").unwrap()
        };
        {
            let mut writer = store.writer().unwrap();
            project::update_note(
                &mut writer,
                "owner",
                &note_outcome.object_id,
                &note_outcome.revision_id,
                None,
                "v2",
            )
            .unwrap();
        }

        let dest = tmp();
        let report = export_to_new_root(&store, None, &dest).unwrap();
        assert_eq!(report.manifest.kind, "export-full");
        assert!(report.manifest.project_id.is_none());
        // 2 projects + 3 notes = 5 records; the updated note contributes 2 revisions.
        assert_eq!(report.manifest.record_count, 5);
        assert_eq!(report.manifest.revision_count, 6);

        // Independently re-enumerate and hash every published member,
        // exactly like the task's own "Verification method" requires.
        let control = dest.join(".fehrest-export");
        let mut found = 0usize;
        for entry in walk(&control) {
            let rel = entry
                .strip_prefix(&control)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            if rel == "export-manifest.json" {
                continue;
            }
            let bytes = std::fs::read(&entry).unwrap();
            let expected = report.manifest.members.iter().find(|m| m.path == rel);
            if let Some(m) = expected {
                assert_eq!(bytes.len() as u64, m.length, "length mismatch for {rel}");
                assert_eq!(
                    crate::events::hash_bytes(&bytes),
                    m.sha256,
                    "hash mismatch for {rel}"
                );
                found += 1;
            }
        }
        assert_eq!(found, report.manifest.members.len());

        cleanup(&root);
        cleanup(&dest);
    }

    #[test]
    fn project_export_excludes_other_projects_entirely() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let p1 = new_project(&mut store, "P1");
        let p2 = new_project(&mut store, "P2");
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &p1, None, "p1 content").unwrap();
            project::create_note(&mut writer, "owner", &p2, None, "p2 content").unwrap();
        }

        let dest = tmp();
        let report = export_to_new_root(&store, Some(&p1), &dest).unwrap();
        assert_eq!(report.manifest.kind, "export-project");
        assert_eq!(report.manifest.project_id.as_deref(), Some(p1.as_str()));
        assert_eq!(report.manifest.record_count, 2); // p1 project + its one note

        let control = dest.join(".fehrest-export");
        let contents = walk(&control)
            .iter()
            .filter(|p| {
                p.extension().map(|e| e == "json").unwrap_or(false)
                    && p.to_string_lossy().contains("revisions")
            })
            .map(|p| std::fs::read_to_string(p).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !contents.contains("p2 content"),
            "the other project's data must never appear in a scoped export"
        );
        assert!(
            !contents.contains(&p2),
            "the other project's own object ID must never appear in a scoped export either"
        );

        cleanup(&root);
        cleanup(&dest);
    }

    #[test]
    fn payload_raw_is_byte_identical_to_canonical_state() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let p1 = new_project(&mut store, "P1");
        let (note_outcome, _) = {
            let mut writer = store.writer().unwrap();
            project::create_note(
                &mut writer,
                "owner",
                &p1,
                Some("Unicode 日本語"),
                "body with\r\nCRLF and emoji \u{1F600}",
            )
            .unwrap()
        };
        let (_, canonical_payload) = store
            .read_current(&note_outcome.object_id)
            .unwrap()
            .unwrap();

        let dest = tmp();
        export_to_new_root(&store, Some(&p1), &dest).unwrap();
        let control = dest.join(".fehrest-export");
        let revision_file = walk(&control)
            .into_iter()
            .find(|p| {
                p.to_string_lossy().contains(&note_outcome.object_id)
                    && p.extension().map(|e| e == "json").unwrap_or(false)
            })
            .unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&revision_file).unwrap()).unwrap();
        let payload_raw = parsed["payload_raw"].as_str().unwrap();
        assert_eq!(
            payload_raw, canonical_payload,
            "payload_raw must be byte-identical to the stored canonical payload"
        );

        cleanup(&root);
        cleanup(&dest);
    }

    #[test]
    fn note_bodies_also_get_a_plain_markdown_sidecar_file() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let p1 = new_project(&mut store, "P1");
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &p1, None, "plain markdown body text")
                .unwrap();
        }
        let dest = tmp();
        export_to_new_root(&store, Some(&p1), &dest).unwrap();
        let control = dest.join(".fehrest-export");
        let md_files: Vec<_> = walk(&control)
            .into_iter()
            .filter(|p| {
                p.extension().map(|e| e == "md").unwrap_or(false)
                    && p.file_name().unwrap() != "README.md"
            })
            .collect();
        assert_eq!(md_files.len(), 1);
        assert_eq!(
            std::fs::read_to_string(&md_files[0]).unwrap(),
            "plain markdown body text"
        );

        cleanup(&root);
        cleanup(&dest);
    }

    #[test]
    fn export_never_overwrites_an_existing_destination() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let p1 = new_project(&mut store, "P1");
        let dest = tmp();
        export_to_new_root(&store, Some(&p1), &dest).unwrap();
        let err = export_to_new_root(&store, Some(&p1), &dest).unwrap_err();
        assert!(format!("{err}").contains("already published"));

        cleanup(&root);
        cleanup(&dest);
    }

    #[test]
    fn integrity_root_is_independently_recomputable() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let p1 = new_project(&mut store, "P1");
        let dest = tmp();
        let report = export_to_new_root(&store, Some(&p1), &dest).unwrap();

        let recomputed = compute_integrity_root(
            &report.manifest.schema,
            &report.manifest.kind,
            &report.manifest.vault_id,
            report.manifest.project_id.as_deref(),
            report.manifest.snapshot_head_seq,
            &report.manifest.members,
        );
        assert_eq!(recomputed, report.manifest.integrity_root);

        cleanup(&root);
        cleanup(&dest);
    }

    #[test]
    fn unknown_project_reference_is_refused_before_any_write() {
        let root = tmp();
        let store = CanonicalStore::create(&root).unwrap();
        let dest = tmp();
        let missing = uuid::Uuid::now_v7().to_string();
        let err = export_to_new_root(&store, Some(&missing), &dest).unwrap_err();
        assert!(format!("{err}").contains("invalid project reference"));
        assert!(!dest.join(".fehrest-export").exists());

        cleanup(&root);
        cleanup(&dest);
    }

    fn walk(dir: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        if !dir.exists() {
            return out;
        }
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                out.extend(walk(&path));
            } else {
                out.push(path);
            }
        }
        out
    }
}
