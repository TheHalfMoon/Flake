//! `ExportGrant`: an owner-issued, time-bounded, scope-bounded authorization
//! to compile one disclosure package (`T03-04`).
//!
//! §15's own "Export grant" row: "Owner-issued ID, project, allowed record
//! types/IDs, expiry, byte budget, privacy exclusions, active/revoked
//! state and policy version. Local canonical state only; no authority
//! survives export/import." §18's "Owner flow" names the step this
//! satisfies: "issue or select an unexpired grant" — before any package is
//! ever compiled, the owner has already named exactly which project, which
//! record kinds, which explicit exclusions, how large a package, and how
//! long the authorization lasts.
//!
//! **Not a bearer credential.** A grant object never itself carries read
//! authority the way a capability token would — it is inert canonical
//! state that [`crate::disclosure::compile_disclosure_package`] checks
//! before compiling, exactly like every other admission check in this
//! crate (I02: only Core-admitted checks grant authority, never data
//! shape). §18: "A grant is not a bearer credential embedded in the
//! package and cannot be imported as live authority" — this crate's own
//! `import.rs` never re-derives write/read authority from an imported
//! `ExportGrant`'s mere presence; a fresh grant must always be reissued by
//! the *current* owner of the destination vault.
//!
//! **Ninth `RecordPayload` kind, `Relation`'s exact shape.** An ordinary
//! opaque-payload canonical object committed through
//! `CommandTarget::CreateObject`/`UpdateObject` — no new `CommandTarget`,
//! no schema change. [`revoke_grant`] is the only mutating transition
//! ([`issue_grant`] is a create only, never later edited: a grant's scope
//! is fixed at issuance, per §18 "any change before commit requires a new
//! preview/confirmation; it cannot silently expand the package" — applied
//! here as "a grant cannot silently expand either; issue a new one").

use crate::canonical::{CanonicalStore, CanonicalWriter, CommandOutcome, RecordOrigin};
use crate::capture::{now_rfc3339_utc, parse_rfc3339_utc};
use crate::limits;
use crate::project::{self, RecordPayload};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Map as JsonMap;

/// §18: "Grant default expiry is 24 hours, maximum seven days."
pub const DEFAULT_TTL_SECS: i64 = 24 * 3600;
pub const MAX_TTL_SECS: i64 = 7 * 24 * 3600;

/// The five work-record kinds a grant may name. Deliberately excludes
/// `project`/`source_check`/`review_checkpoint`/`export_grant`/
/// `disclosure_receipt` — none of those is a §18 "ordinary note/action
/// edits, draft decisions, evidence relations" disclosure target.
pub const DISCLOSABLE_KINDS: &[&str] = &["note", "action", "decision", "source", "relation"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantState {
    Active,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportGrant {
    pub payload_schema_version: u32,
    pub project_id: String,
    /// Empty means "every `DISCLOSABLE_KINDS` kind is allowed" — an
    /// explicit, non-empty allowlist narrows it. Never a denylist: adding
    /// a new kind in the future must never silently widen an existing
    /// grant's scope.
    #[serde(default)]
    pub allowed_kinds: Vec<String>,
    /// `None` means no ID restriction beyond kind/exclusions. `Some` is an
    /// explicit allowlist, intersected with `allowed_kinds`.
    #[serde(default)]
    pub allowed_object_ids: Option<Vec<String>>,
    /// Object IDs excluded regardless of `allowed_kinds`/
    /// `allowed_object_ids` — the owner's explicit "never this one".
    #[serde(default)]
    pub privacy_exclusions: Vec<String>,
    pub byte_budget: u32,
    pub expires_at: String,
    pub state: GrantState,
    pub policy_version: u32,
    pub issued_at: String,
    #[serde(flatten)]
    pub unknown: JsonMap<String, serde_json::Value>,
}

pub const POLICY_VERSION: u32 = 1;

impl ExportGrant {
    /// Does this grant currently authorize compiling a package? Checks
    /// both halves §18 requires before any compilation: `state == Active`
    /// and `expires_at` in the future relative to `now_secs`.
    pub fn is_usable_at(&self, now_secs: i64) -> Result<()> {
        if self.state != GrantState::Active {
            return Err(Error::Project(
                "grant is revoked; issue a new grant to disclose again".into(),
            ));
        }
        let expires_secs = parse_rfc3339_utc(&self.expires_at)?;
        if now_secs >= expires_secs {
            return Err(Error::Project(format!(
                "grant expired at {}; issue a new grant to disclose again",
                self.expires_at
            )));
        }
        Ok(())
    }

    /// Is `object_id` of kind `kind_str` in this grant's scope? Privacy
    /// exclusions always win, checked first and unconditionally.
    pub fn covers(&self, kind_str: &str, object_id: &str) -> bool {
        if self.privacy_exclusions.iter().any(|x| x == object_id) {
            return false;
        }
        let kind_ok =
            self.allowed_kinds.is_empty() || self.allowed_kinds.iter().any(|k| k == kind_str);
        if !kind_ok {
            return false;
        }
        match &self.allowed_object_ids {
            None => true,
            Some(ids) => ids.iter().any(|id| id == object_id),
        }
    }
}

/// Issue a new grant. `ttl_secs` must be in `(0, MAX_TTL_SECS]`; the CLI's
/// own default of `DEFAULT_TTL_SECS` is a caller-side convenience, not
/// enforced here — this function enforces only the hard ceiling §18 names.
#[allow(clippy::too_many_arguments)]
pub fn issue_grant(
    writer: &mut CanonicalWriter<'_>,
    actor: &str,
    project_id: &str,
    allowed_kinds: &[String],
    allowed_object_ids: Option<&[String]>,
    privacy_exclusions: &[String],
    byte_budget: u32,
    ttl_secs: i64,
) -> Result<(CommandOutcome, ExportGrant)> {
    project::require_project(writer.store(), project_id)?;
    for k in allowed_kinds {
        if !DISCLOSABLE_KINDS.contains(&k.as_str()) {
            return Err(Error::Project(format!(
                "unrecognized disclosable kind {k:?}; expected one of {DISCLOSABLE_KINDS:?}"
            )));
        }
    }
    if byte_budget == 0 || byte_budget as usize > limits::MAX_PACKAGE_BYTES {
        return Err(Error::Project(format!(
            "byte_budget must be in 1..={} bytes, got {byte_budget}",
            limits::MAX_PACKAGE_BYTES
        )));
    }
    if !(0..=MAX_TTL_SECS).contains(&ttl_secs) || ttl_secs == 0 {
        return Err(Error::Project(format!(
            "ttl_secs must be in 1..={MAX_TTL_SECS} (max seven days), got {ttl_secs}"
        )));
    }

    let now_secs = parse_rfc3339_utc(&now_rfc3339_utc())?;
    let record = RecordPayload::ExportGrant(ExportGrant {
        payload_schema_version: project::RECORD_PAYLOAD_SCHEMA_VERSION,
        project_id: project_id.to_string(),
        allowed_kinds: allowed_kinds.to_vec(),
        allowed_object_ids: allowed_object_ids.map(|ids| ids.to_vec()),
        privacy_exclusions: privacy_exclusions.to_vec(),
        byte_budget,
        expires_at: crate::capture::rfc3339_utc_from_unix_seconds(now_secs + ttl_secs),
        state: GrantState::Active,
        policy_version: POLICY_VERSION,
        issued_at: crate::capture::rfc3339_utc_from_unix_seconds(now_secs),
        unknown: JsonMap::new(),
    });
    let (outcome, record) = project::commit_create(writer, actor, RecordOrigin::User, record)?;
    Ok((
        outcome,
        match record {
            RecordPayload::ExportGrant(g) => g,
            _ => unreachable!(),
        },
    ))
}

/// Revoke an active grant. Refuses to revoke an already-revoked grant (not
/// an idempotent no-op) so a caller cannot mistake a stale revoke command
/// for a fresh one.
pub fn revoke_grant(
    store: &mut CanonicalStore,
    actor: &str,
    grant_id: &str,
    expected_revision_id: &str,
) -> Result<(CommandOutcome, ExportGrant)> {
    let mut grant = current_grant(store, grant_id)?;
    if grant.state != GrantState::Active {
        return Err(Error::Project(format!(
            "grant {grant_id} is already revoked"
        )));
    }
    grant.state = GrantState::Revoked;
    let mut writer = store.writer()?;
    let (outcome, record) = project::commit_update(
        &mut writer,
        actor,
        RecordOrigin::User,
        grant_id,
        expected_revision_id,
        RecordPayload::ExportGrant(grant),
    )?;
    Ok((
        outcome,
        match record {
            RecordPayload::ExportGrant(g) => g,
            _ => unreachable!(),
        },
    ))
}

pub fn current_grant(store: &CanonicalStore, grant_id: &str) -> Result<ExportGrant> {
    let (_, payload) = store
        .read_current(grant_id)?
        .ok_or_else(|| Error::Project(format!("no grant exists with id {grant_id}")))?;
    Ok(RecordPayload::from_json(&payload)?
        .as_export_grant()?
        .clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-grant-{}", uuid::Uuid::now_v7()))
    }

    fn cleanup(p: &Path) {
        let _ = std::fs::remove_dir_all(p);
    }

    fn open_store() -> (PathBuf, CanonicalStore) {
        let root = tmp();
        let store = CanonicalStore::create(&root).unwrap();
        (root, store)
    }

    fn make_project(store: &mut CanonicalStore) -> String {
        let mut writer = store.writer().unwrap();
        let (outcome, _p) = project::create_project(&mut writer, "owner", "P", None).unwrap();
        outcome.object_id
    }

    #[test]
    fn issue_grant_defaults_to_full_disclosable_scope() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let mut writer = store.writer().unwrap();
        let (_outcome, grant) = issue_grant(
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
        drop(writer);
        assert!(grant.allowed_kinds.is_empty());
        assert!(grant.covers("note", "anything"));
        assert!(grant.covers("source", "anything"));
        cleanup(&root);
    }

    #[test]
    fn covers_respects_kind_allowlist_id_allowlist_and_exclusions() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let mut writer = store.writer().unwrap();
        let (_outcome, grant) = issue_grant(
            &mut writer,
            "owner",
            &project_id,
            &["note".to_string()],
            Some(&["n1".to_string(), "n2".to_string()]),
            &["n2".to_string()],
            1024,
            3600,
        )
        .unwrap();
        drop(writer);
        assert!(grant.covers("note", "n1"));
        assert!(!grant.covers("note", "n2"), "explicit exclusion wins");
        assert!(!grant.covers("note", "n3"), "not in the ID allowlist");
        assert!(!grant.covers("action", "n1"), "wrong kind");
        cleanup(&root);
    }

    #[test]
    fn unrecognized_kind_is_refused() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let mut writer = store.writer().unwrap();
        let err = issue_grant(
            &mut writer,
            "owner",
            &project_id,
            &["project".to_string()],
            None,
            &[],
            1024,
            3600,
        )
        .unwrap_err();
        assert!(format!("{err}").contains("unrecognized disclosable kind"));
        cleanup(&root);
    }

    #[test]
    fn byte_budget_out_of_range_is_refused() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let mut writer = store.writer().unwrap();
        let err =
            issue_grant(&mut writer, "owner", &project_id, &[], None, &[], 0, 3600).unwrap_err();
        assert!(format!("{err}").contains("byte_budget"));
        let err = issue_grant(
            &mut writer,
            "owner",
            &project_id,
            &[],
            None,
            &[],
            limits::MAX_PACKAGE_BYTES as u32 + 1,
            3600,
        )
        .unwrap_err();
        assert!(format!("{err}").contains("byte_budget"));
        cleanup(&root);
    }

    #[test]
    fn ttl_beyond_seven_days_is_refused() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let mut writer = store.writer().unwrap();
        let err = issue_grant(
            &mut writer,
            "owner",
            &project_id,
            &[],
            None,
            &[],
            1024,
            MAX_TTL_SECS + 1,
        )
        .unwrap_err();
        assert!(format!("{err}").contains("ttl_secs"));
        cleanup(&root);
    }

    #[test]
    fn revoke_makes_the_grant_unusable_and_cannot_be_repeated() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (grant_id, rev) = {
            let mut writer = store.writer().unwrap();
            let (outcome, _g) = issue_grant(
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
            (outcome.object_id, outcome.revision_id)
        };
        let now = parse_rfc3339_utc(&now_rfc3339_utc()).unwrap();
        assert!(current_grant(&store, &grant_id)
            .unwrap()
            .is_usable_at(now)
            .is_ok());

        let (outcome2, revoked) = revoke_grant(&mut store, "owner", &grant_id, &rev).unwrap();
        assert_eq!(revoked.state, GrantState::Revoked);
        assert!(current_grant(&store, &grant_id)
            .unwrap()
            .is_usable_at(now)
            .is_err());

        let err = revoke_grant(&mut store, "owner", &grant_id, &outcome2.revision_id).unwrap_err();
        assert!(format!("{err}").contains("already revoked"));
        cleanup(&root);
    }

    #[test]
    fn expired_grant_is_unusable() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let mut writer = store.writer().unwrap();
        let (_outcome, grant) =
            issue_grant(&mut writer, "owner", &project_id, &[], None, &[], 1024, 1).unwrap();
        drop(writer);
        let far_future = parse_rfc3339_utc(&grant.expires_at).unwrap() + 10;
        let err = grant.is_usable_at(far_future).unwrap_err();
        assert!(format!("{err}").contains("expired"));
        cleanup(&root);
    }
}
