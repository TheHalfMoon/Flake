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
