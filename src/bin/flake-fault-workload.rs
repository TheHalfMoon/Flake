//! `flake-fault-workload` — a small standalone binary used only by
//! `T05-02`'s own D6 native-unclean-shutdown fault-injection harness
//! (`.github/workflows/t05-02-durability-qualification.yml`'s
//! `d6-vm-unclean-shutdown-linux` job and the equivalent local Hyper-V
//! script for the Windows profile).
//!
//! **What this proves, and what it does not.** This binary performs real
//! canonical-store commits against a real vault on real (virtual) storage,
//! so that a host-level forced kill of the guest running this binary
//! exercises the guest OS's own filesystem page cache/journal exactly as a
//! genuine power loss would — not merely an in-process kill inside a
//! normal, undisturbed host OS (`docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md`
//! §15: "VM shutdown and process kill are labeled separately; neither
//! substitutes for physical power-loss evidence" — this binary is the VM
//! side of that distinction, not the physical side; see
//! `docs/canonical/FOUNDER_T05-02_PHYSICAL_POWER_LOSS_AMENDMENT_2026-09-16.md`
//! for what physical-layer assurance this does not claim).
//!
//! **Resumable by design.** The harness kills this process's *host* (the
//! VM or, for the Windows local profile, the Hyper-V VM) at an
//! unpredictable point mid-run. On the next boot the same invocation is
//! run again against the same vault root: `transaction_head` is read back
//! and committing resumes from there, so "target" is an absolute head
//! sequence number, not a count of new commits this run — restarting
//! never re-does or skips work, and the harness's own post-cycle
//! independent-reader check (`tools/independent-verify/sqlite_reader.py`)
//! is the actual proof of "no acknowledged loss, no false-success
//! recovery", not this binary's own exit status.
//!
//! No network code path exists anywhere in this binary or the library it
//! links (workspace-wide invariant, unchanged).

use fehrest::canonical::{CanonicalStore, CommandInput, CommandTarget, RecordOrigin};
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "\
flake-fault-workload — D6 native-unclean-shutdown fault-injection workload

USAGE:
  flake-fault-workload <vault-root> <target-head-seq>

Opens (or, on first run, creates) a format-2 canonical store at
<vault-root>, then commits small CreateObject records one at a time,
printing `PROGRESS seq=<n>` after each durable commit, until the vault's
own `transaction_head` reaches <target-head-seq>. Intended to be killed
mid-run by an external harness (a VM/host-level SIGKILL, not a process
signal handled by this binary) and re-invoked identically afterward.
";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprint!("{USAGE}");
        return ExitCode::FAILURE;
    }
    let root = PathBuf::from(&args[1]);
    let target: i64 = match args[2].parse() {
        Ok(n) if n >= 0 => n,
        _ => {
            eprintln!("<target-head-seq> must be a non-negative integer");
            return ExitCode::FAILURE;
        }
    };

    let mut store = match CanonicalStore::open(&root) {
        Ok(s) => s,
        Err(_) => match CanonicalStore::create(&root) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("cannot create or open vault at {}: {e}", root.display());
                return ExitCode::FAILURE;
            }
        },
    };

    let (mut seq, _head_hash) = match store.transaction_head() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("cannot read transaction head: {e}");
            return ExitCode::FAILURE;
        }
    };
    println!("RESUME seq={seq} target={target}");
    let _ = std::io::stdout().flush();

    while seq < target {
        let mut writer = match store.writer() {
            Ok(w) => w,
            Err(e) => {
                eprintln!("cannot acquire writer: {e}");
                return ExitCode::FAILURE;
            }
        };
        let payload = format!(
            "{{\"kind\":\"fault-workload\",\"seq\":{},\"pid\":{}}}",
            seq + 1,
            std::process::id()
        );
        let outcome = writer.commit(CommandInput {
            command_id: uuid::Uuid::now_v7().to_string(),
            actor: "fault-workload".into(),
            origin: RecordOrigin::System,
            target: CommandTarget::CreateObject { payload },
        });
        drop(writer);
        match outcome {
            Ok(o) => {
                seq = o.resulting_head_seq;
                println!("PROGRESS seq={seq}");
                let _ = std::io::stdout().flush();
            }
            Err(e) => {
                eprintln!("commit failed at seq={}: {e}", seq + 1);
                return ExitCode::FAILURE;
            }
        }
    }

    println!("DONE seq={seq}");
    ExitCode::SUCCESS
}
