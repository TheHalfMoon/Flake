// Mirrors the Rust DTOs in desktop/src-tauri/src/commands.rs and the
// Core types they wrap (fehrest::project/relation/decision_state/resume).
// Field names are snake_case exactly as Core's own serde derives produce
// them (no `rename_all = "camelCase"` on those structs) -- only Tauri
// *command argument* names are camelCase (Tauri's own JS<->Rust arg-name
// convention), which is unrelated to how a returned struct's own fields
// are cased.

export interface ProjectSummary {
  id: string;
  name: string;
  description: string | null;
  active: boolean;
}

export type ActionState = "open" | "doing" | "blocked" | "done" | "cancelled";

export interface ActionSummary {
  id: string;
  project_id: string;
  title: string;
  body: string | null;
  state: ActionState;
  dependency_ids: string[];
  completion_summary: string | null;
  revision_id: string;
}

export type DecisionBasis = "evidence" | "user_judgment" | "agent_proposal";
export type DecisionVerification = "unreviewed" | "user_reviewed";
export type DecisionLifecycle = "draft" | "accepted" | "superseded" | "withdrawn";

export interface DecisionSummary {
  id: string;
  project_id: string;
  decision_key: string;
  statement: string;
  rationale: string | null;
  basis: DecisionBasis;
  verification: DecisionVerification;
  lifecycle: DecisionLifecycle;
  valid_from: string | null;
  valid_to: string | null;
  revision_id: string;
}

export interface SupersedeResult {
  relation_id: string;
  old_decision: DecisionSummary;
}

export type RelationType = "supports" | "contradicts" | "depends_on" | "relates_to" | "supersedes";

export interface RelationEntry {
  id: string;
  project_id: string;
  relation_type: RelationType;
  from_object_id: string;
  to_object_id: string;
  note: string | null;
}

export interface SearchHit {
  kind: "note" | "action" | "decision";
  id: string;
  title: string | null;
  snippet: string;
}

export type DecisionOutcome = "needs_review" | "no_accepted_decision" | { current_set: string };

export interface ConsideredDecision {
  decision_id: string;
  // Core's own `Decision`, read as of a cutoff -- opaque here, not re-typed.
  decision: Record<string, unknown>;
  admitted: boolean;
  exclusion_reason: string | null;
}

export interface DecisionResolution {
  project_id: string;
  decision_key: string;
  as_of_valid: string;
  as_of_recorded: number;
  outcome: DecisionOutcome;
  considered: ConsideredDecision[];
}

export type CheckStatus = "match" | "changed" | "missing" | "denied";

export interface StaleEvidence {
  source_id: string;
  latest_check: { status: CheckStatus } & Record<string, unknown>;
}

export interface ChangeSummary {
  recorded_seq: number;
  object_id: string;
  kind: string;
}

export interface ResumeView {
  project_id: string;
  reviewed_through_seq: number | null;
  head_seq: number;
  conflicts: DecisionResolution[];
  stale_or_missing_evidence: StaleEvidence[];
  current_decisions: DecisionResolution[];
  // Core's own `Action`/`Note`, read live -- opaque here, not re-typed.
  next_actions: [string, Record<string, unknown>][];
  relevant_notes: [string, Record<string, unknown>][];
  changes_since_checkpoint: ChangeSummary[];
}

export function outcomeLabel(outcome: DecisionOutcome): string {
  if (outcome === "needs_review") return "Needs review (conflicting decisions)";
  if (outcome === "no_accepted_decision") return "No accepted decision";
  return `Current: ${outcome.current_set}`;
}

export interface CheckpointInfo {
  id: string;
  project_id: string;
  owner: string;
  reviewed_through_seq: number;
  reset_reason: string | null;
  revision_id: string;
}

export type SourceCheckStatus = "match" | "changed" | "missing" | "denied";

export interface SourceEntry {
  id: string;
  label: string;
  active: boolean;
  latest_check_status: SourceCheckStatus | null;
  latest_check_at: string | null;
}

export interface SourceCheck {
  source_id: string;
  status: SourceCheckStatus;
  observed_at: string;
}

export type GrantState = "active" | "revoked";

export interface GrantEntry {
  id: string;
  project_id: string;
  allowed_kinds: string[];
  allowed_object_ids: string[] | null;
  privacy_exclusions: string[];
  byte_budget: number;
  expires_at: string;
  state: GrantState;
  issued_at: string;
  revision_id: string;
}

export interface SelectedItem {
  object_id: string;
  kind: string;
}

export interface RejectedItem {
  object_id: string;
  kind: string;
  reason: string;
}

export interface DisclosureReceipt {
  request_id: string;
  grant_id: string;
  project_id: string;
  principal_label: string;
  selected: SelectedItem[];
  rejected: RejectedItem[];
  emitted_byte_count: number;
  emitted_sha256: string;
  created_at: string;
}

export interface PackagePreviewResult {
  receipt: DisclosureReceipt;
  wire: string;
}

export interface PackageCompileResult {
  receipt_id: string;
  receipt: DisclosureReceipt;
  wire: string;
}

export type ProposalStatus = "pending" | "accepted" | "rejected" | "expired";

export type ProposedOperation =
  | { kind: "note_edit"; note_id: string; expected_revision_id: string; title: string | null; body: string }
  | { kind: "draft_decision"; decision_key: string; statement: string; rationale: string | null }
  | {
      kind: "evidence_relation";
      relation_type: RelationType;
      from_object_id: string;
      to_object_id: string;
      note: string | null;
    }
  | { kind: "complete_action"; action_id: string; expected_revision_id: string; summary: string };

export interface ManifestMember {
  path: string;
  length: number;
  sha256: string;
}

export interface BackupManifest {
  schema: string;
  kind: string;
  vault_id: string;
  snapshot_head_seq: number;
  snapshot_head_hash: string | null;
  created_at: string;
  members: ManifestMember[];
  verified: boolean;
}

export interface BackupReport {
  source_root: string;
  backup_root: string;
  manifest: BackupManifest;
}

export interface RestoreReport {
  backup_root: string;
  restored_root: string;
  vault_id: string;
  restored_transaction_head_seq: number;
  restored_object_count: number;
}

export interface RecoveryReport {
  original_root: string;
  recovered_root: string;
  vault_id: string;
  preserved_at: string;
  verified_transaction_head_seq: number;
}

export interface ExportManifest {
  schema: string;
  kind: string;
  vault_id: string;
  project_id: string | null;
  snapshot_head_seq: number;
  snapshot_head_hash: string | null;
  created_at: string;
  record_count: number;
  revision_count: number;
  omissions: string[];
  members: ManifestMember[];
  integrity_root: string;
}

export interface ExportReport {
  dest_root: string;
  manifest: ExportManifest;
}

export interface ExportPreview {
  kind: string;
  project_id: string | null;
  record_count: number;
  revision_count: number;
  snapshot_head_seq: number;
}

export interface ImportPreview {
  kind: string;
  source_vault_id: string;
  project_id: string | null;
  record_count: number;
  revision_count: number;
  conflicts: string[];
}

export interface ImportReport {
  dest_root: string;
  mode: string;
  imported_object_count: number;
  imported_revision_count: number;
  id_map: Record<string, string>;
}

export interface ProposalEntry {
  id: string;
  project_id: string;
  receipt_id: string;
  declared_agent: string | null;
  declared_model: string | null;
  declared_tool: string | null;
  inbound_sha256: string;
  inbound_byte_count: number;
  operations: ProposedOperation[];
  status: ProposalStatus;
  accepted_operation_indices: number[];
  review_reason: string | null;
  reviewed_by: string | null;
  reviewed_at: string | null;
  submitted_at: string;
  revision_id: string;
}

export interface AboutInfo {
  version: string;
  license_spdx: string;
  license_file: string;
  notice_file: string;
  third_party_licenses_file: string;
  source_url: string;
  support_url: string;
  privacy_statement: string;
}
