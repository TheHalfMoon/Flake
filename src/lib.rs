//! # Fehrest — Phase T headless thesis-proof
//!
//! **This is an experiment, not a product.** Its purpose is to test whether a fresh
//! agent continues long-running work more correctly with a small local Fehrest Core
//! than with strong simpler baselines. A negative result is a successful experiment.
//!
//! Any persistence format here is `EXPERIMENTAL_PHASE_T_FORMAT` /
//! `NOT_PRODUCT_FORMAT_FREEZE`.
//!
//! ## What is deliberately absent
//!
//! No graph, vectors, embeddings, CRDT, sync, MCP, Cedar, UI, automatic memory
//! promotion, or network code path. Each is either hypothesis-gated or unauthorized
//! ([ARCHITECTURE_FREEZE §9](../docs/canonical/ARCHITECTURE_FREEZE.md), Phase T
//! authorization boundary).

pub mod backup;
pub mod canonical;
pub mod capture;
pub mod cli;
pub mod context;
pub mod derived;
pub mod envelope;
pub mod events;
pub mod identity;
pub mod locator;
pub mod markdown;
pub mod memory;
pub mod migration;
pub mod project;
pub mod recovery;
pub mod relation;
pub mod temporal;
pub mod vault;

/// Local resource-safety bounds (F-CORE-15).
///
/// **These are technical safety limits, not product quotas.** There is deliberately
/// no daily limit, no tier, no trial exhaustion, and no vendor-controlled
/// availability concept anywhere in this codebase — a grep for those concepts is
/// part of the checklist (CL-55).
///
/// Values are Phase T fixtures sized for the experiment, not measured budgets.
/// Real values come from B-0, which has not run.
pub mod limits {
    /// Largest canonical object admitted.
    pub const MAX_OBJECT_BYTES: usize = 1 << 20; // 1 MiB
    /// Largest single memory statement.
    pub const MAX_STATEMENT_BYTES: usize = 8 << 10; // 8 KiB
    /// Largest event detail payload.
    pub const MAX_EVENT_BYTES: usize = 16 << 10; // 16 KiB
    /// Largest compiled context package.
    pub const MAX_PACKAGE_BYTES: usize = 256 << 10; // 256 KiB
    /// Largest accepted search query.
    pub const MAX_QUERY_BYTES: usize = 1 << 10; // 1 KiB
    /// Largest number of candidates returned from lexical search.
    pub const MAX_SEARCH_RESULTS: usize = 200;
    /// Largest raw serialized command payload `canonical::commit` accepts
    /// before any transaction opens (`T02-02`). This is a transport-level
    /// resource-safety backstop distinct from any specific record type's own
    /// tighter product-facing field limit (`MAX_OBJECT_BYTES` for text
    /// bodies, `capture::MAX_ARTIFACT_BYTES` for opaque artifacts — each
    /// already checked earlier by their own module). Sized to admit a
    /// hex-encoded `capture::MAX_ARTIFACT_BYTES` artifact (2x inflation)
    /// plus JSON envelope overhead, with margin.
    pub const MAX_COMMAND_PAYLOAD_BYTES: usize = 150 << 20; // 150 MiB
}

#[derive(Debug)]
pub enum Error {
    Vault(String),
    /// Filesystem containment refused a locator. Distinct from `IdentityMismatch`
    /// on purpose: they defend disjoint failures (E §12.1) and collapsing them
    /// would hide which guarantee actually fired.
    Containment(String),
    /// Post-open verification found the opened content is a different object.
    IdentityMismatch {
        expected: String,
        actual: String,
    },
    NoFrontmatter,
    MissingId,
    InvalidId(String),
    Derived(String),
    /// The format-2 canonical store (`canonical.rs`, T01-02): staged
    /// creation, publication, guard/database identity or schema-recognition
    /// failure. Never a mutation of an already-published store on this
    /// path — see `canonical.rs` module docs for the exact scope boundary.
    Canonical(String),
    /// Recovery/access-coordination failure (`recovery.rs`, T01-04):
    /// exclusive recovery access denied by a currently-open normal
    /// connection, forensic preservation failure, or recovery-candidate
    /// verification failure. Distinct from `Canonical`/`Vault` so a caller
    /// can tell "this store's own content is wrong" apart from "recovery
    /// itself could not proceed" — see `recovery.rs` module docs.
    Recovery(String),
    /// Backup/restore failure (`backup.rs`, T01-05): cancellation before
    /// publication, a manifest/member digest mismatch, or backup-candidate
    /// verification failure. See `backup.rs` module docs.
    Backup(String),
    /// Legacy format-1-to-format-2 migration failure (`migration.rs`,
    /// T01-06): an ambiguous "complete" migration request, an unadmittable
    /// or unrecognized explicit selection, or an import-time admission
    /// failure. See `migration.rs` module docs.
    Migration(String),
    /// Typed record admission failure (`project.rs`, T02-01): a field-limit
    /// violation, an invalid cross-project reference, a malformed or
    /// unsupported record payload, or a wrong-kind object reference. See
    /// `project.rs` module docs.
    Project(String),
    /// Source/artifact admission failure (`capture.rs`, T02-02): a refused
    /// symlink, non-regular-file, obviously-secret filename, oversized
    /// artifact, or unreachable selected path. Distinct from `Project` so a
    /// caller can tell "this record's fields are invalid" apart from "this
    /// security boundary refused the selection" — see `capture.rs` module
    /// docs.
    Capture(String),
    Event(String),
    Memory(String),
    /// An invalid supersession edge. Never silently normalised (F §6.1).
    InvalidSupersession(String),
    /// Another process holds the canonical write lock (F-CORE-13).
    WriterLocked {
        holder: String,
        path: String,
    },
    /// A readonly open found the vault's control directory but no identity
    /// file inside it (T01-01). Distinct from "not a vault": readonly opens
    /// never create or upcast state — only a writer-context open may. The
    /// caller should open for write to perform the explicit legacy migration.
    MissingMetadata {
        control_dir: String,
    },
    /// A local resource-safety bound was exceeded. Explicit, audited, and never a
    /// silent discard of canonical state.
    LimitExceeded {
        what: &'static str,
        limit: usize,
        actual: usize,
    },
    Scope(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Vault(m) => write!(f, "vault error: {m}"),
            Error::Containment(m) => write!(f, "containment refused: {m}"),
            Error::IdentityMismatch { expected, actual } => write!(
                f,
                "identity mismatch: requested {expected}, opened content claims {actual}"
            ),
            Error::NoFrontmatter => write!(f, "no canonical frontmatter"),
            Error::MissingId => write!(f, "frontmatter has no id"),
            Error::InvalidId(s) => write!(f, "invalid object id: {s}"),
            Error::Derived(m) => write!(f, "derived store error: {m}"),
            Error::Canonical(m) => write!(f, "canonical store error: {m}"),
            Error::Recovery(m) => write!(f, "recovery error: {m}"),
            Error::Backup(m) => write!(f, "backup error: {m}"),
            Error::Migration(m) => write!(f, "migration error: {m}"),
            Error::Project(m) => write!(f, "project error: {m}"),
            Error::Capture(m) => write!(f, "capture error: {m}"),
            Error::Event(m) => write!(f, "event log error: {m}"),
            Error::Memory(m) => write!(f, "memory error: {m}"),
            Error::InvalidSupersession(m) => write!(f, "invalid supersession: {m}"),
            Error::WriterLocked { holder, path } => write!(
                f,
                "vault is locked by another writer ({holder}); lock file: {path}"
            ),
            Error::MissingMetadata { control_dir } => write!(
                f,
                "vault metadata missing at {control_dir}/vault.json; open for write to migrate this legacy vault (readonly open never creates it)"
            ),
            Error::LimitExceeded {
                what,
                limit,
                actual,
            } => write!(
                f,
                "resource safety limit exceeded for {what}: {actual} > {limit}"
            ),
            Error::Scope(m) => write!(f, "scope error: {m}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
