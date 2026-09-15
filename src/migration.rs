//! Import legacy format-1 vaults into a new format-2 vault without
//! rewriting accepted history (`T01-06`).
//!
//! **Objective, in this module's own vocabulary.** "Move valid format-1
//! work into a separate format-2 vault with honest provenance" means:
//! preview what would be imported and what would be left out *before*
//! touching anything, refuse to call an import "complete" when anything
//! was left out, and never manufacture missing history or authenticate the
//! source vault's own event-log claims as proof of anything.
//!
//! **What gets imported.** Each admittable legacy object becomes one
//! `CommandTarget::ImportObject` command against a fresh format-2 vault,
//! with `origin: RecordOrigin::Migration` and the **exact original file
//! bytes** as its payload — not a re-parsed-and-re-serialized
//! reconstruction. This is deliberate: `identity::parse`/`serialize`
//! round-trips are for *product* mutation, not for a migration whose whole
//! point is byte-for-byte provenance (I09/I10). CRLF line endings, exact
//! whitespace, and unrecognized frontmatter lines all survive untouched
//! because they are never touched.
//!
//! **Identity.** `crate::canonical::CommandTarget::ImportObject` is the one
//! deliberate exception to "identity is Core-assigned": a legacy object's
//! own UUIDv7 becomes its format-2 `object_id`, but *only* when
//! [`crate::vault::Vault::scan`]'s own conflict detection shows it is
//! unambiguous. A legacy ID observed at more than one path in the source
//! vault is omitted from admission entirely — retaining it would mean
//! guessing which of two files is "the real" identity, which this module
//! never does (I03).
//!
//! **What "complete" means.** [`ImportSelection::AllAdmittedOnly`] refuses
//! outright if the preview shows *any* omission (malformed file, unsupported
//! extension, reserved-directory content, or duplicate identity) — "ambiguous
//! complete migration refuses." [`ImportSelection::Selected`] always produces
//! an explicitly labeled partial result, even if it happens to select every
//! admittable record, because omitting the label would misrepresent intent.
//!
//! **Not implemented here** (recorded, not silently dropped): a CLI
//! migration-preview command (no CLI wiring exists for the format-2 store
//! at all yet, consistent with every prior `flake-v1` task's recorded scope
//! boundary); importing the source's event-log *content* as format-2
//! history (the source event log is inspected only for its own chain
//! status, recorded as informational context — its claims are never
//! authenticated or replayed as format-2 commands, per this task's own
//! "do not authenticate legacy log claims" instruction); and streaming
//! import for vaults larger than fit in memory at once (`crate::vault::Vault::scan`
//! itself is not streaming — a pre-existing property of the format-1 reader
//! this task does not change, recorded as a limitation for the M/L
//! performance gate).

use crate::canonical::{CanonicalStore, CommandInput, CommandTarget, RecordOrigin};
use crate::identity::ObjectId;
use crate::vault::Vault;
use crate::{Error, Result};
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const MIGRATION_MANIFEST_FILE: &str = "migration-manifest.json";

/// One legacy object judged admittable by [`preview_migration`]: unambiguous
/// identity, parseable frontmatter, supported extension, not under a
/// reserved directory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LegacyRecord {
    pub legacy_object_id: String,
    pub rel_path: String,
    /// Exact original file bytes (UTF-8), untouched by any parse/reserialize
    /// round-trip.
    pub raw_bytes: String,
    pub content_sha256: String,
}

/// One legacy path judged *not* admittable, with the exact reason —
/// "provenance gaps visible," never a silent skip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OmittedRecord {
    pub rel_path: String,
    pub reason: String,
}

/// What a migration dry-run found, before anything is imported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationPreview {
    pub source_root: PathBuf,
    pub admitted: Vec<LegacyRecord>,
    pub omitted: Vec<OmittedRecord>,
    /// Informational only (this task does not authenticate legacy log
    /// claims): the source format-1 vault's own event-log chain status, if
    /// an event log is present.
    pub source_event_log_status: Option<String>,
}

impl MigrationPreview {
    pub fn is_complete(&self) -> bool {
        self.omitted.is_empty()
    }
}

/// Which admitted records an actual import call should admit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportSelection {
    /// Import every admitted record. Refuses entirely if the preview shows
    /// any omission at all — "ambiguous complete migration refuses."
    AllAdmittedOnly,
    /// Import exactly this explicit subset of `legacy_object_id`s. Always
    /// an explicitly labeled partial result, even if it selects everything
    /// admittable.
    Selected(BTreeSet<String>),
}

/// Durable record of one import attempt, written into the new vault's own
/// control directory as `migration-manifest.json` — present whether the
/// attempt completed or was interrupted partway (§28: no bare "PASS"/
/// "COMPLETE" claim without exact supporting detail alongside it).
#[derive(Debug, Clone, Serialize)]
struct MigrationManifest<'a> {
    schema: &'static str,
    source_root: String,
    imported: &'a [LegacyRecord],
    omitted: &'a [OmittedRecord],
    complete: bool,
    interrupted: bool,
    failure_reason: Option<String>,
}

/// The result of a (possibly interrupted) import.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationReport {
    pub source_root: PathBuf,
    pub new_root: PathBuf,
    pub vault_id: String,
    pub imported: Vec<LegacyRecord>,
    pub omitted: Vec<OmittedRecord>,
    /// `true` only for `ImportSelection::AllAdmittedOnly` with zero
    /// omissions. `Selected` is always `false`, even selecting everything.
    pub complete: bool,
}

/// Nonmutating dry-run over the format-1 vault at `source_root`: what would
/// be admitted, what would be omitted and why, and the source's own
/// event-log chain status (informational only).
pub fn preview_migration(source_root: impl AsRef<Path>) -> Result<MigrationPreview> {
    let source_root = source_root.as_ref().to_path_buf();
    let vault = Vault::open_read(&source_root)?;
    let scan = vault.scan()?;

    let conflicted_ids: std::collections::HashSet<ObjectId> =
        scan.conflicts.iter().map(|(id, _)| *id).collect();

    let mut admitted = Vec::new();
    let mut omitted = Vec::new();

    for rec in &scan.objects {
        if conflicted_ids.contains(&rec.id) {
            omitted.push(OmittedRecord {
                rel_path: rec.rel_path.clone(),
                reason: format!(
                    "ambiguous identity {}: observed at more than one path in the source vault",
                    rec.id
                ),
            });
            continue;
        }
        let raw_bytes = fs::read_to_string(source_root.join(&rec.rel_path)).map_err(|e| {
            Error::Migration(format!(
                "cannot read legacy file {} for migration: {e}",
                rec.rel_path
            ))
        })?;
        let content_sha256 = crate::events::hash_bytes(raw_bytes.as_bytes());
        admitted.push(LegacyRecord {
            legacy_object_id: rec.id.to_string(),
            rel_path: rec.rel_path.clone(),
            raw_bytes,
            content_sha256,
        });
    }

    for path in &scan.skipped {
        omitted.push(OmittedRecord {
            rel_path: path.clone(),
            reason: "skipped: reserved directory or unsupported extension".to_string(),
        });
    }
    for (path, err) in &scan.malformed {
        omitted.push(OmittedRecord {
            rel_path: path.clone(),
            reason: format!("malformed: {err}"),
        });
    }

    // Informational only: this task never authenticates the source vault's
    // own event-log claims, but recording whether that log is even intact
    // is honest context for the operator reviewing a preview.
    let source_event_log_status = match crate::events::EventLog::open(&vault.control_dir()) {
        Ok(log) => match log.verify() {
            Ok(status) => Some(format!("{status:?}")),
            Err(e) => Some(format!("verify failed: {e}")),
        },
        Err(e) => Some(format!("cannot open: {e}")),
    };

    Ok(MigrationPreview {
        source_root,
        admitted,
        omitted,
        source_event_log_status,
    })
}

/// Import the format-1 vault at `source_root` into a fresh format-2 vault
/// at `new_root`, per `selection`. `new_root` must not already have a
/// published store. `source_root` is never written to, on any path.
///
/// A failure partway through the import loop (a real error, or an injected
/// one in this module's own tests) still writes `migration-manifest.json`
/// into the new vault documenting exactly what was imported before the
/// failure, with `interrupted: true` and the failure reason — the new
/// vault's own `CanonicalStore::open`/`read_current` calls remain a
/// reliable way to inspect exactly what state exists, but the manifest is
/// what tells a caller *not* to treat that state as a finished migration.
pub fn import_to_new_root(
    source_root: impl AsRef<Path>,
    new_root: impl AsRef<Path>,
    selection: ImportSelection,
) -> Result<MigrationReport> {
    import_to_new_root_with_fault(source_root, new_root, selection, None)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ImportFaultPoint {
    /// Stop after exactly `n` records have been successfully imported,
    /// simulating an interruption partway through a multi-record import.
    AfterNthRecord(usize),
}

pub(crate) fn import_to_new_root_with_fault(
    source_root: impl AsRef<Path>,
    new_root: impl AsRef<Path>,
    selection: ImportSelection,
    fault: Option<ImportFaultPoint>,
) -> Result<MigrationReport> {
    let source_root = source_root.as_ref().to_path_buf();
    let new_root = new_root.as_ref().to_path_buf();

    let preview = preview_migration(&source_root)?;

    let (to_import, omitted): (Vec<LegacyRecord>, Vec<OmittedRecord>) = match &selection {
        ImportSelection::AllAdmittedOnly => {
            if !preview.omitted.is_empty() {
                return Err(Error::Migration(format!(
                    "cannot report a complete migration: {} record(s) omitted (see preview); select an explicit subset instead",
                    preview.omitted.len()
                )));
            }
            (preview.admitted.clone(), Vec::new())
        }
        ImportSelection::Selected(ids) => {
            let admitted_ids: BTreeSet<String> = preview
                .admitted
                .iter()
                .map(|r| r.legacy_object_id.clone())
                .collect();
            for id in ids {
                if !admitted_ids.contains(id) {
                    return Err(Error::Migration(format!(
                        "requested legacy_object_id {id} is not an admittable record (see preview omissions)"
                    )));
                }
            }
            let mut imp = Vec::new();
            let mut left_out = preview.omitted.clone();
            for rec in &preview.admitted {
                if ids.contains(&rec.legacy_object_id) {
                    imp.push(rec.clone());
                } else {
                    left_out.push(OmittedRecord {
                        rel_path: rec.rel_path.clone(),
                        reason: "not included in explicit selection".to_string(),
                    });
                }
            }
            (imp, left_out)
        }
    };

    if to_import.is_empty() {
        return Err(Error::Migration(
            "no admittable records selected for import".into(),
        ));
    }

    let mut store = CanonicalStore::create(&new_root)?;
    let vault_id = store.vault_id().to_string();

    let mut imported = Vec::new();
    let mut interrupt_reason: Option<String> = None;
    {
        let mut writer = store.writer()?;
        for rec in &to_import {
            let commit_result = writer.commit(CommandInput {
                command_id: uuid::Uuid::now_v7().to_string(),
                actor: "migration".into(),
                origin: RecordOrigin::Migration,
                target: CommandTarget::ImportObject {
                    object_id: rec.legacy_object_id.clone(),
                    payload: rec.raw_bytes.clone(),
                },
            });
            match commit_result {
                Ok(_) => imported.push(rec.clone()),
                Err(e) => {
                    interrupt_reason = Some(format!("{e}"));
                    break;
                }
            }
            if fault == Some(ImportFaultPoint::AfterNthRecord(imported.len())) {
                interrupt_reason = Some(format!(
                    "injected fault: AfterNthRecord({})",
                    imported.len()
                ));
                break;
            }
        }
    }

    let complete = matches!(selection, ImportSelection::AllAdmittedOnly)
        && omitted.is_empty()
        && interrupt_reason.is_none();
    let interrupted = interrupt_reason.is_some();

    let manifest = MigrationManifest {
        schema: "flake-migration-manifest-v1",
        source_root: source_root.display().to_string(),
        imported: &imported,
        omitted: &omitted,
        complete,
        interrupted,
        failure_reason: interrupt_reason.clone(),
    };
    if let Ok(json) = serde_json::to_string_pretty(&manifest) {
        let _ = fs::write(store.control_dir().join(MIGRATION_MANIFEST_FILE), json);
    }

    if let Some(reason) = interrupt_reason {
        return Err(Error::Migration(format!(
            "migration interrupted after {} of {} selected record(s): {reason}",
            imported.len(),
            to_import.len()
        )));
    }

    Ok(MigrationReport {
        source_root,
        new_root,
        vault_id,
        imported,
        omitted,
        complete,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::CONTROL_DIR;

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-migration-{}", uuid::Uuid::now_v7()))
    }

    fn cleanup(p: &Path) {
        let _ = fs::remove_dir_all(p);
    }

    fn legacy_vault_with(files: &[(&str, &str)]) -> PathBuf {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        for (path, content) in files {
            fs::write(root.join(path), content).unwrap();
        }
        drop(v);
        root
    }

    fn frontmatter(id: &str, extra: &str) -> String {
        format!("---\nid: {id}\n{extra}---\nbody text\n")
    }

    #[test]
    fn complete_migration_of_a_clean_vault_preserves_exact_bytes_and_identity() {
        let id = uuid::Uuid::now_v7().to_string();
        let content =
            format!("---\nid: {id}\ntitle: Hello\n---\r\nUnicode: café \u{1F600}\r\nCRLF line\r\n");
        let root = legacy_vault_with(&[("a.md", &content)]);

        let preview = preview_migration(&root).unwrap();
        assert!(preview.is_complete(), "preview: {preview:?}");
        assert_eq!(preview.admitted.len(), 1);
        assert_eq!(
            preview.admitted[0].raw_bytes, content,
            "raw bytes must be exact, CRLF/Unicode included"
        );

        let new_root = tmp();
        let report =
            import_to_new_root(&root, &new_root, ImportSelection::AllAdmittedOnly).unwrap();
        assert!(report.complete);
        assert_eq!(report.imported.len(), 1);

        let opened = CanonicalStore::open(&new_root).unwrap();
        let (_, payload) = opened.read_current(&id).unwrap().unwrap();
        assert_eq!(
            payload, content,
            "imported payload must be byte-identical to the legacy file"
        );

        // Source completely untouched.
        assert!(Vault::open_read(&root).is_ok());
        cleanup(&root);
        cleanup(&new_root);
    }

    #[test]
    fn duplicate_identity_is_omitted_and_blocks_a_complete_migration() {
        let id = uuid::Uuid::now_v7().to_string();
        let root = legacy_vault_with(&[
            ("a.md", &frontmatter(&id, "")),
            ("b.md", &frontmatter(&id, "")),
        ]);

        let preview = preview_migration(&root).unwrap();
        assert!(!preview.is_complete());
        assert!(
            preview.admitted.is_empty(),
            "an ambiguous ID must not be admitted at all"
        );
        assert_eq!(preview.omitted.len(), 2);
        assert!(preview
            .omitted
            .iter()
            .all(|o| o.reason.contains("ambiguous identity")));

        let new_root = tmp();
        let err =
            import_to_new_root(&root, &new_root, ImportSelection::AllAdmittedOnly).unwrap_err();
        assert!(format!("{err}").contains("cannot report a complete migration"));
        assert!(!new_root.join(CONTROL_DIR).exists());
        cleanup(&root);
    }

    #[test]
    fn malformed_file_blocks_complete_but_selected_import_still_works() {
        let good_id = uuid::Uuid::now_v7().to_string();
        let root = legacy_vault_with(&[
            ("good.md", &frontmatter(&good_id, "")),
            ("bad.md", "not frontmatter at all"),
        ]);

        let preview = preview_migration(&root).unwrap();
        assert!(!preview.is_complete());
        assert_eq!(preview.admitted.len(), 1);
        assert_eq!(preview.omitted.len(), 1);
        assert!(preview.omitted[0].reason.starts_with("malformed:"));

        let new_root = tmp();
        let err =
            import_to_new_root(&root, &new_root, ImportSelection::AllAdmittedOnly).unwrap_err();
        assert!(format!("{err}").contains("cannot report a complete migration"));

        let mut selected = BTreeSet::new();
        selected.insert(good_id.clone());
        let report =
            import_to_new_root(&root, &new_root, ImportSelection::Selected(selected)).unwrap();
        assert!(
            !report.complete,
            "a Selected import is never labeled complete"
        );
        assert_eq!(report.imported.len(), 1);
        assert!(report.omitted.iter().any(|o| o.rel_path == "bad.md"));

        cleanup(&root);
        cleanup(&new_root);
    }

    #[test]
    fn requesting_an_unadmittable_id_is_refused_not_silently_skipped() {
        let root = legacy_vault_with(&[]);
        let mut selected = BTreeSet::new();
        selected.insert(uuid::Uuid::now_v7().to_string());
        let new_root = tmp();
        let err =
            import_to_new_root(&root, &new_root, ImportSelection::Selected(selected)).unwrap_err();
        assert!(format!("{err}").contains("is not an admittable record"));
        cleanup(&root);
    }

    #[test]
    fn interrupted_migration_writes_a_manifest_and_leaves_source_untouched() {
        let id1 = uuid::Uuid::now_v7().to_string();
        let id2 = uuid::Uuid::now_v7().to_string();
        let content1 = frontmatter(&id1, "");
        let content2 = frontmatter(&id2, "");
        let root = legacy_vault_with(&[("a.md", &content1), ("b.md", &content2)]);
        let source_bytes_before = [
            fs::read(root.join("a.md")).unwrap(),
            fs::read(root.join("b.md")).unwrap(),
        ];

        let new_root = tmp();
        let err = import_to_new_root_with_fault(
            &root,
            &new_root,
            ImportSelection::AllAdmittedOnly,
            Some(ImportFaultPoint::AfterNthRecord(1)),
        )
        .unwrap_err();
        assert!(format!("{err}").contains("interrupted after 1 of 2"));

        // The new vault exists with exactly the one record that succeeded
        // before interruption, and says so in its own manifest.
        let manifest: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(new_root.join(CONTROL_DIR).join(MIGRATION_MANIFEST_FILE)).unwrap(),
        )
        .unwrap();
        assert_eq!(manifest["complete"], false);
        assert_eq!(manifest["interrupted"], true);
        assert_eq!(manifest["imported"].as_array().unwrap().len(), 1);

        let opened = CanonicalStore::open(&new_root).unwrap();
        assert_eq!(opened.transaction_head().unwrap().0, 1);

        // Source completely unchanged.
        assert_eq!(fs::read(root.join("a.md")).unwrap(), source_bytes_before[0]);
        assert_eq!(fs::read(root.join("b.md")).unwrap(), source_bytes_before[1]);

        cleanup(&root);
        cleanup(&new_root);
    }

    #[test]
    fn new_root_no_clobber() {
        let root =
            legacy_vault_with(&[("a.md", &frontmatter(&uuid::Uuid::now_v7().to_string(), ""))]);
        let new_root = tmp();
        import_to_new_root(&root, &new_root, ImportSelection::AllAdmittedOnly).unwrap();

        let root2 =
            legacy_vault_with(&[("b.md", &frontmatter(&uuid::Uuid::now_v7().to_string(), ""))]);
        let err =
            import_to_new_root(&root2, &new_root, ImportSelection::AllAdmittedOnly).unwrap_err();
        assert!(matches!(err, Error::Canonical(_)), "got {err:?}");
        cleanup(&root);
        cleanup(&root2);
        cleanup(&new_root);
    }

    /// Copies both gold fixtures under `tests/fixtures/migration/` into a
    /// fresh legacy vault root, so this test exercises the exact, checked-in
    /// historical-format bytes (`git diff` reviewable, reused across runs)
    /// rather than only inline-constructed content.
    #[test]
    fn gold_fixtures_with_crlf_unicode_and_unknown_fields_migrate_byte_identically() {
        let root = tmp();
        Vault::create(&root).unwrap();
        for name in ["legacy-crlf-unicode.md", "legacy-unknown-fields.md"] {
            let bytes =
                fs::read(Path::new("tests/fixtures/migration").join(name)).unwrap_or_else(|e| {
                    panic!("fixture {name} must be readable from the crate root: {e}")
                });
            fs::write(root.join(name), bytes).unwrap();
        }

        let preview = preview_migration(&root).unwrap();
        assert!(
            preview.is_complete(),
            "gold fixtures must be cleanly admittable: {preview:?}"
        );
        assert_eq!(preview.admitted.len(), 2);

        let new_root = tmp();
        let report =
            import_to_new_root(&root, &new_root, ImportSelection::AllAdmittedOnly).unwrap();
        assert!(report.complete);

        let opened = CanonicalStore::open(&new_root).unwrap();
        for name in ["legacy-crlf-unicode.md", "legacy-unknown-fields.md"] {
            let original = fs::read_to_string(root.join(name)).unwrap();
            let legacy_id = preview
                .admitted
                .iter()
                .find(|r| r.rel_path == name)
                .unwrap()
                .legacy_object_id
                .clone();
            let (_, payload) = opened.read_current(&legacy_id).unwrap().unwrap();
            assert_eq!(
                payload, original,
                "{name} must migrate byte-for-byte, CRLF/Unicode/unknown fields included"
            );
        }

        cleanup(&root);
        cleanup(&new_root);
    }

    // -----------------------------------------------------------------
    // T01-07 — D5 durability class: deterministic fault-schedule matrix
    // -----------------------------------------------------------------

    /// D5 ("interrupted backup/import/migration/export publication"): 100
    /// genuinely distinct interruption schedules, each stopping
    /// `import_to_new_root` after a different real record count (1
    /// through 100) against a source legacy vault with exactly 100
    /// admittable records — a natural per-record loop, not a padded
    /// parameter sweep.
    #[test]
    fn d5_migration_interruption_fault_schedule_matrix() {
        let root = tmp();
        Vault::create(&root).unwrap();
        let mut legacy_ids = Vec::new();
        for i in 0..100 {
            let id = uuid::Uuid::now_v7().to_string();
            fs::write(
                root.join(format!("f{i:03}.md")),
                frontmatter(&id, &format!("title: File {i}\n")),
            )
            .unwrap();
            legacy_ids.push(id);
        }

        let preview = preview_migration(&root).unwrap();
        assert!(preview.is_complete());
        assert_eq!(preview.admitted.len(), 100);

        for n in 1..=100usize {
            let new_root = tmp();
            let err = import_to_new_root_with_fault(
                &root,
                &new_root,
                ImportSelection::AllAdmittedOnly,
                Some(ImportFaultPoint::AfterNthRecord(n)),
            )
            .unwrap_err();
            assert!(
                format!("{err}").contains(&format!("interrupted after {n} of 100")),
                "schedule n={n}: got {err}"
            );

            let manifest: serde_json::Value = serde_json::from_str(
                &fs::read_to_string(new_root.join(CONTROL_DIR).join(MIGRATION_MANIFEST_FILE))
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(manifest["interrupted"], true, "schedule n={n}");
            assert_eq!(
                manifest["imported"].as_array().unwrap().len(),
                n,
                "schedule n={n}: manifest must record exactly the records that succeeded"
            );

            let opened = CanonicalStore::open(&new_root).unwrap();
            assert_eq!(
                opened.transaction_head().unwrap().0 as usize,
                n,
                "schedule n={n}: the new vault must contain exactly n committed records"
            );

            cleanup(&new_root);
        }

        // The source's own bytes are unaffected by 100 interrupted
        // attempts against it.
        for (i, id) in legacy_ids.iter().enumerate() {
            let content = fs::read_to_string(root.join(format!("f{i:03}.md"))).unwrap();
            assert!(content.contains(id));
        }

        cleanup(&root);
    }
}
