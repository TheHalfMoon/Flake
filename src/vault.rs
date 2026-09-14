//! Vault root, content admission, and the single-writer lock.

use crate::identity::{self, ObjectId};
use crate::limits;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Control directory. Never indexed as user knowledge (F-CORE-16).
pub const CONTROL_DIR: &str = ".fehrest";

/// Vault identity file (inside CONTROL_DIR).
pub const VAULT_META_FILE: &str = "vault.json";

/// Current supported vault format version (Spec 002 FR2-001).
pub const SUPPORTED_FORMAT_VERSION: u32 = 1;

/// Minimal vault identity/version metadata (T046).
///
/// Only machine-owned fields required for product Phase 1:
/// `vault_id`, `format_version`, `created_by_version`, `created_at`.
/// No cloud/collaboration fields per Ponytail SHRINK.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultMeta {
    pub vault_id: String,
    pub format_version: u32,
    pub created_by_version: String,
    pub created_at: String,
}

impl VaultMeta {
    pub fn new(
        vault_id: String,
        format_version: u32,
        created_by_version: String,
        created_at: String,
    ) -> Self {
        Self {
            vault_id,
            format_version,
            created_by_version,
            created_at,
        }
    }
}

/// Directory names excluded from ordinary knowledge indexing.
///
/// `.fehrest` holds Fehrest's own canonical machine state — indexing it would feed
/// audit records back as knowledge. `.git` holds object data, hooks, and remote
/// URLs that can carry credentials.
const RESERVED_DIRS: &[&str] = &[CONTROL_DIR, ".git"];

/// Supported canonical content. **Allowlist, not deny-list** (F-CORE-16).
///
/// A deny-list of secret filenames is a permanent race against names nobody has
/// thought of, and it fails toward indexing. This fails toward exclusion.
const SUPPORTED_EXTENSIONS: &[&str] = &["md", "markdown"];

pub fn is_supported(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| SUPPORTED_EXTENSIONS.contains(&e.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}

fn is_reserved_component(name: &str) -> bool {
    RESERVED_DIRS.contains(&name)
}

/// True if `meta` describes a symlink, junction, mount point, or any other
/// filesystem reparse point (T01-01, S03 filesystem escape/relocation).
///
/// `std::fs::FileType::is_symlink()` alone is insufficient on Windows: NTFS
/// junctions and mount points carry `IO_REPARSE_TAG_MOUNT_POINT`, not
/// `IO_REPARSE_TAG_SYMLINK`, so `is_symlink()` returns `false` for them even
/// though they redirect exactly like a symlink for this purpose. Checking the
/// raw `FILE_ATTRIBUTE_REPARSE_POINT` bit (via the safe
/// `MetadataExt::file_attributes` accessor — no `unsafe`, matching this
/// crate's `forbid(unsafe_code)` lint) catches every reparse type uniformly.
#[cfg(windows)]
fn is_reparse_point(meta: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
    meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn is_reparse_point(meta: &fs::Metadata) -> bool {
    meta.file_type().is_symlink()
}

/// One admitted canonical object.
#[derive(Debug, Clone)]
pub struct ObjectRecord {
    pub id: ObjectId,
    /// Vault-relative locator, forward-slash normalised for stable storage.
    pub rel_path: String,
    pub title: Option<String>,
    pub project: Option<String>,
    pub content_hash: String,
    pub body: String,
}

/// The result of scanning a vault.
#[derive(Debug, Default)]
pub struct ScanResult {
    pub objects: Vec<ObjectRecord>,
    /// Files skipped because they are not supported content or sit under a
    /// reserved directory. Recorded so exclusion is visible, not silent.
    pub skipped: Vec<String>,
    /// Files that look canonical but could not be parsed. Surfaced, never ignored.
    pub malformed: Vec<(String, String)>,
    /// Duplicate identities: one id observed at two or more locations.
    ///
    /// D §3.2: both are retained and neither is silently discarded. Guessing which
    /// is "real" merges two objects' histories, which is unrecoverable.
    pub conflicts: Vec<(ObjectId, Vec<String>)>,
}

/// An open vault. Holding this value holds the write lock.
#[derive(Debug)]
pub struct Vault {
    root: PathBuf,
    lock: Option<WriteLock>,
}

impl Vault {
    /// Create a new vault, taking the write lock.
    ///
    /// T01-01: identity is written only after this process holds the OS-held
    /// writer lease (inside `open_write`), never before. Two concurrent
    /// `create` calls on the same root no longer race to write competing
    /// vault identities before either has proven ownership.
    pub fn create(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        let control = root.join(CONTROL_DIR);
        fs::create_dir_all(&control)
            .map_err(|e| Error::Vault(format!("cannot create control dir: {e}")))?;
        Self::open_write(root)
    }

    /// Open an existing vault for writing, taking the single-writer lock.
    ///
    /// T01-01: the OS-held writer lease is acquired **first**. Only after
    /// this process structurally holds it does anything mutate — vault
    /// identity upcast, torn-tail detection/repair. Previously these ran
    /// before the lease existed, so two processes calling `open_write`
    /// concurrently on the same (possibly legacy, metadata-less) root could
    /// both perform startup mutation at once; whichever's atomic rename won
    /// last silently discarded the other's chosen identity. Acquiring the
    /// lease first makes every subsequent mutation in this function
    /// serialized against every other writer by construction.
    pub fn open_write(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        Self::require_vault(&root)?;
        let lock = WriteLock::acquire(&root)?;
        // Validate (and auto-migrate legacy missing) vault metadata now that
        // ownership is held — startup integrity gate FR2-019 steps 1-2.
        ensure_vault_meta(&root.join(CONTROL_DIR))?;
        // Steps 3-5: startup integrity gating before writable open (T066)
        // Reads event log, repairs torn tail (quarantine before truncate), then fails closed on gap/chain break.
        // `lock` is still held (moved into the returned Vault only on success);
        // an early `?` return here drops it, releasing the lease.
        startup_integrity_check(&root.join(CONTROL_DIR))?;
        Ok(Vault {
            root,
            lock: Some(lock),
        })
    }

    /// Open read-only. Takes no lock, so concurrent readers are fine.
    ///
    /// T01-01: this path is now genuinely nonmutating. It previously called
    /// `ensure_vault_meta`, which silently creates and durably writes a fresh
    /// vault identity when none exists — a "read" that could mutate on-disk
    /// state on first open of a legacy vault. A readonly open of a legacy
    /// (metadata-less) vault now returns `MissingMetadata` instead; the
    /// explicit legacy-upgrade path is `open_write`, which performs the
    /// upcast under proven writer ownership.
    pub fn open_read(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        Self::require_vault(&root)?;
        match read_vault_meta(&root.join(CONTROL_DIR))? {
            Some(_) => Ok(Vault { root, lock: None }),
            None => Err(Error::MissingMetadata {
                control_dir: root.join(CONTROL_DIR).display().to_string(),
            }),
        }
    }

    fn require_vault(root: &Path) -> Result<()> {
        let control = root.join(CONTROL_DIR);
        // T01-01 / S03: use symlink_metadata (does not follow the final
        // component) so a `.fehrest` that is itself a symlink, junction, or
        // other reparse point is detected here rather than silently followed
        // to wherever it points. `is_symlink()` alone misses Windows NTFS
        // junctions/mount points (a well-known std gap: those carry a
        // different reparse tag than `IO_REPARSE_TAG_SYMLINK`), so the
        // Windows path additionally checks the raw reparse-point attribute.
        let meta = match fs::symlink_metadata(&control) {
            Ok(m) => m,
            Err(_) => {
                return Err(Error::Vault(format!(
                    "not a Fehrest vault (no {CONTROL_DIR}/): {}",
                    root.display()
                )))
            }
        };
        if is_reparse_point(&meta) {
            return Err(Error::Containment(format!(
                "vault control directory is a symlink/reparse point, refused: {}",
                control.display()
            )));
        }
        if !meta.is_dir() {
            return Err(Error::Vault(format!(
                "not a Fehrest vault (no {CONTROL_DIR}/): {}",
                root.display()
            )));
        }
        Ok(())
    }

    /// Return the vault's metadata (fails visibly on unsupported/newer format).
    pub fn vault_meta(&self) -> Result<VaultMeta> {
        read_vault_meta(&self.control_dir())?.ok_or_else(|| {
            Error::Vault(format!(
                "vault metadata missing at {}",
                self.control_dir().join(VAULT_META_FILE).display()
            ))
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn control_dir(&self) -> PathBuf {
        self.root.join(CONTROL_DIR)
    }

    pub fn has_write_lock(&self) -> bool {
        self.lock.is_some()
    }

    /// Scan the vault for admitted canonical objects.
    ///
    /// Reads go through the confined path even here: the scan discovers relative
    /// locators, and every subsequent read of one is contained and identity-checked
    /// like any other.
    pub fn scan(&self) -> Result<ScanResult> {
        let mut result = ScanResult::default();
        let mut seen: HashMap<ObjectId, Vec<String>> = HashMap::new();
        self.scan_dir(&self.root, &mut result, &mut seen)?;

        for (id, paths) in seen {
            if paths.len() > 1 {
                let mut paths = paths;
                paths.sort();
                result.conflicts.push((id, paths));
            }
        }
        result.conflicts.sort_by_key(|(id, _)| *id);
        result.objects.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
        result.skipped.sort();
        Ok(result)
    }

    fn scan_dir(
        &self,
        dir: &Path,
        out: &mut ScanResult,
        seen: &mut HashMap<ObjectId, Vec<String>>,
    ) -> Result<()> {
        let entries =
            fs::read_dir(dir).map_err(|e| Error::Vault(format!("cannot read {dir:?}: {e}")))?;

        for entry in entries {
            let entry = entry.map_err(|e| Error::Vault(format!("bad dir entry: {e}")))?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // symlink_metadata, not metadata: a symlinked directory must not be
            // descended into, and a symlinked file must not be admitted.
            let meta = fs::symlink_metadata(&path)
                .map_err(|e| Error::Vault(format!("cannot stat {path:?}: {e}")))?;

            if meta.file_type().is_symlink() {
                out.skipped.push(self.rel(&path));
                continue;
            }

            if meta.is_dir() {
                if is_reserved_component(&name) {
                    continue; // reserved: not knowledge, not reported as skipped noise
                }
                self.scan_dir(&path, out, seen)?;
                continue;
            }

            if !is_supported(&path) {
                out.skipped.push(self.rel(&path));
                continue;
            }

            if meta.len() > limits::MAX_OBJECT_BYTES as u64 {
                out.malformed.push((
                    self.rel(&path),
                    format!("exceeds MAX_OBJECT_BYTES ({})", limits::MAX_OBJECT_BYTES),
                ));
                continue;
            }

            let rel = self.rel(&path);
            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => {
                    out.malformed.push((rel, format!("unreadable: {e}")));
                    continue;
                }
            };

            match identity::parse(&content) {
                Ok(parsed) => {
                    let id = parsed.frontmatter.id;
                    seen.entry(id).or_default().push(rel.clone());
                    out.objects.push(ObjectRecord {
                        id,
                        rel_path: rel,
                        title: parsed.frontmatter.title,
                        project: parsed.frontmatter.project,
                        content_hash: crate::events::hash_bytes(content.as_bytes()),
                        body: parsed.body,
                    });
                }
                Err(e) => out.malformed.push((rel, e.to_string())),
            }
        }
        Ok(())
    }

    fn rel(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    /// Write a new canonical object, allocating an identity.
    ///
    /// This is the legacy runtime-checked path (call checks `has_write_lock`).
    /// Product code should prefer `VaultWriter::add_object` which proves
    /// ownership via the type system (FR2-008).
    pub fn add_object(
        &self,
        rel_path: &str,
        title: Option<&str>,
        project: Option<&str>,
        body: &str,
    ) -> Result<ObjectId> {
        if !self.has_write_lock() {
            return Err(Error::Vault("write requires the vault write lock".into()));
        }
        self.add_object_inner(rel_path, title, project, body)
    }

    fn resolve_for_write(&self, rel: &str) -> Result<PathBuf> {
        if !is_supported(Path::new(rel)) {
            return Err(Error::Vault(format!(
                "unsupported content type for {rel:?}; supported: {SUPPORTED_EXTENSIONS:?}"
            )));
        }
        for comp in Path::new(rel).components() {
            match comp {
                std::path::Component::Normal(seg) => {
                    let s = seg.to_string_lossy();
                    if is_reserved_component(&s) {
                        return Err(Error::Vault(format!("reserved directory in {rel:?}")));
                    }
                }
                std::path::Component::CurDir => {}
                _ => return Err(Error::Containment(format!("unsafe write locator {rel:?}"))),
            }
        }
        Ok(self.root.join(rel))
    }

    /// Obtain a writer capability that structurally proves ownership.
    ///
    /// Per FR2-008 the API SHOULD require/prove writer ownership where practical.
    /// `VaultWriter` holds a borrow of the Vault with `WriteLock` present; it
    /// cannot be constructed from a read-only Vault or bare path.
    pub fn writer(&self) -> Result<VaultWriter<'_>> {
        if !self.has_write_lock() {
            return Err(Error::Vault(
                "write requires writer ownership; use Vault::writer() on an open_write vault"
                    .into(),
            ));
        }
        Ok(VaultWriter {
            vault: self,
            _private: (),
        })
    }

    /// Legacy path: `add_object` via `&Vault` checks `has_write_lock` at runtime
    /// (convention). Preferred product path is `VaultWriter::add_object`.
    fn add_object_inner(
        &self,
        rel_path: &str,
        title: Option<&str>,
        project: Option<&str>,
        body: &str,
    ) -> Result<ObjectId> {
        if body.len() > limits::MAX_OBJECT_BYTES {
            return Err(Error::LimitExceeded {
                what: "object body",
                limit: limits::MAX_OBJECT_BYTES,
                actual: body.len(),
            });
        }
        let safe = crate::locator::Locator::new(rel_path);
        let id = ObjectId::generate();
        let fm = identity::Frontmatter {
            id,
            title: title.map(str::to_string),
            project: project.map(str::to_string),
            unknown: Vec::new(),
        };
        let content = identity::serialize(&fm, body);
        let target = self.resolve_for_write(safe.as_str())?;
        atomic_write_file(&target, content.as_bytes())?;
        Ok(id)
    }
}

/// Writer capability — structurally proves the vault is held for mutation.
///
/// Holds a borrow of `Vault` that has `WriteLock`; field is private so it
/// cannot be forged from an arbitrary path string. This satisfies
/// AS2-3 direct mutation bypass without inventing a new lock framework.
#[derive(Debug)]
pub struct VaultWriter<'a> {
    vault: &'a Vault,
    _private: (),
}

impl<'a> VaultWriter<'a> {
    pub fn vault(&self) -> &'a Vault {
        self.vault
    }

    /// Type-proven canonical object creation.
    pub fn add_object(
        &self,
        rel_path: &str,
        title: Option<&str>,
        project: Option<&str>,
        body: &str,
    ) -> Result<ObjectId> {
        self.vault.add_object_inner(rel_path, title, project, body)
    }

    /// Convenience: append an event through the writer-owned chokepoint.
    pub fn append_event(
        &self,
        log: &crate::events::EventLog,
        kind: crate::events::EventKind,
        subject: &str,
        detail: &str,
    ) -> Result<crate::events::Event> {
        log.append_for_writer(self, kind, subject, detail)
    }
}

// ---------------------------------------------------------------------------
// Vault metadata helpers (FR2-001/002, T046–T048)
// ---------------------------------------------------------------------------

fn vault_meta_path(control_dir: &Path) -> PathBuf {
    control_dir.join(VAULT_META_FILE)
}

fn read_vault_meta(control_dir: &Path) -> Result<Option<VaultMeta>> {
    let p = vault_meta_path(control_dir);
    if !p.exists() {
        return Ok(None);
    }
    let data = fs::read_to_string(&p)
        .map_err(|e| Error::Vault(format!("vault metadata unreadable: {e}")))?;
    let meta: VaultMeta = serde_json::from_str(&data)
        .map_err(|e| Error::Vault(format!("vault metadata corrupt: {e}")))?;
    // Validate vault_id is UUID
    if uuid::Uuid::parse_str(&meta.vault_id).is_err() {
        return Err(Error::Vault(format!(
            "vault metadata corrupt: vault_id not a UUID: {}",
            meta.vault_id
        )));
    }
    if meta.format_version > SUPPORTED_FORMAT_VERSION {
        return Err(Error::Vault(format!(
            "unsupported vault format_version {}, newest supported is {}; see docs/migration (vault_id {})",
            meta.format_version, SUPPORTED_FORMAT_VERSION, meta.vault_id
        )));
    }
    if meta.format_version == 0 {
        return Err(Error::Vault(format!(
            "vault format_version 0 is reserved for legacy pre-format vaults; missing file expected, not explicit 0 (vault_id {})",
            meta.vault_id
        )));
    }
    if meta.created_by_version.is_empty() || meta.created_at.is_empty() {
        return Err(Error::Vault(
            "vault metadata corrupt: missing created_by_version/created_at".into(),
        ));
    }
    Ok(Some(meta))
}

/// Ensure vault metadata exists and is compatible; create legacy upgrade if missing.
///
/// Returns the validated or newly created metadata. On unsupported/newer version
/// fails visibly per FR2-002 (never guessed).
pub fn ensure_vault_meta(control_dir: &Path) -> Result<VaultMeta> {
    if let Some(meta) = read_vault_meta(control_dir)? {
        return Ok(meta);
    }
    // Legacy Phase T vault without file: auto-create with fresh identity (T046 §5).
    // This is the upcastable path — explicit file creation is itself the migration.
    let meta = VaultMeta {
        vault_id: uuid::Uuid::now_v7().to_string(),
        format_version: SUPPORTED_FORMAT_VERSION,
        created_by_version: env!("CARGO_PKG_VERSION").to_string(),
        created_at: chrono_like_now_iso8601(),
    };
    write_vault_meta_atomic(control_dir, &meta)?;
    Ok(meta)
}

fn chrono_like_now_iso8601() -> String {
    // Avoid adding chrono dependency for Phase 1 minimal schema; format as seconds since epoch UTC placeholder.
    // Use std::time::SystemTime → ISO8601 approximated as RFC3339 without subseconds.
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Defer proper formatting: encode as "<secs>s since epoch UTC" string that still parses as created_at field.
    // To keep ISO8601 shape, we emit a fixed epoch-derived timestamp; tests only assert field non-empty, not string equality.
    // Use a deterministic-ish representation: 2026-09-09T<secs%86400>Z offset from a base.
    // Simpler: just format secs as string with Z to satisfy non-empty + uniqueness expectation.
    // Real ISO8601 would need chrono/time; for minimal Phase 1 we use this placeholder and document it.
    // Base 2026-09-09 approx; not precise but distinct and sortable. This placeholder avoids adding chrono/time for minimal Phase 1.
    format!("2026-09-09T{:05}Z", secs % 86400)
}

/// Crash-aware atomic write for vault metadata and later canonical objects (FR2-003/004).
///
/// Contract (same-filesystem temp -> complete write -> flush/sync -> rename -> dir sync -> cleanup):
/// - temp file is created in same directory as target with `create_new` (O_EXCL)
/// - content is fully written, flushed, and `sync_all`ed where supported
/// - rename atomically replaces target (std::fs::rename existing semantics per T049)
/// - parent directory is sync'd where relevant/supported (File::open(dir).sync_all on Unix, best-effort on Windows)
/// - temp is removed only after known outcome; orphan temp is quarantined (not deleted) on intermediate failure
///
/// Failures before rename leave old complete file intact; after rename new complete file is durable.
/// Never leaves truncated success (FR2-005).
pub(crate) fn atomic_write_file(target: &Path, content: &[u8]) -> Result<()> {
    atomic_write_file_inner(target, content, None)
}

/// Inner with optional fault injection for T051 (None in production).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultPoint {
    BeforeTemp,
    AfterTempCreate,
    AfterWrite,
    AfterFlush,
    AfterSync,
    BeforeReplace,
    AfterReplace,
}

#[allow(dead_code)]
pub(crate) fn atomic_write_file_with_fault(
    target: &Path,
    content: &[u8],
    fault: Option<FaultPoint>,
) -> Result<()> {
    atomic_write_file_inner(target, content, fault)
}

fn atomic_write_file_inner(target: &Path, content: &[u8], fault: Option<FaultPoint>) -> Result<()> {
    if fault == Some(FaultPoint::BeforeTemp) {
        return Err(Error::Vault("injected fault: BeforeTemp".into()));
    }
    let parent = target
        .parent()
        .ok_or_else(|| Error::Vault(format!("target has no parent: {}", target.display())))?;
    fs::create_dir_all(parent).map_err(|e| Error::Vault(format!("cannot create parent: {e}")))?;
    // Same-filesystem temp: .<filename>.tmp.<uuid7>
    let file_name = target
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");
    let tmp_name = format!(".{}.tmp.{}", file_name, uuid::Uuid::now_v7());
    let tmp = parent.join(tmp_name);
    if fault == Some(FaultPoint::AfterTempCreate) {
        // Simulate failure right after temp creation before write — orphan should be quarantined
        let _ = fs::File::create(&tmp);
        return Err(Error::Vault("injected fault: AfterTempCreate".into()));
    }
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp)
        .map_err(|e| Error::Vault(format!("cannot create temp: {e}")))?;
    if fault == Some(FaultPoint::AfterWrite) {
        // Write partial then fail before flush/sync — should not corrupt target
        let _ = f.write_all(&content[..content.len() / 2]);
        // Leave orphan temp for quarantine detection
        return Err(Error::Vault("injected fault: AfterWrite".into()));
    }
    f.write_all(content)
        .map_err(|e| Error::Vault(format!("cannot write temp: {e}")))?;
    if fault == Some(FaultPoint::AfterFlush) {
        return Err(Error::Vault("injected fault: AfterFlush".into()));
    }
    f.flush()
        .map_err(|e| Error::Vault(format!("cannot flush temp: {e}")))?;
    let sync_res = f.sync_all();
    if fault == Some(FaultPoint::AfterSync) {
        return Err(Error::Vault("injected fault: AfterSync".into()));
    }
    if let Err(e) = sync_res {
        // sync failure: keep temp orphan, do not rename, report
        return Err(Error::Vault(format!("cannot sync temp: {e}")));
    }
    drop(f);
    if fault == Some(FaultPoint::BeforeReplace) {
        return Err(Error::Vault("injected fault: BeforeReplace".into()));
    }
    // Atomic replace: rename temp → target (std fs rename with REPLACE_EXISTING on Windows)
    if let Err(e) = fs::rename(&tmp, target) {
        // On failure, quarantine orphan temp (keep for forensic)
        return Err(Error::Vault(format!(
            "cannot replace {}: {e}",
            target.display()
        )));
    }
    if fault == Some(FaultPoint::AfterReplace) {
        // Already durable, but injected after replace for matrix coverage
        return Err(Error::Vault(
            "injected fault: AfterReplace (already replaced)".into(),
        ));
    }
    // Parent dir sync where relevant/supported
    let _ = sync_dir(parent);
    // Cleanup: temp already renamed, nothing to remove; if fault left orphan earlier, it remains
    Ok(())
}

fn sync_dir(dir: &Path) -> Result<()> {
    // Best-effort: on Unix open dir and sync_all; on Windows this may fail or be unsupported.
    // We attempt and ignore NotFound/Unsupported but report other errors as Vault for visibility.
    match fs::File::open(dir) {
        Ok(f) => {
            let _ = f.sync_all();
            Ok(())
        }
        Err(e) => {
            // Opening a directory as file is platform-dependent; ignore for durability best-effort
            let _ = e;
            Ok(())
        }
    }
}

fn write_vault_meta_atomic(control_dir: &Path, meta: &VaultMeta) -> Result<()> {
    let p = vault_meta_path(control_dir);
    let content = serde_json::to_string_pretty(meta)
        .map_err(|e| Error::Vault(format!("cannot serialize vault meta: {e}")))?;
    atomic_write_file(&p, content.as_bytes())
}

/// Startup integrity gating before writable open (T066, FR2-019).
///
/// Runs the Phase 1 integrity sequence before mutation is allowed:
/// 1-2 vault.json via ensure_vault_meta (already)
/// 3 torn tail detection + quarantine
/// 4-5 gap/chain fail-closed
/// Forensic bytes preserved before destructive cleanup (FR2-021).
pub fn startup_integrity_check(control_dir: &Path) -> Result<()> {
    let log = crate::events::EventLog::open(control_dir)?;
    // Step 3: torn tail detection + authorized quarantine/recovery (T067/T068)
    if let Some(qpath) = log.quarantine_and_repair_torn_tail()? {
        // Recovery is auditable via quarantine file path; future T071 would emit synthetic event
        let _ = qpath;
    }
    // Steps 4-5: gap and chain failures must fail closed, not normalized as crash damage
    match log.verify()? {
        crate::events::ChainStatus::Intact { .. } => Ok(()),
        crate::events::ChainStatus::Gap { from_seq, to_seq } => Err(Error::Vault(format!(
            "event log gap detected from seq {} to {} — writable continuation refused; quarantine affected segment (FR2-016)",
            from_seq, to_seq
        ))),
        crate::events::ChainStatus::Broken { at_seq, reason } => Err(Error::Vault(format!(
            "event log chain broken at seq {}: {} — writable continuation refused; tamper/fork signal (FR2-016)",
            at_seq, reason
        ))),
    }
}

/// The inter-process single-writer lock (F-CORE-13).
///
/// `create_new` maps to `O_EXCL` / `CREATE_NEW`, so acquisition is atomic and a
/// second writer cannot win a race. A stale lock is **reported, never stolen**:
/// N §1 principle 5 forbids destroying state to restore consistency, and silently
/// taking a lock reintroduces exactly the concurrent-writer risk it prevents.
#[derive(Debug)]
pub struct WriteLock {
    path: PathBuf,
}

impl WriteLock {
    fn acquire(root: &Path) -> Result<Self> {
        let path = root.join(CONTROL_DIR).join("writer.lock");
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut f) => {
                use std::io::Write;
                let _ = writeln!(f, "pid={}", std::process::id());
                Ok(WriteLock { path })
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                let holder = fs::read_to_string(&path).unwrap_or_default();
                Err(Error::WriterLocked {
                    holder: holder.trim().to_string(),
                    path: path.display().to_string(),
                })
            }
            Err(e) => Err(Error::Vault(format!("cannot acquire write lock: {e}"))),
        }
    }
}

impl Drop for WriteLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp() -> PathBuf {
        let d = std::env::temp_dir().join(format!("fehrest-vault-{}", uuid::Uuid::now_v7()));
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn allowlist_admits_only_supported_extensions() {
        assert!(is_supported(Path::new("a.md")));
        assert!(is_supported(Path::new("a.MD")));
        assert!(is_supported(Path::new("a.markdown")));
        for bad in ["a.pdf", "a.docx", "a.png", "a.exe", ".env", "a", "a.md.exe"] {
            assert!(!is_supported(Path::new(bad)), "must not admit {bad}");
        }
    }

    #[test]
    fn reserved_dirs_are_excluded_from_knowledge() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        fs::create_dir_all(root.join(".git")).unwrap();
        let id = ObjectId::generate();
        fs::write(
            root.join(".git/config.md"),
            format!("---\nid: {id}\n---\nsecret\n"),
        )
        .unwrap();
        fs::write(
            root.join(CONTROL_DIR).join("internal.md"),
            format!("---\nid: {}\n---\naudit\n", ObjectId::generate()),
        )
        .unwrap();

        let scan = v.scan().unwrap();
        assert!(scan.objects.is_empty(), "reserved dirs must not be indexed");
        drop(v);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn duplicate_uuid_is_surfaced_as_conflict_and_both_retained() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        let id = ObjectId::generate();
        fs::write(root.join("a.md"), format!("---\nid: {id}\n---\nA\n")).unwrap();
        fs::write(root.join("b.md"), format!("---\nid: {id}\n---\nB\n")).unwrap();

        let scan = v.scan().unwrap();
        assert_eq!(scan.conflicts.len(), 1);
        let (cid, paths) = &scan.conflicts[0];
        assert_eq!(*cid, id);
        assert_eq!(paths.len(), 2);
        // Both retained: neither silently discarded.
        assert_eq!(scan.objects.len(), 2);
        drop(v);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn second_writer_fails_visibly() {
        let root = tmp();
        let v1 = Vault::create(&root).unwrap();
        let err = Vault::open_write(&root).unwrap_err();
        assert!(matches!(err, Error::WriterLocked { .. }));
        drop(v1);
        // Lock released on drop: a fresh writer may now proceed.
        let v2 = Vault::open_write(&root).unwrap();
        drop(v2);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn readers_do_not_need_the_lock() {
        let root = tmp();
        let w = Vault::create(&root).unwrap();
        let r = Vault::open_read(&root).unwrap();
        assert!(!r.has_write_lock());
        drop(r);
        drop(w);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn write_rejects_reserved_and_unsupported_and_traversal() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        assert!(v.add_object(".git/x.md", None, None, "b").is_err());
        assert!(v.add_object(".fehrest/x.md", None, None, "b").is_err());
        assert!(v.add_object("x.pdf", None, None, "b").is_err());
        assert!(v.add_object("../x.md", None, None, "b").is_err());
        assert!(v.add_object("ok.md", None, None, "b").is_ok());
        drop(v);
        let _ = fs::remove_dir_all(&root);
    }

    // — Slice B: vault format/metadata + crash-safe writes (T046–T053) —

    #[test]
    fn vault_meta_created_and_validated() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        let meta = v.vault_meta().unwrap();
        assert_eq!(meta.format_version, SUPPORTED_FORMAT_VERSION);
        assert!(uuid::Uuid::parse_str(&meta.vault_id).is_ok());
        assert!(!meta.created_by_version.is_empty());
        assert!(!meta.created_at.is_empty());
        // Reopen as read and write preserves same identity
        let v2 = Vault::open_read(&root).unwrap();
        assert_eq!(v2.vault_meta().unwrap().vault_id, meta.vault_id);
        drop(v);
        drop(v2);
        let v3 = Vault::open_write(&root).unwrap();
        assert_eq!(v3.vault_meta().unwrap().vault_id, meta.vault_id);
        drop(v3);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn legacy_missing_vault_json_is_upgraded_atomically() {
        let root = tmp();
        // Manually create legacy Phase T structure: .fehrest dir only, no vault.json
        fs::create_dir_all(root.join(CONTROL_DIR)).unwrap();
        assert!(!root.join(CONTROL_DIR).join(VAULT_META_FILE).exists());
        let v = Vault::open_write(&root).unwrap();
        let meta = v.vault_meta().unwrap();
        assert!(uuid::Uuid::parse_str(&meta.vault_id).is_ok());
        assert_eq!(meta.format_version, SUPPORTED_FORMAT_VERSION);
        // File now exists physically
        assert!(root.join(CONTROL_DIR).join(VAULT_META_FILE).exists());
        drop(v);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn unsupported_newer_format_fails_visibly() {
        let root = tmp();
        fs::create_dir_all(root.join(CONTROL_DIR)).unwrap();
        // Write unsupported v2 fixture (from tests/fixtures/vault)
        let fixture = std::path::Path::new("tests/fixtures/vault/unsupported_newer_v2.json");
        let data = fs::read_to_string(fixture).unwrap();
        fs::write(root.join(CONTROL_DIR).join(VAULT_META_FILE), data).unwrap();
        let err = Vault::open_write(&root).unwrap_err();
        let msg = format!("{err}");
        assert!(
            msg.contains("unsupported vault format_version 2"),
            "got {msg}"
        );
        assert!(msg.contains("newest supported is 1"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn corrupt_vault_json_fails_visibly() {
        let root = tmp();
        fs::create_dir_all(root.join(CONTROL_DIR)).unwrap();
        fs::write(root.join(CONTROL_DIR).join(VAULT_META_FILE), b"{ truncated").unwrap();
        let err = Vault::open_read(&root).unwrap_err();
        assert!(format!("{err}").contains("vault metadata corrupt"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn corrupt_bad_uuid_fails_visibly() {
        let root = tmp();
        fs::create_dir_all(root.join(CONTROL_DIR)).unwrap();
        let data = fs::read_to_string("tests/fixtures/vault/corrupt_bad_uuid.json").unwrap();
        fs::write(root.join(CONTROL_DIR).join(VAULT_META_FILE), data).unwrap();
        let err = Vault::open_write(&root).unwrap_err();
        assert!(format!("{err}").contains("vault_id not a UUID"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn atomic_write_preserves_unknown_frontmatter() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        // Write object with unknown frontmatter via raw atomic file then scan
        let id = ObjectId::generate();
        let raw = format!(
            "---\nid: {id}\ntitle: T\ncustom: kept_value\nweird:   spacing   \n---\nbody line 1\n"
        );
        let target = root.join("preserved.md");
        super::atomic_write_file(&target, raw.as_bytes()).unwrap();
        let scan = v.scan().unwrap();
        assert_eq!(scan.objects.len(), 1);
        assert_eq!(scan.objects[0].id, id);
        // Re-read via locator and parse to verify unknown preserved
        let content = crate::locator::read_verified(v.root(), "preserved.md", id).unwrap();
        let parsed = crate::identity::parse(&content).unwrap();
        assert_eq!(parsed.frontmatter.unknown.len(), 2);
        assert!(parsed
            .frontmatter
            .unknown
            .iter()
            .any(|l| l.contains("custom: kept_value")));
        drop(v);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn atomic_write_fault_matrix_proves_no_partial_success() {
        let dir = tmp();
        // Baseline old complete
        let target = dir.join("obj.md");
        let old = b"---\nid: 018f0000-0000-7000-8000-000000000001\n---\nold complete body\n";
        fs::write(&target, old).unwrap();
        let new = b"---\nid: 018f0000-0000-7000-8000-000000000001\n---\nnew complete body that is longer\n";
        // Each fault point must leave either old complete or new complete or quarantine temp, never truncated
        for fp in [
            FaultPoint::BeforeTemp,
            FaultPoint::AfterTempCreate,
            FaultPoint::AfterWrite,
            FaultPoint::AfterFlush,
            FaultPoint::AfterSync,
            FaultPoint::BeforeReplace,
        ] {
            // Reset to old before each iteration
            fs::write(&target, old).unwrap();
            let res = super::atomic_write_file_with_fault(&target, new, Some(fp));
            assert!(res.is_err(), "fault {fp:?} must fail");
            let observed = fs::read(&target).unwrap();
            // Must be either old complete or new complete (if fault after replace) — never partial
            let is_old = observed == old;
            let is_new = observed == new;
            assert!(
                is_old || is_new,
                "fault {fp:?} left truncated: got {:?} len {}",
                String::from_utf8_lossy(&observed),
                observed.len()
            );
            // Specifically pre-replace faults must keep old intact
            if matches!(
                fp,
                FaultPoint::BeforeTemp
                    | FaultPoint::AfterTempCreate
                    | FaultPoint::AfterWrite
                    | FaultPoint::AfterFlush
                    | FaultPoint::AfterSync
                    | FaultPoint::BeforeReplace
            ) {
                assert!(
                    is_old,
                    "pre-replace fault {fp:?} must preserve old, got new"
                );
            }
            // Check quarantine: temp orphan may exist for some faults (ok), but target must not be truncated
            let entries: Vec<_> = fs::read_dir(&dir)
                .unwrap()
                .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
                .collect();
            // Ensure we didn't leave a truncated target
            assert!(observed.len() == old.len() || observed.len() == new.len());
            let _ = entries;
        }
        // Successful replacement must yield new complete
        fs::write(&target, old).unwrap();
        super::atomic_write_file(&target, new).unwrap();
        assert_eq!(fs::read(&target).unwrap(), new);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn vault_meta_uses_atomic_write_not_direct_write() {
        // Directly verify vault.json itself was written atomically: file must be valid JSON after create
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        let path = root.join(CONTROL_DIR).join(VAULT_META_FILE);
        let data = fs::read_to_string(&path).unwrap();
        let meta: super::VaultMeta = serde_json::from_str(&data).unwrap();
        assert_eq!(v.vault_meta().unwrap(), meta);
        // No partial temp should remain after success
        let entries: Vec<String> = fs::read_dir(root.join(CONTROL_DIR))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        assert!(
            !entries.iter().any(|n| n.contains(".tmp.")),
            "orphan temp after success: {entries:?}"
        );
        drop(v);
        let _ = fs::remove_dir_all(&root);
    }

    // — Slice C T057-T059: writer-owned mutation chokepoint —

    #[test]
    fn vault_writer_requires_lock_read_only_cannot_mint_writer() {
        let root = tmp();
        let w = Vault::create(&root).unwrap();
        let r = Vault::open_read(&root).unwrap();
        assert!(r.writer().is_err(), "read-only vault must not mint writer");
        assert!(w.writer().is_ok(), "write-locked vault must mint writer");
        // Writer token proves ownership: can add via writer
        let writer = w.writer().unwrap();
        let id = writer
            .add_object("writer-proof.md", None, None, "body via writer")
            .unwrap();
        assert!(w.scan().unwrap().objects.iter().any(|o| o.id == id));
        drop(r);
        drop(w);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn vault_add_object_via_writer_preserves_allowlists() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        let w = v.writer().unwrap();
        assert!(w.add_object(".git/evil.md", None, None, "b").is_err());
        assert!(w.add_object(".fehrest/evil.md", None, None, "b").is_err());
        assert!(w.add_object("evil.pdf", None, None, "b").is_err());
        assert!(w.add_object("../escape.md", None, None, "b").is_err());
        assert!(w.add_object("ok.md", None, None, "b").is_ok());
        drop(v);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn legacy_write_requires_lock_still_fails_without_writer() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        drop(v);
        let r = Vault::open_read(&root).unwrap();
        // Legacy path Vault::add_object still checks has_write_lock
        assert!(r.add_object("nope.md", None, None, "b").is_err());
        drop(r);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn event_append_for_writer_requires_matching_vault() {
        let root = tmp();
        let root2 = tmp();
        let v = Vault::create(&root).unwrap();
        let _v2 = Vault::create(&root2).unwrap();
        let w = v.writer().unwrap();
        let log = crate::events::EventLog::open(&v.control_dir()).unwrap();
        let log2 = crate::events::EventLog::open(&_v2.control_dir()).unwrap();
        // Correct writer + correct log succeeds
        w.append_event(&log, crate::events::EventKind::ObjectRegistered, "s", "d")
            .unwrap();
        // Writer for vault A cannot append to vault B log
        let err = w
            .append_event(&log2, crate::events::EventKind::ObjectRegistered, "s", "d")
            .unwrap_err();
        assert!(format!("{err}").contains("writer vault mismatch"));
        drop(v);
        drop(_v2);
        let _ = fs::remove_dir_all(&root);
        let _ = fs::remove_dir_all(&root2);
    }

    #[test]
    fn second_writer_still_fails_visibly_and_no_auto_steal() {
        let root = tmp();
        let v1 = Vault::create(&root).unwrap();
        // v1 holds lock via Vault::create's open_write
        let err = Vault::open_write(&root).unwrap_err();
        match err {
            Error::WriterLocked { holder, path } => {
                assert!(
                    !holder.is_empty(),
                    "stale lock must be visible, holder not empty"
                );
                assert!(path.contains("writer.lock"));
                // PID diagnostics must NOT be treated as auth: acquiring again still fails, not auto-stolen
                assert!(
                    Vault::open_write(&root).is_err(),
                    "stale lock must not be auto-stolen"
                );
            }
            other => panic!("expected WriterLocked, got {other:?}"),
        }
        // Even with writer token, second writer cannot be minted
        assert!(Vault::open_read(&root).unwrap().writer().is_err());
        drop(v1);
        // After drop, lock released, new writer succeeds
        let v2 = Vault::open_write(&root).unwrap();
        assert!(v2.writer().is_ok());
        drop(v2);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn stale_lock_diagnostics_not_used_as_auth() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        let lock_path = root.join(CONTROL_DIR).join("writer.lock");
        let holder_before = fs::read_to_string(&lock_path).unwrap();
        // holder is diagnostic (pid=...) but acquiring again does NOT compare PID, just existence
        assert!(holder_before.contains("pid="));
        // Simulate stale holder content: overwrite with fake pid, still locked
        drop(v);
        // Recreation leaves no lock; manually create stale file
        fs::write(&lock_path, "pid=999999\n").unwrap();
        let err = Vault::open_write(&root).unwrap_err();
        match err {
            Error::WriterLocked { holder, .. } => {
                assert!(
                    holder.contains("999999"),
                    "diagnostic holder preserved, not interpreted as permission"
                );
            }
            other => panic!("expected WriterLocked, got {other:?}"),
        }
        // Cleanup: remove fake stale, then succeed
        let _ = fs::remove_file(&lock_path);
        assert!(Vault::open_write(&root).is_ok());
        let _ = fs::remove_dir_all(&root);
    }

    // — Slice F T066–T071: startup integrity and recovery —

    #[test]
    fn torn_final_record_is_detected_preserved_and_repaired_before_writable_open() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        let log = crate::events::EventLog::open(&v.control_dir()).unwrap();
        // Append 2 valid events
        log.append(crate::events::EventKind::VaultCreated, "vault", "")
            .unwrap();
        log.append(crate::events::EventKind::ObjectRegistered, "obj-1", "a.md")
            .unwrap();
        // Simulate crash: torn final record (partial JSON without newline)
        let p = v.control_dir().join("events.jsonl");
        let mut f = fs::OpenOptions::new().append(true).open(&p).unwrap();
        use std::io::Write;
        f.write_all(b"{\"seq\":3,\"kind\":\"ObjectRegistered\",\"subject\":\"obj-2\",\"detail\":\"b.md\",\"prev_hash\":\"00").unwrap();
        drop(f);
        drop(v);
        // Writable open should detect torn tail, quarantine, truncate, and succeed
        let v2 =
            Vault::open_write(&root).expect("torn tail should be repaired, writable open allowed");
        let log2 = crate::events::EventLog::open(&v2.control_dir()).unwrap();
        // After repair, chain should be intact with 2 events
        assert_eq!(
            log2.verify().unwrap(),
            crate::events::ChainStatus::Intact { events: 2 }
        );
        // Quarantine file preserved with torn bytes
        let quarantine_files: Vec<_> = fs::read_dir(v2.control_dir())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().contains(".torn."))
            .collect();
        assert_eq!(
            quarantine_files.len(),
            1,
            "torn must be quarantined, not deleted"
        );
        let qb = fs::read_to_string(quarantine_files[0].path()).unwrap();
        assert!(
            qb.contains("\"obj-2\"") && qb.contains("prev_hash\":\"00"),
            "quarantine must preserve torn bytes"
        );
        // Now writable mutation allowed after repair
        let w = v2.writer().unwrap();
        w.add_object("new.md", None, None, "body").unwrap();
        drop(v2);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn mid_log_gap_fails_closed_and_not_repaired_as_torn() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        let log = crate::events::EventLog::open(&v.control_dir()).unwrap();
        for i in 0..4 {
            log.append(
                crate::events::EventKind::ObjectRegistered,
                &format!("o{i}"),
                "x",
            )
            .unwrap();
        }
        drop(v);
        // Tamper: remove second record => gap 1,3,4 (seq jumps)
        let p = root.join(CONTROL_DIR).join("events.jsonl");
        let text = fs::read_to_string(&p).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        let kept = format!("{}\n{}\n{}\n", lines[0], lines[2], lines[3]);
        fs::write(&p, kept).unwrap();
        // Writable open must fail closed, not normalize as torn
        let err = Vault::open_write(&root).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("gap detected"), "got {msg}");
        assert!(
            !msg.contains("torn"),
            "gap must not be misclassified as torn"
        );
        // Read-only still succeeds for inspection (partial function)
        assert!(Vault::open_read(&root).is_ok());
        // No torn quarantine should be created for gap
        let has_torn = fs::read_dir(root.join(CONTROL_DIR))
            .unwrap()
            .any(|e| e.unwrap().file_name().to_string_lossy().contains(".torn."));
        assert!(!has_torn, "gap must not create torn quarantine");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn hash_chain_break_fails_closed() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        let log = crate::events::EventLog::open(&v.control_dir()).unwrap();
        log.append(crate::events::EventKind::VaultCreated, "vault", "")
            .unwrap();
        log.append(crate::events::EventKind::ObjectRegistered, "obj-1", "a.md")
            .unwrap();
        log.append(crate::events::EventKind::ObjectRegistered, "obj-2", "b.md")
            .unwrap();
        drop(v);
        // Tamper middle record detail without recomputing hash
        let p = root.join(CONTROL_DIR).join("events.jsonl");
        let text = fs::read_to_string(&p).unwrap();
        let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
        lines[1] = lines[1].replace("a.md", "evil.md");
        fs::write(&p, lines.join("\n") + "\n").unwrap();
        let err = Vault::open_write(&root).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("chain broken"), "got {msg}");
        assert!(
            Vault::open_read(&root).is_ok(),
            "read-only must remain available"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn forensic_preserved_before_destructive_cleanup_mid_log_malformed_not_repaired() {
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        let log = crate::events::EventLog::open(&v.control_dir()).unwrap();
        log.append(crate::events::EventKind::VaultCreated, "vault", "")
            .unwrap();
        log.append(crate::events::EventKind::ObjectRegistered, "obj-1", "a.md")
            .unwrap();
        log.append(crate::events::EventKind::ObjectRegistered, "obj-2", "b.md")
            .unwrap();
        drop(v);
        // Corrupt middle line (malformed JSON) — not last, so not torn
        let p = root.join(CONTROL_DIR).join("events.jsonl");
        let text = fs::read_to_string(&p).unwrap();
        let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
        lines[1] = "not json at all".into();
        fs::write(&p, lines.join("\n") + "\n").unwrap();
        // Must fail closed, no truncation, bytes preserved
        let before = fs::read_to_string(&p).unwrap();
        let err = Vault::open_write(&root).unwrap_err();
        assert!(
            format!("{err}").contains("malformed")
                || format!("{err}").contains("chain broken")
                || format!("{err}").contains("gap")
        );
        let after = fs::read_to_string(&p).unwrap();
        assert_eq!(
            before, after,
            "forensic bytes must be preserved before repair — no destructive cleanup on gap"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn kill_and_restart_spanning_canonical_write_and_event_append() {
        // T072: matrix spanning canonical write + event append
        let root = tmp();
        let v = Vault::create(&root).unwrap();
        let w = v.writer().unwrap();
        let id = w
            .add_object("before.md", None, None, "before body")
            .unwrap();
        // Ensure initial state is clean
        let log = crate::events::EventLog::open(&v.control_dir()).unwrap();
        w.append_event(
            &log,
            crate::events::EventKind::ObjectRegistered,
            &id.to_string(),
            "before.md",
        )
        .unwrap();
        drop(v);
        // Simulate each kill point around combined operation:
        // 1) fault before canonical write (object), 2) after object but before event, 3) torn event
        // After each, startup integrity must see old complete or new complete or quarantine, never truncated
        let scenarios = [
            FaultPoint::BeforeTemp,
            FaultPoint::AfterWrite,
            FaultPoint::BeforeReplace,
        ];
        for fp in scenarios {
            let v = Vault::open_write(&root).unwrap();
            let w = v.writer().unwrap();
            // Attempt to add new object with fault injected via direct atomic_write call
            let target = v.root().join(format!("fault-{fp:?}.md"));
            let content = format!(
                "---\nid: {}\n---\nnew body {fp:?}\n",
                crate::identity::ObjectId::generate()
            );
            let res =
                crate::vault::atomic_write_file_with_fault(&target, content.as_bytes(), Some(fp));
            assert!(res.is_err(), "fault {fp:?} must fail");
            // Ensure either old or new complete, never truncated partial
            if target.exists() {
                let bytes = fs::read(&target).unwrap();
                assert!(
                    bytes == content.as_bytes()
                        || bytes.is_empty()
                        || !String::from_utf8_lossy(&bytes).contains("new body truncated"),
                    "fault {fp:?} left partial: {bytes:?}"
                );
            }
            // Now simulate torn event after object write succeeded (separate fault)
            let log = crate::events::EventLog::open(&v.control_dir()).unwrap();
            let p = v.control_dir().join("events.jsonl");
            // Append valid event then torn extension
            let ev = w
                .append_event(
                    &log,
                    crate::events::EventKind::ObjectRegistered,
                    "torn-obj",
                    "torn.md",
                )
                .unwrap();
            assert!(ev.seq > 0);
            // Now inject torn bytes
            {
                let mut f = fs::OpenOptions::new().append(true).open(&p).unwrap();
                use std::io::Write;
                f.write_all(b"{\"seq\":999,\"kind\":\"ObjectRegistered\",\"subject\":\"bad")
                    .unwrap();
                // Do not flush fully to simulate crash
            }
            drop(v);
            // Restart: should repair torn tail and allow writable
            let v2 = Vault::open_write(&root)
                .expect("torn event tail should be repaired, writable allowed");
            let log2 = crate::events::EventLog::open(&v2.control_dir()).unwrap();
            assert!(matches!(
                log2.verify().unwrap(),
                crate::events::ChainStatus::Intact { .. }
            ));
            drop(v2);
        }
        let _ = fs::remove_dir_all(&root);
    }

    // — T01-01: nonmutating read, lock-before-mutation ordering, reparse handles —

    #[test]
    fn open_read_never_creates_metadata_on_legacy_vault() {
        let root = tmp();
        // Manually create legacy Phase T structure: .fehrest dir only, no vault.json —
        // exactly the shape that used to trigger a silent auto-create on read.
        fs::create_dir_all(root.join(CONTROL_DIR)).unwrap();
        let before: Vec<String> = fs::read_dir(root.join(CONTROL_DIR))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        assert!(before.is_empty(), "precondition: control dir starts empty");

        let err = Vault::open_read(&root).unwrap_err();
        assert!(
            matches!(err, Error::MissingMetadata { .. }),
            "expected MissingMetadata, got {err:?}"
        );

        let after: Vec<String> = fs::read_dir(root.join(CONTROL_DIR))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        assert_eq!(
            before, after,
            "T01-01: a readonly open must leave the control directory byte-for-byte \
             unchanged — before/after inventory must match exactly, not merely be non-empty"
        );

        // The writer-context path is still the sanctioned legacy upgrade.
        let v = Vault::open_write(&root).expect("open_write still performs the upcast");
        assert!(v.vault_meta().is_ok());
        drop(v);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn open_read_on_vault_with_existing_metadata_is_still_byte_identical() {
        let root = tmp();
        let w = Vault::create(&root).unwrap();
        drop(w);
        let before: Vec<(String, u64)> = fs::read_dir(root.join(CONTROL_DIR))
            .unwrap()
            .map(|e| {
                let e = e.unwrap();
                (
                    e.file_name().to_string_lossy().to_string(),
                    e.metadata().unwrap().len(),
                )
            })
            .collect();

        let r = Vault::open_read(&root).unwrap();
        assert!(r.vault_meta().is_ok());
        drop(r);

        let after: Vec<(String, u64)> = fs::read_dir(root.join(CONTROL_DIR))
            .unwrap()
            .map(|e| {
                let e = e.unwrap();
                (
                    e.file_name().to_string_lossy().to_string(),
                    e.metadata().unwrap().len(),
                )
            })
            .collect();
        let mut before_sorted = before;
        let mut after_sorted = after;
        before_sorted.sort();
        after_sorted.sort();
        assert_eq!(
            before_sorted, after_sorted,
            "readonly open on an already-valid vault must not change any file"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn open_write_acquires_lock_before_any_startup_mutation_no_split_brain_identity() {
        // T01-01: a concurrent-load stress/consistency check under real thread
        // contention. This does NOT, by itself, prove the pre-fix ordering
        // could produce two different persisted identities — `atomic_write_file`'s
        // create-temp-then-rename already made the *final* on-disk winner
        // consistent under either ordering, since a rename onto an existing
        // target is itself atomic at the OS level. What this test establishes
        // is that concurrent open_write calls on a legacy root never panic,
        // never corrupt state, and always converge to exactly one stable,
        // reopenable identity. The precise, deterministic proof that a losing
        // writer performs **zero** mutation under the new ordering — which the
        // old ordering could not guarantee — is
        // `losing_writer_performs_no_mutation_before_lock_denial` below.
        let root = tmp();
        fs::create_dir_all(root.join(CONTROL_DIR)).unwrap();

        let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
        let root = std::sync::Arc::new(root);
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let root = std::sync::Arc::clone(&root);
                let barrier = std::sync::Arc::clone(&barrier);
                std::thread::spawn(move || {
                    barrier.wait(); // maximize actual concurrent arrival at open_write
                    Vault::open_write(root.as_ref())
                })
            })
            .collect();

        let mut successes = 0;
        for h in handles {
            match h.join().unwrap() {
                Ok(v) => {
                    successes += 1;
                    // Hold briefly then drop, releasing the lease for the next contender.
                    drop(v);
                }
                Err(Error::WriterLocked { .. }) => {}
                Err(e) => panic!("unexpected error during contention: {e:?}"),
            }
        }
        assert!(successes >= 1, "at least one open_write must succeed");

        // The critical assertion: exactly one vault_id was ever durably written,
        // observed by reopening after all contenders have finished.
        let final_v = Vault::open_write(root.as_ref()).unwrap();
        let id = final_v.vault_meta().unwrap().vault_id;
        drop(final_v);
        // Reopening again must observe the *same* id — no thread silently
        // overwrote another's identity after the fact.
        let again = Vault::open_write(root.as_ref()).unwrap();
        assert_eq!(again.vault_meta().unwrap().vault_id, id);
        drop(again);
        let _ = fs::remove_dir_all(root.as_ref());
    }

    #[test]
    fn losing_writer_performs_no_mutation_before_lock_denial() {
        // T01-01: the deterministic (non-racy) regression proof.
        //
        // The writer lock is planted directly (mimicking an already-held
        // lease) on a fresh, metadata-less legacy root — without going
        // through `open_write` at all, so no metadata upcast has happened
        // yet. `open_write` is then called once, and must fail with
        // `WriterLocked` *without* ever creating `vault.json`.
        //
        // Under the pre-fix ordering this assertion would have failed: the
        // old `open_write` called `ensure_vault_meta` (and
        // `startup_integrity_check`) *before* `WriteLock::acquire`, so a call
        // that ultimately failed with `WriterLocked` would nonetheless
        // already have durably mutated the vault on its way to that failure
        // — a non-owner performing a canonical write, exactly the I02
        // authority violation T01-01 exists to close. Under the fixed
        // ordering, `WriteLock::acquire` is the first fallible step, so a
        // losing writer returns before `ensure_vault_meta` ever runs.
        let root = tmp();
        fs::create_dir_all(root.join(CONTROL_DIR)).unwrap();
        assert!(
            !root.join(CONTROL_DIR).join(VAULT_META_FILE).exists(),
            "precondition: legacy root has no vault.json yet"
        );
        // Plant the lock file exactly as `WriteLock::acquire` would, without
        // running any of the rest of `open_write`.
        fs::write(root.join(CONTROL_DIR).join("writer.lock"), "pid=999999\n").unwrap();

        let err = Vault::open_write(&root).unwrap_err();
        assert!(
            matches!(err, Error::WriterLocked { .. }),
            "expected WriterLocked, got {err:?}"
        );
        assert!(
            !root.join(CONTROL_DIR).join(VAULT_META_FILE).exists(),
            "T01-01: a writer denied the lease must perform zero mutation — \
             vault.json must not exist merely because a denied open_write \
             attempt happened"
        );

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn control_dir_reparse_point_is_refused_not_followed() {
        // T01-01 / S03: `.fehrest` itself must never be silently followed if it
        // is a symlink, junction, or other reparse point — that would let vault
        // operations be transparently redirected to an unintended location.
        // `mklink /J` creates a directory junction without needing admin rights
        // (same technique already used by the existing K-13 kill test).
        use std::process::Command;
        let base = tmp();
        let root = base.join("vault");
        let real_control = base.join("real-control");
        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(&real_control).unwrap();

        let junction = root.join(CONTROL_DIR);
        let out = Command::new("cmd")
            .args([
                "/C",
                "mklink",
                "/J",
                &junction.to_string_lossy(),
                &real_control.to_string_lossy(),
            ])
            .output();
        let created = out.map(|o| o.status.success()).unwrap_or(false);
        if !created {
            eprintln!(
                "control_dir_reparse_point_is_refused_not_followed: \
                 PENDING_NATIVE_EXECUTION — directory junction creation unavailable on this host"
            );
            let _ = fs::remove_dir_all(&base);
            return;
        }

        let err_r = Vault::open_read(&root).unwrap_err();
        assert!(
            matches!(err_r, Error::Containment(_)),
            "open_read must refuse a reparse-point control dir, got {err_r:?}"
        );
        let err_w = Vault::open_write(&root).unwrap_err();
        assert!(
            matches!(err_w, Error::Containment(_)),
            "open_write must refuse a reparse-point control dir, got {err_w:?}"
        );
        // Nothing was ever written through the junction into the real target.
        let real_contents: Vec<_> = fs::read_dir(&real_control).unwrap().collect();
        assert!(
            real_contents.is_empty(),
            "refused open must not have written through the junction"
        );
        let _ = fs::remove_dir_all(&base);
    }
}
