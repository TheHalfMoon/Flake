//! Minimal headless CLI. Hand dispatch — ten subcommands do not justify a
//! command-line framework and its proc-macro tree (Ponytail DELETE: `clap`).

use crate::canonical::CanonicalStore;
use crate::capture;
use crate::checkpoint;
use crate::context::{self, CompileRequest, SourceItem};
use crate::decision_state;
use crate::derived::Derived;
use crate::disclosure;
use crate::envelope::TrustLevel;
use crate::events::{ChainStatus, EventKind, EventLog};
use crate::export;
use crate::grant;
use crate::import;
use crate::index;
use crate::markdown;
use crate::memory::Scope;
use crate::project::{self, DecisionBasis, DecisionVerification};
use crate::proposal;
use crate::relation::{self, RelationType};
use crate::resume;
use crate::source_check;
use crate::vault::Vault;
use crate::{limits, Error, Result};
use std::path::PathBuf;

/// Split a `--depends-on a,b,c`-style flag into individual IDs, trimming
/// whitespace and dropping empty entries (so a bare `--depends-on` or a
/// trailing comma means "no dependencies", not one empty-string ID).
fn split_ids(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

fn parse_basis(s: &str) -> Result<DecisionBasis> {
    match s {
        "evidence" => Ok(DecisionBasis::Evidence),
        "user-judgment" => Ok(DecisionBasis::UserJudgment),
        "agent-proposal" => Ok(DecisionBasis::AgentProposal),
        other => Err(Error::Project(format!(
            "unknown --basis {other} (expected evidence|user-judgment|agent-proposal)"
        ))),
    }
}

fn parse_verification(s: &str) -> Result<DecisionVerification> {
    match s {
        "unreviewed" => Ok(DecisionVerification::Unreviewed),
        "user-reviewed" => Ok(DecisionVerification::UserReviewed),
        other => Err(Error::Project(format!(
            "unknown --verification {other} (expected unreviewed|user-reviewed)"
        ))),
    }
}

fn parse_relation_type(s: &str) -> Result<RelationType> {
    match s {
        "supports" => Ok(RelationType::Supports),
        "contradicts" => Ok(RelationType::Contradicts),
        "depends_on" => Ok(RelationType::DependsOn),
        "relates_to" => Ok(RelationType::RelatesTo),
        "supersedes" => Ok(RelationType::Supersedes),
        other => Err(Error::Project(format!(
            "unknown --type {other} (expected supports|contradicts|depends_on|relates_to|supersedes)"
        ))),
    }
}

/// Every format-2 CLI command runs as this fixed principal (T02-01: no
/// authentication concept exists in this headless CLI). A future task that
/// adds real actor identity replaces this constant at one call site each,
/// not a redesign.
const CLI_ACTOR: &str = "owner";

pub const USAGE: &str = "\
fehrest — Phase T headless thesis-proof (EXPERIMENTAL, not a product)

USAGE:
  fehrest <command> --vault <path> [options]

COMMANDS:
  init              Create a vault
  add               Add a canonical object      --path <rel> --body <text> [--title T] [--project P]
  scan              List admitted objects, conflicts and exclusions
  rebuild           Rebuild derived state from canonical state
  search            Lexical candidate search     --query <text> [--limit N]
  read              Read an object by id         --id <uuid>
  compile           Compile a context package    [--project P] [--budget BYTES] [--as-of DAY]
  manifest          Show the last package manifest
  events            Show the event log
  verify            Verify the event chain

FORMAT-2 COMMANDS (T02-01; --vault names a separate format-2 store root):
  canonical-init    Create a format-2 store
  project-create    Create a project             --name N [--description D]
  project-archive   Archive a project             --id <uuid>
  project-unarchive Unarchive a project           --id <uuid>
  project-show      Show a project                --id <uuid>
  note-create       Create a note                 --project <uuid> --body T [--title T]
  note-update       Replace a note's title/body   --id <uuid> --expect <revision-uuid> --body T [--title T]
  action-create     Create an action              --project <uuid> --title T [--body T] [--depends-on id1,id2]
  decision-create   Create a decision             --project <uuid> --key K --statement T [--rationale T] [--basis evidence|user-judgment|agent-proposal] [--verification unreviewed|user-reviewed] [--valid-from TS] [--valid-to TS]
  record-show       Show any typed record          --id <uuid> [--preview N]
  project-records   List a project's work records --project <uuid>

FORMAT-2 CAPTURE COMMANDS (T02-02):
  capture           Default-Note capture           --project <uuid> --body T [--kind note|action|decision] [--title T] [--key K] [--rationale R]
  source-import     Import one selected file       --project <uuid> --label L --path <local-path>
  source-reference  Register a manual reference     --project <uuid> --label L [--repository R] [--commit C] [--path P]
  source-deactivate Mark a source unavailable       --id <uuid>
  source-reactivate Mark a source available again   --id <uuid>
  source-extract    Recover a source's exact bytes  --id <uuid> --out <local-path>
  project-sources   List a project's sources        --project <uuid>

FORMAT-2 DECISION/ACTION/RELATION COMMANDS (T02-03):
  action-start          Move an action to Doing         --id <uuid> --expect <revision-uuid>
  action-block          Move an action to Blocked        --id <uuid> --expect <revision-uuid> [--reason R]
  action-complete       Complete an action                --id <uuid> --expect <revision-uuid> --summary T [--override-reason R]
  action-cancel         Cancel an action                   --id <uuid> --expect <revision-uuid> [--reason R]
  action-reopen         Reopen a done/cancelled action   --id <uuid> --expect <revision-uuid> --reason R
  action-set-dependencies  Replace an action's dependency list --id <uuid> --expect <revision-uuid> [--depends-on id1,id2]
  decision-accept       Owner-accept a draft decision      --id <uuid> --expect <revision-uuid>
  decision-withdraw     Withdraw a draft/accepted decision  --id <uuid> --expect <revision-uuid> --reason R
  decision-supersede    Supersede an accepted decision      --new <uuid> --old <uuid> --expect <old-revision-uuid> --reason R
  relation-create       Link two records                   --project <uuid> --type supports|contradicts|depends_on|relates_to|supersedes --from <uuid> --to <uuid> [--note N]
  object-relations      List relations touching an object  --id <uuid>
  project-relations     List a project's relations        --project <uuid>

FORMAT-2 SEARCH INDEX COMMANDS (T02-04):
  fts-rebuild       Full rebuild of the derived search index  --vault <path>
  fts-update        Incremental update (rebuilds if none exists yet) --vault <path>
  fts-status        Show the index's own checkpoint            --vault <path>
  fts-search        Search Notes/Actions/Decisions              --query T [--project <uuid>] [--limit N]

FORMAT-2 EXPORT COMMANDS (T02-05):
  export-preview    Show scope/counts without writing       [--project <uuid>]
  export-run        Write a portable export package         --out <path> [--project <uuid>]

FORMAT-2 IMPORT COMMANDS (T02-06; --source names a published export root):
  import-preview       Validate a package, show scope/conflicts    --source <path>
  import-full-restore  Import into a brand-new empty vault          --source <path> --vault <path>
  import-merge          Import into this --vault, new identities    --source <path>

FORMAT-2 SOURCE-CHECK COMMANDS (T03-01):
  source-check          Recheck a file-backed source's bytes        --id <uuid>
  source-checks          Show one source's full check history        --id <uuid>
  project-source-checks  List a project's check history              --project <uuid>
  source-reselect        Record a move; refuses if content changed  --id <uuid> --expect <revision-uuid> --path <new-local-path>
  source-admit-change    Admit changed bytes as a new revision       --id <uuid> --expect <revision-uuid> [--path <new-local-path>]

FORMAT-2 TEMPORAL RESOLUTION COMMANDS (T03-02):
  decision-state    Resolve a decision key's current accepted state, with reasons and negative evidence for every candidate     --project <uuid> --key K [--as-of-valid TS] [--as-of-recorded N]

FORMAT-2 RESUME/CHECKPOINT COMMANDS (T03-03):
  resume              Show conflicts, stale evidence, current decisions, next actions, notes and changes since checkpoint    --project <uuid>
  checkpoint-mark     Mark reviewed through a sequence (default: current head); --expect required after the first mark     --project <uuid> [--expect <revision-uuid>] [--through N]
  checkpoint-reset    Explicitly move the checkpoint backward with a reason                                                  --project <uuid> --expect <revision-uuid> --through N --reason R
  checkpoint-history  Show every checkpoint revision for a project                                                           --project <uuid>

FORMAT-2 DISCLOSURE COMMANDS (T03-04):
  grant-issue      Issue an owner-scoped export grant                                            --project <uuid> [--kinds note,action,...] [--ids id1,id2] [--exclude id1,id2] [--budget N] [--ttl-hours N]
  grant-revoke     Revoke an active grant                                                          --id <uuid> --expect <revision-uuid>
  package-preview  Compile a disclosure package without persisting a receipt                       --grant <uuid> --request-id R [--principal P]
  package-export   Compile, persist the receipt, and write the package to a staged file            --grant <uuid> --request-id R --out <path> [--principal P]

FORMAT-2 AGENT PROPOSAL COMMANDS (T03-05):
  propose-import      Admit a bounded inbound proposal file as Pending          --project <uuid> --file <path>
  propose-show        Show one proposal's exact operations and review state     --id <uuid>
  project-proposals   List a project's proposals                                --project <uuid>
  propose-accept      Apply selected operation indices and mark Accepted        --id <uuid> --expect <revision-uuid> --select i1,i2,...
  propose-reject      Reject a pending proposal with a reason                   --id <uuid> --expect <revision-uuid> --reason R
  propose-expire      Mark a pending proposal Expired with a reason             --id <uuid> --expect <revision-uuid> --reason R
";

struct Args {
    vault: Option<PathBuf>,
    flags: std::collections::HashMap<String, String>,
}

fn parse_args(argv: &[String]) -> Args {
    let mut flags = std::collections::HashMap::new();
    let mut i = 0;
    while i < argv.len() {
        if let Some(name) = argv[i].strip_prefix("--") {
            let value = argv.get(i + 1).cloned().unwrap_or_default();
            if value.starts_with("--") {
                flags.insert(name.to_string(), String::new());
                i += 1;
            } else {
                flags.insert(name.to_string(), value);
                i += 2;
            }
        } else {
            i += 1;
        }
    }
    Args {
        vault: flags.get("vault").map(PathBuf::from),
        flags,
    }
}

impl Args {
    fn get(&self, k: &str) -> Option<&str> {
        self.flags
            .get(k)
            .map(String::as_str)
            .filter(|s| !s.is_empty())
    }
    fn require(&self, k: &str) -> Result<&str> {
        self.get(k)
            .ok_or_else(|| crate::Error::Vault(format!("missing required --{k}")))
    }
    fn vault_root(&self) -> Result<&PathBuf> {
        self.vault
            .as_ref()
            .ok_or_else(|| crate::Error::Vault("missing required --vault".into()))
    }
}

pub fn run(argv: &[String]) -> Result<i32> {
    if argv.is_empty() || argv[0] == "--help" || argv[0] == "-h" {
        println!("{USAGE}");
        return Ok(0);
    }
    let cmd = argv[0].as_str();
    let args = parse_args(&argv[1..]);

    match cmd {
        "init" => {
            let root = args.vault_root()?;
            let v = Vault::create(root)?;
            let w = v.writer()?;
            let log = EventLog::open(&v.control_dir())?;
            w.append_event(
                &log,
                EventKind::VaultCreated,
                &root.display().to_string(),
                "",
            )?;
            Derived::open(&v.control_dir())?;
            println!("vault created: {}", root.display());
            Ok(0)
        }

        "add" => {
            let v = Vault::open_write(args.vault_root()?)?;
            let w = v.writer()?;
            let id = w.add_object(
                args.require("path")?,
                args.get("title"),
                args.get("project"),
                args.require("body")?,
            )?;
            let log = EventLog::open(&v.control_dir())?;
            w.append_event(
                &log,
                EventKind::ObjectRegistered,
                &id.to_string(),
                args.require("path")?,
            )?;
            println!("{id}");
            Ok(0)
        }

        "scan" => {
            let v = Vault::open_read(args.vault_root()?)?;
            let scan = v.scan()?;
            println!("objects: {}", scan.objects.len());
            for o in &scan.objects {
                println!("  {} {}", o.id, o.rel_path);
            }
            if !scan.conflicts.is_empty() {
                println!("CONFLICTS (both retained, neither discarded):");
                for (id, paths) in &scan.conflicts {
                    println!("  {id} at {paths:?}");
                }
            }
            if !scan.malformed.is_empty() {
                println!("malformed:");
                for (p, why) in &scan.malformed {
                    println!("  {p}: {why}");
                }
            }
            println!("excluded (unsupported or reserved): {}", scan.skipped.len());
            Ok(if scan.conflicts.is_empty() { 0 } else { 2 })
        }

        "rebuild" => {
            let v = Vault::open_write(args.vault_root()?)?;
            let scan = v.scan()?;
            let d = Derived::open(&v.control_dir())?;
            let n = d.rebuild(&scan.objects)?;
            println!("rebuilt derived index: {n} objects");
            Ok(0)
        }

        "search" => {
            let v = Vault::open_read(args.vault_root()?)?;
            let d = Derived::open(&v.control_dir())?;
            let limit = args
                .get("limit")
                .and_then(|s| s.parse().ok())
                .unwrap_or(20usize);
            for c in d.search(args.require("query")?, limit)? {
                println!("{} {}", c.id, c.rel_path);
            }
            Ok(0)
        }

        "read" => {
            let v = Vault::open_read(args.vault_root()?)?;
            let id = crate::identity::ObjectId::parse(args.require("id")?)?;
            let d = Derived::open(&v.control_dir())?;
            // The index supplies an untrusted locator hint; the read is confined
            // and identity-verified before anything is returned.
            let hits = d.search(args.get("query").unwrap_or(""), limits::MAX_SEARCH_RESULTS)?;
            let hint = hits
                .iter()
                .find(|c| c.id == id)
                .map(|c| c.rel_path.clone())
                .or_else(|| {
                    v.scan().ok().and_then(|s| {
                        s.objects
                            .iter()
                            .find(|o| o.id == id)
                            .map(|o| o.rel_path.clone())
                    })
                })
                .ok_or_else(|| crate::Error::Vault(format!("object not found: {id}")))?;
            let content = crate::locator::read_verified(v.root(), &hint, id)?;
            print!("{content}");
            Ok(0)
        }

        "compile" => {
            let v = Vault::open_write(args.vault_root()?)?;
            let w = v.writer()?;
            let scan = v.scan()?;
            let scope = match args.get("project") {
                Some(p) => Scope::project("vault", p),
                None => Scope::vault_global("vault"),
            };
            let budget = args
                .get("budget")
                .and_then(|s| s.parse().ok())
                .unwrap_or(limits::MAX_PACKAGE_BYTES);
            let as_of = args
                .get("as-of")
                .and_then(|s| s.parse().ok())
                .unwrap_or(i64::MAX);

            let items: Vec<SourceItem> = scan
                .objects
                .iter()
                .map(|o| SourceItem {
                    section: "project_state",
                    item_id: o.id.to_string(),
                    content: o.body.clone(),
                    source_content_hash: o.content_hash.clone(),
                    trust_level: TrustLevel::VaultKnowledge,
                    memory: None,
                    superseded_by: None,
                })
                .collect();

            let req = CompileRequest {
                principal: "agent:cli".into(),
                scope,
                as_of_valid: as_of,
                as_of_recorded: u64::MAX,
                budget_bytes: budget,
            };
            let pkg = context::compile(&req, &items);
            let manifest_path = v.control_dir().join("last-manifest.json");
            std::fs::write(
                &manifest_path,
                serde_json::to_string_pretty(&pkg.manifest)
                    .map_err(|e| crate::Error::Derived(e.to_string()))?,
            )
            .map_err(|e| crate::Error::Vault(format!("cannot write manifest: {e}")))?;

            let log = EventLog::open(&v.control_dir())?;
            w.append_event(
                &log,
                EventKind::ContextCompiled,
                &pkg.manifest.context_id,
                &format!(
                    "items={} omitted={} digest={}",
                    pkg.manifest.entries.len(),
                    pkg.manifest.omissions.len(),
                    pkg.manifest.package_digest
                ),
            )?;
            print!("{}", pkg.wire);
            Ok(0)
        }

        "manifest" => {
            let v = Vault::open_read(args.vault_root()?)?;
            let p = v.control_dir().join("last-manifest.json");
            let text = std::fs::read_to_string(&p)
                .map_err(|e| crate::Error::Vault(format!("no manifest yet: {e}")))?;
            println!("{text}");
            Ok(0)
        }

        "events" => {
            let v = Vault::open_read(args.vault_root()?)?;
            let log = EventLog::open(&v.control_dir())?;
            for e in log.read_all()? {
                println!("{:>4} {:?} {} {}", e.seq, e.kind, e.subject, e.detail);
            }
            Ok(0)
        }

        "verify" => {
            let v = Vault::open_read(args.vault_root()?)?;
            let log = EventLog::open(&v.control_dir())?;
            match log.verify()? {
                ChainStatus::Intact { events } => {
                    println!("chain intact: {events} events");
                    // Stated at the point of use, not only in documentation.
                    println!(
                        "note: unkeyed chain — partial-tamper evidence only, NOT authentication"
                    );
                    Ok(0)
                }
                ChainStatus::Broken { at_seq, reason } => {
                    eprintln!("CHAIN BROKEN at seq {at_seq}: {reason}");
                    Ok(3)
                }
                ChainStatus::Gap { from_seq, to_seq } => {
                    eprintln!("CHAIN GAP: expected {from_seq}, found {to_seq}");
                    Ok(3)
                }
            }
        }

        "canonical-init" => {
            let root = args.vault_root()?;
            let store = CanonicalStore::create(root)?;
            println!(
                "format-2 store created: {} (vault_id {})",
                root.display(),
                store.vault_id()
            );
            Ok(0)
        }

        "project-create" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, project) = project::create_project(
                &mut writer,
                CLI_ACTOR,
                args.require("name")?,
                args.get("description"),
            )?;
            println!("{} {}", outcome.object_id, project.name);
            Ok(0)
        }

        "project-archive" | "project-unarchive" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let id = args.require("id")?;
            let (_, project) = if cmd == "project-archive" {
                project::archive_project(&mut store, CLI_ACTOR, id)?
            } else {
                project::unarchive_project(&mut store, CLI_ACTOR, id)?
            };
            println!("{id} active={}", project.active);
            Ok(0)
        }

        "project-show" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let project = project::open_project(&store, args.require("id")?)?;
            println!("{project:?}");
            Ok(0)
        }

        "note-create" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, _note) = project::create_note(
                &mut writer,
                CLI_ACTOR,
                args.require("project")?,
                args.get("title"),
                args.require("body")?,
            )?;
            println!("{} {}", outcome.object_id, outcome.revision_id);
            Ok(0)
        }

        "note-update" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, _note) = project::update_note(
                &mut writer,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                args.get("title"),
                args.require("body")?,
            )?;
            println!("{} {}", outcome.object_id, outcome.revision_id);
            Ok(0)
        }

        "action-create" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let dependency_ids = args.get("depends-on").map(split_ids).unwrap_or_default();
            let (outcome, _action) = project::create_action(
                &mut writer,
                CLI_ACTOR,
                args.require("project")?,
                args.require("title")?,
                args.get("body"),
                &dependency_ids,
            )?;
            println!("{} {}", outcome.object_id, outcome.revision_id);
            Ok(0)
        }

        "decision-create" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let basis = match args.get("basis") {
                Some(s) => parse_basis(s)?,
                None => DecisionBasis::UserJudgment,
            };
            let verification = match args.get("verification") {
                Some(s) => parse_verification(s)?,
                None => DecisionVerification::Unreviewed,
            };
            let (outcome, _decision) = project::create_decision(
                &mut writer,
                CLI_ACTOR,
                args.require("project")?,
                args.require("key")?,
                args.require("statement")?,
                args.get("rationale"),
                basis,
                verification,
                args.get("valid-from"),
                args.get("valid-to"),
            )?;
            println!("{} {}", outcome.object_id, outcome.revision_id);
            Ok(0)
        }

        "action-start" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, action) = project::start_action(
                &mut writer,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
            )?;
            println!(
                "{} {} state={:?}",
                outcome.object_id, outcome.revision_id, action.state
            );
            Ok(0)
        }

        "action-block" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, action) = project::block_action(
                &mut writer,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                args.get("reason"),
            )?;
            println!(
                "{} {} state={:?}",
                outcome.object_id, outcome.revision_id, action.state
            );
            Ok(0)
        }

        "action-complete" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, action) = project::complete_action(
                &mut writer,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                args.require("summary")?,
                args.get("override-reason"),
            )?;
            println!(
                "{} {} state={:?}",
                outcome.object_id, outcome.revision_id, action.state
            );
            Ok(0)
        }

        "action-cancel" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, action) = project::cancel_action(
                &mut writer,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                args.get("reason"),
            )?;
            println!(
                "{} {} state={:?}",
                outcome.object_id, outcome.revision_id, action.state
            );
            Ok(0)
        }

        "action-reopen" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, action) = project::reopen_action(
                &mut writer,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                args.require("reason")?,
            )?;
            println!(
                "{} {} state={:?}",
                outcome.object_id, outcome.revision_id, action.state
            );
            Ok(0)
        }

        "action-set-dependencies" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let dependency_ids = args.get("depends-on").map(split_ids).unwrap_or_default();
            let (outcome, _action) = project::set_action_dependencies(
                &mut writer,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                &dependency_ids,
            )?;
            println!("{} {}", outcome.object_id, outcome.revision_id);
            Ok(0)
        }

        "decision-accept" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, decision) = project::accept_decision(
                &mut writer,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
            )?;
            println!(
                "{} {} lifecycle={:?}",
                outcome.object_id, outcome.revision_id, decision.lifecycle
            );
            Ok(0)
        }

        "decision-withdraw" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, decision) = project::withdraw_decision(
                &mut writer,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                args.require("reason")?,
            )?;
            println!(
                "{} {} lifecycle={:?}",
                outcome.object_id, outcome.revision_id, decision.lifecycle
            );
            Ok(0)
        }

        "decision-supersede" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let (relation_outcome, update_outcome, old_decision) = project::supersede_decision(
                &mut store,
                CLI_ACTOR,
                args.require("new")?,
                args.require("old")?,
                args.require("expect")?,
                args.require("reason")?,
            )?;
            println!(
                "relation {} {} old-decision {} {} lifecycle={:?}",
                relation_outcome.object_id,
                relation_outcome.revision_id,
                update_outcome.object_id,
                update_outcome.revision_id,
                old_decision.lifecycle
            );
            Ok(0)
        }

        "relation-create" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let relation_type = parse_relation_type(args.require("type")?)?;
            let (outcome, _relation) = relation::create_relation(
                &mut writer,
                CLI_ACTOR,
                args.require("project")?,
                relation_type,
                args.require("from")?,
                args.require("to")?,
                args.get("note"),
            )?;
            println!("{} {}", outcome.object_id, outcome.revision_id);
            Ok(0)
        }

        "object-relations" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let relations = relation::list_relations_for_object(&store, args.require("id")?)?;
            println!("relations: {}", relations.len());
            for (object_id, r) in &relations {
                println!("  {object_id} {r:?}");
            }
            Ok(0)
        }

        "project-relations" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let relations = relation::list_project_relations(&store, args.require("project")?)?;
            println!("relations: {}", relations.len());
            for (object_id, r) in &relations {
                println!("  {object_id} {r:?}");
            }
            Ok(0)
        }

        "fts-rebuild" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let status = index::rebuild_index(&store, &store.control_dir())?;
            println!(
                "rebuilt: {} records indexed, built_through_seq={}",
                status.indexed_count, status.built_through_seq
            );
            Ok(0)
        }

        "fts-update" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let status = index::ensure_index_current(&store, &store.control_dir())?;
            println!(
                "updated: {} records indexed, built_through_seq={}",
                status.indexed_count, status.built_through_seq
            );
            Ok(0)
        }

        "fts-status" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            match index::status(&store.control_dir())? {
                Some(status) => {
                    let (current_seq, _) = store.transaction_head()?;
                    println!(
                        "built_through_seq={} built_at={} indexed_count={} current_seq={} lag={}",
                        status.built_through_seq,
                        status.built_at,
                        status.indexed_count,
                        current_seq,
                        current_seq - status.built_through_seq
                    );
                }
                None => println!("no index has been built yet"),
            }
            Ok(0)
        }

        "fts-search" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let limit = args
                .get("limit")
                .and_then(|s| s.parse().ok())
                .unwrap_or(20usize);
            let outcome = index::search(
                &store,
                &store.control_dir(),
                args.get("project"),
                args.require("query")?,
                limit,
            )?;
            println!("status: {:?}", outcome.status);
            println!("hits: {}", outcome.hits.len());
            for hit in &outcome.hits {
                println!(
                    "  {} [{}] project={} title={:?}",
                    hit.object_id, hit.kind, hit.project_id, hit.title
                );
            }
            Ok(0)
        }

        "export-preview" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let preview = export::preview_export(&store, args.get("project"))?;
            println!(
                "kind={} project={:?} records={} revisions={} snapshot_head_seq={}",
                preview.kind,
                preview.project_id,
                preview.record_count,
                preview.revision_count,
                preview.snapshot_head_seq
            );
            Ok(0)
        }

        "export-run" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let report =
                export::export_to_new_root(&store, args.get("project"), args.require("out")?)?;
            println!(
                "exported: kind={} records={} revisions={} integrity_root={} -> {}",
                report.manifest.kind,
                report.manifest.record_count,
                report.manifest.revision_count,
                report.manifest.integrity_root,
                report.dest_root.display()
            );
            Ok(0)
        }

        "import-preview" => {
            let preview = import::preview_import(std::path::Path::new(args.require("source")?))?;
            println!(
                "kind={} source_vault={} project={:?} records={} revisions={} conflicts={}",
                preview.kind,
                preview.source_vault_id,
                preview.project_id,
                preview.record_count,
                preview.revision_count,
                preview.conflicts.len()
            );
            for c in &preview.conflicts {
                println!("  conflict: {c}");
            }
            Ok(0)
        }

        "import-full-restore" => {
            let report = import::import_full_restore(
                std::path::Path::new(args.require("source")?),
                args.vault_root()?,
            )?;
            println!(
                "imported: mode={} objects={} revisions={} -> {}",
                report.mode,
                report.imported_object_count,
                report.imported_revision_count,
                report.dest_root.display()
            );
            Ok(0)
        }

        "import-merge" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let report = import::import_selected_merge(
                &mut store,
                std::path::Path::new(args.require("source")?),
            )?;
            println!(
                "imported: mode={} objects={} revisions={}",
                report.mode, report.imported_object_count, report.imported_revision_count
            );
            for (old, new) in &report.id_map {
                println!("  {old} -> {new}");
            }
            Ok(0)
        }

        "record-show" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            match project::read_record(&store, args.require("id")?)? {
                Some(project::RecordPayload::Note(note)) if args.get("preview").is_some() => {
                    let max_chars = args
                        .get("preview")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(200usize);
                    println!("{}", markdown::preview(&note.body, max_chars));
                    Ok(0)
                }
                Some(record) => {
                    println!("{record:?}");
                    Ok(0)
                }
                None => {
                    eprintln!("no record with that id");
                    Ok(2)
                }
            }
        }

        "project-records" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let records = project::list_project_records(&store, args.require("project")?)?;
            println!("records: {}", records.len());
            for (object_id, record) in &records {
                println!("  {object_id} {record:?}");
            }
            Ok(0)
        }

        "capture" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let project_id = args.require("project")?;
            let kind = args.get("kind").unwrap_or("note");
            match kind {
                "note" => {
                    let (outcome, _) = project::create_note(
                        &mut writer,
                        CLI_ACTOR,
                        project_id,
                        args.get("title"),
                        args.require("body")?,
                    )?;
                    println!("{} {}", outcome.object_id, outcome.revision_id);
                }
                "action" => {
                    let dependency_ids = args.get("depends-on").map(split_ids).unwrap_or_default();
                    let (outcome, _) = project::create_action(
                        &mut writer,
                        CLI_ACTOR,
                        project_id,
                        args.require("title")?,
                        args.get("body"),
                        &dependency_ids,
                    )?;
                    println!("{} {}", outcome.object_id, outcome.revision_id);
                }
                "decision" => {
                    let basis = match args.get("basis") {
                        Some(s) => parse_basis(s)?,
                        None => DecisionBasis::UserJudgment,
                    };
                    let verification = match args.get("verification") {
                        Some(s) => parse_verification(s)?,
                        None => DecisionVerification::Unreviewed,
                    };
                    let (outcome, _) = project::create_decision(
                        &mut writer,
                        CLI_ACTOR,
                        project_id,
                        args.require("key")?,
                        args.require("body")?,
                        args.get("rationale"),
                        basis,
                        verification,
                        args.get("valid-from"),
                        args.get("valid-to"),
                    )?;
                    println!("{} {}", outcome.object_id, outcome.revision_id);
                }
                other => {
                    eprintln!("unknown --kind {other} (expected note|action|decision)");
                    return Ok(64);
                }
            }
            Ok(0)
        }

        "source-import" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, _source) = capture::import_file(
                &mut writer,
                CLI_ACTOR,
                args.require("project")?,
                args.require("label")?,
                std::path::Path::new(args.require("path")?),
            )?;
            println!("{} {}", outcome.object_id, outcome.revision_id);
            Ok(0)
        }

        "source-reference" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let mut writer = store.writer()?;
            let (outcome, _source) = capture::create_manual_reference(
                &mut writer,
                CLI_ACTOR,
                args.require("project")?,
                args.require("label")?,
                args.get("repository"),
                args.get("commit"),
                args.get("path"),
            )?;
            println!("{} {}", outcome.object_id, outcome.revision_id);
            Ok(0)
        }

        "source-deactivate" | "source-reactivate" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let id = args.require("id")?;
            let (_, source) = if cmd == "source-deactivate" {
                capture::deactivate_source(&mut store, CLI_ACTOR, id)?
            } else {
                capture::reactivate_source(&mut store, CLI_ACTOR, id)?
            };
            println!("{id} active={}", source.active);
            Ok(0)
        }

        "source-extract" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let bytes = capture::extract_bytes(&store, args.require("id")?)?;
            let out = args.require("out")?;
            std::fs::write(out, &bytes)
                .map_err(|e| crate::Error::Capture(format!("cannot write {out}: {e}")))?;
            println!("wrote {} bytes to {out}", bytes.len());
            Ok(0)
        }

        "project-sources" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let sources = capture::list_project_sources(&store, args.require("project")?)?;
            println!("sources: {}", sources.len());
            for (object_id, source) in &sources {
                println!("  {object_id} {source:?}");
            }
            Ok(0)
        }

        "source-check" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let (outcome, check) =
                source_check::check_source(&mut store, CLI_ACTOR, args.require("id")?)?;
            println!(
                "{} {} status={:?} observed_sha256={:?}",
                outcome.object_id, outcome.revision_id, check.status, check.observed_sha256
            );
            Ok(0)
        }

        "source-checks" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let checks = source_check::list_checks_for_source(&store, args.require("id")?)?;
            println!("checks: {}", checks.len());
            for (object_id, check) in &checks {
                println!(
                    "  {object_id} {:?} at={} sha256={:?}",
                    check.status, check.observed_at, check.observed_sha256
                );
            }
            Ok(0)
        }

        "project-source-checks" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let checks =
                source_check::list_project_source_checks(&store, args.require("project")?)?;
            println!("checks: {}", checks.len());
            for (object_id, check) in &checks {
                println!("  {object_id} {check:?}");
            }
            Ok(0)
        }

        "source-reselect" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let (outcome, source) = source_check::reselect_source(
                &mut store,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                std::path::Path::new(args.require("path")?),
            )?;
            println!(
                "{} {} claimed_path={:?}",
                outcome.object_id, outcome.revision_id, source.claimed_path
            );
            Ok(0)
        }

        "source-admit-change" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let path_override = args.get("path").map(std::path::Path::new);
            let (outcome, source) = source_check::admit_changed_source(
                &mut store,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                path_override,
            )?;
            println!(
                "{} {} sha256={:?}",
                outcome.object_id,
                outcome.revision_id,
                source.capture.map(|c| c.sha256)
            );
            Ok(0)
        }

        "decision-state" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let as_of_recorded = args
                .get("as-of-recorded")
                .map(|s| {
                    s.parse::<i64>()
                        .map_err(|_| Error::Project(format!("invalid --as-of-recorded: {s}")))
                })
                .transpose()?;
            let resolution = decision_state::resolve_decision_state(
                &store,
                args.require("project")?,
                args.require("key")?,
                args.get("as-of-valid"),
                as_of_recorded,
            )?;
            println!(
                "project={} key={:?} as_of_valid={} as_of_recorded={} outcome={:?}",
                resolution.project_id,
                resolution.decision_key,
                resolution.as_of_valid,
                resolution.as_of_recorded,
                resolution.outcome
            );
            println!("considered: {}", resolution.considered.len());
            for c in &resolution.considered {
                println!(
                    "  {} admitted={} lifecycle={:?} valid_from={:?} valid_to={:?} exclusion_reason={:?}",
                    c.decision_id,
                    c.admitted,
                    c.decision.lifecycle,
                    c.decision.valid_from,
                    c.decision.valid_to,
                    c.exclusion_reason
                );
            }
            Ok(0)
        }

        "resume" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let view = resume::resume(&store, args.require("project")?)?;
            println!(
                "project={} reviewed_through_seq={:?} head_seq={}",
                view.project_id, view.reviewed_through_seq, view.head_seq
            );
            println!("conflicts: {}", view.conflicts.len());
            for c in &view.conflicts {
                println!(
                    "  key={:?} candidates={}",
                    c.decision_key,
                    c.considered.len()
                );
            }
            println!(
                "stale_or_missing_evidence: {}",
                view.stale_or_missing_evidence.len()
            );
            for e in &view.stale_or_missing_evidence {
                println!(
                    "  source={} status={:?} observed_at={}",
                    e.source_id, e.latest_check.status, e.latest_check.observed_at
                );
            }
            println!("current_decisions: {}", view.current_decisions.len());
            for d in &view.current_decisions {
                println!("  key={:?} outcome={:?}", d.decision_key, d.outcome);
            }
            println!("next_actions: {}", view.next_actions.len());
            for (id, a) in &view.next_actions {
                println!("  {id} state={:?} title={:?}", a.state, a.title);
            }
            println!("relevant_notes: {}", view.relevant_notes.len());
            for (id, n) in &view.relevant_notes {
                println!("  {id} title={:?}", n.title);
            }
            println!(
                "changes_since_checkpoint: {}",
                view.changes_since_checkpoint.len()
            );
            for c in &view.changes_since_checkpoint {
                println!("  seq={} {} [{}]", c.recorded_seq, c.object_id, c.kind);
            }
            Ok(0)
        }

        "checkpoint-mark" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let through = args
                .get("through")
                .map(|s| {
                    s.parse::<i64>()
                        .map_err(|_| Error::Project(format!("invalid --through: {s}")))
                })
                .transpose()?;
            let (outcome, checkpoint) = checkpoint::mark_reviewed_through(
                &mut store,
                CLI_ACTOR,
                args.require("project")?,
                args.get("expect"),
                through,
            )?;
            println!(
                "{} {} reviewed_through_seq={}",
                outcome.object_id, outcome.revision_id, checkpoint.reviewed_through_seq
            );
            Ok(0)
        }

        "checkpoint-reset" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let through: i64 = args
                .require("through")?
                .parse()
                .map_err(|_| Error::Project("invalid --through".to_string()))?;
            let (outcome, checkpoint) = checkpoint::reset_checkpoint(
                &mut store,
                CLI_ACTOR,
                args.require("project")?,
                args.require("expect")?,
                through,
                args.require("reason")?,
            )?;
            println!(
                "{} {} reviewed_through_seq={} reset_reason={:?}",
                outcome.object_id,
                outcome.revision_id,
                checkpoint.reviewed_through_seq,
                checkpoint.reset_reason
            );
            Ok(0)
        }

        "checkpoint-history" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let history = checkpoint::checkpoint_history(&store, args.require("project")?)?;
            println!("checkpoints: {}", history.len());
            for c in &history {
                println!(
                    "  reviewed_through_seq={} reset_reason={:?}",
                    c.reviewed_through_seq, c.reset_reason
                );
            }
            Ok(0)
        }

        "grant-issue" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let allowed_kinds = args.get("kinds").map(split_ids).unwrap_or_default();
            let allowed_object_ids = args.get("ids").map(split_ids);
            let privacy_exclusions = args.get("exclude").map(split_ids).unwrap_or_default();
            let byte_budget: u32 = args
                .get("budget")
                .map(|s| {
                    s.parse()
                        .map_err(|_| Error::Project(format!("invalid --budget: {s}")))
                })
                .transpose()?
                .unwrap_or(limits::MAX_PACKAGE_BYTES as u32);
            let ttl_hours: i64 = args
                .get("ttl-hours")
                .map(|s| {
                    s.parse()
                        .map_err(|_| Error::Project(format!("invalid --ttl-hours: {s}")))
                })
                .transpose()?
                .unwrap_or(grant::DEFAULT_TTL_SECS / 3600);
            let mut writer = store.writer()?;
            let (outcome, g) = grant::issue_grant(
                &mut writer,
                CLI_ACTOR,
                args.require("project")?,
                &allowed_kinds,
                allowed_object_ids.as_deref(),
                &privacy_exclusions,
                byte_budget,
                ttl_hours * 3600,
            )?;
            println!(
                "{} {} expires_at={} byte_budget={}",
                outcome.object_id, outcome.revision_id, g.expires_at, g.byte_budget
            );
            Ok(0)
        }

        "grant-revoke" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let (outcome, g) = grant::revoke_grant(
                &mut store,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
            )?;
            println!(
                "{} {} state={:?}",
                outcome.object_id, outcome.revision_id, g.state
            );
            Ok(0)
        }

        "package-preview" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            // No receipt is committed by a preview, so there is no real
            // object_id to print — unlike `package-export` below.
            let (receipt, wire) = disclosure::preview_disclosure_package(
                &store,
                args.require("grant")?,
                args.require("request-id")?,
                args.get("principal").unwrap_or("agent"),
            )?;
            println!(
                "selected={} rejected={} emitted_byte_count={} emitted_sha256={}",
                receipt.selected.len(),
                receipt.rejected.len(),
                receipt.emitted_byte_count,
                receipt.emitted_sha256
            );
            for r in &receipt.rejected {
                println!("  rejected {} [{}]: {}", r.object_id, r.kind, r.reason);
            }
            let _ = wire;
            Ok(0)
        }

        "package-export" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let (receipt_id, receipt, wire) = disclosure::compile_disclosure_package(
                &mut store,
                args.require("grant")?,
                args.require("request-id")?,
                args.get("principal").unwrap_or("agent"),
            )?;
            let out_path = std::path::Path::new(args.require("out")?);
            if out_path.exists() {
                return Err(Error::Project(format!(
                    "refusing to overwrite an existing path: {}",
                    out_path.display()
                )));
            }
            std::fs::write(out_path, wire.as_bytes())
                .map_err(|e| Error::Project(format!("cannot write package file: {e}")))?;
            // Stage/verify: reopen and reread what was actually written,
            // rather than trusting the write call succeeded silently.
            let reread = std::fs::read(out_path)
                .map_err(|e| Error::Project(format!("cannot verify written package file: {e}")))?;
            if reread != wire.as_bytes() {
                return Err(Error::Project(
                    "written package file does not match the compiled bytes".to_string(),
                ));
            }
            println!(
                "receipt_id={} selected={} rejected={} emitted_byte_count={} emitted_sha256={} out={}",
                receipt_id,
                receipt.selected.len(),
                receipt.rejected.len(),
                receipt.emitted_byte_count,
                receipt.emitted_sha256,
                out_path.display()
            );
            Ok(0)
        }

        "propose-import" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let file_path = std::path::Path::new(args.require("file")?);
            let raw_bytes = std::fs::read(file_path)
                .map_err(|e| Error::Project(format!("cannot read proposal file: {e}")))?;
            let (outcome, p) = proposal::admit_proposal(
                &mut store,
                CLI_ACTOR,
                args.require("project")?,
                &raw_bytes,
            )?;
            println!(
                "{} {} status={:?} operations={} declared_agent={:?}",
                outcome.object_id,
                outcome.revision_id,
                p.status,
                p.operations.len(),
                p.declared_agent
            );
            Ok(0)
        }

        "propose-show" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let p = proposal::current_proposal(&store, args.require("id")?)?;
            println!(
                "status={:?} receipt_id={} declared_agent={:?} declared_model={:?} declared_tool={:?}",
                p.status, p.receipt_id, p.declared_agent, p.declared_model, p.declared_tool
            );
            println!(
                "submitted_at={} inbound_sha256={}",
                p.submitted_at, p.inbound_sha256
            );
            for (i, op) in p.operations.iter().enumerate() {
                println!("  [{i}] {op:?}");
            }
            if let Some(reason) = &p.review_reason {
                println!("review_reason={reason:?} reviewed_by={:?}", p.reviewed_by);
            }
            Ok(0)
        }

        "project-proposals" => {
            let store = CanonicalStore::open(args.vault_root()?)?;
            let proposals = proposal::list_project_proposals(&store, args.require("project")?)?;
            println!("proposals: {}", proposals.len());
            for (id, p) in &proposals {
                println!(
                    "  {id} status={:?} operations={}",
                    p.status,
                    p.operations.len()
                );
            }
            Ok(0)
        }

        "propose-accept" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let selected: Vec<usize> = split_ids(args.require("select")?)
                .into_iter()
                .map(|s| {
                    s.parse::<usize>()
                        .map_err(|_| Error::Project(format!("invalid --select index: {s}")))
                })
                .collect::<Result<_>>()?;
            let (outcome, p) = proposal::accept_proposal(
                &mut store,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                &selected,
            )?;
            println!(
                "{} {} status={:?} accepted={:?}",
                outcome.object_id, outcome.revision_id, p.status, p.accepted_operation_indices
            );
            Ok(0)
        }

        "propose-reject" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let (outcome, p) = proposal::reject_proposal(
                &mut store,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                args.require("reason")?,
            )?;
            println!(
                "{} {} status={:?}",
                outcome.object_id, outcome.revision_id, p.status
            );
            Ok(0)
        }

        "propose-expire" => {
            let mut store = CanonicalStore::open(args.vault_root()?)?;
            let (outcome, p) = proposal::expire_proposal(
                &mut store,
                CLI_ACTOR,
                args.require("id")?,
                args.require("expect")?,
                args.require("reason")?,
            )?;
            println!(
                "{} {} status={:?}",
                outcome.object_id, outcome.revision_id, p.status
            );
            Ok(0)
        }

        other => {
            eprintln!("unknown command: {other}\n\n{USAGE}");
            Ok(64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp() -> PathBuf {
        let d = std::env::temp_dir().join(format!("fehrest-cli-{}", uuid::Uuid::now_v7()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    /// T01-01: `init` and `compile` used to append their event directly on a
    /// bare `EventLog`, bypassing the writer-owned chokepoint (the exact class
    /// of unbound mutator this task closes). Both now go through
    /// `VaultWriter::append_event`. This test proves the externally-visible
    /// behavior is unchanged — the vault is created, the event is recorded,
    /// and the resulting chain still verifies intact — so the fix is a pure
    /// authorization-path correction, not a behavior change.
    #[test]
    fn init_appends_vault_created_event_via_writer_and_chain_verifies() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        let code = run(&s(&["init", "--vault", &root_str])).unwrap();
        assert_eq!(code, 0);

        let v = Vault::open_read(&root).unwrap();
        let log = EventLog::open(&v.control_dir()).unwrap();
        let events = log.read_all().unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0].kind, EventKind::VaultCreated));
        assert!(matches!(
            log.verify().unwrap(),
            ChainStatus::Intact { events: 1 }
        ));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn compile_appends_context_compiled_event_via_writer_and_chain_verifies() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(run(&s(&["init", "--vault", &root_str])).unwrap(), 0);
        assert_eq!(
            run(&s(&[
                "add", "--vault", &root_str, "--path", "a.md", "--body", "hello"
            ]))
            .unwrap(),
            0
        );
        assert_eq!(run(&s(&["compile", "--vault", &root_str])).unwrap(), 0);

        let v = Vault::open_read(&root).unwrap();
        let log = EventLog::open(&v.control_dir()).unwrap();
        let events = log.read_all().unwrap();
        // init -> VaultCreated, add -> ObjectRegistered, compile -> ContextCompiled
        assert_eq!(events.len(), 3);
        assert!(matches!(events[2].kind, EventKind::ContextCompiled));
        assert!(matches!(
            log.verify().unwrap(),
            ChainStatus::Intact { events: 3 }
        ));
        let _ = std::fs::remove_dir_all(&root);
    }

    /// `T02-01` acceptance criterion: "Create/open/archive/unarchive and
    /// typed record CRUD-as-revision work through CLI." A full lifecycle
    /// through the actual CLI dispatcher, not just the underlying
    /// `project.rs` functions directly — proves the wiring itself, not only
    /// the logic behind it. IDs are recovered via direct store inspection
    /// between steps (this headless CLI's commands print to stdout, not a
    /// programmatic return value), matching this task's own "CLI round-trip
    /// fixtures" verification method.
    #[test]
    fn full_project_and_record_lifecycle_works_through_the_cli() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();

        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "project-create",
                "--vault",
                &root_str,
                "--name",
                "CLI Project",
                "--description",
                "made via the CLI"
            ]))
            .unwrap(),
            0
        );

        let store = CanonicalStore::open(&root).unwrap();
        let project_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                let record = project::RecordPayload::from_json(&payload).ok()?;
                record.as_project().ok().map(|_| id)
            })
            .expect("the created project must be findable via list_current_objects");
        drop(store);

        assert_eq!(
            run(&s(&[
                "note-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--title",
                "Hello",
                "--body",
                "Body text"
            ]))
            .unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "action-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--title",
                "Do the thing"
            ]))
            .unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "decision-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--key",
                "q1",
                "--statement",
                "We will do X"
            ]))
            .unwrap(),
            0
        );

        // project-records lists exactly the 3 work records just created.
        let store = CanonicalStore::open(&root).unwrap();
        let records = project::list_project_records(&store, &project_id).unwrap();
        assert_eq!(records.len(), 3);
        drop(store);

        // Archive, then unarchive, through the CLI.
        assert_eq!(
            run(&s(&[
                "project-archive",
                "--vault",
                &root_str,
                "--id",
                &project_id
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        assert!(!project::open_project(&store, &project_id).unwrap().active);
        drop(store);

        assert_eq!(
            run(&s(&[
                "project-unarchive",
                "--vault",
                &root_str,
                "--id",
                &project_id
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        assert!(project::open_project(&store, &project_id).unwrap().active);

        // record-show and project-show both exit 0 for a real ID, and
        // record-show exits nonzero (not a panic) for an unknown one.
        assert_eq!(
            run(&s(&[
                "project-show",
                "--vault",
                &root_str,
                "--id",
                &project_id
            ]))
            .unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "record-show",
                "--vault",
                &root_str,
                "--id",
                &uuid::Uuid::now_v7().to_string()
            ]))
            .unwrap(),
            2
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    /// A `note-create` referencing a nonexistent project must fail visibly
    /// through the CLI (propagated as an `Err`, not a panic or a false
    /// success exit code) — "invalid cross-project references... reject."
    #[test]
    fn cli_rejects_a_note_create_with_an_invalid_project_reference() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        let err = run(&s(&[
            "note-create",
            "--vault",
            &root_str,
            "--project",
            &uuid::Uuid::now_v7().to_string(),
            "--body",
            "x",
        ]))
        .unwrap_err();
        assert!(format!("{err}").contains("invalid project reference"));
        let _ = std::fs::remove_dir_all(&root);
    }

    /// `T02-02` acceptance criterion: capture/source/artifact admission
    /// "work through CLI" — a full round trip through the actual dispatcher:
    /// default-Note `capture`, explicit `--kind decision`, a real file
    /// import, extraction with exact byte fidelity, and the preview path.
    #[test]
    fn capture_and_source_lifecycle_works_through_the_cli() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "project-create",
                "--vault",
                &root_str,
                "--name",
                "Capture Project"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let project_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_project()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);

        // Default kind is Note.
        assert_eq!(
            run(&s(&[
                "capture",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--body",
                "captured by default as a note"
            ]))
            .unwrap(),
            0
        );
        // Explicit kind selection.
        assert_eq!(
            run(&s(&[
                "capture",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--kind",
                "decision",
                "--key",
                "q1",
                "--body",
                "We will do X"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        assert_eq!(
            project::list_project_records(&store, &project_id)
                .unwrap()
                .len(),
            2
        );
        drop(store);

        // A real file import, through the CLI, with exact byte recovery.
        let fixture_path = root.join("evidence.bin");
        let fixture_bytes: Vec<u8> = (0u8..=255).collect();
        std::fs::write(&fixture_path, &fixture_bytes).unwrap();
        assert_eq!(
            run(&s(&[
                "source-import",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--label",
                "evidence file",
                "--path",
                &fixture_path.to_string_lossy()
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let sources = capture::list_project_sources(&store, &project_id).unwrap();
        assert_eq!(sources.len(), 1);
        let source_id = sources[0].0.clone();
        drop(store);

        let out_path = root.join("recovered.bin");
        assert_eq!(
            run(&s(&[
                "source-extract",
                "--vault",
                &root_str,
                "--id",
                &source_id,
                "--out",
                &out_path.to_string_lossy()
            ]))
            .unwrap(),
            0
        );
        assert_eq!(std::fs::read(&out_path).unwrap(), fixture_bytes);

        // A manual reference and the deactivate/reactivate lifecycle.
        assert_eq!(
            run(&s(&[
                "source-reference",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--label",
                "external doc",
                "--repository",
                "org/repo"
            ]))
            .unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "source-deactivate",
                "--vault",
                &root_str,
                "--id",
                &source_id
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        assert!(
            !capture::list_project_sources(&store, &project_id)
                .unwrap()
                .iter()
                .find(|(id, _)| id == &source_id)
                .unwrap()
                .1
                .active
        );
        drop(store);
        assert_eq!(
            run(&s(&[
                "source-reactivate",
                "--vault",
                &root_str,
                "--id",
                &source_id
            ]))
            .unwrap(),
            0
        );

        // record-show --preview truncates a note's body without altering it.
        let store = CanonicalStore::open(&root).unwrap();
        let note_id = project::list_project_records(&store, &project_id)
            .unwrap()
            .into_iter()
            .find_map(|(id, r)| matches!(r, project::RecordPayload::Note(_)).then_some(id))
            .unwrap();
        drop(store);
        assert_eq!(
            run(&s(&[
                "record-show",
                "--vault",
                &root_str,
                "--id",
                &note_id,
                "--preview",
                "5"
            ]))
            .unwrap(),
            0
        );

        // An unrecognized capture kind is rejected, not silently defaulted.
        assert_eq!(
            run(&s(&[
                "capture",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--kind",
                "bogus",
                "--body",
                "x"
            ]))
            .unwrap(),
            64
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    /// `T02-03` acceptance criterion: the full `Action` state machine and
    /// `Decision` evidence-linkage/acceptance/supersession "work through
    /// CLI" — exercised end-to-end through the actual dispatcher, not the
    /// underlying `project.rs`/`relation.rs` functions directly.
    #[test]
    fn action_decision_relation_lifecycle_works_through_the_cli() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&["project-create", "--vault", &root_str, "--name", "P"])).unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let project_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_project()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);

        // Action lifecycle: create -> start -> complete, through the CLI.
        assert_eq!(
            run(&s(&[
                "action-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--title",
                "Do it"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let (action_id, action_rev) = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, rev, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_action()
                    .ok()
                    .map(|_| (id, rev))
            })
            .unwrap();
        drop(store);

        assert_eq!(
            run(&s(&[
                "action-start",
                "--vault",
                &root_str,
                "--id",
                &action_id,
                "--expect",
                &action_rev
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let (action_rev, _) = store.read_current(&action_id).unwrap().unwrap();
        drop(store);
        assert_eq!(
            run(&s(&[
                "action-complete",
                "--vault",
                &root_str,
                "--id",
                &action_id,
                "--expect",
                &action_rev,
                "--summary",
                "finished"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let action = project::read_record(&store, &action_id)
            .unwrap()
            .unwrap()
            .as_action()
            .unwrap()
            .clone();
        assert_eq!(action.state, project::ActionState::Done);
        // Reopening without --reason is refused by the CLI-driven path too.
        drop(store);
        let store = CanonicalStore::open(&root).unwrap();
        let (done_rev, _) = store.read_current(&action_id).unwrap().unwrap();
        drop(store);
        // A whitespace-only reason (nonempty at the CLI-arg-parsing layer,
        // so it actually reaches `reopen_action`'s own validation) is
        // refused, not silently treated as present.
        let err = run(&s(&[
            "action-reopen",
            "--vault",
            &root_str,
            "--id",
            &action_id,
            "--expect",
            &done_rev,
            "--reason",
            "   ",
        ]))
        .unwrap_err();
        assert!(format!("{err}").contains("requires an explicit, non-empty reason"));

        // Decision: create two competing decisions on the same key, accept
        // one, import a source, link it as evidence, then supersede.
        assert_eq!(
            run(&s(&[
                "decision-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--key",
                "q1",
                "--statement",
                "Use approach A",
                "--basis",
                "user-judgment",
                "--verification",
                "unreviewed"
            ]))
            .unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "decision-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--key",
                "q1",
                "--statement",
                "Use approach B instead"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let mut decisions: Vec<(String, String)> = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .filter_map(|(id, rev, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_decision()
                    .ok()
                    .map(|_| (id, rev))
            })
            .collect();
        decisions.sort();
        drop(store);
        assert_eq!(decisions.len(), 2);
        let (decision_a, rev_a) = decisions[0].clone();
        let (decision_b, rev_b) = decisions[1].clone();

        assert_eq!(
            run(&s(&[
                "decision-accept",
                "--vault",
                &root_str,
                "--id",
                &decision_a,
                "--expect",
                &rev_a
            ]))
            .unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "decision-accept",
                "--vault",
                &root_str,
                "--id",
                &decision_b,
                "--expect",
                &rev_b
            ]))
            .unwrap(),
            0
        );

        // Link the accepted decision to a manually-referenced source as evidence.
        assert_eq!(
            run(&s(&[
                "source-reference",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--label",
                "benchmark results"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let source_id = capture::list_project_sources(&store, &project_id)
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
            .0;
        drop(store);
        assert_eq!(
            run(&s(&[
                "relation-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--type",
                "supports",
                "--from",
                &decision_a,
                "--to",
                &source_id,
                "--note",
                "cited"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let (decision_a_rev, _) = store.read_current(&decision_a).unwrap().unwrap();
        let linked = relation::list_relations_for_object(&store, &decision_a).unwrap();
        drop(store);
        assert_eq!(linked.len(), 1);
        assert_eq!(linked[0].1.relation_type, RelationType::Supports);

        // Supersede decision_a with decision_b (same key, both accepted).
        assert_eq!(
            run(&s(&[
                "decision-supersede",
                "--vault",
                &root_str,
                "--new",
                &decision_b,
                "--old",
                &decision_a,
                "--expect",
                &decision_a_rev,
                "--reason",
                "benchmark favored B"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let a_after = project::read_record(&store, &decision_a)
            .unwrap()
            .unwrap()
            .as_decision()
            .unwrap()
            .clone();
        assert_eq!(a_after.lifecycle, project::DecisionLifecycle::Superseded);
        let project_relations = relation::list_project_relations(&store, &project_id).unwrap();
        assert_eq!(project_relations.len(), 2); // the Supports link + the Supersedes edge
        drop(store);

        let _ = std::fs::remove_dir_all(&root);
    }

    /// `T02-04` acceptance criterion: "CLI search/rebuild" — a full round
    /// trip through the actual dispatcher: build, search, an incremental
    /// update after a new mutation, and status reporting.
    #[test]
    fn fts_index_rebuild_search_and_status_work_through_the_cli() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&["project-create", "--vault", &root_str, "--name", "P"])).unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let project_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_project()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);

        assert_eq!(
            run(&s(&[
                "note-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--body",
                "searchable content about waffles"
            ]))
            .unwrap(),
            0
        );

        // Before any index exists, fts-status reports absence and
        // fts-search still finds the record via canonical fallback.
        assert_eq!(run(&s(&["fts-status", "--vault", &root_str])).unwrap(), 0);
        assert_eq!(
            run(&s(&[
                "fts-search",
                "--vault",
                &root_str,
                "--query",
                "waffles"
            ]))
            .unwrap(),
            0
        );

        assert_eq!(run(&s(&["fts-rebuild", "--vault", &root_str])).unwrap(), 0);
        assert_eq!(
            run(&s(&[
                "fts-search",
                "--vault",
                &root_str,
                "--query",
                "waffles"
            ]))
            .unwrap(),
            0
        );
        assert_eq!(run(&s(&["fts-status", "--vault", &root_str])).unwrap(), 0);

        // A new mutation, then an incremental update through the CLI.
        assert_eq!(
            run(&s(&[
                "note-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--body",
                "second searchable note about pancakes"
            ]))
            .unwrap(),
            0
        );
        assert_eq!(run(&s(&["fts-update", "--vault", &root_str])).unwrap(), 0);

        let store = CanonicalStore::open(&root).unwrap();
        let outcome = index::search(
            &store,
            &store.control_dir(),
            Some(&project_id),
            "pancakes",
            10,
        )
        .unwrap();
        assert_eq!(outcome.hits.len(), 1);
        assert!(matches!(outcome.status, index::SearchStatus::Fresh { .. }));
        drop(store);

        let _ = std::fs::remove_dir_all(&root);
    }

    /// `T02-05` acceptance criterion: "CLI export preview" — a full round
    /// trip through the actual dispatcher: preview, then run, of both a
    /// full export and a project-scoped export.
    #[test]
    fn export_preview_and_run_work_through_the_cli() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&["project-create", "--vault", &root_str, "--name", "P"])).unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let project_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_project()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);
        assert_eq!(
            run(&s(&[
                "note-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--body",
                "exportable text"
            ]))
            .unwrap(),
            0
        );

        assert_eq!(
            run(&s(&["export-preview", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "export-preview",
                "--vault",
                &root_str,
                "--project",
                &project_id
            ]))
            .unwrap(),
            0
        );

        let full_dest = tmp();
        let full_dest_str = full_dest.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&[
                "export-run",
                "--vault",
                &root_str,
                "--out",
                &full_dest_str
            ]))
            .unwrap(),
            0
        );
        assert!(full_dest
            .join(".fehrest-export")
            .join("export-manifest.json")
            .exists());

        let project_dest = tmp();
        let project_dest_str = project_dest.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&[
                "export-run",
                "--vault",
                &root_str,
                "--out",
                &project_dest_str,
                "--project",
                &project_id
            ]))
            .unwrap(),
            0
        );
        let manifest_path = project_dest
            .join(".fehrest-export")
            .join("export-manifest.json");
        let manifest: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&manifest_path).unwrap()).unwrap();
        assert_eq!(manifest["kind"], "export-project");

        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(&full_dest);
        let _ = std::fs::remove_dir_all(&project_dest);
    }

    /// `T02-06` acceptance criterion: "CLI preview" plus a full round trip
    /// through the actual dispatcher: export, preview the export, full
    /// restore into a new vault, and a selected-merge import into an
    /// already-populated one.
    #[test]
    fn import_preview_full_restore_and_merge_work_through_the_cli() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&["project-create", "--vault", &root_str, "--name", "P"])).unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let project_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_project()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);
        assert_eq!(
            run(&s(&[
                "note-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--body",
                "importable text"
            ]))
            .unwrap(),
            0
        );

        let export_dest = tmp();
        let export_dest_str = export_dest.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&[
                "export-run",
                "--vault",
                &root_str,
                "--out",
                &export_dest_str
            ]))
            .unwrap(),
            0
        );

        assert_eq!(
            run(&s(&["import-preview", "--source", &export_dest_str])).unwrap(),
            0
        );

        let restore_dest = tmp();
        let restore_dest_str = restore_dest.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&[
                "import-full-restore",
                "--source",
                &export_dest_str,
                "--vault",
                &restore_dest_str
            ]))
            .unwrap(),
            0
        );
        let restored = CanonicalStore::open(&restore_dest).unwrap();
        assert!(
            project::open_project(&restored, &project_id).is_ok(),
            "object_id must be preserved by full restore"
        );
        drop(restored);

        let merge_dest = tmp();
        let merge_dest_str = merge_dest.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &merge_dest_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "import-merge",
                "--source",
                &export_dest_str,
                "--vault",
                &merge_dest_str
            ]))
            .unwrap(),
            0
        );
        let merged = CanonicalStore::open(&merge_dest).unwrap();
        let records = project::list_project_records(&merged, &project_id);
        assert!(
            records.is_err() || records.unwrap().is_empty(),
            "merge must never reuse the source's own project_id as the destination identity"
        );

        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(&export_dest);
        let _ = std::fs::remove_dir_all(&restore_dest);
        let _ = std::fs::remove_dir_all(&merge_dest);
    }

    /// `T03-01` acceptance criterion: "All four freshness states and
    /// moved/deleted-source scenarios show correct immutable history" —
    /// exercised end to end through the actual CLI dispatcher: import,
    /// recheck (Match), edit the file behind Flake's back and recheck
    /// again (Changed, unmutated until explicit admission), explicit
    /// admission, then a verified relocation.
    #[test]
    fn source_check_reselect_and_admit_change_work_through_the_cli() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&["project-create", "--vault", &root_str, "--name", "P"])).unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let project_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_project()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);

        let file_path = root.join("evidence.txt");
        std::fs::write(&file_path, b"original bytes").unwrap();
        let file_path_str = file_path.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&[
                "source-import",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--label",
                "l",
                "--path",
                &file_path_str,
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let source_id = capture::list_project_sources(&store, &project_id)
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
            .0;
        drop(store);

        assert_eq!(
            run(&s(&[
                "source-check",
                "--vault",
                &root_str,
                "--id",
                &source_id
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let history = source_check::list_checks_for_source(&store, &source_id).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].1.status, source_check::CheckStatus::Match);
        let current_revision = store.read_current(&source_id).unwrap().unwrap().0;
        drop(store);

        std::fs::write(&file_path, b"edited behind Flake's back").unwrap();
        assert_eq!(
            run(&s(&[
                "source-check",
                "--vault",
                &root_str,
                "--id",
                &source_id
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let history = source_check::list_checks_for_source(&store, &source_id).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[1].1.status, source_check::CheckStatus::Changed);
        // Not auto-admitted: the current revision is unchanged.
        assert_eq!(
            store.read_current(&source_id).unwrap().unwrap().0,
            current_revision
        );
        drop(store);

        assert_eq!(
            run(&s(&[
                "source-admit-change",
                "--vault",
                &root_str,
                "--id",
                &source_id,
                "--expect",
                &current_revision,
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let (new_revision, payload) = store.read_current(&source_id).unwrap().unwrap();
        assert_ne!(new_revision, current_revision);
        let source = project::RecordPayload::from_json(&payload)
            .unwrap()
            .as_source()
            .unwrap()
            .clone();
        assert_eq!(
            source.capture.unwrap().sha256,
            crate::events::hash_bytes(b"edited behind Flake's back")
        );
        drop(store);

        let moved_path = root.join("moved-evidence.txt");
        std::fs::rename(&file_path, &moved_path).unwrap();
        let moved_path_str = moved_path.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&[
                "source-reselect",
                "--vault",
                &root_str,
                "--id",
                &source_id,
                "--expect",
                &new_revision,
                "--path",
                &moved_path_str,
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let checks = source_check::list_project_source_checks(&store, &project_id).unwrap();
        assert_eq!(
            checks.len(),
            2,
            "reselect itself is not a check observation"
        );
        drop(store);

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn decision_state_reports_no_accepted_then_current_set_then_needs_review_through_the_cli() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&["project-create", "--vault", &root_str, "--name", "P"])).unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let project_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_project()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);

        // Before any decision exists for this key: NoAcceptedDecision.
        assert_eq!(
            run(&s(&[
                "decision-state",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--key",
                "k"
            ]))
            .unwrap(),
            0
        );

        assert_eq!(
            run(&s(&[
                "decision-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--key",
                "k",
                "--statement",
                "first"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let (decision_a_id, decision_a_rev) = project::list_project_records(&store, &project_id)
            .unwrap()
            .into_iter()
            .find_map(|(id, r)| r.as_decision().ok().map(|_| id))
            .map(|id| {
                let rev = store.read_current(&id).unwrap().unwrap().0;
                (id, rev)
            })
            .unwrap();
        drop(store);

        assert_eq!(
            run(&s(&[
                "decision-accept",
                "--vault",
                &root_str,
                "--id",
                &decision_a_id,
                "--expect",
                &decision_a_rev
            ]))
            .unwrap(),
            0
        );

        // Exactly one accepted decision for this key: CurrentSet.
        let resolution = decision_state::resolve_decision_state(
            &CanonicalStore::open(&root).unwrap(),
            &project_id,
            "k",
            None,
            None,
        )
        .unwrap();
        assert_eq!(
            resolution.outcome,
            decision_state::DecisionOutcome::CurrentSet(decision_a_id.clone())
        );
        assert_eq!(
            run(&s(&[
                "decision-state",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--key",
                "k"
            ]))
            .unwrap(),
            0
        );

        // A second, independently accepted decision for the same key with
        // no supersession edge: NeedsReview (§: "incomparable overlapping
        // decisions with same key produce Conflict").
        assert_eq!(
            run(&s(&[
                "decision-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--key",
                "k",
                "--statement",
                "second"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let (decision_b_id, decision_b_rev) = project::list_project_records(&store, &project_id)
            .unwrap()
            .into_iter()
            .find_map(|(id, r)| {
                r.as_decision()
                    .ok()
                    .filter(|d| d.statement == "second")
                    .map(|_| id)
            })
            .map(|id| {
                let rev = store.read_current(&id).unwrap().unwrap().0;
                (id, rev)
            })
            .unwrap();
        drop(store);
        assert_eq!(
            run(&s(&[
                "decision-accept",
                "--vault",
                &root_str,
                "--id",
                &decision_b_id,
                "--expect",
                &decision_b_rev
            ]))
            .unwrap(),
            0
        );

        let resolution = decision_state::resolve_decision_state(
            &CanonicalStore::open(&root).unwrap(),
            &project_id,
            "k",
            None,
            None,
        )
        .unwrap();
        assert_eq!(
            resolution.outcome,
            decision_state::DecisionOutcome::NeedsReview
        );
        assert_eq!(
            resolution.considered.iter().filter(|c| c.admitted).count(),
            2
        );
        assert_eq!(
            run(&s(&[
                "decision-state",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--key",
                "k"
            ]))
            .unwrap(),
            0
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn resume_and_checkpoint_lifecycle_works_through_the_cli() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&["project-create", "--vault", &root_str, "--name", "P"])).unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let project_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_project()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);

        // No checkpoint yet: resume works, checkpoint-history is empty.
        assert_eq!(
            run(&s(&[
                "resume",
                "--vault",
                &root_str,
                "--project",
                &project_id
            ]))
            .unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "checkpoint-history",
                "--vault",
                &root_str,
                "--project",
                &project_id
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        assert!(checkpoint::checkpoint_history(&store, &project_id)
            .unwrap()
            .is_empty());
        drop(store);

        // First mark: no --expect required.
        assert_eq!(
            run(&s(&[
                "checkpoint-mark",
                "--vault",
                &root_str,
                "--project",
                &project_id
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let (checkpoint_id, checkpoint_rev, first_checkpoint) =
            checkpoint::current_checkpoint(&store, &project_id)
                .unwrap()
                .unwrap();
        let head_after_first_mark = first_checkpoint.reviewed_through_seq;
        drop(store);

        // A note created after the mark is "relevant" in resume.
        assert_eq!(
            run(&s(&[
                "note-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--body",
                "after checkpoint"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let view = resume::resume(&store, &project_id).unwrap();
        assert_eq!(view.reviewed_through_seq, Some(head_after_first_mark));
        assert_eq!(view.relevant_notes.len(), 1);
        drop(store);

        assert_eq!(
            run(&s(&[
                "resume",
                "--vault",
                &root_str,
                "--project",
                &project_id
            ]))
            .unwrap(),
            0
        );

        // Second mark requires --expect.
        let err = run(&s(&[
            "checkpoint-mark",
            "--vault",
            &root_str,
            "--project",
            &project_id,
        ]))
        .unwrap_err();
        assert!(format!("{err}").contains("already exists"));
        assert_eq!(
            run(&s(&[
                "checkpoint-mark",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--expect",
                &checkpoint_rev
            ]))
            .unwrap(),
            0
        );

        // Explicit reset moves the checkpoint backward with a reason.
        let store = CanonicalStore::open(&root).unwrap();
        let (_, second_rev, _) = checkpoint::current_checkpoint(&store, &project_id)
            .unwrap()
            .unwrap();
        drop(store);
        assert_eq!(
            run(&s(&[
                "checkpoint-reset",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--expect",
                &second_rev,
                "--through",
                "0",
                "--reason",
                "re-review everything"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let history = checkpoint::checkpoint_history(&store, &project_id).unwrap();
        assert_eq!(history.len(), 3);
        assert_eq!(history[2].reviewed_through_seq, 0);
        assert_eq!(
            history[2].reset_reason.as_deref(),
            Some("re-review everything")
        );
        let _ = checkpoint_id;
        drop(store);

        assert_eq!(
            run(&s(&[
                "checkpoint-history",
                "--vault",
                &root_str,
                "--project",
                &project_id
            ]))
            .unwrap(),
            0
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn grant_and_package_lifecycle_works_through_the_cli() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&["project-create", "--vault", &root_str, "--name", "P"])).unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let project_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_project()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);
        assert_eq!(
            run(&s(&[
                "note-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--body",
                "disclosable note"
            ]))
            .unwrap(),
            0
        );

        assert_eq!(
            run(&s(&[
                "grant-issue",
                "--vault",
                &root_str,
                "--project",
                &project_id
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let (grant_id, grant_rev) = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, rev, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_export_grant()
                    .ok()?;
                Some((id, rev))
            })
            .unwrap();
        drop(store);

        assert_eq!(
            run(&s(&[
                "package-preview",
                "--vault",
                &root_str,
                "--grant",
                &grant_id,
                "--request-id",
                "req-preview"
            ]))
            .unwrap(),
            0
        );

        let out_path = root.join("package.jsonl");
        let out_str = out_path.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&[
                "package-export",
                "--vault",
                &root_str,
                "--grant",
                &grant_id,
                "--request-id",
                "req-export",
                "--out",
                &out_str
            ]))
            .unwrap(),
            0
        );
        let written = std::fs::read_to_string(&out_path).unwrap();
        assert!(written.contains("disclosable note"));

        let store = CanonicalStore::open(&root).unwrap();
        let receipt = disclosure::current_receipt(
            &store,
            &store
                .list_current_objects()
                .unwrap()
                .into_iter()
                .find_map(|(id, _, payload)| {
                    project::RecordPayload::from_json(&payload)
                        .ok()?
                        .as_disclosure_receipt()
                        .ok()
                        .map(|_| id)
                })
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            receipt.emitted_sha256,
            crate::events::hash_bytes(written.as_bytes())
        );
        drop(store);

        // A second export to the same path must not silently overwrite it.
        let err = run(&s(&[
            "package-export",
            "--vault",
            &root_str,
            "--grant",
            &grant_id,
            "--request-id",
            "req-export-2",
            "--out",
            &out_str,
        ]))
        .unwrap_err();
        assert!(format!("{err}").contains("refusing to overwrite"));

        assert_eq!(
            run(&s(&[
                "grant-revoke",
                "--vault",
                &root_str,
                "--id",
                &grant_id,
                "--expect",
                &grant_rev
            ]))
            .unwrap(),
            0
        );
        let err = run(&s(&[
            "package-preview",
            "--vault",
            &root_str,
            "--grant",
            &grant_id,
            "--request-id",
            "req-after-revoke",
        ]))
        .unwrap_err();
        assert!(format!("{err}").contains("revoked"));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn agent_proposal_lifecycle_works_through_the_cli() {
        let root = tmp();
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&["canonical-init", "--vault", &root_str])).unwrap(),
            0
        );
        assert_eq!(
            run(&s(&["project-create", "--vault", &root_str, "--name", "P"])).unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let project_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_project()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);
        assert_eq!(
            run(&s(&[
                "note-create",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--body",
                "original body"
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let (note_id, note_rev) = project::list_project_records(&store, &project_id)
            .unwrap()
            .into_iter()
            .find_map(|(id, r)| r.as_note().ok().map(|_| id))
            .map(|id| {
                let rev = store.read_current(&id).unwrap().unwrap().0;
                (id, rev)
            })
            .unwrap();
        drop(store);

        assert_eq!(
            run(&s(&[
                "grant-issue",
                "--vault",
                &root_str,
                "--project",
                &project_id
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let grant_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_export_grant()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);

        let package_path = root.join("package.jsonl");
        let package_path_str = package_path.to_string_lossy().to_string();
        assert_eq!(
            run(&s(&[
                "package-export",
                "--vault",
                &root_str,
                "--grant",
                &grant_id,
                "--request-id",
                "req-1",
                "--out",
                &package_path_str
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let receipt_id = store
            .list_current_objects()
            .unwrap()
            .into_iter()
            .find_map(|(id, _, payload)| {
                project::RecordPayload::from_json(&payload)
                    .ok()?
                    .as_disclosure_receipt()
                    .ok()
                    .map(|_| id)
            })
            .unwrap();
        drop(store);

        let proposal_json = format!(
            r#"{{"receipt_id":"{receipt_id}","declared_agent":"cli-test-agent","operations":[{{"kind":"note_edit","note_id":"{note_id}","expected_revision_id":"{note_rev}","body":"edited by agent via cli"}}]}}"#
        );
        let proposal_path = root.join("proposal.json");
        std::fs::write(&proposal_path, &proposal_json).unwrap();
        let proposal_path_str = proposal_path.to_string_lossy().to_string();

        assert_eq!(
            run(&s(&[
                "propose-import",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--file",
                &proposal_path_str
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let (proposal_id, proposal_rev) = proposal::list_project_proposals(&store, &project_id)
            .unwrap()
            .into_iter()
            .next()
            .map(|(id, _)| {
                let rev = store.read_current(&id).unwrap().unwrap().0;
                (id, rev)
            })
            .unwrap();
        drop(store);

        assert_eq!(
            run(&s(&[
                "propose-show",
                "--vault",
                &root_str,
                "--id",
                &proposal_id
            ]))
            .unwrap(),
            0
        );
        assert_eq!(
            run(&s(&[
                "project-proposals",
                "--vault",
                &root_str,
                "--project",
                &project_id
            ]))
            .unwrap(),
            0
        );

        assert_eq!(
            run(&s(&[
                "propose-accept",
                "--vault",
                &root_str,
                "--id",
                &proposal_id,
                "--expect",
                &proposal_rev,
                "--select",
                "0"
            ]))
            .unwrap(),
            0
        );

        let store = CanonicalStore::open(&root).unwrap();
        let (_, note_payload) = store.read_current(&note_id).unwrap().unwrap();
        let note = project::RecordPayload::from_json(&note_payload)
            .unwrap()
            .as_note()
            .unwrap()
            .clone();
        assert_eq!(note.body, "edited by agent via cli");
        let accepted = proposal::current_proposal(&store, &proposal_id).unwrap();
        assert_eq!(accepted.status, proposal::ProposalStatus::Accepted);
        drop(store);

        // A second, separately-admitted proposal can be rejected without
        // touching the already-accepted one.
        std::fs::write(&proposal_path, &proposal_json).unwrap();
        assert_eq!(
            run(&s(&[
                "propose-import",
                "--vault",
                &root_str,
                "--project",
                &project_id,
                "--file",
                &proposal_path_str
            ]))
            .unwrap(),
            0
        );
        let store = CanonicalStore::open(&root).unwrap();
        let (second_id, second_rev) = proposal::list_project_proposals(&store, &project_id)
            .unwrap()
            .into_iter()
            .find(|(id, _)| id != &proposal_id)
            .map(|(id, _)| {
                let rev = store.read_current(&id).unwrap().unwrap().0;
                (id, rev)
            })
            .unwrap();
        drop(store);
        assert_eq!(
            run(&s(&[
                "propose-reject",
                "--vault",
                &root_str,
                "--id",
                &second_id,
                "--expect",
                &second_rev,
                "--reason",
                "duplicate"
            ]))
            .unwrap(),
            0
        );

        let _ = std::fs::remove_dir_all(&root);
    }
}
