//! `pluma-migrate` — versioned standalone offline format-1-to-format-2
//! migration tool (`T05-01`).
//!
//! **Why a separate binary, not a `fehrest` subcommand.** Plan §21's
//! own compatibility promise is "old owned data must remain readable
//! without keeping an obsolete main app." A subcommand bundled into the
//! main `fehrest`/desktop binary would tie a format-1 owner's only import
//! path to whatever the *current* main app happens to ship — exactly the
//! dependency this promise exists to avoid. This binary links only the
//! `fehrest` library crate itself (`src/migration.rs`, unchanged by this
//! task) and nothing else product-specific, so it can be built, published
//! and run on its own, offline, indefinitely, independent of the desktop
//! app's own release cadence. It is still versioned by the same
//! `Cargo.toml` package version as everything else in this workspace
//! (`--version` below), because a migration report that does not name the
//! exact tool version that produced it is not trustworthy evidence.
//!
//! **What this wraps, unchanged.** Every decision this binary reports
//! (admission/omission rules, exact-byte payload preservation, ambiguous
//! identity refusal, "complete" semantics, interrupted-import manifest)
//! is `src/migration.rs`'s own, already reviewed and tested at `T01-06`.
//! This file is purely a thin CLI/JSON shell over
//! [`fehrest::migration::preview_migration`] and
//! [`fehrest::migration::import_to_new_root`] — it adds no new migration
//! policy of its own (Ponytail: no new abstraction beyond an argv parser
//! and a JSON printer).
//!
//! No network code path exists anywhere in this binary or the library it
//! links (workspace-wide invariant, unchanged).
//!
//! **Two shipped names, one binary.** `pluma-migrate` is the canonical name
//! after the 2026-09-20 Flake→Pluma product rename; `flake-migrate` is kept
//! as a deprecated compatibility alias (`Cargo.toml`'s two `[[bin]]`
//! entries both point at this exact file) and will not be removed without
//! a separate, explicitly governed breaking release.

use fehrest::migration::{self, ImportSelection};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "\
pluma-migrate — standalone offline format-1 -> format-2 migration tool

USAGE:
  pluma-migrate preview <source-root>
  pluma-migrate import  <source-root> <new-root> [--select id1,id2,...]
  pluma-migrate --version
  pluma-migrate --help

Every command is fully offline and touches only the two paths named on
its own command line. `preview` never writes anything. `import` creates
<new-root> fresh (refuses if it already exists) and never modifies
<source-root>. Output is one JSON object on stdout; a non-zero exit code
means the requested operation was refused (see the JSON `error` field on
stderr) -- never a silent partial result.

  preview <source-root>
      Nonmutating dry-run: what would be admitted, what would be omitted
      and why, and the source vault's own event-log chain status
      (informational only -- never authenticated as proof of anything).

  import <source-root> <new-root>
      Import every admitted record into a freshly created format-2 vault
      at <new-root>. Refuses entirely, before creating anything, if the
      preview shows any omission at all (\"ambiguous complete migration
      refuses\").

  import <source-root> <new-root> --select id1,id2,...
      Import exactly the named legacy object IDs. Always reports
      complete=false in its own JSON output, even if the selection
      happens to cover every admittable record, because omitting that
      label would misrepresent intent.
";

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    match run(&argv) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

fn run(argv: &[String]) -> fehrest::Result<ExitCode> {
    match argv.first().map(String::as_str) {
        Some("--version") => {
            println!("pluma-migrate {}", env!("CARGO_PKG_VERSION"));
            Ok(ExitCode::from(0))
        }
        Some("--help") | Some("-h") | None => {
            print!("{USAGE}");
            Ok(ExitCode::from(0))
        }
        Some("preview") => cmd_preview(&argv[1..]),
        Some("import") => cmd_import(&argv[1..]),
        Some(other) => {
            eprintln!("error: unknown command {other}\n");
            print!("{USAGE}");
            Ok(ExitCode::from(1))
        }
    }
}

fn cmd_preview(args: &[String]) -> fehrest::Result<ExitCode> {
    let source_root = args
        .first()
        .map(PathBuf::from)
        .ok_or_else(|| fehrest::Error::Migration("preview requires <source-root>".into()))?;

    let preview = migration::preview_migration(&source_root)?;

    let out = serde_json::json!({
        "schema": "flake-migrate-preview-v1",
        "tool_version": env!("CARGO_PKG_VERSION"),
        "source_root": source_root.display().to_string(),
        "admitted_count": preview.admitted.len(),
        "admitted": preview.admitted.iter().map(|r| serde_json::json!({
            "legacy_object_id": r.legacy_object_id,
            "rel_path": r.rel_path,
            "content_sha256": r.content_sha256,
        })).collect::<Vec<_>>(),
        "omitted": preview.omitted.iter().map(|o| serde_json::json!({
            "rel_path": o.rel_path,
            "reason": o.reason,
        })).collect::<Vec<_>>(),
        "source_event_log_status": preview.source_event_log_status,
        "complete": preview.is_complete(),
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
    Ok(ExitCode::from(0))
}

fn cmd_import(args: &[String]) -> fehrest::Result<ExitCode> {
    if args.len() < 2 {
        return Err(fehrest::Error::Migration(
            "import requires <source-root> <new-root> [--select id1,id2,...]".into(),
        ));
    }
    let source_root = PathBuf::from(&args[0]);
    let new_root = PathBuf::from(&args[1]);

    let selection = match args.get(2).map(String::as_str) {
        None => ImportSelection::AllAdmittedOnly,
        Some("--select") => {
            let ids = args.get(3).ok_or_else(|| {
                fehrest::Error::Migration("--select requires a comma-separated ID list".into())
            })?;
            let set: BTreeSet<String> = ids
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect();
            if set.is_empty() {
                return Err(fehrest::Error::Migration(
                    "--select requires at least one non-empty ID".into(),
                ));
            }
            ImportSelection::Selected(set)
        }
        Some(other) => {
            return Err(fehrest::Error::Migration(format!(
                "unknown import option {other} (expected --select)"
            )))
        }
    };

    let report = migration::import_to_new_root(&source_root, &new_root, selection)?;

    let out = serde_json::json!({
        "schema": "flake-migrate-import-v1",
        "tool_version": env!("CARGO_PKG_VERSION"),
        "source_root": report.source_root.display().to_string(),
        "new_root": report.new_root.display().to_string(),
        "vault_id": report.vault_id,
        "imported_count": report.imported.len(),
        "omitted": report.omitted.iter().map(|o| serde_json::json!({
            "rel_path": o.rel_path,
            "reason": o.reason,
        })).collect::<Vec<_>>(),
        "complete": report.complete,
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
    Ok(ExitCode::from(0))
}
