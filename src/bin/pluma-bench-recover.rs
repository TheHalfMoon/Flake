//! `pluma-bench-recover` — `T05-02`'s own section-27 performance-gate
//! support binary ("Full verify / recovery working copy M/L").
//!
//! `crate::recovery::recover_to_new_root` has no CLI surface (recovery is
//! a desktop/library operation, per `T04-05`'s own "expose backup,
//! recovery, import and export safely" scope) -- this binary exists only
//! to time it as a real release-build subprocess, matching every other
//! section-27 measurement's own methodology (a real process invocation,
//! not an in-process `cargo test` timer). It adds no new recovery policy;
//! every decision it reports is `recovery::recover_to_new_root`'s own,
//! unchanged.
//!
//! No network code path exists anywhere in this binary or the library it
//! links (workspace-wide invariant, unchanged).

use fehrest::recovery;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: pluma-bench-recover <original-root> <new-root>");
        return ExitCode::FAILURE;
    }
    let original_root = PathBuf::from(&args[1]);
    let new_root = PathBuf::from(&args[2]);

    let start = Instant::now();
    let result = recovery::recover_to_new_root(&original_root, &new_root);
    let elapsed = start.elapsed();

    match result {
        Ok(report) => {
            println!(
                "{{\"ok\":true,\"elapsed_seconds\":{:.6},\"verified_transaction_head_seq\":{},\"verified_object_count\":{}}}",
                elapsed.as_secs_f64(),
                report.verified_transaction_head_seq,
                report.verified_object_count
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!(
                "{{\"ok\":false,\"elapsed_seconds\":{:.6},\"error\":{:?}}}",
                elapsed.as_secs_f64(),
                e.to_string()
            );
            ExitCode::FAILURE
        }
    }
}
