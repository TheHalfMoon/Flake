//! The disclosure compiler and `DisclosureReceipt` (`T03-04`): compile one
//! bounded, scope-checked package from a live [`crate::grant::ExportGrant`],
//! persisting an immutable receipt before any bytes are ever handed back to
//! a caller.
//!
//! §18's "Owner flow": "...issue or select an unexpired grant → request
//! package → Core checks canonical scope/types/lifecycle/as-of and budget
//! → persist immutable receipt and exact wire bytes → write a staged
//! package file → verify and publish it." [`compile_disclosure_package`]
//! is the middle three steps; the CLI layer does the staged-file
//! write/verify/publish.
//!
//! **Not `context.rs`.** `src/context.rs` is Phase T's own historical,
//! immutable "Context Compiler" (`AGENTS.md` §3) — it compiles from the
//! historical `Memory` type, not from Flake-v1's `Decision`/`Note`/
//! `Action`/`Source`/`Relation`. This module reuses only the *shape* of
//! its two load-bearing properties (budget atomicity: an item is full,
//! truncated, or entirely omitted, never partially-labelled; the emitted
//! record is built from what was actually emitted, not from the selection
//! set) — reimplemented standalone against the live canonical model, per
//! this crate's own established non-reuse pattern (`relation.rs` vs
//! `temporal.rs`, `decision_state.rs` vs `temporal.rs`'s own resolver).
//!
//! **Receipt-before-emission (§18, D1/D5).** [`compile_disclosure_package`]
//! commits the [`DisclosureReceipt`] via `CommandTarget::CreateObject`
//! *before* it returns the wire bytes to its caller. If that commit fails,
//! the function returns `Err` and the caller never receives any bytes —
//! "failed receipt persistence emits no package" is satisfied by
//! construction (there is no code path that returns wire bytes without a
//! successfully committed receipt immediately before it).
//!
//! **§16 cross-project boundary, by construction, not by filtering.**
//! Every candidate item this module ever looks at comes from a
//! project-scoped read (`project::list_project_records`,
//! `capture::list_project_sources`, `relation::list_project_relations`,
//! all already-audited `T02-01`-`T02-03` functions) — another project's
//! records are never read into memory here at all, not read-then-excluded
//! (S02).
//!
//! **Every exclusion carries a reason (§15 "selected/rejected revision IDs
//! and reason codes").** A candidate item can be rejected for a grant
//! reason (wrong kind, not in the ID allowlist, privacy-excluded) or a
//! budget reason (envelope does not fit, no room for any content) or —
//! `Relation` only, checked after every non-`Relation` item has already
//! been decided — because one of its endpoints was not itself disclosed in
//! this same package (never reveal a relation naming an object the
//! recipient cannot otherwise see).
//!
//! **Deliberate scope boundary: only `Accepted`/conflicting decisions are
//! disclosure candidates.** A `Draft` or solely `Withdrawn` decision for a
//! key with no currently-`Accepted` candidate is never offered to the
//! compiler at all (not even as a rejection) — it is not yet an owner
//! assertion an external reader should be told about (§17: "agent output
//! never self-accepts"; a draft is, symmetrically, not yet disclosed
//! either). `T03-02`'s [`crate::decision_state::resolve_decision_state`]
//! is reused unchanged to determine, per distinct `decision_key`, whether
//! its candidates are the `conflicts` group (`NeedsReview`) or the
//! `current_decisions` group (`CurrentSet`).
//!
//! **Deliberate scope boundary: `Source` content is metadata only, never
//! `capture.bytes_hex`.** A `Source`'s canonical payload embeds its exact
//! captured bytes inline (`capture::Capture::bytes_hex`) — this module's
//! rendered "content" for a `Source` is its label, kind, digest and byte
//! length only, never the hex-encoded bytes themselves. A 256 KiB
//! UTF-8-text package was never going to carry arbitrary binary evidence
//! anyway; keeping raw bytes out is a explicit, load-bearing choice here,
//! not an oversight.

use crate::canonical::{CanonicalStore, RecordOrigin};
use crate::capture::now_rfc3339_utc;
use crate::decision_state::{self, DecisionOutcome};
use crate::events::hash_bytes;
use crate::grant::ExportGrant;
use crate::project::{self, Action, ActionState, Decision, Note, RecordPayload};
use crate::relation::Relation;
use crate::source_check::{self, CheckStatus};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Map as JsonMap;
use std::collections::HashSet;

pub const COMPILER_VERSION: u32 = 1;
pub const MAX_REQUEST_ID_BYTES: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemTruncation {
    Full,
    Truncated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectedItem {
    pub object_id: String,
    pub kind: String,
    pub revision_id: String,
    pub truncation: ItemTruncation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RejectedItem {
    pub object_id: String,
    pub kind: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisclosureReceipt {
    pub payload_schema_version: u32,
    pub request_id: String,
    pub grant_id: String,
    pub grant_revision_id: String,
    pub project_id: String,
    pub principal_label: String,
    pub as_of_recorded: i64,
    pub as_of_valid: String,
    pub selected: Vec<SelectedItem>,
    pub rejected: Vec<RejectedItem>,
    pub compiler_version: u32,
    pub policy_version: u32,
    pub emitted_byte_count: u64,
    pub emitted_sha256: String,
    pub created_at: String,
    #[serde(flatten)]
    pub unknown: JsonMap<String, serde_json::Value>,
}

/// One item as emitted onto the wire — never constructed from the
/// selection set, only from what the budget loop actually kept.
///
/// **Wire shape: a header line plus one JSON object per line, not one big
/// JSON document.** `rejected`/omitted items are never written to the wire
/// at all — they live only on the (separately persisted, unbounded)
/// [`DisclosureReceipt`], mirroring `context.rs`'s own already-established
/// separation between its budget-capped `wire: String` and its
/// unbounded `Manifest` (which carries its own `omissions`). Wrapping every
/// item inside one shared JSON document (an `items` array inside an outer
/// object) was tried first and rejected: the outer envelope's own byte
/// cost is not attributable to any single item, which broke "budget
/// atomicity" (measuring the *actual* rendered size, never an estimate) —
/// see this task's own evidence report, "Failed attempts". A flat,
/// line-oriented wire makes every line's own length exactly what it
/// contributes to the total, with no shared-envelope distortion to reason
/// about.
#[derive(Debug, Serialize)]
struct WireItem {
    object_id: String,
    kind: String,
    revision_id: String,
    truncation: &'static str,
    content: String,
}

#[derive(Debug, Serialize)]
struct WireHeader {
    request_id: String,
    grant_id: String,
    project_id: String,
    principal_label: String,
    as_of_recorded: i64,
    as_of_valid: String,
    compiler_version: u32,
    policy_version: u32,
}

/// One record considered for disclosure, before grant/budget decisions.
struct Candidate {
    object_id: String,
    kind: &'static str,
    revision_id: String,
    content: String,
    /// §18 priority-group rank: 0 = conflicts, 1 = stale/missing evidence,
    /// 2 = current decisions, 3 = active actions, 4 = other evidence
    /// (`Match`/never-checked sources), 5 = notes. `Relation`s are handled
    /// in a later pass (rank has no meaning for them) and are always
    /// ordered after every other kind regardless.
    group_rank: u8,
}

fn note_content(n: &Note) -> String {
    format!("{}\n\n{}", n.title.clone().unwrap_or_default(), n.body)
}

fn action_content(a: &Action) -> String {
    format!(
        "{} [{:?}]\n\n{}",
        a.title,
        a.state,
        a.body.clone().unwrap_or_default()
    )
}

fn decision_content(d: &Decision) -> String {
    format!(
        "{}: {}\n\n{}",
        d.decision_key,
        d.statement,
        d.rationale.clone().unwrap_or_default()
    )
}

/// Metadata only — never `capture.bytes_hex` (see module docs).
fn source_content(s: &crate::capture::Source) -> String {
    match &s.capture {
        Some(c) => format!(
            "{} ({:?}, {} bytes, sha256={})",
            s.label, s.source_kind, c.byte_length, c.sha256
        ),
        None => format!("{} ({:?}, reference only)", s.label, s.source_kind),
    }
}

fn relation_content(r: &Relation) -> String {
    format!(
        "{:?}: {} -> {}{}",
        r.relation_type,
        r.from_object_id,
        r.to_object_id,
        r.note
            .as_ref()
            .map(|n| format!(" ({n})"))
            .unwrap_or_default()
    )
}

/// Gather every non-`Relation` disclosure candidate for `project_id`,
/// classified into its §18 priority group. Grant scope is not applied
/// here — that happens in the budget pass, so every exclusion reason is
/// recorded once, in one place.
fn gather_candidates(
    store: &CanonicalStore,
    project_id: &str,
    head_seq: i64,
) -> Result<Vec<Candidate>> {
    let records = project::list_project_records(store, project_id)?;
    let mut out = Vec::new();

    let mut decision_keys: Vec<&str> = records
        .iter()
        .filter_map(|(_, r)| r.as_decision().ok())
        .map(|d| d.decision_key.as_str())
        .collect();
    decision_keys.sort_unstable();
    decision_keys.dedup();

    for key in decision_keys {
        let resolution =
            decision_state::resolve_decision_state(store, project_id, key, None, Some(head_seq))?;
        let group_rank = match resolution.outcome {
            DecisionOutcome::NeedsReview => 0,
            DecisionOutcome::CurrentSet(_) => 2,
            DecisionOutcome::NoAcceptedDecision => continue, // never a candidate at all
        };
        for c in resolution.considered.into_iter().filter(|c| c.admitted) {
            out.push(Candidate {
                object_id: c.decision_id,
                kind: "decision",
                revision_id: String::new(), // filled below from list_current_objects
                content: decision_content(&c.decision),
                group_rank,
            });
        }
    }

    for (source_id, source) in crate::capture::list_project_sources(store, project_id)? {
        let stale = source_check::list_checks_for_source(store, &source_id)?
            .into_iter()
            .last()
            .map(|(_, c)| c.status != CheckStatus::Match)
            .unwrap_or(false);
        out.push(Candidate {
            object_id: source_id,
            kind: "source",
            revision_id: String::new(),
            content: source_content(&source),
            group_rank: if stale { 1 } else { 4 },
        });
    }

    for (id, a) in records.iter().filter_map(|(id, r)| {
        r.as_action()
            .ok()
            .filter(|a| a.state != ActionState::Done && a.state != ActionState::Cancelled)
            .map(|a| (id.clone(), a))
    }) {
        out.push(Candidate {
            object_id: id,
            kind: "action",
            revision_id: String::new(),
            content: action_content(a),
            group_rank: 3,
        });
    }

    for (id, n) in records
        .iter()
        .filter_map(|(id, r)| r.as_note().ok().map(|n| (id.clone(), n)))
    {
        out.push(Candidate {
            object_id: id,
            kind: "note",
            revision_id: String::new(),
            content: note_content(n),
            group_rank: 5,
        });
    }

    // Fill in each candidate's exact current revision_id in one pass,
    // rather than threading it through every branch above.
    for c in &mut out {
        let (revision_id, _) = store.read_current(&c.object_id)?.ok_or_else(|| {
            Error::Project(format!("object {} disappeared mid-compile", c.object_id))
        })?;
        c.revision_id = revision_id;
    }

    out.sort_by(|a, b| {
        a.group_rank
            .cmp(&b.group_rank)
            .then_with(|| a.object_id.cmp(&b.object_id))
    });
    Ok(out)
}

/// Fit `content` within `remaining` bytes when rendered as one [`WireItem`]
/// line, mirroring `context.rs`'s own budget-atomicity shape: measure the
/// actual rendered line length, never an estimate; if the envelope alone
/// (empty content) does not fit, omit entirely; otherwise shrink content at
/// a UTF-8 boundary until it fits, or omit if even empty content does not
/// fit. Returns the item (for the receipt), the exact rendered line (for
/// the wire) and that line's own byte length.
fn fit_to_budget(
    object_id: &str,
    kind: &'static str,
    revision_id: &str,
    content: &str,
    remaining: usize,
) -> std::result::Result<(WireItem, String, usize), &'static str> {
    let render = |truncation: &'static str, body: &str| {
        let item = WireItem {
            object_id: object_id.to_string(),
            kind: kind.to_string(),
            revision_id: revision_id.to_string(),
            truncation,
            content: body.to_string(),
        };
        let line = serde_json::to_string(&item).expect("WireItem always serializes");
        (item, line)
    };

    let (_, empty_rendered) = render("full", "");
    if empty_rendered.len() >= remaining {
        return Err("budget: envelope does not fit");
    }

    let mut body = content.to_string();
    let mut truncation = "full";
    loop {
        let (item, line) = render(truncation, &body);
        if line.len() <= remaining {
            let len = line.len();
            return Ok((item, line, len));
        }
        let excess = line.len() - remaining;
        let mut cut = body.len().saturating_sub(excess.max(1));
        while cut > 0 && !body.is_char_boundary(cut) {
            cut -= 1;
        }
        if cut == 0 && body.is_empty() {
            return Err("budget: no room for any content");
        }
        body.truncate(cut);
        truncation = "truncated";
    }
}

/// Compile a disclosure package for `grant_id`. Requires a mutable store
/// (the receipt commit is itself a canonical write) even though every read
/// in this function is otherwise read-only.
fn load_usable_grant(store: &CanonicalStore, grant_id: &str) -> Result<(String, ExportGrant)> {
    let (revision_id, payload) = store
        .read_current(grant_id)?
        .ok_or_else(|| Error::Project(format!("no grant exists with id {grant_id}")))?;
    let grant = RecordPayload::from_json(&payload)?
        .as_export_grant()?
        .clone();
    let now_secs = crate::capture::parse_rfc3339_utc(&now_rfc3339_utc())?;
    grant.is_usable_at(now_secs)?;
    Ok((revision_id, grant))
}

/// Compile a disclosure package without persisting a receipt or requiring
/// write access — for a `package-preview` CLI command, and for proving
/// deterministic compilation (repeated calls at the same, unchanged vault
/// state produce byte-identical wire output).
pub fn preview_disclosure_package(
    store: &CanonicalStore,
    grant_id: &str,
    request_id: &str,
    principal_label: &str,
) -> Result<(DisclosureReceipt, String)> {
    if request_id.is_empty() || request_id.len() > MAX_REQUEST_ID_BYTES {
        return Err(Error::Project(format!(
            "request_id must be 1-{MAX_REQUEST_ID_BYTES} bytes, got {}",
            request_id.len()
        )));
    }
    let (grant_revision_id, grant) = load_usable_grant(store, grant_id)?;
    let (wire, receipt) = compile_inner(
        store,
        &grant,
        grant_id,
        &grant_revision_id,
        request_id,
        principal_label,
    )?;
    Ok((receipt, wire))
}

/// Compile a disclosure package for `grant_id` and persist its
/// [`DisclosureReceipt`] before returning the wire bytes. Requires a
/// mutable store (the receipt commit is itself a canonical write) even
/// though every read in this function is otherwise read-only.
pub fn compile_disclosure_package(
    store: &mut CanonicalStore,
    grant_id: &str,
    request_id: &str,
    principal_label: &str,
) -> Result<(DisclosureReceipt, String)> {
    if request_id.is_empty() || request_id.len() > MAX_REQUEST_ID_BYTES {
        return Err(Error::Project(format!(
            "request_id must be 1-{MAX_REQUEST_ID_BYTES} bytes, got {}",
            request_id.len()
        )));
    }
    let (grant_revision_id, grant) = load_usable_grant(store, grant_id)?;

    let (wire, receipt_fields) = compile_inner(
        store,
        &grant,
        grant_id,
        &grant_revision_id,
        request_id,
        principal_label,
    )?;

    let mut writer = store.writer()?;
    let record = RecordPayload::DisclosureReceipt(receipt_fields);
    let payload = record.to_json()?;
    let outcome = writer.commit(crate::canonical::CommandInput {
        command_id: request_id.to_string(),
        actor: principal_label.to_string(),
        origin: RecordOrigin::User,
        target: crate::canonical::CommandTarget::CreateObject { payload },
    })?;
    let _ = outcome;

    let receipt = match record {
        RecordPayload::DisclosureReceipt(r) => r,
        _ => unreachable!(),
    };
    Ok((receipt, wire))
}

/// The read-only half of compilation: select/reject every candidate,
/// render the wire bytes, and build (but do not commit) the receipt.
/// Split out from [`compile_disclosure_package`] so a future preview
/// command can call it without ever touching the write path.
fn compile_inner(
    store: &CanonicalStore,
    grant: &ExportGrant,
    grant_id: &str,
    grant_revision_id: &str,
    request_id: &str,
    principal_label: &str,
) -> Result<(String, DisclosureReceipt)> {
    let (head_seq, _) = store.transaction_head()?;
    let as_of_valid = now_rfc3339_utc();
    let candidates = gather_candidates(store, &grant.project_id, head_seq)?;
    let budget = grant.byte_budget as usize;

    let header = WireHeader {
        request_id: request_id.to_string(),
        grant_id: grant_id.to_string(),
        project_id: grant.project_id.clone(),
        principal_label: principal_label.to_string(),
        as_of_recorded: head_seq,
        as_of_valid: as_of_valid.clone(),
        compiler_version: COMPILER_VERSION,
        policy_version: grant.policy_version,
    };
    let header_line = serde_json::to_string(&header)
        .map_err(|e| Error::Project(format!("cannot serialize disclosure header: {e}")))?;
    // The header line's own trailing newline is reserved up front — it is
    // never optional, so it is not measured per-item the way each
    // subsequent line's own newline is.
    if header_line.len() + 1 > budget {
        return Err(Error::Project(format!(
            "grant byte_budget ({budget} bytes) is too small even for an empty package (header alone is {} bytes)",
            header_line.len() + 1
        )));
    }

    let mut selected = Vec::new();
    let mut rejected = Vec::new();
    let mut wire_lines: Vec<String> = vec![header_line.clone()];
    let mut used = header_line.len() + 1;
    let mut selected_ids: HashSet<String> = HashSet::new();

    for c in &candidates {
        if !grant.covers(c.kind, &c.object_id) {
            rejected.push(RejectedItem {
                object_id: c.object_id.clone(),
                kind: c.kind.to_string(),
                reason: reject_reason_for_scope(grant, c.kind, &c.object_id),
            });
            continue;
        }
        // Reserve one byte for this line's own trailing newline, so
        // `fit_to_budget`'s "does it fit in `remaining`" check is exact.
        let remaining = budget.saturating_sub(used).saturating_sub(1);
        match fit_to_budget(&c.object_id, c.kind, &c.revision_id, &c.content, remaining) {
            Ok((item, line, len)) => {
                used += len + 1;
                selected_ids.insert(c.object_id.clone());
                selected.push(SelectedItem {
                    object_id: c.object_id.clone(),
                    kind: c.kind.to_string(),
                    revision_id: c.revision_id.clone(),
                    truncation: if item.truncation == "full" {
                        ItemTruncation::Full
                    } else {
                        ItemTruncation::Truncated
                    },
                });
                wire_lines.push(line);
            }
            Err(reason) => rejected.push(RejectedItem {
                object_id: c.object_id.clone(),
                kind: c.kind.to_string(),
                reason: reason.to_string(),
            }),
        }
    }

    // Relations: only after every non-relation item is decided, and only
    // if both endpoints were actually selected — never reveal a relation
    // naming an object the recipient cannot otherwise see.
    let mut relations = crate::relation::list_project_relations(store, &grant.project_id)?;
    relations.sort_by(|a, b| a.0.cmp(&b.0));
    for (object_id, r) in &relations {
        if !grant.covers("relation", object_id) {
            rejected.push(RejectedItem {
                object_id: object_id.clone(),
                kind: "relation".to_string(),
                reason: reject_reason_for_scope(grant, "relation", object_id),
            });
            continue;
        }
        if !selected_ids.contains(&r.from_object_id) || !selected_ids.contains(&r.to_object_id) {
            rejected.push(RejectedItem {
                object_id: object_id.clone(),
                kind: "relation".to_string(),
                reason: "one or both endpoints were not disclosed in this package".to_string(),
            });
            continue;
        }
        let (revision_id, _) = store.read_current(object_id)?.ok_or_else(|| {
            Error::Project(format!("relation {object_id} disappeared mid-compile"))
        })?;
        let remaining = budget.saturating_sub(used).saturating_sub(1);
        match fit_to_budget(
            object_id,
            "relation",
            &revision_id,
            &relation_content(r),
            remaining,
        ) {
            Ok((item, line, len)) => {
                used += len + 1;
                selected.push(SelectedItem {
                    object_id: object_id.clone(),
                    kind: "relation".to_string(),
                    revision_id,
                    truncation: if item.truncation == "full" {
                        ItemTruncation::Full
                    } else {
                        ItemTruncation::Truncated
                    },
                });
                wire_lines.push(line);
            }
            Err(reason) => rejected.push(RejectedItem {
                object_id: object_id.clone(),
                kind: "relation".to_string(),
                reason: reason.to_string(),
            }),
        }
    }

    selected.sort_by(|a, b| a.object_id.cmp(&b.object_id));
    rejected.sort_by(|a, b| a.object_id.cmp(&b.object_id));
    // `wire_lines[0]` is the header; every subsequent line was appended in
    // selection order, not sorted — sorting would require re-deriving
    // which line belongs to which object_id, and selection order is
    // already deterministic (candidates were gathered in
    // `(group_rank, object_id)` order, relations in `object_id` order).
    let wire = wire_lines.join("\n") + "\n";
    if wire.len() > budget {
        // Structurally should be unreachable (every accepted line was
        // measured, including its own newline, against the remaining
        // budget as it was added), but checked explicitly rather than
        // trusted, since this is the exact property §18's byte cap exists
        // to guarantee.
        return Err(Error::Project(format!(
            "internal error: compiled package ({} bytes) exceeds its own budget ({budget} bytes)",
            wire.len()
        )));
    }
    let emitted_sha256 = hash_bytes(wire.as_bytes());

    let receipt = DisclosureReceipt {
        payload_schema_version: project::RECORD_PAYLOAD_SCHEMA_VERSION,
        request_id: request_id.to_string(),
        grant_id: grant_id.to_string(),
        grant_revision_id: grant_revision_id.to_string(),
        project_id: grant.project_id.clone(),
        principal_label: principal_label.to_string(),
        as_of_recorded: head_seq,
        as_of_valid,
        selected,
        rejected,
        compiler_version: COMPILER_VERSION,
        policy_version: grant.policy_version,
        emitted_byte_count: wire.len() as u64,
        emitted_sha256,
        created_at: now_rfc3339_utc(),
        unknown: JsonMap::new(),
    };
    Ok((wire, receipt))
}

fn reject_reason_for_scope(grant: &ExportGrant, kind: &str, object_id: &str) -> String {
    if grant.privacy_exclusions.iter().any(|x| x == object_id) {
        return "excluded by grant: privacy exclusion".to_string();
    }
    let kind_ok = grant.allowed_kinds.is_empty() || grant.allowed_kinds.iter().any(|k| k == kind);
    if !kind_ok {
        return format!("excluded by grant: kind {kind:?} not in allowed_kinds");
    }
    "excluded by grant: not in allowed_object_ids".to_string()
}

pub fn current_receipt(store: &CanonicalStore, receipt_id: &str) -> Result<DisclosureReceipt> {
    let (_, payload) = store
        .read_current(receipt_id)?
        .ok_or_else(|| Error::Project(format!("no receipt exists with id {receipt_id}")))?;
    Ok(RecordPayload::from_json(&payload)?
        .as_disclosure_receipt()?
        .clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!("fehrest-disclosure-{}", uuid::Uuid::now_v7()))
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

    fn make_grant(
        store: &mut CanonicalStore,
        project_id: &str,
        allowed_kinds: &[String],
        privacy_exclusions: &[String],
        byte_budget: u32,
    ) -> String {
        let mut writer = store.writer().unwrap();
        let (outcome, _g) = crate::grant::issue_grant(
            &mut writer,
            "owner",
            project_id,
            allowed_kinds,
            None,
            privacy_exclusions,
            byte_budget,
            3600,
        )
        .unwrap();
        outcome.object_id
    }

    #[test]
    fn a_clean_note_and_decision_are_fully_disclosed() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, Some("t"), "b").unwrap();
            let (outcome, _) = project::create_decision(
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
            .unwrap();
            project::accept_decision(
                &mut writer,
                "owner",
                &outcome.object_id,
                &outcome.revision_id,
            )
            .unwrap();
        }
        let grant_id = make_grant(&mut store, &project_id, &[], &[], 65536);

        let (receipt, wire) =
            compile_disclosure_package(&mut store, &grant_id, "req-1", "agent-x").unwrap();
        assert_eq!(receipt.selected.len(), 2);
        assert!(receipt.rejected.is_empty());
        assert!(wire.contains("\"kind\":\"note\""));
        assert!(wire.contains("\"kind\":\"decision\""));
        assert_eq!(receipt.emitted_sha256, hash_bytes(wire.as_bytes()));
        assert_eq!(receipt.emitted_byte_count as usize, wire.len());
        cleanup(&root);
    }

    #[test]
    fn draft_only_decision_key_is_never_a_candidate() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        {
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
            .unwrap();
        }
        let grant_id = make_grant(&mut store, &project_id, &[], &[], 65536);
        let (receipt, _wire) =
            compile_disclosure_package(&mut store, &grant_id, "req-1", "agent-x").unwrap();
        assert!(receipt.selected.is_empty());
        assert!(
            receipt.rejected.is_empty(),
            "a draft is not even a rejected candidate"
        );
        cleanup(&root);
    }

    #[test]
    fn privacy_exclusion_is_rejected_with_a_clear_reason_never_disclosed() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let note_id = {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, None, "secret body xyz")
                .unwrap()
                .0
                .object_id
        };
        let grant_id = make_grant(
            &mut store,
            &project_id,
            &[],
            std::slice::from_ref(&note_id),
            65536,
        );
        let (receipt, wire) =
            compile_disclosure_package(&mut store, &grant_id, "req-1", "agent-x").unwrap();
        assert!(receipt.selected.is_empty());
        assert_eq!(receipt.rejected.len(), 1);
        assert_eq!(receipt.rejected[0].object_id, note_id);
        assert!(receipt.rejected[0].reason.contains("privacy exclusion"));
        assert!(!wire.contains("secret body xyz"));
        cleanup(&root);
    }

    #[test]
    fn mixed_project_fixture_never_discloses_the_other_project() {
        let (root, mut store) = open_store();
        let project_a = make_project(&mut store);
        let project_b = make_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_a, None, "project A secret")
                .unwrap();
            project::create_note(&mut writer, "owner", &project_b, None, "project B secret")
                .unwrap();
        }
        let grant_a = make_grant(&mut store, &project_a, &[], &[], 65536);
        let (receipt, wire) =
            compile_disclosure_package(&mut store, &grant_a, "req-1", "agent-x").unwrap();
        assert_eq!(receipt.selected.len(), 1);
        assert!(wire.contains("project A secret"));
        assert!(!wire.contains("project B secret"));
        cleanup(&root);
    }

    #[test]
    fn kind_restricted_grant_excludes_other_kinds() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_action(&mut writer, "owner", &project_id, "an action", None, &[])
                .unwrap();
            project::create_note(&mut writer, "owner", &project_id, None, "a note").unwrap();
        }
        let grant_id = make_grant(&mut store, &project_id, &["note".to_string()], &[], 65536);
        let (receipt, wire) =
            compile_disclosure_package(&mut store, &grant_id, "req-1", "agent-x").unwrap();
        assert_eq!(receipt.selected.len(), 1);
        assert_eq!(receipt.selected[0].kind, "note");
        assert_eq!(receipt.rejected.len(), 1);
        assert_eq!(receipt.rejected[0].kind, "action");
        assert!(!wire.contains("an action"));
        cleanup(&root);
    }

    #[test]
    fn tiny_budget_omits_items_with_a_budget_reason_never_a_partial_leak() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, None, &"x".repeat(10_000))
                .unwrap();
        }
        // Enough room for the header line but not for even this one note's
        // own empty envelope.
        let grant_id = make_grant(&mut store, &project_id, &[], &[], 260);
        let (receipt, wire) =
            compile_disclosure_package(&mut store, &grant_id, "req-1", "agent-x").unwrap();
        assert!(receipt.selected.is_empty());
        assert_eq!(receipt.rejected.len(), 1);
        assert!(receipt.rejected[0].reason.contains("budget"));
        assert!(wire.len() <= 260);
        cleanup(&root);
    }

    #[test]
    fn truncation_never_exceeds_the_byte_budget() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, None, &"y".repeat(5_000))
                .unwrap();
        }
        let grant_id = make_grant(&mut store, &project_id, &[], &[], 500);
        let (receipt, wire) =
            compile_disclosure_package(&mut store, &grant_id, "req-1", "agent-x").unwrap();
        assert_eq!(receipt.selected.len(), 1);
        assert_eq!(receipt.selected[0].truncation, ItemTruncation::Truncated);
        assert!(wire.len() <= 500);
        cleanup(&root);
    }

    #[test]
    fn relation_with_an_undisclosed_endpoint_is_rejected_not_leaked() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (note_id, action_id) = {
            let mut writer = store.writer().unwrap();
            let note_id = project::create_note(&mut writer, "owner", &project_id, None, "n")
                .unwrap()
                .0
                .object_id;
            let action_id =
                project::create_action(&mut writer, "owner", &project_id, "a", None, &[])
                    .unwrap()
                    .0
                    .object_id;
            (note_id, action_id)
        };
        {
            let mut writer = store.writer().unwrap();
            crate::relation::create_relation(
                &mut writer,
                "owner",
                &project_id,
                crate::relation::RelationType::RelatesTo,
                &note_id,
                &action_id,
                None,
            )
            .unwrap();
        }
        // Exclude the action kind entirely: the relation's "to" endpoint is
        // never disclosed, so the relation itself must not be either.
        let grant_id = make_grant(
            &mut store,
            &project_id,
            &["note".to_string(), "relation".to_string()],
            &[],
            65536,
        );
        let (receipt, wire) =
            compile_disclosure_package(&mut store, &grant_id, "req-1", "agent-x").unwrap();
        assert!(receipt.selected.iter().all(|s| s.kind != "relation"));
        let rel_reject = receipt
            .rejected
            .iter()
            .find(|r| r.kind == "relation")
            .expect("relation must appear as rejected, not silently absent");
        assert!(rel_reject.reason.contains("endpoint"));
        assert!(!wire.contains("relates_to"));
        cleanup(&root);
    }

    #[test]
    fn a_budget_too_small_for_even_the_header_is_a_clear_top_level_error() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let grant_id = make_grant(&mut store, &project_id, &[], &[], 10);
        let err =
            compile_disclosure_package(&mut store, &grant_id, "req-1", "agent-x").unwrap_err();
        assert!(format!("{err}").contains("too small even for an empty package"));
        cleanup(&root);
    }

    #[test]
    fn revoked_grant_refuses_compilation() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        let (grant_id, rev) = {
            let mut writer = store.writer().unwrap();
            let (outcome, _g) = crate::grant::issue_grant(
                &mut writer,
                "owner",
                &project_id,
                &[],
                None,
                &[],
                65536,
                3600,
            )
            .unwrap();
            (outcome.object_id, outcome.revision_id)
        };
        crate::grant::revoke_grant(&mut store, "owner", &grant_id, &rev).unwrap();
        let err =
            compile_disclosure_package(&mut store, &grant_id, "req-1", "agent-x").unwrap_err();
        assert!(format!("{err}").contains("revoked"));
        cleanup(&root);
    }

    #[test]
    fn repeated_identical_request_id_yields_the_same_receipt() {
        // Compilation is deterministic: calling `preview_disclosure_package`
        // (read-only, no state change) twice in a row at the same vault
        // state produces byte-identical output. This is the meaningful
        // idempotency property for a *compiler*; genuine crash/retry
        // reconciliation of a committed command is `CanonicalWriter::
        // commit`'s own already-proven mechanism (`T01-03`), reused here
        // unchanged via `compile_disclosure_package`'s `command_id`
        // rather than re-tested from scratch — a real retry after a
        // successful-but-unacknowledged commit cannot be simulated by
        // calling `compile_disclosure_package` twice in the same process,
        // since the first call's own receipt commit legitimately advances
        // `as_of_recorded` for any second call that observes it.
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, None, "n").unwrap();
        }
        let grant_id = make_grant(&mut store, &project_id, &[], &[], 65536);
        let (receipt1, wire1) =
            preview_disclosure_package(&store, &grant_id, "same-req", "agent-x").unwrap();
        let (receipt2, wire2) =
            preview_disclosure_package(&store, &grant_id, "same-req", "agent-x").unwrap();
        assert_eq!(receipt1.emitted_sha256, receipt2.emitted_sha256);
        assert_eq!(wire1, wire2);
        cleanup(&root);
    }

    #[test]
    fn full_receipt_replay_verifies_exact_bytes() {
        let (root, mut store) = open_store();
        let project_id = make_project(&mut store);
        {
            let mut writer = store.writer().unwrap();
            project::create_note(&mut writer, "owner", &project_id, None, "n").unwrap();
        }
        let grant_id = make_grant(&mut store, &project_id, &[], &[], 65536);
        let (receipt, wire) =
            compile_disclosure_package(&mut store, &grant_id, "req-1", "agent-x").unwrap();

        let reread = current_receipt(&store, &{
            // Find the receipt's own object_id: the sole export_grant-free
            // new object created by compile_disclosure_package beyond the
            // grant itself.
            store
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
                .unwrap()
        })
        .unwrap();
        assert_eq!(reread, receipt);
        assert_eq!(reread.emitted_sha256, hash_bytes(wire.as_bytes()));
        assert_eq!(reread.emitted_byte_count as usize, wire.len());
        cleanup(&root);
    }
}
