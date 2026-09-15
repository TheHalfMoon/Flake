//! Validate and import a portable export package (`docs/formats/portable-export-v1.md`,
//! `T02-05`) into a new or existing vault (`T02-06`).
//!
//! **Two import modes, one shared validation path.**
//!
//! - [`import_full_restore`]: imports an `export-full` (or `export-project`)
//!   package into a **brand-new, empty** canonical store. Every object's
//!   `object_id` is preserved exactly, via [`crate::canonical::CommandTarget::ImportObject`]
//!   for its first revision — the same "declared identity preserved"
//!   mechanism `T01-06`'s legacy migration already established and this
//!   task deliberately reuses rather than re-invents. §15's "A full restore
//!   validates and retains the original immutable history prefix" — see
//!   "What is and is not preserved" below for exactly what that means here.
//! - [`import_selected_merge`]: imports a package into an **already-open,
//!   possibly non-empty** destination store. Every object gets a **new**,
//!   Core-assigned `object_id` (ordinary `CreateObject`), and every internal
//!   cross-reference inside the imported payloads (`project_id`,
//!   `Action::dependency_ids`, `Relation`'s endpoints) is rewritten to point
//!   at the new destination identities before commit — §15 "it rewrites
//!   only the newly admitted references and preserves original package
//!   members... as inert imported evidence. It does not splice a foreign
//!   chain into the local transaction chain." The complete old→new mapping
//!   is returned on [`ImportReport::id_map`].
//!
//! **What is and is not preserved, and why.** `object_id` (I03's
//! load-bearing external identity: "vault identity plus record ID forms the
//! external identity") and every revision's exact payload bytes, in the
//! exact original order, are preserved (full-restore) or carried forward
//! after only the documented reference rewrites (selected-merge).
//! `revision_id`/`recorded_seq`/`recorded_at` are **not** preserved
//! byte-for-byte: `crate::canonical::CanonicalWriter::commit` always
//! Core-mints a fresh `revision_id` and `recorded_at` for every command,
//! `ImportObject` included (verified directly against `canonical.rs`'s own
//! commit implementation before writing this module) — there is no lower,
//! privileged bulk-loader that could inject historical envelope values
//! without inventing exactly the new canonical mechanism `AGENTS.md` §6/§7
//! forbids. Each replayed revision's `actor` is `"import"` and `origin` is
//! [`crate::canonical::RecordOrigin::Import`], honestly attributing *this*
//! operation to *this* operation, never impersonating the original actor.
//! This is a real, deliberate fidelity boundary, not a silent gap.
//!
//! **Idempotent retry within one import run.** Every replayed command's
//! `command_id` is the *source* revision's own `revision_id` (already a
//! UUIDv7, already globally distinct per revision) — so a process
//! interruption and retry of `import_full_restore` reconciles through
//! `CanonicalWriter::commit`'s own already-audited idempotency check
//! (`T01-03`/`T01-07`) rather than re-executing or duplicating a command
//! that already landed. `import_selected_merge` cannot offer the same
//! cross-run idempotency (each merge run mints genuinely new destination
//! identities by design, and nothing in the current record model has a
//! durable "have I already merged this external vault's content before"
//! ledger to consult — inventing one would itself be new schema this task's
//! forbidden-scope clause excludes); this is recorded as a deliberate,
//! narrower scope, not a silent gap.
//!
//! **No archive extraction — bounded directory-format members only.** There
//! is no archive/zip format anywhere in this module: the source is read as
//! an ordinary directory, and only the exact paths `export-manifest.json`
//! declares are ever opened — nothing under the source directory is walked
//! or trusted beyond what the manifest names and this module independently
//! re-hashes (S04).
//!
//! **Imported grants (updated at `T03-04`/`T03-05`).**
//! `crate::grant::ExportGrant`, `crate::disclosure::DisclosureReceipt` and
//! `crate::proposal::AgentProposal` now exist, but
//! [`import_selected_merge`] never admits any of the three:
//! `validate_self_contained` refuses a package containing one outright,
//! before any reference rewriting or commit is attempted (§16: "no
//! authority survives export/import" — local disclosure-governance state
//! found inside an imported package is refused, not silently re-admitted
//! as live authority in the destination vault). [`import_full_restore`]
//! is unaffected — a full restore is a same-owner backup, not a shareable
//! disclosure, so all three survive it unchanged like every other object
//! (`crate::export`'s own project-scoped path is what actually omits them
//! from a *shareable* package in the first place; a full/vault-wide
//! export still includes all three).

use crate::canonical::{
    CanonicalStore, CanonicalWriter, CommandInput, CommandTarget, RecordOrigin,
};
use crate::export::{compute_integrity_root, ExportManifest};
use crate::project::RecordPayload;
use crate::{Error, Result};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

const EXPORT_CONTROL_DIR: &str = ".fehrest-export";
const EXPORT_MANIFEST_FILE: &str = "export-manifest.json";

#[derive(Debug, Clone, Deserialize)]
struct ParsedRevisionFile {
    schema: String,
    object_id: String,
    revision_id: String,
    parent_revision_id: Option<String>,
    recorded_seq: i64,
    #[allow(dead_code)]
    recorded_at: String,
    #[allow(dead_code)]
    actor: String,
    #[allow(dead_code)]
    origin: String,
    #[allow(dead_code)]
    kind: Option<String>,
    payload_sha256: String,
    payload_raw: String,
}

struct ParsedPackage {
    manifest: ExportManifest,
    /// `object_id` -> its revisions, sorted oldest-first, chain-validated.
    objects: BTreeMap<String, Vec<ParsedRevisionFile>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportPreview {
    pub kind: String,
    pub source_vault_id: String,
    pub project_id: Option<String>,
    pub record_count: usize,
    pub revision_count: usize,
    /// Validation issues found in the package itself, human-readable.
    /// Nonempty means "this package cannot be imported as-is" — see
    /// [`read_and_validate_package`]'s own refusal points for what lands
    /// here versus a hard `Err` (anything that would require guessing is a
    /// hard `Err`; this list is for informational preview-time reporting of
    /// the same checks, run without side effects).
    pub conflicts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportReport {
    pub dest_root: PathBuf,
    pub mode: &'static str,
    pub imported_object_count: usize,
    pub imported_revision_count: usize,
    /// Empty for `"full-restore"` (identity preserved 1:1, so a map would
    /// be a trivial identity relation). Populated for `"selected-merge"`:
    /// source `object_id` -> new destination `object_id`.
    pub id_map: HashMap<String, String>,
}

fn sha256_hex(bytes: &[u8]) -> String {
    crate::events::hash_bytes(bytes)
}

/// Read, verify, and structurally validate a package at `<source_root>/.fehrest-export/`
/// — every step this task's own "Validate paths, lengths, digests, schema,
/// capability, identity and references before publication" names, except
/// cross-reference validation (package-scope only; see
/// `validate_self_contained`, used only by `import_selected_merge`, since a
/// full-restore package's scope is the whole store by definition and cannot
/// reference anything "outside").
fn read_and_validate_package(source_root: &Path) -> Result<ParsedPackage> {
    let control = source_root.join(EXPORT_CONTROL_DIR);
    let manifest_path = control.join(EXPORT_MANIFEST_FILE);
    let manifest_bytes = std::fs::read(&manifest_path)
        .map_err(|e| Error::Vault(format!("cannot read {}: {e}", manifest_path.display())))?;
    let manifest: ExportManifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|e| Error::Vault(format!("malformed export manifest: {e}")))?;

    if manifest.schema != "flake-export-manifest-v1" {
        return Err(Error::Vault(format!(
            "unsupported export schema {:?}; this build only reads flake-export-manifest-v1",
            manifest.schema
        )));
    }
    if manifest.kind != "export-full" && manifest.kind != "export-project" {
        return Err(Error::Vault(format!(
            "unrecognized export kind {:?}",
            manifest.kind
        )));
    }

    // Every declared member must exist at exactly its declared path, with
    // exactly its declared length and SHA-256 — before any of it is
    // trusted enough to parse.
    for member in &manifest.members {
        let path = control.join(&member.path);
        let bytes = std::fs::read(&path).map_err(|e| {
            Error::Vault(format!(
                "declared export member missing or unreadable: {} ({e})",
                member.path
            ))
        })?;
        if bytes.len() as u64 != member.length {
            return Err(Error::Vault(format!(
                "export member {} length mismatch: declared {}, actual {}",
                member.path,
                member.length,
                bytes.len()
            )));
        }
        let actual_sha256 = sha256_hex(&bytes);
        if actual_sha256 != member.sha256 {
            return Err(Error::Vault(format!(
                "export member {} digest mismatch (tampered or corrupt)",
                member.path
            )));
        }
    }

    // The manifest's own integrity root, independently recomputed — the
    // single check that would catch a manifest whose own `members` list was
    // edited (e.g. to add a member without a matching real file, or to
    // quietly drop one) even if every *listed* member above checks out.
    let recomputed = compute_integrity_root(
        &manifest.schema,
        &manifest.kind,
        &manifest.vault_id,
        manifest.project_id.as_deref(),
        manifest.snapshot_head_seq,
        &manifest.members,
    );
    if recomputed != manifest.integrity_root {
        return Err(Error::Vault(
            "export manifest integrity_root does not match its own declared members (tampered or corrupt)".into(),
        ));
    }

    // Parse every revision file the manifest declared (identified by path
    // prefix, not by walking the directory for anything matching a
    // pattern — only manifest-declared members are ever read at all).
    let mut objects: BTreeMap<String, Vec<ParsedRevisionFile>> = BTreeMap::new();
    let mut seen_ids: std::collections::HashSet<(String, String)> =
        std::collections::HashSet::new();
    for member in &manifest.members {
        if !member.path.starts_with("revisions/") || !member.path.ends_with(".json") {
            continue;
        }
        let bytes = std::fs::read(control.join(&member.path))
            .map_err(|e| Error::Vault(format!("cannot read {}: {e}", member.path)))?;
        let file: ParsedRevisionFile = serde_json::from_slice(&bytes)
            .map_err(|e| Error::Vault(format!("malformed revision file {}: {e}", member.path)))?;
        if file.schema != "flake-export-revision-v1" {
            return Err(Error::Vault(format!(
                "unsupported revision schema in {}: {:?}",
                member.path, file.schema
            )));
        }
        if sha256_hex(file.payload_raw.as_bytes()) != file.payload_sha256 {
            return Err(Error::Vault(format!(
                "revision file {} payload_raw does not match its own declared payload_sha256",
                member.path
            )));
        }
        // Must parse as a recognized typed record — an unparseable payload
        // is a hard refusal, not a silent skip (F07).
        RecordPayload::from_json(&file.payload_raw).map_err(|e| {
            Error::Vault(format!(
                "revision file {} payload does not parse as a typed record: {e}",
                member.path
            ))
        })?;
        if !seen_ids.insert((file.object_id.clone(), file.revision_id.clone())) {
            return Err(Error::Vault(format!(
                "colliding immutable revision id within the package: {} / {}",
                file.object_id, file.revision_id
            )));
        }
        objects
            .entry(file.object_id.clone())
            .or_default()
            .push(file);
    }

    for (object_id, revisions) in objects.iter_mut() {
        revisions.sort_by_key(|r| r.recorded_seq);
        for (i, rev) in revisions.iter().enumerate() {
            let expected_parent = if i == 0 {
                None
            } else {
                Some(revisions[i - 1].revision_id.clone())
            };
            if rev.parent_revision_id != expected_parent {
                return Err(Error::Vault(format!(
                    "revision chain is inconsistent for object {object_id}: revision {} has parent {:?}, expected {:?}",
                    rev.revision_id, rev.parent_revision_id, expected_parent
                )));
            }
        }
    }

    if objects.len() != manifest.record_count {
        return Err(Error::Vault(format!(
            "manifest declares {} records but {} were found in the package",
            manifest.record_count,
            objects.len()
        )));
    }
    let total_revisions: usize = objects.values().map(|v| v.len()).sum();
    if total_revisions != manifest.revision_count {
        return Err(Error::Vault(format!(
            "manifest declares {} revisions but {} were found in the package",
            manifest.revision_count, total_revisions
        )));
    }

    Ok(ParsedPackage { manifest, objects })
}

/// Every `Note`/`Action`/`Decision`/`Source`'s `project_id`, and every
/// `Relation`'s `from_object_id`/`to_object_id`, must name an object also
/// present in this same package — a self-contained package never
/// references anything "outside" it. Only meaningful (and only called) for
/// `import_selected_merge`; a full-restore package's scope is the entire
/// store, so nothing can be "outside" it by definition.
fn validate_self_contained(package: &ParsedPackage) -> Result<()> {
    for (object_id, revisions) in &package.objects {
        let latest = revisions
            .last()
            .expect("every object has at least one revision");
        let record = RecordPayload::from_json(&latest.payload_raw)?;
        // §16: "shareable project packages omit local-only locators, active
        // grants and excluded sensitive fields." `export.rs`'s own
        // project-scoped `project_scope_object_ids` never emits an
        // `ExportGrant`, `DisclosureReceipt` or `AgentProposal` (all three
        // are local disclosure *governance* state — who was authorized,
        // what was actually sent, what an external agent proposed back —
        // not project content), so a legitimate package never contains any
        // of them. This is the defense-in-depth backstop against a
        // hand-crafted or corrupted package trying to smuggle local grant
        // authority, another vault's disclosure history, or another
        // vault's inbound agent negotiation state into a different vault
        // ("no authority survives export/import").
        if matches!(
            record,
            RecordPayload::ExportGrant(_)
                | RecordPayload::DisclosureReceipt(_)
                | RecordPayload::AgentProposal(_)
        ) {
            return Err(Error::Vault(format!(
                "object {object_id} is a {}, which a selected-project merge never admits (§16: local-only disclosure governance state never survives export/import)",
                record.kind_str()
            )));
        }
        let refs: Vec<String> = match &record {
            RecordPayload::Note(n) => vec![n.project_id.clone()],
            RecordPayload::Action(a) => {
                let mut r = vec![a.project_id.clone()];
                r.extend(a.dependency_ids.clone());
                r
            }
            RecordPayload::Decision(d) => vec![d.project_id.clone()],
            RecordPayload::Source(s) => vec![s.project_id.clone()],
            RecordPayload::Relation(rel) => {
                vec![
                    rel.project_id.clone(),
                    rel.from_object_id.clone(),
                    rel.to_object_id.clone(),
                ]
            }
            RecordPayload::Project(_) => vec![],
            RecordPayload::SourceCheck(c) => vec![c.project_id.clone(), c.source_id.clone()],
            RecordPayload::ReviewCheckpoint(c) => vec![c.project_id.clone()],
            // Unreachable: refused above before this match runs. Kept
            // exhaustive rather than a wildcard so a future new field on
            // either kind cannot silently stop being checked here.
            RecordPayload::ExportGrant(g) => vec![g.project_id.clone()],
            RecordPayload::DisclosureReceipt(r) => vec![r.project_id.clone()],
            RecordPayload::AgentProposal(p) => vec![p.project_id.clone()],
        };
        for r in refs {
            if !package.objects.contains_key(&r) {
                return Err(Error::Vault(format!(
                    "object {object_id} references {r}, which is not part of this package (not self-contained)"
                )));
            }
        }
    }
    Ok(())
}

/// Preview a package without writing anything — "sensitive history and
/// scope shown before writing" (§12), applied to import the same way
/// `export::preview_export` applies it to export.
pub fn preview_import(source_root: &Path) -> Result<ImportPreview> {
    let package = read_and_validate_package(source_root)?;
    let mut conflicts = Vec::new();
    if package.manifest.kind == "export-project" {
        if let Err(e) = validate_self_contained(&package) {
            conflicts.push(format!("{e}"));
        }
    }
    Ok(ImportPreview {
        kind: package.manifest.kind.clone(),
        source_vault_id: package.manifest.vault_id.clone(),
        project_id: package.manifest.project_id.clone(),
        record_count: package.objects.len(),
        revision_count: package.objects.values().map(|v| v.len()).sum(),
        conflicts,
    })
}

/// Import a package into a **brand-new, empty** canonical store at
/// `dest_root`. Refuses if `dest_root` already has a published `.fehrest`
/// control directory, and refuses (writing nothing) if the package fails
/// any validation check — "all malformed/ambiguous cases refuse complete
/// success" applied to this mode's own definition of complete (every
/// declared object, in full).
pub fn import_full_restore(source_root: &Path, dest_root: &Path) -> Result<ImportReport> {
    let package = read_and_validate_package(source_root)?;

    let control_marker = dest_root.join(".fehrest");
    if control_marker.exists() {
        return Err(Error::Vault(format!(
            "import destination already exists: {}",
            control_marker.display()
        )));
    }

    let mut store = CanonicalStore::create(dest_root)?;
    let mut writer = store.writer()?;
    let mut imported_objects = 0usize;
    let mut imported_revisions = 0usize;
    for (object_id, revisions) in &package.objects {
        let mut prev_dest_revision_id: Option<String> = None;
        for (i, rev) in revisions.iter().enumerate() {
            let target = if i == 0 {
                CommandTarget::ImportObject {
                    object_id: object_id.clone(),
                    payload: rev.payload_raw.clone(),
                }
            } else {
                CommandTarget::UpdateObject {
                    object_id: object_id.clone(),
                    expected_revision_id: prev_dest_revision_id
                        .clone()
                        .expect("a non-first revision always has a predecessor"),
                    payload: rev.payload_raw.clone(),
                }
            };
            let outcome = writer.commit(CommandInput {
                command_id: rev.revision_id.clone(),
                actor: "import".to_string(),
                origin: RecordOrigin::Import,
                target,
            })?;
            prev_dest_revision_id = Some(outcome.revision_id);
            imported_revisions += 1;
        }
        imported_objects += 1;
    }

    Ok(ImportReport {
        dest_root: dest_root.to_path_buf(),
        mode: "full-restore",
        imported_object_count: imported_objects,
        imported_revision_count: imported_revisions,
        id_map: HashMap::new(),
    })
}

fn remap(id: &str, id_map: &HashMap<String, String>) -> Result<String> {
    id_map.get(id).cloned().ok_or_else(|| {
        Error::Vault(format!(
            "dangling reference during import: {id} was not (yet) imported"
        ))
    })
}

/// Rewrite every internal cross-reference in `record` to the destination
/// identities in `id_map`. `clear_action_dependencies`: `Action` only —
/// forward references to sibling actions cannot be resolved until every
/// action in the package has its own new destination ID, so the first pass
/// clears them (see `import_selected_merge`'s own two-subpass handling).
fn rewrite_references(
    record: &mut RecordPayload,
    id_map: &HashMap<String, String>,
    clear_action_dependencies: bool,
) -> Result<()> {
    match record {
        RecordPayload::Project(_) => {}
        RecordPayload::Note(n) => n.project_id = remap(&n.project_id, id_map)?,
        RecordPayload::Action(a) => {
            a.project_id = remap(&a.project_id, id_map)?;
            if clear_action_dependencies {
                a.dependency_ids = Vec::new();
            } else {
                let mut rewritten = Vec::with_capacity(a.dependency_ids.len());
                for dep in &a.dependency_ids {
                    rewritten.push(remap(dep, id_map)?);
                }
                a.dependency_ids = rewritten;
            }
        }
        RecordPayload::Decision(d) => d.project_id = remap(&d.project_id, id_map)?,
        RecordPayload::Source(s) => s.project_id = remap(&s.project_id, id_map)?,
        RecordPayload::Relation(r) => {
            r.project_id = remap(&r.project_id, id_map)?;
            r.from_object_id = remap(&r.from_object_id, id_map)?;
            r.to_object_id = remap(&r.to_object_id, id_map)?;
            // from_revision_id/to_revision_id are re-pinned by the caller
            // (needs a fresh store read of the already-imported endpoint's
            // current revision), not here.
        }
        RecordPayload::SourceCheck(c) => {
            c.project_id = remap(&c.project_id, id_map)?;
            c.source_id = remap(&c.source_id, id_map)?;
            // checked_revision_id is re-pinned by the caller, exactly like
            // Relation's from_revision_id/to_revision_id above — the
            // original revision_id it names cannot survive replay (T02-06's
            // own already-documented boundary: revision_id is always
            // freshly minted at the destination).
        }
        RecordPayload::ReviewCheckpoint(c) => c.project_id = remap(&c.project_id, id_map)?,
        // Unreachable in practice: `validate_self_contained` refuses any
        // package containing any of these three kinds before this
        // function is ever called on one. Exhaustiveness still requires
        // real arms.
        RecordPayload::ExportGrant(g) => g.project_id = remap(&g.project_id, id_map)?,
        RecordPayload::DisclosureReceipt(r) => r.project_id = remap(&r.project_id, id_map)?,
        RecordPayload::AgentProposal(p) => p.project_id = remap(&p.project_id, id_map)?,
    }
    Ok(())
}

/// Commit one object's full revision chain into `writer` under a freshly
/// Core-assigned destination identity, applying `rewrite` to every
/// revision's parsed payload first. Returns `(new_object_id,
/// final_revision_id)`.
fn import_chain_with_new_identity(
    writer: &mut CanonicalWriter<'_>,
    revisions: &[ParsedRevisionFile],
    id_map: &HashMap<String, String>,
    clear_action_dependencies: bool,
) -> Result<(String, String)> {
    let mut new_object_id: Option<String> = None;
    let mut prev_dest_revision_id: Option<String> = None;
    for (i, rev) in revisions.iter().enumerate() {
        let mut record = RecordPayload::from_json(&rev.payload_raw)?;
        rewrite_references(&mut record, id_map, clear_action_dependencies)?;
        let payload = record.to_json()?;
        let target = if i == 0 {
            CommandTarget::CreateObject { payload }
        } else {
            CommandTarget::UpdateObject {
                object_id: new_object_id.clone().unwrap(),
                expected_revision_id: prev_dest_revision_id.clone().unwrap(),
                payload,
            }
        };
        let outcome = writer.commit(CommandInput {
            command_id: uuid::Uuid::now_v7().to_string(),
            actor: "import".to_string(),
            origin: RecordOrigin::Import,
            target,
        })?;
        if i == 0 {
            new_object_id = Some(outcome.object_id.clone());
        }
        prev_dest_revision_id = Some(outcome.revision_id);
    }
    Ok((new_object_id.unwrap(), prev_dest_revision_id.unwrap()))
}

/// Import a package into an already-open, possibly non-empty destination
/// store. Every object gets a new, Core-assigned identity; every internal
/// cross-reference is rewritten to point at the new identities. See module
/// docs for exactly what this mode does and does not guarantee.
pub fn import_selected_merge(
    store: &mut CanonicalStore,
    source_root: &Path,
) -> Result<ImportReport> {
    let package = read_and_validate_package(source_root)?;
    validate_self_contained(&package)?;

    let mut id_map: HashMap<String, String> = HashMap::new();
    let mut imported_revisions = 0usize;

    let latest_kind = |revisions: &[ParsedRevisionFile]| -> Result<RecordPayload> {
        RecordPayload::from_json(&revisions.last().unwrap().payload_raw)
    };

    // Pass order matters: each pass only rewrites references to objects a
    // *prior* pass has already assigned a destination identity to.
    // Project -> Source -> Note/Decision -> ReviewCheckpoint (only needs
    // project_id, like Note/Decision, so it rides the same simple pass) ->
    // SourceCheck (its own dedicated pass: needs Source's destination
    // identity plus a re-pinned checked_revision_id) -> Action (two
    // subpasses of its own, for sibling-action forward references) ->
    // Relation (references anything, so it must go last).
    let mut writer = store.writer()?;

    for pass_kind in ["project", "source", "note", "decision", "review_checkpoint"] {
        for (object_id, revisions) in &package.objects {
            if latest_kind(revisions)?.kind_str() != pass_kind {
                continue;
            }
            let (new_id, _) =
                import_chain_with_new_identity(&mut writer, revisions, &id_map, false)?;
            id_map.insert(object_id.clone(), new_id);
            imported_revisions += revisions.len();
        }
    }

    // Source checks: reference a Source, which already has a destination
    // identity from the pass above. `checked_revision_id` is re-pinned to
    // the destination Source's *current* revision at import time — the
    // exact same reasoning `rewrite_references` already documents for
    // Relation's endpoint revisions: the original revision_id cannot
    // survive replay, since the destination always mints a fresh one.
    for (object_id, revisions) in &package.objects {
        if latest_kind(revisions)?.kind_str() != "source_check" {
            continue;
        }
        let source_new_id = match latest_kind(revisions)? {
            RecordPayload::SourceCheck(c) => remap(&c.source_id, &id_map)?,
            _ => unreachable!(),
        };
        let (source_current_rev, _) = store_current(&writer, &source_new_id)?;

        let mut prev_dest_revision_id: Option<String> = None;
        let mut new_object_id: Option<String> = None;
        for (i, rev) in revisions.iter().enumerate() {
            let mut record = RecordPayload::from_json(&rev.payload_raw)?;
            rewrite_references(&mut record, &id_map, false)?;
            if let RecordPayload::SourceCheck(c) = &mut record {
                c.checked_revision_id = source_current_rev.clone();
            }
            let payload = record.to_json()?;
            let target = if i == 0 {
                CommandTarget::CreateObject { payload }
            } else {
                CommandTarget::UpdateObject {
                    object_id: new_object_id.clone().unwrap(),
                    expected_revision_id: prev_dest_revision_id.clone().unwrap(),
                    payload,
                }
            };
            let outcome = writer.commit(CommandInput {
                command_id: uuid::Uuid::now_v7().to_string(),
                actor: "import".to_string(),
                origin: RecordOrigin::Import,
                target,
            })?;
            if i == 0 {
                new_object_id = Some(outcome.object_id.clone());
            }
            prev_dest_revision_id = Some(outcome.revision_id);
            imported_revisions += 1;
        }
        id_map.insert(object_id.clone(), new_object_id.unwrap());
    }

    // Actions, subpass 1: create every action with dependency_ids cleared,
    // so every action in the package gets a destination identity before any
    // of them tries to reference a sibling.
    let mut actions_with_deps: Vec<(String, Vec<String>)> = Vec::new();
    for (object_id, revisions) in &package.objects {
        if latest_kind(revisions)?.kind_str() != "action" {
            continue;
        }
        let final_deps = match latest_kind(revisions)? {
            RecordPayload::Action(a) => a.dependency_ids.clone(),
            _ => unreachable!(),
        };
        let (new_id, _) = import_chain_with_new_identity(&mut writer, revisions, &id_map, true)?;
        id_map.insert(object_id.clone(), new_id.clone());
        imported_revisions += revisions.len();
        if !final_deps.is_empty() {
            actions_with_deps.push((new_id, final_deps));
        }
    }
    // Actions, subpass 2: restore each action's dependency_ids, rewritten
    // through the now-complete id_map, as one additional revision.
    for (new_action_id, original_deps) in actions_with_deps {
        let mut rewritten_deps = Vec::with_capacity(original_deps.len());
        for dep in &original_deps {
            rewritten_deps.push(remap(dep, &id_map)?);
        }
        let (current_revision_id, _) = store_current(&writer, &new_action_id)?;
        let mut action = match store_current_record(&writer, &new_action_id)? {
            RecordPayload::Action(a) => a,
            _ => unreachable!(),
        };
        action.dependency_ids = rewritten_deps;
        writer.commit(CommandInput {
            command_id: uuid::Uuid::now_v7().to_string(),
            actor: "import".to_string(),
            origin: RecordOrigin::Import,
            target: CommandTarget::UpdateObject {
                object_id: new_action_id,
                expected_revision_id: current_revision_id,
                payload: RecordPayload::Action(action).to_json()?,
            },
        })?;
        imported_revisions += 1;
    }

    // Relations last: every endpoint kind they could reference already has
    // a destination identity by now.
    for (object_id, revisions) in &package.objects {
        if latest_kind(revisions)?.kind_str() != "relation" {
            continue;
        }
        // Endpoints are re-pinned to the destination's *current* revision
        // of each endpoint at import time (the original pinned revision_id
        // cannot survive replay — see module docs).
        let latest = latest_kind(revisions)?;
        let (from_new, to_new) = match &latest {
            RecordPayload::Relation(r) => (
                remap(&r.from_object_id, &id_map)?,
                remap(&r.to_object_id, &id_map)?,
            ),
            _ => unreachable!(),
        };
        let (from_rev, _) = store_current(&writer, &from_new)?;
        let (to_rev, _) = store_current(&writer, &to_new)?;

        let mut prev_dest_revision_id: Option<String> = None;
        let mut new_object_id: Option<String> = None;
        for (i, rev) in revisions.iter().enumerate() {
            let mut record = RecordPayload::from_json(&rev.payload_raw)?;
            rewrite_references(&mut record, &id_map, false)?;
            if let RecordPayload::Relation(r) = &mut record {
                r.from_revision_id = from_rev.clone();
                r.to_revision_id = to_rev.clone();
            }
            let payload = record.to_json()?;
            let target = if i == 0 {
                CommandTarget::CreateObject { payload }
            } else {
                CommandTarget::UpdateObject {
                    object_id: new_object_id.clone().unwrap(),
                    expected_revision_id: prev_dest_revision_id.clone().unwrap(),
                    payload,
                }
            };
            let outcome = writer.commit(CommandInput {
                command_id: uuid::Uuid::now_v7().to_string(),
                actor: "import".to_string(),
                origin: RecordOrigin::Import,
                target,
            })?;
            if i == 0 {
                new_object_id = Some(outcome.object_id.clone());
            }
            prev_dest_revision_id = Some(outcome.revision_id);
            imported_revisions += 1;
        }
        id_map.insert(object_id.clone(), new_object_id.unwrap());
    }

    Ok(ImportReport {
        dest_root: store.root().to_path_buf(),
        mode: "selected-merge",
        imported_object_count: id_map.len(),
        imported_revision_count: imported_revisions,
        id_map,
    })
}

fn store_current(writer: &CanonicalWriter<'_>, object_id: &str) -> Result<(String, String)> {
    writer
        .store()
        .read_current(object_id)?
        .ok_or_else(|| Error::Vault(format!("expected object {object_id} to already exist")))
}

fn store_current_record(writer: &CanonicalWriter<'_>, object_id: &str) -> Result<RecordPayload> {
    let (_, payload) = store_current(writer, object_id)?;
    RecordPayload::from_json(&payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::CanonicalStore;
    use crate::project;
    use std::path::PathBuf;

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-import-{}", uuid::Uuid::now_v7()))
    }
    fn cleanup(p: &Path) {
        let _ = std::fs::remove_dir_all(p);
    }

    fn build_source_store() -> (PathBuf, CanonicalStore, String, String) {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = {
            let mut writer = store.writer().unwrap();
            project::create_project(&mut writer, "owner", "P", None)
                .unwrap()
                .0
                .object_id
        };
        let dep_id = {
            let mut writer = store.writer().unwrap();
            project::create_action(&mut writer, "owner", &project_id, "dep", None, &[])
                .unwrap()
                .0
                .object_id
        };
        let action_id = {
            let mut writer = store.writer().unwrap();
            project::create_action(
                &mut writer,
                "owner",
                &project_id,
                "depends on dep",
                None,
                std::slice::from_ref(&dep_id),
            )
            .unwrap()
            .0
            .object_id
        };
        {
            let mut writer = store.writer().unwrap();
            project::create_note(
                &mut writer,
                "owner",
                &project_id,
                Some("N"),
                "note body with\r\nCRLF",
            )
            .unwrap();
        }
        (root, store, project_id, action_id)
    }

    #[test]
    fn full_restore_preserves_object_ids_and_all_payload_bytes() {
        let (source_root, store, project_id, action_id) = build_source_store();
        let export_dest = tmp();
        crate::export::export_to_new_root(&store, None, &export_dest).unwrap();

        let import_dest = tmp();
        let report = import_full_restore(&export_dest, &import_dest).unwrap();
        assert_eq!(report.mode, "full-restore");
        assert!(report.id_map.is_empty());

        let dest_store = CanonicalStore::open(&import_dest).unwrap();
        // Same object_id, same current payload (byte-identical), in the new store.
        let (_, orig_payload) = store.read_current(&project_id).unwrap().unwrap();
        let (_, imported_payload) = dest_store.read_current(&project_id).unwrap().unwrap();
        assert_eq!(orig_payload, imported_payload);

        let (_, orig_action) = store.read_current(&action_id).unwrap().unwrap();
        let (_, imported_action) = dest_store.read_current(&action_id).unwrap().unwrap();
        assert_eq!(orig_action, imported_action, "dependency_ids referencing an unchanged object_id survive full-restore byte-identically");

        cleanup(&source_root);
        cleanup(&export_dest);
        cleanup(&import_dest);
    }

    #[test]
    fn full_restore_refuses_an_already_existing_destination() {
        let (source_root, store, _, _) = build_source_store();
        let export_dest = tmp();
        crate::export::export_to_new_root(&store, None, &export_dest).unwrap();
        let import_dest = tmp();
        import_full_restore(&export_dest, &import_dest).unwrap();
        let err = import_full_restore(&export_dest, &import_dest).unwrap_err();
        assert!(format!("{err}").contains("already exists"));

        cleanup(&source_root);
        cleanup(&export_dest);
        cleanup(&import_dest);
    }

    #[test]
    fn full_restore_is_all_or_nothing_on_a_corrupt_member() {
        let (source_root, store, _, _) = build_source_store();
        let export_dest = tmp();
        crate::export::export_to_new_root(&store, None, &export_dest).unwrap();

        // Tamper with one revision file after export.
        let revisions_dir = export_dest.join(".fehrest-export").join("revisions");
        let mut tampered = None;
        for entry in std::fs::read_dir(&revisions_dir).unwrap() {
            let object_dir = entry.unwrap().path();
            for f in std::fs::read_dir(&object_dir).unwrap() {
                let f = f.unwrap().path();
                if f.extension().map(|e| e == "json").unwrap_or(false) {
                    tampered = Some(f);
                    break;
                }
            }
            if tampered.is_some() {
                break;
            }
        }
        let tampered = tampered.unwrap();
        let mut text = std::fs::read_to_string(&tampered).unwrap();
        // Same length as "owner" so this specifically exercises the
        // content-digest check, not the (also-valid) length check.
        text = text.replace("\"owner\"", "\"oWNER\"");
        std::fs::write(&tampered, text).unwrap();

        let import_dest = tmp();
        let err = import_full_restore(&export_dest, &import_dest).unwrap_err();
        assert!(format!("{err}").contains("digest mismatch"), "got: {err}");
        assert!(
            !import_dest.join(".fehrest").exists(),
            "a rejected import must leave no destination vault at all"
        );

        cleanup(&source_root);
        cleanup(&export_dest);
        cleanup(&import_dest);
    }

    #[test]
    fn selected_merge_assigns_new_ids_and_rewrites_references() {
        let (source_root, store, project_id, action_id) = build_source_store();
        let export_dest = tmp();
        crate::export::export_to_new_root(&store, Some(&project_id), &export_dest).unwrap();

        let dest_root = tmp();
        let mut dest_store = CanonicalStore::create(&dest_root).unwrap();
        // The destination already has unrelated content, proving this is a
        // true merge, not a fresh-root restore.
        {
            let mut writer = dest_store.writer().unwrap();
            project::create_project(&mut writer, "owner", "Pre-existing", None).unwrap();
        }

        let report = import_selected_merge(&mut dest_store, &export_dest).unwrap();
        assert_eq!(report.mode, "selected-merge");
        assert_ne!(
            report.id_map[&project_id], project_id,
            "merge must mint a new identity, never reuse the source's"
        );
        assert_ne!(report.id_map[&action_id], action_id);

        let new_project_id = &report.id_map[&project_id];
        let new_action_id = &report.id_map[&action_id];
        let (_, new_action_payload) = dest_store.read_current(new_action_id).unwrap().unwrap();
        let new_action = project::RecordPayload::from_json(&new_action_payload).unwrap();
        let new_action = new_action.as_action().unwrap();
        assert_eq!(
            &new_action.project_id, new_project_id,
            "project_id must be rewritten to the new destination project"
        );
        assert_eq!(new_action.dependency_ids.len(), 1);
        assert_ne!(
            &new_action.dependency_ids[0], &report.id_map[&project_id],
            "sanity: dependency isn't accidentally the project id"
        );
        // The dependency itself must also have been rewritten to its own new identity.
        let orig_dep_id = {
            let (_, orig_action_payload) = store.read_current(&action_id).unwrap().unwrap();
            project::RecordPayload::from_json(&orig_action_payload)
                .unwrap()
                .as_action()
                .unwrap()
                .dependency_ids[0]
                .clone()
        };
        assert_eq!(&new_action.dependency_ids[0], &report.id_map[&orig_dep_id]);

        // Pre-existing destination content is untouched.
        let all = dest_store.list_current_objects().unwrap();
        assert!(
            all.len() > report.imported_object_count,
            "pre-existing destination objects must remain, in addition to the merged ones"
        );

        cleanup(&source_root);
        cleanup(&export_dest);
        cleanup(&dest_root);
    }

    #[test]
    fn merge_relation_endpoints_are_rewritten_and_repinned() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = {
            let mut writer = store.writer().unwrap();
            project::create_project(&mut writer, "owner", "P", None)
                .unwrap()
                .0
                .object_id
        };
        let decision_id = {
            let mut writer = store.writer().unwrap();
            project::create_decision(
                &mut writer,
                "owner",
                &project_id,
                "k",
                "s",
                None,
                project::DecisionBasis::UserJudgment,
                project::DecisionVerification::Unreviewed,
                None,
                None,
            )
            .unwrap()
            .0
            .object_id
        };
        let source_id = {
            let mut writer = store.writer().unwrap();
            crate::capture::create_manual_reference(
                &mut writer,
                "owner",
                &project_id,
                "l",
                None,
                None,
                None,
            )
            .unwrap()
            .0
            .object_id
        };
        {
            let mut writer = store.writer().unwrap();
            crate::relation::create_relation(
                &mut writer,
                "owner",
                &project_id,
                crate::relation::RelationType::Supports,
                &decision_id,
                &source_id,
                None,
            )
            .unwrap();
        }

        let export_dest = tmp();
        crate::export::export_to_new_root(&store, Some(&project_id), &export_dest).unwrap();

        let dest_root = tmp();
        let mut dest_store = CanonicalStore::create(&dest_root).unwrap();
        let report = import_selected_merge(&mut dest_store, &export_dest).unwrap();

        let new_decision_id = &report.id_map[&decision_id];
        let new_source_id = &report.id_map[&source_id];
        let relations =
            crate::relation::list_relations_for_object(&dest_store, new_decision_id).unwrap();
        assert_eq!(relations.len(), 1);
        let (_, rel) = &relations[0];
        assert_eq!(&rel.from_object_id, new_decision_id);
        assert_eq!(&rel.to_object_id, new_source_id);
        let (dest_decision_rev, _) = dest_store.read_current(new_decision_id).unwrap().unwrap();
        let (dest_source_rev, _) = dest_store.read_current(new_source_id).unwrap().unwrap();
        assert_eq!(rel.from_revision_id, dest_decision_rev);
        assert_eq!(rel.to_revision_id, dest_source_rev);

        cleanup(&root);
        cleanup(&export_dest);
        cleanup(&dest_root);
    }

    /// `T03-01`: a merge-imported `SourceCheck`'s `source_id` must follow
    /// the same id_map rewrite every other cross-reference in this module
    /// gets, and its `checked_revision_id` — which cannot survive replay,
    /// since the destination always mints a fresh `revision_id` — must be
    /// re-pinned to the destination `Source`'s actual current revision,
    /// exactly like `Relation`'s own endpoint re-pinning above.
    #[test]
    fn merge_source_check_source_id_is_rewritten_and_checked_revision_repinned() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = {
            let mut writer = store.writer().unwrap();
            project::create_project(&mut writer, "owner", "P", None)
                .unwrap()
                .0
                .object_id
        };
        let file_path = root.join("f.txt");
        std::fs::write(&file_path, b"hello").unwrap();
        let source_id = {
            let mut writer = store.writer().unwrap();
            crate::capture::import_file(&mut writer, "owner", &project_id, "l", &file_path)
                .unwrap()
                .0
                .object_id
        };
        crate::source_check::check_source(&mut store, "owner", &source_id).unwrap();

        let export_dest = tmp();
        crate::export::export_to_new_root(&store, Some(&project_id), &export_dest).unwrap();

        let dest_root = tmp();
        let mut dest_store = CanonicalStore::create(&dest_root).unwrap();
        let report = import_selected_merge(&mut dest_store, &export_dest).unwrap();

        let new_source_id = &report.id_map[&source_id];
        let checks = crate::source_check::list_project_source_checks(
            &dest_store,
            &report.id_map[&project_id],
        )
        .unwrap();
        assert_eq!(checks.len(), 1);
        let (_, check) = &checks[0];
        assert_eq!(
            &check.source_id, new_source_id,
            "source_id must be rewritten to the new destination source, never the original"
        );
        let (dest_source_rev, _) = dest_store.read_current(new_source_id).unwrap().unwrap();
        assert_eq!(
            check.checked_revision_id, dest_source_rev,
            "checked_revision_id must be re-pinned to the destination's actual current revision"
        );

        cleanup(&root);
        cleanup(&export_dest);
        cleanup(&dest_root);
    }

    #[test]
    fn merge_review_checkpoint_is_carried_through_with_its_project_id_rewritten() {
        // Regression for a real gap found during self-review (T03-03): the
        // first draft of `import_selected_merge`'s explicit per-kind pass
        // list omitted `review_checkpoint` entirely, so a merge-imported
        // package would silently drop every checkpoint object — exactly
        // the class of omission `T02-07`'s independent verifier exists to
        // catch (I06/I10: full declared state is readable).
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = {
            let mut writer = store.writer().unwrap();
            project::create_project(&mut writer, "owner", "P", None)
                .unwrap()
                .0
                .object_id
        };
        crate::checkpoint::mark_reviewed_through(&mut store, "owner", &project_id, None, None)
            .unwrap();

        let export_dest = tmp();
        crate::export::export_to_new_root(&store, Some(&project_id), &export_dest).unwrap();

        let dest_root = tmp();
        let mut dest_store = CanonicalStore::create(&dest_root).unwrap();
        let report = import_selected_merge(&mut dest_store, &export_dest).unwrap();

        let new_project_id = &report.id_map[&project_id];
        let (_, _, checkpoint) = crate::checkpoint::current_checkpoint(&dest_store, new_project_id)
            .unwrap()
            .expect("the checkpoint must have been carried through the merge, not dropped");
        assert_eq!(&checkpoint.project_id, new_project_id);

        cleanup(&root);
        cleanup(&export_dest);
        cleanup(&dest_root);
    }

    #[test]
    fn selected_merge_refuses_a_package_containing_a_grant() {
        // §16: "shareable project packages omit local-only locators, active
        // grants and excluded sensitive fields" — `export.rs`'s own
        // project-scoped export never emits an `ExportGrant`, so this test
        // exercises the defense-in-depth backstop directly: point
        // `import_selected_merge` at a *full* (vault-wide) export, which
        // does include local grants, and confirm it is refused rather than
        // silently admitted or silently dropped.
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = {
            let mut writer = store.writer().unwrap();
            project::create_project(&mut writer, "owner", "P", None)
                .unwrap()
                .0
                .object_id
        };
        {
            let mut writer = store.writer().unwrap();
            crate::grant::issue_grant(
                &mut writer,
                "owner",
                &project_id,
                &[],
                None,
                &[],
                1024,
                3600,
            )
            .unwrap();
        }

        let export_dest = tmp();
        crate::export::export_to_new_root(&store, None, &export_dest).unwrap();

        let dest_root = tmp();
        let mut dest_store = CanonicalStore::create(&dest_root).unwrap();
        let err = import_selected_merge(&mut dest_store, &export_dest).unwrap_err();
        assert!(format!("{err}").contains("export_grant"));

        cleanup(&root);
        cleanup(&export_dest);
        cleanup(&dest_root);
    }

    #[test]
    fn full_restore_preserves_grants_unlike_selected_merge() {
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = {
            let mut writer = store.writer().unwrap();
            project::create_project(&mut writer, "owner", "P", None)
                .unwrap()
                .0
                .object_id
        };
        let grant_id = {
            let mut writer = store.writer().unwrap();
            crate::grant::issue_grant(
                &mut writer,
                "owner",
                &project_id,
                &[],
                None,
                &[],
                1024,
                3600,
            )
            .unwrap()
            .0
            .object_id
        };

        let export_dest = tmp();
        crate::export::export_to_new_root(&store, None, &export_dest).unwrap();

        let import_dest = tmp();
        import_full_restore(&export_dest, &import_dest).unwrap();
        let dest_store = CanonicalStore::open(&import_dest).unwrap();
        let restored = crate::grant::current_grant(&dest_store, &grant_id).unwrap();
        assert_eq!(restored.project_id, project_id, "full-restore is a same-owner backup, not a shareable disclosure — grant identity and scope survive unchanged");

        cleanup(&root);
        cleanup(&export_dest);
        cleanup(&import_dest);
    }

    #[test]
    fn selected_merge_refuses_a_package_containing_an_agent_proposal() {
        // §16 extended to `T03-05`'s own new kind: an `AgentProposal` is
        // local disclosure-negotiation state (bound to a receipt in *this*
        // vault), never shareable project content.
        let root = tmp();
        let mut store = CanonicalStore::create(&root).unwrap();
        let project_id = {
            let mut writer = store.writer().unwrap();
            project::create_project(&mut writer, "owner", "P", None)
                .unwrap()
                .0
                .object_id
        };
        let note_id = {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, None, "n")
                .unwrap()
                .0
                .object_id
        };
        let note_rev = store.read_current(&note_id).unwrap().unwrap().0;
        let grant_id = {
            let mut writer = store.writer().unwrap();
            crate::grant::issue_grant(
                &mut writer,
                "owner",
                &project_id,
                &[],
                None,
                &[],
                65536,
                3600,
            )
            .unwrap()
            .0
            .object_id
        };
        crate::disclosure::compile_disclosure_package(&mut store, &grant_id, "req-1", "agent")
            .unwrap();
        let receipt_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                RecordPayload::from_json(&payload)
                    .ok()?
                    .as_disclosure_receipt()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        let json = format!(
            r#"{{"receipt_id":"{receipt_id}","operations":[{{"kind":"note_edit","note_id":"{note_id}","expected_revision_id":"{note_rev}","body":"b"}}]}}"#
        );
        crate::proposal::admit_proposal(&mut store, "owner", &project_id, json.as_bytes()).unwrap();

        let export_dest = tmp();
        crate::export::export_to_new_root(&store, None, &export_dest).unwrap();

        let dest_root = tmp();
        let mut dest_store = CanonicalStore::create(&dest_root).unwrap();
        let err = import_selected_merge(&mut dest_store, &export_dest).unwrap_err();
        // `validate_self_contained` refuses on the *first* governance-state
        // object it encounters (`BTreeMap<object_id, _>` order, not kind
        // order) — a proposal cannot exist without the grant/receipt it
        // references, so this vault necessarily contains all three, and
        // the refusal may legitimately name any one of them. What matters
        // is that the whole batch is refused, not silently admitted or
        // silently thinned down to "just the parts we recognize".
        let msg = format!("{err}");
        assert!(
            msg.contains("export_grant")
                || msg.contains("disclosure_receipt")
                || msg.contains("agent_proposal"),
            "expected refusal naming a local governance-state kind, got: {msg}"
        );

        cleanup(&root);
        cleanup(&export_dest);
        cleanup(&dest_root);
    }

    #[test]
    fn preview_reports_scope_without_writing_anything() {
        let (source_root, store, project_id, _) = build_source_store();
        let export_dest = tmp();
        crate::export::export_to_new_root(&store, Some(&project_id), &export_dest).unwrap();

        let preview = preview_import(&export_dest).unwrap();
        assert_eq!(preview.kind, "export-project");
        assert!(preview.conflicts.is_empty());
        assert_eq!(preview.record_count, 4); // project + dep action + dependent action + note

        cleanup(&source_root);
        cleanup(&export_dest);
    }

    #[test]
    fn tampered_manifest_integrity_root_is_refused() {
        let (source_root, store, _, _) = build_source_store();
        let export_dest = tmp();
        crate::export::export_to_new_root(&store, None, &export_dest).unwrap();

        let manifest_path = export_dest
            .join(".fehrest-export")
            .join("export-manifest.json");
        let mut manifest: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&manifest_path).unwrap()).unwrap();
        // `snapshot_head_seq` feeds `integrity_root`'s own hash input
        // directly, unlike `record_count`/`revision_count` (which are
        // caught by a separate, earlier consistency check) — this
        // specifically exercises the integrity-root verification itself.
        manifest["snapshot_head_seq"] = serde_json::json!(9999);
        std::fs::write(
            &manifest_path,
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();

        let err = preview_import(&export_dest).unwrap_err();
        assert!(format!("{err}").contains("integrity_root"), "got: {err}");

        cleanup(&source_root);
        cleanup(&export_dest);
    }
}
